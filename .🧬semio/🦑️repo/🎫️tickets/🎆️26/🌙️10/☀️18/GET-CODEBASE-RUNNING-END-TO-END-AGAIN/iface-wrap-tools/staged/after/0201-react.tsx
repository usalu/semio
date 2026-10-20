/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🟦️Interpreter/component.tsx
/** 🌳️ `🟦️Interpreter` — turns the semantic UI contract (`semio-framework-ui-contract`'s
 * `UiNodeRecord`/`Component`/`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`) retained by a
 * `📃️UiDocumentStore` into `@semio-tech/ui-react` components. `InterpretedUiNode` is the entry point;
 * `UiNodeView` is the atomic per-node unit — each one subscribes to exactly its own record via
 * `useUiNode`, so a `SetComponent` on one node re-renders exactly that node's component, never its
 * ancestors or siblings (the whole payoff of the flat id-keyed table, see `📃️UiDocumentStore`'s header
 * doc). The scene hosts behind `Component::Surface` are resolved through the element-host registry
 * (`🗺️element-host-registry`); the concrete hosts are registered by the composition root `🪢️ElementHosts` and are
 * never imported here. This entry module owns `InterpretedUiNode` and `interpretUiNode`; everything else lives in
 * its children, which importers import directly: `👥️presence-overlay`, `🌲️tree-panel-boundary`, `🧷️interpreter-context`, `🔌️surface-bridge`, `🚚️surface-scene-lanes`, `🗄️virtual-file-system`, `📐️layout-accessibility`, `⚡️action-dispatch`, `🎛️declarative-controls`, `🔤️display-views`, `✍️input-views`, `🌳️tree-walk`, `🪪️stable-dom-ids`, `🪟️tree-windows`, `🧾️row-actions`, `🖼️surface-views`, `📊️table-window`, `🧱️node-tree`. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { memo, Profiler, useCallback, type ReactNode } from "../⚛️react-port/🟦️.ts";
import { LocalDocumentOwnerContext, LocalDocumentWindowContext, type LocalDocumentOwnerV1 } from "./🧭️local-document-owner/🟦️.ts";
import { inputActionWithWindowV1, type ShellInputActionV1 } from "../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import {
  type ActionDispatchLifecycleV1,
  type SceneLaneRef,
  BOARD2D_SCENE_LANES,
  CANVAS2D_SCENE_LANES,
  PAINT2D_SCENE_LANES,
  WORLD3D_SCENE_LANES,
  NODE_GRAPH_SCENE_LANES,
  nodeGraphSceneFromLanes,
  WORLD3D_SCENE_LANE_KEY_PREFIX,
  board2dSceneFromLanes,
  canvas2dSceneFromLanes,
  paint2dSceneFromLanes,
  world3dSceneFromLanes,
  world3dSceneLaneForBodyKey
} from "@semio-tech/framework";
import { DEFAULT_UI_DOCUMENT_LIMITS, UiDocumentStore, useUiDocumentRoot, type UiDocumentState } from "../📃️UiDocumentStore/🟦️.tsx";
import { type UiIntent, type UiNodeRecord } from "@semio-tech/framework";
import { type UiInterpreterContext, usePluginSurfaceActions } from "./🧷️interpreter-context/🟦️.ts";
import { surfaceKindSceneLanes } from "./🔌️surface-bridge/🟦️.tsx";
import { surfaceLaneTexts, surfaceSceneLaneCache, surfaceSceneLaneText, utf8ByteLength } from "./🚚️surface-scene-lanes/🟦️.ts";
import { accessibilityAriaProps, layoutSpecStyle } from "./📐️layout-accessibility/🟦️.tsx";
import { uiChildReactKeys, uiSiblingReactKeys } from "./🪪️stable-dom-ids/🟦️.ts";
import {
  tableColumnWindowRequestV1,
  treePickIntentInputV1,
  treePickTargetsV1,
  treeWindowBodyRequestsV1,
  treeWindowContainersUnder,
  TreeWindowContext,
  treeWindowRowHeightPx,
  treeWindowScrollViewport,
  treeWindowServedRequestsV1,
  treeWindowViewportMetrics
} from "./🪟️tree-windows/🟦️.ts";
import {
  tableWindowNextColumnV1,
  tableWindowNextRowV1,
  tableWindowScrollLeftForColumnV1,
  tableWindowScrollTopForRowV1,
  tableWindowViewportCapRowsV1
} from "./📊️table-window/🟦️.ts";
import { treeItemToTreeData, UiNodeView } from "./🧱️node-tree/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️UiInterpreter
/** 🌳️ Entry point — resolves `store`'s current root and renders it. */
export function interpretUiNode(store: UiDocumentStore, context: UiInterpreterContext): ReactNode {
  const root = store.getState().root;
  if (root === null) return null;
  return <UiNodeView store={store} id={root} context={context} />;
}

/**
 * 🐢️ `React.memo`'d entry point — `store` is a stable per-surface identity, so only the root id
 * changing (a `SetRoot`/full `loadSnapshot`) causes this to re-subscribe; ordinary node mutations are
 * handled entirely by `UiNodeView`'s own per-id subscription several levels down, never by
 * re-rendering from here.
 */
export const InterpretedUiNode = memo(function InterpretedUiNode({ store, windowId, localDocumentOwner = null, onAction, onIntent, requestContextMenu }: { readonly store: UiDocumentStore; readonly windowId?: string; readonly localDocumentOwner?: LocalDocumentOwnerV1 | null } & Pick<UiInterpreterContext, "onAction" | "onIntent" | "requestContextMenu">): ReactNode {
  const surfaceActions = usePluginSurfaceActions();
  const root = useUiDocumentRoot(store);
  const ownedAction = useCallback((action: ShellInputActionV1, lifecycle?: ActionDispatchLifecycleV1) => onAction(windowId === undefined ? action : inputActionWithWindowV1(action, windowId), lifecycle), [onAction, windowId]);
  const ownedIntent = useCallback((intent: UiIntent) => onIntent(intent, windowId), [onIntent, windowId]);
  if (root === null) return null;
  return <LocalDocumentOwnerContext.Provider value={localDocumentOwner}><LocalDocumentWindowContext.Provider value={windowId ?? null}><UiNodeView store={store} id={root} context={{ store, localDocumentOwner, onAction: ownedAction, onIntent: ownedIntent, requestContextMenu: requestContextMenu ?? surfaceActions }} /></LocalDocumentWindowContext.Provider></LocalDocumentOwnerContext.Provider>;
});
//#endregion 🔖️UiInterpreter

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️unknown-component-placeholder/🟦️.tsx");
  await registerTests1(import.meta.vitest, { DEFAULT_UI_DOCUMENT_LIMITS, Profiler, UiDocumentStore, UiNodeView, accessibilityAriaProps }, { directory: import.meta.dir, url: import.meta.url });
  const { registerTests1: registerContinuousPressTests } = await import("./🧪️tests/🧪️continuous-presses/🟦️.tsx");
  await registerContinuousPressTests(import.meta.vitest, { UiDocumentStore, UiNodeView }, { url: import.meta.url });
  const { registerTests1: registerNumberKeyboardLawTests } = await import("./🧪️tests/🧪️number-keyboard-law/🟦️.tsx");
  await registerNumberKeyboardLawTests(import.meta.vitest, { UiDocumentStore, UiNodeView }, { url: import.meta.url });
  const { registerTests1: registerTextKeyboardLawTests } = await import("./🧪️tests/🧪️text-keyboard-law/🟦️.tsx");
  await registerTextKeyboardLawTests(import.meta.vitest, { UiDocumentStore, UiNodeView }, { url: import.meta.url });
  const { registerTests1: registerContainerNodeIdTests } = await import("./🧪️tests/🪪️container-node-ids/🟦️.tsx");
  await registerContainerNodeIdTests(import.meta.vitest, { UiDocumentStore, UiNodeView, uiChildReactKeys, uiSiblingReactKeys }, { url: import.meta.url });
  const { registerTests1: registerTreeWindowTests } = await import("./🧪️tests/🪟️tree-windows/🟦️.tsx");
  await registerTreeWindowTests(import.meta.vitest, { TreeWindowContext, UiDocumentStore, UiNodeView, treeItemToTreeData, treePickIntentInputV1, treePickTargetsV1, treeWindowBodyRequestsV1, treeWindowContainersUnder, treeWindowRowHeightPx, treeWindowScrollViewport, treeWindowServedRequestsV1, treeWindowViewportMetrics }, { url: import.meta.url });
  const { registerTests1: registerTableWindowTests } = await import("./🧪️tests/📊️table/🟦️.tsx");
  await registerTableWindowTests(
    import.meta.vitest,
    { TreeWindowContext, UiDocumentStore, UiNodeView, treeItemToTreeData, tableColumnWindowRequestV1, tableWindowNextColumnV1, tableWindowNextRowV1, tableWindowScrollLeftForColumnV1, tableWindowScrollTopForRowV1, tableWindowViewportCapRowsV1, treeWindowRowHeightPx },
    { url: import.meta.url },
  );
  const { registerTests1: registerProgressTests } = await import("./🧪️tests/📶️progress/🟦️.tsx");
  await registerProgressTests(import.meta.vitest, { UiDocumentStore, UiNodeView }, { url: import.meta.url });
  const { registerTests1: registerOverlayFlowTests } = await import("./🧪️tests/📐️overlay-flow/🟦️.tsx");
  await registerOverlayFlowTests(import.meta.vitest, { UiDocumentStore, UiNodeView, layoutSpecStyle }, { url: import.meta.url });
  const { registerTests1: registerSurfaceSceneLaneTests } = await import("./🧪️tests/🚚️surface-scene-lanes/🟦️.tsx");
  await registerSurfaceSceneLaneTests(
    import.meta.vitest,
    {
      UiDocumentStore,
      UiNodeView,
      surfaceSceneLaneText,
      surfaceSceneLaneCache,
      utf8ByteLength,
      nodeGraphSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, NODE_GRAPH_SCENE_LANES),
      nodeGraphSceneFromLanes,
      world3dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, WORLD3D_SCENE_LANES),
      canvas2dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, CANVAS2D_SCENE_LANES),
      board2dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, BOARD2D_SCENE_LANES),
      paint2dSurfaceLaneTexts: (record: UiNodeRecord, state: UiDocumentState, declared: readonly SceneLaneRef[]) => surfaceLaneTexts(record, state, declared, PAINT2D_SCENE_LANES),
      world3dSceneFromLanes,
      canvas2dSceneFromLanes,
      board2dSceneFromLanes,
      paint2dSceneFromLanes,
      surfaceKindSceneLanes,
      NODE_GRAPH_SCENE_LANES,
      WORLD3D_SCENE_LANES,
      CANVAS2D_SCENE_LANES,
      BOARD2D_SCENE_LANES,
      PAINT2D_SCENE_LANES,
      WORLD3D_SCENE_LANE_KEY_PREFIX,
      world3dSceneLaneForBodyKey,
    },
    { url: import.meta.url },
  );
}
//#endregion 🧪️Tests
