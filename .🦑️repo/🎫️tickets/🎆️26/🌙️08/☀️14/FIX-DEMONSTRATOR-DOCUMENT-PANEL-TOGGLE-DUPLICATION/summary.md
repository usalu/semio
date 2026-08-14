# Summary - Fix Demonstrator Document Panel Toggle Duplication

## Overview
Resolved the issue where the "Dokument" (Document) panel toggle appeared twice in the demonstrator generator (`procedural3d-play`).

## Root Cause & Fix
1. **Invalid `panelTabKindId` call on String Constant**:
   - Fixed `documentPanelKey` in [`ShellHost/🟦️component.tsx`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/ShellHost/%F0%9F%9F%A6%EF%B8%8Fcomponent.tsx#L2108) to use `FRAMEWORK_PANEL_TAB_ARTIFACT_ID` directly instead of passing a string to `panelTabKindId`, allowing document panel state and tree patching to populate `panelUiByKey["framework.panel.artifact"]` correctly.
2. **Recursive Leaf Tab Inspection**:
   - Updated `hasPluginArtifactTab` in [`ShellHost/🟦️component.tsx`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/ShellHost/%F0%9F%9F%A6%EF%B8%8Fcomponent.tsx#L3876) to use `flattenPanelTabLeaves(pluginLeftTabs)` for leaf matching.
   - Added recursive `panel_tabs_contain_artifact` helper in [`Shell/🧊️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/Shell/%F0%9F%A7%8A%EF%B8%8Fcomponent.rs#L7291) to check tab child trees before prepending a fallback document tab.

## Files Updated
- [`ShellHost/🟦️component.tsx`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/ShellHost/%F0%9F%9F%A6%EF%B8%8Fcomponent.tsx)
- [`Shell/🧊️component.rs`](file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%EF%B8%8F%E2%80%8D%F0%9F%8E%A8%EF%B8%8Fengine/%F0%9F%A7%B1%EF%B8%8Felements/Shell/%F0%9F%A7%8A%EF%B8%8Fcomponent.rs)
