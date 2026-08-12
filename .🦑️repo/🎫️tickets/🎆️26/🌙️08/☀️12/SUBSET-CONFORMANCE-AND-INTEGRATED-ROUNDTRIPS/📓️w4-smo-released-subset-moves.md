# W4 — SMO-RELEASED Subset Engine/Example Glue Repair

Generated: 2026-08-12
Ticket: `26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`
Worker scope: five SMO-RELEASED plugins after structural relocation into `🪆️subsets/✳️any/`

## Plugins

| Plugin | Artifact | Subset |
|--------|----------|--------|
| `🪐️space` | `🏠️home` | `✳️any` |
| `🔋️energy` | `🔋️model` | `✳️any` |
| `🖨️raster` | `🖨️raster` | `✳️any` |
| `🕸️dag` | `🕸️dag` | `✳️any` |
| `🪵️sourcing` | `🗂️curate` | `✳️any` |

Note: sourcing plugin folder is `🪵️sourcing`, not `🍽️sourcing`.

## Prior structural moves (batch log)

See `scratch-w4-batch-structural-moves.txt` — engines and artifact-level `📚️examples/🎬️demo` trees were moved under each `🪆️subsets/✳️any/` root; empty artifact `📚️examples` parents removed.

## Glue repairs (this worker)

### Path rule

```
…/🏅️standards/<ver>/⚙️engine/           → …/🏅️standards/<ver>/🪆️subsets/✳️any/⚙️engine/
…/🗿️artifacts/<art>/📚️examples/🎬️demo/  → …/🗿️artifacts/<art>/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/
```

### `📦️glue.rs` updates

| Plugin | Engine `#[path]` rows | Example `#[path]` rows |
|--------|----------------------:|-----------------------:|
| `🪐️space` | 1 | 3 |
| `🔋️energy` | 52 (50 domain + root + standards shim) | 2 |
| `🖨️raster` | 1 | 2 |
| `🕸️dag` | 1 | 3 |
| `🪵️sourcing` | 1 | 2 |

Re-export shims (`pub mod engine { pub use super::standards::v1::engine::*; }`) unchanged — they still resolve through the updated `standards::v1::engine` module paths.

### Snapshot example `include_str!` fixes

Updated relative paths in `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` from seven-level artifact-root hop (`../../../../../../../📚️examples/…`) to four-level subset hop (`../../../../📚️examples/…`) for all five artifacts.

## Residual / out of scope

- `🪐️space/🦀️component.rs` OS fixture registrations still reference peer plugins' artifact-level examples (`🖍️draw`, `✒️writer`) — those plugins are not in this worker batch.
- App-level `📚️examples/🎬️demo-session` paths unchanged (app-owned per contract).
- Full subset conformance (`subset!`, 11-stage roundtrip, manifest body) not attempted — structural glue alignment only.

## Verification

Command attempted:

```bash
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target-w4-smo" \
  cargo check -p semio-s-plugin-space \
  cargo check -p semio-s-plugin-energy \
  cargo check -p semio-s-plugin-raster \
  cargo check -p semio-s-plugin-dag \
  cargo check -p semio-s-plugin-sourcing
```

Result: **blocked** — host disk full (`No space left on device` during dependency fingerprint writes). Re-run after disk cleanup.

Static audit: no remaining `🏅️standards/🔖️1/⚙️engine` or artifact-level `📚️examples/🎬️demo` references under the five plugin trees (glue + subset snapshot text).

## Changed files

- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
