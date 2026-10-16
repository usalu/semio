# ⏳ Waits for compiler image headroom, then runs `update --workspace --offline` and one batched `check --lib` of every carved crate through cc.py (mutex protocol). Writes okeng-cc-wait.out. usage: waitcc.py <logname> <crates-file> [--tests]
import os, subprocess, sys, time
OKD = os.path.dirname(os.path.abspath(__file__))
IMG = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m'
name = sys.argv[1]; keys = open(sys.argv[2]).read().split(); tests = '--tests' in sys.argv
def avail():
    s = os.statvfs(IMG); return s.f_bavail * s.f_frsize
def run(tag, args):
    return subprocess.run(['python3', '-u', OKD + '/cc.py', tag] + args).returncode
upd = False
while True:
    a = avail()
    print(time.strftime('%H:%M:%S'), 'available', a, flush=True)
    if a < 4294967296 + 4_000_000: time.sleep(45); continue
    if not upd:
        rc = run(name + '-update', ['update', '--workspace', '--offline'])
        print('update rc', rc, flush=True)
        if rc == 3: time.sleep(45); continue
        upd = True
    args = ['check'] + sum((['-p', 'semio-framework-' + k] for k in keys), []) + ['--lib'] + (['--tests'] if tests else []) + ['--keep-going', '--message-format=short']
    rc = run(name, args)
    print('check rc', rc, flush=True)
    if rc == 3: time.sleep(45); continue
    sys.exit(rc)
