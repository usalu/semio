import json
import os
import pathlib
import subprocess
import sys

ticket = pathlib.Path(__file__).resolve().parent
root = ticket.parents[6]
case = sys.argv[1] if len(sys.argv) > 1 else 'lookup'
output = ticket / f'🗑️generated/ui-{case}-frozen'
upper = ticket / '🗑️generated/builder/ov/u/debug/build'
lower = ticket.parent.parent / '☀️18/DEV-PUZZLE-5D-FULLY-WORKING-END-TO-END/🗑️generated/native-target/b/debug/build'
lookup = root / '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup'
manifest = lookup.parents[1] / '📦️packages/🦀️rust'
test = upper / 'semio-s-artifact-puzzle-2d-schema/95ec9f470fab5167/fingerprint/test-lib-semio_s_artifact_puzzle_2d_schema.json'
if case == 'seed-board':
    test = upper / 'semio-s-artifact-puzzle-2d-history-edit-runtime-tests/868dd8f372688dc3/fingerprint/test-lib-semio_s_artifact_puzzle_2d_history_edit_runtime_tests.json'
rustc = root / '.cache/b/t/m/r/699c2d5d27a9c904/toolchains/nightly-2026-07-20-x86_64-unknown-linux-gnu/bin/rustc'
deps = {row[1]: row[3].to_bytes(8, 'little').hex() for row in json.loads(test.read_text())['deps']}
required = ('semio_s_artifact_puzzle_2d_snapshot', 'semio_s_artifact_puzzle_2d_records', 'semio_framework_value', 'semio_framework_test_fixtures', 'semio_framework_trace', 'serde_json')
if case == 'declared-json':
    required += ('semio_framework_diagnostic', 'semio_framework_pack_json', 'semio_os_kernel', 'semio_s_artifact_stdio_json')
elif case == 'seed-board':
    required = ('semio_s_artifact_puzzle_2d_v1', 'semio_framework_dsl_record', 'semio_framework_value')
output.mkdir(parents=True, exist_ok=True)
libs = output / 'deps'
libs.mkdir(exist_ok=True)
native = set()
externs = {}
for cache in (upper, lower):
    for artifact in cache.glob('*/*/out/*'):
        if artifact.suffix == '.a':
            native.add(artifact.parent)
        if not artifact.name.startswith('lib') or artifact.suffix not in ('.rlib', '.rmeta', '.so'):
            continue
        dest = libs / artifact.name
        if not dest.exists():
            dest.symlink_to(artifact)
        name = artifact.name.rsplit('-', 1)[0][3:]
        if name not in required or artifact.suffix != '.rlib':
            continue
        fingerprint = artifact.parent.parent / 'fingerprint' / f'lib-{name}'
        if fingerprint.is_file() and fingerprint.read_text() == deps[name]:
            externs.setdefault(name, dest)

assert externs.keys() >= set(required), set(required) - externs.keys()
source = output / 'harness.rs'
if case == 'lookup':
    test_filter = 'history_edit_puzzle2d_native_lookup'
    harness = f'''use semio_s_artifact_puzzle_2d_snapshot::Puzzle2dSnapshot;
pub mod standards {{
    pub mod v1 {{
        pub mod subsets {{
            pub mod any {{
                pub mod schema {{
                    pub use crate::lookup;
                    pub use semio_s_artifact_puzzle_2d_records::standards::v1::subsets::any::schema::{{records, testkit}};
                }}
            }}
        }}
    }}
}}
#[path = {json.dumps(str(lookup / '🦀️.rs'), ensure_ascii=False)}] pub mod lookup;
#[path = {json.dumps(str(lookup / '🧪️tests/🦀️.rs'), ensure_ascii=False)}] mod tests;
'''
elif case == 'declared-json':
    test_filter = 'declared_json'
    io = lookup.parents[2] / '🚪️io'
    converter = json.dumps(str(io / '🔣️json/🦀️.rs'), ensure_ascii=False)
    importer = json.dumps(str(io / '📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs'), ensure_ascii=False)
    exporter = json.dumps(str(io / '📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs'), ensure_ascii=False)
    fixture = json.dumps(str(io / '🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json'), ensure_ascii=False)
    harness = f'''extern crate semio_os_kernel as store;
use semio_s_artifact_puzzle_2d_snapshot::Puzzle2dSnapshot;
#[path = {converter}] pub mod puzzle2d_json;
#[path = {importer}] pub mod json_input;
#[path = {exporter}] pub mod json_output;
pub mod standards {{ pub mod v1 {{ pub mod subsets {{ pub mod any {{ pub mod io {{
    pub use crate::puzzle2d_json;
    pub mod import {{ pub mod deserializers {{ pub mod artifacts {{ pub mod json {{
        pub mod v_rfc8259 {{ pub use crate::json_input as any; }}
    }} }} }} }}
}} }} }} }} }}
#[test]
fn declared_json_roundtrips_all_authored_binary64_slots() {{
    use store::os_store::serialization::ArtifactSqliteSnapshotHost as _;
    let laws: serde_json::Value = serde_json::from_str(include_str!({fixture})).unwrap();
    let slots = laws["declaredJson"]["floatPointers"].as_array().unwrap();
    assert_eq!(slots.len(), 43);
    for word in laws["binary64Words"].as_array().unwrap() {{
        let mut wire = laws["snapshot"].clone();
        for slot in slots {{
            *wire.pointer_mut(slot.as_str().unwrap()).unwrap() = serde_json::json!({{"bits": word}});
        }}
        let snapshot = json_input::deserialize_bytes(&serde_json::to_vec(&wire).unwrap()).unwrap();
        let bytes = json_output::serialize_bytes(&snapshot);
        snapshot.retire_sqlite_snapshot();
        let actual: serde_json::Value = serde_json::from_slice(&bytes.unwrap()).unwrap();
        assert_eq!(actual, wire, "all slots under word {{word}}");
    }}
}}
'''
elif case == 'seed-board':
    test_filter = 'seed_board'
    harness = '''use semio_framework_dsl_record::{DslField, native_encoding::FieldProjectionView};
use semio_s_artifact_puzzle_2d_v1::standards::v1::subsets::any::schema::{
    mutations::{connect_kind_compatibility::{ConnectKindCompatibility, connect_kind_compatibility}, create_node::{CreateNode, create_node}},
    records::{Puzzle2dCompatSpecificity, Puzzle2dNode},
};

#[test]
fn seed_board_projections_identify_the_first_refused_field() {
    let compatibility: ConnectKindCompatibility = connect_kind_compatibility("a".into(), "a".into(), true, false, Puzzle2dCompatSpecificity::Handle, None);
    assert!(matches!(compatibility.projection_view(&[]), Ok(FieldProjectionView::Record(fields)) if fields.first() == Some(&0)));
    let source = compatibility.projection_view(&[0]).err().unwrap();
    assert_eq!(source, compatibility.source.projection_view(&[]).err().unwrap());
    let node: CreateNode = create_node(Puzzle2dNode { id: "left".into(), ..Default::default() }, None);
    assert!(matches!(node.projection_view(&[0]), Ok(FieldProjectionView::Block)));
    assert!(matches!(node.projection_view(&[0, 0]), Ok(FieldProjectionView::Record(fields)) if fields.first() == Some(&0)));
    let id = node.projection_view(&[0, 0, 0]).err().unwrap();
    assert_eq!(id, node.node.id.projection_view(&[]).err().unwrap());
    assert_eq!(source, id);
    assert!(source.to_string().contains("field owner has no retained source projection"));
}
'''
else:
    raise SystemExit(f'Unknown native probe {case}')
source.write_text(harness)
binary = output / 'lookup-test'
args = [str(rustc), str(source), '--test', '--edition=2021', '-Z', 'threads=1', '-C', 'debuginfo=0', '-L', f'dependency={libs}', '-o', str(binary)]
for name, artifact in externs.items():
    args += ['--extern', f'{name}={artifact}']
    metadata = artifact.with_suffix('.rmeta')
    if metadata.exists():
        args += ['--extern', f'{name}={metadata}']
for directory in sorted(native):
    args += ['-L', f'native={directory}']
print(f'[DEBUG] Linking current {case} source to {len(externs)} cached direct dependencies', flush=True)
result = subprocess.run(args, env={**os.environ, 'CARGO_MANIFEST_DIR': str(manifest), 'ZIG_GLOBAL_CACHE_DIR': str(output / 'zig'), 'ZIG_LOCAL_CACHE_DIR': str(output / 'zig-local')})
if result.returncode:
    raise SystemExit(result.returncode)
raise SystemExit(subprocess.run([str(binary), test_filter, '--nocapture']).returncode)
