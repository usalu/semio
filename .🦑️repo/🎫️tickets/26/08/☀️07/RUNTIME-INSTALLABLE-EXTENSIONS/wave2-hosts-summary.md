# Wave 2 host unification summary

Ticket: `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS`  
Scope: **process**, **forms**, **playbook**, **sourcing** (flow host excluded per wave plan).

Shared primitives (framework core UI):

- `semio_framework_core::Contribution` variants used: `ProcessMachines`, `FormsQuestionKind`, `PlaybookBlockKind`, `SourcingModule`
- `semio_framework_core::ProgramContributionEntry` + `parse_contributions(json)`

Host pattern (mirrors procedural3d / flow extension sync):

1. `contributions_json: String` on app `*Config` (default `"[]"`)
2. `*ConfigOperation::SetContributions { json }` applies config + calls engine `sync_*_contributions` where registry state is needed
3. `*Command::SetContributions` (host-pushed, not manifest palette) writes the config op
4. `DocumentApp::render` calls `sync_*` from `config.contributions_json` so first paint matches hot-install without a prior config op

---

## Process (`process3d-play`)

| Piece | Location |
|-------|----------|
| Config field + op | `🎛️apps/🧊️3d/🎚️config/🦀️component.rs` |
| Command | `🎛️apps/🧊️3d/🎮️commands/🧩️contribution/🦀️component.rs` |
| Registry sync | `🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs` → `sync_process_machine_contributions` |
| Merge | `installed_catalogs()` = built-in generic + wood/concrete/metal/robotic **plus** contributed catalogs |
| Render hook | `🎛️apps/🧊️3d/🦀️component.rs` `render()` |

Contributions filtered by `Contribution::ProcessMachines` with `app_id == "process3d-play"`. `machines_json` deserializes to `Vec<WorkshopMachine>`.

Tests: `sync_process_machine_contributions_merges_hot_installed_catalogs` (engine), config op round-trip, command surface count 36.

Wave 3e: move built-in domain catalogs out of compile-time registry; contributed-only hot path already works.

---

## Forms (`forms-play`)

| Piece | Location |
|-------|----------|
| Config + op | unchanged `contributions_json` / `SetContributions` |
| Parse | `pub use semio_framework_core::{parse_contributions, ProgramContributionEntry}`; local duplicate struct removed |
| Resolution | `find_question_kind_contribution` prefers `FormsQuestionKind`, falls back to `PlaybookBlockKind` |
| Catalogue | `catalogue_kinds` lists both contribution kinds |
| Render | `render_extension_question` uses shared `question_kind_match` |

Tests: primary fixture uses `FormsQuestionKind`; `extension_question_accepts_legacy_playbook_block_kind_contributions`.

---

## Playbook (`playbook-play`)

| Piece | Location |
|-------|----------|
| Config field + op | `🎛️apps/📖️playbook/🎚️config/🦀️component.rs` |
| Command | `🎛️apps/📖️playbook/🎮️commands/🧩️contribution/🦀️component.rs` |
| Palette | `🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs` → `playbook::build_palette(builtins, extensions)` from `contributions_json` `PlaybookBlockKind` entries |

Tests: builder palette includes `buildingComponent` when contributions JSON is set; command surface count 10.

---

## Sourcing (`sourcing-curate`)

| Piece | Location |
|-------|----------|
| Config field + op | `🎛️apps/🗂️curate/🎚️config/🦀️component.rs` |
| Command | `🎛️apps/🗂️curate/🎮️commands/🧩️contribution/🦀️component.rs` |
| Registry sync | `🗿️artifacts/🗂️curate/⚙️engine/🦀️component.rs` → `sync_sourcing_module_contributions` |
| Merge | `sourcing_modules()` / `available_modules()` = beams/windows/slabs builtins **plus** contributed modules |
| Render hook | `🎛️apps/🗂️curate/🦀️component.rs` `render()` |
| Extension metadata | `contributes = ["sourcing.module"]` on beams/windows/slabs extension `Cargo.toml` |

Contributions filtered by `SourcingModule` with `app_id == "sourcing-curate"`. `typology_json` / `kinds_json` deserialize to `TypologyNode` and `Vec<ObjectKind>`.

Tests: `sync_sourcing_module_contributions_adds_hot_installed_modules`; config op round-trip; command surface count 17.

---

## Glue wiring

New command modules registered in each plugin `📦️glue.rs` under `apps/*/commands`.

---

## Verification

Run (when toolchain available):

```bash
bun nx run @semio-tech/process-plugin:test-quick
bun nx run @semio-tech/forms-plugin:test-quick
bun nx run @semio-tech/playbook-plugin:test-quick
bun nx run @semio-tech/sourcing-plugin:test-quick
```

Local agent run blocked by Xcode license on `cc` (blake3 build); code compiles logically — re-run tests after `sudo xcodebuild -license`.

---

## Not in this wave

- Flow host / `flow_core` / flow play app (separate workstream)
- OS shell already aggregates `contributions_json` on `ViewModel` for focused app render
- Wave 3e process catalog crate split
