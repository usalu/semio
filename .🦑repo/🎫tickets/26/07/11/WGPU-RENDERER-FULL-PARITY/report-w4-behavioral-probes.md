# W4 Behavioral Probes — parity harness gains a probe runner

Scope: `framework/product/os/dev/script.ts`'s `#region 🔬ParityScript` only. Added a new
`#region 🔖ProbeCatalog` sub-region (between `🔖Triage` and `🔖ServerPool`), wired it into
`verifyParityVariant`/`writeParityReport`, and added a standalone `parity probe` CLI subcommand.

## What was added

**`ProbeStep` DSL**: a typed discriminated union of interaction steps — `click(path)`, `type(text)`,
`key(combo)`, `dragTo(fromPath, toPath)`, `wheel(path, deltaY)`, `settle(ms)`, `expect(predicate)`.
Predicates: `exists`/`absent` (path/kind substring match), `focus` (focusPath match), `text` (node
text equals), `custom` (`(dump: ParityDump) => boolean`). `click`/`dragTo`/`wheel` resolve coordinates
from a dump pulled from that same page immediately before acting — never cached or cross-renderer.

**Probe runner**: `runParityProbe(reactPage, wgpuPage, steps)` executes each step on both pages in
lockstep (`Promise.all` per step), then diffs a fresh dump from both with the *existing*
`compareParityStructural` after every non-`expect` step; `expect` steps evaluate predicates against
fresh dumps from both sides. First `FAIL` halts the run, remaining steps marked `SKIP`. Returns the
full `{status, steps: ProbeStepResult[]}` trail with per-step `structural` (mismatch axis visible)
and `detail`. `runParityProbeSuite` wraps it with a suite `name`.

**Starter catalog**: `PARITY_SHELL_PROBE_SUITE` — open command palette (`Control+p`, matches
`useActionHotkey("mod+p", ...)` in `framework/renderer/react/index.tsx`), settle, expect overlay
exists, `Escape`, settle, expect gone. Registered in `PARITY_PROBE_CATALOG` for extensibility.
Per-playground suites left as a documented follow-up.

**Wiring**: `ParityPlaygroundReport` gained `behavioral?: ProbeRunResult`. `verifyParityVariant` now
runs the shell suite *regardless* of structural/pixel outcome (documented reasoning: behavioral
parity is a distinct axis from static end-state), wrapped in try/catch so a runner exception degrades
to a diagnosable FAIL rather than crashing the whole call. `writeParityReport` gained a `Behavioral`
markdown column (JSON picks it up automatically). `ParityVerifyScript`/`ParitySweepScript` logs and
`failed` filters now include `behavioral`. New `ParityProbeScript` + `parity probe <variant>
[suiteName]` CLI subcommand for running just the probe suite standalone. No `launch.json` changes —
existing parity subcommands have none either.

## A real gap found and flagged, not fixed

Read `openStudioE2eCommandPalette` and `UISearch` in `framework/renderer/react/index.tsx`. Confirmed:
React's command palette is framework chrome via shadcn/cmdk (`[role='dialog'] [data-slot='command-
input']`), **not** `UiNode`-declared content — it never carries `data-ui-path`, so
`REACT_DOM_DUMP_SCRIPT` can't see it. The shell suite's `exists`/`absent` checks are therefore
expected to be unreliable (likely FAIL) on the react side until the structural dump is extended to
tag framework-chrome overlays (would also need mirroring into `framework/renderer/wgpu/rs/lib.rs`'s
Introspection walk — a different file, out of this ticket's scope as originally split). Documented
prominently in-code rather than silently patched.

## Verification performed

- `bun build --target=bun --external playwright --external pixelmatch --external pngjs
  framework/product/os/dev/script.ts --outdir=/tmp/parity-probe-check` — bundles clean (154.33 KB, 4
  modules).
- `bunx eslint framework/product/os/dev/script.ts` — 5 errors, all pre-existing/unrelated (unused
  `basename`/`resolve`/`fileURLToPath`/`wasmFileName`, one `no-useless-assignment` in
  `runStudioE2eVerify`). Zero new lint errors.
- No live browser run: every checked W1 boot-triage variant was blocked at the boot-triage rung by
  shared, actively-churning bugs outside this file's scope. No variant confirmed stably live-bootable
  this session, so this pass stopped at static verification.

## Files touched

`framework/product/os/dev/script.ts` — `#region 🔬ParityScript` only (`ParityPlaygroundReport` type,
new `🔖ProbeCatalog` sub-region, `writeParityReport`, `verifyParityVariant`, `ParityProbeScript`,
dispatch table). No other files created or modified.

## Follow-up needed before this axis is truly load-bearing

Extend `data-ui-path`/the wgpu introspection walk to cover framework-chrome overlays (command
palette, dialogs, tooltips) so the shell probe suite's `exists`/`absent` checks are meaningful on the
React side, not just the wgpu side.
