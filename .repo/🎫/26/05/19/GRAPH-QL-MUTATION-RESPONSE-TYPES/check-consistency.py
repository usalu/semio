"""Fail if legacy GraphQL/kit patterns reappear in semio client sources."""
import re
from pathlib import Path

def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "semio" / "client" / "lib" / "rs" / "lib.rs").is_file():
            return parent
    raise RuntimeError("repo root not found from check-consistency.py")


ROOT = _repo_root()

def has_ops_fallback_in_lib_rs(text: str) -> bool:
    for line in text.splitlines():
        if '.get("ops")' not in line:
            continue
        if ".is_none()" in line or "is_none()" in line:
            continue
        return True
    return False


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


errors: list[str] = []

lib_rs = ROOT / "semio/client/lib/rs/lib.rs"
if not lib_rs.is_file():
    errors.append(f"missing file: {lib_rs}")
else:
    text = lib_rs.read_text(encoding="utf-8")
    needles = [
        ("stub_ok", "stub_ok"),
        ("(@names) =>", "(@names) =>"),
        ("legacy_created_fixed_piece", "legacy_created_fixed_piece"),
        ('if kind == "createdFixedPiece"', 'if kind == "createdFixedPiece"'),
        ("legacy_workspace", "legacy_workspace"),
    ]
    for label, needle in needles:
        if needle in text:
            errors.append(f"lib.rs: {label}")
    if has_ops_fallback_in_lib_rs(text):
        errors.append("lib.rs: ops fallback (.get(\"ops\") outside absence asserts)")

golden_ops = ROOT / "semio/assets/semio/kit-store.golden.ops.semio.json"
if not golden_ops.is_file():
    errors.append(f"missing file: {golden_ops}")
else:
    golden_text = golden_ops.read_text(encoding="utf-8")
    if '"ops":' in golden_text:
        errors.append("kit-store.golden.ops.semio.json: ops key")
    if '"createdFixedPiece"' in golden_text:
        errors.append("kit-store.golden.ops.semio.json: createdFixedPiece kind")

js_index = ROOT / "semio/client/lib/js/index.ts"
if js_index.is_file():
    js_text = js_index.read_text(encoding="utf-8")
    for label, needle in [
        ("openJson", "openJson"),
        ("semioJsonBootstrapUri", "semioJsonBootstrapUri"),
        ("backboneBootstrapUriForStoreOpen", "backboneBootstrapUriForStoreOpen"),
    ]:
        if needle in js_text:
            errors.append(f"index.ts: {label}")

algorithms = ROOT / "semio/dev/algorithms/index.ts"
if algorithms.is_file():
    algo_text = algorithms.read_text(encoding="utf-8")
    if "openSession(JSON.stringify" in algo_text:
        errors.append("algorithms/index.ts: openSession(JSON.stringify bootstrap)")

schema_golden = ROOT / "semio/client/schema/graphql/schema.golden.graphql"
schema_graphql = ROOT / "semio/client/schema/graphql/schema.graphql"
if schema_golden.is_file() and schema_graphql.is_file():
    for field in ("installProjection",):
        g = schema_golden.read_text(encoding="utf-8")
        s = schema_graphql.read_text(encoding="utf-8")
        if field in g and field not in s:
            errors.append(f"schema.graphql missing {field} from golden")

ALLOWED_KIT_STORE_CREATE_ARGS = {
    '"dev://empty"',
    "RS_WASM_EMPTY_STORE_URI",
    "bootstrapUri",
    "uri",
    "RS_WASM_EMPTY_STORE_URI)",
}
for scan_root in (ROOT / "semio", ROOT / ".storybook"):
    if not scan_root.is_dir():
        continue
    for path in scan_root.rglob("*.ts"):
        if "node_modules" in path.parts or path.suffix not in {".ts", ".tsx"}:
            continue
        rel = path.relative_to(ROOT).as_posix()
        if rel.endswith("rs-wasm-transport.ts") or rel.endswith("kit-store.worker.ts"):
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        for m in re.finditer(r"KitStoreHandle\.create\(([^)]+)\)", text):
            arg = m.group(1).strip()
            if arg in ALLOWED_KIT_STORE_CREATE_ARGS:
                continue
            if "dev://empty" in arg or "RS_WASM_EMPTY_STORE_URI" in arg:
                continue
            errors.append(f"{rel}: KitStoreHandle.create({arg}) must use dev://empty + installProjection")

if errors:
    raise SystemExit("consistency check failed:\n" + "\n".join(errors))
print("consistency check passed")
