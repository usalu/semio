#!/usr/bin/env python3
"""🧵 Run the pinned compiler with a serial frontend in the local Draw build."""
import json
import os
from pathlib import Path
import sys

command = sys.argv[1:]
if '--crate-name' in command:
    command += ['-Zthreads=1']
    name = command[command.index('--crate-name') + 1]
    if os.environ.get('DRAW_TEST_COMPILER_RECORDS') == '1' and name.startswith('semio_s_artifact_draw_'):
        directory = Path(__file__).resolve().parent / '🗑️generated' / 'compiler-records'
        directory.mkdir(exist_ok=True)
        keep = {'CARGO', 'OUT_DIR', 'RUSTC_BOOTSTRAP', 'RUST_MIN_STACK', 'SEMIO_BUILD_MODE', 'SEMIO_LANGUAGE'}
        env = {key: value for key, value in os.environ.items()
               if key in keep or key.startswith(('CARGO_MANIFEST_', 'CARGO_PKG_', 'CARGO_CFG_', 'CARGO_FEATURE_'))}
        kind = 'test' if '--test' in command else 'lib'
        record = directory / f'{name}-{kind}.json'
        temporary = record.with_suffix(f'.{os.getpid()}.tmp')
        temporary.write_text(json.dumps({'cwd': os.getcwd(), 'argv': command, 'environment': env}))
        temporary.replace(record)
os.execv(command[0], command)
