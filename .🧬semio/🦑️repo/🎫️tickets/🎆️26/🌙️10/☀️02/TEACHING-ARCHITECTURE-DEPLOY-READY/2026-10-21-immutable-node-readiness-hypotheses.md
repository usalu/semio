# Immutable Node Readiness Hypotheses

Read-only bounded inventory of actual Node loader, worker direct imports and Cargo help dependency. No startup cause is asserted without the next actual log; no Source execution or Native action.

Readiness order is explicit: parse bounded newline bootstrap → verify digest/closed module rows/import map → install hooks → await import(entry) and all transitive initialization → require runCapturedBuildSourceV1 export → publish exact ready line → invoke worker session. Thus no ready line can result from import/link/top-level initialization failure; post-ready worker/request failure is a separate category.

Discriminating observations:

- Bootstrap boundary/hash/shape: exact Complete/Bounded bootstrap, identity, closed manifest or complete import errors indicate pre-import admission. Ordered Binary writes may be split by OS stream chunks; loader accumulates until final newline and forbids trailing bytes before readiness. No credit should arrive until ready.
- Static resolution: hooks require exact declaring-row imports and captured targets, with builtins allowed. Undeclared captured Source import or Only captured Source modules errors distinguish missing snapshot edge from a worker payload fault. JSON uses format=json with actual admitted bytes; TS is stripped then loaded as ESM.
- CommonJS: loader marks every non-JSON module as ESM. A captured .cjs with module.exports or named CJS facade assumptions would fail linking/initialization. No direct worker dependency is .cjs or uses require/module.exports; a transitive failure must identify its actual declaring module rather than assume the historical workspace CJS defect recurred.
- import.meta: loader explicitly projects semioCapturedPhysicalSources and semioCapturedSourceResolutions only when text references them. Other fields come from Node's file-URL module context, not Bun-specific synthesized fields. Direct worker modules show only data module import.meta.url, a standard Node field; no direct import.meta.main/dir/path dependency was found. Any transitive undefined-field failure should retain exact module/line and Node version.
- Node API/version: loader imports registerHooks and stripTypeScriptTypes from node:module; absence/incompatible transforms produce a distinct startup SyntaxError/TypeError before ready. Preserve actual node executable/version diagnostic rather than infer compatibility from parent Bun.
- Worker top-level: actual imports include Cargo resolution/receipt, validator, generation helpers, inventory data and closed JSON policies. Top-level creates inventory from policy; invalid definition linkage can fail before ready. Cargo help adds catalog/language/options imports but inspected help has no top-level physical I/O or import.meta/CJS dependency. Worker exports runCapturedBuildSourceV1 unconditionally; it does not rely on import.meta.main birth.

Next useful evidence is exact observed ready bytes or EOF plus full bounded stderr before and after owned close, terminal status/signal and captured source digest. Preserve first readiness primary while attaching this witness; do not reinterpret queue/Source drift as actual loaded failure.

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟢️node/🟨️.mjs — 9015/b0653019a93e3e6188f977639edf6a0cf1f33c1d5a2dc281060ec82d32e78dbc
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️read.mjs — 30429/8ae833aea06f1a69c6544afc8b732ceeb8d07c869dbee4eb3215446dcb9bc778
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/📖️help/🟦️.ts — 7881/75ba16648897fbe80733a073edb4c12cc3b172c78e221cfc973385461120de99
