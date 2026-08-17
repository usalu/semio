# w5b — flow core relocation (os-flow → flow plugin): STOPPED, not performed

## Assignment
Determine whether relocating `semio-framework-os-flow`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust`) into the flow PLUGIN
(`✏️s/🔌️plugins/🌊️flow/🔨️modules/🫀️core/` or similar) is safely achievable, and if so, do it.

## Step 1 — the wgpu renderer's actual flow/DAG usage
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml:21`:
```
flow = { path = "../../../../../../🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
```
This crate's `[package.metadata.semio]` is `role = "framework"`, `id = "renderer-wgpu"` (Cargo.toml:10-12) —
i.e. it is explicitly tagged as framework-layer, not product/app layer.

The crate root is `📦️glue.rs` (path in Cargo.toml), but it has **zero** `flow::` usage itself — it only
mounts submodules from `../../../../🧱️elements/*` via `#[path]`. Grepping the whole `🧑️‍🎨️engine` tree for
`flow::` finds exactly one consumer file:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`:
```
11: use flow::{dag::dag_screen_to_world, FlowFixture, FlowHost};
...
460: fn note_widget_hit_at_screen(host: &flow::FlowHost, sx: f64, sy: f64) -> Option<(String, f64, f64)> {
461:    use flow::dag::DagNodeKind;
462:    let (world_x, world_y) = dag_screen_to_world(&host.dag, sx, sy);
463:    let node = host.dag.fixture.nodes.iter().find(|node| matches!(node.kind, DagNodeKind::Note { .. }) ...
```
plus several more call sites of `FlowHost`, `FlowFixture`, `dag_screen_to_world` (lines 35, 62, 180, 211,
219, 421, 568, 598, 861 — full list in file).

**Answer to step 4 (generic vs. domain-specific):** not extractable as generic. `FlowHost` is flow's own
session/document host type (`host.dag.fixture` — the flow document's own DAG fixture data), not a bare
graph/geometry structure. The codebase has *already* split out the actually-generic node-graph rendering
piece into its own framework module — the same Cargo.toml also depends on
`framework_surface_node_graph = { path = "...🗺️surface/📦️packages/🦀️rust", package = "semio-framework-surface" }`
(Cargo.toml:23) — so what's left directly wired to `flow::` (`FlowHost`/`FlowFixture`/`DagNodeKind`) is the
irreducible flow-domain remainder, not leftover generic geometry that could be moved alongside it.

## Step 2 — every consumer of `semio-framework-os-flow` workspace-wide
A plain `grep -rl "flow::"` is noisy (matches `workflow::`, `wfc::flow::`, unrelated `set_control_flow`,
etc. — confirmed false positives in `🧰️framework/🛍️products/💻️os/🦀️component.rs`,
`.../🖥️host/🦀️component.rs`, `.../🔨️modules/🏃️run/*`, `.../🔨️modules/🪐️space/🦀️component.rs`,
`🧰️framework/🔨️modules/🧮️math/...`, `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` — all `workflow::`/`wfc::flow::`,
an unrelated `OS` domain type and math's own WFC submodule, nothing to do with `semio-framework-os-flow`).

The reliable signal is the Cargo dependency itself — `grep -rl 'package = "semio-framework-os-flow"' --include=Cargo.toml .`:

**Inside the flow plugin's own tree (moves together, not a concern):**
`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (the flow plugin itself) and its 8 extensions
(`📐️brep`, `📖️dictionary`, `🧠️logic`, `🏗️bim`, `🔤️primitive`, `🧮️math`, `📃️list`, `🖍️draw`, `📝️text`).

**Outside the flow plugin's tree — genuine cross-boundary consumers:**
1. `/Users/ueli/Documents/semio/Cargo.toml` — root workspace alias declarations only (lines 136, 148, 153,
   154), not real usage.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml` —
   the **framework-role** wgpu renderer (step 1 above).
3. `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` — `procedural` plugin. Real, deep usage
   confirmed by grep inside the plugin (excluding `workflow::` false positives): `FlowEvalSession`,
   `with_process_flow_eval_session`, `CameraJson`, `Widget` (e.g. `flow::Widget::InputSlider`),
   `flow_host_with_session`, `flow_palette_catalogue_sections`, `FlowFixture` — across dozens of files
   under `🎛️apps/🧊️3d/` and `🎛️apps/◻2d/` (commands, modes, panels, config, presence, artifacts).
4. `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/Cargo.toml` — `forms` plugin, similar direct `flow::` usage
   in its `📦️glue.rs` and artifact schema/mutation files.
5. `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/Cargo.toml` and
   `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` — `playbook` plugin
   (this is the same crate whose `flow::playbook` submodule the prior `w5a-playbook-relocation` wave
   already investigated and stopped on — see below, this generalizes that finding to the whole crate).

So the crate has real, heavy consumers in **4 separate plugins plus one framework-role crate**, none of
which are inside the flow plugin's own tree.

## Step 3 — would moving os-flow itself create a circular edge?
`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` `[dependencies]`: `semio-framework-os-kernel`,
`semio-framework-os-infinite`, `semio-framework`, `neural_engine`, `math` (`semio-framework-math`),
`ui_styling`, `ui_wgpu` (`semio-framework-ui`), `semio-framework-3d`, `semio-framework-2d` — all
framework-tree crates, all in the correct direction (plugin → framework) regardless of where os-flow
physically lives. `[dev-dependencies]` point at the flow plugin's own extensions
(`semio-s-plugin-flow-extension-{brep,primitive,math,text,logic,dictionary,list}`), which would already be
siblings inside the plugin tree post-move — no new edge there either.
**Conclusion: os-flow's own dependency list does not create a cycle if relocated.** The problem identified
below is entirely about the *consumers* (step 2), not os-flow's own deps.

## Step 4 — the actual, ticket-authoritative blocker
`.dependency-cruiser.cjs` (the layering-enforcement mechanism this very ticket's Wave 1 authored) has a
rule for exactly this direction, `frameworkNoSRule` (lines 170-184):
```js
/** 🧱️ `framework-no-s` (W1 of `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`): `🧰️framework` must not
 * import `✏️s` app/plugin/module code — framework is the substrate `✏️s` builds on, never the reverse.
 * WARN, not error: real violations of this direction exist elsewhere in the codebase and are this
 * initiative's job to clear in a later wave, not this ticket's; promote to error once they're gone. */
function frameworkNoSRule() {
  return {
    name: "framework-no-s",
    severity: "warn",
    from: { path: "^🧰️framework/" },
    to: { path: ["^✏️s/"].concat(S_PACKAGES.map((p) => `^${escapeRegex(p.name)}$`)) },
  };
}
```
The wgpu renderer crate is unambiguously under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/...` and
is explicitly `role = "framework"`. **Today its dependency on `semio-framework-os-flow` is compliant**
(framework → framework, since os-flow currently lives under `🧰️framework/`, and its own
`[package.metadata.semio]` tags it `role = "product"`). Physically moving the crate to
`✏️s/🔌️plugins/🌊️flow/🔨️modules/🫀️core/` would turn that same edge into `🧰️framework/ → ✏️s/🔌️plugins/🌊️flow/`
— a brand-new instance of the *exact* violation `framework-no-s` exists to eliminate, introduced by this
ticket's own wave into this ticket's own layering rule. It is currently WARN (staged, not ERROR) precisely
because *pre-existing* violations of this direction are still being cleared elsewhere — the rule's own
comment is explicit that new instances are not the intent ("promote to error once they're gone"), not an
invitation to add more while they're being cleared.

(By contrast, the `procedural`/`forms`/`playbook` plugin consumers are *not* blocked by tooling: the
`crossPluginRules` cross-plugin-import ban explicitly carves out `flow` — `if (from === to || to === "🌊️flow") continue;`,
line 87 — so plugin → flow-plugin is pre-sanctioned, "media-graph canvas embed" per the doc comment at
lines 81-82. The renderer is the one consumer this move cannot satisfy.)

## Precedent
This mirrors `📓️w5a-playbook-relocation.md` (STOPPED, zero edits) almost exactly, one layer up: that wave
found os-flow's own `vcs` component needing playbook's module internally (same-crate) plus 32 external
files across `procedural`/`forms`/`flow` depending on `flow::playbook` directly, concluding "there is no
version of this move that doesn't introduce a new layering edge somewhere." This wave finds the same shape
for the *entire* os-flow crate, made worse by involving a framework-role crate (not just plugin-to-plugin):
moving the whole crate satisfies none of its non-flow-plugin consumers without creating a new backwards
edge — for `procedural`/`forms`/`playbook` the edge happens to already be pre-authorized by tooling, but
for the wgpu renderer it is squarely the violation this ticket exists to remove.

## Decision
**STOP. Zero edits made.** Confirmed via `git status --short` before and after this session: no changes
under `🧰️framework/…/🌊️flow/`, `✏️s/🔌️plugins/🌊️flow/`, or the wgpu renderer/EngineCanvas paths from this
task.

## Recommendation for whoever picks this back up
Same shape as w5a's recommendation, at the crate level: this needs an architecture-owner decision first —
either (a) the wgpu renderer's `EngineCanvas` flow-embed is accepted as a second, renderer-specific
exemption alongside the existing plugin-to-flow one (i.e. `frameworkNoSRule`'s `to` pattern gets a
`flow`-shaped carve-out mirroring `crossPluginRules`' `to === "🌊️flow"` continue, explicitly endorsing
framework → flow-plugin for this one case), or (b) `FlowHost`/`FlowFixture`/the `dag` module the renderer
actually touches get pulled into a genuinely-generic, framework-owned module (`🗺️surface/🕸️node-graph`,
which the renderer already depends on for the rest of its graph rendering) so the renderer keeps a
framework→framework edge and only the flow-domain-specific remainder (evaluation session, palette
catalogue, playbook, etc.) moves into the plugin. Only once that's settled does "move the crate + fix N
call sites" become a well-scoped, non-layering-violating task.

## Files touched
None.
