# Research: Duplicate Document Panel Toggle in Demonstrator Generator

## Root Cause Analysis

1. **Incorrect `panelTabKindId` call on String Constant in `ShellHost`**:
   - In [`ShellHost/🟦️component.tsx`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/ShellHost/%F0%9F%9F%A6%EF%B8%8Fcomponent.tsx#L2108), `panelTabKindId(FRAMEWORK_PANEL_TAB_ARTIFACT_ID)` was called on a raw string constant (`"framework.panel.artifact"`).
   - `panelTabKindId` expects an object matching the `PanelTabKind` union (e.g., `{ kind: "app", id: "framework.panel.artifact" }`). Passing a string returns `undefined`.
   - Consequently, `documentPanelKey` evaluated to `undefined`, preventing `panelUiByKey` from receiving the populated document panel node.

2. **Shallow Search in `hasPluginArtifactTab` Check**:
   - In [`ShellHost/🟦️component.tsx`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/ShellHost/%F0%9F%9F%A6%EF%B8%8Fcomponent.tsx#L3876), `pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_ARTIFACT_ID)` only checked top-level tab nodes. If an app defined panel tab definitions nested under branch nodes, top-level `tab.id` matching failed, causing `workbenchLeftTabs` to incorrectly prepend a fallback `artifactTab`, creating duplicate "Dokument" tabs.
   - Similarly, in [`Shell/🧊️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/Shell/%F0%9F%A7%8A%EF%B8%8Fcomponent.rs#L7291), `tabs.iter().any(|t| t.id() == FRAMEWORK_PANEL_TAB_ARTIFACT_ID)` performed a shallow top-level check instead of inspecting tab child trees recursively.

## Solution

1. Fix `documentPanelKey` in `ShellHost/🟦️component.tsx` to directly use `FRAMEWORK_PANEL_TAB_ARTIFACT_ID`.
2. Update `hasPluginArtifactTab` in `ShellHost/🟦️component.tsx` to use `flattenPanelTabLeaves(pluginLeftTabs)` for recursive leaf detection.
3. Update `has_document` in `Shell/🧊️component.rs` to use a recursive helper function `panel_tabs_contain_artifact` to inspect nested tab definitions.
