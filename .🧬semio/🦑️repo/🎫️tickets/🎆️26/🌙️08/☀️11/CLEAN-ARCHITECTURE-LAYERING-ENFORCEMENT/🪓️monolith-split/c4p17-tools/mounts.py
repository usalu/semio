"""🧭 c4p17 mount check: every new child is mounted exactly once (#[path] or include!) across the .rs files of its artifact, every mount literal of the parent files resolves to an existing file.
usage: mounts.py"""
import os, re, subprocess, sys, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p17 as P, mkspec as M
RX = re.compile(r'#\[path\s*=\s*"([^"\\]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;|include!\(\s*"([^"\\]+)"\s*\)')
def files_under(rel):
    a = subprocess.run(['git', '-c', 'core.quotepath=false', '-c', 'core.fsmonitor=false', 'ls-files', '--', rel], cwd=P.REPO, capture_output=True, text=True).stdout.split('\n')
    b = subprocess.run(['git', '-c', 'core.quotepath=false', '-c', 'core.fsmonitor=false', 'ls-files', '--others', '--exclude-standard', '--', rel], cwd=P.REPO, capture_output=True, text=True).stdout.split('\n')
    return sorted({x for x in a + b if x.endswith('.rs')})
arts = {}
for tag, f in M.FILES.items():
    art = f.split('/🏅️standards/')[0] if '/🏅️standards/' in f else f.split('/🔨️modules/')[0]
    arts.setdefault(art, []).append(tag)
bad = 0
for art, tags in arts.items():
    counts = collections.Counter()
    missing = []
    fl = files_under(art)
    for rel in fl:
        p = os.path.join(P.REPO, rel)
        if not os.path.exists(p):
            continue
        try:
            txt = open(p, encoding='utf-8').read()
        except Exception:
            continue
        for m in RX.finditer(txt):
            lit = m.group(1) or m.group(2)
            tgt = os.path.normpath(os.path.join(os.path.dirname(p), lit))
            counts[tgt] += 1
    for tag in tags:
        base = os.path.join(P.REPO, os.path.dirname(M.FILES[tag]))
        sp = __import__('specs.' + tag, fromlist=['SPEC']).SPEC
        for c in sp['children']:
            tgt = os.path.join(base, c['folder'], '🦀️.rs')
            n = counts.get(tgt, 0)
            ex = os.path.exists(tgt)
            if n != 1 or not ex:
                bad += 1
                print('BAD', tag, c['folder'], 'mounts', n, 'exists', ex)
        for m in RX.finditer(open(os.path.join(P.REPO, M.FILES[tag]), encoding='utf-8').read()):
            lit = m.group(1) or m.group(2)
            tgt = os.path.normpath(os.path.join(base, lit))
            if not os.path.exists(tgt):
                missing.append((tag, lit))
    dups = [(os.path.relpath(t, P.REPO)[-70:], n) for t, n in counts.items() if n > 1 and t.endswith('.rs') and any(os.path.dirname(M.FILES[tg]) in t for tg in tags)]
    print(art.split('/')[-1], 'files scanned', len(fl), 'children mounted once: ' + ('yes' if not bad else 'NO'), 'missing parent-mount targets', missing, 'dup .rs mounts inside the artifact (any, incl. old)', dups[:3])
print('MOUNTS', 'OK' if not bad else 'BAD')
