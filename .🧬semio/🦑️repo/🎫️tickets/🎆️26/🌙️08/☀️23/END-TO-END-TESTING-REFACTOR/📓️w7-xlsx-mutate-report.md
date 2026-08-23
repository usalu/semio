# Wave 7 — XLSX ECMA-376/✳️any mutation oracle

Case: `mutate-xlsx-ecma-376`. References: `calamine` 0.36 (read), `rust_xlsxwriter` 0.96 (write).

## Verification (verbatim)

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-xlsx-ecma-376
2 high-priority breach(es) across 1 rule(s):
      2  testing/contract
  testing/contract  epw-energyplus-any  Mutation catalog epw-energyplus-any (13 kinds) is claimed by no feature
  testing/contract  docx-ecma-376-any  Mutation catalog docx-ecma-376-any (13 kinds) is claimed by no feature
```
Both breaches name `epw-energyplus-any` and `docx-ecma-376-any` — other subsets peers are still
writing. None name xlsx.

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-xlsx-ecma-376
[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0
```
21/21: 10 `mutate-<kind>` + 10 `inverse-<kind>` + 1 `identity-round-trip`. Confirmed against the
per-scenario result artifacts in
`.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-stdio-artifacts-xlsx-fd8cc3-mutate-xlsx-ecma-376-oracle-rust/📤️results.jsonl`
— all 21 rows `passed`.

The Rust SUBJECT phase was not compiled/verified this wave (expected per the fleet brief — a
concurrent os-kernel refactor blocks it repo-wide). The subject half is written and `sut`-gated.
A direct `cargo build --features oracles --lib` in
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` (the oracle crate only) succeeds cleanly.

## The constraint and how scenarios were typed

`calamine` parses a workbook into resolved cell values but exposes NO accessor for its own raw
shared-string table — confirmed by reading the vendored source
(`~/.cargo/registry/.../calamine-0.36.1/src/xlsx/mod.rs`): `Xlsx<RS>::strings: Vec<String>` and
`read_shared_strings` are both private fields/methods, not `pub`. `rust_xlsxwriter` can only
assemble a brand-new package (never open and patch one), and its own shared-string table
(`shared_strings_table.rs`) is populated ONLY as a byproduct of `write_string` on a cell — no API
targets a pool entry independent of a cell write (confirmed by reading
`rust_xlsxwriter-0.96.0/src/worksheet.rs`).

Result: 7 of the 10 declared `XlsxMutation` kinds are fully representable as "read the whole
workbook into a grid via `calamine`, change the grid, rebuild the whole workbook via
`rust_xlsxwriter`" — a genuine second producer — so their `mutate` scenario is `@mode-differential`:
`no-mutation`, `set-snapshot`, `insert-sheet`, `remove-sheet`, `rename-sheet`, `set-cell`,
`remove-cell`. The remaining 3 — `insert-shared-string`, `remove-shared-string`,
`set-shared-string` — address the shared-string pool by an INDEX independent of any cell
reference, exactly the one axis neither crate exposes. Their `mutate` scenario is typed
`@mode-round-trip` instead: the oracle honestly returns the input bytes unchanged (the correct
answer this pairing can give — no cell is affected either way), and the comparison instead
carries `sharedStringCount` as adapter-tracked arithmetic (`shared_string_count_after` in the
case's own `🦀️component.rs`) mirrored against the subject's real `XlsxWorkbook::shared_strings.len()`.
All 10 kinds' `inverse` scenario is `@mode-property` (same convention every other wave-7 case uses).
Full reasoning is also in the Feature file's own description and in
`.../🧪️oracle/🦀️component.rs`'s module doc comment.

This is a more granular treatment than the pre-existing note in
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml` ("the XLSX case is typed as an
independent-reader round trip rather than a differential — see 📓️w8-plan.md"), which reads as a
ballpark/blanket characterization written before this wave started. `📓️w8-plan.md` itself only
carries the same one-line summary, nothing more specific. Given the fleet brief's explicit
instruction to "establish what is actually achievable" and mix modes per kind, I typed 7/10 kinds
differential (they generalize a real second producer) and reserved round-trip/property for the 3
where no representation exists in either reference crate.

## Real fixture — provenance chain

Committed at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🧫️fixtures/📕️reuse-marketplaces.xlsx`
(10,433 bytes), referenced as `shared://📕️reuse-marketplaces.xlsx`. Chain:
`♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex` (real systematic survey of
European building-component reuse marketplaces)
→ `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv` (already committed by
a peer, 50 rows × 12 columns, real platform names/countries)
→ derived ONCE by `.🧬semio/.../xlsx-fixture-derive` (scratch crate, this ticket folder) with
`rust_xlsxwriter` into a genuine two-sheet ECMA-376 workbook: sheet "Marktplätze" is the real
survey table verbatim; sheet "Länderübersicht" is a real per-country tally computed from the same
data's "Land" column (8 distinct countries). Repeated real values (country names, access
categories, platform channels) deduplicated into a genuine shared-string table —
`xl/sharedStrings.xml` reports `count="622" uniqueCount="229"`, confirmed by unzipping the
committed file directly, not assumed.

## Findings

- **`📚️examples/🎬️demo/🖼️assets/📕️example.xlsx` is a 0-byte placeholder**, not a real fixture (`ls -la`
  confirms 0 bytes, last touched Aug 10). It is untouched by this case; the mutation case uses the
  real derived fixture above instead, per the ticket's own instruction.
- **`calamine` 0.36's shared-string table is entirely private API** (`strings` field,
  `read_shared_strings` method) — this is a hard capability gap, not a version/feature-flag issue;
  no combination of public `Reader`/`ReaderRef` methods reaches it.
- **`rust_xlsxwriter` 0.96 has no API to create an SST entry independent of a cell write** — every
  entry is a byproduct of `Worksheet::write_string`. Confirmed by reading `worksheet.rs`'s
  `store_string`/`shared_string_index` call sites; there is no `insert_shared_string`-shaped method
  anywhere in the crate.
- Both gaps together mean a genuinely independent differential replay of `InsertSharedString`/
  `RemoveSharedString`/`SetSharedString` is not achievable with this reference pairing, for any
  fixture design — not a fixture-specific limitation.
- The oracle crate itself compiles cleanly (`cargo build --features oracles --lib`); a **repo-wide,
  unrelated `cargo test --lib` breakage** (`semio_framework_async_macros` unresolved — affects every
  subset's own `#[cfg(test)]` module, confirmed also failing identically for `tsv`/other subsets, not
  xlsx-specific) means the crate's `#[cfg(test)]` unit tests cannot be run via a raw `cargo test` in
  that directory; only the sanctioned `bun ./📜️script.ts` pipeline resolves it (which is what
  produced the 21/21 green run above).
- A concurrent peer's WIP in `dxf/r12`'s oracle module (`E0502` borrow-checker errors, uncommitted,
  edited 23:12 same day) transiently blocked `cargo build` on the whole crate; resolved by the peer
  before this case's own verification run — not this ticket's bug, per the fleet brief's own
  guidance on concurrent WIP.

## Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — filled in (was a stub)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` — new (catalog + oracle registration + `semantic-spreadsheet-v1` profile)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added `KINDS` const + `kinds_match_enum_and_catalog` test
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🧪️tests/mutate-xlsx-ecma-376/component.feature` — new
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🧪️tests/mutate-xlsx-ecma-376/🦀️component.rs` — new (adapter, oracle + `sut`-gated subject)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🧫️fixtures/📕️reuse-marketplaces.xlsx` — new (real fixture, derivation above)
- `.🧬semio/.../END-TO-END-TESTING-REFACTOR/xlsx-fixture-derive/` — scratch derivation crate (this ticket folder, kept per convention)

No shared family module edits were needed (RFC 4180 CSV's own `tabular` module wasn't reused —
XLSX's multi-sheet/typed-cell shape doesn't fit it, hence the new `semantic-spreadsheet-v1` profile,
contributed the same way TSV contributed `semantic-tabular-mutate-v1`). No `noOracleDecision` was
needed — a credible oracle pairing exists and is registered; only 3 of its 10 kinds fall short of
differential coverage, and that is declared per-scenario rather than by omitting the oracle
entirely.
