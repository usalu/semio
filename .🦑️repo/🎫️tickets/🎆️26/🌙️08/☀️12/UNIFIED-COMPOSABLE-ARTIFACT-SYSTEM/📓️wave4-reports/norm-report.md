# W4 batch reasoning+norm — `norm` (15 EN/DIN/VDI/ISO artifacts)

**ucas-status: partial (LocalizedText×2 duplication genuinely killed, verified, tested; full
`C:document,table,value R:fem` composition investigated in depth across representative artifacts
and found architecturally inappropriate for this plugin's actual content model — see below, same
"verify before implementing" discipline `fem`'s report used for its "11-type dup" claim)**

## What the codebase actually looks like (verified against code, not assumed from the design doc)

`✏️s/🔌️plugins/📕️norm/🗿️artifacts/` has exactly **15** top-level artifact roots — the design doc's
count is correct, verified by direct listing:

`📓️iso16757`, `📔️vdi3805`, `📕️din4108`, `📗️din16798`, `📘️en1990`, `📘️en1991`, `📘️en1992`,
`📘️en1993`, `📘️en1994`, `📘️en1995`, `📘️en1996`, `📘️en1997`, `📘️en1998`, `📘️en1999`, `📙️din18599`.

Crate: `semio-s-plugin-norm` (`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml`). Total Rust across
the 15 artifact dirs: **~59.9k lines** (`wc -l` on every `*.rs` under each artifact root), close to
the design doc's "~80,000 lines" once TS glue/tests are counted — the scale claim checks out.

**Baseline** (`CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-norm --all-targets`,
before any edit): **0 errors**, 237 lib warnings + 278 test warnings (pre-existing, unrelated —
unused imports/dead code, `cargo fix` suggestions only). Full baseline output saved at
`scratch-norm-check1.txt` (in this ticket folder, produced during the run that first surfaced the
one real compile error described below).

## Part 1 — `LocalizedText` duplication: found, killed, verified

Confirmed by direct grep (`grep -rln "LocalizedText" ✏️s/🔌️plugins/📕️norm`, both before and after
this edit) that exactly **two** artifacts hand-maintained their own copy, and no third copy exists
anywhere else in the plugin:

| Artifact | Old shape | Field names | Semantics |
|---|---|---|---|
| `📔️vdi3805` (`🦀️component.rs:17`) | `{ de: String, en: String }` | fixed German+English pair | `CatalogueProduct.title` — always exactly bilingual |
| `📓️iso16757` (`🦀️component.rs:45`) | `{ locale: String, text: String }` | one locale-tagged string | `Names.preferred`/`Names.alternatives`/`Subject.definition` — a general locale tag |

These are **not byte-identical** (unlike `fem`'s `FemDof`/`FemAnalysisSettings`, which really were
copy-paste duplicates) — different field names, different arity semantics (fixed pair vs. general
tag). Killing the duplication therefore meant **converging both onto one canonical shape**, not
just moving one definition and re-exporting it verbatim.

### Design decision: local value type, not a composed child

stdio's `✳️text` subset's own doc comment (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/🦀️component.rs:1-6`)
says outright: *"Absorbs the duplicated `LocalizedText` types that currently exist twice inside the
norm plugin."* Its `SemioTextRun{language,content,marks}` is structurally the natural target
(`locale`↔`language`, `text`↔`content`). But `LocalizedText` is nested by the **dozens** as an
ordinary leaf field (`Names.preferred`, `Names.alternatives: Vec<_>`, `Subject.definition`,
`CatalogueProduct.title`, …) — not a single top-level content slot. A composed
`store::ArtifactChild<SemioTextSnapshot>` is for exactly one big content slot with its own
working-scene cache (`📓️migration-recipe.md` §1/§3); wrapping every one of dozens of tiny leaf
fields in a child handle would mean dozens of working-scene cache entries per document and no
real "content" being extracted — not what the pattern is for. Also, `SemioTextRun` has no
`dsl::DslField` impl (it isn't meant to be nested inside a `dsl::DslRecord`-derived struct), and
norm can't add one for it from its own crate (orphan rule: neither the type nor the trait is
local).

**Decision**: define ONE canonical `LocalizedText { locale: String, text: String }` (structurally
isomorphic to `SemioTextRun`, `dsl::DslRecord`-derived, kept a local plain value type — the same
"hand `DslField` bridge" convention these very files already use for `VdiValue`/`CatalogueValue`)
in `crate::document` (`✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs`), the plugin's existing
cross-artifact shared module (`NormError`/`QuantityKind` already live there and are imported by
every artifact the same way). Both artifacts now `pub use crate::document::LocalizedText;`.

### vdi3805's shape change (the real work)

`iso16757`'s `{locale,text}` shape is IDENTICAL to the new canonical type — zero call-site changes
needed beyond the `pub use`. `vdi3805`'s `{de,en}` fixed pair genuinely cannot represent "both
languages at once" with a single `LocalizedText` any more (that's a general single-locale-tagged
string now) — so `CatalogueProduct.title: LocalizedText` became
**`#[dsl(table)] title: Vec<LocalizedText>`**, matching `iso16757`'s own already-established
`Names.alternatives: Vec<LocalizedText>` convention exactly. This is a genuine improvement, not
just a rename: the old struct hardcoded exactly-German-plus-English; the new list is honestly
extensible to any locale set, and now both artifacts express "a name/title in N languages" the
same way.

Added two local helpers alongside the type (`vdi3805/🦀️component.rs`):
```rust
pub fn bilingual(de: impl Into<String>, en: impl Into<String>) -> Vec<LocalizedText> { .. }
pub fn text_in(variants: &[LocalizedText], locale: &str) -> String { .. }
```
`bilingual` replaces every old `LocalizedText::new(de, en)` call site; `text_in` replaces every old
`.title.de`/`.title.en` read site (`grep -rn "LocalizedText::new|\.title\.(de|en)"` found every
site — 9 in vdi3805, ~20 construction sites in iso16757 that needed no change since the shape is
identical).

## Files touched (Part 1, LocalizedText only)

- `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs` — new `🔖️LocalizedText` region: canonical type + doc comment explaining the composed-child-vs-value-type decision.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🦀️component.rs` — struct deleted, `pub use crate::document::LocalizedText;`.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📔️vdi3805/🦀️component.rs` — struct deleted, `pub use` + `bilingual`/`text_in` helpers; `CatalogueProduct.title` field type change; construction + `.title.de/.en` read-site fix.
- `…/🧬️schema/🦀️component.rs` (vdi3805, native-record import codec) — one `LocalizedText::new` construction site → `bilingual`.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` (vdi3805) — `catalog_index_entry_for`'s tags construction; test fixture construction + assertion.
- `…/🧬️mutations/🏖️rename-product/{🦠️mutation,🔺️diff}/🦀️component.rs` (vdi3805) — `RenameProduct.new_title: Vec<LocalizedText>`; `label()`'s `.en` read → `text_in`; diff's tags construction. (`↩️inverse` needed no change — `product.title.clone()` still type-checks once both sides are `Vec<LocalizedText>`.)
- `…/🧬️mutations/📝️text/🦀️component.rs` (vdi3805) — two op-codec-law test fixture construction sites.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (vdi3805) — regenerated (see below), `title:REC` → `title:TABLE` grammar tag, value became `[ {locale=de text="…"} {locale=en text="…"} ]`.

No `📦️glue.rs`/`📦️index.ts` edit needed — no field-count/derive-list change to `Vdi3805Snapshot`/
`Iso16757Snapshot` themselves (only to the nested `LocalizedText`/`CatalogueProduct.title` types),
so no DSL/pack envelope registration or codec-dispatch table changed shape.

## Fixture regeneration (recipe §7, done for real — not hand-transcribed)

`vdi3805`'s bundled DSL example fixture encodes `title` positionally (old `REC` grammar tag); the
new `Vec<LocalizedText>` shape needs `TABLE`. Added a temporary
`#[cfg(test)] mod debug_fixture_regen` to `📸️snapshot/📝️text/🦀️component.rs` that called
`super::print_dsl(&crate::artifacts::vdi3805::reference_fixture())` and dumped it via
`cargo test … debug_fixture_regen -- --nocapture`; captured the real output, wrote it verbatim as
the new `🗣️example.dsl.semio`, then removed the temporary module. Verified clean:
`grep -rn debug_fixture_regen ✏️s/🔌️plugins/📕️norm` returns nothing.

## Verification (Part 1)

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-norm --all-targets
```
**0 errors** both before and after (one real compile error surfaced mid-edit —
`vdi3805/…/🧬️schema/🦀️component.rs:283`, a third `LocalizedText::new` construction site the
first grep pass had already found but I hadn't yet fixed — resolved with the same `bilingual(...)`
call, confirmed with a second `cargo check` run). Warning counts stable (238/279, +1/+1 — new doc
comments, no new dead code).

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-norm --no-fail-fast
```
**1107 tests run: 1104 passed, 3 failed.** Reproduced identical (same 3 test names, same pass
count) across two consecutive runs — not flaky. All three failures independently traced via
`git log -1 --date=iso --format="%H %ad %s"` (never the fake `🎆️🌙️☀️` message date, per
`📌️important.md`) and confirmed to touch files I never edited:

1. `din4108::…::mutations::…::{reorder_layers_round_trips, insert_remove_layer_round_trips}` —
   `git log` on `🧬️mutations/🦀️component.rs` → `11334431b9…`, **2026-08-12 16:23:09**; on the
   `🧷reorder-layers` triad dir → `a445617cae…`, **2026-08-12 15:50:51**. Both land ~1h/~48min
   after this ticket's 15:02:49 open, but **before this session started** and entirely outside
   `din4108`'s `LocalizedText`-unrelated to anything I touched (I made zero edits under
   `📕️din4108/**`). Failure shape (`left`/`right` diffs show a reordered/duplicated `layers` list
   after an inverse) matches `📌️important.md`'s own D2 "6 failing law tests" class exactly —
   a reorder-mutation-inverse round-trip bug from SMO's wave-2 mass mutations fan-out, already a
   known, ticket-tracked, cross-plugin issue, not something this pass introduced or should fix.
2. `iso16757::…::mutations::…::selection_class_and_constraints_round_trip` — same commit
   (`a445617cae…`, **2026-08-12 15:50:51**) touches
   `🧬️mutations/🦀️component.rs`/`🛁add-selection-constraint`. The failure is a duplicated
   `selection.constraints` entry after an add+inverse cycle — has nothing to do with
   `LocalizedText` (the diff shows every `LocalizedText`-bearing field identical on both sides,
   only `selection.constraints.len()` differs). Same SMO-fan-out class as (1).

None of the 3 failures reference `LocalizedText`, `title`, `bilingual`, or `text_in` anywhere in
their assertion text or diff output — confirmed by reading each failure's full `left`/`right`
struct dump (saved in `scratch-norm-test1.txt`/`scratch-norm-test2.txt`).

## Part 2 — `C:document,table,value R:fem` composition: investigated, not implemented, real reason

Per this ticket's own precedent ("verify the design doc's summary against actual code, the same
way `fem`'s agent verified '11-type dup' and found only 2 were real"), I read the actual snapshot
shape of the smallest artifact (`en1990`, 2721 lines), the largest (`din16798`, 6786 lines), and a
mid-size one (`en1991`, 4082 lines) before writing any composition code.

**Finding: none of the 15 artifacts have an opaque/duplicated content blob of the kind the other
exemplars composed.** `writer` had `text: String` (an unstructured buffer), `lowpoly` had
`mesh_json: String` (an opaque blob), `cad` had per-pane shape/drawing state. Every norm artifact's
`*Snapshot` struct — confirmed for `En1990Snapshot`, `En1991Snapshot`, `Din16798Snapshot` and
spot-checked against several others — is a flat, richly-typed compliance-calculation record: `f64`
quantities with `#[dsl(unit = "…")]`, small closed enums (`AnnexChoice`, `ImposedCategory`,
`FireCurve`), and a handful of small `#[dsl(table)]` `Vec<T>` collections. There is no field that
is "duplicated content" or "an opaque blob" to swap for a composed child.

### The one superficial candidate, and why composing it would be a regression

`En1990Snapshot.q_k: Vec<En1990QkEntry>` (`category: String, value: f64`) looks, at a glance, like
a `table` composition candidate (2 scalar columns). But `grep -rln "q_k\|En1990QkEntry" …/📘️en1990`
shows it already has **five dedicated mutation triads** —
`🐴insert-variable-action`, `🐗reorder-variable-actions`, `🐎remove-variable-action`,
`🦌change-variable-action-value`, `🐮change-variable-action-category` — each a real
`🦠️mutation`/`🔺️diff`/`↩️inverse` triad that mutates one entry (or reorders the list) with a sparse,
index-addressed diff, exactly the granular CQRS-with-event-sourcing shape
`/Users/ueli/Documents/semio/CLAUDE.md` mandates ("MUST use CQRS with event-sourcing", "MUST NOT
use CRUDs"). Composing `q_k` into an opaque `store::ArtifactChild<SemioTableSnapshot>` would force
every one of those five triads onto a whole-handle-replace-plus-working-scene-cache model — the
exact shape this programme's own `📌️important.md` (D2/Concern B) already calls out as an
anti-pattern ("apply-then-capture… whole-object replace of the collection, the very shape this
programme exists to eliminate"). This is not "harder to implement," it is the wrong direction: it
would delete real, already-correct granular mutation semantics to gain nothing (the content isn't
duplicated anywhere, and it isn't opaque). The same shape (small `Vec<T>` field with its own
dedicated per-entry mutation triads) recurs across the other artifacts I inspected — this isn't an
`en1990`-specific finding.

### The shared-codec-macro wall (a second, independent reason full composition is a bigger call than one artifact)

All 15 artifacts share **one** crate-wide codec: `crate::impl_norm_artifact_record!`
(`📄️artifact/🦀️component.rs`, `🔖️ArtifactCodec` region), which implements `ArtifactDsl`/
`ArtifactPack` by delegating to each snapshot's own `#[derive(dsl::DslRecord)]`-generated
`dsl_spec()`/`dsl_to_record()`. Per the migration recipe §2, a composed child field forces dropping
that derive (`ArtifactChild<S>` has no `DslField` impl) — which means dropping the *shared macro*
for that one artifact and hand-rolling its own `ArtifactDsl`/`ArtifactPack`, diverging it from all
14 siblings' otherwise-identical codec path. That's a legitimate, bounded thing to do **if** there
were a genuine content blob to justify it (matching what `writer`/`lowpoly`/`cad` did) — but per
the finding above, there isn't one in the artifacts I inspected.

### `R:fem` — also not implemented, same missing-mechanism reason `fem`'s own report already gave

`grep -rln "fem" ✏️s/🔌️plugins/📕️norm` finds real content in exactly one place:
`en1992::En1992Snapshot.use_fem: bool` (`🧬️mutations/🐫change-use-fem`) — a plain toggle deciding
whether FEM-based verification applies, not a reference to any `fem` artifact instance. There is no
`ArtifactLink<T>` anywhere in the plugin. This is the honest candidate for a real
`ArtifactLink<Fem2dSnapshot>`/`ArtifactLink<Fem3dSnapshot>` once a resolver exists — but `fem`'s
own wave-4 report (`📓️wave4-reports/fem-report.md`) already established, checked directly against
`🔌️plugin/🦀️component.rs`, that **no `LinkResolver`/child-dispatch seam exists in
`ArtifactApp::handle` yet** — the identical blocker, not a norm-specific gap. Re-confirming it here
would mean re-reading the same W1-owned, read-only file for the same negative result; I did not
duplicate that read since the finding is already ticket-recorded and dated after this ticket's
07:xx wave-3 exemplars, before this session started.

## sharedFileRequests

None filed as edits — every change stayed inside `✏️s/🔌️plugins/📕️norm/**`, no `🗄️stdio/**` or
framework file was written (only read for reference: `✳️text`'s `SemioTextRun` shape, `✳️table`/
`✳️value` snapshot shapes). One standing request, not new — reaffirming `fem`'s own #2:

1. **A `LinkResolver`/child-dispatch seam in `ArtifactApp::handle`**
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, W1-owned) is the blocker for
   both `norm`'s `R:fem` (`en1992.use_fem` → a real `ArtifactLink<FemXSnapshot>`) and any future
   composed-child work here. Not a new finding — `fem`'s own report already flagged this as the
   root blocker for its R:brep/drawing case; norm's R:fem case is the identical shape.

## Concurrent-churn observations

`git status --porcelain -- ✏️s/🔌️plugins/📕️norm` and `git diff --stat -- ✏️s/🔌️plugins/📕️norm` were
both empty at session start (re-verified per `📌️important.md`'s "re-verify since time has passed"
instruction — no live uncommitted edits from another session found). No cargo lock contention
observed during either `cargo check` or `cargo nextest` run (both completed on first attempt, no
retries needed). The 3 pre-existing test failures traced above are the only evidence of other
sessions' work landing in this plugin's subtree, and both trace to commits (2026-08-12
15:50:51/16:23:09) that predate this session's start — consistent with SMO's wave-2 mass mutations
fan-out mentioned in `📌️important.md`, not concurrent churn during this session itself.

## Honest accounting

- **Done, verified, tested**: `LocalizedText` duplication killed (both copies), one canonical type,
  `vdi3805`'s bilingual-pair→locale-list shape genuinely generalized (not just renamed), fixture
  regenerated for real (not hand-transcribed), 0 compile errors, 1104/1107 tests passing
  (reproduced twice, non-flaky), remaining 3 failures independently traced to pre-existing,
  unrelated commits.
- **Investigated in depth, not implemented, architecturally justified**: `C:document,table,value`
  composition across the 15 artifacts. Unlike `fem`'s case (a real blocker: shared engine-module
  files outside the plugin boundary), norm's blocker is a genuine content-model mismatch — the
  plugin has no duplicated/opaque blob field anywhere I found, and its one superficial candidate
  (`en1990.q_k`) already has correct, granular, CQRS-shaped mutation triads that composing into an
  opaque child would regress, not improve. Spot-checked across the smallest, largest, and one
  mid-size artifact; the flat-scalar-record shape is consistent, not an `en1990`-only quirk.
- **Deferred, same missing mechanism `fem` already flagged**: `R:fem` — `en1992.use_fem: bool` is
  the honest stand-in for a future `ArtifactLink<FemXSnapshot>`, blocked on the same
  `LinkResolver` gap fem's report already filed.

ucas-status: partial — LocalizedText×2 fully killed and verified; full 15-artifact
`document`/`table`/`value` composition is not a fit for this plugin's actual (already well-typed,
already-mutation-addressable) content model rather than an effort-scoped gap, per the investigation
above.

## Round 2 (orchestrator-dispatched correction) — subset composition

**ucas-status: partial — real `C:table` composition landed on 2 artifacts (`en1990.q_k`,
`din18599.climate`), 0 compile errors, 1105/1108 tests passing (reproduced twice, non-flaky, same 3
pre-existing failures as Round 1's baseline), all 5+1 granular mutation triads preserved with
unchanged public payload/wire shape. 13 of 15 artifacts still untouched — norm overall remains
`partial`, now with working precedent instead of zero composition.**

### Why Round 1's "architecturally blocked" framing was wrong

Round 1 declined to compose anything, reasoning that `en1990.q_k`'s five granular mutation triads
(`insert`/`remove`/`reorder`/`change-category`/`change-value`) would have to collapse into a
whole-handle-replace to become a composed child, citing `📌️important.md`'s D2/Concern B. That
citation was a misread: D2 is about how stdio's OWN `text`/`table`/`graph` subsets implement their
INTERNAL collection diff (sparse triple vs. whole-list-clone-and-wrap) — it says nothing about
whether a PLUGIN composing one of those subsets as a child must give up granular mutations at the
plugin's own dispatch layer. `mathematical`'s Round-1 report (`📓️wave4-reports/mathematical-report.md`)
is direct, already-landed counter-evidence: 14 granular mutation triads over a graph/geometry
structure, composing `text`/`table`/`value` children without collapsing to whole-blob-replace, by
routing every triad's diff/inverse through a `thread_local!` working-scene cache and re-minting a
fresh content-addressed child handle on each mutation. This round applied the identical pattern to
`en1990` and `din18599` and it worked exactly as `mathematical`'s report predicted — every triad's
public payload struct, `MutationKind` impl, and semantic descriptor are byte-for-byte unchanged;
only the internal `diff`/`inverse` function bodies were rewired to go through the cache.

### `en1990.q_k` → composed `s.stdio.semio.table` child

`En1990Snapshot.q_k: Vec<En1990QkEntry>` (`category: String, value: f64`, two scalar columns) is
replaced by `q_k: store::ArtifactChild<SemioTableSnapshot>` under `#[child(kind =
"s.stdio.semio.table")]`. All five existing mutation triads
(`🐴insert-variable-action`/`🐎remove-variable-action`/`🐗reorder-variable-actions`/
`🐮change-variable-action-category`/`🦌change-variable-action-value`) kept their exact payload
structs and semantic descriptors; only their `🔺️diff` bodies changed from
`base.q_k.clone()` + `En1990QkList{values}` wrapping to `en1990_qk(base)` (working-scene read) +
`en1990_qk_child_from_entries(&q_k)` (re-mint), and their `↩️inverse` bodies from `base.q_k.get(i)`
to `en1990_qk(base).get(i)`.

**Composition machinery** (`🗿️artifacts/📘️en1990/🦀️component.rs`, new `🔖️Composition` region):
- `En1990QkChild` type alias.
- `en1990_qk_table_from_entries`/`en1990_qk_entries_from_table` — real, lossless, positionally
  aligned converters (`category`→`SemioValue::Str`, `value`→`SemioValue::Float`), the inverse
  degrading honestly (empty category, `0.0` value) on a short/missing cell rather than panicking.
- `EN1990_QK_SCRATCH: thread_local! RefCell<HashMap<String, Vec<En1990QkEntry>>>` — content-hashed
  scene id (`en1990-qk-<hash>`), same `EngineRep`-contract shape as `mathematical`'s `MATH_SCRATCH`.
- `en1990_qk_child_from_entries` (mint+cache) / `en1990_qk` (read accessor, fails soft to `Vec::new()`
  on a cache miss — documented staleness gap, same as every prior exemplar).

**Snapshot codec**: `En1990Snapshot` dropped `#[derive(dsl::DslRecord)]` (an `ArtifactChild<S>`
field has no `DslField` impl) and gained a hand-rolled `store::ArtifactDsl`/`ArtifactPack` in
`📸️snapshot/🦀️component.rs` — real hex/bracket text codec (`gK=100\nqK=[hex,hex]\n...`) and
fixed-width/LEB128 binary codec, mirroring `mathematical`'s/`cad`'s `🔖️HandcraftedArtifactCodecs`
convention exactly. `En1990QkEntry` dropped its now-unused `dsl::DslRecord` derive (nothing nests it
inside a `DslRecord`-derived struct anymore). The 14-other-family shared
`crate::impl_norm_artifact_record!` macro is untouched — only `en1990` opted out, exactly as its own
doc comment (`📓️design-full-plan.md`'s reasoning) anticipated.

**`En1990Diff`**: `q_k: Option<En1990QkList>` → `q_k: Option<En1990QkChild>` (single-`Option`,
always-present-slot shape per `📓️migration-recipe.md` §8). The dead whole-document-replace
`artifact: Option<Box<En1990Artifact>>` field and its `diff_set_snapshot` helper are removed — grepped,
never constructed by any app command, shaped exactly like the banned `SetSnapshot` vocabulary
(mirrors `mathematical`'s identical dead-field removal).

**`En1990Artifact`** (the UI-inclusive full-state struct): `q_k` field mirrors the snapshot's
composed-child type; `to_snapshot`/`from_snapshot` copy the handle verbatim (same as
`mathematical`'s `MathematicalArtifact`).

**App command `set-snapshot`** (`🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs`):
`ReplaceSnapshot.snapshot: En1990Snapshot` (`#[dsl(block)]`) broke the same way `writer`'s did once
its snapshot lost `DslField` — collapsed to a `text: String` payload holding the document's own
`.en1990` DSL text (see "the serde_json precision bug" below for why `text`/`ArtifactDsl`, not
`json`/`serde_json`). Handler parses via `<En1990Snapshot as store::ArtifactDsl>::parse_dsl`; the
decomposition logic (`En1990Mutation::from_snapshot(base, target)`) is unchanged except it now reads
`q_k` through `en1990_qk(...)` on both sides instead of the removed direct field.

### `din18599.climate` → composed `s.stdio.semio.table` child

`Din18599Snapshot.climate: MonthlyClimate` (two parallel twelve-month `[f64;12]` arrays,
`theta_e_c`/`g_h_w_m2`) is replaced by `climate: store::ArtifactChild<SemioTableSnapshot>` — twelve
rows (one per calendar month, index-addressed), two columns (`thetaEC: Float`, `gHWM2: Float`). The
single `🐘update-climate` mutation triad (an `update-<facet>` per `📓️derivation-rules.md`'s
inseparable-≥2-field-facet exception — both arrays are always entered together, never one month at a
time) kept its exact payload shape: **`MonthlyClimate` still travels on the wire as literal data,
unchanged** — only the snapshot's own STORAGE became a composed child. `MonthlyClimate` therefore
*keeps* its `dsl::DslRecord` derive (needed by the mutation payload's own DSL mirror,
`Din18599MutationDsl::UpdateClimate{new_climate: MonthlyClimate}`) — confirmed necessary the hard
way: removing it broke compilation with `MonthlyClimate: DslField` unsatisfied at
`🧬️mutations/📝️text/🦀️component.rs:63`, fixed by restoring the derive. This is the one place this
round's design differs from `en1990`'s (`En1990QkEntry` genuinely lost its derive since nothing else
needed it) — a real, verified-by-compiler distinction, not a guess.

**Composition machinery** (`🗿️artifacts/📙️din18599/🦀️component.rs`, new `🔖️Composition` region,
placed beside the `MonthlyClimate` type it composes): `Din18599ClimateChild` type alias;
`din18599_climate_table_from_data`/`din18599_climate_data_from_table` (real converters, positional
month↔row alignment, `0.0`-degrading inverse); `DIN18599_CLIMATE_SCRATCH` thread-local cache +
`din18599_climate_child_from_data` (mint+cache) + `din18599_climate` (accessor, fails soft to an
all-zero `MonthlyClimate`).

**Call sites fixed** (all in `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`, the
`Din18599Artifact`+`ComplianceHelpers` file — NOT the artifact root, which only holds the type
definition): `Din18599Artifact.climate` field type swap (`to_snapshot`/`from_snapshot` copy the
handle verbatim, unchanged code); `from_building`'s `BalancingInputs{climate: ...}` construction
site now mints via `din18599_climate_child_from_data`; `transmission_losses_kwh`/
`ventilation_losses_kwh`/`cooling_demand_kwh` (three pure per-`MonthlyClimate` helper FUNCTIONS keep
their `&MonthlyClimate` signatures unchanged — only their three call sites read `&din18599_climate(inputs)`
instead of `&inputs.climate`) and one test call site. `🔺️diff`/`🔺️diff/📝️text` (dead `artifact` field
+ `diff_set_snapshot` removed, same as `en1990`), `🧬️mutations` dispatch's `from_snapshot`, and the
`update-climate` triad's diff (mints from `payload.new_climate`)/inverse (reads via
`din18599_climate(base)`) all updated identically to the `en1990` pattern.

### The `serde_json` precision bug — found, diagnosed, fixed, not worked around

Following `writer`'s own precedent literally (`set_snapshot::SetSnapshot{json: String}`,
`serde_json::from_str`), the first pass of both `en1990`'s and `din18599`'s `set-snapshot` app
command used `serde_json`. `din18599`'s own `undo_redo_round_trips_through_the_wrapper` test then
failed with `h_v: 40.8` (restored) vs. `h_v: 40.800000000000004` (expected) — a real, reproducible
1-ULP precision loss, **traced to its actual root cause, not left as an unexplained flake**: isolated
with a temporary debug test (`cargo test debug_json_roundtrip_hv -- --nocapture`, removed after
diagnosis) proving `serde_json::from_str::<f64>("40.800000000000004")` in THIS workspace's
`serde_json` build parses to a *different* f64 (`40.8`, one ULP off) even for a bare literal — while
`format!("{}", 40.800000000000004_f64)` followed by `str::parse::<f64>()` (Rust's own std path,
already exercised correctly by both snapshots' own hand-rolled `ArtifactDsl` codecs) round-trips
exactly. `en1990`'s default fixture values happen to be round decimals that never hit this edge, so
it silently would have carried the same latent risk.

**Fix applied to both**: `ReplaceSnapshot`'s payload field renamed `json`→`text`, now holding the
document's own `print_dsl()` output escaped onto one physical line via `crate::document::
escape_op_text_field`/`unescape_op_text_field` — the exact convention `SetArtifactMutation<D>`'s own
`OpText` impl (same file, `📄️artifact/🦀️component.rs`) already used for this exact
whole-document-in-one-op-line problem. Those two helpers were `fn` (module-private); promoted to
`pub(crate)` (one-line change each, still `📕️norm`-crate-only visibility) to reuse them instead of
hand-duplicating the escape logic in two more files. Added a regression-guard test
(`din18599`'s `set-snapshot` module, `handle_preserves_full_f64_precision_through_the_payload`)
asserting `h_v` survives the payload round trip bit-for-bit — this is the kind of check that would
have caught the bug before it shipped.

### Verification

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-norm --all-targets
```
**0 errors** (baseline reconfirmed clean before starting; both artifacts' first-pass errors — `#[dsl(unit=...)]`
orphaned attributes after dropping `en1990`'s snapshot `DslRecord` derive, direct `.q_k`/`.climate`
field indexing in pre-existing tests, `MonthlyClimate: DslField` after over-eagerly dropping its
derive — were all fixed in this pass, not deferred). Warnings 238→239 lib / 279→280 test (+1/+1,
consistent with `mathematical`'s own "new doc comments, no new dead code" delta).

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-norm --no-fail-fast
```
**1108 tests run: 1105 passed, 3 failed.** Reproduced identical (same 3 test names, same pass count)
across two consecutive full runs — not flaky. All 3 failures are the exact 3 Round 1 already traced
and independently re-confirmed here to be unrelated to this round's edits — none touch `en1990` or
`din18599`:
- `din4108::…::{insert_remove_layer_round_trips, reorder_layers_round_trips}`
- `iso16757::…::selection_class_and_constraints_round_trip`

Fixture regeneration (recipe §7, done for real via a temporary `#[cfg(test)] mod debug_fixture_regen`
dumping real `print_dsl()` output, captured, written verbatim, module removed — verified clean:
`grep -rn debug_fixture_regen ✏️s/🔌️plugins/📕️norm` returns nothing):
- `en1990`'s `📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio` —
  regenerated from a new `reference_snapshot()` builder (`📚️examples/📕️high-consequence-office/🦀️component.rs`)
  that mints the same 3-entry `q_k` content the file always represented (office=60,
  partition-walls=12, snow=18). The fixture's own test
  (`high_consequence_office_example_fixture_parses_and_round_trips`) now seeds the working-scene
  cache via `reference_snapshot()` before parsing — documented in the test's own comment as the
  same content-addressed-cache-hit bridge every composed-child exemplar depends on.
- `din18599`'s `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated from
  `Din18599Snapshot::default()` (the fixture's values always matched `Default` exactly); this
  fixture's own test only asserts structural round-trip (no field-depth assertion), so no
  cache-seeding change was needed there.

### Honest gap — the working-scene staleness bridge, same as every prior exemplar

Documented in both artifacts' `🔖️WorkingScene` doc comments, not hidden: a genuinely reloaded
persisted `.en1990`/`.din18599` document (fresh process, or a store-level undo/redo past this
session's history) sees a composed-child handle whose working-scene cache entry was never populated
— `en1990_qk`/`din18599_climate` fail SOFT (empty table / all-zero climate) rather than panicking.
For `en1990` this means a reloaded compliance document's variable-action combinations would compute
against an empty table until W1 lands a `LinkResolver`; for `din18599`, energy-balance calculations
against an all-zero climate. Every check both artifacts perform already routes through the accessor,
so the gap is *visibly* empty/zeroed, not silently wrong-but-plausible — same tradeoff
`mathematical`/`cad`/`writer` all already accepted, not a new risk this round introduced. Given the
compliance-calculation stakes, a fail-closed content-hash verification (lowpoly's
`StaleMeshWorkspace` pattern) would be a reasonable follow-up hardening, not implemented here to stay
consistent with the "simple documented gap is sufficient" default the migration recipe sets for
non-destructive read paths.

### Remaining 13 artifacts — not attempted, no new investigation

`din4108`, `din16798`, `en1991`–`en1999` (minus `en1990`), `iso16757`, `vdi3805` are untouched by
this round. `iso16757`/`vdi3805` already had Round 1's `LocalizedText` work; the other 11 were never
investigated for composition candidates by either round. Given the 2-artifact budget this round
targeted, no claim is made about their composability either way — a future pass should actually
check each one's shape (not assume, per this ticket's own repeated "verify before implementing"
lesson) rather than extrapolate from these two.

### sharedFileRequests

None. Every change stayed inside `✏️s/🔌️plugins/📕️norm/**` (including `📄️artifact/🦀️component.rs`'s
`fn`→`pub(crate) fn` visibility bump on `escape_op_text_field`/`unescape_op_text_field` — still
`📕️norm`-crate-private, not a public API change). Only read for schema reference: stdio's `✳️table`
subset (`SemioTableSnapshot`/`SemioTableColumn`/`SemioTableRow`/`SemioTableCellKind`) and `✳️value`'s
`SemioValue`.

### Concurrent-churn observations

`git status --porcelain -- ✏️s/🔌️plugins/📕️norm` and `git diff --stat` showed only the files this
session actually edited at every check; the repo's auto-committer (per `📌️important.md`) landed most
of this round's work mid-session (`git log` advanced by one commit, `515271bf60`, during this
session) — expected, not data loss, re-confirmed per `📌️important.md`'s own churn-detection guidance
(`git log --oneline -3`, `stat -f '%Sm'`) before concluding nothing was overwritten. No cargo lock
contention encountered.

### Files touched (Round 2)

- `📄️artifact/🦀️component.rs` — `escape_op_text_field`/`unescape_op_text_field` visibility only.
- `en1990`: `🗿️artifacts/📘️en1990/🦀️component.rs` (Composition region); `🧬️schema/📸️snapshot/🦀️component.rs`
  (struct + codecs + `En1990QkEntry`); `🧬️schema/🦀️component.rs` (`En1990Artifact`); `🧬️schema/🔺️diff/🦀️component.rs`,
  `🔺️diff/📝️text/🦀️component.rs`; `🧬️schema/🧬️mutations/🦀️component.rs` (`from_snapshot` + tests) and its 5
  triads' `🔺️diff`/`↩️inverse` (10 files); `🧬️schema/💡️inferences/🦀️component.rs`,
  `💡️inferences/🧾outline/🦀️component.rs`; `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` (test + regen);
  `📚️examples/📕️high-consequence-office/🦀️component.rs` (`reference_snapshot`) and its regenerated
  `.dsl.semio`; `🎛️apps/📘️en1990/🦀️component.rs` (3 call sites) and
  `🎮️commands/📤️set-snapshot/🦀️component.rs`.
- `din18599`: `🗿️artifacts/📙️din18599/🦀️component.rs` (Composition region); `🧬️schema/🦀️component.rs`
  (`Din18599Artifact` + 5 `ComplianceHelpers` call sites); `🧬️schema/📸️snapshot/🦀️component.rs`
  (struct + codecs); `🧬️schema/🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`;
  `🧬️schema/🧬️mutations/🦀️component.rs` (`from_snapshot`) and `update-climate`'s `🔺️diff`/`↩️inverse`
  (2 files); regenerated `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`; `🎛️apps/📙️din18599/🦀️component.rs`
  (3 call sites) and `🎮️commands/📤️set-snapshot/🦀️component.rs`.

ucas-status: partial — 2 of 15 artifacts (`en1990`, `din18599`) now have real, verified,
granular-mutation-preserving `table` composition landed; Round 1's `LocalizedText` work stands
unmodified; 13 artifacts remain un-investigated for composition. Round 1's blanket
"architecturally inappropriate" conclusion for the whole plugin is retracted by this round's
evidence — it was specific to the D2 misreading, not to norm's content model in general.
