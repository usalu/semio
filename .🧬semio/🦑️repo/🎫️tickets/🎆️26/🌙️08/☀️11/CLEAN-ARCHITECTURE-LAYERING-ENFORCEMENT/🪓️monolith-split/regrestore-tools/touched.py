import sys, os, json, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run import *

def touched(id):
    mp = manifest_path(id)
    if not os.path.exists(mp):
        return []
    man = json.load(open(mp))
    out = []
    for p, m in man.items():
        fp = os.path.join(ROOT, p)
        if os.path.exists(fp) and sha(read(fp)) != m['full']:
            out.append(p)
    return out

if __name__ == '__main__':
    cmd = sys.argv[1]
    ids = sys.argv[2:] or list(SPECS)
    for i in ids:
        t = touched(i)
        if cmd == 'list':
            print(i, len(t))
        elif cmd == 'rustfmt':
            bad = 0
            for p in t:
                if not p.endswith('.rs'):
                    continue
                r = subprocess.run(['rustfmt', '--edition', '2021', '--emit', 'stdout', '--config-path', ROOT + '/rustfmt.toml'], input=read(os.path.join(ROOT, p)).encode(), capture_output=True)
                if r.returncode not in (0, 1):
                    bad += 1
                    print('  RUSTFMT FAIL', p[-80:], r.returncode, r.stderr.decode()[:200])
            print(i, 'rust files touched', sum(1 for p in t if p.endswith('.rs')), 'parse failures', bad)
