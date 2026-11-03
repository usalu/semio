import json, sys, pathlib
from collections import Counter, defaultdict
FILE_CAP = 200000
MANIFEST = """[package]
workspace = "../../../../../../.."
name = "semio-s-plugin-norm-ndp-{key}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Norm NDP registry rows of the {key} Eurocode part"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[dependencies]
semio-s-plugin-norm-ndp-model = {{ path = "../../../../🧱️model/📦️packages/🦀️rust" }}

[lib]
path = "../../🦀️.rs"
"""
args = [a for a in sys.argv[1:] if not a.startswith("--")]
check = "--check" in sys.argv
src, out = args[0], pathlib.Path(args[1])
if out.resolve().parent.name != "🔖️ndp" or out.name != "🗃️table.rs": sys.exit(f"refusing output {out}: must be <…>/🔖️ndp/🗃️table.rs")
PEND = "to be recorded at verification"
d = sorted(json.loads(pathlib.Path(src).read_text(encoding="utf-8"))["entries"], key=lambda e: e["id"])
parts = defaultdict(list)
for e in d:
    parts[e["id"].split(".")[1]].append(e)
def s(x): return json.dumps(x, ensure_ascii=False)
def v(x): return "Unset" if x is None else f"Number({float(x)!r})" if isinstance(x, (int, float)) else f"Text({s(x)})"
folder = out.with_suffix("")
total = 0
files = {}
for key, es in sorted(parts.items()):
    count = Counter(t for e in es for t in (e["code"], e["source"]["document"], e["unit"], e["symbol"]) + (() if e["source"]["edition"] == PEND else (e["source"]["edition"],)) if len(t) > 2)
    pool = {t: f"S{i}" for i, (t, n) in enumerate(count.most_common()) if n >= 4}
    r = lambda x: pool.get(x) or s(x)
    L = [f"//! 🗃️ Static NDP rows of {key} generated from `🔣️.json` by the ticket codegen; sorted by id. Edit the JSON, not this file.", "", "use semio_s_plugin_norm_ndp_model::NdpValue::*;", "use semio_s_plugin_norm_ndp_model::{c, d, NdpEntry};", ""]
    L += [f"const {n}: &str = {s(t)};" for t, n in pool.items()]
    L += ["", f"/// 📊️ Registered NDPs and table cells of {key}.", "pub const ROWS: &[NdpEntry] = &["]
    for e in es:
        so = e["source"]
        if e["recommended"] == e["naDe"]:
            head = f'c({s(e["id"])}, {r(e["code"])}, {s(e["clause"])}, {r(e["symbol"])}, {v(e["recommended"])}, {r(e["unit"])}, {r(so["document"])})'
        else:
            head = f'd({s(e["id"])}, {r(e["code"])}, {s(e["clause"])}, {r(e["symbol"])}, {v(e["recommended"])}, {v(e["naDe"])}, {r(e["unit"])}, {r(so["document"])})'
        if so["edition"] != PEND: head += f".ed({r(so['edition'])})"
        if so["locator"] != e["clause"]: head += f".at({s(so['locator'])})"
        per = e.get("annexStatus")
        if per:
            en_v, de_v = per["en"]["status"] == "verified", per["naDe"]["status"] == "verified"
            if (e["status"] == "verified") != (en_v and de_v): sys.exit(f"{e['id']}: row status must be verified exactly when both annexes are")
            for side, ok in (("en", en_v), ("naDe", de_v)):
                if ok and not per[side]["evidence"].strip(): sys.exit(f"{e['id']}: verified annex {side} has no evidence")
            if en_v and de_v: head += ".ver()"
            elif en_v: head += ".ver_en()"
            elif de_v: head += ".ver_de()"
        elif e["status"] == "verified": head += ".ver()"
        L.append(f"    {head},")
    L.append("];")
    text = "\n".join(L) + "\n"
    n = len(text.encode())
    if n > FILE_CAP: sys.exit(f"{key} table {n} bytes exceeds {FILE_CAP}")
    total += n
    files[folder / key / "🦀️.rs"] = text
    files[folder / key / "📦️packages" / "🦀️rust" / "Cargo.toml"] = MANIFEST.format(key=key)
    names = sorted(parts)
A = ["//! 🗃️ Static NDP table generated from `🔣️.json` by the ticket codegen: the per-Eurocode row crates joined at compile time into one id-sorted slice for binary search. Edit the JSON, not this file.", "", "use super::NdpEntry;", ""]
for k in names: A += [f"use semio_s_plugin_norm_ndp_{k} as {k};"]
A += ["", f"const PARTS: [&[NdpEntry]; {len(names)}] = [" + ", ".join(f"{k}::ROWS" for k in names) + "];", "",
"const LEN: usize = {", "    let mut total = 0;", "    let mut part = 0;", "    while part < PARTS.len() {", "        total += PARTS[part].len();", "        part += 1;", "    }", "    total", "};", "",
"const fn join() -> [NdpEntry; LEN] {", "    let mut out = [PARTS[0][0]; LEN];", "    let (mut index, mut part) = (0, 0);", "    while part < PARTS.len() {", "        let mut row = 0;", "        while row < PARTS[part].len() {", "            out[index] = PARTS[part][row];", "            index += 1;", "            row += 1;", "        }", "        part += 1;", "    }", "    out", "}", "",
"/// 📊️ All registered NDPs and table cells, sorted by id.", "pub static NDP_TABLE: &[NdpEntry] = &join();"]
text = "\n".join(A) + "\n"
total += len(text.encode())
files[out] = text
disk = {p for p in folder.rglob("*") if p.is_file()} | ({out} if out.exists() else set())
stale = sorted(disk - set(files))
drift = sorted(p for p, x in files.items() if not p.exists() or p.read_bytes() != x.encode("utf-8"))
if check:
    for p in drift + stale: print("DRIFT" if p in files else "STALE", p)
    if drift or stale: sys.exit(f"committed tables differ from a fresh codegen run: {len(drift)} drifted, {len(stale)} stale; run codegen.py")
    print("ok: committed tables equal a fresh codegen run,", len(d), "entries")
    sys.exit(0)
import shutil
for p in stale: p.unlink()
for old in sorted(folder.rglob("*"), reverse=True):
    if old.is_dir() and not any(old.iterdir()): shutil.rmtree(old)
for p, x in files.items():
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_bytes(x.encode("utf-8"))
print("total", total, "entries", len(d))
