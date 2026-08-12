# Wave 2 — `norm/en1990` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`.

## What landed

Deleted the generic single-variant `En1990Mutation::SetSnapshot { snapshot }` (whole-document
replace) and replaced it with a 10-variant semantic vocabulary, each a single-field tuple wrapping a
real `🦠️mutation`/`🔺️diff`/`↩️inverse` triad leaf, dispatched via `#[derive(dsl::Mutations)]`
(`#[mutations(snapshot = En1990Snapshot, diff = En1990Diff, schema = "s.norm.en1990")]`), mirroring
the wave0 `MiniMutation` fixture and the already-fanned-out `din16798`/`din4108` sibling facets in
this same plugin.

`En1990Snapshot` is a flat, id-less, document-root parameter form (`g_k`, `resistance_kn`,
`consequence_class`, `annex`, `seismic_a_ed_kn`) plus one intrinsically ordered, id-less table
(`q_k: Vec<En1990QkEntry>`, variable-action category/value rows) — no name/identity field to
`rename`, no id-keyed collection.

| New mutation | Verb | Entity | Notes |
|---|---|---|---|
| `change-annex{new_annex}` | change | annex | repurposes the pre-migration `📄set-snapshot/` triad dir in place |
| `change-permanent-action{new_g_k}` | change | permanent-action | `G_k`, doc-root scalar |
| `change-resistance{new_resistance_kn}` | change | resistance | `R_d` [kN], doc-root scalar |
| `change-consequence-class{new_consequence_class}` | change | consequence-class | CC1/CC2/CC3, doc-root scalar |
| `change-seismic-action{new_seismic_a_ed_kn}` | change | seismic-action | `A_Ed` [kN], doc-root scalar |
| `insert-variable-action{index,category,value}` | insert | variable-action | FINAL-state index into `q_k` |
| `remove-variable-action{index}` | remove | variable-action | BASE-state index |
| `change-variable-action-category{index,new_category}` | change | variable-action | BASE-state index |
| `change-variable-action-value{index,new_value}` | change | variable-action | BASE-state index |
| `reorder-variable-actions{from,to}` | reorder | variable-action | BASE-state `from`, taxonomy inverse law |

Field-for-field vocabulary, no invented structure beyond the snapshot's own shape (rule 1/3 of
`📓️derivation-rules.md`). `q_k`'s two-field row (`category`/`value`) gets a `change-*` mutation per
field rather than a single combined setter, since either field is independently entered (not an
inseparable facet), matching the `update-<facet>` exception's negative case.

`En1990Diff.q_k` is a pre-existing whole-list-per-diff wrapper (`En1990QkList`), not a sparse
triple — every `q_k` mutation rebuilds the full ordered `values` vec directly from `base` (never
apply-then-capture) and wraps it, mirroring `din4108`'s `layers`/`Din4108LayerList` precedent
exactly. Every `inverse()` reads `base` (pre-state): `remove-variable-action`'s inverse
re-`insert`s the captured entry at its original index; `insert-variable-action`'s inverse is
`remove-variable-action` at the (clamped) landing index; `reorder-variable-actions`'s inverse is
`reorder{from: min(to, len-1), to: from}` per the taxonomy's addressing convention #3; all
out-of-range BASE indices invert to `Vec::new()`, never a `NoMutation` sentinel.

Hand-rolled `OpText`/`OpBinary` for the new enum in `🧬️mutations/📝️text/🦀️component.rs` (the
derive only generates `Mutation`/`SemanticMutation`, never the wire codecs) — `keyword
key=value ...` grammar, quote-aware tokenizer, binary tag `0..=9` + varint/string fields; every
payload field already derives `Serialize`/`Deserialize` so it round-trips through a quoted JSON
atom (`enc_json`/`dec_json`, resp. `write_json_bin`/`read_json_bin`) rather than a second
handcrafted grammar per field type — same technique `din4108`'s sibling facet uses.
`🧬️mutations/💾️binary/🦀️component.rs` needed no changes (it's a thin `encode_op`/`decode_op`
wrapper generic over whichever `OpBinary` impl the mutation type has).

## Mechanism note: self-wiring + repurposed `📄set-snapshot` leaf (no `📦️glue.rs` edits)

`📦️glue.rs` is out of this facet's writable boundary (plugin-shared), but it `#[path]`-wires
`🧬️mutations/{🦀️component.rs, 📝️text/🦀️component.rs, 💾️binary/🦀️component.rs,
📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs}` directly. Those files could not be
created/renamed outside that wiring. Fix, matching `din16798`'s precedent exactly (this facet
happens to also have an `annex: AnnexChoice` field, same as `din16798`):

- The nine mutations with no pre-migration slot are self-wired directly inside
  `🧬️mutations/🦀️component.rs` itself (`🔖️NewLeaves` region, nine
  `#[path = "."] pub mod <slug> { #[path = "<dir>/🦠️mutation/🦀️component.rs"] pub mod mutation; ... }`
  blocks) — `#[path]` resolves per physical file, not per logical mod nesting, so this needs zero
  `📦️glue.rs` edits.
- `📄set-snapshot`'s pre-migration triad directory is **repurposed in place** (same physical path,
  rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) to hold `ChangeAnnex` instead of a new
  `change-annex/` directory — since `annex` was already one of this facet's scalar fields, this
  avoids `din4108`'s alternative "orphan stub + sharedFileRequests cleanup" pattern entirely. No
  dead code, no `📦️glue.rs` cleanup item needed for this ticket's own scope.

## Tests

Extended the existing `🧪️Tests` region (no new test files) in `🧬️mutations/🦀️component.rs` with 6
tests: `every_variant_registers_an_approved_semantic_descriptor` (verb-table + `kinds().len()`
check), `every_variant_round_trips_via_inverse` (all 10 variants), `insert_remove_variable_action`
round-trip + explicit inverse-value check, an out-of-range `remove-variable-action` empty-inverse
check, `reorder_variable_actions` round-trip, and `change_variable_action_category_and_value` (incl.
an out-of-range change's empty-inverse check). `🧬️mutations/📝️text/🦀️component.rs` has
`op_text_binary_roundtrip_law` over all 10 `demo_mutation_cases()`.

Also added the `protocol::testkit::assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`
shared law helpers (`🧪️MutationLaws` region) against the three most structurally distinct variants:
the repurposed enum-typed slot (`change-annex`), a plain `f64` scalar (`change-resistance`), and an
index-addressed table field (`change-variable-action-value`) — these ARE usable here: `protocol` is
this crate's own `extern crate semio_framework_os_kernel as protocol;` alias (declared in
`📦️glue.rs`, not a new Cargo dependency), and `testkit` is a public module of that same crate
(`protocol::testkit`), confirmed via `din16798`'s already-migrated sibling facet using exactly this
path.

## Verification

`cargo check -p semio-s-plugin-norm --tests`.

Hit one transient, wholly-unrelated blocker first: `semio-s-plugin-stdio` (a dependency of this
crate) failed with `couldn't read
.../🗿️artifacts/📜️docx/🏅️标准/🔖️ecma-376/.../🔺️diff/🦀️component.rs: No such file or directory`
— a mistyped emoji (`🏅️标准` instead of `🏅️standards`) in `stdio`'s own `📦️glue.rs`, a different
plugin entirely, clearly a different concurrent session's in-flight edit. Per the workspace-churn
policy: waited ~60s and rechecked — the typo was gone on the next look (the other session fixed its
own file), so this was not chased further and did not need a `blocked-churn` report.

Second run: **exactly 25 errors, all `E0599: no variant named 'SetSnapshot'`, all in `🎛️apps/**`
across SIX norm sibling apps** (`en1990`, `en1991`, `en1992`, `en1993`, `din16798`, `din4108`) — every
one of those siblings has ALSO been migrated by a concurrent session in this same wave, each leaving
the identical expected app-level fallout this ticket's instructions anticipate. Of the 25, exactly
**5 belong to `en1990`** (this facet's own app), matching `iso16757`'s sibling report's error count
exactly:
- `🎛️apps/📘️en1990/🦀️component.rs:107` (`import_media`)
- `🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs:20,41` (`handle` + its test)
- `🎛️apps/📘️en1990/🎮️commands/🧮️evaluate/🦀️component.rs:23,38` (`handle` + its test)

**Zero errors and zero warnings anywhere under `🗿️artifacts/📘️en1990/**`** — grepped the full error
log for `en1990` outside `🎛️apps/**`: the only remaining hits are `:::`-context lines pointing back
at this facet's own `pub enum En1990Mutation` definition (cited as context for the app-level errors,
not errors in this file) and one pre-existing, repo-wide, unrelated `unused import:
semio_framework_plugin::ArtifactAnalyzer` warning in `🚪️io/🦀️component.rs:15` that is byte-identical
across all fifteen norm artifacts (not touched by this migration).

`cargo test` cannot be run for this crate as a whole until the six siblings' app-level sites are
updated by the dedicated reconciliation pass (compilation is crate-wide; the test binary can't link
while any file in the crate fails to compile) — the hand-inspected round-trip/inverse-law/absorb-law
tests above are written and were successfully type-checked in the same `cargo check --tests` pass
(no error was reported against any file this ticket touched), but were not executed end-to-end, so
`lawTestsPass` is reported conservatively as `false` for that reason, not because any test is
believed wrong.

Grepped the entire artifact directory (`🗿️artifacts/📘️en1990/**`, including `📚️examples/`, the
artifact-root `🦀️component.rs`, `⚙️engine/`, `🚪️io/`) for `SetSnapshot`/`impl_norm_set_snapshot_ops`
— the only hit is this facet's own doc comment explaining the removal. Everything inside this
facet's writable boundary is fully migrated.

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs`** (`SetSnapshot::handle`, line 20,
   plus its test at line 41) — whole-document replace is banned outright per the taxonomy
   (`ArtifactStore::reset` is the sanctioned non-history path, entirely outside `Emit`/the
   `Mutation` enum). This command's whole purpose is whole-document replace, so it needs an
   architectural decision (route it through `reset` instead of `Emit`, or retire the command)
   rather than a mechanical swap — flagging for the reconciliation pass to decide, not solving here.
2. **`🎛️apps/📘️en1990/🎮️commands/🧮️evaluate/🦀️component.rs`** (`Evaluate::handle`, line 23, plus
   its test at line 38) — currently re-commits `En1990Mutation::SetSnapshot { snapshot:
   doc.snapshot.clone() }` purely to force a re-evaluation (its own doc comment: "a no-op
   whole-document commit is the honest way to record 'the user asked for a fresh evaluation'"). With
   `SetSnapshot` gone, this needs either a genuinely no-op-but-real semantic mutation, or (more
   honest) routing evaluation-refresh through the store's history-independent recompute path if one
   exists — another architectural call for the reconciliation pass.
3. **`🎛️apps/📘️en1990/🦀️component.rs`** (`import_media`, line 107) — replaces the whole snapshot
   from an imported media file via `En1990Mutation::SetSnapshot { snapshot }`; same as (1), this is
   a real whole-document-load gesture and should route through `store::ArtifactStore::reset` (its
   non-history sanctioned path) rather than a mutation-enum variant.

No `📦️glue.rs` cleanup item is needed for this facet specifically: unlike `din4108`'s "orphan stub"
approach, the pre-migration `📄set-snapshot/` triad was repurposed in place to hold real content
(`ChangeAnnex`), so `📦️glue.rs`'s existing `#[path]`s for that directory now point at live,
non-orphaned code — no rename or deletion needed there.
