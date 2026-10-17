"""🎨 c4p17 rustfmt check: parse success of every live file of the split and the count of rustfmt hunks (`--check`) versus the pre-edit snapshots.
usage: fmtcheck.py"""
import os, re, subprocess, sys, shutil, tempfile
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p17 as P, mkspec as M
CMD = ['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config', 'skip_children=true', '--config-path', P.REPO]
def run(path, check):
    p = subprocess.run(CMD + (['--check'] if check else ['--emit', 'stdout']) + [path], capture_output=True, text=True)
    return p.returncode, p.stdout, p.stderr
tot_before = tot_after = parse_fail = nfiles = 0
for tag, f in M.FILES.items():
    base = os.path.join(P.REPO, os.path.dirname(f))
    sp = __import__('specs.' + tag, fromlist=['SPEC']).SPEC
    live = [os.path.join(P.REPO, f)] + [os.path.join(base, c['folder'], '🦀️.rs') for c in sp['children']]
    snap = os.path.join(T, 'orig', tag + '.rs.txt')
    tmp = os.path.join(P.SCR, 'fmt-' + tag + '.rs')
    shutil.copyfile(snap, tmp)
    rc, so, se = run(tmp, True)
    before = len(re.findall(r'^Diff in ', so, re.M))
    rc0, _, se0 = run(tmp, False)
    after = 0
    for p in live:
        nfiles += 1
        rc1, so1, se1 = run(p, True)
        after += len(re.findall(r'^Diff in ', so1, re.M))
        rc2, _, se2 = run(p, False)
        if rc2 != 0 and 'error' in se2.lower():
            parse_fail += 1
            print('PARSE FAIL', p[-80:], se2[:200])
    tot_before += before; tot_after += after
    print('%-10s hunks before %3d after %3d (parse of snapshot rc=%d)' % (tag, before, after, rc0))
print('files', nfiles, 'parse failures', parse_fail, 'hunks before', tot_before, 'after', tot_after)
