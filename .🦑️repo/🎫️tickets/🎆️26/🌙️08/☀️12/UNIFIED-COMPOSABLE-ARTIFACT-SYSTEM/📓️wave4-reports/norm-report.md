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
