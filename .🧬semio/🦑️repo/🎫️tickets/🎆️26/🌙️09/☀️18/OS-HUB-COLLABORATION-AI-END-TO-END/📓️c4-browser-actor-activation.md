# C4 — the browser-actor activation rejection, named

Slice C4 (session 6, 2026-09-21). Owns the LAST blocker of outcome 3 named by C3 §3.2/§3.3:
the document's browser-actor child rejects its activation invocation, anonymously.

Inherited state, re-measured at slice start:

| what | value |
|---|---|
| hub | `http://127.0.0.1:7611` pid **48489**, `/readyz` `status: ready`, `openPlan: true`, runId `d8a8fe0d50f30762a4369546fa348a84`, `mcpWorkspace: false` |
| `s` serve | 6190 (vite pid 42307) — untouched all slice |
| `gis2d` serve | 6191 (vite pid 66294) — untouched, HTTP 200, picks up every TS edit below |
| space / document | `01a0c00f-4f3c-7834-a7e6-2ccf9de925db` / `artifact-2fb248125b8b2b4d56de25933d30ed21` |
| humans | `user1@semio.dev`, `user2@semio.dev` (both author members since C3 §3.1) |
| catalog generation | `94a9788a14060725869ab35b12b491b1d884d487bca3a1535ece346a2b757a71`, gis `closed-actor.mjs` **63 651 023 B** |
| disk | 27 GiB free on `/System/Volumes/Data` |

Nothing was started, killed, rebuilt or restarted by this slice. No cargo ran. No hub, no serve.

## 0. Headline

**The failure now speaks, and what it says is not a semio defect.** Two real defects were found on
the way and one of them is fixed; the third and last one is a bug in **jco 1.27.0's async
`task.return` lifting**, located to the line, in the catalog's own generated actor.

```
browser actor child: invocation rejected: invoke reactor/poll:
  TypeError: undefined is not iterable (cannot read property Symbol(Symbol.iterator))
  at _liftFlatVariantInner@3673:11 < _liftFlatResultInner@3856:14 < taskReturn@2751:33 < fn@8020:21
```

read off the live shell's own fault surface (`🗑️generated/c4-reason-run5.txt`), on the DOM, with two
signed-in humans' document sockets open on hub 7611. C3 spent a slice unable to read this sentence.

## 1. Make the failure speak (C3 §3.3.1 + §3.3.2) — LANDED

C3's finding was exact: `🧵️child/👷️worker/🟦️.ts:73` was `catch { if (phase === "active") send({ kind:
"rejected", sequence }); }` — the guest's error was discarded whole, and the child is a NESTED worker
whose console never reaches the page, so no log line could ever substitute. The reason had to travel
in the frame.

Schema first, in `🧵️child/🧬️schema/🟦️.ts`:

```ts
BROWSER_ACTOR_CHILD_REJECTION_LIMITS = { pathBytes: 128, classBytes: 64, messageBytes: 512, frameBytes: 256, frames: 4 }
BROWSER_ACTOR_CHILD_REJECTION_PHASES = ["load", "invoke"]
type BrowserActorChildRejectionV1 = { phase; path; errorClass; message; frame }
childRejectionReason(phase, path, error)   // constructor, hostile-getter safe, byte-bounded
childRejectionFrame(stack)                 // "<fn>@<line>:<col>" × up to 4, joined by "<"
isChildRejectionReason(value)              // exact-record admission off the wire
childRejectionText(reason)                 // the one line every owner above reports
boundChildText(value, limit)               // exact UTF-8 truncation
```

`frame` deliberately carries **no** module URL, blob id or origin — only `function@line:col`, which is
enough to locate a fault inside the one bundle whose sha256 the owner already verified. That property
is asserted by law (`JSON.stringify(reason)` contains neither `blob:` nor the origin).

The whole path, end to end:

| hop | file | what changed |
|---|---|---|
| guest → child | `🧵️child/👷️worker/🟦️.ts:73` | `catch (error) { … send({ kind: "rejected", sequence, reason: childRejectionReason("invoke", value.path, error) }) }` |
| guest activation → child | same, `:60` | the `load` catch now sends `fault` with `childRejectionReason("load", ["activate"], error)` instead of an anonymous `fault()` |
| child handlers | same, `:19`, `:87` | `fault(reason?)`; `port.onmessageerror` and the `receive` catch no longer pass their own argument in as a reason |
| child → store worker | `🧵️child/🟦️.ts:151` | `exact("sequence", "reason") && isChildRejectionReason(value.reason)`; the thrown Error is `"browser actor child: invocation rejected: " + childRejectionText(reason)`; a new `fault` branch closes with the named reason instead of `"protocol violation"`; a `rejection` getter retains the last one |
| store worker → page | `🏪️store/👷️worker/🟦️.ts:1217`, `:2604` | `emitExecutionTargetStatus(…, diagnostic?)`, bounded by the new `EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES = 1024`; the activation catch passes `error.message` |
| wire type | `💻️os/🟦️.ts:1266` | `execution-target-status` gains `diagnostic?: string`, with the doc comment saying exactly why it is the one exception to the code-only rule |
| page → DOM | `🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx:143` | the `role="alert"` section gains `data-semio-execution-target-diagnostic`. The **localized text is unchanged** — the privacy law on the rendered payload still holds; the diagnostic is an attribute an operator or a probe reads |

**Laws** (vitest, in-source): new file
`🧵️child/🧬️schema/🧪️tests/🧪️browser-actor-child-rejection-carries-a-bounded-typed-reason/🟦️.ts`, wired
by an `import.meta.vitest` block at the end of the schema module and registered in
`💻️os/🧪️tests/🎚️config/🟦️.ts` (`includeSource` + `coverage.include` — the config's own comment forbids
listing it in `include` as well). It pins the phase vocabulary, the exact constructed record, the
text rendering, frozenness, the URL-free frame extraction (4 shapes), empty-message and
non-Error inputs, hostile throwing `name`/`message` getters, every byte bound with its `...` suffix,
and **11 hostile rows** the validator must refuse (wrong phase, extra key, missing key, each bound + 1,
a non-string field, a getter instead of a data property).

```
npx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" -t "browser actor child rejection carries a bounded typed reason"
Test Files  1 passed | 5 skipped (6)      Tests  1 passed | 359 skipped (360)
```

No `[DEBUG]` line, no temporary instrumentation, nothing to remove: every hop above is product code
that stays. `npx tsc -p 🧰️framework/🛍️products/💻️os/tsconfig.json --noEmit` → **0 errors in any file
this slice touched** (49 remaining errors are peers', in renderer tests and `ShellHost/🟦️.tsx:5121`,
all pre-existing).

## 2. Defect 1 — the load deadline was flat, the bundle is 63.6 MB — FIXED

The moment the failure could speak, the first live run named a different failure than C3's:

```
c4r1  browser actor child: deadline
c4r2  browser actor child: deadline loading 5000ms bundle 63651023B
```

(`🗑️generated/c4-reason-run1.txt`, `c4-reason-run2.txt`; the second after
`🧵️child/🟦️.ts:122` `deadline()` was made to name its phase, bound and admitted byte length —
`close("deadline")` had been anonymous too.)

`BROWSER_ACTOR_CHILD_LIMITS.loadMs` was a flat **5 000 ms** covering: sha256 over the bundle, the
structured-clone transfer, `URL.createObjectURL`, `import()` of a 63 651 023-byte module, and the
guest's `module.activate` — which instantiates a 47 MB wasm component. C3 hit this too and read it as
noise (their §3.2.1 raised all three limits at once to `120 000/120 000/60 000`, saw the symptom
change, and restored the constant without separating the two causes).

Fixed at the root rather than by raising a number:
`🧵️child/🧬️schema/🟦️.ts` gains `loadBytesPerMs: 2048` and

```ts
childLoadDeadlineMs(bundleByteLength) = LIMITS.loadMs + ceil(bundleByteLength / LIMITS.loadBytesPerMs)
```

— `loadMs` is the fixed overhead, `loadBytesPerMs` the admitted throughput floor the slowest supported
engine must beat. `🧵️child/🟦️.ts:85` calls it with `this.admission.bundleByteLength`, which the owner
has already bounded to `actorBytes` at admission, so the budget can never be caller-inflated. For this
actor the budget becomes 5 000 + 31 080 = **36 080 ms**; for the law suite's one-line fixture modules
it is still ~5 000 ms, so `tight-loop-load` keeps its meaning.

**Measured effect:** run `c4r3` got past load for the first time and reached the invocation — the
symptom C3 could only produce by disabling all three deadlines at once now happens with real,
principled budgets.

## 3. Defect 2 — `describe` works, `poll` does not: jco 1.27.0's async `task.return` — NOT FIXED

### 3.1 What the live system says

`🗑️generated/c4-reason-run3.txt` → `c4-reason-run5.txt`, each a full sign-in + attach of `user1` on
serve 6191 against hub 7611, reading the shell's own DOM:

```
run3  invoke reactor/poll: TypeError: undefined is not iterable (cannot read property Symbol(Symbol.iterator))
run4  … at _liftFlatVariantInner@3673:11
run5  … at _liftFlatVariantInner@3673:11<_liftFlatResultInner@3856:14<taskReturn@2751:33<fn@8020:21
```

Every line number is inside the catalog's own
`…/generations/94a9788a…/packages/gis/browser/closed-actor.mjs` (63 651 023 B, the file the client
fetched and whose sha256 the lease verified), which makes them directly readable on disk.

### 3.2 The line

`closed-actor.mjs:3667-3673`, inside jco's `_liftFlatVariantInner`:

```js
const [tag, liftFn, caseSize32, caseAlign32, caseFlatCount] = caseMetas[caseIdx];
```

`caseMetas[caseIdx]` is `undefined`. `caseIdx` came from `_liftFlatU8(ctx)` two lines up, and
`_liftFlatU8` (`:3402`) under `ctx.useDirectParams` returns **`ctx.params[0]`**.

The caller chain is `fn@8020` = `reactor100Poll(...)` → `taskReturn@2751` → the `result` lift. So this
is the **guest returning** `semio:framework/reactor@1.0.0#poll` through the async ABI's `task.return`.

### 3.3 Why it is jco and not semio, corroborated four ways

1. **The same actor instance answers `describe` correctly seconds earlier.**
   `DocumentExecutionTargetLease.activateBrowserActor` (`🏪️store/👷️worker/🟦️.ts:1141`) does
   `child.invoke(["describe","describe"], [])`, verifies the bytes against the plan's descriptor
   (`verifyBrowserActorDescribeV1`) and only then is `openGuest`'s `poll` reached at all. Reaching the
   `poll` rejection *proves* instantiation, the closed host/WASI shims (including TC1's new
   `wall-clock`/`insecure-seed`), the digest chain and one full async round trip are sound.
2. **`describe` is async-lifted too** (`closed-actor.mjs:8746` `async: true`, `:8752` `isAsync: true`),
   so "async exports are broken" is too coarse. The difference is the *size* of the returned value.
3. **The metas say so.** Every `task.return` trampoline in this component is bound with
   `useDirectParams: true`. Their outer result metas:

   | trampoline | line | `variantSize32` | `variantFlatCount` | works |
   |---|---|---|---|---|
   | 23 | 9484 | 16 | 4 | — |
   | 25 | 9593 | 20 | 5 | — |
   | **27 = `reactor/poll`** | **9801** | **344** | **`null`** | **no** |

   `variantFlatCount: null` is jco saying *"I could not flatten this"* — a 344-byte record is far past
   the Canonical ABI's 16-flat parameter limit, so `task.return` must pass **one pointer** into the
   return area. The guest does exactly that; jco then lifts with `useDirectParams: true` and reads that
   **pointer** as a u8 discriminant, so `caseMetas[<linear memory offset>]` is `undefined`.
4. **jco already knows** — its guard is simply in the wrong order. `closed-actor.mjs:3697-3701`:
   ```js
   if (origUseParams) {
     if (variantFlatCount === undefined || variantFlatCount === null) {
       throw new Error("cannot lift variant with unknown flat count");
   ```
   That is 24 lines **below** the destructuring that throws first, so the condition jco anticipated can
   never be reported; it always presents as an opaque `TypeError`.

**Named:** jco 1.27.0 binds an async export's `task.return` trampoline with `useDirectParams: true`
even when the result cannot be flattened (`variantFlatCount: null`) and must be returned indirectly;
the return-area pointer is then lifted as the variant discriminant. Every export in this component
whose result is 16-20 bytes works; the one whose result is 344 bytes — `reactor.poll`, the single
entry point through which every document mutation travels — does not. **This is why not one `Commands`
frame has ever reached the hub.**

### 3.4 Why it is not fixed here

The fix is upstream (or a patched `js-component-bindgen` at transpile time in
`closedBrowserComponentFactory`) and cannot be tested without re-transpiling and **re-materialising the
trusted catalog** — TC1 measured 37 min for that chain, under the fleet wasm mutex, plus a hub
republish, and the pre-publication gis cold-map law in front of it. That does not fit this slice's
runway, and a jco patch landed unverified would be worse than a precise hand-off. Two candidate
routes for the next owner, in order of cost:

1. **Patch the lift, not the generator.** `taskReturn` (`:2727`) already carries the indirect branch
   (`liftCtx.storagePtr = params[0]; liftCtx.storageLen = params[1]`). Selecting it whenever the outer
   lift meta's `variantFlatCount` is `null` is a two-line change in jco's emitted runtime; the
   browser-bundle already post-processes and re-parses the transpiled output
   (`🌐️browser-bundle/📜️script.ts:490-515`), so it has the seam to do it and to pin it with a law.
2. **Upstream `@bytecodealliance/jco`** — 1.27.0 is what `node_modules` carries and what TC1's
   `derive` stage used.

Either way the verification is cheap once the actor is rebuilt: re-run `🐍️c4-actor-reason-probe.mjs`
and read the same DOM attribute. That is what §1 bought.

## 4. Two users, live, per step

| # | step | verdict | witness |
|---|---|---|---|
| — | the failure is named, not anonymous | **OBSERVED** | `c4-reason-run5.txt`: full typed reason on `[data-semio-execution-target-diagnostic]`, from a nested worker, in a real browser, on the live hub |
| — | load budget scales with the bundle | **OBSERVED** | `c4r2` `deadline loading 5000ms bundle 63651023B` → `c4r3` reaches the invocation |
| 2 | live edit A→B | **not run** | blocked on §3; the browser actor still never activates, so no mutation can reach `relayMutationsToHub` |
| 3 | live edit B→A | **not run** | same |
| 4 | per-user undo with a crossing edit | **not run** | same — C3 §2 step 4 records the non-crossing form honestly and this slice adds nothing to it |
| 5 | presence symmetric and stable (C3 §3.4) | **not run** | C3 already suspected it is downstream of §3.2 ("a document whose browser actor never activated may never publish a beat"); that remains the live hypothesis and this slice did not test it |
| 6 | connection loss + catch-up | **not run** | vacuous while nothing crosses |
| 7 | two-writer convergence, 10 + 10 edits | **not run** | vacuous while nothing crosses |

Steps 2-7 are exactly as blocked as C3 left them. What changed is that the blocker is no longer a
guess: it is a named line in a named file with a named upstream owner.

## 5. Permanent wiring

**Not added**, for C3 §4's reason unchanged — the live-edit half of the scenario still fails, and an nx
target would pin a red path as a gate. The new probe is permanent and parameterised:

```sh
bun "$T/🐍️c4-actor-reason-probe.mjs" [user1|user2] [shellUrl] [hubHostPort] [spaceId] [documentId]
# env: C4_TAG=<capture prefix>  C4_WAIT_MS=<status poll budget, default 90000>
```

## 6. Honest gaps

- **The root cause is named and located, not fixed.** Nothing in §3 is inferred from source alone:
  every line number came out of a live browser, and every meta was read off the exact bundle the client
  fetched. But no jco patch was written and no catalog was re-materialised, so **no claim is made that
  two users can edit live.**
- **Steps 2-7 of C3's scenario were not run at all**, for the same reason C3 did not run 6/7: they
  would produce ticks with no content.
- **The presence asymmetry (C3 §3.4) was not touched.** It is still un-root-caused.
- **`user2` was not driven in a second context this slice.** Every capture is a single `user1` context;
  the two-socket property is C3's measurement, re-asserted from the hub's state, not re-measured here.
  The blocker is per-client and reproduces in one context, so a second context would have cost ~2 min
  per run and added nothing.
- **The new `fault`-with-reason branch in `🧵️child/🟦️.ts` was exercised only by the vitest laws and by
  the live `load` path, not by the chromium containment suite**
  (`browser-actor-child-worker-containment`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5560`), which was not
  run: it launches vite + playwright and belongs to a hub verb, and this slice had no budget for it.
  Its existing laws should still hold — `wasi-exit`, `activation-failure` and the wire rows all assert
  `phase === "closed"` and a rejected promise, which the named-reason close still produces — but that
  is reasoned, **not measured**, and it is the first thing the next owner should run.
- **The scaled load budget's rate (`2048 B/ms`) is a floor chosen with headroom, not a measurement.**
  The bundle now loads inside it; the actual elapsed load time was never isolated.
- **Nothing was rebuilt.** Every experiment is TypeScript picked up by the running vite serve 6191
  (pid 66294, untouched) or plain reads of the published catalog on disk.

## 7. Files changed

Product code:

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` | rejection vocabulary, `childRejectionReason` / `childRejectionFrame` / `isChildRejectionReason` / `childRejectionText` / `boundChildText`; `loadBytesPerMs` + `childLoadDeadlineMs`; in-source vitest block |
| `…/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts` | `rejected` and `fault` frames carry the typed reason; `fault(reason?)` and its three call sites |
| `…/🌐️browser-bundle/🧵️child/🟦️.ts` | validated `reason` on `rejected`, new `fault` branch, `rejection` getter, self-naming `deadline()`, scaled load budget, re-exports |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | `EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES`, `emitExecutionTargetStatus(…, diagnostic?)`, the activation catch names its error |
| `🧰️framework/🛍️products/💻️os/🟦️.ts` | `execution-target-status.diagnostic?: string` + its contract comment |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx` | `data-semio-execution-target-diagnostic` on the alert section |

Laws / config:

| file | change |
|---|---|
| `…/🧵️child/🧬️schema/🧪️tests/🧪️browser-actor-child-rejection-carries-a-bounded-typed-reason/🟦️.ts` | **new** — the rejection-record law |
| `🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts` | the child schema module registered in `includeSource` + `coverage.include` |

Ticket-owned: `🐍️c4-actor-reason-probe.mjs` (new, permanent, parameterised); captures
`🗑️generated/c4-reason-run1.txt` … `c4-reason-run5.txt`, `c4*-actor-reason-user1.json`,
`c4*-actor-reason-user1-console.txt`, `c4*-actor-reason-user1.png`.

No peer's file, capture or `🗑️generated` entry was touched, and nothing of C3's was reverted —
`BROWSER_ACTOR_CHILD_LIMITS.loadMs/invokeMs/bootMs` are still C3's restored values; only the new
`loadBytesPerMs` term was added beside them.

## 8. State at hand-off

| what | pid | note |
|---|---|---|
| hub 7611, data root `gm1-boot` | `os-hub-7611` **48489** | untouched, `status: ready` |
| `s` serve 6190 | vite **42307** | untouched |
| `gis2d` serve 6191 | vite **66294** | untouched; it serves every edit above — no rebuild needed to re-verify |

**For the next owner, in order:** (1) run `browser-actor-child-worker-containment` once to re-verify the
widened child protocol in chromium; (2) take §3.4 route 1 — select jco's existing indirect `task.return`
branch when the lift meta's `variantFlatCount` is `null`, pin it with a law in the browser-bundle's
post-processing, re-derive the actor and re-materialise the catalog; (3) re-run
`🐍️c4-actor-reason-probe.mjs` and read the same DOM attribute; (4) then C3's
`🐍️c3-collab-scenario.mjs` with `C3_ONLY=2,3,4,6,7,8`.
