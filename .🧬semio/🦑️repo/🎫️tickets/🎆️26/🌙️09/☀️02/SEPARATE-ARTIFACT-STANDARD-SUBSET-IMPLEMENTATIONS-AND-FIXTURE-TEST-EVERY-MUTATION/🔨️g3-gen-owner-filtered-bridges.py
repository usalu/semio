#!/usr/bin/env python3
# 🏭️ Generates ONE artifact-level production mutation bridge for each of note/draw/sequence/
# mathematical(equation) — artifacts whose subsets have NOT yet been split into their own aggregate
# enum (unlike s.stdio.semio's 17). Each artifact still has exactly ONE `#[derive(dsl::Mutations)]`
# enum, held in its `✳️any` subset, and per-mutation-kind ownership is attributed by each leaf's own
# compiler-validated sidecar `owner` field (B4 Part 1 step 3, confirmed unchanged in this session).
#
# `mutationBridgeFor` (🧰️framework/…/🧪️test/📜️script.ts:333) walks UP from a subset's own owner
# looking for `🏭️bridge/📜️script.ts`, so placing ONE bridge at the ARTIFACT root
# (🗿️artifacts/<artifact>/🏭️bridge/) is inherited by every one of that artifact's subsets — B4 Part 1
# step 5's "one bridge per artifact, not per subset" — instead of copying the same crate N times.
#
# The bridge takes --subset on argv (passed through by InventoryScript) and filters
# `<Enum as Mutation<Snapshot>>::DESCRIPTORS` by the subset segment parsed out of each descriptor's
# own `owner` field (`/🪆️subsets/✳️<subset>/`, the same PROFILE_MARKER-adjacent shape
# `mutationCatalogProblems` already parses at 🟦️.ts:657) — never by a hand-maintained kind→subset
# table.
import os

REPO = "/Users/ueli/Documents/semio"

# key -> (bridge_dir, crate_dep_name, crate_dep_relpath (from bridge dir), mod_path_after_crate,
#         enum, snapshot, artifact_id, standard_id, standard_ident)
TARGETS = {
    "note": dict(
        bridge_dir="✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏭️bridge",
        crate="semio-s-plugin-note",
        crate_relpath="../../../📦️packages/🦀️rust",
        mod_path="artifacts::note::standards::v1::subsets::any::schema",
        enum="NoteMutation",
        snapshot="NoteSnapshot",
        artifact="s.note.note",
        standard="1",
    ),
    "drawing": dict(
        bridge_dir="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏭️bridge",
        crate="semio-s-plugin-draw",
        crate_relpath="../../../📦️packages/🦀️rust",
        mod_path="artifacts::drawing::standards::v1::subsets::any::schema",
        enum="DrawingMutation",
        snapshot="DrawingSnapshot",
        artifact="s.draw.drawing",
        standard="1",
    ),
    "sequence": dict(
        bridge_dir="✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏭️bridge",
        crate="semio-s-plugin-sequence",
        crate_relpath="../../../📦️packages/🦀️rust",
        mod_path="artifacts::sequence::standards::v1::subsets::any::schema",
        enum="SequenceMutation",
        snapshot="SequenceSnapshot",
        artifact="s.sequence.sequence",
        standard="1",
    ),
    "equation": dict(
        bridge_dir="✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏭️bridge",
        crate="semio-s-plugin-mathematical",
        crate_relpath="../../../📦️packages/🦀️rust",
        mod_path="artifacts::equation::standards::v1::subsets::any::schema",
        enum="EquationMutation",
        snapshot="EquationSnapshot",
        artifact="s.mathematical.equation",
        standard="1",
    ),
}

FRAMEWORK_OS_KERNEL_RELPATH = "../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
PACK_RELPATH = "../../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust"
# ↑ 6 ups from a 🗿️artifacts/<name>/🏭️bridge dir (✏️s/🔌️plugins/<plugin>/🗿️artifacts/<name>/🏭️bridge)
#   to repo root — same depth as the 🔌️plugins/<plugin>/📦️packages/🦀️rust crate path is 3 ups.

CARGO_TOML = '''[workspace]

[package]
name = "semio-{key}-bridge"
version = "0.1.0"
edition = "2021"
publish = false
description = "🏭️ Production mutation bridge for s.{key} — answers listMutations from {enum}::DESCRIPTORS, filtered by each descriptor's own owner-derived subset."

[[bin]]
name = "semio-{key}-bridge"
path = "🦀️.rs"

[dependencies]
{crate} = {{ path = "{crate_relpath}", default-features = false }}
semio-framework-os-kernel = {{ path = "{kernel_relpath}" }}
pack = {{ path = "{pack_relpath}", package = "semio-framework-pack" }}
'''

SCRIPT_TS = '''#!/usr/bin/env bun
// 🏭️ Language-neutral production mutation bridge for `{artifact}@{standard}/<subset>`. One bridge
// serves EVERY subset of this artifact (`mutationBridgeFor` walks up from a subset's own owner to
// find it) because the enum itself is not yet split per-subset — B4 Part 1 step 5.
//
//   bun 📜️script.ts list-mutations {artifact} {standard} <subset>

import {{ spawnSync }} from "node:child_process";
import {{ join }} from "node:path";

const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {{
  const [command = "", artifact = "{artifact}", standard = "{standard}", subset = "any"] = argv;
  if (command !== "list-mutations") {{
    console.error(`[bridge] unknown command ${{JSON.stringify(command)}} — expected list-mutations <artifact> <standard> <subset>`);
    return 2;
  }}
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "target"), "bridge");
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-{key}-bridge", "--", command, artifact, standard, subset], {{
    cwd: import.meta.dir,
    encoding: "utf8",
    env: {{ ...process.env, CARGO_TARGET_DIR: target }},
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
'''

RUST = '''//! 🏭️ Production mutation bridge for `{artifact}@{standard}` — every subset. Reads
//! `{enum}::DESCRIPTORS` (compiler-derived, `#[derive(dsl::Mutations)]`) and groups each row by the
//! subset segment parsed out of its OWN `owner` field — a fact `dsl::MutationLeaf`'s derive already
//! enforces matches the leaf's real source path (B4 Part 1 step 3) — filtering to the `--subset`
//! CLI argument `test inventory` passes through. No hand-written kind→subset table.

use semio_framework_os_kernel::Mutation;
use semio_framework_os_kernel as protocol;
use {crate_ident}::{mod_path}::mutations::{enum};
use {crate_ident}::{mod_path}::snapshot::{snapshot};

/// 🎯️ Same severity→outcome-class translation as the brep/mesh bridges (`protocol_outcomes`
/// there) — Info/Warning still applied, Error/Fatal rejected, deduplicated in first-seen order.
fn protocol_outcomes(classes: &[protocol::MutationOutcomeClass]) -> Vec<&'static str> {{
    let mut seen: Vec<&'static str> = Vec::new();
    for outcome in classes.iter() {{
        let mapped = match format!("{{outcome:?}}").as_str() {{
            "Applied" | "Info" | "Warning" => "applied",
            "Error" | "Fatal" => "rejected",
            _ => continue,
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

/// 🪆️ The `✳️<subset>` segment of a compiled leaf's own `owner` path — the same
/// `/🪆️subsets/✳️<name>/` shape `mutationCatalogProblems` parses server-side (🟦️.ts:657).
fn subset_of(owner: &str) -> &str {{
    match owner.split_once("/🪆️subsets/✳️") {{
        Some((_, rest)) => rest.split('/').next().unwrap_or(""),
        None => "",
    }}
}}

fn main() {{
    let argv: Vec<String> = std::env::args().collect();
    if argv.get(1).map(String::as_str) != Some("list-mutations") {{
        eprintln!("usage: bridge list-mutations <artifact> <standard> <subset>");
        std::process::exit(2);
    }}
    let artifact = argv.get(2).cloned().unwrap_or_else(|| "{artifact}".to_string());
    let standard = argv.get(3).cloned().unwrap_or_else(|| "{standard}".to_string());
    let subset = argv.get(4).cloned().unwrap_or_else(|| "any".to_string());

    let descriptors = <{enum} as Mutation<{snapshot}>>::DESCRIPTORS;
    let rows: Vec<pack::JsonValue> = descriptors
        .iter()
        .filter(|d| subset_of(d.owner) == subset)
        .map(|d| {{
            pack::json_object([
                ("id".to_string(), pack::JsonValue::from(d.semantic_kind)),
                ("variant".to_string(), pack::JsonValue::from(d.aggregate_variant)),
                ("outcomes".to_string(), pack::json_array(protocol_outcomes(d.outcome_classes).into_iter().map(pack::JsonValue::from))),
            ])
        }})
        .collect();
    let out = pack::json_object([
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from(artifact.as_str())),
        ("standard".to_string(), pack::JsonValue::from(standard.as_str())),
        ("subset".to_string(), pack::JsonValue::from(subset.as_str())),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{{}}", pack::json_to_string(&out));
}}
'''


def main() -> None:
    for key, t in TARGETS.items():
        bridge_dir = os.path.join(REPO, t["bridge_dir"])
        os.makedirs(bridge_dir, exist_ok=True)
        crate_ident = t["crate"].replace("-", "_")
        with open(os.path.join(bridge_dir, "Cargo.toml"), "w") as f:
            f.write(CARGO_TOML.format(
                key=key, enum=t["enum"], crate=t["crate"], crate_relpath=t["crate_relpath"],
                kernel_relpath=FRAMEWORK_OS_KERNEL_RELPATH, pack_relpath=PACK_RELPATH,
            ))
        with open(os.path.join(bridge_dir, "📜️script.ts"), "w") as f:
            f.write(SCRIPT_TS.format(key=key, artifact=t["artifact"], standard=t["standard"]))
        with open(os.path.join(bridge_dir, "🦀️.rs"), "w") as f:
            f.write(RUST.format(
                key=key, artifact=t["artifact"], standard=t["standard"], enum=t["enum"],
                snapshot=t["snapshot"], crate_ident=crate_ident, mod_path=t["mod_path"],
            ))
        print(f"wrote owner-filtered bridge for {key} ({t['enum']})")


if __name__ == "__main__":
    main()
