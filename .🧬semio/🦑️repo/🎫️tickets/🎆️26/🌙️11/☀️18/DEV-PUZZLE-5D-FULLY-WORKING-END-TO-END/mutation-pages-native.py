from pathlib import Path
import json
import subprocess

root = Path.cwd()
ticket = Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
build = generated / 'native-target/b/debug/build'
rustc = root / '.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc'
publication = max(build.glob('semio-os-store-one-item-publication/*/fingerprint/lib-*.json'), key=lambda path: path.stat().st_mtime_ns)
metadata = json.loads(publication.read_text())
dependencies = {}
for _, name, _, digest in metadata['deps']:
    if name in {'semio_framework_value', 'serde'}:
        marker = digest.to_bytes(8, 'little').hex()
        record = next(path for path in build.glob(f'*/*/fingerprint/lib-{name}') if path.read_text() == marker)
        dependencies[name] = next((record.parent.parent / 'out').glob(f'lib{name}-*.rlib'))
serde_metadata = json.loads((dependencies['serde'].parent.parent / 'fingerprint/lib-serde.json').read_text())
serde_core = next(digest for _, name, _, digest in serde_metadata['deps'] if name == 'serde_core')
for record in build.glob('serde_json/*/fingerprint/lib-serde_json.json'):
    if any(name == 'serde_core' and digest == serde_core for _, name, _, digest in json.loads(record.read_text())['deps']):
        dependencies['serde_json'] = next((record.parent.parent / 'out').glob('libserde_json-*.rlib'))
        break
source = root / '🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📦️input-pages/🦀️.rs'
output = generated / 'mutation-pages-native'
args = [str(rustc), '--edition=2021', '--crate-name', 'semio_mutation_pages_law', '--test',
        '-Zthreads=1', '-Clinker-features=+lld', '-Clink-self-contained=+linker', '-Zunstable-options',
        '-o', str(output), '-']
for path in sorted(build.glob('*/*/out')):
    args.extend(['-L', f'dependency={path}'])
for name, path in dependencies.items():
    args.extend(['--extern', f'{name}={path}'])
    if path.with_suffix('.rmeta').exists():
        args.extend(['--extern', f'{name}={path.with_suffix(".rmeta")}'])
subprocess.run(args, input=f'#[path = "{source}"] mod input_pages;', text=True, check=True)
subprocess.run([output, 'mutation_pages_keep_order_and_bound_each_backing_release'], check=True)
