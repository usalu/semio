# A5 Report — Norm (Shared NormConfig)

## Summary

Wave A5 for `📕️norm` is complete. One plugin-level owner at `✏️s/🔌️plugins/📕️norm/🎚️config` now has a five-leaf config schema facet documenting `NormConfig` (`selected_check_index: Option<u32>`, all `local-ui`). Sibling `👥️presence` ships empty `NormPresence` + `NormPresenceMutation::Noop` (Snapshot/Noop empty pattern like VCS/Forms/NoPresence) with five schema leaves. Rust glue nests `config { component; schema }` and `presence { component; schema }`. All 15 DocumentApps bind `type Presence = NormPresence` / `type PresenceMutation = NormPresenceMutation`.

## Files touched

### Created
- `✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/📕️norm/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- This report: `🧪a5-norm-report.md`

### Updated
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` — nest config + presence modules
- All 15 `🎛️apps/*/🦀️component.rs` — Presence bindings + `use crate::presence::{…}`

## Gate tails

### 1. Scoped policy (`scope` includes `📕️norm`)
```
0
```
Pass. (The prompt's `JSON.stringify(x).includes("norm")` also matches unrelated owners whose breach *reason* text says "normative"; those are not norm-plugin breaches. Scope filter is the correct gate.)

### 2. `cargo check -p semio-s-plugin-norm`
```
warning: `semio-s-plugin-norm` (lib) generated 204 warnings …
    Finished `dev` profile [unoptimized] target(s) in 2m 38s
```
Pass (exit 0). Warnings are pre-existing in artifact engines, not from A5 leaves.

### 3. `cargo test -p semio-s-plugin-norm --lib`
```
test result: ok. 834 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```
Pass.

## Unverified
- Repo MCP (`repo://goals`, ticket_open/close) was unavailable in this session; work stayed inside existing ticket `26/08/09/APP-SCHEMA-FACETS`.
- TS package glue was not extended (same as completed empty-presence peers like vcs/forms); Rust glue + DocumentApp bindings are wired.
- Runtime presence pack round-trip was not exercised beyond compile/test (empty `DocumentPack` mirrors VCS).
