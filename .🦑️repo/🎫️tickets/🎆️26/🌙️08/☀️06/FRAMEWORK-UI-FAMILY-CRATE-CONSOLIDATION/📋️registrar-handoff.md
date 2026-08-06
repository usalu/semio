# 📋️ Registrar handoff — `FRAMEWORK-UI-FAMILY-CRATE-CONSOLIDATION` (W8b)

## 1. Root `Cargo.toml` — `[workspace] members` — **DONE**

Live root already lists:
```
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust",
```
Old per-target member lines are gone. Per-target `Cargo.toml` files must stay absent.

## 2. Root `Cargo.toml` — `[workspace.dependencies]` — **DONE**

```
semio-framework-ui-styling = { path = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust" }
semio-framework-ui = { path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust" }
```
Old `semio-framework-ui-wgpu` workspace alias is gone.

## 3. Cross-owner dependent repoints — **DONE**

Dependents keep aliases `ui_wgpu` / `ui_tui` with `package = "semio-framework-ui"`.
Feature map: `engine`→`wgpu-engine`, `terminal`→`tui-terminal`, `bindgen`→`tui-bindgen`, `typegen` unchanged.
Merged crate `default = []` — pass `features = ["wgpu"]` (or `wgpu-engine` / `tui-terminal`) explicitly.

## 4. nx / orchestrator

- One project `@semio-tech/ui-rs` at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust`.
- Root verify gate: `@semio-tech/ui-wgpu-rs:check` → `@semio-tech/ui-rs:check`.
- `.vscode/launch.json` — no ui-wgpu-rs/ui-tui-rs entries; no regen needed.

## 5. Storybook (C9)

No storybook regen — `@semio-tech/ui-react` already at Shape V2; `ui` scope stays hand-curated.
Flagged elsewhere: dangling assets + renderer-modules sourceRoots in `.storybook/scopes.ts`.

## 6. Shape V2 / feature layout vs spicy-umbrella

Plan: one crate with features `tui` / `wgpu` / `typegen`. Live matches and adds namespaced
subfeatures from the old split crates (**intentional**):

| Plan / old | Live |
|---|---|
| `tui` | `tui` |
| ui-tui `terminal` | `tui-terminal` |
| ui-tui `bindgen` | `tui-bindgen` |
| `wgpu` | `wgpu` |
| ui-wgpu `engine` | `wgpu-engine` |
| `typegen` | `typegen` |

**Multi-target layout (intentional):** sources under `🎯️targets/{tui,wgpu}/`, feature-gated via
`#[path]` from `📦️packages/🦀️rust/📦️lib.rs` — plan exception ("features beat separate modules").

`build = "build.rs"` on the **parent** package only; generated axes committed under the wgpu target.
Do not restore `build` on a target manifest.

**3D dep:** `kernel_3d_scene` → `package = "semio-s-3d"` at s-modules 3d packages path; optional via `wgpu-engine`.

## 7. Finish-pass verification (2026-08-06)

Root `cargo check -p` blocked by unrelated duplicate `semio-framework-editor` (packages +
implementations both members) — outside this ticket.

Isolated overlay on merged `semio-framework-ui` (optional 3d dep stripped only for resolve, restored):

| Features | Result |
|---|---|
| `tui` | ✅ |
| `wgpu` | ✅ (pulls styling) |
| `typegen` | ✅ |
| `tui-terminal` | ✅ |
| `tui,wgpu,typegen` | ✅ |

Fixed `🎯️targets/{tui,wgpu}/📦️lib.rs` element `#[path]`s after UI-ELEMENT emoji folder rename.
Logs: `verify/recheck-*.txt`.

## 8. Remaining non-UI blockers

1. Resolve duplicate `semio-framework-editor` so root `-p semio-framework-ui` / styling works.
2. Aliases `ui_wgpu`/`ui_tui` already kept — no call-site rewrite required.
