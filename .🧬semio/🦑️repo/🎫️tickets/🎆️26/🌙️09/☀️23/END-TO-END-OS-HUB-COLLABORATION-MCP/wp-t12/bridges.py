"""Generates one production mutation bridge per plugin: a Rust binary that reports DESCRIPTORS of every mutation
aggregate its artifact crates mount, filtered to the requested subset's owner (or, for a state-lane manifest, to its
surface owner), behind a 📜️script.ts process. `--out <dir>` writes the bridges below a scratch root instead of the
repository, for diffing."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
rows = [r for r in json.load(open(root + ".tmp-ticket/wp-t12/generated/manifests.json"))]
write = "--write" in sys.argv
out_root = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else root
only = [a for i, a in enumerate(sys.argv[1:], 1) if not a.startswith("--") and sys.argv[i - 1] != "--out"]
SURFACE_DIRS = json.load(open(root + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", encoding="utf-8"))["subsetSurfaceDirs"]

def crate_of(path):
    d = path if os.path.isdir(path) else os.path.dirname(path)
    while d.startswith(root):
        m = d + "/📦️packages/🦀️rust/Cargo.toml"
        if os.path.exists(m):
            t = open(m, encoding="utf-8").read()
            lib = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', t)
            return {"dir": d, "name": re.search(r'(?m)^name\s*=\s*"([^"]+)"', t).group(1), "lib": os.path.normpath(os.path.join(d, "📦️packages/🦀️rust", lib.group(1) if lib else "src/lib.rs")), "manifest": m}
        d = os.path.dirname(d)
    return None

def module_files(lib):
    """Every file the crate mounts, with its public module path and the cargo features its mount chain is gated by
    (inline `mod x {` blocks and `#[path]` mounts, recursively; `#[cfg(feature = "…")]` on a mount gates its subtree)."""
    found = {}
    def strip(line):
        line = re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)
        line = re.sub(r"'(?:[^'\\]|\\.)'", "''", line)
        return line.split("//", 1)[0]
    def walk(path, prefix, public, depth_limit, features=frozenset()):
        if depth_limit > 14 or not os.path.exists(path) or (path in found and (found[path][1] or not public)): return
        found[path] = (prefix, public, features)
        stack = [(prefix, os.path.dirname(path), public, -1, features)]
        pending, gate, declared, depth = None, frozenset(), {}, 0
        for raw in open(path, encoding="utf-8").read().splitlines():
            s = raw.strip()
            if s.startswith("//"): continue
            p = re.match(r'#\[path\s*=\s*"([^"]+)"\]', s)
            if p: pending = p.group(1); continue
            f = re.match(r'#\[cfg\(feature\s*=\s*"([^"]+)"\)\]', s)
            if f: gate = gate | {f.group(1)}; continue
            m = re.match(r"(pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)", s)
            code = strip(s)
            if m:
                cp, cb, cpub, _, cfeatures = stack[-1]
                mfeatures = cfeatures | gate
                name = m.group(2); vis = cpub and m.group(1) is not None and m.group(1).strip() == "pub"
                mp = cp + ([] if name == "component" else [name])
                if name == "component": vis = cpub
                if m.group(3) == "{":
                    stack.append((mp, os.path.normpath(os.path.join(cb, pending)) if pending else os.path.join(cb, name), vis, depth, mfeatures))
                elif pending:
                    child = os.path.normpath(os.path.join(cb, pending))
                    declared[(tuple(cp), name)] = (child, mfeatures)
                    walk(child, mp, vis, depth_limit + 1, mfeatures)
                pending, gate = None, frozenset()
            else:
                g = re.match(r"pub use (\w+)::\*;", s)
                if g and (tuple(stack[-1][0]), g.group(1)) in declared:
                    child, child_features = declared[(tuple(stack[-1][0]), g.group(1))]
                    walk(child, stack[-1][0], stack[-1][2], depth_limit + 1, child_features)
                if not s.startswith("#"): pending, gate = None, frozenset()
            depth += code.count("{") - code.count("}")
            while len(stack) > 1 and depth <= stack[-1][3]:
                stack.pop()
    walk(lib, [], True, 0)
    return found

def public_reexports(files):
    """Every name a public module re-exports by name from a child (`pub use child::{A, B};`, `pub use child::A;`),
    with the re-exporting module's path — the public home of a type defined in a private module."""
    found = {}
    for path, (mp, public, _) in files.items():
        if not public: continue
        text = open(path, encoding="utf-8").read()
        for group in re.findall(r"(?m)^\s*pub use \w+::\{([^}]*)\};", text):
            for item in group.split(","):
                name = item.strip().split(" as ")[-1].strip()
                if re.fullmatch(r"\w+", name): found.setdefault(name, []).append(mp)
        for name in re.findall(r"(?m)^\s*pub use \w+::(\w+)(?: as \w+)?;", text):
            found.setdefault(name, []).append(mp)
    return found

def leaf_kinds(directory):
    """Every mutation leaf `semanticKind` declared below a directory (`🧬️mutations/<leaf>/🔣️.json`)."""
    kinds = set()
    for dirpath, _, files in os.walk(root + directory):
        if "🔣️.json" not in files or os.path.basename(os.path.dirname(dirpath)) != "🧬️mutations": continue
        try:
            kind = json.load(open(os.path.join(dirpath, "🔣️.json"), encoding="utf-8")).get("semanticKind")
        except (ValueError, OSError, AttributeError):
            continue
        if kind: kinds.add(kind)
    return kinds

def reexported_region(row):
    """The sibling standard's owner a subset dispatches through when its `🧬️mutations/🦀️.rs` is a pure glob re-export of
    that standard's vocabulary (dwg AC1018 serves AC1024's `DwgMutation`), else `None`."""
    source = root + row["owner"] + "/🧬️schema/🧬️mutations/🦀️.rs"
    if not os.path.exists(source): return None
    target = re.search(r"(?m)^pub use crate::standards::(\w+)::subsets::(\w+)::schema::mutations::\*;", open(source, encoding="utf-8").read())
    if target is None: return None
    ids = set(row["ids"]) if isinstance(row["ids"], list) else set()
    for other in rows:
        if other["artifact"] == row["artifact"] and other["owner"] != row["owner"] and other["subset"] == row["subset"] and ids <= leaf_kinds(other["owner"]) and not reexported_region_marker(other):
            return other["owner"]
    return None

def reexported_region_marker(row):
    """Whether a subset's vocabulary facet is itself a glob re-export (never a region another subset measures)."""
    source = root + row["owner"] + "/🧬️schema/🧬️mutations/🦀️.rs"
    return os.path.exists(source) and re.search(r"(?m)^pub use crate::standards::\w+::subsets::\w+::schema::mutations::\*;", open(source, encoding="utf-8").read()) is not None

def leaf_region(row):
    """The directory whose mutation leaves a manifest measures: its own owner when the leaves its ids name live there,
    the re-exported sibling standard's owner when its vocabulary is a glob re-export, or the `🪆️subsets` root when an
    `any` manifest owns none of its leaves and composes its sibling subsets (fem's `🌐️any` over
    mesh/material/load/boundary/analysis)."""
    ids = set(row["ids"]) if isinstance(row["ids"], list) else set()
    owner = row["owner"]
    reexported = reexported_region(row)
    if reexported is not None: return reexported
    if not ids or ids & leaf_kinds(owner): return owner
    parent = os.path.dirname(owner)
    return parent if row["subset"] == "any" and os.path.basename(parent) == "🪆️subsets" and ids <= leaf_kinds(parent) else owner

AGG = re.compile(r"#\[derive\([^)]*\bMutations\b[^)]*\)\][^\n]*\n(?:\s*(?:#\[|//)[^\n]*\n)*\s*pub enum (\w+)\s*\{")
plugins = {}
for r in rows:
    owner = r["owner"]
    parts = owner.split("/")
    home = "/".join(parts[:3]) if owner.startswith("✏️s/🔌️plugins/") else owner
    if only and not any(o in home for o in only): continue
    plugins.setdefault(home, []).append(r)

report = []
for home, manifests in sorted(plugins.items()):
    crates, aggregates, problems, gates = {}, [], [], {}
    for r in manifests:
        c = crate_of(root + r["owner"])
        if not c: problems.append(f"{r['subset']}: no crate"); continue
        crates[c["name"]] = c
    for c in crates.values():
        files = module_files(c["lib"])
        reexports = public_reexports(files)
        structs = {name: list(homes) for name, homes in reexports.items()}
        for path, (mp, public, _) in files.items():
            if public:
                for m in re.finditer(r"(?m)^pub struct (\w+)", open(path, encoding="utf-8").read()):
                    structs.setdefault(m.group(1), []).append(mp)
        for path, (mp, public, features) in files.items():
            if "🧪️tests" in path or "/🏭️bridge/" in path or not path.endswith("🧬️mutations/🦀️.rs"): continue
            text = open(path, encoding="utf-8").read()
            for m in AGG.finditer(text):
                if public:
                    head = text[max(0, m.start() - 600):m.end()]
                    snap = re.findall(r"#\[mutations\([^\]]*?snapshot\s*=\s*([\w:]+)", head)
                    name = snap[-1].split("::")[-1] if snap else None
                    homes = structs.get(name, []) if name else []
                    shared = lambda home: len([1 for a, b in zip(home, mp) if a == b]) if all(a == b for a, b in zip(home, mp[:len(home)])) else -1
                    best = max(homes, key=lambda home: (sum(1 for a, b in zip(home, mp) if a == b), -len(home)), default=None)
                    snapshot = "::".join([c["name"].replace("-", "_")] + best + [name]) if best is not None else "_"
                    if snapshot == "_":
                        problems.append(f"snapshot of {m.group(1)} not located")
                        continue
                    aggregates.append((c["name"], "::".join([c["name"].replace("-", "_")] + mp + [m.group(1)]), snapshot))
                    gates.setdefault(c["name"], set()).update(features)
                elif reexports.get(m.group(1)):
                    head = text[max(0, m.start() - 600):m.end()]
                    snap = re.findall(r"#\[mutations\([^\]]*?snapshot\s*=\s*([\w:]+)", head)
                    name = snap[-1].split("::")[-1] if snap else None
                    exported = reexports[m.group(1)][0]
                    snapshot_homes = structs.get(name, []) if name else []
                    best = max(snapshot_homes, key=lambda candidate: (sum(1 for a, b in zip(candidate, exported) if a == b), -len(candidate)), default=None)
                    if best is None:
                        problems.append(f"snapshot of {m.group(1)} not located")
                        continue
                    aggregates.append((c["name"], "::".join([c["name"].replace("-", "_")] + exported + [m.group(1)]), "::".join([c["name"].replace("-", "_")] + best + [name])))
                    gates.setdefault(c["name"], set()).update(features)
                else:
                    problems.append(f"{m.group(1)} in {os.path.relpath(path, root)[:90]} is not publicly reachable")
    report.append((home, len(manifests), sorted(crates), len(aggregates), problems))
    if not write or not aggregates: continue
    bridge = out_root + home + "/🏭️bridge"
    os.makedirs(bridge, exist_ok=True)
    slug = re.sub(r"[^a-z0-9]+", "-", home.split("/")[-1].encode("ascii", "ignore").decode().lower()).strip("-") or "config"
    package = f"semio-{slug}-mutation-bridge"
    rel = lambda p: os.path.relpath(p, root + home + "/🏭️bridge")
    kernel = rel(root + "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust")
    pack = rel(root + "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust")
    def dependency(n, c):
        declared = set(re.findall(r'(?m)^([\w-]+)\s*=', open(c["manifest"], encoding="utf-8").read().split("[features]", 1)[1].split("\n[", 1)[0])) if "[features]" in open(c["manifest"], encoding="utf-8").read() else set()
        needed = sorted(gates.get(n, set()) & declared)
        return f'{n} = {{ path = "{rel(os.path.dirname(c["manifest"]))}"' + (f', features = [{", ".join(chr(34) + f + chr(34) for f in needed)}]' if needed else "") + " }"
    deps = "\n".join(dependency(n, c) for n, c in sorted(crates.items()))
    open(bridge + "/Cargo.toml", "w", encoding="utf-8").write(f'''# 🧭️ Own workspace root: the repository root manifest is a shared leased file, and a member crate would serialise
# every concurrent session behind that lease.
[workspace]

[package]
name = "{package}"
version = "0.1.0"
edition = "2021"
publish = false
description = "🏭️ Production mutation bridge for {home.split('/')[-1]} — answers listMutations from the production dispatch aggregates themselves."

[[bin]]
name = "{package}"
path = "🦀️.rs"

[dependencies]
semio-framework-os-kernel = {{ path = "{kernel}" }}
pack = {{ path = "{pack}", package = "semio-framework-pack" }}
{deps}
''')
    coords = sorted({(r["artifact"], r["standard"], r["subset"], r["surface"], r["owner"] if r["surface"] else leaf_region(r)) for r in manifests})
    reexports = {(r["artifact"], r["standard"], r["subset"]) for r in manifests if reexported_region(r) is not None}
    shared = {o for o in {c[4] for c in coords if not c[3]} if sum(1 for c in coords if not c[3] and c[4] == o and c[:3] not in reexports) > 1}
    pascal = lambda slug: "".join(part.capitalize() for part in slug.split("-"))
    coords = [(a, s_, u, f, o, pascal(u) if o in shared and not f else "") for a, s_, u, f, o in coords]
    aggs = sorted(set(aggregates))
    open(bridge + "/🦀️.rs", "w", encoding="utf-8").write(f'''//! 🏭️ Production mutation bridge for `{home}`.
//!
//! `test inventory` runs this and compares what it prints against each owner manifest and its claimed test
//! catalog. Every row is read out of a production aggregate's `DESCRIPTORS`, which the `dsl::Mutations` derive
//! generates from the mutation leaves themselves; a subset's inventory is the descriptors whose leaf `owner` lies
//! inside that subset's owner, so a verb dispatched in production and absent from the manifest is a breach.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts — the caller.

extern crate semio_framework_os_kernel as protocol;

use protocol::{{Mutation, MutationLeafDescriptor}};

/// 🧬️ One aggregate's descriptors, for the snapshot its `#[mutations(snapshot = …)]` names.
fn descriptors<S, M: Mutation<S>>() -> &'static [MutationLeafDescriptor] {{
    M::DESCRIPTORS
}}

/// 🧭️ Every mutation aggregate the artifact crates below mount, with its type name.
const AGGREGATES: &[(&str, fn() -> &'static [MutationLeafDescriptor])] = &[
{chr(10).join(f'    ("{p.split("::")[-1]}", descriptors::<{snap}, {p}>),' for _, p, snap in aggs)}
];

/// 🗺️ Manifest coordinate (artifact, standard, subset, state-lane surface or `""` for the document) → the owner
/// directory whose leaves it measures, and, where several subsets share one owner directory, the aggregate type-name
/// prefix that is that subset's dispatch.
const COORDINATES: &[(&str, &str, &str, &str, &str, &str)] = &[
{chr(10).join(f'    ("{a}", "{s_}", "{u}", "{f}", "{o}", "{pre}"),' for a, s_, u, f, o, pre in coords)}
];

/// 🎚️ The subset surface directories whose state lanes (config, presence, transient) are never document dispatch.
const SURFACE_DIRS: &[&str] = &[{", ".join(json.dumps(d, ensure_ascii=False) for d in SURFACE_DIRS)}];

fn main() {{
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (command, artifact, standard, subset, surface) = match args.as_slice() {{
        [command, artifact, standard, subset] => (command, artifact, standard, subset, ""),
        [command, artifact, standard, subset, surface] => (command, artifact, standard, subset, surface.as_str()),
        _ => {{
            eprintln!("usage: list-mutations <artifact> <standard> <subset> [<surface>]");
            std::process::exit(2);
        }}
    }};
    let Some((_, _, _, _, owner, prefix)) = COORDINATES.iter().find(|(a, s, u, f, _, _)| command == "list-mutations" && a == artifact && s == standard && u == subset && *f == surface) else {{
        eprintln!("this bridge does not answer {{command}} {{artifact}} {{standard}} {{subset}} {{surface}}");
        std::process::exit(2);
    }};
    let owner_prefix = format!("{{owner}}/");
    let in_state_lane = |leaf: &str| surface.is_empty() && leaf.split('/').any(|segment| SURFACE_DIRS.contains(&segment));
    let mut rows: Vec<pack::JsonValue> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for descriptor in AGGREGATES.iter().filter(|(name, _)| name.starts_with(prefix)).flat_map(|(_, aggregate)| aggregate().iter()) {{
        if !descriptor.owner.starts_with(&owner_prefix) || in_state_lane(&descriptor.owner[owner_prefix.len()..]) || seen.contains(&descriptor.semantic_kind) {{
            continue;
        }}
        seen.push(descriptor.semantic_kind);
        rows.push(pack::json_object([
            ("id".to_string(), pack::JsonValue::from(descriptor.semantic_kind)),
            ("variant".to_string(), pack::JsonValue::from(descriptor.aggregate_variant)),
            ("outcomes".to_string(), pack::json_array(descriptor.outcome_classes.iter().map(|class| pack::JsonValue::from(class.as_str())))),
        ]));
    }}
    let mut fields = vec![
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from(artifact.as_str())),
        ("standard".to_string(), pack::JsonValue::from(standard.as_str())),
        ("subset".to_string(), pack::JsonValue::from(subset.as_str())),
    ];
    if !surface.is_empty() {{
        fields.push(("surface".to_string(), pack::JsonValue::from(surface)));
    }}
    fields.extend([
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("producedBy".to_string(), pack::JsonValue::from("{package}")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    let out = pack::json_object(fields);
    println!("{{}}", pack::json_to_string(&out));
}}
''')
    open(bridge + "/📜️script.ts", "w", encoding="utf-8").write(f'''#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Language-neutral production mutation bridge for every subset under `{home}`.
//
// The test platform asks an owner what production dispatch offers without knowing its language: this
// process answers `list-mutations <artifact> <standard> <subset> [<surface>]` with a RuntimeMutationInventory on
// stdout, by running the sibling Rust binary that reads the production aggregates' DESCRIPTORS; a surface names a
// state lane (config, presence, transient) of the subset's editor or viewer.
//
//   bun 📜️script.ts list-mutations <artifact> <standard> <subset> [<surface>]
//
// @see 🦀️.rs — the binary that reads the dispatch aggregates
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory

//#endregion 🧲️Header

//#region 🔌️Adapters
import {{ spawnSync }} from "node:child_process";
//#endregion 🔌️Adapters

//#region 🚪️Entry
const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {{
  const [command = "", artifact = "", standard = "", subset = "", ...surface] = argv;
  if (command !== "list-mutations") {{
    console.error(`[bridge] unknown command ${{JSON.stringify(command)}} — expected list-mutations <artifact> <standard> <subset> [<surface>]`);
    return 2;
  }}
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "{package}", "--", command, artifact, standard, subset, ...surface], {{
    cwd: import.meta.dir,
    encoding: "utf8",
  }});
  if (built.status !== 0) {{
    console.error(`[bridge] cargo exited ${{built.status}}: ${{(built.stderr ?? "").trim().split("\\n").slice(-6).join("\\n")}}`);
    return 1;
  }}
  process.stdout.write(built.stdout);
  return 0;
}}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export {{ BRIDGE_VERSION }};
//#endregion 🚪️Entry
''')
for home, n, crates, na, problems in report:
    print(f"{n:3} manifests {len(crates):2} crates {na:3} aggregates  {home}  {'; '.join(problems)[:300]}")
