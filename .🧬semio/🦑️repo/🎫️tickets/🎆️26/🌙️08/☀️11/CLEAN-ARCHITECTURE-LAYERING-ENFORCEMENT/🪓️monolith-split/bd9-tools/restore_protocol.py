#!/usr/bin/env python3
"""Re-add [DEBUG] statements that strip_debug removed but that are protocol (embedded in strings, or consumed elsewhere).

3-way merge per file: current worktree file + (index blob stripped legacy-style -> index blob stripped string-aware with stems).
usage: restore_protocol.py STEMS_FILE JOURNAL_JSON [--apply] [--out=REPORT]
"""
import json, os, subprocess, sys, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import strip_debug as sd


def main():
    stems = [l.rstrip('\n') for l in open(sys.argv[1], encoding='utf-8') if l.strip()]
    journal = json.load(open(sys.argv[2], encoding='utf-8'))
    apply = '--apply' in sys.argv
    out = [a for a in sys.argv if a.startswith('--out=')]
    files = []
    only = [a for a in sys.argv if a.startswith('--only=')]
    if only:
        files = [e['file'] for e in json.load(open(only[0].split('=', 1)[1], encoding='utf-8')) if e['status'] in ('merged', 'conflict')]
    else:
        for e in journal:
            if (e.get('delete') or e.get('expr')) and e['file'] not in files:
                files.append(e['file'])
    rep = []
    tmp = tempfile.mkdtemp(prefix='bd9-merge-', dir=os.environ.get('BD9_TMP'))
    for f in files:
        r = subprocess.run(['git', '-c', 'core.fsmonitor=false', 'show', ':' + f], capture_output=True)
        if r.returncode != 0:
            rep.append({'file': f, 'status': 'no-index'})
            continue
        base = r.stdout.decode('utf-8')
        ball, _ = sd.process(f, base, legacy=True)
        bkeep, st = sd.process(f, base, legacy=False, stems=stems)
        if ball == bkeep:
            continue
        cur_st = os.stat(f)
        cur = open(f, encoding='utf-8').read()
        pa, pb, pc = (os.path.join(tmp, n) for n in ('cur', 'base', 'other'))
        open(pa, 'w', encoding='utf-8').write(cur)
        open(pb, 'w', encoding='utf-8').write(ball)
        open(pc, 'w', encoding='utf-8').write(bkeep)
        mr = subprocess.run(['git', 'merge-file', '-p', pa, pb, pc], capture_output=True)
        conflicts = mr.returncode
        item = {'file': f, 'conflicts': conflicts, 'kept': len(st.get('kept', [])), 'instring': len(st.get('instring', []))}
        if conflicts == 0:
            new = mr.stdout.decode('utf-8')
            item['status'] = 'merged' if new != cur else 'nochange'
            if apply and new != cur:
                if os.stat(f).st_mtime_ns != cur_st.st_mtime_ns:
                    item['status'] = 'changed-meanwhile'
                else:
                    t = f + '.bd9tmp'
                    open(t, 'w', encoding='utf-8').write(new)
                    os.replace(t, f)
                    item['status'] = 'applied'
        else:
            item['status'] = 'conflict'
        rep.append(item)
    c = {}
    for i in rep:
        c[i['status']] = c.get(i['status'], 0) + 1
    print(json.dumps(c))
    if out:
        json.dump(rep, open(out[0].split('=', 1)[1], 'w', encoding='utf-8'), ensure_ascii=False, indent=1)


main()
