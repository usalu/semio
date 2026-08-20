# 📌️ Binding rules — SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY

**Empty this file before `ticket_close`.**

Sibling program: `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` (the "U-program") is LIVE with ~13
concurrent agents. Its `📌️important.md` binds us too except where U1/U2 below override it. Read it.

---

## U1 — OWNER RULING 2026-08-20: the new UI crates use LITERAL `fn`, not `async fn`

The owner was shown the conflict directly (U-program **R2** demands the literal `async` keyword on
every first-party fn; this program's approved plan makes frame phases synchronous) and chose
**literal sync `fn` in the UI crates**. This is an owner decision and it **supersedes R2 for the
crates listed below** — it is not a packet-level deviation and not re-litigable by any packet.

Scope of the override — exactly these:

- `semio-framework-ui-contract`
- `semio-framework-ui-runtime`
- `semio-framework-ui-render` and its four backend targets
- `semio-framework-ui-host`
- the new `os_host` / `kernel_seam` / `deadlines` / `winit_app` regions in the OS renderer crate

Everything OUTSIDE that list still obeys R2. When our code calls into U-program crates, the call site
is at a boundary and may be `async` — the override covers *our* function declarations, not theirs.

**What must be sync (the whole point):** semantic composition, reconciliation and patch generation,
`apply_patch`/`validate_snapshot`, layout, prepaint, paint, `Scene::finish`, hit-testing, capture/
target/bubble dispatch, entity lease bodies, `transact()`, `build_frame()`. A frame is a
run-to-completion transaction; no suspension point may exist inside it.

**What stays async:** the outer event loop, GPU device/adapter construction, transport, actor
mailbox round-trips, asset and font loading. `spawn_local` hands a future to the embedder's executor;
the runtime itself never awaits.

Tag every sync fn in these crates:
`// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md`
Unavailable dependencies are `Measurement::{Ready,Pending,Failed}` + invalidation — **never** a
mid-frame await, and never a `block_on` (U-program R4 never sanctions the winit thread or any wasm
host path; that prohibition still stands and we do not want a bridge here anyway).

## U2 — OWNER RULING 2026-08-20: full speed, absorb the peer's working tree

The owner chose **full speed on everything** knowing the U-program owns the guest SDK, the WIT
schema, the plugin fleet and the parity harness right now, and that `semio-framework-plugin --lib`
is currently EXIT 101 mid-rewrite. Consequences, binding:

- **The working tree is the baseline, never `HEAD`.** Re-read every shared file from disk immediately
  before each edit. Surgical region-scoped edits only; never a full-file rewrite of a shared file.
- **Absorb, never delete, peer work** you find in a file you are editing. If you cannot preserve it,
  stop and report rather than dropping it.
- Packets whose acceptance depends on a peer-red crate report `blocked-external` with the exact
  error. Do not fix their crates.

## U3 — zero `dyn` on first-party traits (U-program O1/R1/R11 — NOT overridden, we comply)

`dyn T` for a first-party trait `T` is banned. `dyn Future`/`Fn`/`Any`/`Error` remain permitted per
R1. Apply R11's own procedure — and note it produces a *better* design here anyway:

| seam | set | resolution |
|---|---|---|
| `GraphicsBackend` | exactly ONE impl compiles per target (webgpu\|metal\|d3d12\|vulkan are cfg-exclusive) | **concrete type behind a cfg'd type alias** `pub type ActiveBackend = …;` — no enum, no box |
| `KernelSeam` | one impl per platform (native / wasm) | concrete type behind a cfg'd alias, or a generic parameter on `OsHost` |
| `CommandSink` | open-ish | generic parameter on the gateway |
| `Surface` | closed first-party set | enum dispatch (`dyn_enum_close!` if the macro fits) |
| `Element` | open, and type-erased per frame | erasure via **fn-pointer vtable in the frame arena**, not `dyn Element`; fn-pointer slots are E4-class and language-fixed |

`#![allow(async_fn_in_trait)]` (U-program R7) is irrelevant to us — our traits are sync.

## U4 — the coordinator owns every build (U-program rule 23, measured; adopted verbatim)

Executors **write code and reasoning, run only cheap non-cargo checks, and mark acceptance UNRUN**.
`sol` runs every gate and pastes the numbers. Reasons, all measured in this tree: the Bash tool
auto-backgrounds at ~120 s, a subagent's detached job cannot report across its turn boundary, and
with the U-program's ~13 agents also building, even a 600 s timeout will not finish a wgpu build.
Five peer packets in one wave ended a turn idling on a detached build; ~1.4M tokens bought nothing.

- Cargo target dir goes in the **session scratchpad**, never the ticket folder — a build with
  `CARGO_TARGET_DIR=<ticket>/…` fails with EPERM on build-script output (U-program rule 24).
- Never bare `--workspace` cargo. Always `-p <crate>`, and run **both** `--lib` and `--all-targets`
  (U-program rule 26: each flag has hidden a different real defect here).
- Every cargo command carries an explicit `timeout` of 600000 ms (U-program rule 19).

## U5 — gates are per-crate; the workspace is externally RED

Measured at anchor `5e7b8046be`: `semio-framework-number` (lib, 620 errors) and
`semio-framework-actor` (lib test, 499 errors) fail from the U-program's asyncify pass. Our W1
dependency footprint (`semio-framework-ui-styling`, `semio-framework-ui`, `semio-framework-geometry`)
is GREEN. Therefore: no gate in this program may be `cargo check --workspace`. "No worse than
baseline" is measured against that **named failing set**, never against zero, and never against a
count (U-program rule 11: baselines are named sets).

## U6 — packet slugs are descriptive, never letter-numbers

U-program rule 15: `A1`/`H2`/`P1`/`M1`/`R1`/`G1`/`W0` all collide with ids that ticket already used,
and one packet nearly overwrote a finalized report. Our slugs:
`contract-doc` `contract-layout` `contract-action` `contract-builder` `contract-scene-move`
`render-scene` `shader-repair` `backend-iface` `render-frame` `render-text` `render-dispatch`
`render-elements` `render-surface` `ui-host` `runtime-entity` `runtime-present` `runtime-reconcile`
`runtime-gateway` `runtime-transact` `backend-webgpu` `backend-metal` `backend-d3d12`
`backend-vulkan` `conformance-harness` `wit-flip` `host-bridge` `sdk-flip` `fleet-*` `os-host`
`legacy-delete`. Reports are `📓️terra-<slug>-report.md`, audits `📓️luna-<topic>-audit.md`.

## U7 — registrar-only files (sol edits these; everyone else emits a `registrar-request` and stops)

Union of ours and the U-program's: root `/Cargo.toml`, `/Cargo.lock`, `/📜️script.ts`,
`/📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`,
`🦑️repo/…/🔣️taxonomy.json`, all `🤖️generated/**`, `🔌️plugin/…/📇️registry/**`,
`🛂️manifest/🟦️component.ts`, `🔌️plugin/🧬️schema/📜️component.wit`,
`Shell/🧊️component.rs`, `ShellHost/🟦️component.tsx`.

## U8 — hard prohibitions (every agent)

1. **No git-modifying commands.** No `commit`, `stash`, `checkout`, `reset`, `add`, `worktree`.
   Other sessions are live and an auto-commit bot runs. `git status` is not a churn detector — use
   `git log --oneline -3 -- <path>`.
2. **Only `sol` calls `ticket_open`/`ticket_close`/`ticket_reopen`.** A subagent closing this ticket
   closes the whole program.
3. **Never edit outside your packet's OWNS list.** A region name inside a shared file is not
   ownership.
4. **Never claim a check passed without its output and exit code.** `cmd | tail -N; echo $?` reports
   *tail's* exit code (U-program rule 10) — read the summary line or drop the pipe.
5. **`🧱️elements/<Element>/` co-location dirs and taxonomy leaf files are never inlined or merged.**
   If a consolidation pass re-inlines one, re-split it (repo MCP `section_extract`/`section_move`).
6. Scratch files are `.txt`/`.md`/`.json` **inside the ticket folder** — never `.log` (repo-wide
   gitignored; `ticket_close` silently drops them).
7. `[DEBUG] ` marks genuinely temporary lines only, and they are removed before a packet reports
   done. 312+ permanent operator diagnostics in this repo already carry the prefix — never
   blanket-strip it.
8. A negative result from a query that cannot report its own failure is not evidence of absence
   (U-program rule 21). Emoji-path globbing has silently under-reported; confirm with a second,
   differently-implemented tool where a negative would change a judgement.

## U9 — environment

- **Disk: 77 GiB free of 926 GiB (92 %) at open.** The U-program filled this disk once already.
  `sol` checks `df -h` at every wave start and asks the owner before deleting anything.
- Installed targets: `aarch64-apple-darwin`, `wasm32-unknown-unknown`, `wasm32-wasip1`,
  `wasm32-wasip2`, plus `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu` added in W0 for the
  D3D12/Vulkan compile-only cross-checks.
- Parity numbers are currently **worthless as baselines** — the U-program's `parity-rebaseline`
  packet found the 58-variant suite has been booting react-new against wgpu-old, a cross-architecture
  diff rather than a regression check. Do not gate on parity until that packet lands.
