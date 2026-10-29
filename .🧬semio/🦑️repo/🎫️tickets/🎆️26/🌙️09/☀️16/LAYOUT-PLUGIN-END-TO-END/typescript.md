# TypeScript Verification

Final result: **75 tests passed across 16 files**, 313 Bun assertions, 16.67 seconds. The expanded aggregate contains 36 checks, including every audited omitted portable export. Final strict TypeScript compilation passed with exit 0.

The existing layout-js target ran only two asset-existence tests. The package runner now includes all 15 Bun suites plus a new aggregate suite containing 36 checks. Its initial baseline registered eight existing portable oracle runners and six standalone behavior checks; the final expansion adds all 21 audited portable exports and an oracle-isolation regression. The project manifest declares the literal runtime entries; cancellation, progress, environment and quick-level wall-clock budgets propagate through the repository execution interface. The package target also runs a narrow strict TypeScript compile of the aggregate and its imported production/test closure.

## Concrete Failures Repaired

- Two test readers referenced the Dictionary schema before its move into 🧬️schema; both now read the current schema.
- The page Drawing SQLite suite imported the removed embedded Drawing codec and type. It now tests the current reference-only layout_drawing_member contract with document/page identity ownership and dialect checks. Shared language-neutral fixtures and the independent SQLite C-engine oracle were updated together. The Rust agent added matching coverage of both valid cases and all eight refusals to its relational member test.
- The structural native JSON corpus was passed directly to the canonical bigint snapshot parser. The test now uses the named layoutArtifactFromNativeJson boundary; its independent RFC 6902 and Ajv assertions are retained.

## Successful Validation

Aggregate command:

    repo --service=off tools exec bun -- test './✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts'

Result: **14 passed, 0 failed**, 13.50 seconds. These checks cover Drawing segments, composed Drawing export, page member ownership through real SQLite, flow projection, text flow, affine export, PDF/PNG/package streams, canonical schema/mutations, genuine Drawing producer identity, PDF marked text scopes, parent creation, randomized event inverse/replay algebra, retained page caching and window ownership.

Strict compile command:

    repo --service=off tools exec bun -- ./node_modules/typescript/bin/tsc --noEmit --strict --skipLibCheck --resolveJsonModule --esModuleInterop --target ESNext --module ESNext --moduleResolution bundler --allowImportingTsExtensions --types bun './✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts'

Result: **exit 0**.

Structural regression command:

    repo --service=off tools exec bun -- test './✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🏗️structure/🟦️.ts'

Result: **2 passed, 0 failed**, including 124 assertions.

## Complete Package Validation

The initial complete 16-file package list executed through managed Bun: **53 passed, 0 failed**, 313 assertions, 35.06 seconds. The font agent corrected both stale work-ledger callers. The package runner forwards its repository test budget as Bun --timeout so a valid quick-level case is not killed by an unrelated fixed five-second tool default. The complete process remains capped by the same quick-level budget.

An expanded strict compile additionally covering the changed package runner and structural test exposed an inaccurate existing optional-key cast; it was corrected. Its final rerun passed with exit 0.

The original declared repo run @semio-tech/layout-js:test -- --level=medium target encountered prolonged capacity and peer graph-publication waits. Its old session was unavailable when polled once on resumption, so no terminal result is claimed from that route. It reached graph generation 721 and restarted admission with its initial request timestamp. Managed tool execution honors the same machine capacity gate and supplied the concrete test/strict results above.

## Refused Diagnostic Attempts

- Native direct invocation of the package script path is unsupported (Unknown command).
- The first managed Bun operand lacked ./; the native guard refused it before tests started. Every successful command above uses an explicit path.

No runtime legacy codec or compatibility layer was restored, and no dependency manifest or lock was edited.

## Exact Final Complete-Suite Command

    repo --service=off tools exec bun -- test --timeout 300000 '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🖼️frame-planning/🧪️tests/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/📄️large-sheet/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖼️canvas/🧵️paragraph-work/🧪️tests/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/👁️layer-order/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔣️transport/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🏗️structure/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️frame-selection/🟦️.ts' '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts'

## Exact Expanded Strict Compile Command

    repo --service=off tools exec bun -- ./node_modules/typescript/bin/tsc --noEmit --strict --skipLibCheck --resolveJsonModule --esModuleInterop --target ESNext --module ESNext --moduleResolution bundler --allowImportingTsExtensions --types bun './✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts' './✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/🟦️.ts' './✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🏗️structure/🟦️.ts'

Result: **exit 0**, covering the aggregate behavior entry, package runner, structural regression and their imported production/test closure.

## Files Changed by This TypeScript Work

- ✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📋️project.json
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/behavior.test.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️native-json/🧪️tests/run.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🧾️data/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🏗️structure/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/📄️page-canvas/sqlite.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/📄️page-canvas/sqlite-oracle.mjs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🧪️tests/📄️page-canvas/🔣️.json

The two font oracle repairs and Rust shared-fixture consumer belong to the corresponding agents' reports. This report is also retained in the ticket.

## Expanded Portable Coverage Work

The independent audit identified 20 portable behavior files and 21 exports beyond the original aggregate. They are now registered with their existing independent oracle interfaces. The first runtime pass reached 33 passing checks and two readonly-source failures. Both failures were caused by the test-only fast-json-patch oracle borrowing operation objects: a later patch mutates an object inserted by an earlier operation. The adapter now clones both operands before calling the external library. A new language-neutral patch-isolation fixture checks expected output and preservation of authored operation objects without weakening either original readonly-source assertion.

During integration, an editing script reused its destination variable and briefly wrote source-entry JSON into the transfer oracle module. The exact pre-edit source captured earlier in this session was restored. A focused read-only git diff of that oracle is empty, confirming restoration relative to the workspace baseline; no peer edits were removed. The source-entry manifest was then written through its own distinct destination variable.

The expanded strict compile passed before runtime. The old declared-target session 21929 was polled once on resumption and returned Unknown process id; no new heavy graph test target was started.

## Final Expanded Validation

- Expanded aggregate: **36 passed, 0 failed**, 17.80 seconds.
- Complete unchanged 16-file package operand list with the expanded aggregate: **75 passed, 0 failed**, 313 Bun assertions, 16.67 seconds.
- Final strict TypeScript compile of the aggregate, changed runner, structural regression and imported production/test closure: **exit 0**.
- These runs use the exact managed Bun full-suite and strict commands retained above, without lightweight test execution.
- The added library adapters preserve all existing oracle expectations, ownership/refusal laws and cancellation assertions. The only failure correction needed after expansion was test-oracle operand isolation; production code was unchanged.

### Additional Files

- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/portable-oracles.mjs
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/portable-oracles.d.mts
- ✏️s/🔌️plugins/📏️layout/🧪️tests/🔬️typescript-package-commands/portable-oracles.json

The existing behavior aggregate and source-entry manifest were extended. The temporarily overwritten transfer oracle was restored exactly and has an empty focused git diff.
