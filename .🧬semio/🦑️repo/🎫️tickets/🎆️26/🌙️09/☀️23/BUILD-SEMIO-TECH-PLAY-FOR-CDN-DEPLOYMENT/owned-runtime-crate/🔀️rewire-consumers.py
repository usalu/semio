"""🔀️ Rewires every consumer of the moved owned-runtime vocabulary onto `semio-framework-plugin-interpreter`: import paths,
the owned compile API, manifests, profiles and the source-text gates that read the moved code. Asserts every match count
against the CURRENT source and writes nothing unless `--write` is passed and every edit matched."""
import pathlib, re, sys

REPO = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
MOVED = {"OwnedRuntime", "TurnFault", "PluginHostError", "GuestCallCancellation", "JobBudget", "JobStep", "GuestDocumentPair", "GuestDocumentMirror", "sqlite_wire", "owned_engine_identity", "interpreter", "IoRouteConflict", "IoEntryRouteConflict", "OwnedCompiledGuest"}
writes = {}


def text_of(path):
    return writes.get(path, None) or path.read_text()


def edit(path, pairs):
    text = text_of(path)
    for old, new, count in pairs:
        found = text.count(old)
        assert found == count, f"{path}: {found} != {count}: {old[:150]!r}"
        text = text.replace(old, new)
    writes[path] = text


def repath(path, expected):
    text = text_of(path)

    def split_list(match):
        names = [name.strip() for name in match.group(1).split(",") if name.strip()]
        moved = [name for name in names if name.split(" as ")[0].strip() in MOVED]
        kept = [name for name in names if name not in moved]
        if not moved:
            return match.group(0)
        sep = ", " if ", " in match.group(1) else ","
        parts = []
        if kept:
            parts.append(f"use semio_framework_plugin_host::{{{sep.join(kept)}}};")
        parts.append(f"use semio_framework_plugin_interpreter::{{{sep.join(moved)}}};" if len(moved) > 1 else f"use semio_framework_plugin_interpreter::{moved[0]};")
        return " ".join(parts) if "\n" not in match.group(0) else "\n".join(parts)

    text, lists = re.subn(r"use semio_framework_plugin_host::\{([^}]*)\};", split_list, text)
    text, paths = re.subn(r"semio_framework_plugin_host::(" + "|".join(sorted(MOVED, key=len, reverse=True)) + r")\b", r"semio_framework_plugin_interpreter::\1", text)
    remaining = len(re.findall(r"semio_framework_plugin_host::\{[^}]*\b(" + "|".join(MOVED) + r")\b", text))
    assert remaining == 0, f"{path}: moved names remain in a host import list"
    assert text.count("semio_framework_plugin_interpreter") >= expected, f"{path}: {text.count('semio_framework_plugin_interpreter')} < {expected}"
    writes[path] = text


hub = REPO / "🌎️hub"
consumers = {
    hub / "🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs": 9,
    hub / "🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs": 3,
    hub / "🧪️tests/🔏️trusted-catalog-profile/🪶️count/🦀️.rs": 1,
    hub / "🧪️tests/🧩️component-codecs/🦀️.rs": 1,
    hub / "🧩️compositions/🖍️draw/🧪️tests/🌐️guest/🦀️.rs": 1,
    REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs": 1,
    REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🧪️tests/🔬️unit/🦀️.rs": 1,
    REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs": 1,
    REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs": 2,
    REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs": 8,
    PLUGIN / "🖥️host/🧵️shard/👶️child/🦀️.rs": 2,
    REPO / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🌐️guest/🦀️.rs": 1,
    REPO / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🌐️guest/🦀️.rs": 1,
}
for path, expected in consumers.items():
    repath(path, expected)

catalog = hub / "🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"
edit(catalog, [
    ("semio_framework_plugin_host::CompiledHandle", "semio_framework_plugin_interpreter::OwnedCompiledGuest", 5),
    ("                let (runtime, package) = (Arc::clone(&self.runtime), self.package.clone());\n                let compiled = interpret_off_worker(context, move |_handle, _progress, _cancellation| runtime.compile_component(&package, &bytes).map_err(semio_framework_plugin_interpreter::TurnFault::Host))",
     "                let runtime = Arc::clone(&self.runtime);\n                let compiled = interpret_off_worker(context, move |_handle, _progress, _cancellation| runtime.compile_component(&bytes).map_err(semio_framework_plugin_interpreter::TurnFault::Host))", 1),
])
count = hub / "🧪️tests/🔏️trusted-catalog-profile/🪶️count/🦀️.rs"
edit(count, [
    ("use semio_framework_plugin_host::{GuestRuntime,CompiledHandle,PackageRef,PackageId,PackageHash,Budget}; use semio_framework_plugin_interpreter::OwnedRuntime;", "use semio_framework_plugin_host::Budget;\nuse semio_framework_plugin_interpreter::{OwnedCompiledGuest,OwnedRuntime};", 1),
    ("pub codec_runtime:Arc<OwnedRuntime>,pub compiled:CompiledHandle}", "pub codec_runtime:Arc<OwnedRuntime>,pub compiled:OwnedCompiledGuest}", 1),
    (" let runtime=Arc::new(OwnedRuntime::new());let package=PackageRef{package:PackageId(\"semio:neutral-host-fixture\".into()),hash:PackageHash(*semio_framework_hash::hash(component).as_bytes())};\n let compiled=runtime.compile(&package,component).await.map_err(", " let runtime=Arc::new(OwnedRuntime::new());\n let compiled=runtime.compile_component(component).map_err(", 1),
])
codecs = hub / "🧪️tests/🧩️component-codecs/🦀️.rs"
edit(codecs, [("let runtime=OwnedRuntime::new();let compiled=runtime.compile(&package_ref(\"semio:gis\",&bytes),&bytes).await.unwrap();", "let runtime=OwnedRuntime::new();let compiled=runtime.compile_component(&bytes).unwrap();", 1)])
note = REPO / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🌐️guest/🦀️.rs"
note_text = text_of(note)
for name in ["owned_codec_genesis_answers_on_a_real_plugin_component", "owned_codec_genesis_answers_by_artifact_kind_too", "owned_codec_pack_schema_hash_answers_on_a_real_plugin_component", "owned_codec_print_mirror_round_trips_a_genesis_pair"]:
    start = note_text.index(f"async fn {name}() {{")
    end = note_text.index("\n}\n", start)
    body = note_text[start:end]
    assert body.count('let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");') == 1, name
    note_text = note_text[:start] + body.replace('let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");', 'let compiled = runtime.compile_component(&bytes).expect("compile plugin component");') + note_text[end:]
writes[note] = note_text
edit(note, [('let owned_compiled = owned.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");', 'let owned_compiled = owned.compile_component(&bytes).expect("compile plugin component");', 1)])

describe = PLUGIN / "🖨️describe/🛂️descriptor-emission/🦀️.rs"
edit(describe, [
    ("use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, OwnedRuntime, PackageHash, PackageId, PackageRef, TurnFault};\n/// 🧠️ The one owned interpreter, compiled once in the plugin host that the emitter drives; the owned core-export law reads\n/// its export set.\n#[cfg(test)]\nuse semio_framework_plugin_host::interpreter;\n",
     "use semio_framework_plugin_interpreter::{OwnedCompiledGuest, OwnedRuntime, TurnFault};\n/// 🧠️ The owned interpreter the emitter drives; the owned core-export law reads its export set.\n#[cfg(test)]\nuse semio_framework_plugin_interpreter::interpreter;\n", 1),
    ("async fn execute_describe_owned(wasm_bytes: &[u8], source: &Path) -> Result<(Vec<u8>, OwnedRuntime, CompiledHandle), DescribeError> {", "async fn execute_describe_owned(wasm_bytes: &[u8], source: &Path) -> Result<(Vec<u8>, OwnedRuntime, OwnedCompiledGuest), DescribeError> {", 1),
    ("    let package = PackageRef { package: PackageId(source.display().to_string()), hash: PackageHash([0; 32]) };\n    let compiled = runtime.compile(&package, wasm_bytes).await.map_err(", "    let compiled = runtime.compile_component(wasm_bytes).map_err(", 1),
    ("async fn first_owned_codec(runtime: &OwnedRuntime, compiled: &CompiledHandle, pairs:", "async fn first_owned_codec(runtime: &OwnedRuntime, compiled: &OwnedCompiledGuest, pairs:", 1),
    ("    let package = PackageRef { package: PackageId(wasm_path.display().to_string()), hash: PackageHash([0; 32]) };\n    let compiled = runtime.compile(&package, &wasm_bytes).await.map_err(", "    let compiled = runtime.compile_component(&wasm_bytes).map_err(", 1),
])

admission = PLUGIN / "🖥️host/🎟️context/🤝️admission/🧪️tests/🟦️.ts"
edit(admission, [
    ('const host=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8"),', 'const host=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8"),runtime=readFileSync(new URL("../../../../🧠️interpreter/🏃️runtime/🦀️.rs",import.meta.url),"utf8"),', 1),
    ('expect(host.includes("ActorCall(crate::reactor::original_actor_context::OriginalActorFixedCall)")).toBe(true)', 'expect(runtime.includes("ActorCall(original_actor_context::OriginalActorFixedCall)")).toBe(true)', 1),
])
host_script = PLUGIN / "🖥️host/📦️packages/🦀️rust/📜️script.ts"
edit(host_script, [
    ('  const runtime = readFileSync(join(hostRoot, "⏳️runtime", "🦀️.rs"), "utf8");\n', '  const runtime = readFileSync(join(hostRoot, "⏳️runtime", "🦀️.rs"), "utf8");\n  const owned = readFileSync(join(hostRoot, "..", "🧠️interpreter", "🏃️runtime", "🦀️.rs"), "utf8");\n', 1),
    ('  assert(host.includes("Guest(semio_framework::Fault)"), "typed guest fault must survive host decode");', '  assert(owned.includes("Guest(semio_framework::Fault)"), "typed guest fault must survive host decode");', 1),
    ('  assert(host.includes("result.map_err(|error| decode_guest_fault_bytes(&error))"), "owned ABI must use typed decoder");', '  assert(owned.includes("result.map_err(|error| decode_guest_fault_bytes(&error))"), "owned ABI must use typed decoder");', 1),
])
guest_script = PLUGIN / "📦️packages/🦀️rust/📜️script.ts"
edit(guest_script, [
    ('"operation::input::tests::original_snapshot_input_cancellation_retains_actual_json_backing","snapshot_call::tests::original_snapshot_pending_fuel_preserves_original_completed_ceiling"]}]',
     '"snapshot_call::tests::original_snapshot_pending_fuel_preserves_original_completed_ceiling"]},{package:"semio-framework-plugin-interpreter",target:{kind:"lib"},laws:["operation::input::tests::original_snapshot_input_cancellation_retains_actual_json_backing"]}]', 1),
])

root = REPO / "Cargo.toml"
edit(root, [
    ('    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust",\n', '    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust",\n    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust",\n', 1),
    ('semio-framework-plugin-host = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }\n', 'semio-framework-plugin-host = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }\nsemio-framework-plugin-interpreter = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust" }\n', 1),
    ("# 🧮️ The owned wasm interpreter (`semio-framework-plugin-host::interpreter`) runs every trusted-catalog", "# 🧮️ The owned wasm interpreter (`semio-framework-plugin-interpreter`) runs every trusted-catalog", 1),
    ("[profile.dev.package.semio-framework-plugin-host]\nopt-level = 3\n", "[profile.dev.package.semio-framework-plugin-interpreter]\nopt-level = 3\n", 1),
])
hub_workspace = hub / "Cargo.toml"
edit(hub_workspace, [
    ('semio-framework-plugin-host = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }\n', 'semio-framework-plugin-host = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }\nsemio-framework-plugin-interpreter = { path = "../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust" }\n', 1),
    ("# 🧮️ The owned wasm interpreter (`semio-framework-plugin-host::interpreter`) runs every trusted-catalog", "# 🧮️ The owned wasm interpreter (`semio-framework-plugin-interpreter`) runs every trusted-catalog", 1),
    ("[profile.dev.package.semio-framework-plugin-host]\nopt-level = 3\n", "[profile.dev.package.semio-framework-plugin-interpreter]\nopt-level = 3\n", 1),
])
manifests = [
    (hub / "📦️packages/🦀️rust/Cargo.toml", 'semio-framework-plugin-host = { path = "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }\n', 'semio-framework-plugin-interpreter = { workspace = true }\n'),
    (hub / "🧩️compositions/🖍️draw/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-plugin-host.workspace = true\n', 'semio-framework-plugin-interpreter.workspace = true\n'),
    (REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-plugin-host = { path = "../../../🔌️plugin/🖥️host/📦️packages/🦀️rust", package = "semio-framework-plugin-host" }\n', 'semio-framework-plugin-interpreter = { workspace = true }\n'),
    (REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-plugin-host = { path = "../../../🔌️plugin/🖥️host/📦️packages/🦀️rust", package = "semio-framework-plugin-host" }\n', 'semio-framework-plugin-interpreter = { workspace = true }\n'),
    (REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-plugin-host = { path = "../../../../../../🔌️plugin/🖥️host/📦️packages/🦀️rust", package = "semio-framework-plugin-host" }\n', 'semio-framework-plugin-interpreter = { workspace = true }\n'),
    (REPO / "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-plugin-host = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust", package = "semio-framework-plugin-host" }\n', 'semio-framework-plugin-interpreter = { path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/📦️packages/🦀️rust" }\n'),
    (PLUGIN / "🖥️host/📦️packages/🦀️rust/Cargo.toml", 'semio-framework-os-config = { workspace = true }\n', 'semio-framework-plugin-interpreter = { workspace = true }\n'),
]
for path, anchor, line in manifests:
    edit(path, [(anchor, anchor + line, 1)])
describe_manifest = PLUGIN / "🖨️describe/📦️packages/🦀️rust/Cargo.toml"
edit(describe_manifest, [
    ('semio-framework-plugin-host = { path = "../../../🖥️host/📦️packages/🦀️rust" }\n', '# 🧠️ The owned interpreter and its guest runtime: the emitter runs every component under it and links no plugin host.\nsemio-framework-plugin-interpreter = { workspace = true }\n', 1),
])

if "--write" not in sys.argv:
    for path in writes:
        print(f"[dry-run] would write {path.relative_to(REPO)}")
    sys.exit(0)
for path, text in writes.items():
    path.write_text(text)
print(f"[rewire-consumers] wrote {len(writes)} files")
