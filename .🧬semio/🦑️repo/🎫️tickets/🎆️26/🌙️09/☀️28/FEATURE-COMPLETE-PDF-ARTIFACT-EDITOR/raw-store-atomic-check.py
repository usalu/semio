"""🏪 Links the guarded Store merger and paid original writer against the exact governed native declarations."""

import json
from pathlib import Path
import re
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
STAGE = runpy.run_path(str(TICKET / "raw-store-atomic-stage.py"))
ROOT = STAGE["ROOT"]
CACHE = Path(sys.argv[1]).resolve()
WASM = "--wasm" in sys.argv
OUT = TICKET / "🗑️generated/raw-store-atomic"
if WASM:
    OUT /= "w"
OUT.mkdir(parents=True, exist_ok=True)
STAGE["stage"]()
POST = json.loads(STAGE["OUT"].read_text(encoding="utf8"))["candidate"]
RUST = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
ARTIFACTS = {p.name: p for p in CACHE.glob("*/*/out/*") if p.suffix in (".rmeta", ".dylib", ".dll", ".so")}
if WASM:
    host = CACHE.parents[2] / "prototype/build"
    ARTIFACTS.update((p.name, p) for p in host.glob("*/*/out/*") if p.suffix in (".dylib", ".dll", ".so"))
DIRECTORIES = {p.parent for p in ARTIFACTS.values()}
SEARCH = OUT / "deps"
SEARCH.mkdir(exist_ok=True)
for name, path in ARTIFACTS.items():
    link = SEARCH / name
    if not link.exists():
        link.symlink_to(path)


def candidates(name):
    module = [p for key, p in ARTIFACTS.items() if key.startswith(f"lib{name}-") and key.endswith(".rmeta")]
    return module or [p for key, p in ARTIFACTS.items() if key.startswith(f"lib{name}-") and p.suffix in (".dylib", ".so", ".dll")]


def exact_baseline(name):
    [artifact] = candidates(name)
    content = subprocess.run([*RUST, "-Z", "ls=root", str(artifact)], check=True, capture_output=True, text=True).stdout
    identifiers = dict(re.findall(r"^\d+ ([\w]+)-([a-f0-9]+) hash ", content, re.M))
    return {name: ARTIFACTS[f"lib{name}-{digest}.rmeta"] for name, digest in identifiers.items()
            if f"lib{name}-{digest}.rmeta" in ARTIFACTS}


def compile(name, file, dependencies):
    answer = OUT / (f"lib{name}.rlib" if WASM else f"lib{name}.rmeta")
    mode = ["--target=wasm32-wasip2", "--emit=link"] if WASM else ["--emit=metadata"]
    args = ["--crate-name", name, "--crate-type=rlib", "--edition=2021", *mode, str(file),
            "-o", str(answer), f"-Ldependency={SEARCH}", f"-Ldependency={OUT}"]
    for alias, artifact in dependencies.items():
        args += ["--extern", f"{alias}={artifact}"]
        if WASM and artifact.suffix == ".rmeta" and artifact.with_suffix(".rlib").exists():
            args += ["--extern", f"{alias}={artifact.with_suffix('.rlib')}"]
    if WASM:
        args += [f"-Ldependency={path}" for path in sorted(DIRECTORIES)]
    response = OUT / f"{name}.args"
    response.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run([*RUST, f"@{response}"], check=True)
    return answer


def source(path, destination, substitutions):
    content = POST.get(str(path)) or (ROOT / path).read_text(encoding="utf8")
    value = re.sub(r'(#\[path\s*=\s*)"([^"]+)"', lambda m:
                   f'{m[1]}"{substitutions.get(m[2], (ROOT / path.parent / m[2]).resolve())}"', content)
    result = OUT / destination
    result.write_text(value, encoding="utf8")
    return result


def manifest_dependencies(directory, exact, override):
    path = directory / "📦️packages/🦀️rust/Cargo.toml"
    manifest = POST.get(str(path)) or (ROOT / path).read_text(encoding="utf8")
    content = manifest.split("[dependencies]", 1)[1].split("\n[", 1)[0]
    result = {}
    for row in content.splitlines():
        if "=" not in row:
            continue
        alias, value = row.split("=", 1)
        alias = alias.strip().removesuffix(".workspace")
        package = re.search(r'package\s*=\s*"([^"]+)"', value)
        name = (package[1] if package else alias).replace("-", "_")
        alias = alias.replace("-", "_")
        if name == "semio_framework_replication":
            name = alias = "protocol"
        if name == "semio_framework_pack" and "pack" in exact:
            name = alias = "pack"
        if alias in override:
            result[alias] = override[alias]
        elif name in exact:
            result[alias] = exact[name]
        else:
            options = candidates(name)
            if not options:
                print(f"[DEBUG] Optional unused dependency has no exact metadata artifact: {name}")
                continue
            [artifact] = options
            result[alias] = artifact
    return result


baseline = exact_baseline("semio_os_store_engine_bundle")
transport = exact_baseline("semio_os_store_operation_wire")

causal = OUT / "causal.rs"
if WASM:
    group = runpy.run_path(str(TICKET / "raw-spr-group-stage.py"))
    if (ROOT / group["MODULE"]).exists():
        POST.update({str(path): (ROOT / path).read_text(encoding="utf8") for path in (group["MODULE"], group["OWNER"])})
    else:
        group["stage"]()
        POST.update(json.loads(group["OUT"].read_text(encoding="utf8"))["candidate"])
    multipart = source(group["MODULE"], "multipart.rs", {})
    binary = source(group["OWNER"], "causal-binary.rs", {"🧩️atomic-group.rs": multipart})
    causal.write_text(f'pub use real_causal::*;\n#[path="{binary}"] pub mod binary;\n', encoding="utf8")
else:
    causal.write_text(f"""
pub use real_causal::*;
pub mod binary {{
    pub use real_causal::binary::*;
    #[path = "{TICKET / 'raw-spr-group.rs'}"] pub mod atomic_group;
}}
""", encoding="utf8")
bindings = {
    "real_causal": transport["semio_framework_replication_causal"], "semio_framework_value": transport["semio_framework_value"],
    "semio_framework_hash": transport["semio_framework_hash"],
}
if WASM:
    bindings.update(manifest_dependencies(group["MANIFEST"].parents[2], transport, {}))
causal_artifact = compile("causal_source_overlay", causal, bindings)

spr = OUT / "spr.rs"
if WASM:
    spr.write_text("pub use real_spr::*;\npub use causal_source_overlay::binary::{atomic_group, VerifiedAtomicEdit, decode_verified_atomic_backbone, decode_verified_atomic_backbone_at_document};\n", encoding="utf8")
    spr_inputs = {"real_spr": baseline["semio_os_spr"], "causal_source_overlay": causal_artifact}
else:
    spr.write_text("""
pub use real_spr::*;
pub struct VerifiedAtomicEdit { id: String, rows: Vec<MutationEnvelope> }
impl VerifiedAtomicEdit {
    pub fn edit_id(&self) -> &str { &self.id }
    pub fn events(&self) -> &[MutationEnvelope] { &self.rows }
}
""", encoding="utf8")
    spr_inputs = {"real_spr": baseline["semio_os_spr"]}
spr_artifact = compile("spr_source_overlay", spr, spr_inputs)
child = source(STAGE["ATOMIC"], "atomic-ingest.rs", {})
engine = source(STAGE["ENGINE"], "bundle.rs", {
    "🧩️atomic-ingest.rs": child,
    "📤️message/🦀️.rs": source(STAGE["MESSAGE_BUNDLE"], "bundle-message.rs", {}),
})
store_bundle = compile("store_bundle_overlay", engine, manifest_dependencies(STAGE["ENGINE"].parent, baseline, {"semio_os_spr": spr_artifact}))
wire_custody = source(STAGE["CUSTODY"], "wire-custody.rs", {})
wire_edit = source(STAGE["WRITER"], "wire-edit.rs", {"🧳️custody/🦀️.rs": wire_custody})
wire_root = STAGE["WRITER"].parent.parent / "🦀️.rs"
wire = source(wire_root, "operation-wire.rs", {
    "📤️edit/🦀️.rs": wire_edit,
    "📦️output/🦀️.rs": source(STAGE["OUTPUT"], "wire-output.rs", {}),
    "🧳️original/🦀️.rs": source(STAGE["ORIGINAL"], "wire-original.rs", {}),
    "📜️history/🦀️.rs": source(STAGE["HISTORY"], "wire-history.rs", {}),
    "📜️history/📤️batch/🦀️.rs": source(STAGE["BATCH"], "wire-batch.rs", {}),
})
store_wire = compile("store_operation_wire_overlay", wire, manifest_dependencies(wire_root.parent, transport, {"semio_framework_replication_causal": causal_artifact}))
print("[DEBUG] Type-checked the guarded Store group merger and original precommit wire writer")

if "--transport" in sys.argv:
    transport_stage = runpy.run_path(str(TICKET / "raw-spr-transport-stage.py"))
    transport_stage["stage"]()
    POST.update(json.loads(transport_stage["OUTPUT"].read_text(encoding="utf8"))["candidate"])
    backbone_root = transport_stage["BACKBONE"]
    inbox_links = {}
    if "--inbox-law" in sys.argv:
        inbox = runpy.run_path(str(TICKET / "raw-store-inbox-stage.py"))
        inbox["stage"]()
        POST.update(json.loads(inbox["OUT"].read_text(encoding="utf8"))["candidate"])
        inbox_links["📥️atomic-inbox.rs"] = source(inbox["INBOX"], "atomic-inbox.rs", {})
    native_transport = source(transport_stage["NATIVE"], "atomic-transport.rs", inbox_links)
    native_backbone = source(backbone_root, "store-backbone.rs", {"🧩️atomic-transport.rs": native_transport})
    baseline_backbone = exact_baseline("semio_os_store_backbone")
    backbone_artifact = compile("store_backbone_overlay", native_backbone,
        manifest_dependencies(backbone_root.parent, baseline_backbone, {"semio_os_spr": spr_artifact}))
    (real_backbone,) = candidates("semio_os_store_backbone")
    facade = OUT / "backbone-transport-facade.rs"
    facade_api = OUT / "atomic-transport-facade.rs"
    transport_content = native_transport.read_text(encoding="utf8")
    front = transport_content.index("pub fn take_atomic_front(")
    receiver = transport_content.index("/// 📥️ Holds one stable", front)
    facade_api.write_text(transport_content[:front] +
        "pub fn take_atomic_front(_: &Backbones) -> Result<Option<CausalPayload>, VcsError> { Ok(None) }\n\n" +
        transport_content[receiver:], encoding="utf8")
    facade.write_text(f'pub use real_backbone::*;\n#[path="{facade_api}"] pub mod atomic_transport;\n', encoding="utf8")
    backbone_facade = compile("store_backbone_transport_facade", facade, {
        "real_backbone": real_backbone, "semio_os_spr": spr_artifact,
        "semio_framework_value": baseline_backbone["semio_framework_value"], "semio_os_vcs": baseline_backbone["semio_os_vcs"],
    })
    cursor_source = transport_stage["STORE_CURSOR"]
    cursor = source(cursor_source, "store-purpose-cursor.rs", {})
    cursor_store = source(cursor_source.parents[1] / "🦀️.rs", "store-purpose-lifted.rs",
        {"🖱️store-cursor-operations/🦀️.rs": cursor})
    core_root = cursor_source.parents[3] / "🦀️.rs"
    core = source(core_root, "store-purpose-core.rs", {"📥️lifted/🏪️store/🦀️.rs": cursor_store})
    core_exact = exact_baseline("semio_os_store_engine_core")
    compile("store_engine_core_overlay", core, manifest_dependencies(core_root.parent, core_exact, {"semio_os_spr": spr_artifact}))
    store_driver = transport_stage["STORE_DRIVER"]
    purpose_wrapper = (TICKET / "raw-store-atomic-transaction-store.rs").read_text(encoding="utf8")
    POST[str(store_driver)] = POST[str(store_driver)].replace(purpose_wrapper, "")
    driver = source(store_driver, "atomic-store-driver.rs", {})
    pump = source(STAGE["CURSOR"], "atomic-store-pump.rs", {})
    lifted = source(store_driver.parents[1] / "🦀️.rs", "atomic-lifted-store.rs", {
        "📑️delegating-operations-and-publication/🦀️.rs": driver,
        "↩️supersede-undo-redo-and-backbone-pump/🦀️.rs": pump,
    })
    store_root = STAGE["ENGINE"].parents[1] / "🦀️.rs"
    store = source(store_root, "store-engine.rs", {"📥️lifted/🏪️store/🦀️.rs": lifted})
    store_exact = exact_baseline("semio_os_store_engine")
    compile("store_engine_overlay", store, manifest_dependencies(store_root.parent, store_exact, {
        "semio_os_spr": spr_artifact, "semio_os_store_backbone": backbone_facade,
        "semio_os_store_engine_bundle": store_bundle,
    }))
    print("[DEBUG] Type-checked actual atomic Channel and Port and Store delivery frontiers")

    if WASM and "--transport-law" in sys.argv:
        subprocess.run(["repo", "tools", "exec", "--light", "python", "--", str(TICKET / "raw-spr-transport-check.py")], check=True)
        law = OUT / "atomic-transport-law.wasm"
        args = ["--edition=2021", "--target=wasm32-wasip2", "-Copt-level=1", "-Awarnings",
                str(TICKET / "raw-store-atomic-backbone-law.rs"), "-o", str(law), f"-Ldependency={SEARCH}", f"-Ldependency={OUT}"]
        if "--inbox-law" in sys.argv:
            args += ["--cfg", "inbox"]
        args += [f"-Ldependency={path}" for path in sorted(DIRECTORIES)]
        for alias, artifact in (("store_backbone_overlay", backbone_artifact), ("spr_source_overlay", spr_artifact),
                                ("semio_framework_value", baseline_backbone["semio_framework_value"])):
            args += ["--extern", f"{alias}={artifact}"]
            if artifact.suffix == ".rmeta":
                args += ["--extern", f"{alias}={artifact.with_suffix('.rlib')}"]
        response = OUT / "atomic-transport-law.args"
        response.write_text("\n".join(args) + "\n", encoding="utf8")
        subprocess.run([*RUST, f"@{response}"], check=True)
        subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(law)], check=True)

if WASM and "--pdf-law" in sys.argv:
    graph = runpy.run_path(str(TICKET / "raw-png-graph-stage.py"))
    graph["stage"]()
    POST.update(json.loads(graph["PROBE"].read_text(encoding="utf8"))["candidate"])
    native = source(graph["OWNER"], "png-native.rs", {})
    entry = OUT / "pdf.rs"
    entry.write_text(f"""
pub mod standards {{
    pub mod v1_7 {{ pub mod subsets {{ pub mod base {{
        pub mod schema {{
            pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
        }}
        pub mod io {{
            pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::error;
        }}
    }}}}}}
}}
#[path = "{native}"] mod native;
#[path = "{TICKET / 'raw-store-atomic-pdf-law.rs'}"] mod law;
fn main() {{ law::run(); }}
""", encoding="utf8")
    binary = OUT / "store-pdf.wasm"
    args = ["--edition=2021", "--target=wasm32-wasip2", "-Copt-level=1", "-Awarnings", str(entry),
            "-o", str(binary), f"-Ldependency={SEARCH}", f"-Ldependency={OUT}"]
    args += [f"-Ldependency={path}" for path in sorted(DIRECTORIES)]
    dependencies = {
        "spr_source_overlay": spr_artifact, "store_bundle_overlay": store_bundle, "causal_source_overlay": causal_artifact,
        "store_operation_wire_overlay": store_wire,
    }
    for name in ("semio_os_store_document", "semio_os_store_engine", "semio_os_store_test_support", "semio_os_store_text_io",
                 "semio_os_store_space", "semio_os_store_engine_owners", "semio_os_store_initialization_runtime",
                 "semio_os_store_space_schema", "semio_os_vcs", "semio_framework_value",
                 "semio_framework_hash", "semio_framework_pack_json", "semio_framework_artifact_reference",
                 "semio_s_artifact_stdio_pdf_snapshot_2", "semio_s_artifact_stdio_pdf_inferences",
                 "semio_s_artifact_stdio_pdf_mut_family_8", "semio_s_artifact_stdio_pdf_mut_base_5"):
        dependency = baseline.get(name)
        if dependency is None:
            [dependency] = candidates(name)
        dependencies[name] = dependency
    for alias, artifact in dependencies.items():
        args += ["--extern", f"{alias}={artifact}"]
        if artifact.suffix == ".rmeta" and artifact.with_suffix(".rlib").exists():
            args += ["--extern", f"{alias}={artifact.with_suffix('.rlib')}"]
    response = OUT / "pdf-law.args"
    response.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run([*RUST, f"@{response}"], check=True)
    subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(binary),
                    "--jco-import", str(TICKET / "raw-store-atomic-stack.mjs")], check=True)
