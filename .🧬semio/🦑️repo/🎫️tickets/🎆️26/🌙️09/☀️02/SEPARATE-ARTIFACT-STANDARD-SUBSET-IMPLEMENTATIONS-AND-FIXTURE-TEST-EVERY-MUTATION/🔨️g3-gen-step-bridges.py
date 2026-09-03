#!/usr/bin/env python3
# 🏭️ Generates one production mutation bridge per remaining s.stdio.step@ap214 subset (base, cc1-
# cc5 — cc6 already has a working bridge, fixed by E4 this ticket). Every step subset already has
# its own fully split `Step<Subset>Mutation` aggregate enum (confirmed: base->StepMutation,
# cc1->StepCc1Mutation, ..., cc5->StepCc5Mutation), all sharing ONE `StepSnapshot` type that lives
# in `base`'s own schema — unlike semio's 17 (own snapshot per subset), so every step bridge here
# imports `StepSnapshot` from the `base` subset specifically, matching the real module tree, not a
# per-subset one. Same DESCRIPTORS + whole-plugin-external-link shape as ✳️cc6's fix and the semio
# generator (🔨️g3-gen-semio-bridges.py) — see that file's header for the full rationale.
import os

REPO = "/Users/ueli/Documents/semio"
STEP_SUBSETS_DIR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets"

# subset_dir -> EnumName (Snapshot is always StepSnapshot, imported from `base`)
SUBSETS = {
    "base": "StepMutation",
    "cc1": "StepCc1Mutation",
    "cc2": "StepCc2Mutation",
    "cc3": "StepCc3Mutation",
    "cc4": "StepCc4Mutation",
    "cc5": "StepCc5Mutation",
}

CARGO_TOML = '''# 🧭️ Own workspace root, like ✳️cc6's bridge: the repository root manifest is a shared leased file.
[workspace]

[package]
name = "semio-step-ap214-{subset}-bridge"
version = "0.1.0"
edition = "2021"
publish = false
description = "🏭️ Production mutation bridge for s.stdio.step@ap214/✳️{subset} — answers listMutations from {enum}::DESCRIPTORS."

[[bin]]
name = "semio-step-ap214-{subset}-bridge"
path = "🦀️.rs"

[dependencies]
semio-s-plugin-stdio = {{ path = "../../../../../../../📦️packages/🦀️rust", default-features = false }}
semio-framework-os-kernel = {{ path = "../../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }}
pack = {{ path = "../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }}
'''

SCRIPT_TS = '''#!/usr/bin/env bun
// 🏭️ Production mutation bridge for `s.stdio.step@ap214/✳️{subset}`. Same shape as ✳️cc6's own
// bridge (fixed this ticket by shard E4) — see that file for the full doc comment.
import {{ spawnSync }} from "node:child_process";
import {{ join }} from "node:path";

const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {{
  const [command = "", artifact = "s.stdio.step", standard = "ap214", subset = "{subset}"] = argv;
  if (command !== "list-mutations") {{
    console.error(`[bridge] unknown command ${{JSON.stringify(command)}} — expected list-mutations <artifact> <standard> <subset>`);
    return 2;
  }}
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "target"), "bridge");
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-step-ap214-{subset}-bridge", "--", command, artifact, standard, subset], {{
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

RUST = '''//! 🏭️ Production mutation bridge for `s.stdio.step@ap214/✳️{subset}`. Reads
//! `{enum}::DESCRIPTORS` (compiler-derived) rather than a hand-written `every_variant()` — the
//! pattern this ticket's E4 shard recommended as the follow-up to the ✳️cc6 bridge's minimal fix,
//! now applied to a fresh subset instead of retrofitted onto cc6's already-working one.

use semio_framework_os_kernel::Mutation;
use semio_framework_os_kernel as protocol;
use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::{subset_ident}::schema::mutations::{enum};
use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;

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

fn main() {{
    let descriptors = <{enum} as Mutation<StepSnapshot>>::DESCRIPTORS;
    let rows: Vec<pack::JsonValue> = descriptors
        .iter()
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
        ("artifact".to_string(), pack::JsonValue::from("s.stdio.step")),
        ("standard".to_string(), pack::JsonValue::from("ap214")),
        ("subset".to_string(), pack::JsonValue::from("{subset}")),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{{}}", pack::json_to_string(&out));
}}
'''


def main() -> None:
    for subset, enum in SUBSETS.items():
        bridge_dir = os.path.join(REPO, STEP_SUBSETS_DIR, f"✳️{subset}", "🏭️bridge")
        os.makedirs(bridge_dir, exist_ok=True)
        with open(os.path.join(bridge_dir, "Cargo.toml"), "w") as f:
            f.write(CARGO_TOML.format(subset=subset, enum=enum))
        with open(os.path.join(bridge_dir, "📜️script.ts"), "w") as f:
            f.write(SCRIPT_TS.format(subset=subset))
        with open(os.path.join(bridge_dir, "🦀️.rs"), "w") as f:
            f.write(RUST.format(subset=subset, subset_ident=subset, enum=enum))
        print(f"wrote bridge for step/{subset} ({enum})")


if __name__ == "__main__":
    main()
