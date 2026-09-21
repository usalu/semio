# TreeItem Accessibility Label Fallback

## Defect and contract

Checkpoint 19's accepted General panel projected its six setting rows as unnamed `treeitem` nodes even though each typed `TreeItemProps` carried the visible label. The shared projection read only `AccessibilitySpec.label`; the browser mirror correctly exposed the empty projection it received.

The shared Rust and TypeScript projection now use the intrinsic `TreeItemProps.label` only when the explicit accessibility label is absent. `TreeSection` is unchanged. The schema-neutral accessibility fixture adds both required cases: an unlabeled TreeItem projects `Intrinsic row`, while a visible `Visible row` with explicit `Explicit override` projects the override.

## Accepted General authority

`accepted_general_tree_hugs_the_bottom_anchor_on_the_next_shell_panel_walk` already creates the actual `build_settings_general_ui` document through retained ingress, completes layout/paint, and seals plus acknowledges its hit/input registry. A test-only dump helper now reads that accepted accessibility revision, and the law requires these exact key/name pairs with role `treeitem`:

- `framework.settings.appearance` — Appearance
- `framework.settings.layout` — Layout
- `framework.settings.driver` — Driver
- `framework.settings.language` — Language
- `framework.settings.terminology` — Terminology
- `framework.settings.mergePolicy` — Merge policy

The browser parity journey also waits for Playwright's role/name locator for all six names after General publishes. This validates the accessible-name behavior through the real React DOM and WGPU accessibility mirror rather than matching raw attributes.

## Verification

The scoped command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace --nxIgnoreCycles=true -- bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📜️script.ts accessibility-projection-oracle
```

It passed 255 TypeScript twin assertions. Receipt: `🗑️generated/astra-runtime/accessibility-treeitem/ts-oracle.log`. All touched Rust sources parse through `rustfmt --edition 2021 --emit stdout`. Root owns the native gates:

- `every_published_record_projects_the_way_the_shared_fixture_declares`
- `accepted_general_tree_hugs_the_bottom_anchor_on_the_next_shell_panel_walk`

## Probe interference note

`bun --check` is not a syntax-only operation for the journey script and accidentally started a legacy parity probe. The owned PID 61868 was terminated immediately; no owned child remained. It may have written into the already-active `🗑️generated/w5b-parity` output, so no file from that run is a valid receipt. PID 59760 and its browser descendants were left untouched on root's explicit instruction. No production source was changed by the probe.
