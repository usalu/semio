# Wave 4 Report — Architect / Shooting / Remodel

## Gate

| Crate | Command | Result | Log |
|-------|---------|--------|-----|
| `semio-s-plugin-remodel` | `cargo check -p semio-s-plugin-remodel` | **PASS** | `🧪wave4-remodel-check.txt` |
| `semio-s-plugin-shooting` | `cargo check -p semio-s-plugin-shooting` | **PASS** | `🧪wave4-shooting-check.txt` |
| `semio-s-plugin-architect` | `cargo check -p semio-s-plugin-architect` | **PASS** | `🧪wave4-architect-check.txt` |

`DEVELOPER_DIR=/Library/Developer/CommandLineTools` used on this machine.

## Pattern (from Wave 3 lowpoly)

For each artifact:

1. `🧬️mutations/` facet with per-mutation dirs (`🦠️mutation` / `🔺️diff` / `↩️inverse` + rust/ts leaves)
2. Root `🧬️mutations/🦀️component.rs` — dispatch enum `*Mutation` + `impl Mutation<P>`
3. Slim `🔧️op` to OpText/OpBinary + grammar; re-export enum; `start mutation`
4. `⚙️engine` implements `ArtifactEngine`
5. Rename `*Operation` → `*Mutation`, Emit/DocumentApp fields, Collection*, apply/inverse
6. Protocol `schema *.operation` → `*.mutation`
7. Glue + TS `*_mutations` export

**Kept:** Op brand (`🔧️op`, `OpText`, `OpBinary`, `print_op`/`parse_op`/`encode_op`/`decode_op`, `LanguageRole::Ops`).

## Remodel (`📸️remodel`)

- **Enum:** `RemodelMutation` (20 `Set*` variants), `#[serde(tag = "mutation")]`, `dsl::DslEnum`
- **Dirs:** `🎞️set-streams` … `✅️set-qc` (unique emojis)
- **Apply:** `apply_remodel_mutation` / `apply_remodel_mutation_in_place` + per-leaf apply/inverse
- **Engine:** `RemodelEngine`
- **TS:** `remodel_mutations`
- **Collateral:** replaced broken `blake3` id helper with `AtomicU64` monotonic counter in `next_remodel_id`

## Shooting (`🎥️shooting`)

- **Enum:** `ShootingMutation` — keeps `CollectionMutation` wrappers for Assets/Shots/SavedCameras (DSL mirror remains in `🔧️op` for OpText)
- **Dirs:** 11 mutation dirs (`📦assets` … `📄set-fixture`)
- **Apply:** collection leaves + diff-apply for non-collection variants
- **Engine:** `ShootingEngine`
- **Config:** `ShootingConfigMutation`
- **TS:** `shooting_mutations`
- **Collateral:** same `AtomicU64` fix for `next_shooting_id`; removed broken `impl OpText for used` dead codec block

## Architect program (`🏛️architect` / `🏛️program`)

- **Enum:** `ProgramMutation` (~72 variants) — `CollectionMutation` per register + meta/adjacency/SetProgram
- **Dirs:** 72 emoji-prefixed dirs (see `🧪wave4-architect-variants.json`)
- **Apply:** `apply_program_mutation` / `inverse_program_mutation` on root (leaves structural; register apply stays centralized)
- **Op:** JSON-line OpText/OpBinary (unchanged strategy; collection wrappers block DslEnum)
- **Diff:** `ProgramDiff.mutations`
- **Engine:** `ProgramEngine` + `register_architect_exports` alias for plugin setup
- **Config:** `ArchitectConfigMutation`
- **TS:** `program_mutations`
- **Preserved domain:** `Program.operations` register field + `OperationalRequirement` (not renamed)

## Files of note

| Plugin | Mutations root | Op | Engine | Glue |
|--------|----------------|----|--------|------|
| remodel | `…/🧬️mutations/🦀️component.rs` | slim codecs | `RemodelEngine` | `mutations` mod |
| shooting | `…/🧬️mutations/🦀️component.rs` | DSL mirror + codecs | `ShootingEngine` | `mutations` mod |
| architect | `…/🧬️mutations/🦀️component.rs` | JSON codecs | `ProgramEngine` | `mutations` mod |

## Ticket artifacts

- `🧪wave4-migrate-remodel.py` — remodel generator
- `🧪wave4-architect-op-backup.rs` / `🧪wave4-shooting-op-backup.rs` — pre-slim backups
- `🧪wave4-architect-variants.json` — 72 variant emoji map
- Per-crate `🧪wave4-*-check.txt` logs
