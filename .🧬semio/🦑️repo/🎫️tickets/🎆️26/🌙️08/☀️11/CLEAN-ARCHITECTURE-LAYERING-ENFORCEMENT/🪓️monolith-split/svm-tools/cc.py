# 🛠️ One sanctioned `bun repo cargo ... check` under the fleet compile mutex (WAVE5-BRIEF "Compile admission is single-slot"): acquire (mkdir, retry every 60 s), run once, release always. usage: cc.py <logname> <cargo args after -->; log 🗑️generated/svm-cc-<logname>.log. Exit 3 = headroom refusal (retry after 10 minutes).
import os, subprocess, sys, time, datetime, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = HERE + '/../🗑️generated'
LOCK = GEN + '/cargo-mutex.lock'
name = sys.argv[1]; args = sys.argv[2:]
log = f'{GEN}/svm-cc-{name}.log'
def stale():
    try: return time.time() - os.path.getmtime(LOCK + '/owner') > 75 * 60
    except OSError: return False
while True:
    try:
        os.mkdir(LOCK); break
    except FileExistsError:
        if stale():
            print('stale mutex removed', open(LOCK + '/owner').read().strip(), flush=True)
            os.remove(LOCK + '/owner'); os.rmdir(LOCK); continue
        print('mutex busy:', open(LOCK + '/owner').read().strip() if os.path.exists(LOCK + '/owner') else '?', flush=True)
        time.sleep(60)
try:
    open(LOCK + '/owner', 'w').write(f'svm {os.getpid()} {datetime.datetime.now().isoformat(timespec="seconds")}\n')
    print('acquired', flush=True)
    env = dict(os.environ, SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}')
    with open(log, 'w', encoding='utf8') as f:
        p = subprocess.run(['perl', '-e', 'alarm 3500; exec @ARGV', '--', 'bun', 'repo', 'cargo', '--store=shared', '--configuration=prototype', '--parallelism=4', '--'] + args, cwd='/Users/ueli/Documents/semio', env=env, stdout=f, stderr=subprocess.STDOUT)
finally:
    try:
        os.remove(LOCK + '/owner'); os.rmdir(LOCK)
    except OSError: pass
    print('released', flush=True)
txt = open(log, encoding='utf8', errors='replace').read()
if p.returncode != 0 and 'Insufficient physical compiler initial headroom' in txt:
    print('headroom refusal'); sys.exit(3)
print('exit', p.returncode); sys.exit(p.returncode)
