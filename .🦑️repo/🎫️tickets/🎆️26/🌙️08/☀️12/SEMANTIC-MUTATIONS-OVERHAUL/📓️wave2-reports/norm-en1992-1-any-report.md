# Wave 2 — `norm/en1992/standards/1/subsets/any` mutations facet

## Facet

`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
(crate `semio-s-plugin-norm`).

## Derivation

`En1992Snapshot` is a flat, id-less, document-root parameter form: thirty-five persistent scalar
fields feeding the bending/shear, fire, bridge-fatigue, liquid-retaining crack-width and anchor
checks. No id-keyed collections, no name/identity field to `rename`, no ordered/index-keyed
collections, no relationship/hierarchy fields. Per `derivation-rules.md` rule 1 ("change-<field>
per remaining scalar"), every one of the 35 fields becomes its own `change-<field>` mutation. None
qualify for the `update-<facet>` grouping exception — each check input (e.g. `m_ed_knm`, `v_ed_kn`,
`fire_rating`, `anchor_cracked`) is independently measured/entered in the host UI, never validated
as an atomic multi-field bundle, so grouping them would invent structure not present in the schema.

This mirrors this ticket's `din16798`/`vdi3805` precedents (same plugin, same "flat scalar norm
calculator" artifact shape) almost exactly, including the same `📄set-snapshot`-repurposing trick
(see below).

## What changed

- **Deleted** the generic `En1992Mutation::SetSnapshot { snapshot }` variant and its hand-written
  `impl protocol::Mutation` dispatch (`diff`/`inverse` matching on `SetSnapshot`) — banned outright
  per `taxonomy.md`. Also removed the `crate::impl_norm_set_snapshot_ops!(...)` macro call that
  supplied `OpText`/`OpBinary` via the old variant.
- **Added** 35 semantic `change-<field>` mutations, one per `En1992Snapshot` scalar field
  (`annex`, `m_ed_knm`, `v_ed_kn`, `f_ck`, `b_mm`, `d_mm`, `a_s_mm2`, `f_yk`, `rho_l`, `n_ed_kn`,
  `p_kn`, `a_c_mm2`, `use_fem`, `span_m`, `udl_kn_m`, `fire_rating`, `provided_axis_distance_mm`,
  `bridge_sigma_c_mpa`, `bridge_delta_sigma_s_mpa`, `tightness_class`, `hd_over_h`,
  `liquid_sigma_s_mpa`, `liquid_rho_p_eff`, `liquid_f_ct_eff_mpa`, `liquid_e_s_mpa`,
  `liquid_s_r_max_mm`, `anchor_h_ef_mm`, `anchor_cracked`, `anchor_f_uk_mpa`, `anchor_f_yk_mpa`,
  `anchor_a_s_mm2`, `anchor_d_mm`, `anchor_c1_mm`, `anchor_n_ed_kn`, `anchor_v_ed_kn`). Each is a
  real triad leaf: `🦠️mutation` (payload struct + `impl protocol::MutationKind<En1992Snapshot,
  En1992Mutation>` with a real `SEMANTICS` const, delegating `diff`/`inverse` to the sibling
  leaves), `🔺️diff` (handcrafted sparse `En1992Diff { <field>: Some(payload.new_<field>), ..
  Default::default() }`, never apply-then-capture), `↩️inverse` (handcrafted `vec![ChangeX { new_x:
  base.x.clone() }]`, reconstructed from captured BASE state — `change` is its own inverse partner
  per the taxonomy).
- **Repurposed `📄set-snapshot/` in place** (kept its physical directory name — `📦️glue.rs`
  path-includes that exact triad and is outside this facet's writable boundary) to hold
  `ChangeAnnex` instead of the banned `SetSnapshot`, exactly like `din16798`/`vdi3805` did. The
  other 34 triads are self-wired directly in `🧬️mutations/🦀️component.rs` via nested
  `#[path = "."] pub mod <name> { ... }` blocks, so no other shared file needed touching.
- **Dispatch enum** (`🧬️mutations/🦀️component.rs`): now
  `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]`
  `#[mutations(snapshot = En1992Snapshot, diff = En1992Diff, schema = "s.norm.en1992")]` with 35
  single-field tuple variants; the old hand-written `impl protocol::Mutation` block is gone — the
  derive generates `Mutation`/`SemanticMutation` now.
- **`🧬️mutations/📝️text/🦀️component.rs`**: rewrote the hand-rolled `OpText`/`OpBinary` for all 35
  variants (grammar `change-<field> new-<field>=<value>`, one keyword per variant), following the
  `iso16757` precedent exactly (scalar fields encode natively — `f64` via its own binary/decimal
  form, `bool` via `true`/`false`/`0`/`1` — the three enum-typed fields, `annex`, `fire_rating`,
  `tightness_class`, round-trip through a quoted JSON string since they already derive
  `Serialize`/`Deserialize`). Added `demo_mutation_cases()` (one value per variant) and an
  `op_text_binary_roundtrip_law` test. The old test referencing `En1992Mutation::SetSnapshot` is
  gone.
- **`🧬️mutations/💾️binary/🦀️component.rs`**: untouched — it only wraps `OpBinary::encode_op`/
  `decode_op` generically and already imported `...mutations::text::En1992Mutation`, so it needed
  no edits.
- **Tests** (existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs`, extended, no new test
  files): `every_mutation()` fixture (35 entries), `round_trip` helper via `vcs::apply_mutation`,
  `every_variant_registers_an_approved_semantic_descriptor`, `every_variant_round_trips_via_inverse`,
  plus three `protocol::testkit::assert_mutation_inverse_law` /
  `assert_mutation_diff_absorb_law` law tests (per task step e) on the three most structurally
  distinct variants: `change-annex` (enum), `change-m-ed-knm` (`f64`), `change-use-fem` (`bool`).
  `semio-s-plugin-norm` already depends on `semio-framework-os-kernel` (aliased `protocol`), whose
  `testkit` submodule is reachable as `protocol::testkit` (confirmed already in use by
  `din16798`/`vdi3805` in this same crate), so no new Cargo dependency was needed.

## Naming

Every `kind`/`entity`/`record` string was derived by literally running the same `to_kebab`
algorithm the `#[derive(dsl_derive::Mutations)]` macro uses at compile time
(`🗣️dsl/✨️derive/🦀️component.rs`'s `to_kebab`) against each chosen `Change<Field>` variant ident, so
the compile-time `SEMANTICS.kind == own kebab` assertion is satisfied by construction. A few
fields with adjacent single-letter segments collapse under that algorithm's acronym rule exactly
like `HTTPServer → http-server` does — e.g. `a_s_mm2 → change-as-mm2` (not `change-a-s-mm2`),
`a_c_mm2 → change-ac-mm2`, `liquid_e_s_mpa → change-liquid-es-mpa`. This is intentional and
required for the derive to compile, not an inconsistency.

## Deferred (per task step f, "not blocking")

`🧬️mutations/📖️component.grammar.semio` and `💾️binary/📡️component.protocol.semio` were left
unchanged (still describe the old generic shape) — consistent with the `iso16757` precedent in
this same plugin, which also left them stale after its own migration.

## sharedFileRequests

App-level emit call sites outside this facet's writable boundary (`🎛️apps/**`) still construct
`En1992Mutation::SetSnapshot { snapshot }`, which no longer exists. Per the task instructions these
were **not** edited; a later cross-facet pass should update:

- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🦀️component.rs:107` — `import_media` builds
  `En1992Mutation::SetSnapshot { snapshot }`; needs a real semantic mutation (or a
  `store::ArtifactStore::reset`-based import path instead, per taxonomy's "whole-document replace
  goes through `reset`, not the mutation enum" rule).
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🎮️commands/📤️set-snapshot/🦀️component.rs:20,41` — the
  `setSnapshot` command itself constructs/matches `En1992Mutation::SetSnapshot`; likely the whole
  command should be retired in favour of `reset`, mirroring the taxonomy's decision that
  whole-document replace is deliberately NOT an in-history mutation.
- `✏️s/🔌️plugins/📕️norm/🎛️apps/📘️en1992/🎮️commands/🧮️evaluate/🦀️component.rs:23,38` — same
  `SetSnapshot`-construction pattern used to commit an evaluated snapshot back to history; needs
  reworking to emit the specific `change-*` mutations that actually changed, or another
  non-`SetSnapshot` path.
- (Not en1992-specific, but observed while verifying:) the identical `SetSnapshot`-in-`🎛️apps/**`
  pattern is currently broken for `en1990`, `en1991`, `en1993`, `en1994`, `din4108`, `din16798`,
  `iso16757`, `vdi3805` too — every norm facet that has already migrated off `SetSnapshot` this
  wave. A single consolidated pass across `🎛️apps/**` (per plugin, or per app) will fix all of
  these at once rather than one at a time.

## Verification

`cargo check -p semio-s-plugin-norm` / `cargo check -p semio-s-plugin-norm --tests`: **zero**
errors and **zero** warnings originate from any file inside this facet's artifact directory (after
one cleanup: dropped an unused `protocol::SemanticMutation` import from the tests region — the
methods are all called through fully-qualified `<En1992Mutation as protocol::SemanticMutation<..>>`
paths). The only warning ever attributed to a line in this facet's `🧬️mutations/🦀️component.rs` is
`` `testkit` is ambiguous `` (a pre-existing framework-level glob-import ambiguity in
`semio-framework-os-kernel`'s `📦️glue.rs`, between `os_spr::*` and `os_pack::*` both exporting a
`testkit` module — present identically for `din16798`'s already-landed use of the same
`protocol::testkit::assert_mutation_inverse_law` call, not introduced by this change, not inside
this facet's writable boundary to fix).

The crate-wide `cargo check -p semio-s-plugin-norm` still exits non-zero, entirely because of
`no variant named 'SetSnapshot'` errors in `🎛️apps/**` — 5 each for `en1992` (this facet, expected,
tracked above) and identically for `en1990`/`en1991`/`en1993`/`en1994`/`din4108`/`din16798`/
`iso16757`/`vdi3805` (other facets' own already-landed wave2 work, not mine, not fixed here per the
task's explicit instruction not to touch `🎛️apps/**`). Retried three times across ~10 minutes of
wall time per the workspace-churn protocol; watched two genuinely transient failures come and go in
the process (a `vdi3805` `dsl_derive` scope error, and a `semio-s-plugin-stdio` dependency-crate
`E0433` from a different plugin's concurrent edit) — both resolved themselves on retry, confirming
they were other sessions' in-flight work, not anything in this facet.

Could not run `cargo test -p semio-s-plugin-norm` — the whole crate (this facet's `🎛️apps/**`
included) fails to link a test binary until the `sharedFileRequests` above are addressed in a later
pass, exactly the same blocker every other already-migrated norm facet in this crate currently has.
Correctness of the 35 `diff`/`inverse` pairs and the 3 `protocol::testkit` law-test bodies was
verified by type-checking (`cargo check --tests` compiles them successfully) and by mirroring the
proven-passing `din16798`/`iso16757` triad shape field-for-field; it was **not** confirmed by an
actual test run, and this report does not claim otherwise.

## Files touched (108)

- `🧬️mutations/🦀️component.rs` (rewritten: dispatch enum + leaf wiring + tests)
- `🧬️mutations/📝️text/🦀️component.rs` (rewritten: `OpText`/`OpBinary` + grammar codec + demo cases + tests)
- `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` (repurposed in place → `ChangeAnnex`)
- 34 new leaf directories `🧬️mutations/🔧change-<field>/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`

Not touched: `🧬️mutations/💾️binary/🦀️component.rs` (already generic, needed no edit), any `.ts`/
`.json`/`.graphql`/`.proto`/`.semio` file, `📦️glue.rs`, plugin-root `🦀️component.rs`, `🎛️apps/**`,
any other artifact or plugin.
