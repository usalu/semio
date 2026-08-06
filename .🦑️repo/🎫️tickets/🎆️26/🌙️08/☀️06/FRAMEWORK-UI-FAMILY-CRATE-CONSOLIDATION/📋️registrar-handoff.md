# 📋️ Registrar handoff — `FRAMEWORK-UI-FAMILY-CRATE-CONSOLIDATION` (W8b)

## 1. Root `Cargo.toml` — `[workspace] members`

```
Remove:
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/⌨️tui",
Add:
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust",
Keep unchanged:
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust",
```

## 2. Root `Cargo.toml` — `[workspace.dependencies]`

```
Remove:
    semio-framework-ui-wgpu = { path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu" }  # 15 refs
Add:
    semio-framework-ui = { path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust" }
Keep unchanged:
    semio-framework-ui-styling = { path = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust" }
```

## 3. Cross-owner dependent repoints (14 manifests, none editable by this ticket)

Old crates `semio-framework-ui-wgpu` / `semio-framework-ui-tui` no longer exist. Every dependent keeps
its local alias (`ui_wgpu` / `ui_tui`) so **no `lib.rs` call site changes**, but the `package`, `path`
and `features` fields move to the merged crate.

Feature mapping: `engine` → `wgpu-engine`, `terminal` → `tui-terminal`, `bindgen` → `tui-bindgen`,
`typegen` → `typegen` (unchanged). A dependent that previously took ui-wgpu with **default features**
must now pass `features = ["wgpu"]` explicitly — the merged crate's `default` is empty because the two
targets' dependency weights differ radically.

| Dependent manifest | Old line | New line |
|---|---|---|
| `🧰️framework/⚡️implementations/🦀️rust/Cargo.toml:20` | `ui_wgpu = { path = "../../🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu", package = "semio-framework-ui-wgpu" }` | `ui_wgpu = { path = "../../🔨️modules/🖱️ui/📦️packages/🦀️rust", package = "semio-framework-ui", features = ["wgpu"] }` |
| `🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust/Cargo.toml:25` | `…/🎯️targets/🧊️wgpu", package = "semio-framework-ui-wgpu" }` | `…/📦️packages/🦀️rust", package = "semio-framework-ui", features = ["wgpu"] }` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/⚡️implementations/🦀️rust/Cargo.toml:28` | same shape | same shape |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml:33` | `…, features = ["engine"], package = "semio-framework-ui-wgpu" }` | `…, features = ["wgpu-engine"], package = "semio-framework-ui" }` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/⚡️implementations/🦀️rust/Cargo.toml:52` | same shape | `features = ["wgpu"]` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/Cargo.toml:23` | same shape | `features = ["wgpu"]` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️implementations/🦀️rust/Cargo.toml:15` | same shape | `features = ["wgpu"]` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🦀️rust/🌍️world/Cargo.toml:14` | `features = ["engine"]` | `features = ["wgpu-engine"]` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust/Cargo.toml:24` | same shape | `features = ["wgpu"]` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml:54` | same shape | `features = ["wgpu"]` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:48` | same shape | `features = ["wgpu"]` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml:34` | same shape | `features = ["wgpu"]` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/Cargo.toml:27` | same shape | `features = ["wgpu"]` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml:39` | same shape | `features = ["wgpu"]` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/⚡️implementations/🦀️rust/Cargo.toml:19` | `ui_tui = { path = "…/🎯️targets/⌨️tui", features = ["terminal"], package = "semio-framework-ui-tui" }` | `ui_tui = { path = "…/📦️packages/🦀️rust", features = ["tui-terminal"], package = "semio-framework-ui" }` |

Path depth note: the merged crate sits two directory levels **above** the old `🎯️targets/<t>/` dirs, so
every relative `path` loses its trailing `/🎯️targets/🧊️wgpu` (or `/🎯️targets/⌨️tui`) segment and nothing else.

The `ui_wgpu`/`ui_tui` aliases must be kept — the merged crate's items live under `ui_wgpu::wgpu::…` /
`ui_tui::tui::…` only if the alias is dropped; with the aliases retained the dependents' existing
`ui_wgpu::component::…` paths break. **Registrar action required in the same pass:** either keep the
alias and add `use ui_wgpu::wgpu as ui_wgpu_root;`-style shims (not recommended), or — preferred —
rename the alias to `ui` and rewrite call sites `ui_wgpu::X` → `ui::wgpu::X`, `ui_tui::X` → `ui::tui::X`.
This is a pure prefix rewrite; the module trees under `wgpu::`/`tui::` are byte-identical to the old
crate roots.

## 4. nx / orchestrator

- Old projects `@semio-tech/ui-wgpu-rs` + `@semio-tech/ui-tui-rs` → **one** project `@semio-tech/ui-rs`
  at `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust` (targets: `test`, `test-quick`, `test-long`,
  `test-exhaustive`, `check-wasm`, `generate`, `check`).
- Root `📜️script.ts` `verify gate` already repointed by this ticket:
  `@semio-tech/ui-wgpu-rs:check` → `@semio-tech/ui-rs:check` (one line, ~669).
- `.vscode/launch.json` untouched — **it carries no ui-wgpu-rs/ui-tui-rs entries** (verified by grep), so
  no launch regen is needed for this rename.

## 5. Storybook (C9)

**No storybook regen is required for this ticket** and `.storybook/**` was left untouched:
`@semio-tech/ui-react` did **not** move here — it was already at its Shape V2 path
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` when this ticket resumed, and
`.storybook/scopes.ts`'s hand-curated `ui` scope already points there. Verified statically: all four
`sourceRoots` of the `ui` scope and both ui-react aliases (`@elements/ui/globals.css`,
`@semio-tech/ui-react/globals.css` in the `compose` scope) resolve to existing files.

The `ui` scope **stays in `HAND_CURATED_SCOPES`** rather than opting into M9's `GENERATED_SCOPES`: it has
four custom `aliases`, a cross-owner `sourceRoot` (`♾️infinite/🖼️canvas/🎨️react-renderer`) and custom
`storyGlobs` (co-located `🧱️elements/**/🧪️story.tsx` plus the legacy `stories/ui/**` glob) — exactly the
three documented reasons an entry stays hand-curated.

If a later pass does move ui-react, the regen command is:
`bun nx run @semio-tech/repo:storybook-scopes` (M9 generator; see
`26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`).

**Flagged, not fixed (other owners):** `.storybook/scopes.ts` has two now-dangling sourceRoots that are
*not* ui's — `🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript` (stale after the assets
family de-sandwich; belongs to `FRAMEWORK-ASSETS-FAMILY-DE-SANDWICH`) and
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📺️renderer-modules`.

## 6. Concurrent-session collision (read before applying)

`UI-ELEMENT-CO-LOCATION-RESTRUCTURE` was **actively writing inside `🖱️ui/📦️packages/🦀️rust/🎯️targets/`
during this ticket's session** (its scratch files timestamped 12:22–12:33 on 2026-08-06) — it split the
17 881-line `🧊️wgpu/📦️lib.rs` godfile into per-region `🦀️<region>.rs` files. That split is *compatible*
with (and built on top of) this consolidation: it preserved the `crate::wgpu::` prefixes and the
`wgpu-engine` feature name introduced here.

However it also **repeatedly recreated the old per-target `Cargo.toml`/`📋️project.json`/`📜️script.ts`
files this ticket deletes**, and flip-flopped the 3D dependency between `semio-s-3d` (the W7 s-modules
rename) and `kernel_3d_scene`. Before applying the member swap, re-verify:

```
test ! -e "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml"
test ! -e "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/⌨️tui/Cargo.toml"
```

**3D dependency:** the merged manifest currently declares
`kernel_3d_scene = { path = "…/🧊️3d/🎬️scene/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-3d-scene", optional = true }`
to match the source on disk at handoff time. When W7's `semio-s-3d` rename lands, this becomes
`semio-s-3d = { path = "…/🧊️3d/📦️packages/🦀️rust", optional = true }` and the `wgpu-engine` feature entry
`"dep:kernel_3d_scene"` becomes `"dep:semio-s-3d"` — two lines, source side already uses whichever name
that agent last wrote.

## 7. Commands still un-run (need a healthy workspace, i.e. post-registrar)

```
cargo check --workspace
cargo check -p semio-framework-ui --features wgpu-engine,tui-terminal,typegen
cargo build -p semio-framework-ui --target wasm32-wasip2 --features wgpu
bun nx run @semio-tech/ui-rs:test
bun ./📜️script.ts verify gate
```
