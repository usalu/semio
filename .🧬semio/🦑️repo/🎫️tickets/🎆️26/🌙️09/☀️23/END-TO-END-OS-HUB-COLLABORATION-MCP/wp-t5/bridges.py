"""Generates one production mutation bridge per plugin: a Rust binary that reports DESCRIPTORS of every mutation
aggregate its artifact crates mount, filtered to the requested subset's owner, behind a 📜️script.ts process."""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
rows = [r for r in json.load(open(root + ".tmp-ticket/wp-t5/generated/manifests.json"))]
write = "--write" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]

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
    """Every file the crate mounts, with its public module path (inline `mod x {` blocks and `#[path]` mounts, recursively)."""
    found = {}
    def strip(line):
        line = re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)
        line = re.sub(r"'(?:[^'\\]|\\.)'", "''", line)
        return line.split("//", 1)[0]
    def walk(path, prefix, public, depth_limit):
        if depth_limit > 14 or not os.path.exists(path) or (path in found and (found[path][1] or not public)): return
        found[path] = (prefix, public)
        stack = [(prefix, os.path.dirname(path), public, -1)]
        pending, declared, depth = None, {}, 0
        for raw in open(path, encoding="utf-8").read().splitlines():
            s = raw.strip()
            if s.startswith("//"): continue
            p = re.match(r'#\[path\s*=\s*"([^"]+)"\]', s)
            if p: pending = p.group(1); continue
            m = re.match(r"(pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)", s)
            code = strip(s)
            if m:
                cp, cb, cpub, _ = stack[-1]
                name = m.group(2); vis = cpub and m.group(1) is not None and m.group(1).strip() == "pub"
                mp = cp + ([] if name == "component" else [name])
                if name == "component": vis = cpub
                if m.group(3) == "{":
                    stack.append((mp, os.path.normpath(os.path.join(cb, pending)) if pending else os.path.join(cb, name), vis, depth))
                elif pending:
                    child = os.path.normpath(os.path.join(cb, pending))
                    declared[(tuple(cp), name)] = child
                    walk(child, mp, vis, depth_limit + 1)
                pending = None
            else:
                g = re.match(r"pub use (\w+)::\*;", s)
                if g and (tuple(stack[-1][0]), g.group(1)) in declared:
                    walk(declared[(tuple(stack[-1][0]), g.group(1))], stack[-1][0], stack[-1][2], depth_limit + 1)
                if not s.startswith("#"): pending = None
            depth += code.count("{") - code.count("}")
            while len(stack) > 1 and depth <= stack[-1][3]:
                stack.pop()
    walk(lib, [], True, 0)
    return found

AGG = re.compile(r"#\[derive\([^)]*\bMutations\b[^)]*\)\][^\n]*\n(?:\s*#\[[^\n]*\n)*\s*pub enum (\w+)\s*\{")
plugins = {}
for r in rows:
    owner = r["owner"]
    parts = owner.split("/")
    home = "/".join(parts[:3]) if owner.startswith("✏️s/🔌️plugins/") else owner
    if only and not any(o in home for o in only): continue
    plugins.setdefault(home, []).append(r)

report = []
for home, manifests in sorted(plugins.items()):
    crates, aggregates, problems = {}, [], []
    for r in manifests:
        c = crate_of(root + r["owner"])
        if not c: problems.append(f"{r['subset']}: no crate"); continue
        crates[c["name"]] = c
    for c in crates.values():
        files = module_files(c["lib"])
        structs = {}
        for path, (mp, public) in files.items():
            if public:
                for m in re.finditer(r"(?m)^pub struct (\w+)", open(path, encoding="utf-8").read()):
                    structs.setdefault(m.group(1), "::".join([c["name"].replace("-", "_")] + mp + [m.group(1)]))
        for path, (mp, public) in files.items():
            if "🧪️tests" in path or "/🏭️bridge/" in path or not path.endswith("🧬️mutations/🦀️.rs") or any(seg in path for seg in ("/✏️editor/", "/👁️viewer/", "/👥️presence/")): continue
            text = open(path, encoding="utf-8").read()
            for m in AGG.finditer(text):
                if public:
                    head = text[max(0, m.start() - 600):m.end()]
                    snap = re.findall(r"#\[mutations\([^\]]*?snapshot\s*=\s*([\w:]+)", head)
                    snapshot = structs.get(snap[-1].split("::")[-1], "_") if snap else "_"
                    if snapshot == "_": problems.append(f"snapshot of {m.group(1)} not located")
                    aggregates.append((c["name"], "::".join([c["name"].replace("-", "_")] + mp + [m.group(1)]), snapshot))
                else:
                    problems.append(f"{m.group(1)} in {os.path.relpath(path, root)[:90]} is not publicly reachable")
    report.append((home, len(manifests), sorted(crates), len(aggregates), problems))
    if not write or not aggregates: continue
    bridge = root + home + "/🏭️bridge"
    os.makedirs(bridge, exist_ok=True)
    slug = re.sub(r"[^a-z0-9]+", "-", home.split("/")[-1].encode("ascii", "ignore").decode().lower()).strip("-") or "config"
    package = f"semio-{slug}-mutation-bridge"
    rel = lambda p: os.path.relpath(p, bridge)
    kernel = rel(root + "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust")
    pack = rel(root + "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust")
    deps = "\n".join(f'{n} = {{ path = "{rel(os.path.dirname(c["manifest"]))}" }}' for n, c in sorted(crates.items()))
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
    coords = sorted({(r["artifact"], r["standard"], r["subset"], r["owner"]) for r in manifests})
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

use protocol::{{Mutation, MutationLeafDescriptor, MutationOutcomeClass}};

/// 🧬️ One aggregate's descriptors, for the snapshot its `#[mutations(snapshot = …)]` names.
fn descriptors<S, M: Mutation<S>>() -> &'static [MutationLeafDescriptor] {{
    M::DESCRIPTORS
}}

/// 🧭️ Every mutation aggregate the artifact crates below mount.
const AGGREGATES: &[fn() -> &'static [MutationLeafDescriptor]] = &[
{chr(10).join(f"    descriptors::<{snap}, {p}>," for _, p, snap in aggs)}
];

/// 🗺️ Manifest coordinate → the owner directory whose leaves it measures.
const COORDINATES: &[(&str, &str, &str, &str)] = &[
{chr(10).join(f'    ("{a}", "{s}", "{u}", "{o}"),' for a, s, u, o in coords)}
];

/// 🎯️ Production outcome severities as protocol outcome classes: `Info`/`Warning` ride on an applied outcome.
fn protocol_outcomes(classes: &[MutationOutcomeClass]) -> Vec<&'static str> {{
    let mut seen: Vec<&'static str> = Vec::new();
    for outcome in classes {{
        let mapped = match outcome {{
            MutationOutcomeClass::Applied | MutationOutcomeClass::Info | MutationOutcomeClass::Warning => "applied",
            MutationOutcomeClass::Error | MutationOutcomeClass::Fatal => "rejected",
        }};
        if !seen.contains(&mapped) {{
            seen.push(mapped);
        }}
    }}
    if seen.is_empty() {{
        seen.push("applied");
    }}
    seen
}}

fn main() {{
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [command, artifact, standard, subset] = args.as_slice() else {{
        eprintln!("usage: list-mutations <artifact> <standard> <subset>");
        std::process::exit(2);
    }};
    let Some((_, _, _, owner)) = COORDINATES.iter().find(|(a, s, u, _)| command == "list-mutations" && a == artifact && s == standard && u == subset) else {{
        eprintln!("this bridge does not answer {{command}} {{artifact}} {{standard}} {{subset}}");
        std::process::exit(2);
    }};
    let prefix = format!("{{owner}}/");
    let mut rows: Vec<pack::JsonValue> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for descriptor in AGGREGATES.iter().flat_map(|aggregate| aggregate().iter()) {{
        if !descriptor.owner.starts_with(&prefix) || seen.contains(&descriptor.semantic_kind) {{
            continue;
        }}
        seen.push(descriptor.semantic_kind);
        rows.push(pack::json_object([
            ("id".to_string(), pack::JsonValue::from(descriptor.semantic_kind)),
            ("variant".to_string(), pack::JsonValue::from(descriptor.aggregate_variant)),
            ("outcomes".to_string(), pack::json_array(protocol_outcomes(descriptor.outcome_classes).into_iter().map(pack::JsonValue::from))),
        ]));
    }}
    let out = pack::json_object([
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from(artifact.as_str())),
        ("standard".to_string(), pack::JsonValue::from(standard.as_str())),
        ("subset".to_string(), pack::JsonValue::from(subset.as_str())),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("producedBy".to_string(), pack::JsonValue::from("{package}")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
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
// process answers `list-mutations <artifact> <standard> <subset>` with a RuntimeMutationInventory on
// stdout, by running the sibling Rust binary that reads the production aggregates' DESCRIPTORS.
//
//   bun 📜️script.ts list-mutations <artifact> <standard> <subset>
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
  const [command = "", artifact = "", standard = "", subset = ""] = argv;
  if (command !== "list-mutations") {{
    console.error(`[bridge] unknown command ${{JSON.stringify(command)}} — expected list-mutations <artifact> <standard> <subset>`);
    return 2;
  }}
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "{package}", "--", command, artifact, standard, subset], {{
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
