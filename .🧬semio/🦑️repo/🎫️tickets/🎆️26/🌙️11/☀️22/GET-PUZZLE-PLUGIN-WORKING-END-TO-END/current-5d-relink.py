from pathlib import Path
import json, os, subprocess, sys
ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
gen = ticket / "🗑️generated"
cache = gen / "current-native/b/debug/build"
scratch = gen / "current-5d-relink"
libs = scratch / "deps"
libs.mkdir(parents=True, exist_ok=True)
name = 'semio_s_artifact_puzzle_5d_v1_unit_tests'
fingerprints = cache.glob(f'semio-s-artifact-puzzle-5d-v1-unit-tests/*/fingerprint/test-lib-{name}.json')
registry_probe = '--window-registry' in sys.argv
fingerprint = max(fingerprints, key=lambda path: path.stat().st_mtime_ns) if registry_probe else next(fingerprints)
meta = json.loads(fingerprint.read_text())
manifest = next(root.glob('✏️s/🔌*/🧩*/🗿*/🖐*/🏅*/🔖*/🪆*/✳*/✏️*/🧪*/🔬️unit-2/📦*/🦀*/Cargo.toml')).parent
toolchain = root/'.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin'
native = set()
artifacts = {}
for output in cache.glob('*/*/out/*'):
    if output.suffix == '.a':
        native.add(output.parent)
    if output.suffix not in ('.rlib','.so','.rmeta') or not output.name.startswith('lib'):
        continue
    link = libs/output.name
    try:
        link.symlink_to(output)
    except FileExistsError:
        pass
    if output.suffix == '.rmeta':
        continue
    stem = output.name.rsplit('-',1)[0][3:]
    fp = output.parent.parent/'fingerprint'/f'lib-{stem}'
    if fp.exists():
        artifacts[fp.read_text().strip()] = link
probe = '--window' in sys.argv
source = ticket/'ui-window-registry-probe.rs' if registry_probe else ticket/'current-window-probe.rs' if probe else manifest/'🦀️.rs'
executable = scratch/('window-registry-probe' if registry_probe else 'window-probe' if probe else 'tests')
args = [str(toolchain/'rustc'), str(source), '-Z','unstable-options','--edition=2021','--test','--crate-name',name,'-C','debuginfo=0','-C','linker-features=+lld','-C','link-self-contained=+linker','-L',f'dependency={libs}','-o',str(executable)]
dependencies = meta['deps']
if registry_probe:
    selected = {'semio_plugin', 'semio_os_kernel', 'semio_framework_job', 'semio_framework_async', 'semio_framework_async_macros', 'semio_framework_pack_json', 'semio_s_artifact_puzzle_5d_v1', 'semio_s_artifact_puzzle_5d_tests', 'semio_s_artifact_puzzle_5d_testkit'}
    dependencies = [entry for entry in dependencies if entry[1] in selected]
    plugin = next(entry for entry in dependencies if entry[1] == 'semio_plugin')
    plugin_fingerprint = artifacts[plugin[3].to_bytes(8,'little').hex()].resolve().parent.parent/'fingerprint/lib-semio_plugin.json'
    dependencies.append(next(entry for entry in json.loads(plugin_fingerprint.read_text())['deps'] if entry[1] == 'semio_plugin_window_config'))
    args.extend(['-Z','threads=1'])
for _, alias, _, checksum in dependencies:
    lib = artifacts[checksum.to_bytes(8,"little").hex()]
    args.extend(['--extern',f'{alias}={lib}'])
    if lib.with_suffix('.rmeta').is_file():
        args.extend(['--extern',f'{alias}={lib.with_suffix(".rmeta")}'])
for path in native:
    args.extend(['-L',f'native={path}'])
for feature in json.loads(meta['features']):
    args.extend(['--cfg',f'feature="{feature}"'])
env = {**os.environ, 'ZIG_GLOBAL_CACHE_DIR':str(gen/'zig'), 'ZIG_LOCAL_CACHE_DIR':str(gen/'zig-local'),'CARGO_MANIFEST_DIR':str(manifest), 'CARGO_PKG_VERSION':'0.1.0','CARGO_PKG_NAME':name.replace('_','-'),'RUST_MIN_STACK':'134217728','OUT_DIR':str(fingerprint.parent.parent/'out')}
subprocess.run(args, env=env,check=True)
sys.exit(subprocess.run([str(executable),'--nocapture','--test-threads=2',*[arg for arg in sys.argv[1:] if arg not in ('--window','--window-registry')]],env=env,cwd=manifest).returncode)
