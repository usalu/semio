# Architect Program XLSX and ZIP Export Implementation

## Scope

Replace the `architect/program@1/any` XLSX and ZIP JSON-shape coercions with an explicit, shared projection of the program document.

## Confirmed model

- `ProgramSnapshot` currently owns 64 direct `Vec<Record>` registers.
- `knowledge` and `benchmarks` are two composed table-backed registers, bringing the register total to 66.
- `meta`, `project`, and `governance` are mutable singleton blocks. They must remain observable for the six document-level mutations.
- `schema` is document data and must not be discarded.
- The real target models are `XlsxSnapshot { opc, workbook }` and `ZipSnapshot { entries, comment }`; neither is structurally related to `ProgramSnapshot`.

The shared projection therefore has 70 named tables: `program`, the three singleton blocks, and the 66 registers. Singleton tables have one row. Register tables have one row per record. Every row is an object whose top-level fields become columns; nested arrays and objects use canonical JSON text in XLSX cells and remain structured JSON in ZIP members.

## File contracts

- XLSX: one worksheet per table, a header row, then one row per object. Empty registers remain present as empty worksheets.
- ZIP: one `<table>.json` member per table containing a JSON array. Empty registers remain present as `[]` members.
- `serialize_bytes()` remains a Semio `ArtifactPack`, matching all existing target-artifact serializers and importers. Independent format readers validate bytes produced by the target format codecs before the Semio envelope is applied.

## Independent validation

- XLSX is decoded through the approved `calamine` 0.36 oracle exposed by `semio-s-plugin-stdio-test-oracle`.
- ZIP is decoded through the approved `zip` 6 oracle exposed by the same test-only owner interface.
- The feature contract is expressed in Gherkin so the behavior is language-neutral; Rust owns the subject and approved reference-reader bindings.

## Pre-existing build blocker

`bun nx run @semio-tech/architect-plugin:test-quick --skip-nx-cache` currently stops in `semio-framework` before compiling architect. The earlier 4,620 stdio mutation-bound failures are no longer the first blocker. The current failure is 12 unrelated serde-bound errors for `CommandPageCursor`, `CommandIngressStatus`, and `FixedCommandPage` in a concurrent framework refactor. Exporter-specific static checks and the independently buildable test surfaces will still be run; the blocked full command will be rerun and recorded without claiming success.

## Hidden-stub gate

The repository gate already contains a third detector for a serializer that combines `serde_json::to_value(snapshot)` with `serde_json::from_value(...)`. This was added after the original investigation report. This implementation will add regression coverage for that detector rather than duplicate it.

## Implemented

- Added a shared, explicit 70-table projection and used it from both serializers.
- XLSX now builds real worksheets, derives stable columns from each register's fields, writes typed scalar cells, and writes nested values as compact JSON text.
- ZIP now builds one JSON member per table and can emit a real ZIP byte stream through stdio's ZIP 2.0 codec.
- Added raw ECMA-376/ZIP byte entry points for independent readers while preserving the existing `ArtifactPack` contract of `serialize_bytes()`.
- Added language-neutral XLSX export coverage using `calamine` 0.36 and registered the approved test-oracle host with its `oracles` feature.
- Extended the exhaustive mutation case so all 266 mutations export their result through ZIP and the approved `zip` 6 reader verifies the exact subject bytes. The subset manifest now declares all 266 mutations.
- Added the Rust test-host `subjectRawInputs` bridge already supported by the TypeScript/Python path.
- Added focused regression coverage for the hidden serde-`Value` coercion detector.
- Left the explicitly documented CSV single-field stub unchanged.
- Aligned the affected feature assets with the concurrent taxonomy rename from `component.*` leaves to the current dot-basename leaves.

## Validation results

- `bun nx run @semio-tech/repo-test:test-quick --skip-nx-cache -- --test-name-pattern 'serde Value coercion'`: PASS, 1 test, 0 failures.
- Focused contract validation for `export-program-xlsx` and `mutate-program-1`: PASS, 2 cases, 0 breaches.
- Oracle registry inspection: PASS, ZIP and XLSX oracles registered, one manifest with exactly 266 mutations, Rust oracle host resolved with `features = ["oracles"]`.
- `rustfmt` on all changed Rust sources: PASS.
- `git diff --check HEAD -- <ticket files>`: PASS.
- `bun nx run @semio-tech/repo-test-domain:test-subject --skip-nx-cache -- quick --case export-program-xlsx --implementation rust`: BLOCKED by the repository runner's 30-second Cargo budget. Two concurrent peer loops were continuously running `cargo check`, and the runner reported shared target-directory lock contention before the generated host executed.
- Earlier full/focused Rust attempts were also blocked outside this change by missing concurrently renamed/generated framework schema and jco-probe taxonomy files. No exporter-specific Rust runtime result is claimed.
