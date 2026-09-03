#!/usr/bin/env python3
# 🏭️ Generates one production mutation bridge per remaining s.stdio.semio@v1 subset. Each bridge
# links `semio-s-plugin-stdio` (whole plugin, already compiled and cached in the shared scratch
# CARGO_TARGET_DIR by the cc6 bridge build this session) plus `semio-framework-os-kernel` directly
# (the plugin's own `protocol` alias is crate-private, so an external bridge cannot reach
# `Mutation`/`DESCRIPTORS` through it — B4/E4 verified this against real source), and reads the
# per-subset aggregate mutation enum's compiler-derived `DESCRIPTORS` — no hand-written
# `every_variant()`/`outcomes_of()` vocabulary, mirroring the existing `brep`/`mesh` bridges' pattern
# (see 🧬 Part 4 of 📓️e4-runtime-inventories.md) but as an external dependency like `cc6`, not a
# `#[path]` source-remount, because every one of these subsets is itself the artifact under this
# ticket's OWN split — mounting each one's individual file graph by hand (base/geometry, base/
# triples, per-subset snapshot/diff, N leaf mutation dirs, +text/+binary codec submodules) is exactly
# the kind of guess this rule exists to prevent; letting cargo resolve the already-public module tree
# through the compiled plugin crate is the same production answer with zero manual tracing.
import os

REPO = "/Users/ueli/Documents/semio"
SEMIO_SUBSETS_DIR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"

# subset_dir -> (EnumName, SnapshotName)
SUBSETS = {
    "animation": ("SemioAnimationMutation", "SemioAnimationSnapshot"),
    "audio": ("SemioAudioMutation", "SemioAudioSnapshot"),
    "base": ("SemioMutation", "SemioSnapshot"),
    "cad": ("SemioCadMutation", "SemioCadSnapshot"),
    "document": ("SemioDocumentMutation", "SemioDocumentSnapshot"),
    "drawing": ("SemioDrawingMutation", "SemioDrawingSnapshot"),
    "flow": ("SemioFlowMutation", "SemioFlowSnapshot"),
    "graph": ("SemioGraphMutation", "SemioGraphSnapshot"),
    "image": ("SemioImageMutation", "SemioImageSnapshot"),
    "kit": ("SemioKitMutation", "SemioKitSnapshot"),
    "model": ("SemioModelMutation", "SemioModelSnapshot"),
    "object": ("SemioObjectMutation", "SemioObjectSnapshot"),
    "presentation": ("SemioPresentationMutation", "SemioPresentationSnapshot"),
    "table": ("SemioTableMutation", "SemioTableSnapshot"),
    "text": ("SemioTextMutation", "SemioTextSnapshot"),
    "value": ("SemioValueMutation", "SemioValueSnapshot"),
    "video": ("SemioVideoMutation", "SemioVideoSnapshot"),
}

CARGO_TOML = '''# 🧭️ Own workspace root, like every other bridge here: the repository root manifest is a shared
# leased file, and a member crate would serialise every concurrent session behind that lease.
[workspace]

[package]
name = "semio-semio-v1-{subset}-bridge"
version = "0.1.0"
edition = "2021"
publish = false
description = "🏭️ Production mutation bridge for s.stdio.semio@v1/✳️{subset} — answers listMutations from {enum}::DESCRIPTORS."

[[bin]]
name = "semio-semio-v1-{subset}-bridge"
path = "🦀️.rs"

# 🔒️ TEST-ONLY IN EFFECT, PRODUCTION IN SUBSTANCE: links the real plugin so the answer comes from
# production dispatch itself. Adds no dependency to the plugin — the edge points one way, same shape
# as the ✳️cc6 bridge (not the narrower #[path]-remount ✳️brep/✳️mesh use — see 🦀️.rs docstring).
[dependencies]
semio-s-plugin-stdio = {{ path = "../../../../../../../📦️packages/🦀️rust", default-features = false }}
semio-framework-os-kernel = {{ path = "../../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }}
pack = {{ path = "../../../../../../../../../../🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }}
'''

SCRIPT_TS = '''#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Language-neutral production mutation bridge for `s.stdio.semio@v1/✳️{subset}`.
//
// The test platform must be able to ask ANY owner what production dispatch offers, without knowing
// which language that owner is written in. So the bridge contract is a PROCESS: one executable at
// `<owner>/🏭️bridge/📜️script.ts` that answers `list-mutations <artifact> <standard> <subset>` with a
// RuntimeMutationInventory on stdout. This subset's dispatch is Rust, so this script builds and runs
// the sibling binary; a TypeScript-owned subset would answer inline and the platform could not tell.
//
//   bun 📜️script.ts list-mutations s.stdio.semio v1 {subset}
//
// @see 🦀️.rs — the binary that reads {enum}::DESCRIPTORS
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — RuntimeMutationInventory

//#endregion 🧲️Header

//#region 🔌️Adapters
import {{ spawnSync }} from "node:child_process";
import {{ join }} from "node:path";
//#endregion 🔌️Adapters

//#region 🚪️Entry
const BRIDGE_VERSION = 1;

function main(argv: readonly string[]): number {{
  const [command = "", artifact = "s.stdio.semio", standard = "v1", subset = "{subset}"] = argv;
  if (command !== "list-mutations") {{
    console.error(`[bridge] unknown command ${{JSON.stringify(command)}} — expected list-mutations <artifact> <standard> <subset>`);
    return 2;
  }}
  // 🏭️`--offline` and an agent-scoped target directory: the bridge runs inside a test sweep alongside
  // peer sessions, and a shared target directory is the single biggest source of lock contention here.
  const target = process.env.CARGO_TARGET_DIR ?? join(process.env.SEMIO_AGENT_CACHE ?? join(import.meta.dir, "target"), "bridge");
  const built = spawnSync("cargo", ["run", "--quiet", "--offline", "--bin", "semio-semio-v1-{subset}-bridge", "--", command, artifact, standard, subset], {{
    cwd: import.meta.dir,
    encoding: "utf8",
    env: {{ ...process.env, CARGO_TARGET_DIR: target }},
  }});
  if (built.status !== 0) {{
    // 🚫️A bridge that cannot run must SAY SO and exit non-zero. Emitting a plausible-looking inventory
    // from the manifest would make the equality gate compare the manifest with itself, which is the
    // one thing Protocol v2's runtime half exists to prevent.
    console.error(`[bridge] cargo exited ${{built.status}}: ${{(built.stderr ?? "").trim().split("\\n").slice(-6).join("\\n")}}`);
    return 1;
  }}
  process.stdout.write(built.stdout);
  return 0;
}}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
export {{ BRIDGE_VERSION }};
//#endregion 🚪️Entry
'''

RUST = '''//! 🏭️ Production mutation bridge for `s.stdio.semio@v1/✳️{subset}`.
//!
//! `test inventory` runs this and compares what it prints against the owner manifest and the claimed
//! test catalog; `contract` then requires all three to agree EXACTLY. The answer is not written here
//! — it is read out of `{enum}::DESCRIPTORS`, which the `dsl::Mutations` derive generates from the
//! mutation leaves themselves. A mutation reachable in production but absent from the manifest shows
//! up as a breach rather than as a coverage footnote.
//!
//! Unlike `✳️brep`/`✳️mesh` (which `#[path]`-remount only their own subset's files so unrelated
//! artifact churn cannot block them), this bridge links the compiled `semio-s-plugin-stdio` crate
//! externally, the same shape `✳️cc6` uses — tracing this subset's own module dependency graph by
//! hand (base geometry/triples, per-leaf mutation dirs, +text/+binary codec submodules) would be a
//! guess dressed up as a mount; the plugin's own `#[path]` tree already resolved it correctly, and
//! that tree is `pub` all the way down, so reading it through the compiled crate is the same
//! production answer without re-deriving it.

use semio_framework_os_kernel::Mutation;
use semio_framework_os_kernel as protocol;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::{subset_ident}::schema::mutations::{enum};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::{subset_ident}::schema::snapshot::{snapshot};

/// 🎯️ Maps production's outcome severities onto the PROTOCOL's outcome classes. `Info`/`Warning` are
/// diagnostics ON an outcome — a mutation that applies with a warning still applied — and `Error`/
/// `Fatal` are the refusal. The test platform collapses them identically in `outcomeClassesOf`, in one
/// place and for the same reason; a bridge emitting raw severities would disagree with the manifest on
/// every mutation while both described the same behaviour.
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
        ("artifact".to_string(), pack::JsonValue::from("s.stdio.semio")),
        ("standard".to_string(), pack::JsonValue::from("v1")),
        ("subset".to_string(), pack::JsonValue::from("{subset}")),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{{}}", pack::json_to_string(&out));
}}
'''

EMOJI = {
    "animation": "🎞️", "audio": "🔊", "base": "🌐", "cad": "📐", "document": "📄",
    "drawing": "✏️", "flow": "🌊", "graph": "🕸️", "image": "🖼️", "kit": "🧰",
    "model": "🧊", "object": "📦", "presentation": "📽️", "table": "🔢", "text": "📝", "value": "🔣️", "video": "🎥",
}


def main() -> None:
    for subset, (enum, snapshot) in SUBSETS.items():
        emoji = EMOJI[subset]
        subset_dirname = f"✳️{subset}"
        bridge_dir = os.path.join(REPO, SEMIO_SUBSETS_DIR, subset_dirname, "🏭️bridge")
        os.makedirs(bridge_dir, exist_ok=True)
        with open(os.path.join(bridge_dir, "Cargo.toml"), "w") as f:
            f.write(CARGO_TOML.format(subset=subset, enum=enum))
        with open(os.path.join(bridge_dir, "📜️script.ts"), "w") as f:
            f.write(SCRIPT_TS.format(subset=subset, enum=enum))
        with open(os.path.join(bridge_dir, "🦀️.rs"), "w") as f:
            f.write(RUST.format(subset=subset, subset_ident=subset, enum=enum, snapshot=snapshot))
        print(f"wrote bridge for {subset} ({enum})")


if __name__ == "__main__":
    main()
