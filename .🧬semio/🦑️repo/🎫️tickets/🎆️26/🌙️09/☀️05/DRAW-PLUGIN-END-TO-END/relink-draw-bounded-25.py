"""🖍️ Relink the compiled Draw dependency snapshot using one backend and linker thread."""
from pathlib import Path
import os
import shlex
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
root = Path.cwd()
line = next(line for line in (generated / 'native-wasi25.log').read_text().splitlines()
            if "process didn't exit successfully:" in line and '--crate-name semio_hub_draw_component ' in line)
command = shlex.split(line.split('`', 2)[1])
assert Path(command[0]).name == 'rustc' and command[1:3] == ['--crate-name', 'semio_hub_draw_component']
output = generated / 'bounded25'
output.mkdir(exist_ok=True)
command[command.index('--out-dir') + 1] = str(output)
command.extend(['-Zthreads=1', '-Ccodegen-units=1', '-Clink-arg=--append-lld-flag=--threads=1', '-Clink-arg=--append-lld-flag=--strip-all'])
environment = {**os.environ, 'SEMIO_BUILD_MODE': 'prototype'}
with (generated / 'native-wasi-bounded25.log').open('w') as log:
    result = subprocess.run(command, cwd=root / '🌎️hub', env=environment, stdout=log, stderr=log)
raise SystemExit(result.returncode)
