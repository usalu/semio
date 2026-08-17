# 📓️ sol → terra packet brief (verbatim) — P9-shellstate-module

You are "terra", an executor on ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` in /Users/ueli/Documents/semio. Packet id: **P9-shellstate-module**. Model: Sonnet 5. Coordinator ("sol") is the main chat.

## 0. First action
Read in full, in order:
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📌️important.md`
- `…/📓️design-decisions.md`
- `…/📓️luna-shellstate-audit.md` — **this is your specification**; it classifies all 76 TS `ShellAction` variants, the ShellHost `useState`/`useRef` inventory, the wgpu shell state fields, the kernel effects that mutate shell state, and proposes a ~60–70 variant `ShellCommand` enum
- `…/📓️luna-testinfra-audit.md` §"Cookbook" (module/crate/package boilerplate, the four required test levels)
- `/Users/ueli/Documents/semio/CLAUDE.md`
Then save this brief verbatim as `…/📓️sol-P9-shellstate-packet.md`.

## 1. Owned writable paths (EXCLUSIVE)
```
🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/**            (entire new module — you create it)
.🧬semio/…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️sol-P9-shellstate-packet.md
.🧬semio/…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️terra-P9-report.md  (+ .txt scratch there)
.🧬semio/…/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️lease-P9-*.md
```
You do NOT edit: the React shell, the wgpu shell, `🎠️kernel`, root files, or anything else. Wiring the new module into the kernel glue and adopting it in the shells are LATER packets. Any change you need outside §1 is a `lease-request` file + a fenced block in your report.

## 2. Why this exists
Semantic UI state (which windows exist, what is focused, active mode/tool/utility, panel and dock layout, dialogs/overlays, selection, sync/merge state, user-visible prefs) currently lives three times over: a TS reducer in `🧱️elements/Shell/🟦️component.tsx`, ~22 `useState`s in `ShellHost/🟦️component.tsx`, and a Rust struct in `Shell/🧊️component.rs`. Nothing outside those files can observe or drive it, which is exactly why the OS is not LLM-first. This packet creates the **single source of truth**: a pure Rust `ShellState` + typed `ShellCommand` + total `reduce` function, with a TypeScript twin and shared fixtures proving the two agree. The shells and the MCP gateway all become clients of it.

## 3. Required result
New module `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/` containing:
- `🦀️component.rs` — Rust SSOT. `ShellState`, `ShellCommand`, `ShellEvent`, `ShellError`, and `pub fn reduce(state: &ShellState, command: &ShellCommand, now_ms: u64) -> Result<(ShellState, Vec<ShellEvent>), ShellError>`. Pure: no I/O, no clock (caller passes `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/`std::thread`/`SystemTime`/`Instant::now` — it must compile for native AND `wasm32-unknown-unknown` (the React host and wgpu-web will both run it). serde + `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` on every public type, matching how `🛂️manifest` does it. One `//#region` per type group.
- `🟦️component.ts` — the TypeScript twin reducer with the same semantics, importing the generated types.
- `🤖️generated/🟦️shell.ts` — ts-rs output (produced by a `typegen` target; commit what the target generates).
- `🧫️fixtures/*.json` — shared parity fixtures, each `{name, state, command, expected: {state, events}}` (or `{error}`), covering every `ShellCommand` variant at least once plus the tricky paths (focus after closing the focused window, panel path memory, dock reset, dialog stacking, mode↔tool mutual exclusion). **Both** the Rust tests and the TS tests load these same files — that is the mechanism that keeps the twins honest.
- `📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs,📜️script.ts,📋️project.json}` and `📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,🧪️vitest.config.ts}` per the cookbook. Crate name `semio-framework-os-shell`, package `@semio-tech/framework-os-shell`. All four test levels declared.
- `pub fn shell_capabilities() -> Vec<ShellCapability>` — one machine-readable descriptor per `ShellCommand` variant (`id` like `ui.window.focus`, title, description, the variant's JSON Schema via schemars, plus flags for whether it is observable-only). Define `ShellCapability` locally in this module (the gateway compiles it into its catalog later; do NOT depend on the gateway crate or on `🛂️manifest`).

## 4. Design constraints
- **Scope discipline**: include only rows the audit classified SEMANTIC. Render caches (UiNode trees, engagements, measures, label overlays) and transient pixel/drag state stay out — they belong to the renderer. If the audit left a row unclassified, decide, and record the decision with its justification in your report.
- **`ShellCommand` is the vocabulary the LLM will speak.** Names must be stable, self-describing and domain-shaped (`FocusWindow{window_id}`, not `SetActiveWindowId{value}`); no `Updatable<T>`/functional-updater payloads (that is a React idiom and cannot cross a wire); every payload must be plain serializable data.
- `reduce` is **total and pure**: same inputs → same outputs, no panics, invalid transitions return `ShellError` rather than silently no-oping. `ShellState.revision` increments on every accepted command; a rejected command does not change state.
- Keep the informal `shell.*` verb strings the wgpu shell already logs (`shell.windowClose`, `shell.applyNamedLayout`, `shell.panelToggle`, …) as the `ShellCapability.id`s where they correspond, so history rows and agent capabilities share one vocabulary. Cite in your report which existing verb maps to which variant.
- Reuse existing framework types by name where the audit shows they already exist (`WindowLayoutNode`, `Anchor`, `IconName`, `Locale`, `Terminology`, `MergePolicy`, `Conflict`, …) **only if** you can depend on them without pulling in a mid-rewrite crate — check first; if depending on `semio-framework` is clean, do it; if it drags in the contested plugin/kernel surface, define the minimal local mirror and say so explicitly in the report (do not silently duplicate).

## 5. Acceptance (FOREGROUND, paste output + exit codes)
```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-shell
CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-shell --target wasm32-unknown-unknown 2>&1 | grep -c "^warning"   # → 0
CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-shell --features typegen typegen   # or whatever the repo's typegen convention is — copy it from a sibling module
bun nx run @semio-tech/framework-os-shell:test-quick   (TS parity tests over the same fixtures)
```
Also paste a grep proving purity: no `wasm_bindgen|web_sys|winit|tokio|std::thread|SystemTime|Instant::now|std::fs|std::net` in `🦀️component.rs` outside doc comments.
The crate will not build via `-p` until sol adds it to the root `Cargo.toml` — **emit that lease-request in your first few minutes** (file `…/📓️lease-P9-cargo-member.md` + fenced block in the report: exact member line in sorted position, exact `[workspace.dependencies]` alias, and the root `package.json` workspaces entry for the TS package), keep working, then run acceptance once sol confirms. If sol has not confirmed by the time you are done, say so plainly and paste what you could run. Never invent results.

## 6. Hard rules
No git-modifying commands. No `ticket_close`/`ticket_reopen`/repo-MCP write tools. Nothing outside §1. **Never background a build** (no `&`, no `run_in_background`, no poll loop — use a long foreground timeout). Scratch `.txt`/`.md`/`.json` in the ticket folder, never `.log`. `[DEBUG] ` prefix on temp logs, removed before done. No claim of a passing test without pasted output + exit code. Never edit `AGENTS.md`. No compat shims, no deprecations, no migration paths — this is a greenfield SSOT.

## 7. Report
`…/📓️terra-P9-report.md`: baseline HEAD + SHA-256s of created files; the full `ShellCommand` variant list with, for each, the audit row(s) it subsumes (so sol can verify coverage); rows you deliberately excluded and why; the existing-verb → variant mapping; acceptance output verbatim; leases emitted; and a "what the shells must do to adopt this" section (the exact adapter shape the React reducer and the wgpu shell will need), which is the input to the later adoption packets.
