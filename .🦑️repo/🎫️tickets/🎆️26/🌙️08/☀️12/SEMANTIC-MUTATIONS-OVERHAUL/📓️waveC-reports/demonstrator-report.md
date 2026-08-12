# Wave-C funnel — `demonstrator/playground` mutations facet

Facet: `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-demonstrator`. Picks up from
`📓️wave2-reports/demonstrator-playground-1-any-report.md`, which derived the vocabulary
(`ChangeSchema`, the facet's one real mutation — `PlaygroundSnapshot` is a single `schema` metadata
scalar) but left the old `🖼️set-snapshot`/`🫙no-mutation` scaffolding self-wired-adjacent because it
could not touch `📦️glue.rs`. Wave2's own report also flagged `cargoCheck: not-run` (its check never
finished in the time it had).

**Status: done.**

## 1. Compile

`cargo check -p semio-s-plugin-demonstrator` — wave2's dispatch-enum rewrite already compiled
cleanly with no app-level call sites anywhere in this plugin constructing the retired
`PlaygroundMutation::SetSnapshot`/`::NoMutation` (grepped the whole artifact + app tree — confirmed
zero). No app-level compile fixes needed for this facet (matches the remaining-work-map's own note:
"demonstrator, mathematical: clean").

## 2. Directory + glue trueing

- Deleted the two orphan scaffolds outright: `🧬️mutations/🖼️set-snapshot/` and
  `🧬️mutations/🫙no-mutation/` (3 files each, doc-comment-only stubs left over from wave2 — no
  glue.rs mount could keep them alive once I removed the mount, and no code referenced them).
- Removed their two `pub mod set_snapshot { .. }` / `pub mod no_mutation { .. }` blocks from
  `📦️glue.rs`'s `mutations` block, replacing both with one real mount:
  `pub mod change_schema { pub mod mutation; pub mod diff; pub mod inverse; }` pointing at the
  existing `✒️change-schema/` triad (payload `ChangeSchema { new_schema: String }`, verb `change`,
  kind `change-schema`, record `ChangedSchema`).
- Removed the dispatch file's inline `#[path = "."] pub mod change_schema { .. }` self-wiring
  (the `🔖️LeafWiring` region) — the triad is now glue-mounted like every other migrated facet.
  `dsl_derive::Mutations` → `dsl::Mutations` (this crate's actual re-exported alias, confirmed via
  `📦️glue.rs`'s `extern crate semio_framework_os_kernel as dsl;`, same finding as the `shooting`/
  `lowpoly` siblings under this ticket).

One triad directory, one variant, one glue mount — 1:1 in both directions, satisfying the
dispatch-coverage policy rule trivially (nothing to disambiguate with only one kind).

### Emoji table (1 mutation)

| Emoji | Slug | Kind |
|---|---|---|
| ✒️ | change-schema | `change-schema` |

TS mirror: added the missing `🟦️component.ts` stub beside all 3 `✒️change-schema` leaves (matching
the repo-wide stub convention; see the `shooting` report's §2 note on why non-stub content was
out of scope).

## 3. Remaining debt

- Rewrote the top-level `📖️component.grammar.semio` (mutations-root, NOT the `📝️text/` sibling),
  which still described the retired `no-mutation | set-snapshot` alternation — now describes
  `change-schema new-schema=<TEXT>` as the sole rule. Left `📝️text/📖️component.grammar.semio` /
  `💾️binary/📡️component.protocol.semio` as-is (generic envelope framing, no per-variant keywords —
  matches the already-migrated `mathematical` sibling's equivalent files, per wave2's own note).
- Added a `⚖️SemanticLaws` test (`change_schema_obeys_the_inverse_and_absorb_laws`, using
  `protocol::os_spr::testkit::assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`) to the
  dispatch file's existing `#[cfg(test)]` region. Wave2 skipped this citing no existing `testkit`
  dependency; re-checked and this crate already depends on `semio-framework-os-kernel` (aliased
  `dsl`/`store`/`protocol`) which re-exports `protocol::os_spr::testkit` — no new Cargo dependency
  needed, same finding as `shooting`.

## Final sweep

```
grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/🎪️demonstrator --include="*.rs" --include="*.ts"
```
0 hits.

## Gates

- `cargo check -p semio-s-plugin-demonstrator`: **0 errors** (verified after the glue.rs rewire and
  orphan deletion; confirmed zero hits inside `🎪️demonstrator`/`🎪️playground` paths across several
  check runs, including ones where transitively-pulled-in unrelated plugins — `procedural`,
  `process`, `sourcing`, `puzzle` — were mid-refactor elsewhere and briefly broke the full
  workspace build; none of those errors ever touched this plugin's own files).
- `cargo test -p semio-s-plugin-demonstrator --lib`: **blocked-churn**, ran twice (both times ~15
  minutes apart). Both runs fail identically with 16 errors, all inside
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs`
  (`VcsArtifactApp`/`SpaceMember`: `E0277` `(dyn SpaceMember + 'static)` cannot be sent between
  threads safely + one `E0499` double-borrow), zero hits in `🎪️demonstrator`/`🎪️playground` paths in
  either run — another session's in-progress `Send`-bound work on the shared plugin module, per the
  brief's "retry, never fix" rule for framework churn. `cargo check` (above) is unaffected by this
  (test-only code path); the facet's own tests were hand-verified against real type signatures.
  Verbatim first error: `error[E0277]: (dyn SpaceMember + 'static) cannot be sent between threads
  safely --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6250:40`.

## Files touched

Created: `🧬️mutations/✒️change-schema/{🦠️mutation,🔺️diff,↩️inverse}/🟦️component.ts` (3 stubs).

Removed: `🧬️mutations/🖼️set-snapshot/**`, `🧬️mutations/🫙no-mutation/**` (6 `.rs` files + dirs).

Modified:
- `📦️packages/🦀️rust/📦️glue.rs` (`mutations` block: 2 orphan mounts removed, 1 real
  `change_schema` mount added)
- `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  (self-wiring region removed, `dsl::Mutations`, top doc comment rewritten, `⚖️SemanticLaws` test
  added)
- `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
  (doc-comment `dsl_derive::Mutations` → `dsl::Mutations` reference corrected)
- `🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️component.grammar.semio`
  (rewritten for the real 1-kind vocabulary)

## sharedFileRequests

None — this facet had no app-level call sites needing fixes.

## allowlistKeysToRemove

`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` entries confirmed by `bun ./📜️script.ts policy` to no longer
reference banned vocabulary:

- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

## Deviations

- TS mirror kept as a stub — see `shooting` report's §2 note (repo-wide norm, low-priority policy
  rule, out of scope this pass).
- `cargo test` could not be observed green (blocked-churn, see Gates) — `cargo check` is
  unambiguously clean, and the facet's own test additions were hand-verified against the testkit's
  real signatures.
