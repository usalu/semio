# 🎮️ W14e — flow, note and puzzle2d on the wgpu renderer

Packet W14e of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Every claim below was produced by a live
boot on this machine unless it is explicitly marked NOT RUN. Logs, dumps and screenshots live under
`🗑️generated/` with the `w14e-` prefix.

Machine conditions for the whole packet: load average 90–175, swap 35 GB of 35 GB used, 8–20 peer
`rustc` processes. Every nx invocation needed `NX_DAEMON=false` (see §0.1) and several dev serves died
under memory pressure mid-probe.

---

## 0. Infrastructure findings that block anyone booting a playground right now

### 0.1 `nx` daemon times out on this machine — every activation needs `NX_DAEMON=false`

`bun nx run @semio-tech/framework-os-dev:activate-flow-wgpu-dev` sat in
`Calculating the project graph on the Nx Daemon` for 16 minutes and then died with

```
NX   The daemon timed out while processing REQUEST_PROJECT_GRAPH
error: script "nx" exited with code 1
```

(`🗑️generated/w14e-flow-activate-1.txt`). The same command with `NX_DAEMON=false` reaches
`Running target … and 37 tasks it depends on` in about three minutes
(`🗑️generated/w14e-flow-activate-2.txt`). This is the `project-dev-boot-repo-walks` family under peer
load; nothing in this ticket's source causes it, but every packet that activates should pass
`NX_DAEMON=false` while the machine is loaded.

### 0.2 Peer dev serves on the canonical React ports are not usable as twins

- `6080` (note React, peer's) answers 200 but every module request returns
  `504 (Outdated Optimize Dep)`: its vite dep cache directory
  `.🧬semio/🦑️repo/⚡️cache/vite/os-dev/note-react-dev/` **no longer exists on disk** while the serve
  still hands out `?v=f9854e65` URLs. Page renders nothing (`canvases: []`, `innerText: ""`).
  Evidence: `🗑️generated/w14e-note-react/requests-failed.txt` (first capture).
- `6012` (puzzle2d React, peer's) was listening at the start of the packet and was gone 20 minutes
  later.
- `6016` (flow React, peer's) serves, but the plugin aborts at boot — see §1.1, which is a real
  product bug, not a serve problem.

A twin must therefore be started by the packet that needs it; `bun ./📜️script.ts serve <variant> react dev`
with `S_OS_PORT` set, run from
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`, bypasses the nx graph entirely
and is the cheapest way to get one.

---
