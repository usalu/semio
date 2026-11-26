"""🖍️ Relink the compiled Draw dependency snapshot using one backend and linker thread."""
from pathlib import Path
import os
import shlex
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
root = Path.cwd()
with (generated / 'native-wasi25.log').open() as source:
    line = next(row for row in source if "process didn't exit successfully:" in row and '--crate-name semio_hub_draw_component ' in row)
command = shlex.split(line.split('`', 2)[1])
compiler = Path(command[0])
assert compiler.stem == 'rustc' and command[1:3] == ['--crate-name', 'semio_hub_draw_component']
toolchain = compiler.parent.parent.name
command = ['repo', 'tools', 'exec', 'rustup', '--', 'run', toolchain, 'rustc', *command[1:]]
output = generated / 'bounded25'
output.mkdir(exist_ok=True)
command[command.index('--out-dir') + 1] = str(output)
command.extend(['-Zthreads=1', '-Ccodegen-units=1', '-Clink-arg=--append-lld-flag=--threads=1', '-Clink-arg=--append-lld-flag=--strip-all'])
environment = {**os.environ, 'SEMIO_BUILD_MODE': 'prototype', 'SEMIO_COMPILER_PARALLELISM': '1'}
with (generated / 'native-wasi-bounded25.log').open('w') as log:
    result = subprocess.run(command, cwd=root / '🌎️hub', env=environment, stdout=log, stderr=log)
raise SystemExit(result.returncode)
