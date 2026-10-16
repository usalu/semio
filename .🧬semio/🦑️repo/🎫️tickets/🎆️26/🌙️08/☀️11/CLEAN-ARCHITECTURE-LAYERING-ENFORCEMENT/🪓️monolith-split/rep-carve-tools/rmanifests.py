"""📜 Generates the carved crate manifests, project files and the facade manifest from the staged sources."""
import os, re, json
from rlib import *
import rcrates as C
from rs import tokenize

FWR = FW[:-len("🔨️modules/")] if FW.endswith("🔨️modules/") else FW
P_ = FW + "🎒️pack/"

# ident -> (cargo name, kind, path)  kind: path | workspace | version
EXT = {
    "semio_framework_replication_error": ("semio-framework-replication-error", "path", R + "⚠️error/" + PACKAGE),
    "semio_framework_replication_crypto": ("semio-framework-replication-crypto", "path", R + "🔐️crypto/" + PACKAGE),
    "semio_framework_mutation_diff": ("semio-framework-mutation-diff", "path", R + "🎮️mutation/🔺️diff/" + PACKAGE),
    "semio_framework_pack_source": ("semio-framework-pack-source", "path", P_ + "🚰️source/" + PACKAGE),
    "semio_framework_pack_json": ("semio-framework-pack-json", "path", P_ + "🔤️json/" + PACKAGE),
    "semio_framework_pack_error": ("semio-framework-pack-error", "path", P_ + "⚠️error/" + PACKAGE),
    "semio_framework_trace": ("semio-framework-trace", "path", FW + "⏱️trace/" + PACKAGE),
    "semio_framework_diagnostic": ("semio-framework-diagnostic", "path", FW + "⚠️diagnostic/" + PACKAGE),
    "semio_framework_schema_registry": ("semio-framework-schema-registry", "path", FW + "🧬️schema/📇️registry/" + PACKAGE),
    "semio_framework_schema_state": ("semio-framework-schema-state", "path", FW + "🧬️schema/📶️state/" + PACKAGE),
    "semio_framework_value": ("semio-framework-value", "workspace", None),
    "semio_framework_hash": ("semio-framework-hash", "path", FW + "🔏️hash/" + PACKAGE),
    "semio_framework_deflate": ("semio-framework-deflate", "path-optional", FW + "🗜️deflate/" + PACKAGE),
    "semio_framework_io_base64": ("semio-framework-io-base64", "path", FW + "🚪️io/🔤️base64/" + PACKAGE),
    "semio_framework_value_derive": ("semio-framework-value-derive", "path", FW + "🌱️value/✨️derive/" + PACKAGE),
    "semio_framework_trace_memory_testing": ("semio-framework-trace-memory-testing", "path", FW + "⏱️trace/🧮️memory/🧪️testing/📥️requests/" + PACKAGE),
    "semio_framework_async_macros": ("semio-framework-async-macros", "path", FW + "⏳️async/✨️macros/" + PACKAGE),
    "serde": ("serde", "line", 'serde = { version = "1.0.219", features = ["derive"] }'),
    "serde_json": ("serde_json", "line", 'serde_json = "1.0.140"'),
    "blake3": ("blake3", "line", 'blake3 = "1.8.2"'),
}

def strip_cfg_test(text):
    out = []; last = 0
    for it in items(text):
        if any(re.search(r"cfg\s*\(\s*test\s*\)", a) for a in it["attrs"]):
            out.append(text[last:it["start"]]); last = it["end"]
    out.append(text[last:])
    return "".join(out)

def path_idents(text):
    toks = [x for x in tokenize(text) if x[0] not in ("ws", "comment")]
    out = set()
    for k, (kd, tx, a, b) in enumerate(toks):
        if kd != "id": continue
        nxt = toks[k + 1][1] + toks[k + 2][1] if k + 2 < len(toks) else ""
        prev = toks[k - 1][1] if k else ""
        if nxt == "::" and prev != ".":
            if prev == ":" and k >= 2 and toks[k - 2][1] == ":":
                before = toks[k - 3] if k >= 3 else ("punct", "", 0, 0)
                if before[0] == "id" or before[1] in (">", ")", "]"): continue
            out.add(tx)
        elif prev == "use":
            out.add(tx)
    return out

def is_test_file(f):
    return "🧪️tests" in f or "🧪️testing" in f

def deps_of(files):
    names = set(EXT) | {C.ident(k) for k in C.KEYS}
    normal = {}; dev = {}
    for f in files:
        t = read_stage_or_base(f)
        if is_test_file(f):
            use = path_idents(t) & names
            for i in use: dev.setdefault(i, set()).add(f)
        else:
            use = path_idents(strip_cfg_test(t)) & names
            for i in use: normal.setdefault(i, set()).add(f)
            for i in (path_idents(t) & names) - use: dev.setdefault(i, set()).add(f)
    for i in list(dev):
        if i in normal: del dev[i]
    return normal, dev

def rel(frm_abs, to_abs): return os.path.relpath(to_abs, frm_abs)

def dep_name(i):
    if i in EXT: return EXT[i][0]
    for k in C.KEYS:
        if C.ident(k) == i: return C.cname(k)
    return i

def dep_line(i, here_abs, optional=False, features=None):
    if i in EXT:
        name, kind, path = EXT[i]
        if kind == "workspace": return name + ".workspace = true"
        if kind == "line": return path
        extra = ", optional = true" if (optional or kind == "path-optional") else ""
        return name + ' = { path = "' + rel(here_abs, REPO + path) + '"' + extra + " }"
    for k in C.KEYS:
        if C.ident(k) == i:
            return C.cname(k) + ' = { path = "' + rel(here_abs, REPO + C.pkgdir(k)) + '" }'
    raise KeyError(i)

def uses_deflate_feature(files):
    return any(re.search(r'feature\s*=\s*"deflate"', read_stage_or_base(f)) for f in files)

def crate_manifest(key):
    files = crawl(C.libroot(key))
    normal, dev = deps_of(files)
    normal.pop(C.ident(key), None); dev.pop(C.ident(key), None)
    here = REPO + C.pkgdir(key)
    lines = ["[package]", 'workspace = "' + rel(here, REPO + "🧰️framework") + '"', 'name = "' + C.cname(key) + '"', 'version = "0.1.0"', 'edition = "2021"', "rust-version.workspace = true",
             'description = "' + C.description(key) + '"', "",
             "[package.metadata.semio]", 'role = "framework"', 'id = "' + C.short_id(key) + '"', "",
             "[package.metadata.semio.rust-source-boundary]", "maxSourceBytes = 262144", "maxSourceFiles = 32", "maxActiveLines = 4000", "",
             "[lints]", "workspace = true", "",
             "[lib]", 'path = "' + rel(here, REPO + C.libroot(key)) + '"', ""]
    feats = []
    if uses_deflate_feature(files):
        if "semio_framework_deflate" in normal:
            feats.append('deflate = ["dep:semio-framework-deflate"]')
        else:
            sub = [C.cname(k) for k in C.KEYS if C.ident(k) in normal and k in DEFLATE_CRATES]
            feats.append("deflate = [" + ", ".join('"' + s + '/deflate"' for s in sub) + "]")
    if feats:
        lines += ["[features]"] + feats + [""]
    if key == "history":
        lines += ["[[test]]", 'name = "history_supersede_fold"', 'path = "../../🧪️tests/🔬️supersede-fold/🦀️.rs"', ""]
    lines += ["[dependencies]"]
    for i in sorted(normal, key=dep_name):
        lines.append(dep_line(i, here))
    lines += ["", "[dev-dependencies]"]
    for i in sorted(dev, key=dep_name):
        lines.append(dep_line(i, here))
    return "\n".join(lines) + "\n", normal, dev

DEFLATE_CRATES = {"codec"}

def project_json(key):
    return json.dumps({"name": C.project_name(key), "tags": ["scope:framework", "language:rust"]}, ensure_ascii=False, indent=2) + "\n"

def run():
    report = {}
    for key in C.KEYS:
        text, normal, dev = crate_manifest(key)
        write(C.pkgdir(key) + "/Cargo.toml", text)
        write(C.pkgdir(key) + "/📋️project.json", project_json(key))
        report[key] = (sorted(normal), sorted(dev))
    return report

if __name__ == "__main__":
    for k, v in run().items(): print(k, v)

PAGED = R + "🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/📄️paged-digest/🦀️.rs"

def facade_manifest():
    base = read_base(FACADE_PKG + "/Cargo.toml")
    here = REPO + FACADE_PKG
    files = crawl(FACADE_ROOT)
    for g in crawl(PAGED):
        if g not in files: files.append(g)
    normal, dev = deps_of([f for f in files if f != PAGED])
    pn, pd = deps_of([PAGED])
    for i in set(pn) | set(pd):
        if i not in normal: dev.setdefault(i, set())
    normal.pop("protocol", None)
    for k in C.KEYS: normal.setdefault(C.ident(k), set()); dev.pop(C.ident(k), None)
    for i in list(dev):
        if i in normal: del dev[i]
    head = base[:base.index("[[test]]")]
    head = head.replace("[lints]\nworkspace = true", "[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true")
    tests = base[base.index("[[test]]"):base.index("[features]")]
    defl = ["dep:semio-framework-deflate"] + [C.cname(k) + "/deflate" for k in ("codec", "format")]
    out = head + tests + "[features]\n# 🗜️ Deflate segment compression (`CodecId(1)`); off by default so the pure contract stays dependency-light.\ndeflate = [" + ", ".join('"' + d + '"' for d in defl) + "]\n\n[dependencies]\n"
    for i in sorted(normal, key=dep_name):
        out += dep_line(i, here) + "\n"
    out += "\n[dev-dependencies]\n"
    for i in sorted(dev, key=dep_name):
        out += dep_line(i, here) + "\n"
    return out, sorted(normal), sorted(dev)

def run_facade():
    text, n, d = facade_manifest()
    write(FACADE_PKG + "/Cargo.toml", text)
    return n, d
