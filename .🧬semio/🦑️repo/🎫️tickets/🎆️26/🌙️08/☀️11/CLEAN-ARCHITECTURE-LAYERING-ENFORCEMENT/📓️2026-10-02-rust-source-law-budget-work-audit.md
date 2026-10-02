# Rust Source Law Budget Work Audit

Read-only source/log inspection; no tests/compiler/jobs. Paths relative to Repo library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Actual budget and receipts

Package `📦️packages/🟦️typescript/📜️script.ts:144–149` invokes Bun test under 45,000ms for full source-direction and focused scopes/graph selectors. Focused selectors run their original named laws, not alternative implementations. Scope law `🧪️tests/🧱️rust-source-direction/🟦️.ts:322–391` also has 45,000ms timeout and four workers. Thirty owner rows each preserve isolated files, physical replacements/deletion, rustc dep-info+link, original binary runtime, graph/scopes checks and whole physical-policy execution.

Retained ticket goal-root logs: rust-scope-facts-complete-producer.log reports genuine RED 1 test/282 assertions/41.51s; rust-finite-macro-scope-provider.log older16-law run reports607 assertions/29.26s; rust-scope-facts-provider.log reports RED1 test/6.68s. These runs have different corpus/provider states and failure points; they cannot establish an optimization delta. Current debug logs give row completion/native exit/problem count, not compile/runtime/write/graph/provider timings. The 41.51s scope-only run leaves little headroom but does not prove compiler versus filesystem versus policy loader dominance. Nx graph delay occurs outside this Bun law timing.

## Concrete duplicate provider work

Every row's writeLayerOracle :17–27 rereads the main corpus, actual boundary policy, taxonomy and eight helper source files, then writes those unchanged bytes into a distinct root. Capture those bytes once at cohort start, keeping the exact authored root authority snapshot and per-row copies. This removes repeated reads/JSON parsing; it does not avoid policy load or cross-root authority validation, and must retain a snapshot-change refusal if concurrent edits would otherwise produce mixed authorities.

Each scope row parses all Rust compile refs into refs map (:355), builds inspectRustModuleGraph (:356), then later calls inspectRustSourceDirection (:376), whose executor recollects source bytes, compile refs and graph (`🕸️dependencies/🧭️direction/🦀️source/🏃️execution/🟦️.ts:99–107`). That is actual duplicate pure work. If profiling confirms material cost, have the canonical executor expose its captured graph/reference diagnostics to the existing test through an owned result contract; validate those exact facts rather than constructing a second graph. Do not inject a prebuilt graph into execution without proving it matches the executor's fresh physical capture. Current standalone graph assertions are useful independent consumer checks; combining must retain their expected scope/mount outputs and AJV validation.

Executor physical walk :76–84 serially awaits lstat/readdir/read for all inputs. Bounded concurrent read scheduling under an owned captured census could improve provider work, but preserve raw-prefix no-follow admission, exclusion decisions, cancellation, deterministic sorted output and source stability. It cannot omit taxonomy/helper trees because fixtures copy them for actual bootstrap authority. Do not share one cached policy outcome between rows: root/workspace membership and deletion differ.

## Native obligations retained

Full route additionally compiles owner policy, linked data, inline anchors, traversal success/failure, literal/reference corpus, composite macro runtime+test binary, every macro refusal and graph authority cohort (:30,104,158,184,212,256,393). Compiler invocation counts differ by cohort; thirty scope binaries cannot simply be replaced by the earlier macro composite because mounts, manifest/deletion and runtime source selection differ. Retain every actual compiler and runtime call plus binary output checks. No cached native outcome, deadline increase or skipped law is recommended.

Semantics-preserving next step is stage timing inside the existing registered owner route: snapshot/write; rustc child wall time; binary runtime; standalone graph; physical provider; cleanup. Four-worker concurrency is already bounded and is not evidence that larger concurrency helps on the shared compiler fleet. Keep the original45s contract and optimize measured repeated owned work first. Separate artifact cleanup can be measured but must retain deletion witnesses and orderly process completion.
