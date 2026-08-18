# 📓️ lease-request — P10 → ShellHost mount seam

**Requested by:** terra, packet P10-react-shell. **Target file (registrar-only, not edited by me):**
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
(also the peer ticket `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`'s H1 territory — last touched by their
H1 commit `d9542d156a`, confirmed via `git log --date=iso --oneline -3` immediately before writing this
file). **Status: not applied — sol/registrar to apply after review.**

## What this buys

Everything an agent needs to observe/drive the shell — `AgentBridge`'s `useAgentBridge()` hook,
`AgentPresence`, `AgentApprovals` — is fully built, self-contained, and tested inside my own owned
directories (`🧱️elements/AgentBridge/**`, `AgentPresence/**`, `AgentApprovals/**`). None of it does
anything for a real user until `ShellHost` calls the hook and mounts the two visual elements. This is
that seam.

## Exact diff

**1. Imports** — add alongside `ShellHost`'s existing sibling-element imports (e.g. next to the
`ShellHelpers`/`ShellSync` imports):

```tsx
import { useAgentBridge } from "../AgentBridge/🟦️component.tsx";
import { AgentPresence } from "../AgentPresence/🟦️component.tsx";
import { AgentApprovals } from "../AgentApprovals/🟦️component.tsx";
```

**2. Hook call** — add once, anywhere in `FrameworkOsShell`'s body after `shellSessionIdRef` is
initialized (audit `📓️luna-shellstate-audit.md` §2 line 1084: `shellSessionIdRef` — SEMANTIC, "shell
lifetime session id" — already exists, is stable, and is exactly the id `AgentBridge`'s `Hello` frame
needs):

```tsx
const agentBridge = useAgentBridge({ shellSessionId: shellSessionIdRef.current });
```

`useAgentBridge()` with no `config` option calls `discoverAgentBridgeConfig()` internally, which reads
`import.meta.env.VITE_SEMIO_BRIDGE_URL`/`VITE_SEMIO_BRIDGE_TOKEN` and stays `status: "disabled"`
(never throws, never blocks render) when neither is set — see
`.🧬semio/…/📓️terra-P10-report.md`'s "port/token discovery" section for why no live gateway exists to
dial yet regardless.

**3. Mount the two visual elements** — add next to the other always-mounted, prop-light utility hosts
in the final JSX return (e.g. right after `<TextSelectionContextMenuHost />`):

```tsx
<AgentPresence status={agentBridge.status} presence={agentBridge.presence} />
<AgentApprovals approvals={agentBridge.pendingApprovals} onDecision={agentBridge.resolveApproval} />
```

Total: 3 import lines + 1 hook-call line + 2 JSX lines. The hook-call and JSX lines are the actual
"one-to-three-line" integration surface the packet brief asked for; the 3 import lines are the
unavoidable per-file import overhead of a co-location-convention new element (same shape every other
sibling-element import in this file already takes).

## What is deliberately NOT in this diff

- No `package.json`/`vitest.config.ts` change — `AgentBridge`/`AgentApprovals` import the shell
  reducer twin and bridge codec via **relative paths** (`../../../../🖥️shell/🟦️component.ts`,
  `../../../../🌉️mcp/🧵️bridge/🟦️component.ts`), the same cross-module-tree pattern `ShellHost` itself
  already uses for `🟦️backbone-worker.ts` and `💻️os/🎚️config/…` — so no new workspace dependency or
  alias is needed anywhere for this seam to compile and run.
- No change to `ShellHost`'s existing `shellReducer`/`ShellState` — `AgentBridge` keeps its own
  `ShellState` mirror (`createDefaultShellState()`, reduced via `@semio-tech/framework-os-shell`'s
  `reduce()`) rather than replacing `ShellHost`'s live UI state. Full adoption (routing `ShellHost`'s
  own dispatches through `reduce()` so the bridge publishes the *real* live UI state) is P9 report
  §8.1's "adoption packet" work, out of this packet's scope and this lease's diff.

## Optional, non-blocking follow-up lease

`AgentBridge/🧪️component.test.ts`, `AgentPresence/🧪️component.test.tsx`,
`AgentApprovals/🧪️component.test.tsx` are co-located per convention but are **not** discovered by
`bun nx run @semio-tech/framework-renderer-react:test` today — that project's
`🧪️vitest.config.ts` sets `root` to the `⚛️react` package directory, which does not reach into
`🧱️elements/**` (confirmed empirically: the existing suite lives entirely in one file,
`⚛️react/🧪️index.test.ts`, and no `*.test.ts(x)` file exists anywhere under `🧱️elements/` today).
Wiring them in would need one more `test.include` glob entry in that config file, e.g.:

```ts
test: { include: ["**/*.{test,spec}.?(c|m)[jt]s?(x)", "../../../../🧱️elements/**/*.{test,spec}.?(c|m)[jt]s?(x)"] },
```

Not requested as part of this lease (kept separate since it is optional/non-blocking) — my acceptance
instead verifies these 3 files with a direct foreground `vitest run --root … --config …` invocation
(see `📓️terra-P10-report.md` §Acceptance for the exact command and pasted output: 51/51 passing).
