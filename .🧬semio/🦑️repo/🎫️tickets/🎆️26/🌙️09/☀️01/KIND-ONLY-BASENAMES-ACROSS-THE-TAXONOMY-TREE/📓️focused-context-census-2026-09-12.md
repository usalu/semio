# Focused Context Census

After the WGPU ordering corrections restored strict taxonomy loading, the coordinator called the real `inventoryTaxonomy` API read-only on three bounded active scopes, with one worker, progress and a ticket-local cancellation path. No normalization plan or mutation ran. These are contextual findings in the current shared tree, not a global result or a statement about historical baselines.

| Scope | Entries | Findings | Duration |
| --- | ---: | ---: | ---: |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements` | 337 | 21 | 5.112s |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements` | 358 | 105 | 6.013s |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization` | 39 | 13 | 3.465s |

UI elements has 21 directory findings. Two new semantic owners—Ports interactive jobs and Diagram layout—were sent to the active framework source executor for immediate exact-context closure. The remaining 19 include component test cases, the WGPU presence test, modal fixture and Nakagin fixture contexts. OS elements has 105 directory findings across actual component owners and descendant tests; resolve authoritative OS element membership rather than add an arbitrary PascalCase escape. The normalization scope has 12 directory findings plus one lexical opaque-target report.

The latter is a concrete source-as-data boundary: normalization `🧫️fixtures/🚪️source-admission/🔣️.json` line 213 has the string `compose` inside `input.opaquePrefixes` for case `opaque-prefix-is-repository-rooted-and-compose-is-casefolded-at-any-segment`. It represents a simulated candidate repository filter in the portable test; it is not an active repository import. Preserve this meaningful hostile/casefold fixture. Review exact fixture-schema reference roles instead of renaming the simulated token or exempting the entire fixture tree.

All exact findings follow so the disposable JSON files can eventually be removed without losing the audit.

## ui-elements

- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⌨️Command/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Checkbox/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/🔬️wgpu-unit` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧫️fixtures/♿️modal` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📻️TableAvatar/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔀️Toggle/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️tests/🧩️component` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧫️fixtures/🏢️nakagin` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧾️Form/🧪️tests/🧩️component` — Directory has no registered semantic kind

## os-elements

- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-catalogue-workflow-drop` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🧩️wgpu-engine-surfaces` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌳️GraphTimelineHost` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-block-list` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-diff-view` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-event-feed` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-icon-render` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-render-entry` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-text-editor` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-virtual-file-system` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🚪️ready-opening` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎟️invite-capability` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope/🌐️browser` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📇️directory-bootstrap` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🌐️browser` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🎥️tutorial` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🧫️fixtures/🖥️mounted` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🪪️catalog-authority` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🚪️ready-opening` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/👥️presence-scope` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/📇️directory-bootstrap` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔀️surface-switch` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔬️mounted-gis-map-probe-v1` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/⏩️seek` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🎥️tutorial/🧵️serial` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission/📄️document` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🪪️host-bootstrap` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🧫️fixtures/📍️scope` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🩺️fault` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-media-frames` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-retained-chrome-text-laws` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-document-retirement` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-pool-future` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📖️owned` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🧪️tests/📏️step-ceiling` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📡️EventFeedHost` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📤️SegmentedDownload` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/📡️backbone` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧳️packed-text` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔺️DiffViewHost` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪪️container-node-ids` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🛍️wgpu-app-catalogue` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧩️BlockListHost` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🧫️fixtures/⚙️properties` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🧫️fixtures/🗑️delete-space` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🎚️measure-controls` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/📌️panel` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧩️contributions` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🎥️tutorial-interaction` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/📌️panel-carriage` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🔁️plugin-availability-route` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🧩️contributions-push` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-unit` — Directory has no registered semantic kind

## library-normalization

- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission-io` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/💥️generic-stem-collision-resolution` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission` — Directory has no registered semantic kind
- `opaque-reference-target`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission/🔣️.json` — json /@value[194]@5871 lexically targets excluded compose
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission/🧪️io` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/📦️package-boundary-classification` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🚪️source-admission` — Directory has no registered semantic kind
- `directory-kind-unresolved`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🚪️source-admission/🧪️io` — Directory has no registered semantic kind
