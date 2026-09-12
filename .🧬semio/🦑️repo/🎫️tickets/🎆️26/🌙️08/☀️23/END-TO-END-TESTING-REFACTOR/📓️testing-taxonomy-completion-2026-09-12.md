# Canonical Testing Layout

Tests now use `<semantic-owner>/🧪️tests/<case>/<implementation>`. Executable implementations are direct case leaves. Shared input data and synthetic programs live under owner `🧫️fixtures`; examples use `📚️examples`; independent reference implementations use plural `🔮️oracles`. Obsolete testkit/support collections, nested implementation packages, and legacy test filenames were removed or folded into their semantic cases. Active callers, manifests, fixture readers, Nx metadata, and launch commands were corrected.

The reported renderer violation is now `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx`.

## Acceptance Evidence

- The final actual repository scan inspected 139,975 directories and files and returned **zero layout findings**. See [the census](📓️testing-taxonomy-census-2026-09-12.md).
- The schema-first layout guard passed **74/74 tests, 216 assertions**, with independent schema, path, compiler, dependency, and filesystem checks. Schema catalog generation, documentation, and freshness checking passed. See [the guard report](📓️testing-taxonomy-guard-2026-09-12.md).
- Independent Terra audits accepted the [framework/core layout](📓️framework-testing-taxonomy-acceptance-audit-2026-09-12.md) and [plugin/hub layout](📓️other-testing-taxonomy-acceptance-audit-2026-09-12.md). The final reader and runner corrections are recorded separately below.
- The main Astra coordination used multiple Sol execution workers and multiple Terra read-only auditors, up to the four available concurrent slots. Concurrent source work was preserved. No modifying Git command or worktree was used.

## Runtime Verification

These are selected checks actually run against the affected implementation paths, not a claim that the entire evolving repository passes:

| Area | Result |
| --- | --- |
| Core async and selected kernel tests | 24/24 and 6/6 passed |
| Actor fixture consumer | 10/10 passed |
| Renderer fixture consumers and raster witness | 35/35 and 11/11 passed |
| Renderer step ceiling | 3/3 passed before and after extraction |
| Document oracle consumers | 11/11 passed before and after relocation |
| UI conformance | 62/62 independent Ajv vectors and six Rust tests passed |
| JCO runtime and workspace contract | Four runtime scenarios and two contract tests passed |
| Native scale profile | 12/12 passed after final fixture placement |
| Relocated plugin/directory Rust laws | 11/11 focused tests passed; plugin test target compiled |
| Recursive composition reference oracle | Eight neutral vectors passed through Ajv/Graphlib |
| Procedural mount and document readers | Python/TypeScript mount checks and CAD, Stdio Object, Stdio Kit document checks passed; Python and Node agreed on all 15 moved JSON files |
| Energy and Sequence reference checks | All 11 Energy checks passed; independent Sequence Ajv oracle passed |
| Native reference oracle classification | Two independent compiler/parser comparators passed over 38 vectors |
| Puzzle3D extraction | All ten test names and both complete test-body hashes preserved; four Rust files parsed successfully |
| Final Cargo-relative readers | All ten corrected paths verified; Energy test target compiled and the direct Rust structural reader passed 1/1 |
| Dev launch references | Four scale launch commands resolve to declared targets; source roots and host fixture reader resolve |

Exact commands, outputs, before/after hashes, and scoped file changes remain in the linked reports and the other date-matched evidence files in this ticket. In particular, [final runner repairs](📓️testing-taxonomy-runner-repairs-2026-09-12.md) cover project/launch metadata, and [final reader repairs](📓️testing-taxonomy-final-reader-repairs-2026-09-12.md) cover the ten Cargo-relative paths found by the last audit.

## Verification Limits

Existing or concurrent product failures were retained with their assertions intact:

- Animate's index test fails before and after relocation because its slide parser and emoji path grammar disagree.
- Procedural preview cancellation has the same before/after emitted-invocation failure, `1 !== 2`; its other five selected contracts pass.
- Two of three Rust recursive-composition laws fail on lifecycle state (`ValidatingClosure` / expected `Ready`, observed `Fault`). The moved input resolves and the independent oracle passes.
- The scale WASM build reaches the current WIT schema but fails on the concurrently changed guest ABI. Native profile tests pass.
- The Energy EPJSON check reaches the corrected fixture root but its expected committed `⚡️model.epJSON` leaf is absent; no unrelated fixture regeneration was performed.
- The full Sequence script stops at strict Ajv's unknown `x-semio-child-kind` keyword. Its independent protocol oracle passes.
- Broader Rust/package checks encountered concurrent missing generated modules or production compile errors; the renderer interpreter corpus encountered `themeColorVar is not a function`. These are not reported as passing.

No compatibility alias, blanket script exemption, or blanket Rust `include!` exception was added. Canonical test-gated Rust wiring preserves lexical ownership. Explicit host test and dev benchmark commands may consume synthetic fixture artifacts as test orchestration; production runtime imports of fixture source were removed.

## Change and Retention Record

The [complete authored file ledger](📓️testing-taxonomy-complete-files-2026-09-12.md) combines executor moves/patches and coordinator inputs, including removed and intermediate paths. It does not attribute the concurrent Git diff to this task. The [closure audit](📓️testing-taxonomy-closure-audit-2026-09-12.md) independently checked the executor ledger and output-cleanup plan.

The actual repository MCP close request, [lifecycle proof](📓️testing-taxonomy-lifecycle-proof-2026-09-12.md), and [retention proof](📓️testing-taxonomy-retention-2026-09-12.md) are retained in this ticket. Authored scripts, configuration, input data, and Markdown reports are preserved; generated output and enumerated historical tool logs are removed after verification.
