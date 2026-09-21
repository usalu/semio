# Checkpoint 19 Tree-Row Accessibility and Active-Pane Audit

## Scope and evidence

This is a source and recorded-artifact audit only. No browser run, build, or production source was changed.

The current WGPU General snapshot publishes four unnamed \`treeitem\` nodes followed by six more unnamed tree rows whose child comboboxes expose only their values: \`system\`, \`desktop\`, \`default\`, \`en\`, \`native\`, and \`Normal\`. See [wgpu-settings-general.json](./🗑️generated/astra-runtime/checkpoint19-iab/wgpu-settings-general.json). The paired React General snapshot names those rows — Appearance, Layout, Driver, Language, Terminology, and Merge policy — as well as the app identity rows. See [react-general-aligned.json](./🗑️generated/astra-runtime/checkpoint19-iab/react-general-aligned.json).

This is a present, user-visible accessibility parity defect. A screen-reader user can operate the WGPU row's combobox but cannot learn which setting it changes.

The saved active-pane evidence does **not** establish a current styling defect. [wgpu-light-selected.jpg](./🗑️generated/astra-runtime/checkpoint19-iab/wgpu-light-selected.jpg) visibly highlights both the Settings branch and its General leaf; [wgpu-settings-general.json](./🗑️generated/astra-runtime/checkpoint19-iab/wgpu-settings-general.json) also reports \`tab "General" [selected]\`. The older settings captures are different panel states, so they are not a valid contrary pixel comparison.

## Causal path

The General producer supplies the visible row labels. \`settings_tree_select_item_with\` writes its \`label\` into \`UiTreeItemNode\` at [Shell WGPU source:5578](../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L5578), and \`build_settings_general_ui\` supplies Appearance through Merge policy using that helper at [Shell WGPU source:7901](../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L7901). The typed wire equivalent requires \`TreeItemProps.label\` at [component contract:523](../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs#L523).

The shared projection discards that intrinsic tree-row name. \`accessibility_projection_node\` uses only \`record.accessibility.label\` at [accessibility contract:231](../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs#L231), while its role/focus rules identify the same record as a focusable \`treeitem\` at [accessibility contract:79](../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs#L79) and [accessibility contract:104](../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs#L104). The WGPU projection walk does not restore a component label; it forwards this node unchanged at [WGPU accessibility projection:104](../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs#L104).

The browser mirror is behaving correctly for its input: it sets \`aria-label\` only when the projection has \`node.label\` at [accessibility mirror:79](../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts#L79). React retains the visible row text in its DOM, which is why the same General data receives names in the recorded React tree.

## Smallest coherent repair

Keep \`AccessibilitySpec.label\` as the authoritative explicit override. In the **shared contract** projection, give a \`Component::TreeItem(props)\` the fallback name \`props.label\` only when \`record.accessibility.label\` is absent. Do not put a WGPU-only workaround in the browser mirror, and do not copy labels into every Shell row producer.

This is a semantic-name rule, not an ARIA override: React's row already has that visible text as its accessible name; the WGPU canvas needs the projection to state it. It preserves an author-provided accessibility label, keeps all labels within the existing typed \`Label\` credit, and applies to app-provided tree rows as well as Shell settings.

Tree sections should be a separate decision. This evidence proves unnamed \`treeitem\` rows. It does not prove that a \`TreeSectionProps.label\` must name its \`group\` projection, so that broader fallback should not be bundled into the repair.

## Fail-first acceptance law

Add one shared/Native projection law that creates the actual General tree through \`ShellState::build_settings_general_ui\`, publishes it through the normal retained document ingress, waits for the accepted paint/ACK boundary, and reads the existing accessibility dump. It must require these exact \`treeitem\` names:

- Appearance
- Layout
- Driver
- Language
- Terminology
- Merge policy

It must also require the explicit-label precedence case: a TreeItem with visible label A and \`AccessibilitySpec.label\` B projects B. A TreeItem with no explicit label projects A. This prevents a future WGPU-only mirror change and pins the shared name rule.

Extend the existing browser parity probe at [parity-interact-probe.mjs](./🐍️parity-interact-probe.mjs) to require the same six named treeitems after it opens General. Its existing accessibility snapshot transport exercises the actual browser mirror; comparing only the combobox values would repeat the defect.

## Confidence

High for the tree-row loss: the current WGPU and React accessibility snapshots show the exact mismatch, and the source path has a direct label omission. High that the saved active-pane style is currently present. No conclusion is made about unrecorded panes or a fresh full checkpoint run.

