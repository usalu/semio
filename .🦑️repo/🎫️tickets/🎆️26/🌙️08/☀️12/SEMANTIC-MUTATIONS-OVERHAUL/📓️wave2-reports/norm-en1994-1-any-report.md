# Wave 2 — `norm/en1994` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`.

## Snapshot shape (why 22 flat `change-*` mutations, no CRUD/collections)

`En1994Snapshot` (`📸️snapshot/🦀️component.rs`) is a flat calculation worksheet for EN 1994
(composite steel/concrete): 22 document-root persistent scalars — one enum (`annex:
AnnexChoice`), two `String`s (`fire_rating`, `deck_type`, `fatigue_detail` — three, corrected
below), and the rest `f64` (design actions, resistances, stud/material properties, fire and
fatigue parameters). No id-keyed collection, no ordered/index-keyed list, no relationship/edge
field, no hierarchy — every one of `en1994`'s twenty-two fields is independently user-settable
(confirmed against `⚙️engine/🦀️component.rs`'s `check_full_composite`/`evaluate`, which reads
every field as a free calculation input, and the `📥️inputs` window, which just JSON-dumps the
whole snapshot with no grouped-editing UI). Per `📓️derivation-rules.md` rule 1 this is the
"document-level scalars" case: `change-<field>` per scalar, no `update-<facet>` grouping (no
subset of these 22 fields is validated/set together as an inseparable unit — each is an
independent EN 1994 input parameter), no `rename-*` (there is no name/key identity field on this
artifact). Deleted the generic single-variant `En1994Mutation::SetSnapshot { snapshot }` (whole
-document replace, banned per taxonomy) entirely and replaced it with this 22-variant vocabulary.

| New mutation | Verb | Entity | Field | Type |
|---|---|---|---|---|
| `change-annex` | change | annex | `annex` | `AnnexChoice` |
| `change-m-ed-knm` | change | m-ed-knm | `m_ed_knm` (design moment M_Ed) | `f64` |
| `change-v-ed-kn` | change | v-ed-kn | `v_ed_kn` (design shear V_Ed) | `f64` |
| `change-m-pla` | change | m-pla | `m_pla` (steel plastic moment) | `f64` |
| `change-m-pl-rd` | change | m-pl-rd | `m_pl_rd` (plastic moment resistance) | `f64` |
| `change-eta` | change | eta | `eta` (shear connection degree η) | `f64` |
| `change-vl-rd` | change | vl-rd | `v_l_rd` (longitudinal shear resistance) | `f64` |
| `change-insulation-thickness-mm` | change | insulation-thickness-mm | `insulation_thickness_mm` | `f64` |
| `change-fire-rating` | change | fire-rating | `fire_rating` | `String` |
| `change-deck-type` | change | deck-type | `deck_type` | `String` |
| `change-delta-sigma-mpa` | change | delta-sigma-mpa | `delta_sigma_mpa` (bridge fatigue stress range) | `f64` |
| `change-fatigue-detail` | change | fatigue-detail | `fatigue_detail` | `String` |
| `change-d-mm` | change | d-mm | `d_mm` (stud diameter) | `f64` |
| `change-h-sc-mm` | change | h-sc-mm | `h_sc_mm` (stud height) | `f64` |
| `change-f-ck-mpa` | change | f-ck-mpa | `f_ck_mpa` (concrete strength) | `f64` |
| `change-fu-mpa` | change | fu-mpa | `f_u_mpa` (stud ultimate strength) | `f64` |
| `change-e-cm-mpa` | change | e-cm-mpa | `e_cm_mpa` (concrete modulus) | `f64` |
| `change-v-ed-per-stud-kn` | change | v-ed-per-stud-kn | `v_ed_per_stud_kn` | `f64` |
| `change-span-m` | change | span-m | `span_m` | `f64` |
| `change-fy-mpa` | change | fy-mpa | `f_y_mpa` (steel yield strength) | `f64` |
| `change-n-cycles-stud` | change | n-cycles-stud | `n_cycles_stud` | `f64` |
| `change-delta-tau-stud-mpa` | change | delta-tau-stud-mpa | `delta_tau_stud_mpa` | `f64` |

**Naming note on 3 slugs** (`change-vl-rd`, `change-fu-mpa`, `change-fy-mpa`): the derive's
`to_kebab` merges a run of single-uppercase-letter segments that isn't itself followed by a
lowercase-starting segment (`VLRd` → `vl-rd`, not `v-l-rd`; same shape as `FUMpa`/`FYMpa`). Since
`#[derive(dsl::Mutations)]` compile-time-asserts `SEMANTICS.kind == kebab(variant name)`, these
three slugs are the variant name's *actual* kebab form rather than a literal underscore-for-dash
swap of the Rust field name — verified by hand against the derive's exact `to_kebab` algorithm
before writing any files, and confirmed by `cargo check` raising zero
`SEMANTICS.kind must equal ... kebab form` compile errors across all 22 variants.

## Triad leaves

All 22 are the same shape (`🦠️mutation` payload + delegating `MutationKind` impl, `🔺️diff` builds
`En1994Diff { <field>: Some(payload.new_<field>), ..Default::default() }` directly from the
payload — `En1994Diff` already carries one `Option<T>` per snapshot field, so no new diff
plumbing was needed — `↩️inverse` reads `base.<field>` and re-emits the same variant). Every
`inverse()` is unconditional (every field always exists on a flat scalar snapshot, so there is no
"missing target" case here — unlike an id-keyed collection's `delete`).

Self-wired directly inside `🧬️mutations/🦀️component.rs` (`🔖️LeafWiring` region, 22
`#[path = "."] pub mod change_<field> { pub mod mutation; pub mod diff; pub mod inverse; }`
blocks) rather than in `📦️glue.rs` — same pattern as the already-migrated `iso16757`/`mathematical`
facets, since `📦️glue.rs` is plugin-shared and out of this facet's writable boundary.

## Orphaned `📄set-snapshot` leaf

`📦️glue.rs` (`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs`, out of this facet's boundary)
`#[path]`-wires `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`
directly (`pub mod set_snapshot { pub mod mutation; pub mod diff; pub mod inverse; }` inside its
`en1994::standards::v1::subsets::any::schema::mutations` block) — those 3 files could not be
deleted without breaking that wiring. Rewrote them to orphaned stubs (doc-comment explaining the
ban + a still-referenced `apply()` helper kept alive in the `🦠️mutation` file so it stays
non-dead-code; `🔺️diff`/`↩️inverse` are doc-comment-only), exactly mirroring the `iso16757`
precedent. Cleanup (deleting the `set_snapshot` block from `📦️glue.rs`) is a `sharedFileRequests`
item below.

## Wire codecs (`OpText`/`OpBinary`)

Old `crate::impl_norm_set_snapshot_ops!(En1994Mutation, En1994Snapshot)` macro invocation
(plugin-shared macro in `📄️artifact/🦀️component.rs`, assumes a single `SetSnapshot` variant) is
gone — hand-rolled `OpText`/`OpBinary` for the 22-variant enum now live in
`🧬️mutations/📝️text/🦀️component.rs`. Grammar: `change-<field> new-<field>=<value>` (one keyword,
one arg — no JSON-per-field wrapping needed except for `annex`, the only non-primitive-scalar
field, which round-trips through a quoted JSON string reusing its existing
`Serialize`/`Deserialize`). Binary: `tag u8 | value` per variant — `f64` as fixed 8 little-endian
bytes, `String` length-prefixed UTF-8, `annex` length-prefixed JSON. `demo_mutation_cases()`
covers all 22 variants; `op_text_binary_roundtrip_law` round-trips every one through both codecs.
`🧬️mutations/💾️binary/🦀️component.rs` needed no edits — it already just delegates to whatever
`OpBinary` impl exists for `En1994Mutation`.

## Tests

Extended the existing `🧪️Tests` region (no new test files) in `🧬️mutations/🦀️component.rs`:
`demo_mutation_cases()` (one representative value per variant) + `every_variant_round_trips_and
_restores_base` (diff→apply→inverse→apply round-trip law over all 22, from `En1994Snapshot::
default()`), plus targeted tests for `change-annex`, `change-m-ed-knm` (incl. explicit inverse
value check), `change-fire-rating`, `change-eta` (explicit inverse value check against a non
-default base), `semantic_kinds_cover_every_variant` (`kinds().len() == 22` + one
`semantics()`/`verb` check), and a `label()` human-readability check.
`🧬️mutations/📝️text/🦀️component.rs` has its own `op_text_binary_roundtrip_law` over all 22 demo
cases.

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/.../📡️spr/🧪️testkit/🦀️component.rs` — grepped `✏️s/🔌️plugins/📕️norm` for an existing
`testkit`/`os_spr::testkit` import first, per instructions; none exists in this crate, and
`Cargo.toml` is plugin-shared (out of this facet's writable boundary regardless), so per the
task's explicit fallback this step was skipped. The hand-written round-trip/inverse tests above
cover the same laws directly.

## Verification

`cargo check -p semio-s-plugin-norm --tests`. This is a heavily concurrently-edited workspace;
first two attempts failed entirely inside `semio-s-plugin-stdio` (a dependency of
`semio-s-plugin-norm`, itself outside `📕️norm` entirely) with two *different* transient errors
(`cannot find inferences in schema`, then a "file not found" on a `docx` artifact file) — textbook
concurrent-workspace churn from another session actively editing `stdio`'s artifacts, not
reachable/fixable from this facet. A third attempt got past `stdio` and fully checked
`semio-s-plugin-norm`, surfacing **45 errors, all of the shape `no variant named SetSnapshot found
for enum <Artifact>Mutation`**, spread across `iso16757`, `vdi3805`, `en1990`–`en1999`, `din4108`,
`din16798`, and `en1994` — i.e. every norm sibling artifact whose `SetSnapshot` generic variant a
concurrent wave-2 session has already removed, same as this facet. **Exactly 5 of the 45 are
`en1994`**, all in `🎛️apps/📘️en1994/**` (out of this facet's writable boundary):
- `🎛️apps/📘️en1994/🎮️commands/📤️set-snapshot/🦀️component.rs:20,41`
- `🎛️apps/📘️en1994/🎮️commands/🧮️evaluate/🦀️component.rs:23,38`
- `🎛️apps/📘️en1994/🦀️component.rs:107`

All 5 are `En1994Mutation::SetSnapshot` construction sites — exactly the app-level call sites
identified below as `sharedFileRequests`. **Zero errors anywhere under this facet's own artifact
directory.** The only warnings touching `🗿️artifacts/📘️en1994/**` are pre-existing, in files this
facet didn't touch (`🦀️component.rs` unused-import warnings on the artifact-root re-exports,
`⚙️engine`/`🚪️io` unused-import/lint warnings) — not introduced here. The compile-time
`SEMANTICS.kind == kebab(variant)` assertion the derive generates per variant passed for all 22
(no such error appeared), independently confirming the 3 non-obvious slugs above (`change-vl-rd`/
`change-fu-mpa`/`change-fy-mpa`) are correct. `cargo test` cannot be run for this crate as a whole
until the 5 app-level sites (here) plus the sibling facets' own app-level sites are updated by the
dedicated reconciliation pass — `lawTestsPass` is reported conservatively as `false` for that
reason, not because any test is believed wrong; the round-trip/inverse-law tests above are written
and were type-checked clean by the same `cargo check --tests` run (no error appeared anywhere in
this facet's own test code).

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`📦️glue.rs`, `en1994`'s `mutations` block** (the `pub mod set_snapshot { pub mod mutation;
   pub mod diff; pub mod inverse; }` sub-block inside `en1994::standards::v1::subsets::any::
   schema::mutations`) — once items 2–4 below are fixed, delete this block entirely (the
   `📄set-snapshot` leaf files it `#[path]`-wires are orphaned stubs now).
2. **`🎛️apps/📘️en1994/🎮️commands/📤️set-snapshot/🦀️component.rs`** (`SetSnapshot::handle`, line
   20) — whole-document replace is banned outright per the taxonomy (`ArtifactStore::reset` is the
   sanctioned non-history path). This command's whole purpose is whole-document replace, so it
   needs an architectural decision (route through `reset`, or retire the command) rather than a
   mechanical swap — flagging for the reconciliation pass.
3. **`🎛️apps/📘️en1994/🎮️commands/🧮️evaluate/🦀️component.rs`** (`Evaluate::handle`, line 23) —
   currently re-commits `En1994Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }` purely to
   force a re-evaluation. With `SetSnapshot` gone, this needs either a genuinely no-op-but-real
   semantic mutation, or routing evaluation-refresh through a history-independent recompute path —
   an architectural call for the reconciliation pass (same shape as the `iso16757` precedent's
   identical `evaluate` command).
4. **`🎛️apps/📘️en1994/🦀️component.rs`** (`import_media`, line 107) — replaces the whole snapshot
   from an imported media file via `En1994Mutation::SetSnapshot { snapshot }`; should route through
   `store::ArtifactStore::reset` (its non-history sanctioned path) rather than a mutation-enum
   variant.

Grepped the entire artifact directory (`🗿️artifacts/📘️en1994/**`, including `📚️examples/`, the
artifact-root `🦀️component.rs`, `⚙️engine/`) for `SetSnapshot` — no other call sites found beyond
the orphaned leaf's own doc-comment mentions. Everything inside this facet's writable boundary is
fully migrated; only the 4 `🎛️apps/**`/`📦️glue.rs` items above remain, and they require the
plugin-wide reconciliation pass (other sibling facets have the identical 3–5-site pattern).

## Files touched (all inside this facet's writable boundary)

- `🧬️mutations/🦀️component.rs` — rewritten: 22-variant `#[derive(dsl::Mutations)]` dispatch enum +
  `🔖️LeafWiring` self-wiring + extended `🧪️Tests` region.
- `🧬️mutations/📝️text/🦀️component.rs` — rewritten: hand-rolled `OpText`/`OpBinary` for the new
  enum, demo cases, round-trip law test.
- `🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` — orphaned to stubs
  (still referenced by `📦️glue.rs`, out of boundary).
- 22 new triad-leaf directories, 3 files each (`🦠️mutation`, `🔺️diff`, `↩️inverse`), one per
  `change-<field>` mutation listed in the table above.
