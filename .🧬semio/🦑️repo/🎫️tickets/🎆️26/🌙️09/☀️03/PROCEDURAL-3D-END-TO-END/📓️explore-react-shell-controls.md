# 🖱️ React shell controls and DOM markers for the runtime verification

Sources: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/{ShellHost,NodeGraph,World3dHost}/🟦️.tsx`.

| Thing | Marker |
|---|---|
| Example switcher | `Select` id `playground.navbar.fixture`, trigger button id `playground.navbar.fixture.trigger`; `onValueChange` → `setActiveExample` (ShellHost ~:5801) |
| Mode switcher | `ButtonGroup` id `playground.navbar.modes`, items `playground.navbar.modes.<modeId>`, active item `data-state="on"` (ShellHost ~:5827) |
| Flow window host | `div.semio-node-graph-host` with `data-surface-id`, `data-status-json`, `data-fixture-json`; empty state `div.semio-node-graph-empty`; nodes are `div.rounded.border…` with a `div.border-b` label child (SSR fallback), plus WebGL + label canvases |
| 3d preview host | `div.semio-world-3d-host` with `data-meshes-json`, `data-instances-json`, `data-status-json` (JSON arrays — mesh/instance counts readable from the DOM); empty state `div.semio-world-3d-empty`; `role="status"` aria-busy while computing |
| Rejected action | `console.error("[DEBUG] action failed", …)` (ShellHost ~:3820); toast `div[data-semio-transient-notice][data-notice-code="<fault code>"]` `role="status"` |
| Global debug handle | none (`window.__SEMIO__` etc. do not exist) |

Verification probe (javascript_tool): count `.semio-node-graph-host` nodes, parse
`.semio-world-3d-host[data-meshes-json]` / `[data-instances-json]` lengths, list
`[data-semio-transient-notice]` codes, and iterate the `playground.navbar.fixture` options to load each of
the 8 examples.
