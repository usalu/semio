import subprocess, sys, os, glob
R = '/Users/ueli/Documents/semio'
CFG = R + '/rustfmt.toml'


def hunks(path):
    p = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--check', '--config-path', CFG, '--config', 'skip_children=true', path], capture_output=True, text=True)
    n = p.stdout.count('\nDiff in ') + (1 if p.stdout.startswith('Diff in ') else 0)
    return p.returncode, n, p.stderr.strip()[:200]


if __name__ == '__main__':
    tot = 0
    for path in sys.argv[1:]:
        rc, n, err = hunks(path)
        tot += n
        print(rc, n, os.path.basename(os.path.dirname(path)), err)
    print('total hunks', tot)
