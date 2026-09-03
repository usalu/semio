#!/usr/bin/env python3
# 🏭️ Generalized generator for the "already split per subset, ONE shared Snapshot type living in a
# `base` (or otherwise-named) sibling subset" pattern — confirmed this session for s.stdio.step@ap214
# (StepSnapshot in ✳️base), s.stdio.pdf@1.4 and @1.7 (PdfSnapshot in ✳️base, per standard), and
# s.stdio.ifc@2x3 (Ifc2x3Snapshot in ✳️base). Distinct from s.stdio.semio@v1 (own Snapshot PER
# subset — 🔨️g3-gen-semio-bridges.py) and from note/draw/sequence/mathematical (single un-split
# enum, owner-filtered — 🔨️g3-gen-owner-filtered-bridges.py). Same DESCRIPTORS + whole-plugin-
# external-link shape as all the others; see 🔨️g3-gen-semio-bridges.py's header for the rationale.
import os

REPO = "/Users/ueli/Documents/semio"

# Each entry: artifact python key -> dict with bridge base dir template, standard ident, subset->enum
# map, snapshot type, snapshot's owning subset ident (usually "base").
TARGETS = [
    dict(
        artifact_dir="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets",
        artifact="s.stdio.step", standard="ap214", standard_ident="v_ap214",
        artifact_mod="step",
        snapshot="StepSnapshot", snapshot_subset="base",
        subsets={"base": "StepMutation", "cc1": "StepCc1Mutation", "cc2": "StepCc2Mutation", "cc3": "StepCc3Mutation", "cc4": "StepCc4Mutation", "cc5": "StepCc5Mutation"},
    ),
    dict(
        artifact_dir="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets",
        artifact="s.stdio.pdf", standard="1.4", standard_ident="v1_4",
        artifact_mod="pdf",
        snapshot="PdfSnapshot", snapshot_subset="base",
        subsets={"base": "PdfMutation", "a": "PdfA1Mutation", "x": "PdfX1Mutation"},
    ),
    dict(
        artifact_dir="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets",
        artifact="s.stdio.pdf", standard="1.7", standard_ident="v1_7",
        artifact_mod="pdf",
        snapshot="PdfSnapshot", snapshot_subset="base",
        subsets={"base": "PdfMutation", "a": "PdfAMutation", "e": "PdfEMutation", "h": "PdfHMutation", "ua": "PdfUaMutation", "vt": "PdfVtMutation", "x": "PdfXMutation"},
    ),
    dict(
        artifact_dir="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets",
        artifact="s.stdio.ifc", standard="2x3", standard_ident="v2x3",
        artifact_mod="ifc",
        snapshot="Ifc2x3Snapshot", snapshot_subset="base",
        subsets={"base": "Ifc2x3Mutation", "cobie": "Ifc2x3CobieMutation", "cv20": "Ifc2x3Cv20Mutation", "sav": "Ifc2x3SavMutation"},
    ),
]

CARGO_TOML = '''[workspace]

[package]
name = "semio-{artifact_mod}-{standard_ident}-{subset}-bridge"
version = "0.1.0"
edition = "2021"
publish = false
description = "🏭️ Production mutation bridge for {artifact}@{standard}/✳️{subset} — answers listMutations from {enum}::DESCRIPTORS."

[[bin]]
name = "semio-{artifact_mod}-{standard_ident}-{subset}-bridge"
path = "🦀️.rs"

[dependencies]
semio-s-plugin-stdio = {{ path = "../../../../../../../📦️packages/🦀️rust", default-features = false }}
semio-framework-os-kernel = {{ path = "../../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }}
pack = {{ path = "../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }}
'''

SCRIPT_TS = '''#!/usr/bin/env bun
// 🏭️ Production mutation bridge for `{artifact}@{standard}/✳️{subset}`.
import {{ spawnSync }} from "node:child_process";
import {{ join }} from "node:path";

const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {{
  const [command = "", artifact = "{artifact}", standard = "{standard}", subset = "{subset}"] = argv;
  if (command !== "list-mutations") {{
    console.error(`[bridge] unknown command ${{JSON.stringify(command)}} — expected list-mutations <artifact> <standard> <subset>`);
    return 2;
  }}
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "target"), "bridge");
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-{artifact_mod}-{standard_ident}-{subset}-bridge", "--", command, artifact, standard, subset], {{
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

RUST = '''//! 🏭️ Production mutation bridge for `{artifact}@{standard}/✳️{subset}`. Reads
//! `{enum}::DESCRIPTORS` (compiler-derived, `#[derive(dsl::Mutations)]`) — no hand-written
//! vocabulary. `{snapshot}` is shared across every subset of this artifact/standard and lives in
//! `✳️{snapshot_subset}`, confirmed by reading the real `pub struct {snapshot}` definition, not
//! assumed from naming alone.

use semio_framework_os_kernel::Mutation;
use semio_framework_os_kernel as protocol;
use semio_s_plugin_stdio::artifacts::{artifact_mod}::standards::{standard_ident}::subsets::{subset_ident}::schema::mutations::{enum};
use semio_s_plugin_stdio::artifacts::{artifact_mod}::standards::{standard_ident}::subsets::{snapshot_subset}::schema::snapshot::{snapshot};

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
    let descriptors = <{enum} as Mutation<{snapshot}>>::DESCRIPTORS;
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
        ("artifact".to_string(), pack::JsonValue::from("{artifact}")),
        ("standard".to_string(), pack::JsonValue::from("{standard}")),
        ("subset".to_string(), pack::JsonValue::from("{subset}")),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{{}}", pack::json_to_string(&out));
}}
'''


def main() -> None:
    for t in TARGETS:
        for subset, enum in t["subsets"].items():
            bridge_dir = os.path.join(REPO, t["artifact_dir"], f"✳️{subset}", "🏭️bridge")
            os.makedirs(bridge_dir, exist_ok=True)
            fmt = dict(
                artifact=t["artifact"], standard=t["standard"], standard_ident=t["standard_ident"],
                artifact_mod=t["artifact_mod"], subset=subset, subset_ident=subset, enum=enum,
                snapshot=t["snapshot"], snapshot_subset=t["snapshot_subset"],
            )
            with open(os.path.join(bridge_dir, "Cargo.toml"), "w") as f:
                f.write(CARGO_TOML.format(**fmt))
            with open(os.path.join(bridge_dir, "📜️script.ts"), "w") as f:
                f.write(SCRIPT_TS.format(**fmt))
            with open(os.path.join(bridge_dir, "🦀️.rs"), "w") as f:
                f.write(RUST.format(**fmt))
            print(f"wrote bridge for {t['artifact']}@{t['standard']}/{subset} ({enum})")


if __name__ == "__main__":
    main()
