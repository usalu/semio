"""🔬️ Type-checks the exact staged DB contract, Sync, recovery, submit and ledger packages."""

import json
from pathlib import Path
import re
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/raw-hub-atomic/db"
STAGE = runpy.run_path(str(TICKET / "raw-hub-socket-stage.py"))
STAGE["stage"]()
ROOT = STAGE["ROOT"]
POST = json.loads(STAGE["OUT"].read_text(encoding="utf8"))["candidate"]
ATOMIC = runpy.run_path(str(TICKET / "raw-hub-atomic-stage.py"))
ADMISSION = runpy.run_path(str(TICKET / "raw-hub-paged-db-admission.py"))
RUST = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
CACHE = Path(sys.argv[1]).resolve()
ARTIFACTS = {p.name: p for p in CACHE.rglob("*") if p.suffix in (".rmeta", ".dylib", ".so", ".dll") and p.name.startswith("lib")}
OUT.mkdir(parents=True, exist_ok=True)
SEARCH = OUT / "deps"
SEARCH.mkdir(exist_ok=True)
for name, path in ARTIFACTS.items():
    link = SEARCH / name
    if not link.exists():
        link.symlink_to(path)


def exact(crate):
    choices = [p for filename, p in ARTIFACTS.items() if filename.startswith(f"lib{crate}-") and p.suffix == ".rmeta"]
    assert len(choices) == 1, (crate, choices)
    primary = choices[0]
    metadata = subprocess.run([*RUST, "-Z", "ls=root", str(primary)], check=True, capture_output=True, text=True).stdout
    identities = re.findall(r"^\d+ ([\w]+)-([a-f0-9]+) hash ", metadata, re.M)
    externs = {name: ARTIFACTS[f"lib{name}-{digest}.rmeta"] for name, digest in identities if f"lib{name}-{digest}.rmeta" in ARTIFACTS}
    return primary, externs


def compile(name, path, externs):
    output = OUT / f"lib{name}.rmeta"
    args = ["--crate-name", name, "--crate-type=rlib", "--edition=2021", "--emit=metadata", str(path), "-o", str(output),
            f"-Ldependency={SEARCH}", f"-Ldependency={OUT}"]
    for alias, artifact in externs.items():
        args.extend(["--extern", f"{alias}={artifact}"])
    response = OUT / f"{name}.args"
    response.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run([*RUST, f"@{response}"], check=True)
    return output


def source(path, target, mapped=None):
    mapped = mapped or {}
    result = OUT / target
    code = POST.get(str(path)) or (ROOT / path).read_text(encoding="utf8")
    code = re.sub(r'(#\[path\s*=\s*)"([^"]+)"', lambda m: f'{m[1]}"{mapped.get(m[2], ROOT / path.parent / m[2])}"', code)
    result.write_text(code, encoding="utf8")
    return result


def package(path, kind, override):
    manifest = (ROOT / path.parent / "📦️packages/🦀️rust/Cargo.toml").read_text(encoding="utf8")
    keys = manifest.split("[dependencies]", 1)[1].split("\n[", 1)[0]
    try:
        _, baseline = exact(kind)
    except AssertionError:
        baseline = {}
    scope = {}
    for line in keys.splitlines():
        if "=" not in line:
            continue
        key = line.split("=", 1)[0].strip().removesuffix(".workspace")
        alias = key.replace("-", "_")
        if key == "semio-framework-replication":
            alias = "protocol"
        elif key == "semio-framework-pack" and "pack" in baseline:
            alias = "pack"
        elif key.startswith("semio-os-kernel-db-"):
            alias = key.removeprefix("semio-os-kernel-").replace("-", "_")
        if alias in override:
            scope[alias] = override[alias]
        elif alias in baseline:
            scope[alias] = baseline[alias]
        else:
            choices = [value for filename, value in ARTIFACTS.items() if filename.startswith(f"lib{alias}-")]
            assert len(choices) == 1, (kind, alias, choices)
            scope[alias] = choices[0]
    return scope


causal = ATOMIC["CAUSAL"]
hub = source(ATOMIC["HUB_CODEC"], "hub.rs")
group = source(causal.with_name("🧩️atomic-group.rs"), "group.rs")
binary = source(causal, "binary.rs", {"🏪️atomic-hub.rs": hub, "🧩️atomic-group.rs": group})
facade = OUT / "causal.rs"
facade.write_text('pub use real_causal::*;\n#[path="binary.rs"] pub mod binary;\n', encoding="utf8")
real_causal, exact_causal = exact("semio_framework_replication_causal")
causal_deps = package(ATOMIC["PROTOCOL"] / "🔗️causal/🦀️.rs", "semio_framework_replication_causal", {})
causal_deps["real_causal"] = real_causal
causal_api = compile("hub_causal", facade, causal_deps)
protocol = OUT / "protocol.rs"
protocol.write_text("pub use real_protocol::*;\npub use hub_causal::binary::{atomic_hub, atomic_group, VerifiedAtomicEdit};\n", encoding="utf8")
real_protocol, _ = exact("protocol")
protocol_api = compile("hub_protocol", protocol, {"real_protocol": real_protocol, "hub_causal": causal_api})

sync = ATOMIC["SYNC_LIB"]
hello = sync.parent / "🎞️hello-replay-tail-and-execution/🦀️.rs"
sync_code = source(sync, "sync.rs", {
    "🪞️replica-sync-protocol/🦀️.rs": source(ATOMIC["SYNC"], "replay.rs"),
    "🎟️hello-admission-and-backing/🦀️.rs": source(ADMISSION["BACKING"], "backing.rs"),
    "🎞️hello-replay-tail-and-execution/🦀️.rs": source(hello, "hello.rs"),
})
contract = ATOMIC["CONTRACT"].parent.parent / "🦀️.rs"
contract_code = source(contract, "contract.rs", {
    "🏷️ids-command-and-messages/🦀️.rs": source(ATOMIC["CONTRACT"], "commands.rs"),
    "🧾️receipts-and-checkpoints/🦀️.rs": source(ADMISSION["RECEIPTS"], "receipts.rs"),
    "🛡️durable-group-recovery/🦀️.rs": source(ADMISSION["RECOVERY"], "recovery.rs"),
    "🗺️pathmap-diff-and-bridge/🦀️.rs": source(ADMISSION["PATHMAP"], "pathmap.rs"),
})
engine = ATOMIC["SUBMIT"].parent.parent / "🦀️.rs"
engine_code = source(engine, "engine.rs", {
    "🚦️engine-submit-pipeline/🦀️.rs": source(ATOMIC["SUBMIT"], "submit.rs"),
    "🌱️engine-lifecycle/🦀️.rs": source(ATOMIC["LIFECYCLE"], "lifecycle.rs"),
    "⚙️config-index-backlog-and-open-rejection/🦀️.rs": source(ATOMIC["CONFIG"], "config.rs"),
    "📈️ledger-tail/🦀️.rs": source(ATOMIC["LEDGER"], "ledger.rs"),
    "↩️engine-undo-durable-group-query-and-preview/🦀️.rs": source(ADMISSION["UNDO"], "undo.rs"),
})
alias = {"protocol": protocol_api}
sync_api = compile("hub_db_sync", sync_code, package(sync, "db_sync", alias))
contract_api = compile("hub_db_contract", contract_code, package(contract, "db_artifact_contract", alias))
compile("hub_db_engine", engine_code, package(engine, "db_artifact_engine",
        {"db_sync": sync_api, "db_artifact_contract": contract_api, **alias}))
print("[DEBUG] Type-checked the actual grouped DB command, submit, restart, streaming hello and ledger cuts against governed native metadata")
