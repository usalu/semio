"""📜 Generates the producer crate manifests, project files and the facade manifest from the staged sources."""
import os, re, json
from lib import *
from crates import *
import usetree
from rs import tokenize

EXT = {
    "protocol": ("semio-framework-replication", "workspace", None),
    "semio_framework_value": ("semio-framework-value", "path", FW + "🌱️value/📦️packages/🦀️rust"),
    "semio_framework_dsl_record": ("semio-framework-dsl-record", "path", FW + "🗣️dsl/🧬️schema/📦️packages/🦀️rust"),
    "semio_framework_hash": ("semio-framework-hash", "path", FW + "🔏️hash/📦️packages/🦀️rust"),
    "semio_framework_async": ("semio-framework-async", "workspace", None),
    "semio_framework_diagnostic": ("semio-framework-diagnostic", "path", FW + "⚠️diagnostic/📦️packages/🦀️rust"),
    "semio_framework_io_schema": ("semio-framework-io-schema", "path", FW + "🚪️io/🧬️schema/📦️packages/🦀️rust"),
    "semio_framework_io_sqlite_snapshot": ("semio-framework-io-sqlite-snapshot", "path", FW + "🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust"),
    "semio_framework_pack_error": ("semio-framework-pack-error", "path", P + "⚠️error/📦️packages/🦀️rust"),
    "semio_framework_pack_source": ("semio-framework-pack-source", "path", P + "🚰️source/📦️packages/🦀️rust"),
    "semio_framework_pack_json": ("semio-framework-pack-json", "path", P + "🔤️json/📦️packages/🦀️rust"),
    "semio_framework_job": ("semio-framework-job", "path+testkit", FW + "🧵️job/📦️packages/🦀️rust"),
    "semio_framework_trace": ("semio-framework-trace", "path", FW + "⏱️trace/📦️packages/🦀️rust"),
    "semio_framework_dsl": ("semio-framework-dsl", "path", FW + "🗣️dsl/📦️packages/🦀️rust"),
    "semio_framework_async_macros": ("semio-framework-async-macros", "path", FW + "⏳️async/✨️macros/📦️packages/🦀️rust"),
    "semio_framework_dsl_record_derive": ("semio-framework-dsl-record-derive", "path", FW + "🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust"),
    "serde_json": ("serde_json", "serde_json", None),
    "serde": ("serde", "serde", None),
    "blake3": ("blake3", "blake3", None),
}
DEV_ONLY_SPECS = {"serde_json": 'serde_json = { version = "1.0.140", features = ["float_roundtrip"] }', "serde": 'serde = { version = "1.0.228", features = ["derive"] }', "blake3": 'blake3 = "1.8.2"'}

def pkg_abs(key): return REPO + pkgdir(key)

def rel(frm_abs, to_abs): return os.path.relpath(to_abs, frm_abs)

def path_idents(text):
    toks = [x for x in tokenize(text) if x[0] not in ("ws", "comment")]
    out = set()
    for k, (kd, tx, a, b) in enumerate(toks):
        if kd != "id": continue
        nxt = toks[k + 1][1] + toks[k + 2][1] if k + 2 < len(toks) else ""
        prev = toks[k - 1][1] if k else ""
        if nxt == "::" and prev not in (".",) and not (k >= 2 and toks[k - 1][1] == ":" and toks[k - 2][1] == ":" and toks[k - 3][1] not in ("crate",) and False):
            if prev == ":" and k >= 2 and toks[k - 2][1] == ":":
                before = toks[k - 3] if k >= 3 else ("punct", "", 0, 0)
                if before[0] == "id" or before[1] in (">", ")", "]"): continue
            out.add(tx)
        elif prev == "use" or (prev == "{" and False):
            out.add(tx)
    return out

def file_idents(files):
    res = {}
    names = set(EXT) | {ident(k) for k, *_ in CRATES}
    for f in files:
        t = read_stage_or_base(f)
        for i in path_idents(t) & names:
            res.setdefault(i, set()).add(f)
    return res

def is_test_file(f):
    return "🧪️tests" in f or "🧪️testing" in f or "⏱️trace" in f

def deps_of(key, files=None):
    files = files or crawl(libroot(key))
    used = file_idents(files)
    normal = {}; dev = {}
    for i, fs in used.items():
        if i == ident(key): continue
        (dev if all(is_test_file(f) for f in fs) else normal)[i] = fs
    return normal, dev

def dep_line(ident_, here_pkg_abs, features=None):
    if ident_ in EXT:
        name, kind, path = EXT[ident_]
        if kind == "workspace":
            feats = (', features = ["' + '", "'.join(features) + '"]') if features else ""
            return name + " = { workspace = true" + feats + " }"
        if kind == "path":
            return name + ' = { path = "' + rel(here_pkg_abs, REPO + path) + '" }'
        if kind == "path+testkit":
            return name + ' = { path = "' + rel(here_pkg_abs, REPO + path) + '", features = ["testkit"] }'
        return DEV_ONLY_SPECS[ident_]
    for k, *_ in CRATES:
        if ident(k) == ident_:
            return cname(k) + ' = { path = "' + rel(here_pkg_abs, pkg_abs(k)) + '" }'
    raise KeyError(ident_)

def uses_deflate(files):
    return any(re.search(r'feature\s*=\s*"deflate"', read_stage_or_base(f)) for f in files)

def crate_manifest(key):
    files = crawl(libroot(key))
    normal, dev = deps_of(key, files)
    here = pkg_abs(key)
    ws = rel(here, REPO + "🧰️framework")
    lines = ["[package]", 'workspace = "' + ws + '"', 'name = "' + cname(key) + '"', 'version = "0.1.0"', 'edition = "2021"', "rust-version.workspace = true",
             'description = "' + BY_KEY[key][3] + '"', "",
             "[package.metadata.semio]", 'role = "framework"', 'id = "' + short_id(key) + '"', "",
             "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
             "[lints]", "workspace = true", "",
             "[lib]", 'path = "' + rel(here, REPO + libroot(key)) + '"', ""]
    defl = uses_deflate(files)
    if defl:
        lines += ["[features]", 'deflate = ["semio-framework-replication/deflate"]', ""]
    lines += ["[dependencies]"]
    block = {i for i in ("semio_framework_async",) if any(re.search(r"\bblock_on\b", read_stage_or_base(f)) and "block_on" in ids(read_stage_or_base(f)) for f in files)}
    lib_block = any("block_on" in ids(read_stage_or_base(f)) for f in files if not is_test_file(f))
    for i in sorted(normal, key=lambda i: dep_name(i)):
        lines.append(dep_line(i, here, features=(["entrypoint"] if (i in block and lib_block) else None)))
    lines += ["", "[dev-dependencies]"]
    devs = dict.fromkeys(dev)
    for i in block:
        if i in normal and not lib_block: devs[i] = None
    for i in sorted(devs, key=lambda i: dep_name(i)):
        lines.append(dep_line(i, here, features=(["entrypoint"] if i in block else None)))
    return "\n".join(lines) + "\n", normal, dev, defl

def dep_name(i):
    if i in EXT: return EXT[i][0]
    for k, *_ in CRATES:
        if ident(k) == i: return cname(k)
    return i

def project_json(key):
    return json.dumps({"name": "@semio-tech/" + short_id(key) + "-rs", "tags": ["scope:framework", "language:rust"]}, ensure_ascii=False, indent=2) + "\n"

def run():
    report = {}
    for key, *_ in CRATES:
        if key == "record":
            pass
        text, normal, dev, defl = crate_manifest(key)
        write(pkgdir(key) + "/Cargo.toml", text)
        if key != "record":
            write(pkgdir(key) + "/📋️project.json", project_json(key))
        report[key] = (sorted(normal), sorted(dev), defl)
    return report

if __name__ == "__main__":
    for k, v in run().items(): print(k, v)

FACADE_PKG = P + "📦️packages/🦀️rust"
FACADE_INTEGRATION = [P + "🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs", P + "🌱️value/🛫️encode/🫳️borrowed/📨️wire/🧪️tests/🦀️.rs", P + "🧪️tests/🔬️record-laws/🦀️.rs", P + "🧪️tests/🧮️record-schema-laws/🦀️.rs"]

def facade_files():
    files = crawl(FACADE_PKG + "/🦀️.rs")
    for f in FACADE_INTEGRATION:
        for g in crawl(f):
            if g not in files: files.append(g)
    return files

def facade_manifest():
    base = read_base(FACADE_PKG + "/Cargo.toml")
    files = facade_files()
    normal, dev = deps_of("format", files)
    here = REPO + FACADE_PKG
    crate_idents = [ident(k) for k, *_ in CRATES]
    for i in crate_idents:
        normal.setdefault(i, set())
    head = base[:base.index("[features]")]
    head = head.replace('[lints]\nworkspace = true', '[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true')
    tests = '[[test]]\nname = "pack_record_laws"\npath = "../../🧪️tests/🔬️record-laws/🦀️.rs"\n\n[[test]]\nname = "pack_record_schema_laws"\npath = "../../🧪️tests/🧮️record-schema-laws/🦀️.rs"\n\n' + base[base.index("[[test]]\nname = \"original-owned-wire\""):]
    defl = ['"semio-framework-replication/deflate"'] + ['"' + cname(k) + '/deflate"' for k, *_ in CRATES if uses_deflate(crawl(libroot(k)))]
    out = head + "[features]\ndefault = [\"deflate\"]\ndeflate = [" + ", ".join(defl) + "]\ncorruption-testing = []\n\n[dependencies]\n"
    for i in sorted(normal, key=dep_name):
        out += dep_line(i, here) + "\n"
    out += "\n[dev-dependencies]\n"
    for i in sorted(dev, key=dep_name):
        if i in normal: continue
        out += dep_line(i, here) + "\n"
    out += "\n" + tests
    return out, sorted(normal), sorted(dev)

def run_facade():
    text, n, d = facade_manifest()
    write(FACADE_PKG + "/Cargo.toml", text)
    return n, d
