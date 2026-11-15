# Course dev/build planning closure

## Facts

- Even `repo run @semio-tech/print:deps-tectonic --dry-run` needed the 405-owner slice (peak 4.7 GB, cancelled at 338/405). The slice is the transitive closure over every dependency edge of each admitted owner (`sliceClosureV1` in `build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs`: "every admitted owner admits the owners it imports, transitively"); the course reaches `@semio-tech/print` through `dependsOn` of the four PDF targets.
- Closure of `@semio-tech/print` in the sealed generation 16: 676 owners. With one edge removed: 32.
- Accidental edge (the only one): `@semio-tech/print` -> `test-framework-products-print-da77cb` (the generated `tests-v2` project rooted at `print/🧪️tests`, 480 edges incl. the Rust subject closure `semio-framework-print` -> `semio-os-kernel`, `semio-plugin`, renderer, plugin registry, `repo`, `repo-lib`, `repo-test-domain`). Source of the edge: seven files `print/📦️packages/🟦️typescript/🧪️testing/**` re-exporting test cases from `print/🧪️tests`. `productionModuleSourceImports` only treats `🧪️tests` and `📜️script.ts` as non-production, so these counted as production imports.
- The only consumer of `@semio-tech/print/testing/*` was `print/🧬️schema/💡️inferences/🔬️probes/🟦️.ts` (owned by no project, so relative imports there create no edges).
- Same bug class elsewhere (not touched, hashing coverage unknown): `fem-js`, `framework`, `framework-kernel`, `framework-plugin`, `norm-js`, `repo-lib` (2), a `breach-` script -> `test-*` projects; 9 pairs in total, all from `🧪️testing` / `🧪️test-values` files.
- Course `build` closure edges: course 36 owners, proctor 173, print 32.

## Changes

- `print/🧬️schema/💡️inferences/🔬️probes/🟦️.ts`: imports the test cases and fixtures from `../../../🧪️tests/...` directly.
- `print/📦️packages/🟦️typescript/🧪️testing/` deleted (7 files) and the seven `./testing/*` exports removed from its `package.json` (no compat layer; no other consumer in the workspace).
- Planner host bug: `build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs` measured host memory with `os.freemem()` (free pages only, ~100 MB on a loaded macOS host with GBs of inactive cache), so any heavy slice load was cancelled with "only 70 MB of free memory". It now uses `availableMemoryBytesV1` (`🧠️memory`, free + inactive pages / `MemAvailable`), the reader the governor already uses. Test added to `build/🏡️resident/🧪️tests/🛑️cancel/🟦️.ts`.

## Measured

| plan | before | after |
| --- | --- | --- |
| `print:deps-tectonic --dry-run` | 405 owners to re-infer, 4.7 GB at 80 s, cancelled (memory floor) | 32 owners, peak worker 1.75 GB, 67 s |
| `course:build` | never planned | 275 owners, plan 10-30 s |
| `course:dev --dry-run`, `course:test-e2e --dry-run` | never planned | 184 owners each, exit 0 |

First run after the edge removal re-inferred the stale old closure once (406 owners, about 10 min, worker peak 7.9 GB) and sealed the new generation; later plans are the figures above.

## Proof

- `repo run @teaching/architecture-course:build --skip-remote-cache`: exit 0 (second run; the first stopped in `print:deps-tex` with "Pinned Print bundle range refused", a transient answer of the TeX bundle host, retried alone `repo run @semio-tech/print:deps-tex` exit 0). `deps-tectonic` provisioned the compiler, all four PDFs exist under `.cache/b/p/m/print/prototype/documents/course/`.
- `:test` exit 0, 848 passed (19 files). `:typecheck` exit 0.

## Not verified / follow-ups

- `@semio-tech/print-viz-inference:test`: 629 of 631 probes pass, 2 errors that look environmental (missing Playwright browser; ajv draft-07 meta-schema), not compared against a pre-change run.
- Whole-repo pins (exports/taxonomy tests) were not run.
- Planner follow-up (not done): `refreshClosure` verifies the whole old-generation closure before shrinking it; expanding progressively from trusted owners would avoid re-inferring owners an edit removed from the closure. Cheapest general fix for the remaining 8 production -> test-project edges: treat `🧪️testing` importers like `🧪️tests` in `productionModuleSourceImports`, after checking hash coverage of each consumer.
