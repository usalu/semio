import functools
import json
import os
import pathlib
import subprocess
import sys
import tomllib

ticket = pathlib.Path(__file__).resolve().parent
root = ticket.parents[6]
mounted = '--refresh-mounted' in sys.argv
variant = 'mounted' if mounted else 'frozen'
output = ticket / f'🗑️generated/ui-history-{variant}'
upper = ticket / '🗑️generated/builder/ov/u/debug/build'
lower = ticket.parent.parent / '☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target/b/debug/build'
puzzle = root / '✏️s/🔌️plugins/🧩️puzzle'
standard = puzzle / '🗿️artifacts/◻️2d/🏅️standards/🔖️1'
editor = standard / '🪆️subsets/✳️any/✏️editor'
test_manifest = editor / '🧪️tests/🧪️history-edit-runtime/📦️packages/🦀️rust'
jobs_manifest = puzzle / '🎮️commands/📤️jobs/📦️packages/🦀️rust'
test_name = 'semio_s_artifact_puzzle_2d_history_edit_runtime_tests'
test_fingerprint = upper / 'semio-s-artifact-puzzle-2d-history-edit-runtime-tests/868dd8f372688dc3/fingerprint' / f'test-lib-{test_name}.json'
rustc = root / '.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc'
retained_name = 'semio_s_artifact_puzzle_2d_retained'
fill_name = 'semio_s_artifact_puzzle_2d_tests'
jobs_name = 'semio_s_plugin_puzzle_jobs'
refresh_names = set()
if '--refresh-editor' in sys.argv:
    refresh_names.add(retained_name)
if '--refresh-fill' in sys.argv:
    refresh_names.add(fill_name)
if mounted:
    refresh_names.update({'semio_plugin_tool_run', 'semio_plugin_tool_run_turns', retained_name, fill_name, jobs_name})
sources = {
    retained_name: editor / '🧵️retained/📦️packages/🦀️rust',
    fill_name: standard / '🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🧪️tests/📦️packages/🦀️rust',
    'semio_s_artifact_puzzle_2d_v1': standard / '📦️packages/🦀️rust',
    'semio_s_artifact_puzzle_2d_unit_tests': editor / '🧪️tests/🔬️unit/📦️packages/🦀️rust',
}
if mounted:
    manifest_paths = subprocess.check_output(['rg', '--files', '-g', 'Cargo.toml', '✏️s', '🧰️framework'], cwd=root, text=True).splitlines()
    manifests = {}
    for manifest_path in manifest_paths:
        path = root / manifest_path
        package = tomllib.loads(path.read_text()).get('package', {})
        if 'name' in package:
            manifests[package['name'].replace('-', '_')] = path
output.mkdir(parents=True, exist_ok=True)
cached_fixture = ticket / '🗑️generated/ui-history-frozen'
libs = cached_fixture / 'deps'
libs.mkdir(parents=True, exist_ok=True)
native = set()
fingerprints = {}
artifacts = {}
index = cached_fixture / 'fingerprint-index.json'
if index.exists():
    saved = json.loads(index.read_text())
    artifacts = {key: (name, pathlib.Path(path)) for key, (name, path) in saved['artifacts'].items()}
    fingerprints = {key: pathlib.Path(path) for key, path in saved['fingerprints'].items()}
    native = set(map(pathlib.Path, saved['native']))
else:
    for cache in (upper, lower):
        for artifact in cache.glob('*/*/out/*'):
            if artifact.suffix == '.a':
                native.add(artifact.parent)
            if not artifact.name.startswith('lib') or artifact.suffix not in ('.rlib', '.rmeta', '.so'):
                continue
            dest = libs / artifact.name
            if not dest.exists():
                dest.symlink_to(artifact)
            if artifact.suffix == '.rmeta':
                continue
            name = artifact.name.rsplit('-', 1)[0][3:]
            fingerprint = artifact.parent.parent / 'fingerprint' / f'lib-{name}'
            if fingerprint.is_file():
                key = fingerprint.read_text().strip()
                fingerprints.setdefault(key, fingerprint.with_suffix('.json'))
                artifacts.setdefault(key, (name, dest))
    index.write_text(json.dumps({'artifacts': artifacts, 'fingerprints': fingerprints, 'native': sorted(native)}, default=str))


@functools.cache
def definition(fingerprint):
    return json.loads(fingerprint.read_text())


def dependencies(fingerprint):
    return {alias: checksum.to_bytes(8, 'little').hex() for _, alias, _, checksum in definition(fingerprint)['deps']}


def cached(key):
    return artifacts[key][1]


@functools.cache
def needs_refresh(key):
    return key in artifacts and (artifacts[key][0] in refresh_names or any(needs_refresh(child) for child in dependencies(fingerprints[key]).values()))


def compile_source(name, manifest, source, fingerprint, replacements, test=False, selected_deps=None):
    variant_key = f'{variant}_{fingerprint.parent.parent.name}' if mounted else variant
    library = f'lib{name}-history_{variant_key}.rlib' if mounted else f'lib{name}-history_probe.rlib'
    executable = 'history-tests' if name == test_name else f'{name}-tests'
    target = output / (executable if test else library)
    if not test and '--reuse-compiled' in sys.argv and target.exists():
        print(f'[DEBUG] Reusing refreshed {name} from this scratch directory', flush=True)
        return target
    args = [str(rustc), str(source), '--crate-name', name, '--edition=2021', '-Z', 'threads=1', '-Z', 'unstable-options',
            '-C', f'metadata=ui_history_{variant_key}', '-C', 'debuginfo=0', '-C', 'opt-level=1',
            '-C', 'linker-features=+lld', '-C', 'link-self-contained=+linker', '-L', f'dependency={libs}', '-L', f'dependency={output}', '-o', str(target)]
    args += ['--test'] if test else ['--crate-type', 'rlib']
    for feature in json.loads(definition(fingerprint)['features']):
        args += ['--cfg', f'feature="{feature}"']
    for alias, key in (selected_deps or dependencies(fingerprint)).items():
        artifact = replacements.get(alias) or cached(key)
        args += ['--extern', f'{alias}={artifact}']
        metadata = artifact.with_suffix('.rmeta')
        if metadata.exists():
            args += ['--extern', f'{alias}={metadata}']
    for directory in sorted(native):
        args += ['-L', f'native={directory}']
    environment = {**os.environ, 'CARGO_MANIFEST_DIR': str(manifest), 'CARGO_PKG_NAME': name.replace('_', '-'),
                   'CARGO_PKG_VERSION': '0.1.0', 'OUT_DIR': str(fingerprint.parent.parent / 'out'),
                   'ZIG_GLOBAL_CACHE_DIR': str(output / 'zig'), 'ZIG_LOCAL_CACHE_DIR': str(output / 'zig-local')}
    print(f'[DEBUG] Compiling {name} with {len(replacements)} refreshed dependencies', flush=True)
    result = subprocess.run(args, env=environment)
    if result.returncode:
        raise SystemExit(result.returncode)
    return target


def source_of(name):
    if mounted:
        manifest_file = manifests[name]
        manifest = manifest_file.parent
        path = tomllib.loads(manifest_file.read_text()).get('lib', {}).get('path', 'src/lib.rs')
        return manifest, manifest / path
    manifest = sources[name]
    return manifest, manifest / '🦀️.rs'


@functools.cache
def refresh(key):
    name = artifacts[key][0]
    fingerprint = fingerprints[key]
    manifest, source = source_of(name)
    replacements = {alias: refresh(child) for alias, child in dependencies(fingerprint).items() if needs_refresh(child)}
    return compile_source(name, manifest, source, fingerprint, replacements)


test_deps = dependencies(test_fingerprint)
if '--semantic-probe' in sys.argv:
    job = dependencies(fingerprints[test_deps['semio_framework_job']])
    name = 'semio_framework_job_context'
    context_key = job[name]
    manifest, source = source_of(name)
    clock_name = 'semio_framework_job_clock'
    clock_key = dependencies(fingerprints[context_key])[clock_name]
    clock_manifest, clock_source = source_of(clock_name)
    clock = compile_source(clock_name, clock_manifest, clock_source, fingerprints[clock_key], {})
    context = compile_source(name, manifest, source, fingerprints[context_key], {clock_name: clock})
    test = manifest.parent.parent / '🧪️tests/📊️native-progress/semantic-step.rs'
    probe = compile_source('semantic_probe', manifest, test, test_fingerprint, {name: context}, test=True,
                           selected_deps={name: context_key, 'serde_json': test_deps['serde_json']})
    raise SystemExit(subprocess.run([str(probe), '--nocapture', '--test-threads=1'], env={**os.environ, 'RUST_MIN_STACK': '134217728'}).returncode)
if any(flag in sys.argv for flag in ('--show-refresh-plan', '--compile-mounted-owners', '--checkpoint-probe')):
    reachable = set()

    def collect(key):
        if key in reachable or not needs_refresh(key):
            return
        reachable.add(key)
        for child in dependencies(fingerprints[key]).values():
            collect(child)

    for key in test_deps.values():
        collect(key)
if '--checkpoint-probe' in sys.argv:
    ledger_key, = (key for key in reachable if artifacts[key][0] == 'semio_plugin_tool_run')
    ledger_fingerprint = fingerprints[ledger_key]
    direct = dependencies(ledger_fingerprint)
    selected = {name: direct[name] for name in ('semio_framework_job', 'semio_framework_job_payload', 'semio_framework_value')}
    selected['semio_plugin_tool_run'] = ledger_key
    probe = compile_source('checkpoint_probe', ticket, ticket / 'ui-history-checkpoint-probe.rs', ledger_fingerprint,
                           {'semio_plugin_tool_run': refresh(ledger_key)}, test=True, selected_deps=selected)
    raise SystemExit(subprocess.run([str(probe), '--nocapture', '--test-threads=1'], env={**os.environ, 'RUST_MIN_STACK': '134217728'}).returncode)
if '--compile-mounted-owners' in sys.argv:
    owners = {'semio_plugin_tool_run', 'semio_plugin_tool_run_turns'}

    @functools.cache
    def compile_owner(key):
        name = artifacts[key][0]
        fingerprint = fingerprints[key]
        manifest, source = source_of(name)
        replacements = {alias: compile_owner(child) for alias, child in dependencies(fingerprint).items() if artifacts.get(child, ('',))[0] in owners}
        return compile_source(name, manifest, source, fingerprint, replacements)

    for key in sorted(reachable):
        if artifacts[key][0] in owners:
            compile_owner(key)
    raise SystemExit(0)
if '--show-refresh-plan' in sys.argv:
    for key in sorted(reachable, key=lambda key: artifacts[key][0]):
        name = artifacts[key][0]
        print(f'{name}: {manifests.get(name)}')
    raise SystemExit(0)
replacements = {}
if refresh_names:
    replacements.update({alias: refresh(key) for alias, key in test_deps.items() if needs_refresh(key)})
if '--refresh-fill-only' in sys.argv:
    replacements[fill_name] = refresh(test_deps[fill_name])
if not mounted:
    jobs_key = test_deps[jobs_name]
    replacements[jobs_name] = compile_source(jobs_name, jobs_manifest, jobs_manifest.parent.parent / '🦀️.rs', fingerprints[jobs_key], {})
executable = compile_source(test_name, test_manifest, test_manifest / '🦀️.rs', test_fingerprint, replacements, test=True)
filters = [arg for arg in sys.argv[1:] if not arg.startswith('--')] or ['create_edge::tests', 'engagement_submit::tests', 'precompute::fill::tests']
raise SystemExit(subprocess.run([str(executable), '--nocapture', '--test-threads=1', *filters], env={**os.environ, 'RUST_MIN_STACK': '134217728'}).returncode)
