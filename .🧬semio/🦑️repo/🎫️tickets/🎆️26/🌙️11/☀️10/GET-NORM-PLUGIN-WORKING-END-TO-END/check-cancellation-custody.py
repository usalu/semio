"""🧾 Build only the revised cancellation issuer against a warmed WASI cohort and run its actor law."""
from pathlib import Path
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
repo = ticket.parents[6]
work = ticket / '🗑️generated'
lane = repo / '.cache/b/r/m/u/l' / sys.argv[1]
build = lane / 'wasm32-wasip2/prototype/build'
cancellation = repo / '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation'
rustc = os.environ['RUSTC']
dependencies = [f'-Ldependency={out}' for out in build.glob('*/*/out')]
dependencies += [f'-Ldependency={out}' for out in lane.glob('prototype/build/*/*/out')]
base = ['--edition=2021', '--target=wasm32-wasip2', '-Copt-level=1', '-Cpanic=abort', *dependencies]


def externs(names):
    parts = []
    for name in names:
        library = max((build / name).glob('*/out/*.rlib'), key=lambda path: path.stat().st_mtime)
        alias = name.replace('-', '_')
        for path in (library, library.with_suffix('.rmeta')):
            parts.extend(['--extern', f'{alias}={path}'])
    return parts


def compile(name, flags):
    response = work / f'{name}.args'
    response.write_text('\n'.join(flags) + '\n')
    if subprocess.run([rustc, '@' + str(response)]).returncode:
        raise SystemExit(f'{name} failed to compile')


operation = work / 'libsemio_plugin_operation.rlib'
operation_dependencies = ['serde_json', 'semio-framework', 'semio-framework-async', 'semio-framework-job',
    'semio-framework-job-payload', 'semio-os-kernel', 'semio-plugin-ownership', 'semio-framework-value',
    'semio-ui-locale', 'semio-framework-trace']
compile('cancellation-operation', [*base, '--crate-type=rlib', '--crate-name=semio_plugin_operation',
    str(cancellation / '🦀️.rs'), '-o', str(operation), *externs(operation_dependencies)])
component = work / 'cancellation-custody.wasm'
compile('cancellation-custody', [*base, str(ticket / 'check-cancellation-custody.rs'), '-o', str(component),
    '--extern', f'semio_plugin_operation={operation}', *externs(['semio-framework-actor-retained-context',
    'semio-framework-actor-retained-turn', 'semio-framework-job', 'semio-framework-value'])])
subprocess.run(['node_modules/.bin/jco', 'run', '--jco-dir', work / 'cancellation-custody-jco', component], cwd=repo, check=True)
