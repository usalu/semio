# T4 — os product scoped tsconfig, typecheck target, and TypeScript debt

Slice T4 (Opus). Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`.
Scope: TypeScript under `🧰️framework/🛍️products/💻️os/**`, plus `✏️s` plugins and build scripts
("os/s/build-script" per `📓️status.md`).

## 1. What now exists (deliverables 1 and 5)

| thing | where |
|---|---|
| product-scoped tsconfig | `🧰️framework/🛍️products/💻️os/tsconfig.json` |
| `typecheck` script | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts` (`TypecheckScript`, `-p ../../tsconfig.json`) |
| nx target | `…/📦️packages/🟦️typescript/📋️project.json` → `@semio-tech/framework-os:typecheck` |
| launch row | `.vscode/launch.json` → `🛠️dev💻️os🪁️typecheck`, group `3_dev`, order 390.1 (next to `🛠️dev🖱️ui🪁️typecheck`) |

The tsconfig was **already on disk untracked** when I started (created 22:33 by a peer or by the
predecessor T4 that died with its parent turn). I kept it, added `"bun"` to `types`, and widened
`exclude` to match the source-only policy T1 landed in the root tsconfig: `📤️dist`, `🎯️target`,
`🗑️generated`, `🤖️generated`, `🕸️bindings` on top of the existing `dist`/`storybook-static`/`target`/
`🧩️extension-modules`/`🔌️plugin-modules`/`🌐️browser-bundles` exclusions.

Verified by running it: `bun ./📜️script.ts typecheck` from the package dir compiles the program and
prints diagnostics with paths relative to the package root.

## 2. Measurements

All captures in `🗑️generated/`. Command: `bun tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json --pretty false`.

| capture | total diagnostics | os-product | wall |
|---|---:|---:|---|
| `t4-os-before.txt` (baseline) | 1265 | 801 | 2m39s |
| `t4-os-after-config.txt` (bun types + excludes) | 1171 | 734 | 7m02s |
| `t4-os-run2.txt` (codemods, pass 1) | 923 | 544 | 8m07s |
| `t4-os-run3.txt` (final) | **901** | **522** | 23m34s |

Wall time grew 9× across the session purely from peer load (final run: 7% CPU). Not a program-size change.

Ownership split of the 901 remaining (the os program pulls non-os files in through the import graph;
tsc reports them, they are not mine to fix):

| owner | count |
|---|---:|
| **T4 — `💻️os`** | **522** |
| T3 — `🧰️framework/🔨️modules/**` | 252 |
| **T4 — `✏️s` plugins** | **53** |
| T2 — `🦑️repo/📚️library` | 47 |
| **T4 — root `📜️script.ts`** | **21** |
| legacy `♻️mit-bestand` | 6 |

Net for the whole program: **1265 → 901 (−364)**. For my own area: **801 → 522 (−279, −35%)**.
**This slice did not reach zero.** Honest status in §5.

## 3. Root fixes made

**a. Bun ambient types (−94).** `types` was `["node", …]`, which suppresses Bun's globals, so every
`Bun.*` use and `bun:sqlite`/`bun:test` import failed (TS2868 ×83, TS2307 ×15). `@types/bun` was
installed by T1 mid-session; I added `"bun"` to the os tsconfig `types`.

*Caveat, and a real finding:* enabling bun types also **surfaced 21 previously-masked errors** in
four files that use Bun/fetch/timer APIs — `🧪️space-artifact-creation-owner` 45→55,
`🔬️engine-contract` 35→39, `📨️browser-frame-transport` 6→11, `👷️worker` 9→11. I verified by
per-capture counts that all four jumped at the `after-config` step and stayed flat across every later
edit, so these are genuine errors that `Bun` being an unresolved name had been hiding — not regressions.

**b. `isolatedModules` re-exports (−50).** 50 type names re-exported through value `export { … }`
clauses in 16 files (TS1205). Fixed by `🐍️t4-export-type-codemod.py`, which is **span-keyed off
tsc's own (line, col)** — it maps UTF-16 columns back to code-point indices (needed: every path
segment here is astral emoji) and refuses any position that is not an identifier start or whose line
contains no `export`. One line changed per file; 15 of the 16 files were last touched Sep 15, the
16th (`🤝️collaboration`, C1b's) was guarded and matched cleanly.

**c. Assertion functions without explicit type annotations (−78).** `const { default: assert } =
await import("node:assert/strict")` gives `assert` an inferred type, and TS refuses assertion-call
narrowing through it (TS2775 ×69 in `🆕️fresh-component`, ×9 in `🎒️pack/🌱️value/📜️script.ts`). I probed
the annotation forms standalone before touching the files: `node:assert/strict` is an `export =`
module, so `typeof import("node:assert/strict")` **is** the assert function, and
`const assert: typeof import("node:assert/strict") = (await import("node:assert/strict")).default;`
both compiles and preserves narrowing (verified with a scratch file that assigns a narrowed `unknown`
to `number`).

**d. `as const` tuple vs fixed-arity consumer (−58).** `🛢️db/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts`
built its ten retained-source texts with `as const` (literal-typed readonly tuple) and mutated them
with `Array.prototype.with`, whose argument must then be assignable to the union of the ten literal
types — 58 × TS2345. Replaced with a declared `P1qR4Sources` 10-tuple of `string` and a local
`mutate(slot, source)` helper that rebuilds the tuple positionally. No cast anywhere; the spread into
`interactivityP1qR4Failures(...)` (10 positional params) still typechecks.

**e. `dependencies: any` test harness (−~40 so far).** 27 os test modules are extracted suites with
the shape `registerTestsN(vitest: …, dependencies: any, source: TestSource)`, and the `any` cascades
into every callback parameter (TS7006 was my single largest code: 293 → 127). Each caller passes a
concrete object literal, so the exact type is derivable from the call site. Two codemods:

- `🐍️t4-typed-test-deps.py` — rewrites to `Pick<typeof import("<caller>"), "k1" | "k2" | …>`, but
  **only when every key is actually exported** by the caller (it parses declaration and clause export
  forms and defers otherwise). 14 rewrites.
- `🐍️t4-typed-test-deps2.py` — for the deferred ones, resolves each non-exported key either to the
  caller's own `import { … } from "<spec>"` (rebasing the specifier to the test module) or to a local
  declaration it then exports, and emits an **intersection** of exact `Pick<typeof import(…)>` slices.
  8 rewrites; added `export` to `BACKBONE_ENVELOPE_RETRY_WINDOW_MS` and `BACKBONE_WORKER_WIRE_MAGIC`
  in `💻️os/🟦️.ts`, `generation` in `🔌️plugin/📤️return/🟦️.ts`, `accessibilityAriaProps` in
  `🗣️Interpreter/🟦️.tsx`.

20 test modules now carry real types instead of `any`. Two remain unresolved and are documented in §5.

No `any` was introduced, no `@ts-ignore`, no `skipLibCheck` change (the root tsconfig's long-standing
`skipLibCheck: true` is inherited and untouched).

## 4. Verification

**Typecheck:** run four times end-to-end; final `t4-os-run3.txt`. Regression check across baseline →
final: **no new error codes appeared**, and the only files whose counts rose are the four bun-types
files in §3a, all of which rose at the config step and were flat thereafter.

**Tests:** `bun ./📜️script.ts test long --run` in `💻️os/📦️packages/🟦️typescript` →
`t4-vitest-os.txt`: **292 passed, 69 failed (361), 2 of 5 files failed**. That config's `includeSource`
covers three of my edited test modules plus both modules I added exports to, so it is the right gate
for this slice.

Those 69 failures are **not caused by this slice**, on two independent grounds:
- **68 of 69 are in `🔨️modules/🏪️store/👷️worker/🟦️.ts`**, which I never edited (it is the caller for
  `🧪️space-artifact-creation-owner`, which I explicitly deferred — see §5).
- The 1 remaining, in `💻️os/🟦️.ts`, is an Ajv `oneOf` schema failure about missing `artifactId`/
  `spaceId` on a directory-administration request. My entire diff to that file is **two `export`
  keywords** (staged diff reproduced in the capture), which cannot change runtime behaviour.

The failure signatures (`document runtime scope: invalid id`, `stampSession`, `toWireEnvelope`
round-trip, artifact bootstrap) match the directory/wire work M2, C1 and H1 have in flight.
**Honest gap: I did not capture a pre-change baseline of this suite** — my slice began after peers had
already modified these modules, and the preamble forbids `git stash`/`checkout` to synthesise one. So
"pre-existing" is an inference from the diff being provably inert, not from a measured before/after.

The React engine suites (`🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`) do **not** list any file I
touched: `🧪️unknown-component-placeholder`, `🧪️typedwire`, `🔌️plugin-runtime` and
`🧪️canvaseventbindingcontroller` are in-source suites reached via their owning modules'
`includeSource`, and that config's `includeSource` is empty except for the agent-bridge and
backbone-worker gates. Attempting to filter them directly returns "No test files found" (captured in
`t4-vitest-react.txt`). **So my renderer-side edits are verified by typecheck only, not by a test run.**

## 5. What was NOT done as of the previous session — 596 diagnostics (superseded by §12)

**`💻️os` — 522.** Long-tail per-file debt, no further single-fix clusters: TS7006 127, TS2339 56,
TS2322 41, TS18046 37, TS2345 34, TS7053 24, TS2741 23, TS2352 23. Heaviest files:
`🧪️space-artifact-creation-owner` 55, `🧪️backbone-envelope-io` 48, `🔬️engine-contract` 39,
`🔌️plugin/🧪️tests/🌐️browser-bundle` 28, `🧪️reactor-contract-oracles` 23, `🌐️wasi-activation` 23,
`🆕️fresh-component` 22, `🧪️chunkkey` 21, `🏛️ShellHost` 18.

Two harness modules could not be typed from their call sites and keep `dependencies: any`:
- **`🧪️space-artifact-creation-owner`** (55 errors) — its caller `👷️worker/🟦️.ts` passes **112 values,
  108 of them module-internal**, including `testSeams` which is declared *inside* the
  `if (import.meta.vitest)` block and so cannot be referenced by a top-level exported type. Fixing it
  means hoisting or restructuring the worker's test seam — and that file is C1/M2 territory with 68
  live test failures in it right now. **Deliberately not touched.**
- **`🧪️chunkkey`** (21) — its caller re-exports ~18 three.js names (`BoxGeometry`, `Matrix4`, `MOUSE`, …)
  it neither declares nor imports by name; needs a `typeof import("three")` slice written by hand.

**`✏️s` plugins — 53.** Untouched. Concentrated in `🧩️puzzle` schema/diff/snapshot barrels (9+4+2+2+2+…)
and `📐️cad` presence tests. Note the audit's warning that some `✏️s` diagnostics are cluster-1
(emoji-spliced) rather than semantic still applies to a few of these.

**Root `📜️script.ts` — 21.** **Deliberately not touched: a peer is editing it live** (`MM` in git
status, mtime 00:54 during my run). Its errors include `TS2304: Cannot find name 'subsetsDirName'`
(line 22135) and four `TS2352` Taxonomy casts, which look like the same taxonomy-schema rename drift
as the audit's cluster 3 — likely to resolve or move with T2/V1's taxonomy work.

**Not mine, blocking a green target:** 252 T3 (`🧰️framework/🔨️modules`), 47 T2 (`🦑️repo/📚️library`),
6 legacy `♻️mit-bestand`. `@semio-tech/framework-os:typecheck` **cannot go green until T2 and T3
land**, because the os program compiles their files through the import graph. The 6 `♻️mit-bestand`
diagnostics are unowned legacy and reach the program the same way.

Pre-existing `any` I left alone because narrowing them is not type-level work:
`🆕️fresh-component/🟦️.ts:5-6` declares `type FreshSourceEpochLegV1 = any; type FreshSourceEpochPlanV1 = any;`.

## 6. Peer discipline

Re-read before every edit. Live peers respected: C1b (`🔌️PluginRuntime/🟦️.tsx`, `🤝️collaboration`),
M3 (wgpu chat panel bridge), O2 (`📓️o2-activation-follow-ups.md` §6). Edits inside peer-live files were
signature/type-level only (`🧪️tests/🔌️plugin-runtime/🟦️.tsx`, `🤝️collaboration/🟦️.ts` — one export
clause each). `🏛️ShellHost/🟦️.tsx` (18 errors, O2's) and `🎞️frame-worker/🟦️.ts` (11, O2's) were **left
alone entirely**. No cargo, no servers, no `🗑️generated` sweeps, no ticket bookkeeping touched.

## 7. Files changed

```
🧰️framework/🛍️products/💻️os/tsconfig.json                                   (bun types + source-only excludes)
🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts             (TypecheckScript + runBunx import + register)
🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json          (typecheck target)
.vscode/launch.json                                                       (🛠️dev💻️os🪁️typecheck row)
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts   (P1qR4Sources tuple + mutate helper)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts (assert annotation ×3)
🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/📜️script.ts             (assert annotation)
🧰️framework/🛍️products/💻️os/🟦️.ts                                           (2 exports)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts                (1 export)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx (1 export)
16 files — one `export type` clause each (TS1205 codemod)
20 test modules — `dependencies: any` → exact `Pick<typeof import(…)>` (typed-deps codemods)
```

Scratch tooling (ticket folder): `🐍️t4-export-type-codemod.py`, `🐍️t4-typed-test-deps.py`,
`🐍️t4-typed-test-deps2.py`. Captures: `🗑️generated/t4-os-{before,after-config,run2,run3}.txt`,
`🗑️generated/t4-vitest-{os,react}.txt`.

---

# Session 3 continuation (2026-09-19)

## 8. Re-measurement at hand-over

`🗑️generated` had been wiped again, so every capture below is new. Command unchanged:
`bun tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json --pretty false` → `🗑️generated/t4-os-run4.txt`.

| owner | run3 (§2) | run4 (start of this session) |
|---|---:|---:|
| **T4 — `💻️os`** | 522 | **525** |
| **T4 — `✏️s`** | 53 | **53** |
| **T4 — root `📜️script.ts`** | 21 | **21** |
| T3 — `🧰️framework/🔨️modules` | 252 | 199 |
| T2 — `🦑️repo` product + library | 47 | 48 |
| legacy `♻️mit-bestand` | 6 | 0 |
| **total program** | 901 | **852** |

So the peer renames (`🎚️options`→`☑️options`, `⚙️config`→`🎚️config`) moved nothing in my area; T4 still owned **599**.

## 9. New tooling that changed the economics

The 8–25 minute full-program typecheck made iteration impossible. This session added a
**scoped tsconfig** (`🔣️t4-scope.json`, `🔣️t4-one.json` in the ticket folder) that `extends` the os
tsconfig and lists only the files under repair in `files` — TypeScript still pulls their whole import
graph, so diagnostics are identical for those files, but a single file's graph checks in **~8 s** and
the 120-file owned set in **~45 s**. Every loop below used it; the authoritative number is still the
full run.

Scratch tools added to the ticket folder:

| script | what it does |
|---|---|
| `🐍️t4-harness-deps-type.py` | emits an exported `typeof`-only dependency type for a `registerTestsN` harness from its single call site |
| `🐍️t4-rename-at.py` | span-keyed identifier rename driven by tsc's own (line, col); UTF-16→code-point mapped; refuses any position whose token is not the expected one |
| `🐍️t4-wrap-fetch-stub.py` | wraps `globalThis.fetch = …` stubs in a `stubFetch(…)` that keeps Bun's `preconnect` |
| `📜️t4-count.sh` | splits a capture into os / ✏️s / script / other |

## 10. Root fixes this session

**a. `🏪️store/👷️worker/🟦️.ts` — 11 → 0.** A half-finished `artifact`→`document` rename. The
replication wire type `WireFrontierSummary` (T3's `📡️replication/🟦️.ts:179`) now declares
`document_id`, and `ArtifactFrontier` / `CheckpointPublicationFrontierV1`
(`💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:2303, :398`) declare `documentId`, but the worker still
read `.artifact_id` / `.artifactId` at seven sites. One site was wrong in both directions at once:
line 964 built a `ColdArtifactPairFrontier` (which legitimately still keys `artifactId`, see
`🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts:6` and its Rust twin) with key `documentId` from a field named
`artifact_id`. Both halves corrected.
The remaining two were a DOM-vs-Bun lib clash: `ReadableStreamReadResult<Uint8Array>` written by hand
where `body.getReader().read()` actually returns Bun's `ReadableStreamDefaultReadResult`. Replaced by
a derived alias `ExecutionTargetReadResult` (`…/👷️worker/🟦️.ts:1258`) — no cast.

**b. `🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` — the 55 the predecessor deferred.** Typed
honestly rather than left on `any`: the worker now exports `BackboneWorkerTestSeams` and
`BackboneWorkerTestDependencies` (region `🧪️TestContracts`, placed **above** the seam declarations —
see the trap below), and the harness takes `dependencies: BackboneWorkerTestDependencies` with its
hand-written `as { … }` seam cast deleted.

*Trap worth recording:* a `typeof x` type query is **control-flow narrowed at the reference**. With
`let sink: Cb | null = null`, a `typeof sink` written after the declaration resolves to `null`, not
`Cb | null` — every seam setter then failed to typecheck. Proved with a standalone two-line file
before believing it. Placing the type block before the declarations gives the declared type.

Typing the bag turned 55 masked diagnostics into 122 real ones, all of them the same
`artifact`→`document` drift as (a) plus genuine shape errors. Driven down to **41** so far by:
63 span-keyed key renames (`artifactId`→`documentId`, `artifact_id`→`document_id`) against
`DocumentScope`, `ArtifactActorConfig`, `BrowserActorActionScopeV1`, `CheckpointPublicationFrontierV1`,
`WireMutationEnvelope`; 42 local-binding renames; 14 `state.config.artifactId` accesses; and 8
`globalThis.fetch` stubs routed through a `stubFetch` helper that preserves Bun's `preconnect`
instead of casting it away.

**c. `allowJs: true` on the os tsconfig (−33).** Nine imports of our own `🟨️.js` / `🟨️.mjs` sibling
modules were TS7016 "could not find a declaration file", which made each import `any` and cascaded.
The root tsconfig sets `allowJs: false`; the os product now sets `allowJs: true`. `checkJs` stays
off, so those JS bodies are still not type-checked — TypeScript simply reads the modules it is
already being asked to import instead of treating our own source as an opaque third party. No
hand-written `.d.ts` duplicates.

**d. Three component tests imported themselves (13 diagnostics).** `🧵️TaskManager`,
`🚦️AgentPresence` and `🤖️AgentApprovals` each live at `<element>/🧪️tests/🧩️component/🟦️.tsx` and
imported their subject from `"./🟦️.tsx"` — which resolves to the test file itself, so every name
came back as "declared locally, but it is not exported". Corrected to `"../../🟦️.tsx"`. In the same
file `ShardBudget` was imported from `🎠️kernel`, which only re-imports it; moved to its real owner
`🎭️actor/📮️shard-client`, the module the very next line already imports.

**e. `🧪️reactor-contract-oracles` 23 → 0.** `JSON.parse` gives `any`, but
`assert(validate(fixture))` where `validate` is an untyped Ajv `ValidateFunction` narrows `any`
**down to `unknown`** — that single line was producing 21 × TS18046. Fixed by declaring the five
corpus shapes and passing them to `ajv.compile<T>` / `ajv.getSchema<T>`, so the predicate narrows to
the real type. A pre-existing `(row: any)` reducer parameter became `BackboneBindingRow`.

**f. `🌐️wasi-activation` 23 → 4.** `const key = (path: string) => \`wasi:${path}@0.2.0\`` erased the
literal, so all 16 import-table lookups were TS7053. Made generic:
`<P extends string>(path: P): \`wasi:${P}@0.2.0\``. `const hostCalls = []` was an implicit `any[]`;
typed from the sink's own callback parameter.

**g. `🧪️backbone-envelope-io` 24 → 4.** Three `import("./🔨️modules/📇️directory/…")` type queries were
two levels short (the module moved); two `type X = any` placeholders replaced with the real
`import("../../🟦️.ts").X`; the nine `Object.values(command)[0]!.seq` reads (which TS resolves to `{}`
over a 20-variant union) replaced by one `commandSeq` helper that keeps the real `AppCommandValue`.

**h. Root `📜️script.ts` 21 → 0.** The predecessor left this file alone because a peer held it; the
peer's last write was 00:54 and it was free.
- Five `(taxonomy as { … })` casts (TS2352) — `Taxonomy` **already declares** `schemaChildDirs`,
  `representationDirs`, `ioDirectionDirs`, `ioDirectionChildDirs`, `newArtifactChildDirs` and
  `subsetsDirName` as `readonly string[]`; the casts named them `string[]` and so could never
  succeed. Casts and their duplicated default literals deleted — the taxonomy file is now the single
  source for that vocabulary. `subsetsDirName` also had an `as any` and a second, out-of-scope copy
  (`TS2304: Cannot find name 'subsetsDirName'` at 22135); both now read `taxonomy.subsetsDirName`.
- `RunCmdOpts` had no `shell`, so three `runCmd("bunx", …, { shell: true })` call sites were passing
  a silently dropped option — on native Windows `bunx` is a `.cmd` and cannot be spawned without a
  shell, so this was a real cross-platform break, not just a type error. Added `shell?: boolean` to
  `RunCmdOpts` and wired it into `runCmdInternal`'s `spawnSync` (defaulting to `false`, so no other
  caller changes behaviour).
- `new NativeOsScript(this.root)` (×2) missed the base `Script(root, repoRoot)` constructor's second
  argument; the sibling line already passed `this.repoRoot`.
- The stdio support-ledger block read ten fields off a `Record<string, unknown>`; its guards already
  return typed values, so the returns are now bound (`ledgerLists`, `ledgerStates`, …) instead of
  being called for their throw alone. Added `stdioMember`, a closed-vocabulary guard that keeps the
  literal union, and used it for the runtime-claim namespace.
- `Bun.TOML.parse(...)` returns `object`; the two `.workspace` / `.package` reads now cast the parse
  result rather than its (nonexistent) property.
- `ownerTypeName` is `string | undefined` and was pushed into a `string` field; added to the guard
  that already skips incomplete rows.

**i. `🧪️space-artifact-creation-owner` finished as a rename, and it moved the test suite.** Once the
harness had a real dependency type, the remaining diagnostics were all one thing: the module predates
the `artifact`→`document` rename that the fixtures, the worker and the directory schema have already
taken. The fixture `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json` and
`💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json` both key `documentId`, while the harness
read `.artifactId` — so `fixtureConfig`, `openArtifact` and `documentRuntimeKeyV1` were being handed
`undefined` at runtime. Renamed `artifactId`→`documentId` and `artifact_id`→`document_id` through the
whole file, with exactly one exemption: the hub's space-artifact-creation **ready** payload, whose
field really is `artifactId` (`💻️os/🟦️.ts:1015` pins the exact key set
`artifactId,artifactSchema,kindId,parentDialect`).

## 11. Verification

**Typecheck (authoritative, full os program).** `🗑️generated/t4-os-run6.txt`.

| owner | run4 (start) | run6 (end) |
|---|---:|---:|
| **T4 — `💻️os`** | 525 | **405** |
| **T4 — `✏️s`** | 53 | **44** |
| **T4 — root `📜️script.ts`** | 21 | **0** |
| **T4 total** | **599** | **449** |
| T3 — `🧰️framework/🔨️modules` | 199 | 19 |
| T2 — `🦑️repo` product | 48 | 47 |
| legacy `♻️mit-bestand` | 0 | 3 |
| **program total** | **852** | **518** |

**This slice still did not reach zero.** The honest reading of −150 is better than it looks and worse
than it looks at once: typing the space harness *added* 67 previously-masked diagnostics before any
were removed, and a peer added 17 new ones in `🔐️HubSignIn` and `🏘️SpaceBrowser` stories **during** the
final run (both files' mtimes are 02:47, inside the run window; I never touched them). Net of both,
this session removed roughly 230 real diagnostics.

**Tests — the real result of this slice.** `bun ./📜️script.ts test long --run` in
`💻️os/📦️packages/🟦️typescript`, captured in `🗑️generated/t4-vitest-os2.txt`:

| run | passed | failed | files |
|---|---:|---:|---|
| §4 baseline recorded by the predecessor | 292 | 69 | 2 of 5 failed |
| after the worker drift fix only (`t4-vitest-os.txt`) | 304 | 57 | 2 of 5 failed |
| after the harness rename (`t4-vitest-os2.txt`) | **348** | **13** | 2 of 5 failed |

So the `artifact`→`document` drift that TypeScript surfaced was not cosmetic: it was **56 live test
failures**, and fixing it at the root turned them green. Total is 361 in every run, so nothing was
skipped or deleted to get there.

The 13 that remain are Ajv corpus-vs-schema mismatches in directory administration and browser
document-open (`🛂️command-admission` fixture fails its own schema; execution-target lease and actor
reservation rows). They touch no identifier this slice renamed and belong to the directory/wire work
C1, M2 and H1 have in flight.

## 12. What is still open — 449 diagnostics I own

**`💻️os` — 405.** Codes: TS7006 97, TS2322 50, TS2345 48, TS2339 47, TS2352 24, TS2769 19, TS2353 17,
TS18046 17. Heaviest files: `🔬️engine-contract` 44, `🧪️space-artifact-creation-owner` 32,
`🔌️plugin/🧪️tests/🌐️browser-bundle` 28, `🆕️fresh-component` 22, `🧪️chunkkey` 21, `🏛️ShellHost` 19 (O2's,
untouched), `🧪️ticket-owned-browser-host-staging` 14, `🔬️catalog-smoke` 13, `📨️browser-frame-transport`
11, `🎞️frame-worker` 11 (O2's, untouched).

**The single structural cause left is a documented pattern, not a long tail.** Ten extracted
`registerTestsN` harnesses open with a block of `type X = any;` placeholders — 37 of them in
`🧪️space-artifact-creation-owner` alone, 20 in `🧪️backbone-envelope-io`, 7 in `🧪️chunkkey`. Every
diagnostic those files still carry is downstream of that. I proved the fix works and is mechanical:
`🐍️t4-deany-aliases.py` indexes the exported declarations of `💻️os` and `🔨️modules` and rewrites each
placeholder to `import("<relative path>").X` when exactly one module exports it — it resolves **30 of
37** in the space harness. Two done by hand (`AppChannelHandle`, `AppCommandValue` in
`🧪️backbone-envelope-io`) took that file from 24 to 4.

**I deliberately did not run that codemod across the remaining files near the end of this session.**
De-anying surfaces the hidden errors *before* they can be fixed — the space harness went 55 → 122 the
moment it was typed — and handing over a number that had just doubled with no time left to drive it
back down would misrepresent the state. It is the correct next step for whoever picks this up, one
file at a time, with the scoped tsconfig loop (~8 s per file) rather than the full program.

**`✏️s` — 44.** Two clusters, both genuine defects rather than type noise:
- **🧩️puzzle diff modules reference parsers their generator never emitted.**
  `🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/…/🧬️schema/🔺️diff/🟦️.ts` call
  `parsePuzzle<N>dObjectPatch` / `…AttractionPatch` / `…TargetVolumePatch` / `…ReferencePatch` from
  their `…PatchEntry` parsers, and those functions do not exist anywhere in the tree. These modules
  have therefore never been executed. ~14 diagnostics; belongs with the 🧩️puzzle slice, since writing
  the missing parsers by hand is artifact-schema work, not typing work.
- Fixed here: the nine `📝️text/🟦️.ts` modules whose `parse<Export>Text` returned the **object** guard
  for a `string` export (`…GuardObject` → `…GuardString`) — a copy-paste in the text-representation
  emitter, 9 diagnostics.
- The rest is per-artifact: `🖐️5d` schema tuple arity, `📐️cad` presence-retirement test,
  `🔱️trinity` document contract, `🗄️stdio` semio contract.

**Not mine, still blocking a green `@semio-tech/framework-os:typecheck`:** 47 T2 (`🦑️repo` product),
19 T3 (`🔨️modules`), 3 legacy `♻️mit-bestand`.

**Third-party oracle libraries with no types (6 diagnostics, left alone deliberately).**
`whatwg-url` ×3, `lodash-es` ×2, `@webassemblyjs/{ieee754,leb128}` ×2 are used as independent
validation oracles in tests and ship no declarations. The options are a dev-dependency change to a
shared `package.json` or a hand-written ambient `.d.ts` — the first belongs to whoever owns the
manifest, the second has no precedent anywhere in this tree. Recorded rather than guessed at.

## 13. Peer discipline (this session)

Re-read before every edit; never waited on a peer. `🏛️ShellHost/🟦️.tsx` (19) and `🎞️frame-worker/🟦️.ts`
(11) are O2's and were left untouched, as in §6. Edits outside `💻️os` were four, each minimal and
additive: `RunCmdOpts.shell` + its `spawnSync` wiring (`🦑️repo/…/🏃️process/🟦️.ts`) and the three
component-test import paths. `🔍️discovery/🟦️.ts` ended the session at exactly the 4 diagnostics it
started with — verified, because I briefly duplicated five `Taxonomy` fields there and reverted them.
No cargo, no servers, no `🗑️generated` sweeps, no ticket bookkeeping, no git-modifying commands.

## 14. Files changed (this session)

```
🧰️framework/🛍️products/💻️os/tsconfig.json                                    allowJs: true
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts                 artifact→document drift ×7, ExecutionTargetReadResult, exported test-contract types
🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts     typed dependency bag, whole-file document rename, stubFetch
🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts              moved-module imports ×3, two real types, commandSeq helper
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts   five fixture types + typed Ajv compiles
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts   literal-preserving key(), typed hostCalls
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/{🧵️TaskManager,🚦️AgentPresence,🤖️AgentApprovals}/🧪️tests/🧩️component/🟦️.tsx   self-import → ../../, ShardBudget owner
📜️script.ts                                                                 taxonomy casts, shell opt, NativeOsScript ctor, stdio ledger typing, stdioMember
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts             RunCmdOpts.shell + spawnSync wiring
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/…/{📸️snapshot,🔺️diff,🧬️mutations}/📝️text/🟦️.ts   ×9 GuardObject → GuardString
```

Scratch tooling added (ticket folder): `🐍️t4-harness-deps-type.py`, `🐍️t4-rename-at.py`,
`🐍️t4-wrap-fetch-stub.py`, `🐍️t4-deany-aliases.py`, `📜️t4-count.sh`, and the scoped configs
`🔣️t4-scope.json` / `🔣️t4-one.json`. Captures: `🗑️generated/t4-os-run{4,5,6}.txt`,
`t4-scope{1,2,3}.txt`, `t4-vitest-os.txt`, `t4-vitest-os2.txt`.

---

# Session 4 continuation — slice T4b (2026-09-19)

## 15. Baseline at hand-over

Scoped config (`🔣️t4-scope.json`, 120 files, 33 s wall) → `🗑️generated/t4b-scope-base.txt`:

| owner | §11 run6 (full) | t4b start (scoped) |
|---|---:|---:|
| **T4 — `💻️os`** | 405 | **411** |
| **T4 — `✏️s`** | 44 | **44** |
| **T4 — root `📜️script.ts`** | 0 | **0** |
| **T4 owned total** | **449** | **455** |

The +6 in `💻️os` are peer additions (AU2/U1/U2 UI landing live); see §19.

## 16. De-anying the harnesses — done, and it did **not** spike

`🐍️t4-deany-aliases.py --apply` over the twelve `💻️os` harnesses that still opened with `type X = any;`
resolved **92 of 105** placeholders. The remaining 13 were resolved by hand, so **`💻️os` now carries
zero `type X = any;` lines**:

| name | why the codemod deferred | resolution |
|---|---|---|
| `OwnedParityImage`, `ParityDump`, `ParityNode` | declared unexported, re-exported through a trailing `export { type … }` clause the codemod's regex does not read | `import("../⚖️parity/{🖼️pixels,🏗️structure}/🟦️.ts").X` |
| `PluginRegistryEntry` | two owners (`🎠️kernel`, `🔎️discovery`) | `🔎️discovery` — the module the harness's own `External08` import already names |
| `SpawnDaemonHandle` | lives outside the codemod's search roots | `🦑️repo/🔨️modules/📚️library/🟦️.ts` |
| `RetainedUiPatchCursorRef`, `RetainedUiSurfaceOwnerRef` | nowhere — invented names | the instance types of the `RetainedUiPatchCursor`/`RetainedUiSurfaceOwner` **classes** they wrap |
| `TurnOutcome` | kernel vs `⏱️turn-budget` | kernel, because the harness's `createTurnOutcomeBroadcast` comes from `@semio-tech/framework` |
| `ArtifactState`, `DirectoryCommandTransportOperationV1`, `DocumentBrowserActorChild`, `InferenceOperationV1`, `RustWorkerHost` | declared unexported inside `🏪️store/👷️worker/🟦️.ts` | `export` added to the five declarations, referenced by `import(…)` |
| `HubSpaceArtifactCreationStatusV1` | an import **alias**, not a declaration | `import("…/🌱️space-artifact-creation-v1/🟦️.ts").SpaceArtifactCreationStatusV1` |

**§12 predicted the count would double; it fell.** `💻️os` went 411 → 401 in the same step. The space
harness's 55 → 122 spike happened when its *dependency bag* was typed (§10b), not when its aliases were:
the aliases were mostly already-correct shapes spelled `any`, so naming them added information without
admitting new call sites. Two harnesses that had **no** diagnostics before de-anying still have none.

Two remaining `dependencies: any` bags were also typed, both from their caller rather than by guessing:

- **`🧪️chunkkey` (21 → 0).** §12 said its caller "re-exports ~18 three.js names it neither declares nor
  imports by name". It does declare them — as one `const { Box3, BoxGeometry, … } = …` destructuring,
  which `🐍️t4-harness-deps-type.py`'s declaration regex could not see. Taught the script to read
  destructured bindings, then emitted `WorldR3fTestDependencies` (96 `readonly k: typeof k` members) into
  `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` beside the call site. Exact, and type-only, so nothing is bundled.
- **`🧪️ticket-owned-browser-host-staging`.** Its bag is built two lines above the call as
  `Object.assign({}, Fs, Crypto, …, Owner01…49, External01…14)`, so the parameter is simply
  `typeof testDependencies`. Its count did not move (16), but the 16 are now real diagnostics rather than
  16 that happened to survive an `any`.

## 17. os vitest: 348/13 → **361/361 green**

`bun ./📜️script.ts test long --run` in `💻️os/📦️packages/🟦️typescript`. Captures
`🗑️generated/t4b-vitest-{1..7}.txt`.

| run | passed | failed | files |
|---|---:|---:|---|
| t4b-vitest-1 (hand-over baseline, re-measured) | 348 | 13 | 2 of 5 failed |
| t4b-vitest-2 (directory JSON schema `artifactId`→`documentId`) | 350 | 11 | 1 of 5 |
| t4b-vitest-3 (lease fixture descriptor re-sealed) | 355 | 6 | 1 of 5 |
| t4b-vitest-4 (`appChannelVersion` 15 → 17) | 359 | 2 | 1 of 5 |
| t4b-vitest-5 (delete-space test aligned to its fixture) | 360 | 1 | 1 of 5 |
| **t4b-vitest-7 (cold-pair frontier key)** | **361** | **0** | **5 of 5 passed** |

Five root causes, all half-finished renames or version bumps — none of them "flaky", none fixed by
touching an assertion to make it pass.

**a. `📇️directory/🧬️schema/🔣️.json` was the last holdout of the `artifact`→`document` rename (−2 tests).**
The Rust twin (`🦀️.rs`: `document_id` on `DocumentScope`, `ArtifactFrontier`, `PublicDocumentCatalogEntryV1`,
`ConnectionView`, `DocumentDescriptor`), the TypeScript twin (`🟦️.ts`, same five) and **every fixture**
(`💻️os/🧫️fixtures/📇️directory/*.json`: 0 × `artifactId`, 29 × `documentId`) had moved; only the JSON
schema still required `artifactId`, so `DirectoryCommand`'s `announce-document` variant rejected its own
corpus. Renamed the key in 10 schema objects — surgically, off `git show HEAD:` and a text-level splice,
so the diff is exactly 20 lines and nothing was reformatted. **One deliberate exemption:**
`SpaceArtifactCreationReady` really is keyed `artifactId` (`💻️os/🟦️.ts:1015` pins the exact key set
`artifactId,artifactSchema,kindId,parentDialect`), matching §10's finding.

**b. `🔏️document-execution-target-lease-v1` shipped a descriptor sealed at protocol 15 (−5 tests).**
`🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json` carries the package descriptor
as packed bytes in `descriptorHex`. Its `executionProtocol.appChannelVersion` was **15**, while the
manifest beside it, `DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1` and 40 plugin descriptors are
**17** — so `parseVerifiedPackageDescriptorV1` refused every open with `descriptor mismatch`, surfacing as
the opaque `document open: invalid execution target`. Re-sealed with `🐍️t4b-descriptor-regen.ts`, which
round-trips the existing bytes through `decodePackValue`/`encodePackValue` first and **refuses if they
are not already canonical**, then rewrites the six hashes derived from them (`descriptor.sha256`,
`package.descriptorByteSha256` ×2, `sourceDescriptorByteSha256` ×2, and the digest list entry).
Byte length unchanged at 688.

**c. The same 15 → 17 bump was missing in three more places (−4 tests).**
`💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json` (×2) and the `const: 15` pins inside
`📇️directory/🧬️schema/🔣️.json` (×3). Checked before bumping that
`BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION = 15` is a **separate** constant, consistently 15 across its own
schema and fixtures — `🎯️action-handoff`, `🧾️describe` and `🌉️mcp/🏠️workspace` were left alone on purpose.
`✏️s/🔌️plugins/🧩️puzzle/🔣️.json` was the one plugin descriptor still at 15 and was bumped with them.

**d. `🗑️delete-space`: a stale test, not a product bug (−1 test).** The corpus
(`🛂️SpaceAdministration/🧫️fixtures/🗑️delete-space/🔣️.json`) declares `"confirmation": "get-after-accepted-receipt"`
and `"missingPageStatus": 404`, and the worker implements exactly that. The test drove
`administrationHarness([200])` and asserted **zero** GETs after submit, so the confirmation read got a
200 with an empty body and terminated `failed`/`invalid`. Fixed the test to the corpus it already loads:
`administrationHarness([200, fixture.missingPageStatus])`, one GET expected, plus an explicit
`expect(fixture.confirmation).toBe("get-after-accepted-receipt")` so the next reader sees why.

**e. The cold-pair page header has never been readable by its own parser — a real product bug (−1 test).**
`👷️worker/🟦️.ts:1035` built the page header's `baselineFrontier` with key **`documentId`**, while
`parseColdArtifactPairFrontier` (`🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts:80`) takes an **exact** record keyed
`artifactId` — the sibling half of the same line §10 corrected at `:964`. Every 64-page cold transfer
therefore threw `cold-pair.frontier` on the *final* page, inside a promise nobody awaited: the pages all
moved, `assertApplied` was never reached, `bindDocumentBackbone`/`renderSurface` never ran, and the only
visible symptom was an `unconfirmed` retirement in a `finally` block. Corrected to `artifactId`.

Getting there also required adapting the test's guest mock to a **product change the test had missed**:
the worker no longer passes the command page and the cold page as `poll` arguments 1 and 2; since the
`invokeWithPages` split they are their own `["reactor","stageCommandPage"]` / `["reactor","stageColdPairPage"]`
invocations that precede the poll (`👷️worker/🟦️.ts:1766-1775`). The mock now answers those two paths,
stashes the receipt it would have produced, and the following poll reports it. That alone removed the
uncaught `expected undefined to match object {owner: 0n, …}` — `message.args[1]` had been the turn budget,
not a command page, since the split.

One more genuine defect fixed on the way: the same harness read the worker source through
`new URL("../🔨️modules/…", source.url)`, but `source` is rebased to `💻️os/🟦️.ts`, so it resolved one level
above the product (`🛍️products/🔨️modules/…`) and the GIS-viewer test died on `ENOENT`. Now `"./🔨️modules/…"`.

**Noted, not fixed (K1's hygiene scope):** `🧪️space-artifact-creation-owner` still carries six
`console.log("[DEBUG] …")` lines (browser-render-refusal, browser-cold-pair-transfer).

## 18. 🧩️puzzle diff modules — the defect, and what it actually was

**There is no generator.** I looked for one: no `🏭️generator` directory under `🧩️puzzle` (nine other
plugins have one), no emitter anywhere in the tree for the `…GuardObject`/`…GuardReject` prologue these
modules open with, and `grep` for the emitted strings finds only the emitted files themselves. The
`🔣️.json` / `🟦️.ts` / `🦀️.rs` / `🛰️.proto` / `🔗️.graphql` sibling set is handcrafted, as AGENTS.md
requires ("handcraft all assets … without any ugly migrations"). §12's "their generator never emitted"
was an inference; the real defect is that the TypeScript twin was written short of its own schema.

**What was missing — 13 parsers across 5 modules,** every one of them a `$defs` entry of the sibling
`🔣️.json`, and 11 of them *called* by a function that was written:

| module | called but never declared | also missing |
|---|---|---|
| `◻️2d/🧬️schema/🔺️diff` | `parsePuzzle2dNodePatch`, `…EdgePatch` | `…TargetRegionPatch`, `…TargetRegionPatchEntry` |
| `🧊️3d/🧬️schema` | `parsePuzzle3dObject` | `…Attribute`, `…Author` |
| `🧊️3d/🧬️schema/🔺️diff` | `…ObjectPatch`, `…AttractionPatch`, `…TargetVolumePatch`, `…ReferencePatch` | — |
| `🖐️5d/🧬️schema/📸️snapshot` | `parsePuzzle5dCatalogPartKindExtra` | `parseArtifactRef`, `parseArtifactDialect` |
| `🖐️5d/🧬️schema/🔺️diff` | `…PartPatch`, `…FastenerPatch` | `…TargetVolumePatch`, `…TargetVolumePatchEntry` |

Because `parsePuzzle3dObject` is called from `parsePuzzle3dArtifact`, the 3d **artifact** module was as
dead as the diff ones: any call threw `ReferenceError` at the first object. The pattern is visible in
what *was* emitted — the only types with parsers were those whose every property is `required`. Whatever
produced these files skipped a `$def` the moment it met an optional property or a cross-file `$ref`.

**Root fixes.** The 13 parsers written in each module's own house style (guard prefix, `at` path
threading, `row["k"] === undefined ? undefined : …` for optionals), with the entity parsers imported
from the sibling artifact module rather than re-derived. Four schema-vs-TypeScript disagreements the
missing code had hidden, fixed on the side the other three twins already agreed with:

- **fixed-arity vectors.** `origin`/`orientation`/`position`/`direction`/`point` are declared
  `[number, number, number]` in TypeScript and pinned `minItems/maxItems` in JSON, but `.map()` yields
  `number[]`. Added `…GuardVector3`/`…GuardVector4` to the 3d and 5d prologues — a length-checked build,
  not a cast (8 diagnostics).
- **`Puzzle3dTargetVolume` / `Puzzle3dReference` `id`.** Required in TypeScript (and in the sibling
  `Puzzle3dVortex`), absent from the schema's `required`; added to the schema, parser made non-optional.
  `Puzzle3dObject.origin` likewise.
- **`Puzzle3dCatalogObjectKind.attributes` / `.authors`** were `{"type": "object"}` in the schema and
  anonymous inline shapes in TypeScript. Promoted to real `$defs` (`Puzzle3dAttribute`,
  `Puzzle3dAuthor`), named in TypeScript, parsers emitted.
- **`ArtifactChildHandle.target`** was `{"type": "string"}` in puzzle5d's snapshot schema but
  `ArtifactRef` in its TypeScript twin — and `os/store/child.json`, the canonical owner, `$ref`s
  `framework/io/schema.json#/$defs/ArtifactRef`. Schema corrected to that `$ref`.
- `✏️s/🔌️plugins/🧩️puzzle/🔣️.json` was the one plugin descriptor left at `appChannelVersion: 15` (§17c).

**The test — `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts`, 23 cases, all green.**
For all ten patch-entry parsers across the three artifacts it (1) validates a replacement document with
**Ajv reading the sibling `🔣️.json`** as the independent third-party oracle, (2) runs the owned parser
and asserts the JSON round-trip is identical, (3) asserts the parser refuses each of three hostile
documents the oracle also refuses, and (4) checks the empty patch. Registered in
`🧪️tests/🎚️config/🟦️.ts` (`include`) and in the package's `📋️project.json` `namedInputs`, so
`nx test @semio-tech/puzzle-js` covers it; capture `🗑️generated/t4b-puzzle-test.txt`:
**11 files, 39 passed (was 10 files / 16) — the three diff modules execute for the first time.**

`✏️s` diagnostics: **44 → 23.** What is left is not puzzle: `📐️cad/🗿️artifacts/📐️cad` implicit-any
reducer parameters (6), `🔱️trinity/🔌️jack` Ajv `AnySchema` casts (9), `♻️rewriting` a missing
`jsonPatch` namespace, `🗄️stdio/🧿️semio` two contract casts, `✒️writer` three, and one
`Board2dWasmSession` mismatch in `🧩️puzzle/◻️2d/✏️editor/🌉️wasm`.

## 19. The `🏛️ShellHost` / `🎞️frame-worker` tail, and peer discipline

§6 and §13 left both files untouched as O2's. They were still red, so per the hand-over they were taken
last — `🏛️ShellHost` **20 → 5**, `🎞️frame-worker` **11 → 0**.

- **`🏛️ShellHost` (15 of 20)** was the *same* half-done tutorial-track rename as `🔬️engine-contract`
  (§20a): `tracks: { … document: [] }` and two `for (const documentEvent of slice.document)` loops
  against a `TutorialTracks`/`TutorialSlice` that key `artifact` in the Rust source, the JSON schema, the
  generated binding and the fixture. Plus `applyTutorialSliceToShell` taking a bare `OwnedTutorialRunV1`
  where the class defaults `S = TutorialSnapshotV1` and every ref around it is
  `OwnedTutorialRunV1<DocumentArchivePack>` — which collapsed `run` to `never` and produced nine
  "Property 'ready'/'origin' does not exist on type 'never'" in a row.
- **`🎞️frame-worker` — a live defect, not type noise.** `receive()` guards on `message.lifecycle`, but
  `BrowserFrameShardPort` is a member of `BrowserFrameUiMessage` that carries **no** `lifecycle` and is
  never handled: a `shard-port` message fell through every `kind` branch into the frame-batch path, where
  `BigInt(message.generation)` on `undefined` throws and faults the Worker. It only ever appeared to work
  because `🐚️plugin-bridge`'s `routeShardPorts()` registers its **own** `self.addEventListener("message")`
  in the same Worker and consumes the port from there. Added the explicit early return.
  The other two: `reserveAssetResponse` / `sealAssetResponse` were declared `: void` on the worker handle
  while `🌐️browser-worker/🦀️.rs:130,168` return `Result<bool, JsValue>`, and both call sites assign the
  result to a `boolean` retry flag. Declared `boolean`, matching the Rust twin.

**Peers.** `🔐️HubSignIn`, `🏘️SpaceBrowser`, the TaskManager window and the touch work (AU2/U1/U2) carry
**no diagnostics** in the final full run, so there was nothing to pick up. Files I touched outside
`💻️os`, each minimal:
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts` (one `getByTitle` added to the owned
DOM-test boundary and its adapter — the interface AGENTS.md requires around `@testing-library`),
the five puzzle schema modules and their `🔣️.json` (§18), and
`🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json` (§17b).
No cargo, no servers, no `🗑️generated` sweeps, no ticket bookkeeping, no git-modifying commands.
Every `[DEBUG]` probe used while diagnosing the cold-pair failure was removed; `grep` confirms zero
`[DEBUG]` and zero `__t4b` in both files I instrumented.

**Blocker worth recording:** at 03:57 the disk filled to 1.2 GiB free and *every* command failed with
`ENOSPC` — the first full-program typecheck died with an empty capture. It recovered on its own (a peer
pruned; 54 GiB free minutes later). `bun nx run repo:cache-prune` is the sanctioned remedy
(`.cargo/config.toml` header).

## 20. Verification

**Typecheck (authoritative, full os program).** `bun tsc --noEmit --incremental false -p
🧰️framework/🛍️products/💻️os/tsconfig.json` → `🗑️generated/t4b-os-full2.txt`. 36 s wall (the machine was
quiet by then; the same command was 23 min in §2).

| owner | §11 run6 | t4b start | **t4b end** |
|---|---:|---:|---:|
| **T4 — `💻️os`** | 405 | 411 | **295** |
| **T4 — `✏️s`** | 44 | 44 | **23** |
| **T4 — root `📜️script.ts`** | 0 | 0 | **0** |
| **T4 owned total** | **449** | **455** | **318** |
| T3 — `🧰️framework/🔨️modules` | 19 | 12 | **0** |
| T2 — `🦑️repo` product | 47 | (scoped) | 9 |
| legacy `♻️mit-bestand` | 3 | (scoped) | 3 |
| **program total** | **518** | — | **330** |

**−131 owned (−29%), and −188 for the whole program.** T3's area is now zero, so
`@semio-tech/framework-os:typecheck` is blocked only by T2's 9 and the 3 unowned `♻️mit-bestand`
diagnostics — plus my 318.

**Tests — both suites green, and both moved.**

| suite | command | before | after |
|---|---|---|---|
| `@semio-tech/framework-os` | `bun ./📜️script.ts test long --run` in `💻️os/📦️packages/🟦️typescript` | 348 passed / 13 failed, 2 of 5 files failed | **361 passed / 0 failed, 5 of 5** |
| `@semio-tech/puzzle-js` | `bun ./📜️script.ts test --run` in `🧩️puzzle/📦️packages/🟦️typescript` | 16 passed, 10 files | **39 passed, 11 files** (23 new) |

Both re-run after the last edit of the session. Captures `🗑️generated/t4b-vitest-{1..7}.txt`,
`t4b-puzzle-test.txt`, `t4b-os-full{,2}.txt`, `t4b-scope-{base,deany,run2,run3}.txt`.
Everything in §17–§19 is verified by an actual run; the renderer-side type repairs in §16 and §19
(`🏛️ShellHost`, `🎞️frame-worker`, `🧪️chunkkey`) are verified **by typecheck only** — no suite covers them.

## 21. What is still open — 318 diagnostics I own

**`💻️os` — 295**, now a genuine long tail rather than one structural cause: TS7006 51, TS2345 38,
TS2322 26, TS2339 19, TS2769 18, TS2353 16, TS18046 16, TS18048 13. Heaviest files:
`🧪️space-artifact-creation-owner` 28, `🔌️plugin/🧪️tests/🌐️browser-bundle` 21, `🆕️fresh-component` 18,
`🧪️ticket-owned-browser-host-staging` 16, `🔬️engine-contract` 14, `🔬️catalog-smoke` 13,
`🧪️artifact-admission-and-completion-oracles` 9, `✅️catalog-verification` 8. Known leads inside them:
`PluginHotSwapMarker` is referenced and never declared (staging harness); `🏛️ShellHost:5922` reads a name
`catalog` that does not exist; `🌐️browser-bundle` still has `documentRead`-class drift in its Rollup
fixtures.

**`✏️s` — 23**, and **none of it is 🧩️puzzle schema any more**: `📐️cad/🗿️artifacts/📐️cad` implicit-any
reducer parameters 6, `🔱️trinity/🔌️jack` Ajv `AnySchema` casts 9, `♻️rewriting` a missing `jsonPatch`
namespace 1, `🗄️stdio/🧿️semio` 2, `✒️writer` 3, one `Board2dWasmSession` mismatch in
`🧩️puzzle/◻️2d/✏️editor/🌉️wasm`, one `📐️cad` presence-retirement test.

**Two half-renames I did not finish, and why.** Both are `artifact`→`document` sweeps that stopped at a
language boundary I was told not to cross (TypeScript-only slice, no cargo):

1. **`🛂️manifest` tutorial track.** `🦀️.rs:2358` still declares `pub artifact: Vec<TutorialArtifactEvent>`
   while its own doc comment above it already reads "the sole source of **document** mutation". The JSON
   schema, the fixture file name, the generated `🤖️generated/🪪️manifest/🟦️.ts` and the Rust tests all
   still say `artifact`; two os consumers had been renamed ahead of them. I aligned the **consumers** to
   the declared contract (schema-first is the repo law) rather than leave them broken, so the rename is
   now cleanly one-sided and can be done in one pass by whoever owns the crate: Rust field + enum names,
   `🧬️schema/🔣️.json`, `🧫️fixtures/🎞️tutorial-artifact-track.json` (file name included), the Rust tests,
   regenerate, then `🔬️engine-contract` §20a and `🏛️ShellHost` §19 flip back.
2. **`ColdArtifactPairFrontier`.** `🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts:80` takes an exact record keyed
   `artifactId`, and its Rust twin agrees, so §17e corrected the worker to match. If that vocabulary is
   also meant to become `documentId`, it is a three-language change, not a TypeScript one.

**Third-party oracle libraries with no types (6, unchanged from §12).** `whatwg-url` ×3, `lodash-es` ×2,
`@webassemblyjs/*` ×2 — still a manifest decision, still not guessed at.

## 22. Files changed (this session)

```
🧰️framework/🛍️products/💻️os/tsconfig.json                                     lib: DOM.Iterable
🧰️framework/🛍️products/💻️os/🧪️tests/🌐️fetch-stub/🟦️.ts                        NEW — one shared stubFetch
🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts      6 aliases, shared stubFetch, worker-source URL, delete-space harness, stage-page mock
🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts               TurnOutcome
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts                  5 type exports, cold-pair frontier key (artifactId)
🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json             artifactId→documentId ×20, appChannelVersion const 15→17 ×3
🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json  appChannelVersion ×2
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx     tutorial track + run generic
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts  shard-port return, asset handle booleans
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts    tutorial track, r3f imports, ActionDescriptor, fetch stubs, tree literals
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/{🔌️plugin-runtime,📨️browser-frame-transport,🫀️plugin-load-progress}  stubFetch ×14
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx    activeObjectId nullable, hover guard
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx   2 aliases
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx + 🧪️tests/🧪️chunkkey     WorldR3fTestDependencies
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts  5 aliases + typeof testDependencies
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts   host module path, artifactRead
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🟦️.ts        tree window/granularity
🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts                    getByTitle on the owned DOM-test boundary
🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json      descriptor re-sealed at protocol 17
✏️s/🔌️plugins/🧩️puzzle/🔣️.json                                                appChannelVersion 15→17
✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/…/🧬️schema/{,📸️snapshot/,🔺️diff/}{🟦️.ts,🔣️.json}   13 parsers + 4 contract repairs
✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts                            NEW — 23 cases
✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️config/🟦️.ts, 📦️packages/🟦️typescript/📋️project.json   test registration
6 os test modules — tree literals (window/granularity), self-import fixes carried from §10d
```

Scratch tooling added (ticket folder): `🐍️t4b-fetch-stub.py`, `🐍️t4b-descriptor-regen.ts`,
`🐍️t4b-descriptor-probe.ts`, `🐍️t4b-lease-probe.ts`; `🐍️t4-harness-deps-type.py` taught to read
destructured bindings; `🔣️t4-scope.json` widened to 124 files.

---

# Session 5 continuation — slice T4c (2026-09-19)

## 23. Baseline at hand-over

`bun tsc --noEmit --incremental false --pretty false -p 🧰️framework/🛍️products/💻️os/tsconfig.json`
→ `🗑️generated/t4c-os-full1.txt`, 82 s wall.

| owner | §20 t4b end | **t4c start** |
|---|---:|---:|
| **T4 — `💻️os`** | 295 | **298** |
| **T4 — `✏️s`** | 23 | **23** |
| **T4 — root `📜️script.ts`** | 0 | **0** |
| **T4 owned total** | **318** | **321** |
| T2 — `🦑️repo` product | 9 | 9 |
| legacy `♻️mit-bestand` | 3 | 3 |
| **program total** | **330** | **333** |

Loop tooling for this session: `🐍️t4c-scope.py` (same shape as T3's `🐍️t3b-scope.py`, but extending the
**os** tsconfig) writes `🔣️t4c-one.json` / `🔣️t4c-scope.json`.

## 24. §21's two half-renames, finished across all languages

Both were done with Rust + schema + codegen + fixtures, not just TypeScript. Nothing is left one-sided:
`grep -rn "TutorialArtifact\|tutorial-artifact-track"` and `grep -rn "ColdArtifactPair\|cold-artifact-pair"`
over `*.rs *.ts *.tsx *.json *.wit` (excluding `node_modules`, `🎯️target`, tickets) both return **0**.

### 24a. Tutorial document track (§21 item 1)

`🛂️manifest` declared `pub artifact: Vec<TutorialArtifactEvent>` under a doc comment that already read
"the sole source of **document** mutation", and `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts:281` already wrote
`tracks.document` — so the legacy demonstrator's tutorial was **failing to typecheck against the live
contract** (TS2353), which is what a half-rename hides. Renamed, schema-first:

| layer | file | change |
|---|---|---|
| Rust source | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `TutorialArtifactEvent(Kind)` → `TutorialDocumentEvent(Kind)`; `TutorialTracks.artifact` → `.document`; `TutorialSlice.artifact` → `.document`; `tutorial_slice` locals; `validate_tutorial`'s `sorted_by_at("artifact", …)` label → `"document"` |
| Rust tests | `🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` | three test fns renamed; the parity test's guard **inverted** — it asserted `tracks.get("document").is_none()`, it now asserts `tracks.get("artifact").is_none()` |
| JSON schema | `🛂️manifest/🧬️schema/🔣️.json` | `TutorialArtifactTrackFixture` → `TutorialDocumentTrackFixture`, its `required`/`properties` key `artifact` → `document` |
| fixture | `🛂️manifest/🧫️fixtures/🎞️tutorial-artifact-track.json` | **file renamed** to `🎞️tutorial-document-track.json`, top-level key renamed |
| projection source | `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs` | the two `SchemaMetadata` entries renamed **and moved** to their alphabetical slot (between `TutorialDefinition` and `TutorialEasing`); `TutorialTracks`'s projected field + its doc line (which still said "artifact mutation" while the Rust said "document mutation") |
| generated TS | `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` | regenerated by `bun ./📜️script.ts generate` in `🧰️framework/📦️packages/🦀️rust` (capture `t4c-fw-generate1.txt`) |
| hand TS twin | `🛂️manifest/🟦️.ts`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx` | type aliases, `TutorialTracks.document`, `TutorialSlice.document`, `tutorialSlice` locals, `validateTutorial`'s track label |
| consumers | `🏛️ShellHost/🟦️.tsx`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (3 × `slice.document`), `🔬️engine-contract/🟦️.ts` | flipped from the interim `artifact` spelling §21 had aligned them to |
| catalog | `🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` | regenerated by `bun ./📜️script.ts schema generate` (3244 scopes) |

**Verified:** `bun ./📜️script.ts generate` exits 0 (the typegen export test is a byte-compare against the
committed mirror, so a stale mirror would fail it); `cargo test -p semio-framework --lib tutorial` →
`t4c-fw-tutorial-tests.txt`, **16 passed / 1 failed**, and the 1 failure is **not this rename**:
`tutorial_document_event_kind_round_trips_tagged_camel_case` compares a hand-built `DslValue::Object`
(insertion order `op`, `inverse`) against the same value round-tripped through serde (which comes back
sorted, `inverse`, `op`). That is a `DslValue` map-ordering asymmetry in the `dsl` crate, which this slice
never touched; the assertion body is byte-identical to before the rename (only the fn name changed).

### 24b. Cold document pair (§21 item 2) — it *was* a half-rename, in the opposite direction

§21 recorded this as "consistent, so only do it if the vocabulary is meant to move". It is meant to move,
and the evidence is that the **user-facing half had already moved**: the registered script verbs are
`cold-document-pair-ingress-check` and `cold-document-pair-browser-check`, the worker's constants are
`COLD_DOCUMENT_PAIR_PAGE_BYTES` / `_MAXIMUM_BYTES` / `_MAXIMUM_PAGES`, its mint token is
`semio.os.verified-cold-document-pair.mint/v1` and all six of its throw messages read `cold document pair: …`
— while every *type* and the wire field were still `artifact`. Finished it:

- **299 identifier occurrences** over 25 source files: `ColdArtifactPair*` → `ColdDocumentPair*`,
  `coldArtifactPair*` → `coldDocumentPair*` (15 type names + 9 functions/consts, incl.
  `VerifiedColdArtifactPair`, `parseColdArtifactPairFrontier`, `dropVerifiedColdArtifactPair`).
- **Guest WIT** `🔌️plugin/🧬️schema/📜️.wit`: `cold-artifact-pair-{frontier,header,page,cursor,applied,fault}`
  → `cold-document-pair-*` (16 lines) and the record field `artifact-id: string` → `document-id: string`.
- **Wire field** `artifact_id` → `document_id` / `artifactId` → `documentId` in the four twins that own it:
  `🎭️actor/📥️cold-pair/🦀️.rs` (incl. the `pack::read_str` error labels), `…/🟦️.ts`, `…/🧬️schema/🔣️.json`
  (`required` + `properties`), and the `🎭️actor/🦀️.rs` projection entry; then every reader —
  `🎠️kernel/📥️cold-pair`, `🔌️plugin/⚛️reactor`, `🔌️plugin/🖥️host/📥️cold-pair`, their four Rust test files,
  `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`, and three `👷️worker/🟦️.ts` reads.
  `ArtifactRef.artifact_id` (a different type, ~60 sites in `🔌️plugin/🦀️.rs`) was deliberately **not** touched.
- **Generated TS** `🎭️actor/🤖️generated/🎭️actor/🟦️.ts` regenerated by `bun ./📜️script.ts typegen` in
  `🎭️actor/📦️packages/🦀️rust` (capture `t4c-actor-typegen.txt`, exit 0, `semio-framework-actor` compiled
  clean in 5 m 45 s) — so `ColdDocumentPairFrontier = { document_id: string, … }`.

## 25. Four root causes that account for most of the long tail

§21 called the remaining 295 "a genuine long tail". They are not: four mechanical causes explain most
of them, and each was proved in isolation before being applied. `🐍️t4c-narrow-probe.ts` (ticket folder)
is the standalone file that decided two of them — it is 15 lines and runs in the os program in 2 s.

**a. `ajv.getSchema<T>(ref)!` silently drops the `data is T` predicate; `ajv.compile<T>({ $ref })` keeps it.**
`getSchema` is declared to return `AnyValidateFunction<T> | undefined` — a **union** of the sync
`ValidateFunction<T>` (`(data) => data is T`) and `AsyncValidateFunction<T>` (`(data) => Promise<T>`).
A union of call signatures is not a type guard, so `assert(validate(fixture))` narrows nothing and every
later read is `TS18046: 'fixture' is of type 'unknown'` (or, when the fixture was left as `JSON.parse`'s
`any`, an invisible `any` cascade instead). `compile<T>` returns the sync type directly and narrows.
Probe output: the `getSchema` line errors, the `compile` line does not. §12's `🧪️reactor-contract-oracles`
fix used `compile` and worked; §16's remaining files used `getSchema` and did not — that is the difference.

**b. An assertion function forces an explicit type annotation on everything it touches.**
`assert` from `node:assert/strict` is `asserts value`, and TypeScript refuses to infer the type of any
`const` that an assertion call references — reported as `TS7022: … implicitly has type 'any'` on the
*declaration*, which reads like an unrelated inference bug. `const definition: string = readFileSync(…)`
fixes it. The same rule fires as `TS2775: Assertions require every name in the call target to be declared
with an explicit type annotation` when `assert` itself arrives through a destructured dependency bag —
so a harness must **import `assert` directly** rather than receive it (the house style everywhere else).
This is why typing a `dependencies: Record<string, any>` bag can appear to *add* ~120 diagnostics: the
bag's `any` was suppressing the assertion checker entirely.

**c. `assert(x?.y)` does not narrow `x`; `assert(x !== undefined && x.y)` does.** Optional chaining
inside an assertion argument leaves the subject's type alone, so every later use is `string | undefined`.
Nine sites, all on `process.env.SEMIO_TEST_ARTIFACT_DIR` guards, each followed by `mkdirSync(root)`.

**d. A `never`-returning helper only ends control flow when the *variable* carries the annotation.**
`const deny = (): never => { throw … }` annotates the arrow, not the binding, so `if (!row) deny();`
does not narrow `row` and every field read after it is `TS18048: possibly 'undefined'`.
`const deny: () => never = () => { throw … }` — one line — took
`🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts` from **8 to 0**.

### Files closed by these four (each verified at 0 with the scoped loop)

| file | before | after | what it was |
|---|---:|---:|---|
| `🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component` | 18 | **0** | typed deps bag (`FreshComponentTestDependencies`, 28 `typeof` members emitted into the caller by `🐍️t4c-deps-type.py`), 4 × `getSchema`→`compile` + three fixture types read off the sibling `🧬️schema/🔣️.json`, `assert.throws(fn, undefined, msg)` → `assert.throws(fn, msg)` ×5, `assert(root?.includes)` → `!== undefined &&`, and `FreshComponentLeaseV1.consume`'s parameter declared `Uint8Array<ArrayBuffer>` to match the `new Uint8Array(len)` the implementation actually loans |
| `🔌️plugin/🧪️tests/🌐️browser-bundle` | 21 | **0** | typed deps bag (28 members); `assert` moved out of the bag to a direct `import` (that alone was 100+ TS2775 once the bag stopped being `any`); two `artifactBase?.includes` guards; one `Uint8Array<ArrayBuffer>` |
| `🔌️plugin/📇️registry/✅️catalog-verification` | 8 | **0** | `deny` binding annotation |
| `🔌️plugin/🧪️tests/🧪️artifact-admission-and-completion-oracles` | 9 | **0** | `getSchema`→`compile<T>` + two fixture types from the schemas, four `: string` annotations on assertion-referenced consts, `assert(region)` → `assert(region !== undefined)`; the four hand-written `(row: { id: string })` parameter annotations became unnecessary and were deleted |
| `🧪️tests/🧩️jcoprobe-callback` | 6 | **0** | the file was still plain JS in a `.ts` extension: `const verdicts = []` and `function record(id, ok, detail)`; given a `JcoProbeVerdict` row type |

One `[DEBUG]` console line left behind by a predecessor was removed on the way
(`🆕️fresh-component/🟦️.ts:104`, whose two sibling evidence lines never had the prefix).

## 26. Session 5 re-measure (T4c resumed 2026-09-19 ~23:30)

The previous T4c worker died at 11:36 after landing §24 and §25. Its last capture
`🗑️generated/t4c-os-full3.txt` (11:34) read **242** owned. Re-measured now, whole program,
`🗑️generated/t4c-os-full4.txt` (7 m 21 s wall — 13 % CPU, the fleet is loaded; the scoped loop is the
only workable iteration tool):

| owner | §23 t4c start | §25 end (11:34) | **now** |
|---|---:|---:|---:|
| T4 — `💻️os` | 298 | 219 | **221** |
| T4 — `✏️s` | 23 | 23 | **23** |
| T4 — root `📜️script.ts` | 0 | 0 | **0** |
| **T4 owned total** | **321** | **242** | **244** |
| T2 — `🦑️repo` product | 9 | 9 | 9 |
| legacy `♻️mit-bestand` | 3 | 1 | 1 |

Drift of +2 against §25's capture is peer churn in `🏛️ShellHost` / `🔗️AgentBridge` (C1c and M7).

## 27. Correction to §26's non-owned column

§23's table carried "T2 — `🦑️repo` product 9 / legacy 3" forward unchanged. That is stale. The ten
non-owned diagnostics in `t4c-os-full4.txt` are actually:

| file | count | who |
|---|---:|---|
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` | 7 | **T4 — §24a's own leftover** |
| `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle/🟦️.ts` | 2 | legacy |
| `🦑️repo/…/⚡️caching/🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts` | 1 | T2 |

So §24a's tutorial-track rename was **not** finished: the ui suite still wrote `tracks.artifact` and
read `tutorialSlice(…).artifact` in seven places. §24a's grep only covered the *type* names
(`TutorialArtifact*`), never the renamed FIELD. Fixed here; `grep -rn "tracks\.artifact\|ui: \[\], artifact: \[\]"`
over `*.ts *.tsx *.rs *.json` now returns 0.

## 28. Root causes found and fixed this session

Each is a single cause with many diagnostics downstream, verified with the scoped loop
(`🐍️t4c-scope.py` → `🔣️t4c-one.json` / `🔣️t4c-probe.json`, captures `t4c-wave*.txt`, `t4c-staging*.txt`).

**a. `Object.assign({}, …)` past three sources types the whole bag `any`.**
`lib.es2015.core.d.ts` declares `assign` for 1, 2 and 3 sources and then falls back to
`assign(target: object, ...sources: any[]): any`. `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging`
built its 69-namespace dependency bag that way, so `typeof testDependencies` was `any` and EVERY case in
that 2 000-line file was unchecked. Replacing it with an object-literal spread (`{ ...Fs, ...Crypto, … }`)
took the file from **15 reported → 46 real**, which is the honest number; the 31 that appeared were
already-broken calls. Two of them were live defects rather than type noise:
- **`resolve` was `url.resolve`, not `path.resolve`.** Both `node:path` and `node:url` export `resolve`,
  and `...Url` spread last, so the bag's `resolve` was the deprecated two-argument
  `url.resolve(from, to)` while all three call sites pass one path. Order flipped to `…, ...Url, ...Path`.
- **`buildPluginCatalog` resolved to the wrong function.** `📇️registry/🟦️.ts` exports a zero-argument
  catalog *projection* builder and `📇️registry/🔄️refresh/🟦️.ts` exports the six-argument catalog *build*;
  the later spread won, so the two `buildPluginCatalog(targets, cargoFn, materializeFn, …)` cases were
  calling the projection. Pinned explicitly in the bag with the reason in its docstring. **The name
  collision itself is still open** — see §30.

**b. `atTestLevel`'s generic constraint could never be satisfied by Vitest's `TestAPI`.**
`atTestLevel<Case extends { runIf(condition: boolean): Case }>` requires `runIf` to return the SAME type,
but `@vitest/runner` declares `runIf: (condition: any) => ChainableTestAPI<ExtraContext>` — a supertype,
not `TestAPI`. So every `const itLong = atTestLevel(it, "long")` was an error and every `itLong(…)` after it
was `TS2349: This expression is not callable`. One signature change in
`🦑️repo/🔨️modules/📚️library/🟦️.ts` — `atTestLevel<Gated>(factory: { runIf(condition: boolean): Gated }, level)`
— closed **~27** diagnostics across `🧪️ticket-owned-browser-host-staging` (25) and
`📇️registry/🧪️tests/🎮️playground-session` (2).

**c. Six third-party test oracles had no ambient declaration.**
`whatwg-url` (×3), `lodash-es` + `lodash-es/findIndex.js` (×2) and `@webassemblyjs/ieee754` (×1) were all
`TS7016`. The repo already owns exactly one home for this — `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts`,
whose docstring says "the exact members those call sites use" — so four `declare module` blocks went there
beside the existing `@webassemblyjs/leb128` and `semver` entries. No `any`, no `@types/*` install.

**d. A union constituent with a multi-literal discriminant never narrows.**
`BrowserFrameDomEvent`'s pointer member was `{ type: "pointermove" | "pointerdown" | "pointerup"; … }`.
TypeScript filters *constituents* on a discriminant test; a constituent whose discriminant is a union of
three literals is never removed by `=== "pointermove"` and never narrowed within, so after the pointer,
wheel and resize branches returned, `event.key` still resolved against the pointer member (5 × TS2339 on
one line). Split into three constituents sharing `BrowserFramePointerDomFields` (and the key member into
two sharing `BrowserFrameKeyDomFields`), which makes it a real discriminated union.

**e. `ThemePaletteGroup` grew two members that the settings UI never got.**
`🖱️ui/🎨️styling/🌓️theme/🟦️.ts` declares six groups (`outcome` and `diagram` were added for the wgpu target),
but `📌️ChromePanels`' label map still had four and `satisfies Record<ThemePaletteGroup, UiTranslationKey>`
failed (TS1360 + TS7053). Fixed at the product end, not by narrowing the map: `ui.settings.theme.group.outcome`
and `.diagram` added to `📚️I18n`'s `UiTranslationSchema` and to **both** locale packs in
`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, the label map completed, and `appearanceGroups` widened to all six — so the
two new palettes are now editable in settings instead of being unreachable.

**f. `TreeDataItem.onClick` could not be pressed outside a rendered tree.**
Its contract is `(event: React.MouseEvent, context: TreeDataActivationContext) => void`; tests either called
it with zero arguments (4 × TS2554) or with `{} as never, {} as never` (2 casts in `🔬️engine-contract`).
Added `treeDataActivation()` to `🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` (re-exported from the react target): it wraps a
real DOM `MouseEvent` in React's synthetic surface, so `preventDefault()`/`stopPropagation()`/`button`/`metaKey`
behave as under a rendered row. Both call sites now spread it; the two `as never` casts are gone.

**g. The shell-local `PluginManifest` had no `topicContributions`.**
`🐚️Shell/🟦️.tsx` keeps its own richer `PluginManifest` (documented as deliberate). It omitted
`topicContributions`, which is the ONLY field `scopeContributionsJson` reads — so the host→guest
contributions push in `🏛️ShellHost` typechecked against a weak type with no properties in common, and
carried a `{ ...entry.manifest, workflows: [] }` shim to paper over it. Field added (typed as core's
`TopicContribution`), shim replaced by `{ topicContributions: entry.manifest.topicContributions }`.
The same wrongness made `🩺️window-fault`'s four fixtures cast to `PluginManifest`; those casts are deleted —
the parameter is `Pick<PluginManifest, "topicContributions">` and the literals satisfy it directly.

**h. `AreaState` lost `legacy`/`mixed`.** `🦑️repo/…/🔍️discovery` now declares `AreaState = "clean" | "exempt"`,
but `🔌️plugin/📇️registry/🔎️discovery`'s `mergeAreaStates` still ranked three states and
`📽️projection` still compared against two dead literals (4 + 2 diagnostics). Both rewritten against the live
vocabulary, docstrings and the explanatory comments included.

**i. One 90-line duplicated default.** `🖥️shell/🧪️tests/🧪️semio-tech-framework-os-shell-reduce` hand-rolled a
`defaultState()` for the flat pre-slice shape and aliased `ShellState` from the *renderer*'s `🐚️Shell`
component rather than from the shell SSOT it actually drives. `🖥️shell/🟦️.ts` already exports
`defaultShellState()`, whose docstring calls itself "the ONLY place a default is spelled on this side".
The duplicate is deleted and the test takes `defaultShellState` through its dependency bag.

Smaller root fixes in the same pass: `DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES` imported twice in
`💻️os/🟦️.ts` (once from the schema module, once from the barrel that re-exports it) → TS2300 ×2;
`CRATE_NAME` used but not imported in `🔌️plugin/🖨️describe/🛂️descriptor-emission`; `PluginHotSwapMarker`
not exported from `🧑‍💻dev/🔌️vite-plugins`; `acceptBrowserSessionAuthority` destructured from the backbone
worker's dependency bag although it lives on `testSeams` (and every use already reads it there);
`parseDirectorySessionAuthorityLocaleV1` added to `📇️directory/🪪️session-refresh` so a fixture locale narrows
through the module's own refusal instead of indexing a two-key record with `string`;
`storage: TEST_HOST_STORAGE` added to the six `BrowserFrameWorkerBoot` literals in
`🧪️tests/📨️browser-frame-transport`; `semioPlaygroundReactRefreshCoherenceVitePlugin`'s `config`/
`transformIndexHtml` hooks given the structural parameter types the rest of that file already uses.

## 29. §28 was never measured whole-program — it is, now (§30)

The previous T4c worker was cut at 00:54 with §29 left as a placeholder. Its last scoped captures
(`t4c-typecheck2.txt`) were partial. First act of this session: one full-program run.

---

# Session 5 — slice T4c (resumed 2026-09-20)

## 30. Re-measurement at resume

`bun x tsc --noEmit --incremental false --pretty false -p 🧰️framework/🛍️products/💻️os/tsconfig.json`
→ `🗑️generated/t4c-os-full6.txt`, **8 m 16 s** wall (10 % CPU — the fleet is at load ≈ 90).

| owner | §23 t4c start | §26 (09-19 23:26) | **now (09-20 resume)** |
|---|---:|---:|---:|
| T4 — `💻️os` | 298 | 221 | **122** |
| T4 — `✏️s` | 23 | 23 | **8** |
| T4 — root `📜️script.ts` | 0 | 0 | 0 |
| **T4 owned total** | **321** | **244** | **130** |
| legacy `♻️mit-bestand` | 3 | 1 | 2 |
| T2 — `🦑️repo` product | 9 | 9 | 1 |
| **program total** | **333** | **254** | **133** |

So §28's root fixes landed and hold: 244 → 130 owned. Codes: TS2345 25, TS2322 24, TS2769 14,
TS2339 10, TS2353 10, TS18047 6, TS2556 5, TS2740 5. Heaviest files:
`🧪️space-artifact-creation-owner` 28, `🔬️engine-contract` 14, `🌉️mcp/🧪️tests/🤖️live-agent-loop` 7,
`🧑‍💻dev/🧪️tests/🧹️config` 7, `🌐️World3dHost` 5, `🔱️trinity/🔌️jack/🪜️resumable-query` 4,
`🧊️wgpu/🐚️plugin-bridge` 4, `🏛️ShellHost` 4.

## 31. Root causes found and fixed this session

Each closed with the scoped loop (`🐍️t4c-scope.py` → `🔣️t4c-one.json`, captures `t4c-s5-wave{1,2}.txt`).
A scoped run under the current fleet load costs ~2 min, the whole program ~8 min, so every wave batches
a dozen files.

**a. `FreshnessServer.config.server.hmr` was required; Vite's `ResolvedServerOptions.hmr` is optional.**
`🧑‍💻dev/🔌️vite-plugins`' own structural server type demanded `hmr`, so `ViteDevServer` was NOT assignable
to it, so `configureServer` was not a valid `ServerHook`, so the whole plugin array was not a
`PluginOption[]`, so `createServer(inlineConfig)` fell through `InlineConfig | ResolvedConfig` to
**`ResolvedConfig`** — and reported `configFile: false` as "boolean is not assignable to string",
`server: {…}` as "missing 12 properties of `ResolvedServerOptions`" and the plugin array as incompatible.
Six diagnostics in `🧑‍💻dev/🧪️tests/🧹️config`, one character of cause (`hmr?: unknown`). The probe that
proved it is `🐍️t4c-vite-probe.ts` (ticket folder): the same literal asserted against `InlineConfig`
alone, which names the real chain in one error instead of a union mismatch.
Same file's 7th: `semioSourceWatchVitePlugin` declared `freshness?: SourceFreshnessRegistry` although it
reads exactly one member — narrowed to `Pick<…, "movedInDirectory">`, which deletes the test's
`as ReturnType<typeof createSourceFreshnessRegistry>` cast. **7 → 0.**

**b. `lib` was missing `DOM.AsyncIterable`.** `for await (const chunk of response.body)` is how three
files read a stream; TypeScript ships `ReadableStream[Symbol.asyncIterator]` only in that lib. Added to
`💻️os/tsconfig.json` beside `DOM.Iterable`. Closed `🧊️wgpu/📦️publication`, `🌊️flow/🕸️wasm/🌐️browser/📦️publication`
and the T2-owned `🦑️repo/…/🕸️wasm/📜️script.ts` — **3 → 0** without touching a line of source.

**c. `mode: "test"` inside a vitest `test:` block is not in `InlineConfig`.** Three os-program configs
(`🌉️mcp`, `🖥️shell`, `💻️os/🧪️tests`) carried it; the ~20 sibling configs in the same program do not.
Vitest does read `options.test?.mode` at runtime, but only as an override of the run mode the CLI
already set — so hardcoding `"test"` is a no-op for `vitest run` and would actively break `vitest bench`.
Removed. **3 → 0.**

**d. `Array.prototype.map` erases a tuple's length, so its result cannot be spread into a fixed-arity call.**
`🔬️interactivity-mounted-frame-transaction` (17 positional source parameters) and
`🔬️interactivity-mounted-engine-surface-lifetime` (10) both built `const clean = files.map(read)` and
called `failures(...clean)` → 5 × `TS2556`. Both now declare
`const clean: Parameters<typeof <thatAudit>> = [read(…), …]` and `const mutated: typeof clean = [...clean]`
— derived from the audit's own signature, so a source added without a parameter to receive it is now an
error instead of passing unnoticed. **5 → 0.**

**e. The TS schema-mirror emitter never parenthesised an array element.**
`🌉️mcp/📦️packages/🦀️rust/📜️script.ts`'s `renderType` emitted `` `readonly ${element}[]` `` unconditionally,
so `Vec<Vec<JsonValue>>` became `readonly readonly JsonValue[][]` — which TypeScript refuses outright
(`TS1354`) — and a union element would have silently become `(readonly A) | (B[])`. Fixed in the
**emitter** (bare identifiers unchanged, everything else parenthesised) and the mirror regenerated with
`bun ./📜️script.ts schema-mirror`, never hand-edited. The committed `🔣️.json` was already stale behind
the built binary (29 added lines, 0 removed — `🧬️schema/🦀️.rs` 23:44 vs binary 23:56), so the same run
brought it forward; `schema-mirror --check` is green again where it was red before this session.

**f. `Bun.spawn` is declared twice, so `ReturnType<typeof Bun.spawn>` is a coin toss.**
bun-types declares it generic over the stdio literals; the repo's own
`🦑️repo/…/🏃️process/🌿️environment/🟦️.d.ts` declares `Bun.spawn(…): BunSubprocess` in a merged
`declare namespace Bun`. `🌉️mcp/🧪️tests/🤖️live-agent-loop` annotated its child
`ReturnType<typeof Bun.spawn>`, which picked the overload that types `stdin` as `number | FileSink`
and `stdout` as `number | ReadableStream` — 7 diagnostics on `.write`/`.flush`/`for await`. Dropping the
annotation and letting TypeScript infer the field from the constructor assignment binds the signature the
call actually resolved to. **7 → 0.**

**g. A story that uses `render` still has to satisfy the component's required props.**
`📊️Table`, `🧩️BlockListHost` and `🖼️IconRenderHost` each had one `render`-only story mounting a different
host, and Storybook's `StoryObj<typeof meta>` still demanded `args`. Declared the defaults once on
`meta.args`, which is what lets every story state only what it changes. **3 → 0.**

**h. Two identifiers had been renamed away and one type had moved.**
`DEFAULT_SHARD_BUDGET` → `MAINTENANCE_LANE_DEFAULT_BUDGET` (12 uses in `🧺️turn-patch-batch`);
`ViewModel` is exported by `🐚️Shell`, not by `@semio-tech/framework` (`📇️directory-home-bootstrap`);
`fast-json-patch`'s `Operation` is a named export, not a namespace member of its default export
(`♻️rewriting/…/🗂️map-ownership`). **4 → 0.**

**i. `Board2dWasmSession.setSelectionScreenPreview` disagreed with the only binding that implements it.**
The hand-written contract in `🪪️WasmSessionLoader` said `(flatXy: readonly number[])`; wasm-bindgen emits
`(flat_xy: Float64Array)` for the Rust `&[f64]`, and `semio_puzzle.d.ts` is the only declaration of it in
the repo. No TypeScript caller exists, so the contract was simply wrong. **1 → 0.**

**j. Two test harness literals were missing members their contracts require.**
`storage: TEST_HOST_STORAGE` on the `BrowserFrameWorkerBoot` literals of `⏱️wgpu-ui-turn-budget` and
`🎮️wgpu-browser-input-wire` (the same gap §28 closed in `📨️browser-frame-transport`); `invoke`,
`dispatchInvokeExtension`, `pushScopedContributions` and `captureExtensionCompletion` on
`🧩️package-integration`'s `fakeHandle` — they were reaching the result only through
`Partial<WgpuPluginHandle>`, which made every one of them optional. **3 → 0.**

**k. `wgpuBuildScopedContributionsPack` asked for a whole `PluginManifest` to read one field.**
Its only consumer, `scopeContributionsJson`, already takes `Pick<PluginManifest, "topicContributions">`
(§28g). Narrowed to match, which is what made `🔬️wgpu-extension-dispatch`'s three contribution fixtures
legal without inventing `apps`/`workflows`/`examples`. **3 → 0.**

**l. Smaller root fixes in the same waves.** `requestId: 0` → `""` in `🧪️backbone-envelope-io`
(`LocalInteractionQueryCommand.requestId` is a string) and its `enqueue` stub widened to
`readonly Uint8Array[]`; the six-shape `🪜️resumable-query` fixture read through a
`Partial<Record<…, string>>` binding so an absent statement is a branch rather than an `undefined` handed
to SQLite; `🪪️document-contract`'s two `as Record<string, (input: unknown) => unknown>` module casts
replaced by the direct `snapshot.parseSemioObjectSnapshot` / `diff.parseSemioObjectDiff` imports (both
exist, correctly typed); the same file's `Object.values(value)` given the corpus record's real shape;
`💡️inference-bridge`'s hub-schema annotation extended with the `properties` it actually reads.

## 32. Session 5b (resumed 06:15 after the ~03:00 session-limit cut) — 73 → 10

Re-measured once at resume (`🗑️generated/t4c-s5-full7.txt`, 73 total / **71 owned**), then bucket by bucket.
Final whole-program run `🗑️generated/t4c-s5-full10.txt`: **10 total, 8 owned by T4, 2 legacy**.

| owner | §23 t4c start | §30 (09-20 01:20) | §32 resume | **final** |
|---|---:|---:|---:|---:|
| T4 — `💻️os` | 298 | 122 | 71 | **8** |
| T4 — `✏️s` | 23 | 8 | 0 | **0** |
| T4 — root `📜️script.ts` | 0 | 1 | 0 | **0** |
| **T4 owned total** | **321** | **131** | **71** | **8** |
| legacy `♻️mit-bestand` | 3 | 2 | 2 | 2 |
| T2 — `🦑️repo` product | 9 | 1 | 0 | 0 |
| **program total** | **333** | **133** | **73** | **10** |

### 32a. Root causes closed in this half

**a. `FakeHubWebSocket` was a resemblance, not a `WebSocket` (28 → 1 in `🧪️space-artifact-creation-owner`).**
Seven diagnostics were the test's hub-socket double being forced into `handleHubFrame`/`state.socket`,
each site paying for it separately. `class FakeHubWebSocket implements WebSocket` — with the instance
`CONNECTING`/`OPEN`/`CLOSING`/`CLOSED`, `extensions`, `bufferedAmount`, `binaryType: BinaryType` and three
refusing listener methods — makes the compiler name a member the worker could reach and the double has
not implemented, which is the point of the double. The same file's other eleven: the retained
`InferenceOperationV1` built from the committed `🌐️browser-document-open-v1` corpus through
`parseDocumentExecutionTargetLeaseFieldsV1` instead of a partial literal; a `mirrorRetirement` holder
instead of a `let` TypeScript narrows to its initializer (a `?.()` on `never`); `transferableBuffer`,
which narrows `Uint8Array.buffer` with `instanceof ArrayBuffer` because a `SharedArrayBuffer` view is
not `Transferable`; the readonly `Welcome.bootstrap` variant rebuilt rather than written over; a
session fence on the undo owner; `{ kind: "none" }` excluded from the browser-actor digest rewrite; and
three `expect(x).not.toBeNull()` "narrowings" replaced by refusals that actually narrow.

**b. `UiDriverProvider` declared `children` REQUIRED (21 → 0 in `🔬️engine-contract`).**
`createElement(Comp, props, child)` type-checks `props` alone, so every `createElement(UiDriverProvider,
{ driver }, …)` in the suite was a `TS2769`. One `?` on the provider's own prop — which is what React's
`PropsWithChildren` says anyway — closed six. The rest of that file was contract drift a peer left behind:
`TableHost` imported twice, `snapshotJson` → `documentJson` on `InkCanvasScene` (4 sites), `activeUtility`
gone from `InkDocument`, `method: "click"` gone from `SelectionMarqueeMethod`, `gumballActive` gone from
the gumball selection args, `id` gone from the instance-pick record, `ArgSchema` now requiring `options`,
`GumballPose` needing tuples rather than `number[]`, `new Promise` without its type argument, and a
`never[]`-typed JSON fixture column read through an annotated binding.

**c. Two of my own regressions, found by the next full run and fixed.** `typeof event.remote` inside a
cast annotation reads the DECLARED type, not the narrowed one — and naming the local `remote` then made
it self-referential (`TS2502`); a second binding fixes both. Tightening `LeftoverInteractionViewV1.activeMode`
to `SelectionMode` (which is what the shell's `InteractionState` declares) needed a matching
`leftoverSelectionModeRecord` parser, because the guest encodes that field as free text and an unknown
mode must be dropped rather than smuggled in.

**d. Smaller root fixes.** `Bun.build`/`playwright` reached through an indirect specifier erase their
module types, so `🎨️world3d-scene-shading/📜️script.ts`'s callbacks were implicit `any` and its corpus
`unknown` — typed at the import and with a named fixture type. `isValidConnection` was missing from
`🕸️Diagram`'s `DiagramProps`, so `🕸️NodeGraph`'s connection-validity guard was being dropped at the
boundary instead of asked: added to the contract, destructured, and forwarded to React Flow.
`semanticOwnedFileProjectionContracts` is a union whose `exact-owner-path-catalog` member names
`sourceBasenames` rather than one `sourceFilename`, so the empty-lane marker is now read through a
discriminating helper. `FaultScope` has no `req` (it is a 5-field Rust/WIT/TS contract), so both
extension-answer faults carry the request id in their message instead. `PluginRuntime`'s hot-message
guard compared `decodeBackboneMessage(bytes).kind === "snapshot"` although that variant is now `"genesis"`
— **a dead guard, not type noise**: the hot lane had stopped refusing a genesis pack. `FlowBrowserOptions`
was missing `bindings`, which `🏃️runtime/🟨️.js` destructures and `🪪️WasmSessionLoader` passes — added to
the **projection template** and regenerated with `bun ./📜️script.ts declarations`, never hand-edited.

### 32b. os vitest — K2's ten regressions: 10 → 3, and the three that are not mine

K2 §16.1 measured `@semio-tech/framework-os:test` at 349/359 with ten failures. Re-measured here after
each fix (`🗑️generated/t4c-s5-vitest{1..4}.txt`): **356 passed / 3 failed (359)**.

| test | what it was | disposition |
|---|---|---|
| `tags every AppCommand variant…` | mine, within this session: `requestId` is a **decimal-u64 string**, so `""` throws `local-interaction.invalid-u64`. `"0"`. | fixed |
| 5 × execution-target / lease / body-reader / viewer | `HUB_SESSION_CAPABILITY_PATTERN_V1` is `session.v1.<32 hex>.<64 hex>`; the harness still installed a bare 64-hex proof, so `installHubSessionCapability` refused and every test behind it failed. The capability is now minted inside `acceptCurrentTestBrowserSessionAuthority`. | fixed |
| `allowlists only one canonical safe-decimal event-page route…` | `browserDirectoryRequest`'s allowlist matches the un-prefixed `/directory/…` path the live `DirectoryClient` builds (`requestBaseUrl: ""`); the test still passed `/_semio/hub/directory/…` and was denied. | fixed |
| `browser document first open rejects hostile assets…` | `foreign-plan-scope` now accepts what the fixture expects refused — a plan-scope admission change in the worker, not TypeScript. | **route to W3c / C1c** |
| `browser document actor reservation activates only after an exact current socket Session` | `session activation test deadline`: the activation never settles under the current socket-Session path. | **route to W3c / C1c** |
| `browser document actor transfers one verified cold pair…` | `hub session rebootstrap required` thrown by `hubSessionFetch`. | **route to W3c / C1c** |

### 32c. The typecheck target in the standard chain

`@semio-tech/framework-os:typecheck` (`bun ./📜️script.ts typecheck` → `tsc --noEmit -p ../../tsconfig.json`)
was already registered in `💻️os/📦️packages/🟦️typescript/📋️project.json`. It had **no launcher**; added
`🛠️dev💻️os🪁️typecheck` to `.vscode/launch.json` **and** `.vscode/🧩️launch.seed.jsonc` at group `3_dev`
order `390.2`, immediately after `🛠️dev🧰️framework🪁️typecheck` (390.1), following that family's naming.
No `dependsOn` was added: neither `@semio-tech/framework` nor any sibling chains `typecheck` into `test`,
and an 8-minute whole-program `tsc` in front of every test run would be a different decision from this slice's.

### 32d. What is still open — the 8 I own

| file | count | what it needs |
|---|---:|---|
| `🗣️Interpreter/📖️stories` | 2 | The story is written against the RETIRED `interpretUiNode(node, { onAction })`; the live signature is `(store: UiDocumentStore, context: UiInterpreterContext)`. Its three hand-written `UiNode` JSON trees have to become `UiSnapshot`s loaded into a store (the pattern `🔬️engine-contract`'s `renderContractTree` uses). A real ~150-line story migration, not a type patch. |
| `🧪️space-artifact-creation-owner:3530` | 1 | `state.verifiedColdPair = { assertCurrent(){}, drop(){} }`. `VerifiedColdDocumentPair`'s constructor is mint-token-private and needs a `DocumentExecutionTargetLease` + `WireArtifactBootstrap`; `verifiedColdDocumentPairMintToken` IS already in the test dependency bag, so the honest fix is to mint a real one in that nested describe — it needs a lease harness that block does not have yet. |
| `🌊️flow/🕸️wasm/🌐️browser/📦️publication:53` | 1 | `write` on a `Bun.build` config. The repo's own `🌿️environment/🟦️.d.ts` declares it, bun-types declares it — the merged `Bun` namespace picks the wrong one, the same double-declaration trap as `Bun.spawn` in §31f. |
| `🏛️ShellHost:4923` | 1 | `consumes` is absent from the `plugins` PROP's entry type while `expandPluginRegistry`'s rows have it; the shell's registry element type has to be reconciled with the registry's own. |
| `🎥️tutorial-bridge:81` | 1 | The imported fixture's two snapshots disagree on which `activeUtilityByWindowId` keys exist, so TypeScript requires the union of both. Needs an annotated fixture type. |
| `🎥️world3d-camera-framing:41` | 1 | `FixtureCamera.projection` is `unknown`; the framing call wants `explicitProjection: boolean`. The fixture's own camera shape has to say which. |
| `🧑‍💻dev/🏗️builder/🌐️vite:24` | 1 | `defineConfig(async () => ({…}))` — the returned object is not a `UserConfig` (`worker.format: string`, a plugin array that is not `PluginOption[]`). The file should use the repo's own `defineOwnedBuildConfig`/`OwnedBuildConfig` contract, which is what every other owned build config uses. |

Not mine and left alone: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️scheduledemonstratoridle` ×2 (legacy).

### 32e. Files changed (session 5, both halves)

`🧰️framework/🛍️products/💻️os/tsconfig.json` (`DOM.AsyncIterable`) · `.vscode/launch.json` + `🧩️launch.seed.jsonc`
· `💻️os/📦️packages/🟦️typescript/📜️script.ts` · `🌉️mcp/📦️packages/🦀️rust/📜️script.ts` + regenerated
`🌉️mcp/🧬️schema/{🔣️.json,🟦️.ts}` · `🌊️flow/🕸️wasm/🌐️browser/📝️declaration/📤️projection/🟦️.ts` + regenerated
`🤖️generated/🟦️.d.ts` · `🧑‍💻dev/🔌️vite-plugins/🟦️.ts` · `🧑‍💻dev/📊️benchmarks/🔌️plugins/🌐️browser/🟦️.ts` ·
`🔌️plugin/{📇️registry/🗿️taxonomy-validation,🪟️window-kits/📝️text,🌐️browser-bundle/🌐️host}/🟦️.ts` ·
`🎯️targets/🧊️wgpu/{🐚️plugin-bridge,🚚️browser-frame-transport}/🟦️.ts` ·
`🧱️elements/{🌐️World3dHost,🏛️ShellHost,🔌️PluginRuntime,🛠️ShellHelpers,🪪️WasmSessionLoader,🗣️Interpreter}/🟦️.tsx` ·
`🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts` · `🧰️framework/🔨️modules/🖱️ui/🧱️elements/{🚗️UiDriver,🕸️Diagram}/🟦️.tsx` ·
three `📖️stories` metas · 18 os test modules · three `🎚️config`s · four `✏️s` plugin modules.

Scratch tooling added this session: `🐍️t4c-vite-probe.ts` (ticket folder).

