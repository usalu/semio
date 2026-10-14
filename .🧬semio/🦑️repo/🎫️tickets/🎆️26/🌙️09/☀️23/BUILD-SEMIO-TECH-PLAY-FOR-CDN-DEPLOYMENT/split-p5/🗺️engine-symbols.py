#!/usr/bin/env python3
"""🗺️ Builds the engine symbol map {item name: owning crate} from the pub items of every fem engine crate root, and
rewrites `crate::<engine module>::…` paths (use groups, single uses and inline paths) of one or more files onto the
owning crates. Module names are the old `#[path]` mount names (`algebra`, `model`, `sparse`, `analyses`, …).

usage: 🗺️engine-symbols.py map                       (prints the map as JSON)
       🗺️engine-symbols.py rewrite <prefix> <file>... (prefix: the path head to replace, e.g. `crate` or `semio_s_artifact_fem_2d`)
"""
import json, os, re, sys
ENGINE = "/Users/ueli/Documents/semio/✏️s/🔨️modules/🏗️fem/⚙️engine"
CRATES = [
    ("algebra", "semio_s_fem_algebra", "➕️algebra/🦀️.rs"),
    ("formulation", "semio_s_fem_formulation", "➗️formulation/🦀️.rs"),
    ("model", "semio_s_fem_model", "🏗️model/🦀️.rs"),
    ("model", "semio_s_fem_static", "⚖️static/🦀️.rs"),
    ("elements2d", "semio_s_fem_elements2d", "📏️elements2d/🦀️.rs"),
    ("elements3d", "semio_s_fem_elements3d", "🧱️elements3d/🦀️.rs"),
    ("sparse", "semio_s_fem_sparse", "🔢️sparse/🦀️.rs"),
    ("sparse", "semio_s_fem_sparse_ldlt", "🔢️sparse/🔺️ldlt/🦀️.rs"),
    ("sparse", "semio_s_fem_sparse_pcg", "🔢️sparse/🔁️pcg/🦀️.rs"),
    ("sparse", "semio_s_fem_sparse_subspace", "🔢️sparse/🎵️subspace/🦀️.rs"),
    ("mesh", "semio_s_fem_mesh", "🕸️mesh/🦀️.rs"),
    ("mesh", "semio_s_fem_mesh_derived", "🕸️mesh/🧬️derived/🦀️.rs"),
    ("analyses", "semio_s_fem_analyses_model", "🧮️analyses/🗂️model/🦀️.rs"),
    ("analyses", "semio_s_fem_analyses_assembly", "🧮️analyses/🧩️assembly/🦀️.rs"),
    ("analyses", "semio_s_fem_analyses", "🧮️analyses/🦀️.rs"),
]
# model's re-exported element types moved owners: the old `model::Bar2` path names the elements crates now
ALIASES = {"model": ["elements2d", "elements3d"]}

def symbols():
    table = {}
    for module, crate, rel in CRATES:
        path = os.path.join(ENGINE, rel)
        if not os.path.exists(path): continue
        src = open(path, encoding="utf-8").read()
        for m in re.finditer(r"(?m)^pub\s+(?:const\s+fn|unsafe\s+fn|fn|struct|enum|trait|const|static|type|mod)\s+([A-Za-z_]\w*)", src):
            table.setdefault(module, {}).setdefault(m.group(1), crate)
        for m in re.finditer(r"(?m)^\s*pub enum (\w+)\s*:", src):
            table.setdefault(module, {}).setdefault(m.group(1), crate)
        if "#[dyn_enum]" in src:
            for m in re.finditer(r"#\[dyn_enum\]\s*pub trait (\w+)", src):
                table.setdefault(module, {}).setdefault("__semio_dispatch_" + m.group(1), crate)
    for module, extra in ALIASES.items():
        for e in extra:
            for name, crate in table.get(e, {}).items():
                table[module].setdefault(name, crate)
    return table

def owner(table, module, name):
    return table.get(module, {}).get(name)

def rewrite(prefix, files):
    table = symbols()
    mods = "|".join(sorted(table, key=len, reverse=True))
    pre = re.escape(prefix)
    group = re.compile(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+" + pre + r"::(?P<mod>" + mods + r")::\{(?P<body>[^;]*)\};")
    single = re.compile(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+" + pre + r"::(?P<mod>" + mods + r")::(?P<name>\w+)(?P<alias>\s+as\s+\w+)?;")
    inline = re.compile(r"(?<![\w:])" + pre + r"::(?P<mod>" + mods + r")::(?P<name>\w+)")
    whole = re.compile(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+" + pre + r"::\{(?P<body>[^;]*)\};")
    missing = []
    for f in files:
        s = open(f, encoding="utf-8").read(); before = s
        def g(m):
            names = [x.strip() for x in m.group("body").replace("\n", " ").split(",") if x.strip()]
            by = {}
            for n in names:
                base = n.split(" as ")[0].strip()
                if base == "self":
                    missing.append((f, m.group("mod") + "::self")); by.setdefault("?", []).append(n); continue
                c = owner(table, m.group("mod"), base)
                if not c: missing.append((f, m.group("mod") + "::" + base)); c = "?"
                by.setdefault(c, []).append(n)
            return "\n".join(f"{m.group('indent')}{m.group('vis') or ''}use {c}::{{{', '.join(v)}}};" if len(v) > 1 else f"{m.group('indent')}{m.group('vis') or ''}use {c}::{v[0]};" for c, v in sorted(by.items()))
        s = group.sub(g, s)
        def sg(m):
            c = owner(table, m.group("mod"), m.group("name"))
            if not c: missing.append((f, m.group("mod") + "::" + m.group("name"))); return m.group(0)
            return f"{m.group('indent')}{m.group('vis') or ''}use {c}::{m.group('name')}{m.group('alias') or ''};"
        s = single.sub(sg, s)
        def ig(m):
            c = owner(table, m.group("mod"), m.group("name"))
            if not c: missing.append((f, m.group("mod") + "::" + m.group("name"))); return m.group(0)
            return f"{c}::{m.group('name')}"
        s = inline.sub(ig, s)
        if s != before:
            open(f, "w", encoding="utf-8").write(s)
            print("rewrote", f)
    for f, p in missing: print("MISSING", p, "in", f)

if sys.argv[1] == "map":
    print(json.dumps(symbols(), indent=1, ensure_ascii=False))
else:
    rewrite(sys.argv[2], sys.argv[3:])
