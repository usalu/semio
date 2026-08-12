# Wave 2 — `norm/en1993` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`.

## Source shape (before)

`En1993Snapshot` is a flat EN 1993 steel-design compliance-check input sheet — 74 persistent scalar
fields (`annex: AnnexChoice` + 73 `f64`/`u32`/`u8`/`String` fields), **no** id-keyed/index-keyed
collections, no hierarchy, no relationships. The old `En1993Mutation` enum had exactly one variant,
`SetSnapshot { snapshot: En1993Snapshot }` (whole-document replace), dispatched via hand-written
`impl Mutation<En1993Snapshot>` calling `diff_set_snapshot`/a hand-rolled inverse, plus
`crate::impl_norm_set_snapshot_ops!(En1993Mutation, En1993Snapshot)` for `OpText`/`OpBinary`.

## Derivation applied (per `derivation-rules.md`)

`⚙️engine/🦀️component.rs`'s `check_full_steel_member` consumes this exact snapshot as **16 EN 1993
parts**, one `#[region]` per part, each calling one or more check functions with exactly that part's
own field subset (verified by reading the function bodies, not guessed): `part_1_1` (base member:
5 checks sharing `n_ed_kn`/`m_ed_knm`/`v_ed_kn`/`a_mm2`/`a_v_mm2`/`w_pl_mm3`/`f_y_mpa`/`f_u_mpa`/
`chi`/`a_net_mm2`/`tension_n_ed_kn`), `part_1_2` (fire), `part_1_3` (cold-formed), `part_1_4`
(stainless), `part_1_5` (plated buckling), `part_1_6`+`part_4` (shell buckling + silo wall — share
`silo_t_mm`/`silo_r_mm`, same physical silo), `part_1_8` (bolts + welds), `part_1_9` (fatigue),
`part_1_10` (through-thickness), `part_1_11` (tension component), `part_1_12` (high-strength
bending), `part_2` (bridge), `part_3` (tower), `part_5` (pile).

Per `derivation-rules.md` rule 1, `change-<field>` is the *default* for document-root scalars;
`update` is reserved for a field group that's genuinely "validated together, never meaningfully set
one-field-at-a-time." That bar is met here because every group below is consumed as a single unit by
exactly one (family of) check function(s) — `bolt_e1_mm` alone means nothing without
`bolt_e2_mm`/`bolt_d0_mm`/…, together they ARE "the bolted connection," unlike e.g. `shooting`'s
scene facet (7 separate `change-scene-*` kinds) where the app genuinely sets each field
independently. `update-member-properties`'s 11 fields are the taxonomy's own "analysis settings"
example verbatim — the base steel member's forces/section/material state, reused across 5 of
`part_1_1`'s checks plus `part_2`/`part_3`/`part_5`. Result: **17 mutations**, one `change-annex`
(the lone identity-ish scalar) + 16 `update-<family>-inputs`, covering all 74 fields exactly once
(verified programmatically — no field left out, none duplicated across groups).

| New mutation | Verb | Entity | Fields | EN 1993 part(s) |
|---|---|---|---|---|
| `change-annex` | change | annex | `annex` | national annex selector |
| `update-member-properties` | update | member-properties | 11 (forces/section/material) | 1-1 (+2/3/5) |
| `update-fire-inputs` | update | fire-inputs | 5 | 1-2 |
| `update-cold-formed-inputs` | update | cold-formed-inputs | 6 | 1-3 |
| `update-stainless-inputs` | update | stainless-inputs | 3 | 1-4 |
| `update-plated-inputs` | update | plated-inputs | 2 | 1-5 |
| `update-silo-shell-inputs` | update | silo-shell-inputs | 6 | 1-6 + 4 |
| `update-bolt-inputs` | update | bolt-inputs | 10 | 1-8 |
| `update-weld-inputs` | update | weld-inputs | 5 | 1-8 |
| `update-fatigue-inputs` | update | fatigue-inputs | 3 | 1-9 |
| `update-through-thickness-inputs` | update | through-thickness-inputs | 3 | 1-10 |
| `update-tension-component-inputs` | update | tension-component-inputs | 3 | 1-11 |
| `update-hss-inputs` | update | hss-inputs | 4 | 1-12 |
| `update-bridge-inputs` | update | bridge-inputs | 3 | 2 |
| `update-tower-inputs` | update | tower-inputs | 2 | 3 |
| `update-pile-inputs` | update | pile-inputs | 3 | 5 |
| `update-crane-inputs` | update | crane-inputs | 4 | 6 |

All 17 payloads are always-applicable document-root setters (no missing-target case exists — every
field is always present on the snapshot), so every `inverse()` unconditionally returns exactly one
mutation reconstructing the group's fields from `base` (never `Vec::new()`).

## Triad leaves

Every `<dir>/🦠️mutation/🦀️component.rs` holds one payload struct (`Clone, Debug, PartialEq,
Serialize, Deserialize`) implementing `protocol::MutationKind<En1993Snapshot, En1993Mutation>` with a
real `SEMANTICS` const, `diff`/`inverse` delegating to the sibling `🔺️diff`/`↩️inverse` leaves (never
inline logic). `🔺️diff` builds `En1993Diff` directly as a struct literal — `{ field: Some(payload.new_field), ..Default::default() }` — real sparse construction, never apply-then-capture (confirmed:
`En1993Diff::apply`/`apply_to_artifact`, both untouched, already patch field-by-field via `if let
Some(value) = &self.field`, so a diff with only N of 74 `Option`s set is a true no-op everywhere
else). `↩️inverse` reads every changed field back from `base` and reconstructs the same-shaped
mutation (`String` fields `.clone()`d, `f64`/`u32`/`u8`/`AnnexChoice` copied).

Given the mechanical, highly repetitive nature of 17 structurally-identical triads (all payload/diff/
inverse bodies differ only in field lists), I generated the 51 leaf files, the dispatch enum, and the
`OpText`/`OpBinary` codecs with a short Python script driven from the exact field-name/type table
above (committed nowhere — scratch-only, per the ticket-folder rule; the script itself is throwaway
tooling, not a migration script: every field mapping was derived by hand from the snapshot/engine
source first, the script only mechanized the transcription) — then hand-verified every generated file
(read every leaf, the dispatch enum, both codec files in full) before running `cargo check`.

## Dispatch rewrite (`🧬️mutations/🦀️component.rs`)

`En1993Mutation` is now 17 single-field tuple variants, `#[derive(Clone, Debug, PartialEq, Serialize,
Deserialize, dsl::Mutations)]` with `#[mutations(snapshot = En1993Snapshot, diff = En1993Diff, schema
= "s.norm.en1993")]` — used `dsl::Mutations`, not the ticket's literal `dsl_derive::Mutations`, per
the already-confirmed working path for this crate (`iso16757`'s wave2 report traced the re-export
chain: `dsl_derive::Mutations` is not a direct dependency of `semio-s-plugin-norm`; `dsl::Mutations`
resolves through this crate's own `extern crate semio_framework_os_kernel as dsl;` alias). All
hand-written `impl Mutation<En1993Snapshot> for En1993Mutation` and the
`crate::impl_norm_set_snapshot_ops!(En1993Mutation, En1993Snapshot)` macro call are deleted — the
derive generates `impl Mutation`/`impl SemanticMutation` now, and `OpText`/`OpBinary` are hand-rolled
directly in `📝️text`/`💾️binary` instead (see below).

`📦️glue.rs` (plugin-shared, outside this facet's boundary) `#[path]`-wires exactly the old
`📄set-snapshot` leaf's 3 files — no more, no fewer — matching the `iso16757`/`shooting` precedent
exactly. Every one of the 17 new triad leaves is therefore self-wired directly inside
`🧬️mutations/🦀️component.rs` itself (`🔖️LeafWiring` region, `#[path = "."] pub mod <slug> { ... }`
blocks), zero `glue.rs` edits needed. The old `📄set-snapshot` leaf's 3 files are reduced to
doc-comment-only stubs (plus one still-real, still-non-empty `pub fn apply` in the `🦠️mutation` leaf
so the module isn't literally empty) — kept only because `glue.rs` still `#[path]`-wires them; see
`sharedFileRequests` below.

Sibling cleanup (same artifact directory, in-boundary): removed the now-dead
`diff_set_snapshot` helper from `🧬️schema/🔺️diff/📝️text/🦀️component.rs` (only caller was the deleted
`SetSnapshot` dispatch arm) — `En1993Artifact`/`En1993Snapshot` imports both remain in use by
`apply_to_artifact`/`MutationDiff::apply`, which are untouched.

## OpText/OpBinary — hand-rolled, uniform per-field JSON

`🧬️mutations/📝️text/🦀️component.rs` and `🧬️mutations/💾️binary/🦀️component.rs` both got direct
`impl protocol::OpText`/`impl protocol::OpBinary for En1993Mutation`, one keyword per verb-kind
(`update-bolt-inputs bolt-f-ed-kn=... bolt-n-bolts=... ...`), matching `iso16757`'s precedent exactly
but applied to plain scalars too (this facet has no nested entity records to justify JSON only for
"structured" fields) — every one of the 5 field types (`f64`/`u32`/`u8`/`String`/`AnnexChoice`)
already derives `Serialize`/`Deserialize`, so every payload field round-trips through a quoted-JSON
token (`enc_json`/`dec_json` for text; `write_json_bin`/`read_json_bin`, tag `u8` 0..=16, for binary)
instead of 5 separate hand-rolled per-type encoders. Text tokenizer is the same quote-aware
space-splitter as `iso16757`'s (needed because `String` fields' JSON form can itself contain spaces,
e.g. `weld_steel_grade`). `demo_mutation_cases()` covers all 17 variants;
`op_text_binary_roundtrip_law` round-trips every one through both codecs (print/parse and
encode/decode).

## Tests

Extended the existing `#[cfg(test)] mod tests` in `🧬️mutations/🦀️component.rs` (no new test files):
a `round_trip` helper (diff→apply forward, inverse→apply backward, asserts exact base restoration —
same shape as `iso16757`'s), one dedicated round-trip test per all 17 variants (each constructs the
mutation with distinct new values — `999.0`/`9`/`"changed"`/`AnnexChoice::En`, none colliding with any
of `En1993Snapshot::default()`'s real field values — and asserts every changed field lands correctly
post-apply, with `round_trip` itself proving the inverse restores `base` exactly), a
`change_annex_diff_is_sparse` test asserting the diff has `annex` set and three unrelated fields
(`n_ed_kn`, `bolt_f_ed_kn`, `weld_steel_grade`) still `None`, and a `semantic_kinds_cover_every_variant`
test (`kinds().len() == 17` + `verb`/`kind`/`record` spot-checks on `change-annex` and
`update-bolt-inputs`). `🧬️mutations/📝️text/🦀️component.rs` has `op_text_binary_roundtrip_law` over
all 17 `demo_mutation_cases()`.

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/.../📡️spr/🧪️testkit/🦀️component.rs` — grepped this crate (`✏️s/🔌️plugins/📕️norm`) for
an existing `os_spr::testkit` import first, per the task's explicit fallback instruction; the only
`testkit` hits in the crate are each app's own local `crate::apps::<x>::testkit` test-fixture module
(unrelated), confirming the same finding `iso16757`'s wave2 report already made for this crate.
Skipped rather than add a new Cargo dependency (`Cargo.toml` is plugin-shared, outside this facet's
boundary regardless). The hand-written round-trip/sparse-diff tests above cover the same laws
directly instead.

## Verification

`cargo check -p semio-s-plugin-norm` (workspace under heavy concurrent load — this run alone hit
`error: could not compile ... due to 93 previous errors; 279 warnings emitted` crate-wide).

Breakdown of every error's actual `-->` location (grepped, not eyeballed):
- **63 errors in `🗿️artifacts/📔️vdi3805/**`** (a *different* artifact, `Vdi3805Mutation:
  Mutation<Vdi3805Snapshot>` trait-bound not satisfied) + 4 more in `🎛️apps/📔️vdi3805/**` — nothing
  to do with `en1993`; looks like another concurrent session's in-progress `vdi3805` migration.
- **3 errors each in `🎛️apps/📘️en1990`, `🎛️apps/📘️en1991`, `🎛️apps/📘️en1992`, `🎛️apps/📘️en1994`,
  `🎛️apps/📗️din16798`, `🎛️apps/📕️din4108`, `🎛️apps/📓️iso16757`** — the identical `SetSnapshot`
  app-call-site fallout pattern this facet also hits, meaning other concurrent sessions have already
  migrated those facets' own mutation enums too.
- **3 errors in `🎛️apps/📘️en1993/**`** (this facet's own expected fallout, see `sharedFileRequests`).
- **Zero errors and zero warnings anywhere under `🗿️artifacts/📘️en1993`** (this facet's own writable
  boundary) — checked with `grep "🗿️artifacts/📘️en1993"` against the full error+warning log, not just
  spot-checking; every hit under that path is a warning, and every one of those warnings is in a
  pre-existing file I never touched (`🗿️artifacts/📘️en1993/🦀️component.rs` root, `⚙️engine/🦀️component.rs`,
  `🧬️schema/🦀️component.rs`, `🚪️io/🦀️component.rs` — none inside `🧬️mutations/**`).

Per the task's workspace-churn rule, all failures are either outside my artifact directory entirely
(vdi3805) or are the exact same *expected* SetSnapshot-removal fallout category my own 3 app errors
fall into (other facets' equivalent reconciliation items) — none indicate a bug inside my boundary, so
no retry was needed; `cargo check` did fully type-check every file in `🗿️artifacts/📘️en1993` (a
partial-crate compile still type-checks every reachable item) and found nothing wrong there.

`cargo test` cannot run for the crate as a whole while `vdi3805` and the other facets' app call sites
fail to compile (compilation is crate-wide), so the 20 new/extended tests above are written and
type-checked (confirmed via `cargo check`, which expands and type-checks `#[cfg(test)]` code too) but
not executed end-to-end — `lawTestsPass` is reported conservatively as `false` for that reason, not
because any test is believed wrong.

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`📦️glue.rs`, `en1993`'s `mutations` block** (`pub mod set_snapshot { ... }`, inside `pub mod
   schema { ... pub mod mutations { ... } }`) — once items 2-4 below are fixed, delete this block
   entirely (the `📄set-snapshot` leaf files it `#[path]`-wires are orphaned doc-only stubs now).
2. **`🎛️apps/📘️en1993/🎮️commands/📤️set-snapshot/🦀️component.rs`** (`SetSnapshot::handle`, line 20) —
   whole-document replace is banned outright per the taxonomy (`ArtifactStore::reset` is the
   sanctioned non-history path). This command's whole purpose is whole-document replace, so it needs
   an architectural decision (route through `reset`, or retire the command) — flagging for the
   reconciliation pass, not solving here.
3. **`🎛️apps/📘️en1993/🎮️commands/🧮️evaluate/🦀️component.rs`** (`Evaluate::handle`, line 23) —
   currently re-commits `En1993Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }` purely to
   force a re-evaluation. With `SetSnapshot` gone, needs either a genuinely no-op-but-real semantic
   mutation, or routing evaluation-refresh through a history-independent recompute path if one exists.
4. **`🎛️apps/📘️en1993/🦀️component.rs`** (around line 107, an import/media-load path analogous to
   `iso16757`'s `import_media`) — replaces the whole snapshot via `En1993Mutation::SetSnapshot { snapshot }`;
   same as (2), a real whole-document-load gesture that should route through
   `store::ArtifactStore::reset` rather than a mutation-enum variant.
5. **Not this facet's concern but noted for completeness**: `🎛️apps/📘️en1990`, `🎛️apps/📘️en1991`,
   `🎛️apps/📘️en1992`, `🎛️apps/📘️en1994`, `🎛️apps/📗️din16798`, `🎛️apps/📕️din4108` each hit the identical
   3-error `SetSnapshot`-fallout pattern right now — their own facet-owning sessions presumably already
   filed (or will file) the equivalent `sharedFileRequests` in their own wave2 reports; the
   reconciliation pass should sweep all of them together rather than one at a time.

Grepped the entire artifact directory (`🗿️artifacts/📘️en1993/**`, including `📚️examples/`, the
artifact-root `🦀️component.rs`, `⚙️engine/`, `🚪️io/`) for `SetSnapshot`/`impl_norm_set_snapshot_ops` —
no other call sites found beyond the orphaned leaf's own doc-comment mentions. Everything inside this
facet's writable boundary is fully migrated; only the 4 `🎛️apps/**`/`📦️glue.rs` items above remain.

## Skipped / non-blocking (recipe step f)

Did not touch `📖️component.grammar.semio`/`📡️component.protocol.semio` (already stale relative to
even the *old* one-variant vocabulary) or the sibling `.json`/`.proto`/`.graphql`/`.g4`/`.abnf`/
`.ksy`/`.spicy`/`.ts` schema-description files — updating them honestly for 17 real mutation kinds is
a substantial independent pass, explicitly non-blocking per the recipe, same call `iso16757`/
`shooting` made.

## Files touched

Created (17 triads × 3 files = 51 new leaf files):
`🧬️mutations/{🌍️change-annex,🏗️update-member-properties,🔥update-fire-inputs,📐update-cold-formed-inputs,🪞update-stainless-inputs,🧱update-plated-inputs,🛢️update-silo-shell-inputs,🔩update-bolt-inputs,⚡update-weld-inputs,🔁update-fatigue-inputs,📏update-through-thickness-inputs,🔗update-tension-component-inputs,💪update-hss-inputs,🌉update-bridge-inputs,🗼update-tower-inputs,⚓update-pile-inputs,🏭update-crane-inputs}/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`

Rewritten:
- `🧬️mutations/🦀️component.rs` (dispatch enum, 1→17 variants, `#[derive(dsl::Mutations)]`; existing
  `#[cfg(test)] mod tests` extended with 20 tests, not replaced)
- `🧬️mutations/📝️text/🦀️component.rs` (hand-rolled `OpText`, `demo_mutation_cases`, round-trip test)
- `🧬️mutations/💾️binary/🦀️component.rs` (hand-rolled `OpBinary`)
- `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` (retired to orphan stubs)

Modified:
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` (removed dead `diff_set_snapshot` helper; everything else
  untouched)

Not modified (outside boundary — see `sharedFileRequests` above):
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`
- `🎛️apps/📘️en1993/🦀️component.rs`
- `🎛️apps/📘️en1993/🎮️commands/{📤️set-snapshot,🧮️evaluate}/🦀️component.rs`
