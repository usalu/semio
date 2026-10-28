# Shared IFC Binary Primitive Implementation

The shared scalar graph now retains EXPRESS BINARY values as `Part21Value::Binary(String)`, `IfcValue::Binary(String)` and `StepValue::Binary(String)`. TypeScript IFC values use `{kind:"binary",value:string}`; the STEP graph uses its existing externally tagged shape `{binary:string}`. The canonical string contains the STEP literal content without double quotes: unused high-bit count 0–3 followed by uppercase hexadecimal digits. The zero-length value is `0`; when a first nibble exists it is less than `2^(4-unused)`. Parsers normalize lowercase input, while native values and SQLite reconstruction reject noncanonical lowercase or invalid left padding.

## Public Surface and Persistence

`semio_s_artifact_stdio_contract::part21::is_part21_binary(&str) -> bool` is the only added function export. The shared Part21 parser/writer, ordinary and controlled value projections, canonical JSON tree and typed retirement derives include the new scalar. IFC4 conversion preserves it in all header/entity/complex/aggregate/typed-value locations. The shared Rust IFC text diff codec uses `B[hex-encoded-content]`; its binary codec uses tag 9 followed by the existing length-prefixed string. Mutation codecs consume those same value codecs. Representative diff and mutation law samples now contain a binary scalar.

The shared IFC4 SQLite graph retains a distinct `binary` kind and stores content in the existing `string_value` column. Rust projection, reconstruction, native flat forest, semantic admission and backing census support that scalar. TypeScript projection/reconstruction do the same. The SQL shape permits the string payload for either `string` or `binary` without adding a column; independent complete-schema extent is 3271 bytes, with unchanged row/value counts for the original fixtures.

Adding a shared Part21 enum arm required exhaustive STEP AP214 and IFC2x3 consumers to retain the new scalar as well. Their own scalar codecs and native/SQLite routes now preserve it rather than conflating it with a string. STEP's complete schema extent is 3458; IFC2x3's is 4027. Their TypeScript unions and transport declarations agree. IFC2x3's model structure remains the existing shared Part21 graph. The semio property-set projection explicitly leaves binary values unsupported because its destination property type has no binary member; this projection is separate from lossless IFC exchange/persistence.

IFC4 snapshot JSON Schema and all embedded mutation `IfcValue` definitions now include canonical BINARY pattern validation. Protobuf and GraphQL declarations have the corresponding scalar kind/payload. The new IFC4x3 Rust catalog now accepts `IfcValue::Binary` against the official BINARY primitive using the shared content validator. The coordinator owns TypeScript new-4x3 validation and the exchange worker owns its STEP implementation.

## Verification

- Test-first fixture `binary-cases.json` and language-neutral `binary.feature` were added under shared IFC4 SQLite snapshot tests before implementation. Initial focused TypeScript execution failed on `unknown IFC value kind`, as expected.
- Final focused Bun run: **23 tests passed, 239 assertions, zero failures**, across IFC4 SQLite regression tests, STEP SQLite regression tests, and dedicated STEP/IFC2x3 binary tests. Independent `bun:sqlite` reads inspect the scalar kind/content and corrupt padding before reconstruction rejection. Ajv independently validates valid and invalid canonical binary values against the IFC4 transport schema.
- The IFC4 binary test retains values in headers, aggregates, typed-value item lists and complex members. Canonical examples include `0`, `0F`, `17`, `23`, `31`, `00ABCDEF`; invalid examples exercise missing nibbles, out-of-range unused counts, nonzero left padding, lowercase native values, non-hex characters and embedded literal quotes.
- Added Rust Part21 parser/writer roundtrip and rejection test, controlled native binary fixture, IFC4 native/SQLite test, and representative binary-bearing diff/mutation samples. **These Rust tests have not executed.** `rustfmt --emit stdout --config skip_children=true --edition 2024` parsed 39 relevant Rust files successfully; this checks syntax only.
- Full Rust compilation remains **unverified**. The coordinator's prior Cargo attempt was blocked before compiler invocation by `Original compiler ingress requires the repository launcher and authentic enforcing store`. No compiler/store/launcher/build enforcement was bypassed or changed.
- The existing full IFC2x3 TypeScript SQLite suite could not load `@semio-tech/stdio-registry-ts/contract/part21/testing/fixtures/sqlite-cohort`. The new dedicated IFC2x3 binary test imports the production codec directly and passes. No package/export/build infrastructure was modified to mask that unrelated import issue.
- The `repo` command is absent from this worker's PATH. The coordinator identified cached `repo.exe` as MCP-only and `semio-repo.exe` as an old CLI without a general execution route. Required focused reads/edits/Bun checks and Rust syntax parsing therefore used the available direct tools, as recorded here. No IDE, modifying Git command, worktree, AGENTS edit, external runtime dependency or source-corpus copy was used.

## Files and Cleanup

`binary-files.md` records 75 source/fixture paths containing this worker's edits; shared paths may contain simultaneous changes owned by other workers. The longest listed source path is 153 Unicode code points, below the 256-character requirement. Ticket input scripts are retained. This worker created no generated output files or temporary dependency installation. The parent coordinator must retain sibling reports/input scripts and remove the shared ticket `🗑️generated` folder only after every oracle worker has finished.

## Reproduction

From the repository root, run Bun against these explicit paths (include `./` because emoji-named files do not match Bun's default test filename filters):

- `./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/binary.test.ts`
- `./✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/binary.test.ts`

The input scripts in this ticket document the implementation operations; they are not migrations and should not be blindly rerun over completed shared files.
