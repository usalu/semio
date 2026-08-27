# Wave 2 — `norm/iso16757` (standard 1, subset `any`) — mutations facet

## Facet
`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`.

## What landed

Deleted the generic single-variant `Iso16757Mutation::SetSnapshot { snapshot }` (whole-document
replace) and replaced it with a 21-variant semantic vocabulary, each a single-field tuple wrapping a
real `🦠️mutation`/`🔺️diff`/`↩️inverse` triad leaf, dispatched via `#[derive(dsl::Mutations)]`
(`#[mutations(snapshot = Iso16757Snapshot, diff = Iso16757Diff, schema = "s.norm.iso16757")]`),
mirroring the wave0 `MiniMutation` fixture and the already-fanned-out `mathematical`/`shooting`
facets' shape.

`Iso16757Snapshot` is a genuinely large, rich document (ISO 16757 parts 1/2/4/5: a full product
catalogue, a data dictionary, a geometry catalogue, a selection request, part-number rule/inputs,
script limits, exchange-process stage) — far from the "trivial snapshot" shortcut. Given the size,
this pass derived a substantial but intentionally bounded vocabulary rather than exhaustively
covering every nested collection in one sitting (documented as deferred below, same recipe applies).

| New mutation | Verb | Entity | Notes |
|---|---|---|---|
| `change-exchange-process{new_exchange_process}` | change | exchange-process | doc-root scalar |
| `update-script-limits{new_max_steps,new_max_recursion,new_timeout_ms}` | update | script-limits | inseparable 3-field facet |
| `replace-part-number-rule{new_rule}` | replace | part-number-rule | `Literal`/`Table`/`Script` variants differ structurally |
| `change-part-number-input{key,new_value}` | change | part-number-input | upsert on the `BTreeMap<String,CatalogueValue>`; undo of a fresh key is `remove` |
| `remove-part-number-input{key}` | remove | part-number-input | |
| `change-selection-class{new_class_id}` | change | selection-class | |
| `change-selection-series{new_series_id}` | change | selection-series | |
| `add-selection-constraint{constraint}` | add | selection-constraint | appends to `Vec<SelectionConstraint>` |
| `remove-selection-constraint{index}` | remove | selection-constraint | BASE-state index |
| `rename-catalogue{new_name}` | rename | catalogue | `catalogue.metadata.names.preferred.text` |
| `rename-manufacturer{new_name}` | rename | manufacturer | `catalogue.manufacturer.names.preferred.text` |
| `create-product-group{product_group,index}` / `delete-product-group{id}` / `rename-product-group{id,new_name}` | create/delete/rename | product-group | full CRUD, `catalogue.product_groups` |
| `create-product{product,index}` / `delete-product{id}` / `rename-product{id,new_name}` | create/delete/rename | product | full CRUD, `catalogue.products` |
| `create-property-definition{property_definition,index}` / `delete-property-definition{id}` | create/delete | property-definition | `catalogue.property_definitions` |
| `create-subject{subject,index}` / `delete-subject{id}` | create/delete | subject | `dictionary.subjects` |

**Deliberately deferred** (documented in the dispatch file's header doc comment, same recipe applies
to a follow-up ticket): `product_classes`, `product_series`, `product_indexes`,
`descriptive_objects`, `accessories`/`compositions` edge maps, dictionary `relationships`/
`properties`/`controlled_lists`/`meta_subjects`, and `geometry` (objects map + primitive registry).
None of these were invented into the 21 above; they're honestly left out rather than forced.

`Iso16757Diff` is a coarse, pre-existing sparse-diff shape — one `Option<WholeSubtree>` per
snapshot-root field (`catalogue`, `dictionary`, `geometry`, `selection`, `part_number_rule`,
`part_number_inputs`, `script_limits`, `exchange_process`), not field-level deltas inside those
subtrees. Every `diff()` here is handcrafted directly from `base`: clone the addressed root field,
patch just the targeted nested value with real logic (`Vec::retain`/`insert`/`iter_mut().find` etc.,
never apply-then-capture), wrap in `Some(..)`. Every `inverse()` reads `base` (pre-state):
`delete-*`'s inverse re-`create`s the captured entity at its original index; `create-*`'s inverse is
`Vec::new()` when the id already existed in `base` (no-op create); `change-part-number-input`'s
inverse is `remove-part-number-input` when the key was absent in `base` (upsert semantics, no
`NoMutation` sentinel); `rename-*`/`change-*` invert to `Vec::new()` when the target is missing.

Hand-rolled `OpText`/`OpBinary` for the new enum in `🧬️mutations/📝️text/🦀️component.rs` (the
derive only generates `Mutation`/`SemanticMutation`, never the wire codecs) — `keyword key=value ...`
grammar, quote-aware tokenizer, binary tag `0..=20` + varint/string fields. Structured payload
fields (`ProductGroup`/`Product`/`PropertyDefinition`/`Subject`/`SelectionConstraint`/
`PartNumberRule`/`CatalogueValue`/`ExchangeProcess`) round-trip through a quoted JSON string
(`enc_json`/`dec_json`, resp. `write_json_bin`/`read_json_bin`) rather than a second handcrafted
grammar per structured type — every one of them already derives `Serialize`/`Deserialize`, so this
reuses that losslessly (same technique `mathematical`'s `replace-graph` used for its whole-graph
field). `demo_mutation_cases()` covers all 21 variants (23 cases incl. both `Option` arms of
`change-selection-series`) and `op_text_binary_roundtrip_law` round-trips every one through both
codecs.

## Mechanism note: `dsl::Mutations`, not `dsl_derive::Mutations`

The wave0 report and the `mathematical`/`demonstrator-playground` precedent files use
`#[derive(dsl_derive::Mutations)]`. That bare path is **not resolvable from this crate**:
`semio-s-plugin-norm`'s `Cargo.toml` has no direct dependency on
`semio-framework-os-kernel-dsl-derive` (only `mathematical`'s/`playground`'s glue.rs chains happen to
predate this being checked — see below). Traced the actual re-export chain: the kernel crate's own
`📦️glue.rs` does `extern crate self as dsl;` + `pub mod os_dsl { pub use component::*; ... }` (where
`🗣️dsl/🦀️component.rs` does `pub use dsl_derive::{..., Mutations};`) + a crate-root
`pub use crate::os_dsl::*;` — so `Mutations` **is** reachable at `semio_framework_os_kernel::Mutations`,
i.e. as `dsl::Mutations` (or equivalently `protocol::Mutations`) through norm's own
`extern crate semio_framework_os_kernel as dsl;` alias in its `📦️glue.rs` — exactly the path
`shooting`'s already-migrated facet uses. Switched the dispatch enum's derive to `dsl::Mutations`;
confirmed this is the actual working path via `cargo check` (see Verification). Flagging this for
whoever owns wave0/the other fanned-out facets — `mathematical`'s crate could not be independently
confirmed either way (its `cargo check` hits an unrelated pre-existing `glue.rs` path bug before
reaching this derive at all, per its own wave2 report), so this may be a latent issue there too.

## Mechanism note: self-wiring + orphaned `📄set-snapshot` leaf

`📦️glue.rs` is out of this facet's writable boundary (plugin-shared), but it `#[path]`-wires
`🧬️mutations/📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` directly (lines
132-140 of the pre-ticket file). Those 3 files could not be deleted or renamed without breaking that
wiring. Fix, matching the `playground`/`mathematical` precedent exactly: the 21 new triad leaves are
self-wired directly inside `🧬️mutations/🦀️component.rs` itself (`🔖️LeafWiring` region, 21
`#[path = "."] pub mod <slug> { #[path = "<dir>/🦠️mutation/🦀️component.rs"] pub mod mutation; ... }`
blocks) — zero `glue.rs` edits needed for the new vocabulary. The old `📄set-snapshot` leaf's 3
files were rewritten to orphaned stubs (a doc comment + a still-referenced-nowhere `apply()` helper
in the `🦠️mutation` file so it stays a real, non-empty, non-dead-code module; the `🔺️diff`/
`↩️inverse` files are doc-comment-only) — dead code kept alive only because `glue.rs`'s `#[path]`s
still point at them; see sharedFileRequests below for the cleanup this needs.

## Tests

Extended the existing `🧪️Tests` region (no new test files) in `🧬️mutations/🦀️component.rs` with 11
tests: round-trip + inverse-law style checks for `change-exchange-process`, `update-script-limits`,
`change-part-number-input`/`remove-part-number-input` (incl. the fresh-key-undo-is-remove case),
`change-selection-class`+`add`/`remove-selection-constraint`, `rename-catalogue`/`rename-manufacturer`,
full `create`→`rename`→`delete` sequences for `product-group` and `product` (plus a
missing-id-has-empty-inverse check), `create`→`delete` for `property-definition` and `subject`, and
`kinds().len() == 21` + `semantics()` checks. `🧬️mutations/📝️text/🦀️component.rs` has
`op_text_binary_roundtrip_law` over all 23 `demo_mutation_cases()`.

**Not done**: `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` from
`🧰️framework/.../📡️spr/🧪️testkit/🦀️component.rs` — grepped this crate (`✏️s/🔌️plugins/📕️norm`)
for an existing `testkit`/`os_spr::testkit` import first, per instructions; none exists. Per the
task's explicit fallback this step was skipped rather than risk adding a new Cargo dependency
(`Cargo.toml` is also plugin-shared, outside this facet's writable boundary regardless). The
hand-written round-trip/inverse tests above cover the same laws directly instead.

## Verification

`cargo check -p semio-s-plugin-norm --tests` (workspace is under heavy concurrent load from other
sessions — retried after the fix, each run took several minutes due to build-lock contention, not
this facet's code size).

First run (before the `dsl::Mutations` fix) surfaced 101 errors, 100 of which cascaded from the
single `dsl_derive` resolution bug above, plus 14 genuine bugs of mine (`part_1::Names`/
`part_1::LocalizedText`/`part_1::Cardinality` — these three types actually live at
`crate::artifacts::iso16757::{Names, LocalizedText, Cardinality}`, the top-level "Shared" region;
`part_1` only privately `use super::*`s them, so `part_1::Names` etc. is not a valid public path).
Both classes of bug were inside my own dispatch/text files and have been fixed.

Second run, after both fixes: **exactly 5 errors, all in `🎛️apps/📓️iso16757/**`** (out of this
facet's writable boundary):
- `🎛️apps/📓️iso16757/🎮️commands/📤️set-snapshot/🦀️component.rs:20,41`
- `🎛️apps/📓️iso16757/🎮️commands/🧮️evaluate/🦀️component.rs:23,38`
- `🎛️apps/📓️iso16757/🦀️component.rs:107`

All 5 are `Iso16757Mutation::SetSnapshot` construction sites — exactly the app-level call sites
identified below as `sharedFileRequests`, not a new/different bug. **Zero errors and zero warnings
anywhere under this facet's own artifact directory.** `cargo test` cannot be run for this crate as a
whole until those 5 app-level sites are updated by the dedicated reconciliation pass (compilation is
crate-wide; the lib/test binary can't be built while any file in the crate fails to compile), so the
hand-inspected round-trip/inverse-law tests above are written and type-checked but not yet executed
end-to-end — `lawTestsPass` is reported conservatively as `false` for that reason (not because any
test is believed wrong).

## sharedFileRequests (for the plugin-wide app-reconciliation pass)

1. **`📦️glue.rs`, `mutations` block** (the `pub mod set_snapshot { ... }` block at lines ~132-140 of
   the pre-ticket file, inside `pub mod schema { ... pub mod mutations { ... } }`) — once items 2-4
   below are fixed and this facet's new vocabulary is confirmed compiling end-to-end, delete this
   block entirely (the `📄set-snapshot` leaf files it `#[path]`-wires are orphaned stubs now).
2. **`🎛️apps/📓️iso16757/🎮️commands/📤️set-snapshot/🦀️component.rs`** (`SetSnapshot::handle`, line
   20) — whole-document replace is banned outright per the taxonomy (`ArtifactStore::reset` is the
   sanctioned non-history path, entirely outside `Emit`/the `Mutation` enum). This command's whole
   purpose is whole-document replace, so it needs an architectural decision (route it through
   `reset` instead of `Emit`, or retire the command) rather than a mechanical swap — flagging for the
   reconciliation pass to decide, not solving here.
3. **`🎛️apps/📓️iso16757/🎮️commands/🧮️evaluate/🦀️component.rs`** (`Evaluate::handle`, line 23) —
   currently re-commits `Iso16757Mutation::SetSnapshot { snapshot: doc.snapshot.clone() }` purely to
   force a re-evaluation (see its own doc comment: "a no-op whole-document commit is the honest way
   to record 'the user asked for a fresh evaluation'"). With `SetSnapshot` gone, this needs either a
   genuinely no-op-but-real semantic mutation, or (more honest) routing evaluation-refresh through
   the store's history-independent recompute path if one exists — another architectural call for the
   reconciliation pass.
4. **`🎛️apps/📓️iso16757/🦀️component.rs`** (`import_media`, line 107) — replaces the whole snapshot
   from an imported media file via `Iso16757Mutation::SetSnapshot { snapshot }`; same as (2), this is
   a real whole-document-load gesture and should route through `store::ArtifactStore::reset` (its
   non-history sanctioned path) rather than a mutation-enum variant.
5. **Latent `dsl_derive::Mutations` resolution risk in already-migrated sibling facets** (see the
   mechanism note above) — `mathematical`'s and `demonstrator-playground`'s dispatch enums use bare
   `#[derive(dsl_derive::Mutations)]`; this crate's Cargo.toml doesn't expose that path directly (only
   `dsl`/`protocol`/`store`/`vcs` aliases to the kernel crate do). Worth a follow-up check once
   `mathematical`'s unrelated `glue.rs` path bug (its own wave2 report, item 1) is fixed, to confirm
   whether it actually compiles or was never verified past that blocker.

Grepped the entire artifact directory (`🗿️artifacts/📓️iso16757/**`, including `📚️examples/`, the
artifact-root `🦀️component.rs`, `⚙️engine/`) for `SetSnapshot`/`impl_norm_set_snapshot_ops` — no
other call sites found beyond the orphaned leaf's own doc-comment mentions. Everything inside this
facet's writable boundary is fully migrated; only the 4 `🎛️apps/**`/`📦️glue.rs` items above remain.
