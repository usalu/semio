# 📓️ sol → terra — packet brief, verbatim

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in
`/Users/ueli/Documents/semio`. Packet id: **P10-react-shell**. Model: Sonnet 5.

## 0. First action
Read in full: `…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`; `…/📓️design-decisions.md`;
`…/📓️luna-shellstate-audit.md`; `…/📓️terra-P9-report.md` (**the `ShellState`/`ShellCommand` SSOT you
adopt — read its "what the shells must do to adopt this" section**); `…/📓️terra-P1b-report.md`
(**the `ShellBridge` frame table + the TS twin codec you speak**); `📋️master.md` §2.2–2.3;
`/Users/ueli/Documents/semio/CLAUDE.md`.
Also read the peer ticket's `📓️status.md` (`…/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️status.md`) —
**their W2 is complete**, H1 has just rewritten `ShellHost`, and `Shell/🟦️component.tsx`'s reducer may
have moved under them. Re-read every file from disk before editing and re-check
`git log --date=iso --oneline -3 -- <path>`.
Save this brief verbatim as `…/📓️sol-P10-react-shell-packet.md`.

## 1. State of the world (verified by sol)
The gateway is real and green: **160 Rust tests + 22 TS conformance tests**, 20 MCP tools, dual-era
stdio + Streamable HTTP, a working headless workspace, action prepare/preview/commit/undo with policy +
audit, and a shell-bridge WebSocket server with a Rust↔TS byte-identical frame codec.
`bun ./📜️script.ts dev mcp stdio os` launches it and `.mcp.json` now has a `semio` entry.
What is missing is the **last mile**: a live React shell that an agent can observe and drive.

## 2. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/AgentBridge/**      (new dir)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/AgentPresence/**    (new dir)
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/AgentApprovals/**   (new dir)
.🧬semio/…/📓️sol-P10-react-shell-packet.md, 📓️terra-P10-report.md, 📓️lease-P10-*.md, *.txt
```
**Registrar-only (lease, never edit):** `ShellHost/🟦️component.tsx`, `Shell/🟦️component.tsx`,
`⚛️react/📦️index.tsx`, root `📜️script.ts`, dev `📜️script.ts`, `.vscode/*`. These are also the peer
ticket's H1 territory — a lease is the only safe path.

## 3. Required result
Everything an agent needs to observe and drive the live React shell, built as **self-contained new
elements** that the shell mounts with one line (which you lease).

1. **`AgentBridge`** (`🟦️component.tsx`): a headless React component/hook that dials
   `ws://127.0.0.1:<port>/bridge?token=…` using the TS bridge codec from
   `🌉️mcp/🧵️bridge/🟦️component.ts` (already written and fixture-verified against Rust — **import it,
   do not reimplement it**). It must:
   - discover port+token from `~/.semio/agent/bridge-token` semantics as P1b implemented them (read
     its report; if the browser cannot read that file, take them from a dev-server-injected env/config
     value and say so),
   - send `Hello`, then publish `ShellState` snapshots and `ShellStatePatch`es,
   - receive `ShellCommand` frames and apply them via the `🖥️shell` reducer twin
     (`@semio-tech/framework-os-shell`), returning `ShellCommandResult`,
   - never block the UI thread on the gateway, and reconnect with backoff when the socket drops.
2. **`AgentPresence`**: a small indicator showing connected / working / idle plus the current
   invocation label, driven by `AgentPresence` frames.
3. **`AgentApprovals`**: the `os.agent.approvals` dialog — lists parked approval requests with the
   capability, its diff summary and its risk, and sends `Approval{decision: deny|once|session}` back
   over the bridge. This is the human-in-the-loop gate for destructive agent actions, so it must show
   **what** is being approved, not just a yes/no.
4. **Adoption seam**: since `ShellHost` is leased, provide a single exported mount (e.g.
   `useAgentBridge(...)` plus `<AgentPresence/>`/`<AgentApprovals/>`) whose insertion into `ShellHost`
   is a **one-to-three-line** lease diff, and write that exact diff in your lease file.

## 4. Constraints
- Follow the existing `🧱️elements/<Element>/🟦️component.tsx` co-location convention exactly (see any
  sibling element, e.g. `ShellSync` or `Dock`); include a `🧪️story.tsx` if that is the local
  convention.
- Accessibility and i18n are repo rules, not optional: every control needs an accessible name; all
  user-facing strings need English **and** German (`LocalizedLabel`/the shell's existing locale
  mechanism — copy how a sibling element does it).
- Do not duplicate the bridge codec or the shell reducer — import both.
- If the React shell genuinely cannot reach the bridge without a `ShellHost` change, say so and lease
  it rather than hacking around it.

## 5. Acceptance (FOREGROUND, paste output + exit codes)
```
bun nx run @semio-tech/framework-renderer-react:test        # (confirm the exact project name from its package.json first)
bun nx run @semio-tech/framework-os-shell:test-quick
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-mcp     # must stay 160 passed
```
Plus **component-level tests you write** for: codec round-trip through your bridge client, applying an
inbound `ShellCommand` to the reducer and emitting the result frame, and the approvals dialog rendering
+ dispatching a decision. A full browser e2e is packet P12's job, not yours — do not attempt it.
Note: their H1 just rewrote ShellHost, so `framework-renderer-react` has ~15 pre-existing failures
unrelated to you (their status names them). Establish that baseline FIRST by running the suite before
your changes, record it, and prove you added no new failure.

## 6. Hard rules
All of `📌️important.md`: no git-modifying commands; nothing outside §2 (lease instead); **never
background a build**; scratch `.txt`/`.md`/`.json` in the ticket folder; `[DEBUG] ` removed before
done; never claim an unrun result; never edit `AGENTS.md`; no compat shims. Add no new npm dependency
without a lease.

## 7. Report
`…/📓️terra-P10-report.md`: baseline HEAD + the pre-change test baseline; files created with line
counts; the exact lease diffs; how port/token discovery works in the browser and any compromise you
had to make; acceptance output; and a short "what P12 can now drive end to end" section.
