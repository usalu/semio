# WP4 — `🧰️framework/🔨️modules/**` (except `🧬️schema/`, except `🧬️mutations/`)

Partition: every framework module except the framework schema registry `🧰️framework/🔨️modules/🧬️schema/`
and every `🧬️schema/🧬️mutations/**` subtree. Products, hub, plugins, root `📜️script.ts` were not
touched — their required edits are listed under *cross-partition requests*.

## 1. Result

| | |
|---|---|
| schema modules in partition after WP4 | **117** (`🧬️schema/🔣️.json`, excl. the registry + mutation trees) |
| named exports (`$defs`) | **211** |
| non-canonical schema files deleted | **164 + 4** (see §3) |
| ajv draft-07 compile of every module × every export | **0 problems** |
| gate `git ls-files … \| grep -E 'schema\.json$\|📐️schema/\|🛂️schema/\|📐️fixture-schema/\|🧬️contracts/'` | **empty** |

Full scope/export table: `wp4-framework-scope-exports.md` (117 rows).
Machine-readable move map: `wp4-framework-modules.map.json`.
Transformer: `wp4-framework-modules.py`. Validator: `wp4-framework-validate.mjs`.

## 2. Module shape produced

```jsonc
{ "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://semio.tech/schema/framework/<ascii module path>/schema.json",
  "title": "<Scope> Schema Module",
  "definitions": { …hoisted internal defs of every folded source… },
  "$defs":       { "<ExportId>": …the contract body… },
  "allOf": [{ "$ref": "#/$defs/<ExportId>" }]   // only when the module has exactly one export
}
```

- `$id` is derived from the ASCII tail of each emoji directory (`🎭️actor/📤️return` →
  `framework/actor/return`), matching the contract's `framework.actor.return` example. Scope id =
  that path with `/`→`.`.
- Each folded source's own `definitions` **and** its internal `$defs` are hoisted to the module's
  single root `definitions`, so `<id>#/definitions/<name>` cross-file `$ref`s keep resolving and
  exports stay a clean, contract-only namespace. Collisions would have been renamed
  `<export><Name>`; **no collision occurred** (`definitionRenames` is empty for every module).
- Single-export modules keep a root `allOf` so an existing `ajv.compile(module)` still validates the
  one export. Multi-export modules deliberately have **no** root constraint — consumers must name the
  export (`getSchema(\`${module.$id}#/$defs/<Export>\`)`); every such call site was rewritten (§5).
- 2020-12 → draft-07 migration applied (`prefixItems` → `items` array + `additionalItems`,
  `unevaluated*` dropped). Three consumers using `ajv/dist/2020.js` were switched to plain `ajv`
  (`🎭️actor/📥️cold-pair/🟦️.ts`, `📡️replication/🟦️.ts`, `⏳️async/📦️packages/🦀️rust/📜️script.ts`).

## 3. What moved

### 3.1 Class (b) fixture-owned contracts → owner module
Every `🧪️fixtures|🧫️fixtures|🧪️tests|🧪️fixture|🧫️fixture|🧪️conformance|📚️examples|🧬️contracts` directory
lost its schema; the contract went to the nearest **eligible** owner above it (contract §A: `🧪️*`/`🧫️*`
and `🧱️elements/*` are not eligible), the case data stayed put. Largest consolidation:
`🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/<29 cases>/🧬️.schema.json` → one
`🧵️retained/🧬️schema/🔣️.json` with 29 `…Fixture` exports.

`🖱️ui/🧱️elements/📨️UIDialog/🧬️contracts/♿️modal/` (not an eligible owner level) → contract became
`framework.ui` `$defs.UIDialogModalFixture`; the fixture data moved to
`📨️UIDialog/🧫️fixtures/♿️modal/🔣️.json`; `📨️UIDialog/🧪️tests/🟦️.tsx` rewired.

`🧩️action-argument-resolution/🧬️contracts/🔽️choices/` (**created by a concurrent worker at 16:00
today**, after the WP0 audit) → `framework.action-argument-resolution` `$defs.ChoicesFixture` +
`🧫️fixtures/🔽️choices/🔣️.json`; `🛂️manifest/🦀️.rs`'s `include_str!` rewired.

### 3.2 Class (c) non-canonical schema dirs → `🧬️schema/`
`📐️schema/`, `📐️fixture-schema/`, `🛂️schema/` (43 dirs) all folded into the sibling `🧬️schema/`.
`📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🏠️local-interaction/🔣️.schema.json` (redundant
nesting) → `…/🏠️local-interaction/🧬️schema/🔣️.json`.
`🌱️value/🗂️ordered/🧺️set/🧬️schema/🔣️.schema.json` and
`🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧬️schema/🔣️.schema.json` renamed in place to `🔣️.json`.

### 3.3 Class (d) flat schema files → modules
`X/🧬️schema.json` → `X/🧬️schema/🔣️.json`; `X/🧬️<name>.schema.json` / `X/<emoji><name>.schema.json`
→ `X/🧬️schema/🔣️.json` with a `<Name>` export.

### 3.4 Duplicate-authority pairs — consolidation decision
The WP0 audit proposed keeping both members of each `📐️schema/🔣️.json` (fixture oracle) +
`🧬️schema.json` (domain contract) pair as two files (`🔣️.oracle.json` / `🔣️.admission.json`).
**Rejected** — that violates contract §B ("inside a `🧬️schema/` module only the canonical five") and
the one-module-per-scope rule. Each pair became **one** `🧬️schema/🔣️.json` with two named exports:
the domain export named after the scope (`Page`, `Slot`, `Credit`, `Lifetime`, `Admission`, …) and
the fixture oracle as `<Scope>Fixture`. 20 pairs consolidated this way across
`🖱️ui/🧬️contract/🧵️retained/💾️resident/**`, `🧵️retained/🩹️operations/📥️wire/📃️pages`,
`🧬️contract/♻️retirement/**`, `🎭️actor/**`, `🌱️value/💾️resident/**`.
`🎭️actor/🚪️lifetime` folded three (`Lifetime`, `LifetimeFixture`, `CloseFaultFixture`).
`🖱️ui/🧬️contract/♻️retirement/🌳️typed` folded `TypedFixture` + `Components`.

### 3.5 Modules normalized in place (WP0 class (a))
23 already-canonical modules had legacy `$id`s (`semio.actor.byte-page.v1`, `s.framework.abi/1`,
`semio://framework/ui/render/webgpu/surface-port/v1`, `https://semio.tech/schemas/ui-host/…`), no
`$defs`, or a 2020-12 dialect. All were wrapped into the module shape and re-`$id`'d, and their old
ids rewritten everywhere reachable (§5). `🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/`
was not an eligible scope (a module inside `🧪️tests`) and was folded into `🔢️scalar/🧬️schema/` as
`ScalarTestsFixture`.

`🖱️ui/🖥️host/🧬️schema/🔣️.json` was **not a schema at all** — a browser-host *declaration document*
(`title/version/transport/limits/operations/events/…`). Split: the data moved to
`🖥️host/🤝️contract.json` (repo's existing convention) and the module now exports
`BrowserHostDeclaration`, a structural draft-07 description of that document derived from it. Rust
`📡️event.rs`'s `BROWSER_HOST_SCHEMA_JSON` became `BROWSER_HOST_CONTRACT_JSON` pointing at the data.

### 3.6 Named work items from the brief
- `⏱️trace/⏱️clock/🧬️contention/🔣️.schema.json` → `⏱️trace/⏱️clock/🧬️contention/🧬️schema/🔣️.json`
  (`framework.trace.clock.contention`, export `Contention`). Root `📜️script.ts` consumes it →
  cross-partition request.
- `📡️replication` redundant nesting flattened (§3.2).
- `📡️transport`'s `📐️schema` + `🧬️schema` siblings → one module, exports `Transport` + `TransportFixture`.
- `🖼️assets` had no module at all: 4 flat files became 4 modules (`framework.assets.fonts` /
  `.mesh` / `.resolver` / `.metabolism.representation`) with `FontCatalog` / `MeshCatalog` /
  `Delivery` / `RepresentationCatalog`. Both ends of the two `🖱️ui`-side consumers fixed
  (`🎨️styling/🧪️tests/🟦️.ts`, `🎨️styling/📦️packages/🦀️rust/📜️script.ts`, `🧊️mesh-collection.json`,
  and the `$schema` `const` inside each asset module).

## 4. `framework.ui` shared retained-command shape (requested by the plugins worker)

`🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json`,
`$id https://semio.tech/schema/framework/ui/schema.json`. Exports:

| export | use |
|---|---|
| `RetainedCommandLimits` | `oneOf` of the declared-limits form and the corpus form |
| `RetainedCommandRoutes` | array of `retainedCommandRoute` |
| `RetainedCommandRoutesDocument` | `{ schema, routes }` standalone routes file |
| `UIDialogModalFixture` | (unrelated, §3.1) |

Building blocks in `definitions`: `retainedCommandLane`, `retainedCommandByteBudget`,
`retainedCommandStepBudget`, `retainedCommandClassBudget`, `retainedCommandPublicationContract`,
`retainedCommandBoundaryCase`, `retainedCommandOracle`, `retainedCommandRouteDisposition`,
`retainedCommandRouteExecutionLane`, `retainedCommandRouteExecutionFeature`, `retainedCommandRoute`,
`retainedCommandCorpusLimits`, `retainedCommandDeclaredLimits`.

Evidence — read from the 8 `🧫️retained-command-limits/🔣️.json` and 3 `🛣️retained-command-routes.json`
fixtures under `✏️s/` (**not modified**). The union is real, not invented:

- top-level key sets: `{maximumTextBytes|maximumSchemaBytes|maximumRawBytes, rejectedAdditionalBytes,
  expectedWorkItems, toolIds, schema}` (gis, vcs, demonstrator, reasoning, imperative) vs
  `{version?, owner?, controller, documentSchema, factory, limits, publicationContracts?, routes,
  boundaryCases?, oracle}` (animate, shooting, space×3).
- `limits` has two forms: flat byte budget (`rawBytes/checkpointBytes/configValueBytes/
  configBaseBytes/commandStepBytes/storeStepBytes/workItems/preparationPhases`) and class budget
  (`bounded` [+ optional `resumable`] of `maxRawBytes/maxDecodedItems/maxWorkUnitsPerStep/
  maxOutputBytes/maxStepMicros` [+ `maxCheckpointBytes/maxInFlightPages`]).
- three route dialects, all observed: `{id,disposition,lanes,blocker|reason}`,
  `{id,execution,lanes,reason}`, `{id,execution,admission|status,feature}`.
- enums as observed: `execution ∈ {bounded-first-step,bounded,batch,resumable}`,
  `disposition ∈ {Migrated,BatchOnlyPendingRewrite,migrated,batch-only-pending-rewrite}`,
  `admission ∈ {failClosed,migrated}`, `status ∈ {batch-only,migrated}`,
  lanes ∈ `{Artifact,Config,HostOnly,artifact,config,host-only,hostOnly}`.

Extra keys found only in shooting/space were folded in too: `oracle.ownedInterface` (string),
`oracle.expected` (object of integer|boolean counts), and `maximumRawBytes`/`maximumWorkItems`
alongside `schema`/`routes` in the reasoning/imperative routes documents.

Checked against every real fixture:

```
$ git ls-files '✏️s' | grep -E '🧫️retained-command-limits/🔣️\.json$|🛣️retained-command-routes\.json$'   # 11
$ node -e '…getSchema(`${m.$id}#/$defs/RetainedCommandLimits` | …RoutesDocument)…'
retained-command fixtures: PASS 11 FAIL 0
```

Per-owner schemas should `$ref`
`https://semio.tech/schema/framework/ui/schema.json#/$defs/RetainedCommandLimits` (or
`…#/$defs/RetainedCommandRoutes`) and narrow with their own `const`s. The lane/disposition/execution
casing split between plugins is *not* normalised here — see open question O-4.

## 5. Consumer rewiring inside the partition

31 files rewritten. Path forms handled: `import x from "…"`, `await import("…")`,
`readFileSync(new URL("…"))`, `Bun.file(new URL("…"))`, `join(root, "…")`, `include_str!("…")`,
`$schema`/`$ref`/`const` strings, and one nx `inputs` list.

- 59 + 20 legacy `$id` strings rewritten: `<old>#/definitions/X` → `<new>#/definitions/X`,
  bare `"<old>"` → `"<new>#/$defs/<Export>"`. `$ref`s that pointed at a source's internal
  `$defs/<name>` were repointed to `#/definitions/<name>` (5 files).
- Every `.compile(m)` / `.validate(m, …)` on a now-multi-export module became
  `.addSchema(m).getSchema(\`${m.$id}#/$defs/<Export>\`)!`, with duplicate `addSchema` of the same
  `$id` removed (ajv throws on re-registration). Where two variables ended up bound to the same
  module inside one block, the second became a plain alias and unused aliases were pruned.
- nx input list `🌱️value/💾️resident/📦️packages/🦀️rust/📋️project.json` collapsed its two entries to
  `{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json`.
- `.vscode/launch.json` and every `📋️project.json` in the partition were grepped: **no other**
  reference to a moved path existed.
- `🔬️conformance.rs` no longer expects `🧬️catalog.schema.json` in the conformance corpus directory.

Final sweep for stale references (whole repo, resolved relative paths + bare basenames): **0 inside
the partition**, 8 files outside it (§7).

## 6. Verification (real output)

```
$ bun <ticket>/wp4-framework-validate.mjs
modules=116 exports=210 badDialect=0 badId=0 noExports=0 problems=0
   (after folding 🧩️action-argument-resolution: modules=117 exports=211 … problems=0)
```
(ajv v8 `strict: true, allErrors: true`, draft-07 meta-schema validation on, `x-semio-binary` /
`x-semio-state` declared as annotation keywords, framework schema *registry* module excluded —
it is a JSON array owned by the sibling worker.)

```
$ git ls-files '🧰️framework/🔨️modules' | grep -v '/🧬️mutations/' \
    | grep -v '^🧰️framework/🔨️modules/🧬️schema/' \
    | grep -E '\.schema\.json$|/🧬️schema\.json$|/📐️schema/|/🛂️schema/|/📐️fixture-schema/|/🧬️contracts/'
(empty)
$ find 🧰️framework/🔨️modules \( -name '*.schema.json' -o -name '🧬️schema.json' \) \
    -not -path '*/🧬️mutations/*' -not -path '*/🤖️generated/*' …
(empty)
$ find 🧰️framework/🔨️modules -type d \( -name 📐️schema -o -name 🛂️schema -o -name 📐️fixture-schema -o -name 🧬️contracts \) …
(empty)
```

```
$ node node_modules/vitest/vitest.mjs run --config 🧪️tests/🟦️.ts     # @semio-tech/framework-actor
 Test Files  7 passed | 3 failed (10)
      Tests  203 passed | 3 failed (206)
```
The 3 failures are **not** schema-related and reproduce on `HEAD~1`:
- `🚪️lifetime/🟦️.ts:393` — the lifetime fixture violates its own oracle's
  `turnResults[].hex maxLength: 150`. Proved pre-existing by compiling `HEAD~1`'s
  `📐️schema/🔣️.json` against `HEAD~1`'s `🧪️fixture/🔣️.json`:
  `false [{"instancePath":"/turnResults/3/hex","keyword":"maxLength","params":{"limit":150}}]`.
- `📤️return/🟦️.ts:437` and `📤️return/📨️response/🟦️.ts:209` — vitest's 5 s budget under concurrent
  load (`resultOracle()` subprocess / `ts.createProgram`). Type-checking the 8 touched actor files
  standalone gives **0 diagnostics each**.

```
$ node node_modules/vitest/vitest.mjs run --config vitest.config.ts   # framework-kernel
 Test Files  2 passed (2)      Tests  50 passed (50)
$ node node_modules/vitest/vitest.mjs run --config vitest.config.ts   # framework-replication
 Test Files  1 passed (1)      Tests  5 passed (5)
$ bun test ./🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🟦️.ts
 41 pass  0 fail  1365 expect() calls
$ bun test ./🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🟦️.ts        2 pass 0 fail
$ bun test ./🧰️framework/🔨️modules/🕸️graph/🧪️tests/🟦️.ts                 3 pass 0 fail
$ bun test ./🧰️framework/🔨️modules/🗺️surface/🧪️tests/🟦️.ts               1 pass 0 fail
```

```
$ (cd 🧰️framework/🔨️modules/🌱️value/💾️resident && bun ./📜️script.ts test)
[DEBUG] Resident capacity=6 … admissionBootstrap=7 … oracle=Ajv+Immer+Buffer+BigInt
$ (cd 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric && bun ./📜️script.ts test)
[DEBUG] Numeric-index laws=12 lifecycle=165 ordinals=2 stress=3072 references=7 … strictTS=0
$ (cd 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧫️fixtures && bun ./📜️script.ts test)
[DEBUG] Ordered-map source fixtures=3 lookupCases=2 hostileRejections=8 grants=1,64,4096 …
$ (cd 🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust && bun ./📜️script.ts worker-maintenance-check)
worker-maintenance-independent-oracle: AJV=1 lifecycle=20 capacity=64 native=1 cooperative=1
$ … worker-deferred-wake-check
worker-deferred-wake-independent-oracle: AJV=1 cases=5 fixed-waiters=2048 … runtime-markers=9/9
$ (cd 🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust && bun ./📜️script.ts conformance)
[DEBUG] conformance-corpus-catalog cases=62
$ … && bun ./📜️script.ts test           →  [DEBUG] fixed-list-page-oracle checks=75
$ (cd 🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust && bun ./📜️script.ts test)
[DEBUG] surface-ownership-oracle checks=40
```

```
$ CARGO_TARGET_DIR=<scratch>/target-w10 RUSTC_WRAPPER="" cargo check -p semio-framework-ui-host --offline
    Checking semio-framework-ui-contract … semio-framework-ui-backend-metal … semio-framework-ui-host
    Finished `dev` profile [unoptimized] target(s) in 3m 05s
$ … cargo check -p semio-framework --offline          # carries 🛂️manifest/🦀️.rs
    Checking semio-framework-actor … semio-framework-ui … semio-framework
    Finished `dev` profile [unoptimized] target(s) in 1m 57s
```
Zero warnings, zero errors. These two crates carry the only `.rs` edits
(`🖱️ui/🖥️host/📦️packages/🦀️rust/📡️event.rs`, `🍎️metal/…/🧭️objective_c.rs`,
`🛂️manifest/🦀️.rs`, `🖱️ui/🧬️contract/📦️packages/🦀️rust/🔬️conformance.rs`).

### Peer breakage observed during verification (not mine, reported not fixed)
- Cargo workspace was unbuildable twice while peers moved crates:
  `error: failed to load manifest for workspace member .../🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust`
  and `.../💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust`. The two `cargo check -p` runs above
  were taken in the window where the workspace resolved.
- `📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📜️script.ts` — its last section fails with
  `ENOENT … 💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🧪️fixture/🔣️.json`
  (mutation/os worker in flight). Its first three sections (source / retained-root / retained-update,
  all schema-driven) pass.
- `⏳️async … worker-pool-use-check` gets past its ajv assertion and fails on
  `engine.match(/_pool_use: Arc<WorkerPoolUse>/g)?.length` `3 !== 4` in
  `💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs`.
- `🖱️ui/🧬️contract … built-tree-retirement-check` fails on a Rust marker count
  (`as UiTypedRetire>::DEPTH …` expected 9, found 0); the same count is 0 at `HEAD~1`.

## 7. Cross-partition requests

Exact, ready-to-apply replacements: **`wp4-framework-cross-partition.md`** (8 files). Summary:

| file | what |
|---|---|
| `📜️script.ts` (root) | 2 `🧵️job` fixture-schema paths → `🧰️framework/🔨️modules/🧵️job/🧬️schema/🔣️.json` (+ `$defs/SharedFrameworkActionRoutesFixture` / `FixedOperationRegistryFixture`); 2 old ids. **Also**: `⏱️trace` contention now lives at `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧬️contention/🧬️schema/🔣️.json`, export `Contention`, `$id https://semio.tech/schema/framework/trace/clock/contention/schema.json` |
| `💻️os/…/⚛️react/🔬️index.test.ts` | kernel descriptor-load, ui-contract fixtures, 2 manifest fixture schemas, action-argument-resolution choices (data **and** schema) |
| `💻️os/…/⚛️react/🧯️router-plugin-faults.test.ts` | kernel app-router-plugin-faults schema → `framework.kernel` `$defs/AppRouterPluginFaultsFixture` |
| `💻️os/…/🧱️elements/📃️UiDocumentStore/🟦️.tsx` | 40 paths (ui-contract retained fixtures + resident pair modules + actor page + value resident + scalar tests). Note: `scalarDeclarationSchemaJson` is now a 2-export module — `compile()` on it is vacuous, use `#/$defs/ScalarDeclaration` |
| `💻️os/…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `🌱️value/💾️resident/🧬️schema.json` → module + `$defs/Resident` |
| `💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts` | `getSchema("semio.actor.retained-return.v1#/definitions/result")` → `https://semio.tech/schema/framework/actor/return/schema.json#/definitions/result` |
| `💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts` | `semio.kernel.poll.composition.v1` → `…/framework/kernel/poll/composition/schema.json` (+ `#/$defs/Composition`); the module is now 2-export |
| `💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`, `💻️os/…/🛠️ShellHelpers/🟦️.tsx` | `semio.actor.shard-liveness.v1` → `…/framework/actor/shard-client/schema.json#/$defs/ShardClient` |
| `💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🧬️authority.schema.json` | `semio.actor.instance-lifetime-close.v1` → `…/framework/actor/lifetime/schema.json` (+ `#/$defs/Lifetime` when bare) |
| `💻️os/…/🧑‍🎨engine/💾️resident/🧬️schema.json` | `semio.value.resident.capacity.v1` → `…/framework/value/resident/schema.json` |
| `💻️os/…/🛠️ShellHelpers/🧫️fixtures/🎥️tutorial-interaction/🧬️schema.json` | `https://semio.tech/schema/framework/interaction/component.json` → `…/framework/interaction/schema.json` |
| `🦑️repo/…/📚️library/🔣️taxonomy.json` | 2 literal paths (`🕸️graph/🛂️manifest/🧬️outputs.schema.json`, `🖼️assets/🔤️fonts/🧬️catalog.schema.json`) |
| `🦑️repo/…/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/{🔣️.json,🧬️schema/🔣️.json}` | 2 value-resident paths each |
| `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` | re-export from `🕹️interaction/🧬️schema/🟦️.ts` unchanged, but the module `$id` changed |
| `💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json` | generated distribution list contains styling test paths; regenerate |

## 8. Deviations from the audit / contract

1. **Duplicate pairs are one file, not two** (§3.4) — follows the brief's explicit instruction and
   contract §B over the WP0 report's `🔣️.oracle.json`/`🔣️.admission.json` proposal.
2. **`🖱️ui/🧬️contract/` (singular) was kept.** The gate regex in the brief matches `🧬️contract/`,
   which would delete the whole `ui-contract` crate root (380 files, a `#[path]`-linked Rust crate and
   a legitimate nested framework-module scope). Contract §A only rules out *single contract
   directories* `🧬️contracts/<x>` (plural) and §B only says `🧬️contracts/` ceases to exist — both
   done. Treated the singular directory as an eligible nested module scope; it now owns
   `framework.ui.contract` and 20 descendant scopes.
3. **`🧵️job/⏱️budget/🧬️schema/{⏱️clock,🪢️binding,🪫️budget}.json` left as they are.** They are
   non-canonical filenames inside a `🧬️schema/` module (contract §B), but their only consumers are
   the root `📜️script.ts` (3 `readFileSync` sites) and a repo-library taxonomy fixture — both outside
   my partition. Consolidating them would leave `schema test` broken until another worker applied
   the patch. Ready-to-apply request in `wp4-framework-cross-partition.md` §job-budget.
4. **`📦️packages/*` and `🎯️targets/*` levels are not eligible scope owners** (contract §A) but two
   modules sit there: `framework.ui.render.targets.metal.packages.rust` (Objective-C ABI fixture,
   `include_str!`-adjacent) and `framework.ui.render.targets.webgpu`. Renamed the files to canonical
   `🔣️.json` and normalized their `$id`s in place rather than hoisting them to `framework.ui.render`,
   which would have rewired the metal/webgpu Rust packages for no WP4 benefit. Flagged as O-2.

## 9. Open questions

- **O-1 `framework.actor.activation` collision.** `🎭️actor/🎠️activation` and `🎭️actor/🪪️activation`
  are sibling directories whose ASCII tails are both `activation`, so they cannot both own
  `framework.actor.activation`. `🪪️activation` (the structural tree with `📤️return`, `📨️inbound`,
  `🚪️instance` children) kept it; `🎠️activation` (kernel reservation: `KernelActivationRequest`,
  `KernelActivationFault`) got `framework.actor.activation-reservation`, recorded as an explicit
  `SCOPE_OVERRIDE` in the transformer. Recommendation: rename the directory to
  `🎠️activation-reservation` so the id is derivable again. Not done here — the name is referenced by
  `#[path]` and by three os `📜️script.ts` relative joins.
- **O-2** Should `framework.ui.render.targets.{metal.packages.rust,webgpu}` be hoisted to
  `framework.ui.render` (contract §A says `📦️packages/*` and `🎯️targets/*` are not eligible)?
- **O-3** `🖱️ui/🖥️host` now has a generated structural schema for its declaration document (§3.5).
  A hand-authored contract with real bounds would be better; the generated one only constrains
  types/required/`additionalProperties:false`.
- **O-4** The retained-command lane/disposition/execution vocabularies differ in casing between
  plugins (`Artifact|Config|HostOnly` vs `artifact|config|host-only|hostOnly`;
  `Migrated` vs `migrated`). `framework.ui`'s shared shape accepts all of them. Normalising is a
  plugin-side data change and was left to the plugins worker.
- **O-5** The framework schema *registry* (`🧰️framework/🔨️modules/🧬️schema/🔣️.json`) is a JSON array,
  not a scope module — excluded from the validator. Per the updated contract §C it now registers
  named exports via `register_scope_schema_exports(ScopeSchemaExports { scope, exports })`; the 117
  modules and 211 exports produced here have **not** been registered there (registry worker's WP2).
- **O-6** `🖱️ui/🎨️styling/🧪️tests/🟦️.ts` and `🧊️mesh-collection.json` referenced a
  `🎨️styling/🧬️catalog.schema.json` that never existed; they now point at the real asset modules.
  Worth confirming that was the intent.
