# Wave 4 Singles A — Final Status

`DEVELOPER_DIR=/Library/Developer/CommandLineTools` for all gates. Logs: `🧪wave4-<plugin>-check.txt`.

Helpers in this ticket folder:
- `🧪wave4-singles-a.mts` — bulk rename (`Operation`→`Mutation`, Emit, store dispatch, grammar/protocol)
- `🧪wave4-protocol-rename.mts` — `protocol::Operation` / collection `*_operation` → `*_mutation`
- `🧪wave4-split-owned.mts` + `🧪wave4-*.json` — split owned `🔧️op` into `🧬️mutations` + slim op
- `🧪wave4-owned-triads.mts` + `🧪wave4-note-triads.json` — owned-plugin triad dirs + glue snippet
- `🧪wave4-triads.mts` — kernel-backed dag/flow/forms triad stubs

## Gate summary (2026-08-07)

| Plugin | Crate | Mutations | Engine | glue+TS | `cargo check` |
|--------|-------|-----------|--------|---------|---------------|
| ✒️ writer | `semio-s-plugin-writer` | ✅ | ✅ `WriterEngine` | ✅ | blocked by dep `trinity` (not re-run) |
| 🌿️ vcs | `semio-s-plugin-vcs` | ✅ 6 triads | ✅ `VcsDemoEngine` | ✅ | **PASS** |
| ➗️ mathematical | `semio-s-plugin-mathematical` | ✅ 2 triads | ✅ `MathEngine` | ✅ `mathematical_mutations` | **PASS** |
| 🗒️ note | `semio-s-plugin-note` | ✅ 12 triads | ✅ `NoteEngine` | ✅ `note_mutations` | **PASS** (when `semio-s-3d` builds) |
| 🖨️ raster | `semio-s-plugin-raster` | ⚠️ enum in `🔧️op` | TODO | rename + `blake3` dep | **PASS** (monolithic op; triads TODO) |
| 🕸️ dag | `semio-s-plugin-dag` | ✅ kernel + 5 triads | TODO | ✅ glue | ready; blocked by `semio-s-3d` via infinite |
| 🎬️ sequence | `semio-s-plugin-sequence` | TODO split | TODO | protocol renames done | plugin errors remain + `semio-s-3d` |
| 🎞️ animate (present) | `semio-s-plugin-animate` | TODO split | TODO | rename | `semio-s-3d` + animate engine paths |
| 📜️ imperative | `semio-s-plugin-imperative` | TODO facet | TODO | pack fix | pack/op fixes; `imperative_engine` dep |
| 📏️ layout | `semio-s-plugin-layout` | TODO split | TODO | protocol renames | font asset path + `Mutation` on op |
| 📋️ forms | `semio-s-plugin-forms` | ✅ kernel + 9 triads | TODO | ✅ `flow` + playbook | playbook/op wiring + `semio-s-3d` |
| 🌊️ flow | `semio-s-plugin-flow` | ✅ kernel + 4 triads | TODO | ✅ `flow` dep | triad UFCS fixed; `semio-s-3d` |

## Collateral (this pass)

- **`semio-framework-os-infinite`**: `pub use board::ports::directed_dag::{…}` for DAG kernel surface at crate root (fixes `infinite_board_port_directed_dag::*` imports).
- **`blake3 = "1"`** on note, raster, animate, forms plugin crates (engine id helper).
- **Glue doc comment order**: dag + sequence — `extern crate` after crate doc (fixes `E0753`).
- **Flow glue**: `extern crate flow; pub use flow::playbook;`
- **Forms**: `forms_parse_contributions` + re-export alias (fixes `E0255`).
- **Imperative pack**: removed broken `conversion` type / duplicate `DocumentPack` impl.
- **Protocol rename script** across wave-4 plugin trees.

## Workspace blocker

Several crates transitively depend on **`semio-s-3d`**, which currently fails BREP compile (`int-ss` / `Vec3` ops). When that module is green again, re-run:

```bash
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
for p in semio-s-plugin-note semio-s-plugin-vcs semio-s-plugin-mathematical semio-s-plugin-raster \
  semio-s-plugin-dag semio-s-plugin-flow semio-s-plugin-forms semio-s-plugin-sequence \
  semio-s-plugin-imperative semio-s-plugin-layout semio-s-plugin-animate; do
  cargo check -p "$p" 2>&1 | tee ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/OPERATIONS-TO-MUTATIONS-AND-ARTIFACT-ENGINES/🧪wave4-${p#semio-s-plugin-}-check.txt" | tail -1
done
```

## Remaining migration work (per plugin)

1. **Raster / sequence / present / layout**: run `🧪wave4-split-owned.mts` + `🧪wave4-owned-triads.mts`, add `*Engine`, glue leaves, TS `*_mutations`.
2. **Imperative**: move `ImperativeMutation` + `Mutation` impl to `🧬️mutations`, triad `✂️step-collection`, `ImperativeEngine`.
3. **Dag / flow / forms**: add `*Engine: ArtifactEngine` in `⚙️engine`.
4. **Writer**: unchanged; still blocked on trinity backbone.
