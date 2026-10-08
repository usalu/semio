# Runtime Guard Independent Draft Review — 2026-10-08

Read-only review of `repo/library/discovery/🕸️runtime/🟦️.ts`, first draft. No duplicate tests executed.

## Actionable Coverage Gaps

- Ecma resource scan line118 recognizes only fixed-token `new URL(string,import.meta.url)` shape. `new URL(pathVariable, import.meta.url)` recognizes null only if its token layout still matches; `new URL(join("🧫️fixtures",name),import.meta.url)` and string concatenation move the second argument and silently skip the URL expression. Must inspect balanced argument segments and report computed/unresolved whenever base is import.meta.url. Add neutral cases for nested call/concatenation/template literal first argument and aliased URL constructor if supported.
- Resource calls lines119–121 recognize identifier spellings readFileSync/readFile/file rather than resolved imported binding identity. `import {readFileSync as load} from "node:fs"; load("🧫️fixtures/data.json")` silently bypasses resource discovery. Namespace fs.readFileSync is recognized incidentally by name, but alias binding/indirection remains unresolved. Bind imports using compiler/owned token declarations or explicitly reject unresolved resource callable aliases in reachable runtime. Actual bundler source inputs cannot reveal external filesystem reads, so compiler roster reconciliation alone cannot repair this gap.
- Rust macro_rules bodies are unconditionally skipped at92 and arbitrary macro invocations are not expanded or reported unresolved. A runtime macro may expand to include_str/include_bytes/module imports; source-only zero is therefore insufficient. Actual rustc dep-info/expanded generated roster must cover this, or macro invocations that can introduce inputs must remain unresolved. Document this bound before treating source graph PASS as compiled proof.
- Context exposes one global features/cfg/manifestDirectory set, while actual Cargo graph consists of per-package resolved features/cfg and manifests. Graph runner must invoke/traverse each package with its own context, especially dependency unification and concat!(env!(CARGO_MANIFEST_DIR)). Otherwise feature-enabled dependency module/resource selection can be wrong.

## Correct Distinctions in Draft

Actual token scope visiting handles explicit test function/item attributes instead of wholesale directory filtering, cfg(any(test,feature)) evaluation distinguishes runtime-enabled features, generated origins are graph edges, unresolved computed resource/imports produce findings when recognized, and compiler roster mismatch is separately reported. Direct tests and dev dependency inputs should remain outside production closure. Package alias resolution is exact caller-provided mapping; build runner must populate actual exports/conditions, not guess.

Line numbers are first-draft observations and may change concurrently. Findings were sent directly to runtime owner for correction; no source edits here.

## Resource Base Semantics

First-draft traversal treats all resource literals as relative to source dirname. That is correct for new URL(relative,import.meta.url) and Rust include resources, but JavaScript fs.readFileSync/readFile/Bun.file relative strings resolve against process cwd. Example reachable `src/main.ts` reading `🧫️fixtures/data.json` from workspace cwd must resolve workspace fixture, not `src/🧫️fixtures/data.json`; inverse paths can also silently miss fixture ancestry. Runtime resource references need an explicit base identity (module URL / declared cwd / absolute path), and unknown cwd must yield unresolved rather than guessed module-relative resolution. Actual executable session cwd must bind the runner receipt.

## Updated Runner Review

URL balanced arguments, fs reader alias spellings and declared cwd are now addressed in the updated source with neutral cases. Actual runner independently retains compiler-artifact features and marks missing feature witnesses unresolved; good qualification.

Remaining acceptance limits:

- Runner supplies one target rustc cfg to every reached package, including proc-macro/custom-build/build dependency units. Those units execute/compile for the host, not the selected guest triple. The actual unit identity needs host/target distinction; a host-only resource cfg could be incorrectly excluded under guest wasm cfg. Use actual rustc compiler-unit cfg/target identity or conservatively mark host-unit cfg unresolved.
- Config currently lists only Store worker TypeScript entry. This is a bounded worker closure, not the whole OS browser/main/plugin JS publication closure. Root list must bind shipped entry/mount declarations and actual outputs before universal runtime graph zero.
- Actual runner does not yet pass compiledInputs to inspectRuntimeGraphV1; compiler-artifact JSON features prove feature selection, not source/resource roster. Neutral rustc dep-info tests establish parser agreement only for test examples. Actual Cargo dependency module/resource input reconciliation (and actual bundler metafiles) remains required for compiled input proof.
- Package export alias selection picks import then default then browser, independent of execution realm/conditions. Actual browser/node entry must resolve correct condition order; wildcard/subpath aliases and tsconfig/Vite virtual aliases currently need explicit mapping or unresolved findings.

These are bounded acceptance concerns, not assertions current selected source has an actual fixture edge. Parent and runtime owner notified.

## First Actual Receipt Qualification

Independently parsed first actual report `oct8-runtime-graph-artifacts/runtime-fixture-graph.json`:195 graphs,1625 source identities,17367 findings =17345 unresolved +22 fixture candidates, Nx exit1/49.8s. These are not 22 confirmed shipped runtime violations. First trace Store worker → worker/cold-pair-loading test → fixture corresponds actual source import under `if(import.meta.vitest)` at7184/7189. Updated guard now explicitly transforms that production boundary; first report predates corrected run. Preserve real test-only source imports. Corrected log exists but no completion receipt inspected yet. Missing compiler witnesses and macro-unresolved paths remain separate reasons for refusal, not source success.

## Corrected Actual Receipt

Independently parsed corrected actual run report (overwrites original report path):220 graphs,1528 source identities,1623 findings, all runtime-unresolved-edge; zero fixture-edge records. Nx exit1/33.2s. compilerArtifacts is null, so no actual feature witnesses were supplied. TS graph has2 computed-import unresolved owners (Store worker and Plugin browser-bundle child worker); Rust1395 computed module references plus resource/missing artifact witnesses remain unresolved. This is a correctly refused partial proof, not compiled runtime exclusion PASS, and zero recorded fixture edges must not be generalized while unresolved graph nodes remain.

Updated source distinguishes host/proc-macro/build contexts, parses actual dep-info when available, checks source/output timestamps and source/compiled rosters, and includes esbuild Store worker production bundle inputs. Those fixes are implemented intent; corrected run lacks compiler artifact input, so actual Cargo reconciliation cannot have completed. Neutral43 tests/258expects receipt reported by runtime owner is distinct from this actual refused graph run.
