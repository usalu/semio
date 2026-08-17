# Packet: reasoning/wires — final non-stdio engine dissolution

Target: `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
(556 LOC). Directory **deleted**. This was the last artifact engine inside the `💡️reasoning` plugin's
scope (52/54 dissolved repo-wide before this packet, now +1).

## Baseline

No pre-migration commit was diffed against (HEAD already contains prior work, per the ticket's baseline
warning). Baseline counts were taken by reading the live `⚙️engine/🦀️component.rs` content at the start of
this packet (captured in this session's transcript): 556 lines, one `#[cfg(test)] mod tests` block with
4 tests / 9 `assert!`/`assert_eq!` (the `🔖️WiresExtension` region's tests — `relationship_kind_labels`,
`fixed_identity_set_validation`, `relationship_lookup`, `metabolism_fixture_hydrates_extension`). No other
`#[cfg(test)]` blocks existed in the file (the `🔖️Register`, `🔖️DocumentHelpers`, `🔖️ExampleFixture`,
`🔖️ArtifactEngine`, `🚪️DerivedIoRegistry` regions carried no tests of their own).

## Destination per region

| Region (orig. lines) | Destination | Why |
|---|---|---|
| `🔖️Register` (register/register_artifact_schema/register_artifact_inferences/register_pilot_languages), 12-92 | `🎛️apps/🔌️wires/🦀️component.rs`, new `//#region 🔌️Registration` | rule 6: `register*()` wiring → the app's own top-level component.rs, not glue.rs. Mirrors the already-dissolved `🌊️flow`/`🎛️apps/🌊️flow` and `🏭️process`/`🎛️apps/🧊️3d` pattern exactly. |
| `array_mut`, `entity_id`, `dsl_id`, `dsl_to_json`, `fixture_json_string`, `fixture_camera`, `fixture_nodes`, `fixture_edges`, `wires_identities`, `wires_relationships`, `node_position`, `force_layout_board` (`🔖️DocumentHelpers`, 94-195) | `🧬️schema/🦀️component.rs`, new `//#region 🔖️DocumentHelpers` | rule 3: pure helpers over `DslValue`-shaped documents, no `&Snapshot`/`&mut Snapshot`/app types in any signature. |
| `find_board_node`, `find_board_edge`, `find_relationship` (also in `🔖️DocumentHelpers`, but read `&WiresSnapshot`) | `🧬️schema/💡️inferences/🦀️component.rs`, new `//#region 🔖️LookupHelpers` | rule 2 / discriminator "reads → inferences": each takes `&WiresSnapshot`, not just a `DslValue`. |
| `metabolism_wires_example_snapshot`, `handcrafted_metabolism_snapshot` (`🔖️ExampleFixture`, 197-259) | `🧬️schema/🦀️component.rs`, new `//#region 🔖️ExampleFixture` | rule 3: pure builder over document/schema types (`WiresSnapshot`, `DslValue`, `store::apply_mutation`), zero app dependency, multiple consumers across both schema-level tests and the app. |
| `WiresError`, `RelationshipKind`, `WiresExtension` trait, `DefaultWiresExtension` (+ `graph`/`canvas`/`mindmap` re-export aliases), `🔖️WiresExtension` region + its own tests, 261-451 | `🎛️apps/🔌️wires/📌️panels/🔍️inspection/🦀️component.rs`, new `//#region 🔖️WiresExtension`; its 4 tests merged into the panel's existing `mod tests` | single consumer (`render`'s `DefaultWiresExtension::from_fixture_json` call) — matches the file's own original module-doc rule: "a helper with exactly one consumer lives in that consumer's own component file." Verified via repo-wide grep: zero other consumers of `DefaultWiresExtension`/`WiresExtension`/`RelationshipKind`/`graph`/`canvas`/`mindmap` aliases. |
| `WiresEngine` struct + impl (`🔖️ArtifactEngine`, 454-467) | **deleted outright** | rule 1: verified zero construction sites repo-wide (`grep -rn "WiresEngine"` before the packet showed only the struct's own definition + impl block; nothing else references it). |
| `io_registry` module (`🚪️DerivedIoRegistry`, 468-556: `entries()`, `WIRES_DIALECT`, `WIRES_JSON_BRIDGE_DIALECT`, `rebuild_native_snapshot`, `compose_export_{svg,csv,md,png,json}`) | `🚪️io/🦀️component.rs`, new `//#region 🚪️DerivedIoRegistry`, sibling to the existing `derived_composition` region | rule 5: io_registry/ComposerEntry/serializers → `🚪️io/`. |

## Trap A — the shadowing `io_registry` (found and fixed)

`🗿️artifacts/🔌️wires/🦀️component.rs`'s own `io_registry` module (the wires-artifact-root wrapper, returns
`&'static [&'static ComposerEntry]`) had a bare `use crate::artifacts::wires::standards::v1::engine::io_registry as v1;`
pointing at the now-deleted engine's `io_registry` (returns `&'static [ComposerEntry]`). Repointed the
single line to the new, fully-qualified home:

```rust
use crate::artifacts::wires::standards::v1::subsets::any::io::io_registry as v1;
```

Every other moved reference was fully qualified at its call site (no bare `use engine::…` left anywhere) —
see the full list below.

## Unqualified paths fully qualified (every one)

All were originally `crate::artifacts::wires::engine::X` (bare, single- or multi-name `use` lines, or
inline fully-qualified calls). Repointed per the table above to either
`crate::artifacts::wires::schema::X` or
`crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::X`, splitting any `use {…}`
group whose names spanned both destinations into two `use` lines. Files touched (24, all inside
`💡️reasoning`, all read individually and verified — no pattern substitution):

- `🎛️apps/🔌️wires/🦀️component.rs` (8 call sites + new Registration region)
- `🎛️apps/🔌️wires/📌️panels/🔍️inspection/🦀️component.rs` (import + WiresExtension region, see above)
- `🎛️apps/🔌️wires/📌️panels/📄️artifact/🦀️component.rs`
- `🎛️apps/🔌️wires/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🦀️component.rs`
- `🎛️apps/🔌️wires/🎮️commands/{🗑️delete,🧬️example,🔵️node,🔄️layout,🖱️pointer,🔗️relationship}/🦀️component.rs`
- `🗿️artifacts/🔌️wires/🦀️component.rs` (io_registry repoint + 2 doc-comment mentions)
- `🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs`
- `🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- 11 `🧬️mutations/<slug>/{🦠️mutation,↩️inverse}/🦀️component.rs` leaves (`create-node`×2, `change-node-kind`,
  `set-node-root`, `resize-node`, `delete-node`, `connect-nodes`×2, `disconnect-nodes`, `edit-node-text`,
  `change-node-shape`, `move-node`)
- `🦀️component.rs` (plugin root — `.setup(crate::artifacts::wires::engine::register)` → `.setup(crate::apps::wires::register)`)

## Glue changes

- Removed the `#[path]` mount `pub mod engine;` at `standards::v1` level (was pointing at the now-deleted
  `subsets/✳️any/⚙️engine/🦀️component.rs` — module nesting was `standards::v1::engine`, NOT
  `standards::v1::subsets::any::engine`, confirmed by reading the mount before touching it, per Trap C).
- Removed the legacy shim `pub mod engine { pub use super::standards::v1::engine::*; }` from the
  "Shims: keep pre-migration module paths resolving" block.
- Dangling-mount check (mandatory): **0** dangling `#[path]` targets after both removals.

## External-consumer check (crates outside `💡️reasoning` referencing its engine)

```
grep -rn "reasoning::artifacts::.*::engine::\|wires::engine::" ✏️s 🧰️framework --include="*.rs" | grep -v "^✏️s/🔌️plugins/💡️reasoning/"
```
**Checked, none.** Zero output.

## Structural verification

```
find ✏️s/🔌️plugins/💡️reasoning -path "*🗿️artifacts*" -name "⚙️engine" -type d   → 0 (directory gone)
grep -rn "::engine::\|standards::v1::engine\|subsets::any::engine" ✏️s/🔌️plugins/💡️reasoning   → 0 hits (nothing to enumerate/classify)
```

## Assertion count survival

Original engine file: 1 test mod, 4 tests, 9 asserts (all in the `WiresExtension` region). After the
split: all 4 tests (`relationship_kind_labels`, `fixed_identity_set_validation`, `relationship_lookup`,
`metabolism_fixture_hydrates_extension`) now live in
`🎛️apps/🔌️wires/📌️panels/🔍️inspection/🦀️component.rs`'s existing `mod tests` (extended, not duplicated —
no new test file created). That file's `mod tests` now totals 12 `assert!`/`assert_eq!` across 6 tests
(the 2 pre-existing panel tests' 3 asserts + the 4 moved tests' 9 asserts = 12). No assertions were
dropped; none were added beyond the moved ones.

## Deviations / unverified

- `🎛️apps/🔌️wires/⚙️engine/` (a *different*, already-empty directory under `🎛️apps`, not
  `🗿️artifacts`) was left untouched — out of this packet's scope (target was specifically the artifacts
  engine) and contains zero files, so nothing to dissolve there.
- **Compile status: UNVERIFIED — build-lock contention, not attempted** (per the ticket's mandatory
  instruction not to run `cargo check`). All checks performed were structural: brace/paren balance on
  every edited file (all balanced), directory-existence checks for every `#[path]` mount, and exhaustive
  `grep` enumeration of every `engine::` reference before and after the move (0 remaining).

## Files touched

Modified (33): `🦀️component.rs` (plugin root), `📦️packages/🦀️rust/📦️glue.rs`,
`🗿️artifacts/🔌️wires/🦀️component.rs`,
`🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`,
`.../🧬️schema/💡️inferences/🦀️component.rs`, `.../🧬️schema/📸️snapshot/📝️text/🦀️component.rs`,
`.../🧬️schema/🔺️diff/📝️text/🦀️component.rs`, `.../🧬️schema/🧬️mutations/🦀️component.rs`,
`.../🧬️mutations/{🌱create-node/🦠️mutation,🌱create-node/↩️inverse,🗑️delete-node/↩️inverse,
🧭move-node/↩️inverse,📐resize-node/↩️inverse,🏷️change-node-kind/↩️inverse,🔷change-node-shape/↩️inverse,
✏️edit-node-text/↩️inverse,🔗connect-nodes/🦠️mutation,🔗connect-nodes/↩️inverse,✂️disconnect-nodes/↩️inverse}/🦀️component.rs`,
`.../🚪️io/🦀️component.rs`, `🎛️apps/🔌️wires/🦀️component.rs`,
`🎛️apps/🔌️wires/📌️panels/{🔍️inspection,📄️artifact}/🦀️component.rs`,
`🎛️apps/🔌️wires/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🦀️component.rs`,
`🎛️apps/🔌️wires/🎮️commands/{🗑️delete,🧬️example,🔵️node,🔄️layout,🖱️pointer,🔗️relationship}/🦀️component.rs`.

Deleted (1 file, 1 directory): `🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs`
and its now-empty `⚙️engine/` directory.

Created (0 new source files) — one report file, this one, in the ticket folder.
