# PDF TypeScript Suites

## Commands
- `repo run @semio-tech/stdio-pdf:check` (MCP `run`): passed, exit 0 (task 21 s; most wall time was fleet/graph infra waiting).
- `repo run @semio-tech/stdio-pdf:test` (MCP `run`): failed for infra reasons after 5h44m ("Exact captured closure worker readiness required" in the resident service), not a test failure.
- Fallback: MCP `test_file` over all 20 PDF TS test files (every `🧪️tests` TS file except the package-command routers): 20/20 passed (8.7 s) before and after.
- MCP `test_file` for `📘️stdio-pdf/🧪️tests/🚪️route/🟦️.ts`: passed.
- Native `bun`/`bunx` is not on PATH in this environment; `repo ./📜️script.ts` needs SEMIO_SCRIPT_PROVIDERS from the owner.

## Before / After
- Type errors: none found; check passes. Failing tests: none.
- Gap found: five passing tests were not registered in the package `test` suites list in `📜️script.ts`, including the editor round-trip test.

## Editor round-trip test
Already present: `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔭️pdfjs-page/🟦️.ts`. It opens real PDF bytes (`🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf`), measures pages/text with third-party `pdfjs-dist`, round-trips the snapshot through SQLite, applies a `Pdf17SetPage` edit, saves and reopens, and asserts equality plus the view model. Equivalent XLSX TS tests are schema-only (no byte round trip), so no additional TS test was needed. PDF byte-level save is native Rust (`✏️editor/💾️native-save`), out of TS scope.

## Files Changed
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🟦️typescript/📜️script.ts`: registered pdfjs-page, number-inputs, retirement, octets output and producer tests in `suites`.
- Ticket: `run-pdf-tests.py` (helper), `🗑️generated/` raw outputs.
