from pathlib import Path
import json
import os
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / '🗑️generated'
upper = generated / 'builder/ov/u/debug/build'
lower = ticket.parent.parent / '☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target/b/debug/build'
test = upper / 'semio-s-artifact-puzzle-2d-history-edit-runtime-tests/868dd8f372688dc3/fingerprint/test-lib-semio_s_artifact_puzzle_2d_history_edit_runtime_tests.json'
projection = root / '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot/🎲️board-projection/🦀️.rs'
rustc = root / '.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc'
name = 'semio_framework_pack_json'
key = next(checksum.to_bytes(8, 'little').hex() for _, alias, _, checksum in json.loads(test.read_text())['deps'] if alias == name)
fingerprints = {}
for cache in (upper, lower):
    for fingerprint in cache.glob('*/*/fingerprint/lib-*'):
        if fingerprint.suffix == '.json':
            continue
        checksum = fingerprint.read_text().strip()
        if not checksum or checksum in fingerprints:
            continue
        package = fingerprint.parent.parent
        alias = fingerprint.name.removeprefix('lib-')
        library = package / 'out' / f'lib{alias}-{package.name}.rlib'
        if not library.is_file():
            library = library.with_suffix('.so')
        if library.is_file():
            fingerprints[checksum] = (fingerprint.with_suffix('.json'), library)

selected = set()
def collect(checksum):
    if checksum in selected or checksum not in fingerprints:
        return
    selected.add(checksum)
    for _, _, _, child in json.loads(fingerprints[checksum][0].read_text())['deps']:
        collect(child.to_bytes(8, 'little').hex())

collect(key)
library = fingerprints[key][1]
args = [str(rustc), str(ticket / 'ui-painted-probe.rs'), '--edition=2021', '-Z', 'unstable-options', '-Z', 'threads=1',
        '-C', 'linker-features=+lld', '-C', 'link-self-contained=+linker', '-o', str(generated / 'ui-painted-probe')]
for parent in sorted({fingerprints[checksum][1].parent for checksum in selected}):
    args += ['-L', f'dependency={parent}']
args += ['--extern', f'{name}={library}']
if library.with_suffix('.rmeta').is_file():
    args += ['--extern', f'{name}={library.with_suffix(".rmeta")}']
environment = {**os.environ, 'SEMIO_UI_BOARD_PROJECTION': str(projection), 'ZIG_GLOBAL_CACHE_DIR': str(generated / 'zig'), 'ZIG_LOCAL_CACHE_DIR': str(generated / 'zig-local')}
print(f'[DEBUG] Compiling the source-level board projection with {len(selected)} frozen dependency fingerprints', flush=True)
raise SystemExit(subprocess.run(args, env=environment).returncode)
