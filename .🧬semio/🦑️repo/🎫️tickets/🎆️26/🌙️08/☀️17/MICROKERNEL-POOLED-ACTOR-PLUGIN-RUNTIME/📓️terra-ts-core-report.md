# 📓️ terra — `ts-core` verify + repair report

Packet: verify/repair `🧰️framework`, `🎭️actor`, `🎠️kernel` TS packages (pooled-actor runtime surface).
Owned writable paths only: the three packages' `📦️packages/🟦️typescript/**` trees, plus this ticket folder.

## 🧪️ Vitest — UNIQUE counts, `--reporter=verbose`, confirmed by name

All three commands run via `bun ./📜️script.ts test --reporter=verbose` (the target `nx:run-commands` invokes)
from each package's own directory. Full logs saved next to this report.

| package | command | result | exit |
|---|---|---:|---:|
| `@semio-tech/framework` | `🧰️framework/📦️packages/🟦️typescript` | **87 passed / 0 failed** (1 test file: `🟦️glue.ts`) | **0** |
| `@semio-tech/framework-actor` | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript` | **46 passed / 0 failed** (3 test files: `🧵️shard-client.ts`, `📬️mailbox.ts`, `🧵️turn-scheduler.ts`) | **0** |
| `@semio-tech/framework-kernel` | `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript` | **29 passed / 0 failed** (1 test file: `🟦️component.ts`) | **0** |

Matches (or, for actor, extends) the ticket's recorded baselines exactly: framework 87, actor 40→**46**,
kernel 29. No test vanished; no config double-counts (`include: []` + `includeSource`/`coverage.include`
pattern is correct in all three `🧪️vitest.config.ts`; kernel's uses a `*.ts` glob, actor's lists three
filenames but the directory's other two files, `🖼️wire-turn.ts`/`🧵️shard-runtime.ts`, carry zero
`import.meta.vitest` blocks — checked directly — so nothing is silently excluded).

**The 6 new host-effect-bridge tests in `🧵️shard-client.ts`, confirmed present by name:**
1. `ShardClient host-effect bridge — handler success > resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker`
2. `ShardClient host-effect bridge — handler error > a rejected onHostEffect settles as effect-error, never a hang`
3. `ShardClient host-effect bridge — no handler installed > fails FAST with an explicit effect-error, synchronously, never a silent hang`
4. `ShardClient host-effect bridge — backpressure cap > rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending`
5. `ShardClient host-effect bridge — shard-loss settlement > terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker`
6. `ShardClient host-effect bridge — shard-loss settlement > dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's`

## 🔎 Type-check

None of the three packages has a local `tsconfig.json` or an nx `typecheck` target (only
`test`/`test-quick`/`test-long`/`test-exhaustive` exist in each `📋️project.json`) — bun strips types at
runtime without checking them, so a type-only defect is invisible to `vitest`/`bun test` and to `--lib`
alike. Built a scoped synthetic tsconfig (`extends` the repo root `tsconfig.json`, `include` narrowed to
the three packages) to get a signal; saved at `terra-ts-core-tsconfig-scoped.json` for reuse. Root tsconfig
lacks `allowImportingTsExtensions`, so a bare run floods with `TS5097` on every `.ts`-extension import
repo-wide (a pre-existing, repo-wide gap, not actionable here) — ran with `--allowImportingTsExtensions`
added on the CLI to get past that noise; `import.meta.dir`/`ImportMeta` bun-global errors in `📜️script.ts`/
`🧪️vitest.config.ts` are the same kind of harness artifact (no `bun-types` anywhere in the repo) and were
left alone as out-of-scope infrastructure gaps, not code defects.

**After filtering both artifact classes out, the scoped check found real, in-scope defects — all in
`🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`** (actor and kernel packages: zero real defects beyond
the harness artifacts above). All three were type-only (bun's strip-only TS meant `vitest` stayed green
through every one of them), fixed, and re-verified at exit 0 for the owned-path source files:

1. **`PlayerEvent`/`RecorderEvent` test fixtures didn't satisfy `StatechartEvent`.** Both classes were
   missing the interface's `readonly eventCount: number` member (present on every sibling fixture —
   `UnitFlipEvent`, `CheckoutEvent`). Added `readonly eventCount = 5` (open/pause/play/stop/resume) and
   `= 3` (start/audioStop/videoStop) respectively, matching each class's own `IDS` cardinality.
2. **`PluginRegistryEntry`/`PluginSourceEvent` used as bare type annotations with no local binding.**
   `glue.ts` re-exports the kernel module via `export * from …`, which does NOT create local bindings for
   the re-exporting file's own use — the in-source `PluginSource` test block referenced both names
   unimported. Added them to the existing `import { …, type EphemeralBox, type PluginCatalog } from
   "…/🎠️kernel/🟦️component.ts"` block.
3. **`ActionArgDef` imported from the wrong module, and downstream test data stale against the current
   shape.** `ActionArgDef` is defined and exported by `…/🛂️manifest/🟦️component.ts`; the
   `…/🧮️action-argument-resolution/🟦️component.ts` module only does a private `import type` of it (never
   re-exports), so `import { …, type ActionArgDef } from ".../🧮️action-argument-resolution/…"` resolved to
   an unresolvable name — TS silently treated it as `any` downstream, masking the next defect. Split into
   two imports (value imports still from action-argument-resolution; `type ActionArgDef` from manifest).
   With the real type now enforced, the `effectiveActionArgs` test helper `textArg()` turned out to build
   a stale shape — `control: { kind: "text" }` — left over from before the D6 `ArgSchema`/`ArgPresentation`
   migration (documented at `manifest/🟦️component.ts:20-23`); the live shape needs a required
   `schema: ArgSchema`, not `control`. Replaced with `schema: { kind: "string", options: [] }
   satisfies ArgSchema`-shaped literal (`TEXT_SCHEMA` constant), imported `ArgSchema` alongside
   `ActionArgDef`.

Diff is 4 hunks, all inside `🟦️glue.ts`'s existing import block and its two in-source test fixture areas —
no production (non-test) code touched, no signature changed, no `.await`/async-tag surface involved (this
is pure TS, no R2/R7/R9 exception tags apply). Verified with `git diff HEAD -- "🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts"`.

**Post-fix scoped typecheck** (`terra-ts-core-typecheck.txt`): grepping for
`🧰️framework/📦️packages/🟦️typescript/|🎭️actor/📦️packages/🟦️typescript/|🎠️kernel/📦️packages/🟦️typescript/`
excluding `📜️script.ts`/`🧪️vitest.config.ts` (the two known bun-harness artifacts) → **zero matches**, i.e.
zero real type errors left anywhere in the three owned packages' source files.

**Post-fix vitest re-run** (`terra-ts-core-framework.txt`, same command as above): **87 passed / 0 failed,
exit 0** — unchanged from pre-fix, confirming the fixes were type-only and didn't touch runtime behavior.

## 📉 Regressions

None. All three packages hold their exact recorded/expected UNIQUE counts and exit 0 on both test and the
best-effort typecheck for owned-path files.

## 🎫 lease-request

None needed — everything stayed inside the owned paths.

## 🧭 For the coordinator / siblings

- **Gap, not a regression:** `🧰️framework`, `🎭️actor`, `🎠️kernel` TS packages have no `tsconfig.json` and no
  nx `typecheck` target. Type-only defects (like the three above) are invisible to both `vitest` and any
  `--lib`-style gate as long as bun's strip-only TS is the only thing that ever touches these files. The
  scoped synthetic tsconfig used here (`terra-ts-core-tsconfig-scoped.json`, needs
  `--allowImportingTsExtensions` on the CLI since the repo root tsconfig doesn't set it) is a workable
  stopgap; a real fix (per-package `tsconfig.json` + an nx `typecheck` target, `project.json`/root
  `tsconfig.json` being registrar-only files) is outside this packet's writable scope — flagging for
  whoever owns `📜️script.ts`/`project.json` conventions repo-wide.
- Repo root `tsconfig.json` lacks `allowImportingTsExtensions`; a bare `bunx tsc --noEmit -p tsconfig.json`
  floods with `TS5097` on essentially every first-party `.ts`-extension import. If the "19 pre-existing
  errors" repo-wide baseline recorded elsewhere in this ticket was measured without that flag, it is very
  likely stale/undercounted; if it was measured with an equivalent flag some other way, that mechanism
  should be written down somewhere discoverable (it isn't in `bunfig.toml`, `package.json`, or
  `tsconfig.json` today) — not re-litigated here since `tsconfig.json` is registrar-only.
