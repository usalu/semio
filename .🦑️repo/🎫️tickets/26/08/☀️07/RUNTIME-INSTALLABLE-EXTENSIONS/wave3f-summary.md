# Wave 3.f CAD extension port summary

Ticket: `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS`  
Scope: four CAD domain modules (`spatial-shape`, `aec-building`, `aec-building-energy`, `aec-building-structure`).

## Rust extension crates

Each extension folder now has `🦀️component.rs` + `📦️packages/🦀️rust/` (`ExtensionBundle`, `extension_exports!`, `extends = "cad"`, `contributes = ["cad.computer"]`).

| Module | Crate | `module_id` | `computers_json` highlights |
|--------|-------|-------------|-----------------------------|
| 📐️ spatial-shape | `semio-s-plugin-cad-spatial-shape` | `spatial-shape` | `spatial.shape.geometry` stat, `spatial.shape.volume` property |
| 🏢️ aec-building | `semio-s-plugin-cad-aec-building` | `aec-building` | STEP `importProfiles` for `aec.building` |
| 🔥️ aec-building-energy | `semio-s-plugin-cad-aec-building-energy` | `aec-building-energy` | `energy.demand` stat, `energy.heatedvolume` property, energy import profile |
| 🏛️ aec-building-structure | `semio-s-plugin-cad-aec-building-structure` | `aec-building-structure` | `structure.stability` stat, five import profiles, `aec.building.structure/from_building` transformation |

All contributions target `app_id = "cad-play"`.

Root `Cargo.toml` workspace members added for the four crates. Each extension has `📋️project.json` + `📜️script.ts` (`nx` `test` / `test-quick`).

## Host (`cad-play`)

Mirrors wave-2 sourcing/process pattern:

| Piece | Location |
|-------|----------|
| Config field + op | `🎛️apps/📐️cad/🧮️config/🦀️component.rs` — `contributions_json`, `CadConfigOperation::SetContributions` |
| Command | `🎛️apps/📐️cad/🎮️commands/🧩️contribution/🦀️component.rs` |
| Registry sync (parse/track) | `🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs` → `sync_cad_computer_contributions` |
| Render hook | `🎛️apps/📐️cad/🦀️component.rs` `render()` |
| Glue | `📦️packages/🦀️rust/📦️glue.rs` — `commands::contribution` |

`SetContributions` is registered as `setContributions` / wire `contributions`.

## cad-js (TS consumer)

`🔨️modules/🏃️runtime/🟦️component.ts`:

- `CadComputersManifest` — shape of `computers_json`
- `syncCadComputerContributions(json)` — parses `ProgramContributionEntry[]`, filters `cadComputer` + `cad-play`, calls existing per-module `register()` by `moduleId`
- `shippedCadComputerContributionsJson()` — dev/test default when host has not pushed contributions
- `bootstrapCadModules(contributionsJson?)` — loads model-definition assets only; **no hardcoded** `register()` calls; uses shipped contributions JSON by default

TS extension `🟦️component.ts` files are unchanged (implementations stay in TS); Rust extensions own contribution metadata.

## Verification

```bash
bun nx run @semio-tech/cad-extension-spatial-shape-rust:test-quick
bun nx run @semio-tech/cad-extension-aec-building-rust:test-quick
bun nx run @semio-tech/cad-extension-aec-building-energy-rust:test-quick
bun nx run @semio-tech/cad-extension-aec-building-structure-rust:test-quick
bun nx run @semio-tech/cad-plugin:test-quick
bun nx run @semio-tech/cad-js:test quick
```

Local agent run blocked by Xcode license on `cc` (blake3); re-run after `sudo xcodebuild -license`.

## Follow-up

- Wire OS shell `contributions_json` push into cad-js when the React cad renderer binds config (Rust host already stores JSON).
- Optionally dedupe `shippedCadComputerContributionsJson()` with generated manifest bytes from the four extension crates.
