# JC1 — jco async `task.return` indirect lift, and the catalog republish

Slice JC1 (session 6, 2026-09-21). Owner: the LAST blocker of outcome 3, named by C4 §3 —
every hub document's browser actor dies on its first `reactor.poll` because jco 1.27.0 binds an
async export's `task.return` trampoline with `useDirectParams: true` even when the result is
unflattenable (`variantFlatCount: null`) and must travel through the return-area pointer.

## 0. HANDOFF (top of report)

| field | value |
|---|---|
| **`reactor.poll` lifts a result on the fixed actor** | **YES** — the real gis component, the production closure, activated in Bun under the real WASI/host ports: a fully lifted `TurnResult`, no TypeError (§3). The same harness on the published 1.27.0 actor reproduces C4's exact `_liftFlatVariantInner@3673:11 < taskReturn@2751:33`. |
| **the fix** | **route (a): a version bump.** `@bytecodealliance/jco` 1.27.0 → **1.34.0** (`js-component-bindgen` 0.6.1 → 0.14.0). Bisected: **0.13.0 is the first build** that emits `useDirectParams: false` for an unflattenable async result. **No `bun patch`, no `patchedDependencies`, no post-processing, no WIT change.** |
| **the law** | `validateAsyncTaskReturnLift` — a **build-time refusal** in `🌐️browser-bundle/📜️script.ts:425`, run on every actor the repo closes. It refuses the exact 1.27.0 actor that has been dying in browsers, and admits the 1.34.0 one. Pinned in the suite with 7 hostile rows. |
| hub with the re-published catalog | **not yet** — the bootstrap is running (§4). |
| port / data root | **7621** / `.🧬semio/🌐hub/jc1-boot` (**gm1-boot and hub 7611 are untouched**) |
| pid, once up | `🗑️generated/jc1-hub-pid.txt`; live log `🗑️generated/jc1-hub-dev.txt` |
| binary | `⚡️cache/cargo/target-jc1/debug/os-hub`, built by the bootstrap itself — **mandatory**: the Rust twin of the policy moved to `…-1.34.0-…`, so HS1's and the coordinator's binaries refuse this catalog (§6) |
| resume command | `cd /Users/ueli/Documents/semio && nohup zsh ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️jc1-hub-boot.sh" 7621 > /dev/null 2>&1 & disown` |

Inherited, untouched by this slice: hub **7611** pid 48489 data root `gm1-boot`; serves **6190**
(`s`) and **6191** (`gis2d`). Never touched.

Disk at slice start: **23 GiB** free on `/System/Volumes/Data` (gate is 18 GiB).

## 1. Route decision — (a) version bump, MEASURED, no patch needed

**Route (a) wins outright: upstream already fixed it, and the fix is a published version.**

The decision was made by transpiling the REAL published gis component
(`…/generations/94a9788a…/packages/gis/component.wasm`, 47 440 798 B, copied read-only) with every
published `js-component-bindgen` vendor build between the one jco 1.27.0 carries and the newest one,
using byte-identical `generate(...)` options to `🌐️browser-bundle/📜️script.ts:313`. Probe:
`🐍️jc1-transpile-probe.mjs <vendor js-component-bindgen-component.js> <component.wasm> <outDir>`;
capture `🗑️generated/jc1-version-bisect.txt`.

The probe resolves the actual binding under test rather than a line number: it reads
`'[task-return]poll': … exports0['8']` out of the generated export table, follows `'8'` to its
`trampolineN`, and reports that trampoline's `useDirectParams` together with its outer result meta.

```
jco-transpile  pollTrampoline  useDirectParams  outerVariantSize32  outerVariantFlatCount
0.6.1          trampoline27    true             344                 null      <- jco 1.27.0, the bug
0.6.2 … 0.12.1 trampoline27    true             344                 null
0.13.0         trampoline27    false            344                 null      <- FIXED
0.14.0         trampoline27    false            344                 null
```

Every row is one real transpile of the real component (~2–5 s each), so this is a measurement, not a
changelog reading. `variantSize32: 344` / `variantFlatCount: null` is C4 §3.3's meta 27 exactly, and
0.6.1 reproduces C4's `useDirectParams: true` from a tarball, which pins the harness to the published
`closed-actor.mjs` the browser actually fetched.

**`js-component-bindgen` 0.13.0 is the first build that emits `useDirectParams: false` for an
unflattenable async result** — i.e. it selects the indirect branch `taskReturn` already carries
(`liftCtx.storagePtr = params[0]; liftCtx.storageLen = params[1]`), which is precisely C4 §3.4
route 1, upstream, with no seam of ours.

Corroborating detail, read out of the two vendor cores' embedded emitter templates: 0.14.0 also
rewrote `_liftFlatVariantInner`'s direct-param path around new `variantPayloadFlatTypes` /
`caseFlatTypes` metadata and a join-reinterpretation scratch buffer (`see CanonicalABI
lift_flat_variant`), and moved the "missing memory despite indirect param usage" guard from
`ctx.memory` to `memory`. The emitted `taskReturn` body itself is otherwise unchanged — the fix is at
the **binding site**, not in the runtime, which is why C4's route-1 reading of the seam was right.

Registry mapping of jco → bindgen (`@bytecodealliance/jco` `dependencies`):

| jco | jco-transpile | fixed |
|---|---|---|
| 1.27.0 (installed) | ^0.6.1 | no |
| 1.28.0 … 1.32.1 | ^0.7.0 … ^0.12.1 | no |
| **1.33.0** | **^0.13.0** | **yes** (first) |
| 1.34.0 | ^0.14.0 | yes (latest) |

`0.13.0` and `0.14.0` produce a **byte-identical** `browser-actor.js` and byte-identical core wasms
for this component, so the two are indistinguishable for the actor and the choice is only about how
current the toolchain is.

**Routes (b) and (c) are not taken.** No `bun patch`, no `patchedDependencies` row, no
post-processing of generated output, and `reactor.poll`'s WIT result is untouched.

Two properties the bump must not break, both already measured on the transpiled output:

| property | 0.6.1 | 0.13.0 / 0.14.0 |
|---|---|---|
| `output.imports` (TC1's admitted-import law, `📜️script.ts:316`) | 17 names | **identical set**, `diff = []` |
| `_lowerImport*.bind(` sites (`validateWasiSuspension` parses these) | 40 | 40 |
| generated file count / names | 32 | 32 |

New in 0.13.0+: `_guardMayLeave(` ×22 and `_jcoMaySuspend` ×60 wrappers around core exports — the
export table entries become `Object.assign(exports0['8'], { _jcoMaySuspend: false })` instead of a
bare `exports0['8']`. That is the one shape change the repo's own parsers must be re-proven against.

## 2. Landing the bump — LANDED, suite green

`@bytecodealliance/jco` `^1.7.0` → `^1.34.0` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/package.json:29`, then
`bun install` (`🗑️generated/jc1-bun-install.txt`, `43 packages installed`, lockfile saved). Resolved:

```
@bytecodealliance/jco            1.34.0   (was 1.27.0)
@bytecodealliance/jco-transpile  0.14.0   (was 0.6.1)
@bytecodealliance/preview2-shim  0.25.0   (was 0.20.1)
typescript (top level)           5.9.3    unchanged — jco 1.34.0's own typescript 7.0.2 stays nested
                                          under node_modules/@bytecodealliance/jco/node_modules
```

That last line matters: `📜️script.ts:169` refuses to build unless `ts.version === "5.9.3"`, and the
bump does not disturb it.

The codegen policy identity is part of every published actor record, so the constant moves with the
tool: **`semio.os.browser-jco-1.27.0-jspi.v1` → `semio.os.browser-jco-1.34.0-jspi.v1`**, in all 15
places that spell it (TS, the Rust twin the hub binary enforces, three JSON schemas, and the hub's own
catalog/lease fixtures) — §7 lists them. Version literals moved in the same pass.

### 2a. What the bump broke, and why each repair is the root

The suite `closed-browser-component-factory-check` was run after every repair. Capture:
`🗑️generated/jc1-browser-bundle-laws.txt`.

| law | symptom | repair |
|---|---|---|
| `browser-codegen-policy` / `browser-actor-factory` | `compiler.inputs.length` `11 !== 8` | the count is now **11** |
| `browser-wasi-activation` oracle (subprocess) | `write exceeds the permit returned by checkWrite` | the oracle now calls `checkWrite()` first |
| `browser-wasi-activation` line-buffer oracle (in process) | same | same, per chunk |

**The input count is not a magic number weakened.** `🐍️jc1-compiler-closure.ts` re-derives the closure
independently (`🗑️generated/jc1-compiler-closure.txt`): all **11** modules are inside the three
admitted roots, `OUTSIDE:` is empty. The three new ones are preview2-shim 0.25.0 splitting its browser
shim (`config.js`, `in-memory-filesystem.js`, `opfs-filesystem.js`). The law's property — an exact,
root-admitted closure with no ambient module — is intact; only the exact number moved.

**The `checkWrite` repair fixes the oracle, not the product.** preview2-shim 0.25.0 now enforces the
Preview2 contract: `#permit` starts at `0n` and only `check-write` raises it
(`preview2-shim/dist/browser/io.js:104,116-136`). The oracle was writing without asking for a permit —
a Preview2 client bug that 0.20.1 tolerated. semio's own shim already enforced the same rule, and the
law asserts it four lines above (`limited.checkWrite()` then `limited.write(...)`). So the oracle is
now a *correct* client and the cross-check is strictly stronger than before.

```
browser-actor-codegen-manifest: AJV=1 canonical-arrays=1 valid=1 denied=11
browser-compiler-capsule:       AJV=1 native-Wasm-oracle=1 valid=1 denied=11
browser-compiler-sources:       AJV=1 Node-oracle=1 WebCrypto=1 laws=6 inputs=2
browser-codegen-policy:         AJV=1 stable-stringify=1 WebCrypto=1 laws=7 inputs=13
browser-wasi-activation:        AJV=1 TypeScript=1 Preview2=1 actors=2 laws=17 resources=256 waiters=128
browser-host-activation:        AJV=1 TypeScript=1 laws=22 actors=2 pending-bound=128 close-streams=2
browser-actor-factory:          AJV=1 JCO=1 native-Wasm-oracle=1 actors=2 laws=10 artifact-laws=20 bytes=107507
browser-component-factory:      AJV=1 JCO=1 native-Wasm-oracle=1 actors=2 hostile=11 cancellation=3 bytes=93832
exit=0   (all 8 groups)
```

`browser-actor-factory: JCO=1 … bytes=107507` is a real actor built and closed end to end by
`buildClosedBrowserActorArtifactV1` under jco 1.34.0, including `validateWasiSuspension` over the new
generated shape (the `_guardMayLeave` / `_jcoMaySuspend` wrappers did not disturb its parser).

**TC1's admitted-import law survived untouched.** `output.imports` is the same 17-name set under
0.6.1 and 0.14.0 (§1), so no allowlist, schema bound or vocabulary copy needed widening.

## 2b. A THIRD defect the bump exposed — `validateWasiSuspension` was blind to it — FIXED

The suite went green, and then the first real activation still refused:

```
browser actor bundle: blocking WASI import requires JSPI suspension
```

**The suite could not have caught this**: `validateWasiSuspension` only runs when the actor imports a
WASI interface (`wasiRequired`, `📜️script.ts:423`), and the suite's fixture component imports only
`semio:framework/pure@1.0.0`. So the validator is exercised for the first time by a real plugin —
and it would have killed the bootstrap at `derive`, ~40 min in, exactly as TC1 §2 and §4a were killed.

jco 1.33.0+ changed two generated shapes the validator reads. Both repairs make it require the NEW
shape rather than tolerate either — no compat branch.

| # | shape | 1.27.0 | 1.34.0 | repair |
|---|---|---|---|---|
| 1 | JSPI wrapper | `new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(null, {…importFn: _trampoline13…}))` | `new WebAssembly.Suspending(_suspendingImport(0, _lowerImportBackwardsCompat.bind(null, {…})))` | new `suspendingBinding()` unwraps `_suspendingImport(<componentIdx>, <bind>)` and then applies the existing `lowerBinding` check |
| 2 | core imports | `instantiateCore(module0, { … })` | `instantiateCore(module0, (_setGlobalCurrentTaskMeta({…}), { … }))` | new `coreImportsExpression()` walks parentheses and comma operators to the object literal |

Shape 2 is the nastier one: the imports argument became a **sequence expression**, so
`ts.isObjectLiteralExpression` silently failed, `coreBindings` stayed empty and every blocking import
was denied at `!coreBindings.has(wrappers[0])` — a *silent* loss of the whole check, not a parse error.

`_suspendingImport` is not noise: it is jco's new `_checkMayLeave` + "may this task block" guard that
only suspends when the current task is allowed to (`browser-actor.js:4777-4795`), and it falls back to
a synchronous call otherwise. Requiring it makes the law strictly stronger than before.

## 2c. The coordinator's seven hub reds — MINE, and the cause was an incomplete rename

At 02:07 the coordinator reported seven reds in this lane, all one message —
`Catalog("trusted browser actor identity differs from its package or renderer")`
(`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🌐️browser-actor/🦀️.rs:54`) — plus two
`checkpoint_publication_route_*` failing while *building* the catalog fixture at `🔬️bin-unit:396`.

**They were mine and they were real.** The first rename pass missed **four** copies of the policy
constant, because the `grep | head -30` that enumerated them returned exactly 30 lines and truncated.
The Rust twin (`📇️directory/🧬️schema/🌐️browser-actor/🦀️.rs:120`) demanded `…-1.34.0-…` while four
producers still emitted `…-1.27.0-…`, and the Rust constant is compiled in while the JSON is read at
run time — so the mismatch surfaces only when a test builds a catalog. Landed now:

| file | what it is |
|---|---|
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🌐️browser-actor/🔣️.json` | the actor fixture the catalog laws load |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:390` | **the exact builder the coordinator named** |
| `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs:141` | the profile law's inline catalog |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts:3879` | the hub script's independent TS admission |

`grep -rn "browser-jco-1\.27\.0"` over all `*.ts|*.tsx|*.rs|*.json` outside `node_modules` and
published data roots now returns **0**; 11 files carry `…-1.34.0-…`.

### A SEVENTH copy of the interface vocabulary, which TC1 did not count — also fixed

Reading that last file to repair it turned up an independent re-implementation,
`documentOpenNeutralBrowserActor` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:3851`), that TC1's six-copy
census missed and **no law pins**. It was still on the pre-TC1 vocabulary:

- its `allowed` list had 16 names — **no `wasi:clocks/wall-clock@0.2.0`, no
  `wasi:random/insecure-seed@0.2.9`**, the two TC1 admitted and implemented;
- its bound was `importInterfaces.length > 16`, while the real gis actor carries **17**
  (GM1 §0 observed `browserActor.importInterfaces=17` on the published plan).

So this checker would have thrown `actor-import` on every genuine plugin actor. Fixed: `wall-clock`
joins the `@0.2.0` list, `insecure-seed@0.2.9` is added as a literal (its version differs), bound
16 → 18 to match all six other copies. **The vocabulary now has seven copies and all seven agree.**

### The last red — the fixture's generation digest covers the policy string

The coordinator's 02:45 run took six of the seven reds green and left
`…browser_actor::tests::trusted_browser_actor_metadata_and_generation_match_neutral_corpus`. That test
(`🌐️browser-actor/🧪️tests/🔬️unit/🦀️.rs:28`) hashes `append_generation`'s output and compares it to
`encodingSha256` in `🧫️fixtures/🌐️browser-actor/🔣️.json` — and `append_generation` frames
**`codegen_policy`** as its second field, so renaming the policy necessarily moves that digest.

Recomputed **from the producer**, not by hand: the encoder is
`trustedBootstrapBrowserActorEncoding` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:7862`) over
`trustedBootstrapField` (u64-BE length ++ bytes) and `trustedBootstrapCount` (u32-BE), and its Rust
twin `append_generation`. The model was proven exact **twice** before it was used:

1. the fixture's untouched `noneEncodingSha256` recomputes bit-for-bit
   (`9506f170…ab6c`, the `kind: "none"` encoding is a single framed field);
2. re-running the closed encoding with the **old** `…-1.27.0-…` string reproduces the stored
   `ac0d03c7…48d4` exactly — so the policy string is the only input that moved.

```
encodingSha256  ac0d03c78b5a583be33f341b4e526c4d2759669e23cc404a3c76217c9a5548d4   (1.27.0)
             →  41e1e0f163f884b5546753389be0e7ea2e53fd16bcedf927fbd3584ff8383ed7   (1.34.0)
```

`noneEncodingSha256` does not contain the policy and is unchanged.

**A second failure in that verb is NOT mine.** `document-browser-actor-identity-check` also stops
earlier, at `proveExecutionTargetLeaseCorpus` → "positive vector was denied at manifest"
(`📜️script.ts:5358`). `🐍️jc1-lease-admission-probe.ts` extracts the hub script's own
`documentOpenNeutralBrowserActor` and runs it against both actor rows of the lease fixture:

```
plan:     ADMITTED
manifest: ADMITTED
```

So the actor clause — the only clause this slice touched — admits both, and the denial is elsewhere in
`executionTargetLeaseFieldsAdmissible`'s field boolean. Pre-existing or a peer's; **not renamed by JC1
and not fixed by JC1.**

**`cargo check -p semio-hub --all-targets` was NOT run.** Preamble rule 7 allows one cargo from this
slice at a time and the republish chain's `cargo rustc` (pid 18559, live rustc child 22415) is
building; racing a second cargo into it is exactly what rule 26 was written about. The four repairs
are string-literal substitutions inside JSON string constants and a TS array — none can change a
type — but that is reasoning, not a compile. **The hub suite needs a rerun.**

## 3. The law — instantiate the real gis actor and call `reactor.poll` — OBSERVED

`🐍️jc1-poll-lift-probe.ts <transpiled dir | closed-actor.mjs> <outDir>` closes the transpiled output
through the production `closedBrowserActorBundle` (so `validateWasiSuspension`, the capsule closure
and the retained `🌐️host` / `🌐️wasi` runtime modules are all in the path), imports the bundle,
`activate`s it under the same port shape the real child worker supplies
(`🧵️child/👷️worker/🟦️.ts:40` — `dispatch`/`log`/`traceSpan` all **denied**, `nowMs`, and a `wasi` port
with `nowNs`/`wallNs`/`write`/`exit`), then invokes `describe` and `reactor.poll` once. No hub, no
browser, no serve. Bun 1.3.14 has JSPI natively (`WebAssembly.Suspending` and `.promising` are
functions), so the JSPI actor runs unflagged.

Both runs use the SAME harness and the SAME `packages/gis/component.wasm` (47 440 798 B) copied
read-only out of the published generation `94a9788a…`.

| actor | bytes | `describe` | `reactor.poll` |
|---|---|---|---|
| **published, jco 1.27.0** (the file the browser fetched) | 63 651 023 | ok, `Uint8Array(477654)`, 1541 ms | **`TypeError: undefined is not an object (evaluating 'caseMetas[caseIdx]')`** |
| **rebuilt, jco 1.34.0** | 63 682 719 | ok, `Uint8Array(477654)`, 920 ms | **ok, 10 ms, a fully lifted `TurnResult`** |

The baseline's stack is byte-for-byte C4's live DOM reading, from a shell script instead of a browser:

```
at _liftFlatVariantInner (published-1.27.0.mjs:3673:11)
at taskReturn            (published-1.27.0.mjs:2751:33)
```

C4 read `_liftFlatVariantInner@3673:11 < … < taskReturn@2751:33` off `[data-semio-execution-target-diagnostic]`
with two signed-in humans on hub 7611. **The same two line:column pairs, in the same file, reproduced
headlessly in 28 ms.** That closes any doubt that the probe and the live failure are the same event.

The fixed actor's lifted result, in full:

```json
{ "uiPatches": [], "effects": [], "presence": [],
  "status": { "tag": "idle" }, "fuelUsed": 0n,
  "commandIngress": { "kind": 0, "cursor": { "owner": 0n, "generation": 0n, "commandIndex": 0,
    "commandCount": 0, "instance": 0, "seq": 0n, "kind": 0, "pageIndex": 0, "pageCount": 0,
    "itemCount": 0, "metadata": 0 }, "fault": "Uint8Array(0)" },
  "coldPairIngress": { "tag": "idle" } }
```

Every field of the 344-byte record is present and typed — the discriminant, the three lists, the
nested `commandIngress.cursor` record and both nested variants. This is the value that used to be
`caseMetas[<linear memory offset>]`.

Captures: `🗑️generated/jc1-poll-lift-0.14.0.txt`, `🗑️generated/jc1-poll-lift-1.27.0-baseline.txt`.

### 3a. The law in the browser-bundle suite — a REFUSAL, not a test

The suite's own actor fixture is **synchronous**: its generated actor contains zero `taskReturn.bind`
sites, so any invariant asserted over it would be vacuous. And the real subject is a 47 MB component
that lives in a hub data root, which a hermetic law may not reach. So the law was landed one level up,
where it is strictly stronger than a test:

`validateAsyncTaskReturnLift(source)` in `🌐️browser-bundle/📜️script.ts:425`, called unconditionally
from `closedBrowserActorBundleFromRuntime` (`:464`) beside `validateWasiSuspension`. It walks the
generated actor's TypeScript AST and, for every `taskReturn.bind(null, { … })`, **refuses** when

- `useDirectParams` is `true` while any element of `liftFns` is a `_liftFlat*({ … })` whose own
  top-level `variantFlatCount` is `null` or `undefined`, or
- `useDirectParams` / `liftFns` are missing, or are not a literal `true`/`false` and an array literal
  (an identifier there would let the property be smuggled past the check).

That is C4 §3.3's named defect stated as a build-time law: **no actor carrying it can ever be closed,
published or reach a browser again**, on any jco version, for any plugin. It returns the number of
bindings it inspected so callers and laws can prove it was not vacuous.

Verified against both real generated actors (`🐍️jc1-task-return-law-check.ts`,
`🗑️generated/jc1-task-return-law.txt`):

```
out-0.6.1/browser-actor.js   refused   "unflattenable async result requires an indirect task.return"
out-0.14.0/browser-actor.js  admitted  taskReturnBindings = 7
```

The refused file is the generated source of the actor that has been failing in the browser since the
catalog was published. **Had this law existed, TC1's `derive` would have refused it in 2026-09-20's
run instead of publishing it**, and C4's slice would not have been needed.

Pinned in the suite by `testAsyncTaskReturnLift` (`🧪️tests/🌐️browser-bundle/🟦️.ts`) over the new
fixture `🌐️browser-bundle/🧫️fixtures/🪝️async-task-return/🔣️.json`: one admitted source carrying all
three shapes at once (flattenable+direct, unflattenable+indirect, a non-variant lift) and **7 hostile
rows** — the exact 1.27.0 defect with `null`, the same with `undefined`, the defect hidden behind a
leading harmless lift, a missing `useDirectParams`, a missing `liftFns`, a non-literal
`useDirectParams`, and a non-literal `liftFns`.

```
async-task-return-lift: TypeScript=1 admitted=1 bindings=3 denied=7
```

Full suite, 9 groups, `exit=0` (`🗑️generated/jc1-browser-bundle-laws.txt`).

`🐍️jc1-poll-lift-probe.ts` stays ticket-owned and parameterised — it is the end-to-end measurement on
a real 47 MB component, which belongs beside the catalog, not inside a hermetic suite.

## 3b. Repo gates

| gate | command | result |
|---|---|---|
| browser-bundle suite | `dev … closed-browser-component-factory-check` | **exit 0**, 9 groups (`🗑️generated/jc1-browser-bundle-laws.txt`) |
| TypeScript, os product | `tsc -p 🧰️framework/🛍️products/💻️os/tsconfig.json --noEmit` | **0 errors in any file JC1 changed** (`🗑️generated/jc1-tsc.txt`) |
| dependency truth, ratchet | `verify dependencies` | **JC1 adds no dependency** — see below |
| dependency truth, target | `verify dependencies literal-external` | repo-wide red, pre-existing, no jco row |

`verify dependencies` reports **8 NEW dependencies not in `🔒️dependencies.json`** — `@types/bun`,
`@types/markdown-it`, `@types/micromatch`, `@types/picomatch`, `graphql`, `micromatch`, `picomatch`
and `rust:naga@29.0.4`. **Every one is a peer's**, uncommitted in the root manifest and in
`🖱️ui/🖌️render`; none is JC1's. A version bump of an already-frozen dependency is not a new row, so
`@bytecodealliance/jco` does not appear.

`literal-external` is the repo's red-until-zero mode (`📜️script.ts:8765` says so in as many words):
`target=0, current=237`, with 15 rust `oracle-conflict` rows and 2 `nx` `toolchain-owner-conflict`
rows. It was red before this slice and names nothing of JC1's.

**`🔒️dependencies.json` was deliberately NOT regenerated.** `write-baseline` rewrites the frozen
baseline wholesale, which would silently absorb those 8 peer-introduced dependencies into the ratchet
under this slice's name. The baseline's jco row therefore still reads `"version": "^1.7.0"`; refreshing
it belongs to whoever lands those 8, in one honest pass.

## 4. Republish + live two-user observation

**(in progress — the bootstrap is running; this section is filled only with what is observed.)**

Chain: `📜️jc1-hub-boot.sh 7621`, launched detached, inside the fleet wasm mutex (rule 27), private
`CARGO_TARGET_DIR=⚡️cache/cargo/target-jc1`, **NEW** data root `.🧬semio/🌐hub/jc1-boot` (gm1-boot is
never touched), log `🗑️generated/jc1-hub-dev.txt`.

**Disk gate honoured:** 22 GiB free at launch, above the 18 GiB floor the slice was given.

### 4a. The first launch was killed on purpose, 14 min in

`🌎️hub/📦️packages/🦀️rust/📜️script.ts:72` imports `buildClosedBrowserActorArtifactV1` **statically**, so
the bootstrap process pins the browser-bundle module as it was at process start. The first launch went
up before §2b's two `validateWasiSuspension` repairs existed, so it was carrying the old parser and was
guaranteed to die at `derive` — ~40 min of wasm-release compilation later, with nothing to show.

Killed by pid, leaves first (`16837` rustc → `13045` cargo → `12363` bun → `12352` mutex → `12346`
wrapper); `cat /tmp/semio-wasm-build.lock/owner` then read free, and no `target-jc1` cargo or
`semio_s_plugin` rustc orphan survived. Two peers' mutex scripts (pids `5409`, `31379`, waiting 9 h 35
and 5 h 56) were left alone. Relaunched at 02:02:53, holding the mutex.

## 5. Upstream issue text for `bytecodealliance/jco`

Filed-ready prose; **the defect is fixed in `js-component-bindgen` 0.13.0 / jco 1.33.0**, so this is a
report of *which* release fixed it rather than a request, and is useful to upstream mainly as a
regression pin.

> **Title:** async `task.return` bound with `useDirectParams: true` for an unflattenable result
> (fixed between jco-transpile 0.12.1 and 0.13.0 — regression test?)
>
> Transpiling a `wasm32-wasip2` component with an `async` export whose result is a 344-byte
> `result<record, …>` (`instantiation: async`, `asyncMode: jspi`, `base64Cutoff: 0`), jco 1.27.0
> emits
>
> ```js
> const trampoline27 = taskReturn.bind(null, {
>   useDirectParams: true,
>   liftFns: [_liftFlatResult({ …, variantSize32: 344, variantFlatCount: null })],
> });
> ```
>
> `variantFlatCount: null` is the generator's own statement that the result could not be flattened,
> so the guest returns **one pointer** into the return area. With `useDirectParams: true`,
> `_liftFlatU8` reads `ctx.params[0]` — that pointer — as the variant discriminant, and
> `caseMetas[<linear memory offset>]` is `undefined`:
>
> ```
> TypeError: undefined is not an object (evaluating 'caseMetas[caseIdx]')
>   at _liftFlatVariantInner (…:3673:11)
>   at taskReturn            (…:2751:33)
> ```
>
> The emitted runtime already has the correct branch — `taskReturn` sets
> `liftCtx.storagePtr = params[0]; liftCtx.storageLen = params[1]` when `useDirectParams` is false —
> and the emitted `_liftFlatVariantInner` even has a guard for exactly this case
> (`if (variantFlatCount === undefined || variantFlatCount === null) throw new Error("cannot lift
> variant with unknown flat count")`), but it sits ~24 lines **below** the destructuring that throws
> first, so the anticipated condition can never be reported and always surfaces as an opaque
> `TypeError`.
>
> Exports of the same component whose results are 16 and 20 bytes (`variantFlatCount` 4 and 5) work;
> only the unflattenable one fails.
>
> **Bisected against one fixed 47 MB component**, transpiling with each published
> `js-component-bindgen` vendor build: 0.6.1 … 0.12.1 emit `useDirectParams: true`; **0.13.0 and
> 0.14.0 emit `useDirectParams: false`** for the same trampoline, and the lifted value is correct.
> 0.13.0/0.14.0 output for this component is byte-identical.
>
> **Two asks:** (1) a regression test pinning `variantFlatCount === null ⟹ useDirectParams === false`
> for an async `task.return`, since the failure is silent at codegen time and only appears at the
> first call; (2) move the `cannot lift variant with unknown flat count` guard above the
> `caseMetas[caseIdx]` destructuring so a future recurrence names itself.

## 6. Honest gaps

- **§4 is not finished.** No hub of this slice has answered `/readyz`, no catalog has been published,
  and **no browser has been attached**. Nothing below §3 is claimed as observed.
- **`reactor.poll` is proven in Bun, not yet in a browser.** The activation, the WASI/host port shape
  and the JSPI path are the product's own, and Bun's JSPI is the same proposal Chromium implements,
  but the live-browser re-run of `🐍️c4-actor-reason-probe.mjs` against a hub serving the new catalog
  is the measurement that closes outcome 3, and it has not run.
- **`poll` was called with an empty event list on a freshly activated actor**, which is the cheapest
  call that exercises the 344-byte return path. It proves the lift; it does not prove a mutation
  round trip.
- **`validateAsyncTaskReturnLift` asserts one direction only** — unflattenable ⟹ indirect. The
  converse (flattenable ⟹ direct) was measured true on both real actors but is deliberately not
  asserted: an unmeasured shape would refuse a legitimate actor 40 minutes into a bootstrap.
- **The `browser-actor-child-worker-containment` chromium suite was not run**, exactly as C4 §6 left
  it, and it is still the first thing to run after the republish.
- **`🔒️dependencies.json` is stale for jco** (§3b) and was left so on purpose.
- **Rule 26 and the hub binary.** The bootstrap builds its own `os-hub`
  (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:12631`). That is unavoidable here and not a rule-26 breach by
  hand: the Rust twin of the codegen policy moved to `…-1.34.0-…`, so HS1's copied binary and the
  coordinator's both **refuse this catalog** — the same reason TC1 §4 gave for not copying a binary
  when the actor vocabulary changes. **The coordinator must rebuild their `os-hub` to serve it.**
- **The concurrent peer in `🌐️wasi-activation/🟦️.ts`.** A peer added `testPreview2GuestLogVendoring`
  and their own `checkWrite()` repairs to that file while this slice was in it. Their work was left
  exactly as found, including a `[DEBUG]` line at `:47` that is theirs to remove. The two remaining
  TypeScript errors in that file (`:17`, `:18`, `artifactBase` is `string | undefined`) are theirs.

## 7. Files changed

Product code:

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/package.json` | `@bytecodealliance/jco` `^1.7.0` → `^1.34.0` |
| `bun.lock` | jco 1.27.0 → 1.34.0, jco-transpile 0.6.1 → 0.14.0, preview2-shim 0.20.1 → 0.25.0 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` | version pins (`:17`, `:44`, `:51`, `:59`, `:157-159`, `:227`, `:286`, `:311`, `:347`); `suspendingBinding()` and `coreImportsExpression()` in `validateWasiSuspension`; **new** `validateAsyncTaskReturnLift()` (`:425`) called from `closedBrowserActorBundleFromRuntime` (`:464`); exported and added to `BrowserBundleTestDependencies` |
| `…/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts`, `…/🦀️.rs`, `…/🔣️.json`, `…/🧫️fixtures/🔣️.json` | codegen policy `…-1.27.0-…` → `…-1.34.0-…` (the Rust twin is what the hub binary enforces) |
| `…/📇️directory/🧬️schema/🔣️.json` | the same constant, 3 places |
| `…/🌐️browser-bundle/🧬️schema/🔣️.json` | `jco` / `version` consts → `1.34.0` |

Laws / fixtures:

| file | change |
|---|---|
| `…/🌐️browser-bundle/🧫️fixtures/🪝️async-task-return/🔣️.json` | **new** — 1 admitted source (3 bindings) + 7 hostile rows |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts` | **new** `testAsyncTaskReturnLift`; `compiler.inputs.length` 8 → 11 in two laws |
| `…/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts` | both preview2-shim oracles now take a `checkWrite()` permit before writing |
| `…/🌐️browser-bundle/🧫️fixtures/🔏️codegen-policy/🔣️.json`, `📦️codegen-manifest/🔣️.json`, `🌊️actor-import/🔣️.json`, `🧊️actor-factory/🔣️.json` | pinned tool versions / policy constant |
| `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` | policy constant |
| `🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json`, `…/👥️two-package/🔣️.json` | policy constant |

Ticket-owned: `🐍️jc1-transpile-probe.mjs`, `🐍️jc1-poll-lift-probe.ts`, `🐍️jc1-compiler-closure.ts`,
`🐍️jc1-task-return-law-check.ts`, `📜️jc1-hub-boot.sh`, this report, and the `🗑️generated/jc1-*`
captures.

**No peer's file, capture or `🗑️generated` entry was reverted or removed.**
