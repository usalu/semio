# sourcing — DocumentDsl + OpText (Wave 2)

## Files touched
- `sourcing/curate/rs/lib.rs` — added `//#region 🔖Dsl` (private `mod curate_dsl` hand-rolled
  lexer/printer + `impl vcs::DocumentDsl for CurateDocument`) and `//#region 🔖OpText`
  (`impl vcs::OpText for SourcingOperation`). Extended existing `//#region 🔖Tests` with DSL/OpText/
  store round-trip tests.
- `sourcing/curate/example/demo-stock.curate` (new, replaces `demo-stock.curate.json`, deleted).
- `sourcing/curate/example/empty-curation.curate` (new, replaces `empty-curation.curate.json`, deleted).
- `sourcing/plugin/rs/lib.rs` — `DEMO_STOCK_JSON`/`EMPTY_CURATION_JSON` → `DEMO_STOCK_TEXT`/
  `EMPTY_CURATION_TEXT` (`include_str!` of the new `.curate` files); `default_document()`/
  `empty_document()` now call `CurateDocument::parse_dsl(...)`; `.example(...)` manifest calls still
  pass JSON (that's `AppDefinition::example`'s wire format, framework-owned, unchanged) but now
  built via `serde_json::to_string(&default_document())` / `&empty_document()` instead of the raw
  fixture — mirrors `note/plugin/rs/lib.rs`'s `semio_example_json()` precedent exactly.
- `sourcing/plugin/rs/Cargo.toml` — added `vcs` dependency (needed to bring `vcs::DocumentDsl` into
  scope for `CurateDocument::parse_dsl`), matching `note/plugin/rs/Cargo.toml`'s existing dependency.

## Design
`CurateDocument` has 4 top-level fields (`stock: Vec<ObjectKind>`, `filters: Filters`,
`curated: Vec<CuratedItem>`, `runtime: CurateRuntime`). `.curate` DSL is 4 sections/lines:

```
stock
  kind id=<id> module=<moduleId> availability=<u32> typology=<a/b/c|-> geometry=<token> "<name>"
filters modules=<a,b|-> typology=<a/b|-> minAvailability=<u32> sort=<columnId:asc|desc|-> "<query>"
curated
  pick <objectId> <count>
runtime selected=<objectId|->
```
`geometry` token (one whitespace-free token per `GeometryRecipe` variant):
`box:w,h,d` | `frame:w,h,d,profile` | `slab:w,d,t` | `mesh:pos,pos,...;norm,norm,...;idx,idx,...`.

`SourcingOperation` has exactly one variant (`SetDocument { document: CurateDocument }` — a
whole-document swap, same shape as `writer`'s `WriterOperation::SetDocument`). Op text is:
`setDocument "<escaped .curate document, print_dsl output with \n escaped>"` — reuses
`print_document`/`parse_document` directly (single source of truth), escaping only turns embedded
newlines into `\n` so `print_op` stays one line; `parse_op` unescapes and reparses.

Hand-rolled lexer (`mod curate_dsl`) mirrors `vcs`'s own private structural-line grammar
(`marker key=value ... "trailing quoted text"`) and `writer`'s Wave-1 `mod writer_dsl` precedent —
duplicated locally since `vcs`'s escaping helpers are private to that crate, per the ticket's
"hand-rolled tokenizer only" constraint.

## sourcing modules (beams/windows/slabs)
Confirmed OUT OF SCOPE: `sourcing::beams`/`windows`/`slabs` implement `SourcingModule` (typology +
demo catalogue kinds only) and plug in via `Contribution::SourcingModule`'s `typology_json`/
`kinds_json` — a separate contribution-data channel, unrelated to `SourcingOperation`/
`CurateDocument`'s VCS persistence. They define no operation variants of their own; `SourcingOperation`
has the single `SetDocument` variant and that's the only thing `OpText` needed to cover. No changes
made to `sourcing/module/{beams,windows,slabs}`.

## Verification
See ticket close summary / final agent report for exact pass counts (cargo test run in progress at
time of writing this note under an isolated `CARGO_TARGET_DIR` due to shared `target/` lock
contention).
