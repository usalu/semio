from ast import literal_eval
from pathlib import Path
import os
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
output = generated / 'current-5d-relink' / 'window-registry-probe'
log = generated / 'ui-window-registry-backed.log'
command = next(line for line in log.read_text().splitlines() if line.startswith('subprocess.CalledProcessError: Command '))
args = literal_eval(command[command.index('['):command.rindex("]'") + 1])
unsupported = args.index('link-arg=-Wl,--threads=1')
del args[unsupported - 1:unsupported + 1]
environment = {
    **os.environ,
    'ZIG_GLOBAL_CACHE_DIR': str(generated / 'zig'),
    'ZIG_LOCAL_CACHE_DIR': str(generated / 'zig-local'),
    'RUST_MIN_STACK': '134217728',
}
status = subprocess.run(args, env=environment).returncode
if status:
    raise SystemExit(status)
raise SystemExit(subprocess.run([str(output), '--nocapture', '--test-threads=1'], env=environment).returncode)
