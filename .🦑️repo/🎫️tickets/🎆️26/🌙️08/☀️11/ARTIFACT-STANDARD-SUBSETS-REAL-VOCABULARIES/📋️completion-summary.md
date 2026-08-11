# Completion Summary — Artifact Standard Subsets Real Vocabularies

## What was delivered

**Mechanism (W1)** — clean vocabulary layer replacing the degenerate single-entry `subsetDirs` map:
- `🔣️taxonomy.json`: `subsetDirPrefix`/`subsetSlugPattern`/`subsetAnyId`/`subsetAnyDirName`/`subsetsManifestFilename` (shape-only, mirrors `standardDirPrefix` one level down) + `validateTaxonomy()` checks.
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`: `SubsetId` docs rewritten (family id, never level); `run_subset_validation` now emits `io.subset.validator-missing` instead of silence; `intern_dialect` doc explains why it stays validation-free.
- `📜️script.ts`: `PolicyArtifactDialect` gained logical `subsetId`/raw `subsetDirName`; subset dir prefix/slug validated; `policyListStdioAnyStandardEntries` replaced by structural `policyListStdioSchemaOwningEntries` (schema-owning = has `📸️snapshot/`, not string-matched); new `policyStandardSubsetVocabularyBreaches` (manifest existence, id validity, dirs↔manifest equality both directions, domain-artifacts-stay-any-only, real-subset-has-validator); `policyDialectLiteralPathBreaches`'s `"*"⇔"any"` hardcode replaced with the taxonomy mapping; wired into `verify gate`. Also fixed an unrelated pre-existing bug found while verifying: `policyStripRustCommentsAndStrings` mishandled Rust char literals like `'"'`, causing false-positive "no real trait impl" breaches.
- 85 per-standard `🪆️subsets/🔣️component.json` manifests generated (one per existing artifact-standard; domain artifacts declare `{"*"}` only).

**Pilots (W2)**:
- PDF `✳️a-2b` restructured into `1.7/✳️a` — conformance LEVEL (2b/2u/3b/3u) is now analyzer-detected data (`stdio.pdf.a.level`), never part of the id. Added the previously-missing `🎹️composer/🟦️component.ts` leaf.
- New standard `ifc 🔖️2x3` built from scratch: engine (reuses step's Part-21 parser, own `Ifc2x3Snapshot` type — not a shared-type-identity violation), full schema-owning `✳️any` (real `DiffAlgebra`, `field_sweep`, `sniff`), 3 real subsets `cv20`/`sav`/`cobie`.

**Fan-out (W3, 10 parallel units)** — 32 real, spec-derived conformance subsets total:
pdf 1.4 (`a`,`x`) + 1.7 (`a`,`x`,`e`,`ua`,`vt`,`h`); step ap214 (`cc1`-`cc6`); ifc 2x3 (`cv20`,`sav`,`cobie`) + 4 (`rv`,`dtv`); svg 1.1 (`tiny`,`basic`); docx/xlsx/pptx ecma-376 (`strict`,`transitional` each); tiff 6.0 (`baseline`); jpg jfif-1.01 (`baseline`); xml 1.0 (`valid`); json rfc8259 (`i-json`); zip 2.0 (`iso21320`). Each: full 5-component shape, real hard/soft checks against the retained schema (honest schema-gap diagnostics where the engine can't support a check yet), `SubsetValidator` + registration, inline tests.

## Verification performed

- `cargo check -p semio-s-plugin-stdio`: 0 errors (all new code) — reached this state after fixing one real bug found along the way: `Part21Instance` comparison needed a deref in ifc2x3's diff.
- `cargo test -p semio-s-plugin-stdio --lib`: reached 744/744 passing. 13 real bugs found and fixed in fan-out output:
  - **pdf x/ua/vt composer tests** (3): "conforming" fixtures went through `encode_pack`→`decode_pack`, but 1.7's writer deliberately never re-emits `PdfSnapshot.objects` (documented design) — fixed by hand-crafting raw PDF bytes routed through `AnalyzeSource::Text`, the same technique `✳️a`'s own tests already use.
  - **docx strict composer + transitional builder** (3): `encode_docx` always regenerates `word/document.xml` from the typed model, discarding manually-seeded XML — fixed by encoding the OPC package directly, bypassing the typed-model step; separately, `decode_docx` only recognized the Transitional officeDocument relationship type, rejecting genuinely Strict packages — extended to recognize both (mirrors xlsx's existing dual-type handling).
  - **xlsx strict composer x2**: same "typed-model regenerates on encode" bug — same fix.
  - **jpg baseline composer**: test fed raw `encode_jpg` bytes where `AnalyzeSource::Binary` expects pack-encoded (envelope-wrapped) bytes.
  - **tiff baseline composer x2**: `TiffSnapshot::default()` has a zero-size image the encoder legitimately rejects — fixed with a minimal 1×1 fixture.
  - **ifc2x3 round-trip**: test asserted 6 instances where the fixture actually has 8 (fixture-counting bug in the test itself).
  - (a 14th failure, `txt::field_sweep`, was unrelated/exogenous — fixed itself when the owning concurrent session finished.)
- `bun ./📜️script.ts policy` — direct probe of the new/changed rules: `policyStandardSubsetVocabularyBreaches` 0, `policyStandardsCoverageBreaches` 0, no shrink-only allowlist (DiffAlgebra/field-sweep/sniff-reality/round-trip) gained any entry for a new subset or the new standard.

## Known residual / exogenous state (not this ticket's scope)

At the time of the final check, two OTHER concurrent sessions' in-flight work made full-repo commands (`cargo test` for the whole crate, `verify gate`) non-green:
- `🟪️stl` (StlSnapshot field rename `faces`→`triangles`, confirmed via `git status` as actively uncommitted elsewhere) — blocks whole-crate `cargo check`/`test` at the moment of writing.
- `📕️norm`'s `din4108`/`din16798` (glue.rs references files not yet on disk — pre-existing, mid-scaffold per the open "Norm" goal) — blocks `plugin-registry:check`/`verify gate`.
- Both `📐️step/…/📐️part21` and `🧱️brep` (pre-existing, committed well before this session) already carry the same "not declared by #[path]" plugin-registry note the new `🪜️ladder` module also gets — an established, pre-existing checker limitation for nested engine submodules, not a regression.

Recommend re-running `cargo test -p semio-s-plugin-stdio --lib` and `bun ./📜️script.ts verify gate` once those two sessions land, as a final confirmation — this ticket's own last full clean run (before that churn appeared) was 744/744.
