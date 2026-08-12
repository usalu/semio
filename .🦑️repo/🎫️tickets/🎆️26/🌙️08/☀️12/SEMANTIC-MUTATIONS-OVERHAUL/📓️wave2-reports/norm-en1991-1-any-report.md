# Wave 2 — `norm/en1991` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`.

## Snapshot shape

`En1991Snapshot` (`📸️snapshot/🦀️component.rs`) is a flat bag of **32 independent document-root
scalars** — general (area, imposed category, national annex) plus one block per Eurocode 1 action
family: self-weight (material/thickness/assumed load), fire (curve/resistance/member capacity),
snow (zone/altitude/characteristic load), wind (zone/basic velocity), thermal (ΔT), construction
activity, accidental impact (mass/speed), bridge traffic (lane count/span/lane width/moment
resistance), crane/hoist (class/class/speed), silo (bulk density/height/hydraulic radius/μ/k), and
the two wind size/dynamic factors `c_s`/`c_d`. **No id-keyed or index-keyed collections, no
relationships, no hierarchy, no identity field** (no `name`/`key`) — this is a load-calculation
input form, not a structured document.

Per `📓️derivation-rules.md` rule 1 ("`change-<field>` per remaining scalar; `update-<facet>` ONLY
for an inseparable ≥2-field facet that's never meaningfully set one-field-at-a-time"), none of the
per-part groups (self-weight/fire/snow/wind/bridge/crane/silo) are validated as one atomic facet —
each is its own independently editable input in a calc-parameters form — so this derives exactly
**one `change-<field>` mutation per scalar (32 total)** rather than inventing `update-*` facets the
schema doesn't ask for. No `rename-*` exists (no identity field to rename).

## What landed

Deleted the generic single-variant `En1991Mutation::SetSnapshot { snapshot }` (whole-document
replace) and replaced it with a 32-variant semantic vocabulary, each a single-field tuple wrapping a
real `🦠️mutation`/`🔺️diff`/`↩️inverse` triad leaf, dispatched via `#[derive(dsl::Mutations)]`
(`#[mutations(snapshot = En1991Snapshot, diff = En1991Diff, schema = "s.norm.en1991")]`), mirroring
the wave0 `MiniMutation` fixture and the already-fanned-out `norm/iso16757` facet's shape (same
plugin/crate — confirmed `dsl::Mutations`, not the bare `dsl_derive::Mutations` path used by
`mathematical`/`demonstrator-playground`, is the one that actually resolves from this crate; see
that facet's own wave2 report for the trace).

| New mutation (`change-<field>`) | Kind (`SemanticDescriptor.kind`) | Field type |
|---|---|---|
| area | `change-area-m2` | `f64` |
| imposed category | `change-category` | `ImposedCategory` (enum) |
| national annex | `change-annex` | `AnnexChoice` (enum) |
| self-weight material | `change-self-weight-material` | `String` |
| self-weight thickness | `change-self-weight-thickness-m` | `f64` |
| assumed self-weight load | `change-assumed-gk-kn-m2` | `f64` |
| fire curve | `change-fire-curve` | `FireCurve` (enum) |
| fire resistance | `change-fire-resistance-min` | `f64` |
| fire member capacity factor | `change-fire-member-capacity-c` | `f64` |
| snow zone | `change-snow-zone` | `u8` |
| snow altitude | `change-snow-altitude-m` | `f64` |
| characteristic snow load | `change-en-sk-kn-m2` | `f64` |
| wind zone | `change-wind-zone` | `u8` |
| basic wind velocity | `change-en-vbms` | `f64` |
| thermal delta | `change-delta-tk` | `f64` |
| construction activity | `change-construction-activity` | `String` |
| accidental impact mass | `change-accidental-mass-t` | `f64` |
| accidental impact speed | `change-accidental-speed-km-h` | `f64` |
| bridge lane count | `change-bridge-lane` | `u8` |
| bridge span | `change-bridge-span-m` | `f64` |
| bridge lane width | `change-bridge-lane-width-m` | `f64` |
| bridge moment resistance | `change-bridge-moment-resistance-knm` | `f64` |
| crane class | `change-crane-class` | `String` |
| hoist class | `change-hoist-class` | `String` |
| hoisting speed | `change-hoisting-speed-ms` | `f64` |
| silo bulk density | `change-silo-bulk-density-kn-m3` | `f64` |
| silo height | `change-silo-height-m` | `f64` |
| silo hydraulic radius | `change-silo-hydraulic-radius-m` | `f64` |
| silo friction coefficient | `change-silo-mu` | `f64` |
| silo lateral pressure ratio | `change-silo-k` | `f64` |
| wind size factor c_s | `change-cs` | `f64` |
| wind dynamic factor c_d | `change-cd` | `f64` |

**Kebab-merge note**: `SemanticDescriptor.kind` is required (by the derive's own compile-time
assertion) to equal `to_kebab(variant_name)` exactly, using the derive's own algorithm
(`🗣️dsl/✨️derive/🦀️component.rs`'s `to_kebab`, region `🔖️VariantHelpers`). For 7 fields built
from several single-letter segments with no lowercase separator between them
(`assumed_g_k_kn_m2`, `en_s_k_kn_m2`, `en_v_b_m_s`, `delta_t_k`, `hoisting_speed_m_s`, `c_s`, `c_d`)
that algorithm merges adjacent all-caps runs (a boundary only forms before an uppercase letter that
is itself followed by a lowercase one), so e.g. `ChangeEnVBMS` kebabs to `change-en-vbms` rather
than the field-literal `change-en-v-b-m-s`. Verified this by porting the derive's exact `to_kebab`
function to Python and computing every `kind` from it before writing any file (rather than
discovering the mismatch via a failed compile-time assertion) — the payload's own `new_<field>`
Rust field name, the diff/inverse bodies, and the text-codec's `new-<field>` arg keys all still
address/print the exact original snapshot field name regardless of how the *kind* slug merged.

Every `diff()` is a real one-field `En1991Diff { <field>: Some(payload.new_<field>.clone()),
..Default::default() }` sparse construction (never apply-then-capture) — trivial because
`En1991Diff` (`🔺️diff/🦀️component.rs`, untouched — outside this facet, and already the right
sparse shape) already carries one `Option<T>` per scalar field. Every `inverse()` reads `base` (the
pre-state) and returns a single `change-<field>` mutation carrying `base`'s old value — since every
field is a required (non-`Option`) scalar on `En1991Snapshot`, there is no "missing target" case;
inverse is always exactly one mutation, never `Vec::new()`, for this facet.

Hand-rolled `OpText`/`OpBinary` for the new enum in `🧬️mutations/📝️text/🦀️component.rs` (the
derive only generates `Mutation`/`SemanticMutation`, never the wire codecs, matching the
`iso16757` precedent exactly) — `keyword new-<field>=<value>` grammar (every variant has exactly
one arg), quote-aware tokenizer reused verbatim from that precedent. `f64`/`u8` fields print via
`{}`/parse via `.parse()`; `String` fields via a quoted-string codec (`enc_str`/`dec_str`); the 3
enum fields (`category`/`annex`/`fire_curve`) via a quoted-JSON codec (`enc_json`/`dec_json`) since
they already derive `Serialize`/`Deserialize`. Binary form is `tag u8 | field bytes`: native
little-endian `f64`/single-byte `u8`, length-prefixed UTF-8 for `String`, length-prefixed JSON for
the 3 enums. `demo_mutation_cases()` covers all 32 variants and `op_text_binary_roundtrip_law`
round-trips every one through both codecs.

## Mechanism note: self-wiring + orphaned `📄set-snapshot` leaf

`📦️glue.rs` is out of this facet's writable boundary (plugin-shared), but it `#[path]`-wires
`🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` directly. Those 3
files could not be deleted or renamed without breaking that wiring. Fix, matching the
`iso16757`/`playground`/`mathematical` precedent exactly: the 32 new triad leaves are self-wired
directly inside `🧬️mutations/🦀️component.rs` itself (`🔖️LeafWiring` region, 32
`#[path = "."] pub mod <slug> { ... }` blocks) — zero `glue.rs` edits needed for the new
vocabulary. The old `📄set-snapshot` leaf's 3 `.rs` files were rewritten to orphaned stubs (a doc
comment + a still-referenced-nowhere `apply()` helper in the `🦠️mutation` file so it stays a real,
non-empty, non-dead-code module — `pub fn`s are exempt from `dead_code` lint regardless of callers;
the `🔺️diff`/`↩️inverse` files are doc-comment-only, matching the `iso16757` stub shape exactly) —
dead code kept alive only because `glue.rs`'s `#[path]`s still point at them; see
`sharedFileRequests` below. The leaf's 3 `.ts` files were already stubs (`export {};`) that never
referenced `SetSnapshot` and needed no change.

`💾️binary/🦀️component.rs` (the thin `encode_op`/`decode_op` wrapper) needed **zero changes** — it
already just forwards to whatever implements `OpBinary` for `En1991Mutation`, which the new hand-
rolled impl in `📝️text/🦀️component.rs` now provides.

## Deliberately not touched (documentation-only, non-blocking per the task)

Left `🧬️mutations/📖️component.grammar.semio`, `💾️binary/📡️component.protocol.semio`, and the
sibling `.graphql`/`.json`/`.proto`/`.g4`/`.ebnf`/`.ksy`/`.abnf`/`.spicy` schema-description files
as their pre-existing generic stubs — matching the `iso16757` precedent, which also left these
untouched. Step (f) of the task is explicitly "not blocking."

## Tests

Extended the existing `🧪️Tests` region (no new test files) in `🧬️mutations/🦀️component.rs` with
32 per-variant round-trip tests (one per `change-<field>` mutation, each asserting `diff().apply()`
produces the new value AND that `inverse()` composed back restores the exact base fixture) plus
`semantic_kinds_cover_every_variant` (`kinds().len() == 32` + a `kind`/`record` spot check),
`change_category_inverse_restores_base_category` (explicit inverse-payload assertion), and
`change_of_a_string_field_undoes_to_default_value` (same for a `String` field). `📝️text/🦀️component.rs`
has `op_text_binary_roundtrip_law` over all 32 `demo_mutation_cases()`.

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/.../📡️spr/🧪️testkit/🦀️component.rs` — grepped this crate (`✏️s/🔌️plugins/📕️norm`)
for an existing `testkit`/`os_spr::testkit` import first, per instructions; none exists (confirmed
by the `iso16757` precedent's identical finding). Per the task's explicit fallback this step was
skipped rather than risk adding a new Cargo dependency (`Cargo.toml` is also plugin-shared, outside
this facet's writable boundary regardless). The hand-written round-trip/inverse tests above cover
the same laws directly instead.

## Verification

`cargo check -p semio-s-plugin-norm --tests`, run twice (workspace under heavy concurrent load from
other sessions; first attempt's dependency crate `semio-s-plugin-stdio` itself failed to build on an
unrelated `📰xml` artifact — transient, gone on retry, consistent with house policy on concurrent
workspace churn: retried, didn't chase it).

Second run: **136 errors total, none inside this facet's own directory.** Grepped every error's
`-->` location against `📘️en1991` and confirmed exactly **5**, all in `🎛️apps/📘️en1991/**` (out of
this facet's writable boundary) — the exact same 5-call-site pattern as `iso16757`'s wave2 report:
- `🎛️apps/📘️en1991/🎮️commands/📤️set-snapshot/🦀️component.rs:20,41`
- `🎛️apps/📘️en1991/🎮️commands/🧮️evaluate/🦀️component.rs:23,38`
- `🎛️apps/📘️en1991/🦀️component.rs:107`

All 5 are `En1991Mutation::SetSnapshot` construction sites — exactly the app-level call sites
identified below as `sharedFileRequests`, not a new/different bug. The remaining ~131 errors belong
to *other artifacts* in the same crate that other concurrent sessions are mid-migrating right now
(64 from a currently-broken `Vdi3805Mutation`, plus `SetSnapshot`/derive-related errors scattered
across `en1993`/`en1994`/`din4108`/`din16798`/`iso16757` apps and even `en1990`/`en1992` — none of
which reference `en1991` anywhere, confirmed by grep) — unrelated to this ticket, not chased, per
house policy on concurrent workspace churn. **Zero errors and zero warnings anywhere under this
facet's own artifact directory** (`🧬️schema/🧬️mutations/**`, verified by grepping every `-->`
location, not just the top-line error count).

`cargo test` cannot be run for this crate as a whole until the crate-wide compile errors above (5
expected/documented ones plus ~131 unrelated ones from other sessions' in-progress work) are
resolved — compilation is crate-wide; the lib/test binary can't be built while any file in the
crate fails to compile — so the round-trip/inverse-law tests above are written and confirmed
type-correct (they compiled cleanly under `cargo check --tests`) but not yet executed end-to-end.
`lawTestsPass` is reported conservatively as `false` for that reason, not because any test is
believed wrong.

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`📦️glue.rs`, `mutations` block** (the `pub mod set_snapshot { ... }` block inside
   `pub mod en1991 { ... pub mod schema { ... pub mod mutations { ... } } }`) — once items 2-4
   below are fixed and this facet's new vocabulary is confirmed compiling end-to-end, delete this
   block entirely (the `📄set-snapshot` leaf files it `#[path]`-wires are orphaned stubs now).
2. **`🎛️apps/📘️en1991/🎮️commands/📤️set-snapshot/🦀️component.rs`** (`SetSnapshot::handle`, line
   20) — whole-document replace is banned outright per the taxonomy (`ArtifactStore::reset` is the
   sanctioned non-history path, entirely outside `Emit`/the `Mutation` enum). This command's whole
   purpose is whole-document replace, so it needs an architectural decision (route it through
   `reset` instead of `Emit`, or retire the command) rather than a mechanical swap — flagging for the
   reconciliation pass to decide, not solving here.
3. **`🎛️apps/📘️en1991/🎮️commands/🧮️evaluate/🦀️component.rs`** (`Evaluate::handle`, line 23) —
   currently re-commits `En1991Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }` purely to
   force a re-evaluation. With `SetSnapshot` gone, this needs either a genuinely no-op-but-real
   semantic mutation, or (more honest) routing evaluation-refresh through the store's
   history-independent recompute path if one exists — another architectural call for the
   reconciliation pass.
4. **`🎛️apps/📘️en1991/🦀️component.rs`** (`import_media`, line 107) — replaces the whole snapshot
   from an imported media file via `En1991Mutation::SetSnapshot { snapshot }`; same as (2), this is
   a real whole-document-load gesture and should route through `store::ArtifactStore::reset` (its
   non-history sanctioned path) rather than a mutation-enum variant.

Grepped the entire artifact directory (`🗿️artifacts/📘️en1991/**`, including `📚️examples/`, the
artifact-root `🦀️component.rs`, `⚙️engine/`, `🚪️io/`) for `SetSnapshot`/`impl_norm_set_snapshot_ops`
— no other call sites found beyond the orphaned leaf's own doc-comment mentions. Everything inside
this facet's writable boundary is fully migrated; only the 4 `🎛️apps/**`/`📦️glue.rs` items above
remain.

## Files touched

Created (32 triad leaves × 3 files = 96 new files) under
`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`:
`📐change-area-m2/`, `🏷️change-category/`, `📗change-annex/`, `🧱change-self-weight-material/`,
`📏change-self-weight-thickness-m/`, `⚖️change-assumed-gk-kn-m2/`, `🔥change-fire-curve/`,
`⏱️change-fire-resistance-min/`, `🧯change-fire-member-capacity-c/`, `❄️change-snow-zone/`,
`⛰️change-snow-altitude-m/`, `☃️change-en-sk-kn-m2/`, `🌬️change-wind-zone/`, `💨change-en-vbms/`,
`🌡️change-delta-tk/`, `🏗️change-construction-activity/`, `💥change-accidental-mass-t/`,
`🚗change-accidental-speed-km-h/`, `🌉change-bridge-lane/`, `🛣️change-bridge-span-m/`,
`↔️change-bridge-lane-width-m/`, `🔩change-bridge-moment-resistance-knm/`, `🪝change-crane-class/`,
`⛓️change-hoist-class/`, `⬆️change-hoisting-speed-ms/`, `🌾change-silo-bulk-density-kn-m3/`,
`🗼change-silo-height-m/`, `💧change-silo-hydraulic-radius-m/`, `📉change-silo-mu/`,
`🔢change-silo-k/`, `🌀change-cs/`, `🌪️change-cd/` — each with `🦠️mutation/🦀️component.rs`,
`🔺️diff/🦀️component.rs`, `↩️inverse/🦀️component.rs`.

Rewritten:
- `🧬️mutations/🦀️component.rs` — dispatch enum (32 variants, `#[derive(dsl::Mutations)]`) +
  `🔖️LeafWiring` self-wiring region + extended `🧪️Tests` region.
- `🧬️mutations/📝️text/🦀️component.rs` — hand-rolled `OpText`/`OpBinary` + `demo_mutation_cases` +
  round-trip test.
- `🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`, `.../🔺️diff/🦀️component.rs`,
  `.../↩️inverse/🦀️component.rs` — orphaned to doc-comment stubs.

Unchanged (verified compatible, no edit needed): `🧬️mutations/💾️binary/🦀️component.rs`,
`🧬️mutations/📄set-snapshot/*/🟦️component.ts` (already stubs), `🔺️diff/**` (sibling facet,
already the right sparse shape), `📸️snapshot/**` (sibling facet).
