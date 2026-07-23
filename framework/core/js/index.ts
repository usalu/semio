// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@semio-tech/framework-core` — shared canvas pick helpers, layout factories, and inspector utilities for UI renderers. */
// #endregion 🧲Header

import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../plugin/registry/generated/playgrounds.ts";
import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, pluginModuleUrl } from "../../plugin/registry/generated/plugins.ts";

// #region 🧬GeneratedMirror
/** 🧬 Types generated from `framework/core/rs/lib.rs` via ts-rs (`bun nx run @semio-tech/framework-core:generate`); re-exported below alongside their hand-written neighbors so this stays the one import surface. */
import type {
  ActionDescriptor as GeneratedActionDescriptor,
  ActionKind as GeneratedActionKind,
  ActionDefinition as GeneratedActionDefinition,
  ActionArgDef as GeneratedActionArgDef,
  ActionArgControl as GeneratedActionArgControl,
  ActionArgOption as GeneratedActionArgOption,
  UtilityDefinition as GeneratedUtilityDefinition,
  UtilityRef as GeneratedUtilityRef,
  ToolDefinition as GeneratedToolDefinition,
  ToolRef as GeneratedToolRef,
  CommandScope as GeneratedCommandScope,
  CommandDefinition as GeneratedCommandDefinition,
  CommandRef as GeneratedCommandRef,
  WindowMeasure as GeneratedWindowMeasure,
  WindowEngagementOption as GeneratedWindowEngagementOption,
  WindowEngagementInput as GeneratedWindowEngagementInput,
  WindowEngagementStatus as GeneratedWindowEngagementStatus,
  WindowEngagementPossible as GeneratedWindowEngagementPossible,
  WindowEngagementRingOption as GeneratedWindowEngagementRingOption,
  WindowEngagementToggleGroupOption as GeneratedWindowEngagementToggleGroupOption,
  WindowEngagementSelectItem as GeneratedWindowEngagementSelectItem,
  WindowEngagementControl as GeneratedWindowEngagementControl,
  WindowEngagement as GeneratedWindowEngagement,
  WindowEngagementSlot as GeneratedWindowEngagementSlot,
  WindowOptions as GeneratedWindowOptions,
  ActionRef as GeneratedActionRef,
  PanelGroup as GeneratedPanelGroup,
  PanelTabKind as GeneratedPanelTabKind,
  PanelTabDefinition as GeneratedPanelTabDefinition,
  ModeDefinition as GeneratedModeDefinition,
  WindowKindDefinition as GeneratedWindowKindDefinition,
  AppDefinition as GeneratedAppDefinition,
  IntroductionDefinition as GeneratedIntroductionDefinition,
  IntroductionStepDefinition as GeneratedIntroductionStepDefinition,
  IntroductionPlacement as GeneratedIntroductionPlacement,
  IntroductionAdvance as GeneratedIntroductionAdvance,
  IntroductionLogo as GeneratedIntroductionLogo,
  DialogDefinition as GeneratedDialogDefinition,
} from "./generated/manifest.ts";
// #endregion 🧬GeneratedMirror

export const CANVAS_HOVER_SOURCE_CANVAS = "canvas";
export const CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu";
export const CANVAS_HOVER_SOURCE_CATALOG = "catalog";
export const CANVAS_HOVER_SOURCE_DOCUMENT = "document";

export const FRAMEWORK_PANEL_TAB_DOCUMENT_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL = "Document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";

//#region 🆔ElementId
/** 🆔 Element id of the app shell's navbar/footer — singular, shell-owned chrome. */
export const UI_NAVBAR_ELEMENT_ID = "ui.navbar";
export const UI_FOOTER_ELEMENT_ID = "ui.footer";

/** 🆔 Normalizes arbitrary input into a single camelCase element-id segment — byte-for-byte mirror of
 * `element_id_segment` in `framework/core/rs/lib.rs` (core/js stays DOM-free, so the DOM-facing
 * `elementIdSelector`/alias helpers live in `ui/js/react` instead). */
function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) {
      segment += ch.toLowerCase();
    } else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else {
      segment += ch;
    }
  }
  return segment;
}

/** 🆔 Element id of a window kind's body — `framework.window.{camelCased kind id}`. */
export function windowElementId(kindId: string): string {
  return `framework.window.${elementIdSegment(kindId)}`;
}

/** 🆔 Element id of a panel tab's uncollapsed panel body; `tabId` is already dotted, appended verbatim. */
export function panelTabElementId(tabId: string): string {
  return `framework.panelTab.${tabId}`;
}

/** 🆔 Alias id of the first draggable tree row inside a panel tab, stamped via `data-element-alias`. */
export function panelTabFirstDraggableElementId(tabId: string): string {
  return `framework.panelTab.${tabId}.firstDraggable`;
}
//#endregion 🆔ElementId

export type CanvasPickTarget = {
  readonly domain: string;
  readonly id: string;
  readonly generality: number;
  readonly label: string;
  readonly kind?: string;
};

export type CanvasPickRequest = {
  readonly targets: readonly CanvasPickTarget[];
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers?: Readonly<Record<string, boolean>>;
};

export type CanvasHoverFocus = {
  readonly sourceId: string;
  readonly target: CanvasPickTarget | null;
};

/** 🧬 Generated from Rust `ActionDescriptor` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionDescriptor = GeneratedActionDescriptor;

export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
};

export type WindowLayoutStackNode = {
  readonly kind: "stack";
  readonly size?: number;
  readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
  readonly kind: "row" | "column";
  readonly size?: number;
  readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
  readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};

export type NamedLayout = {
  readonly id: string;
  readonly label: string;
  readonly iconId?: string;
  readonly layout: WindowLayout;
  readonly origin: "builtin" | "user";
  readonly groupPath?: readonly string[];
};

export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

export type UtilityCategory = "selection" | "utilities" | "history" | "sync";

export type UtilityLeaf =
  | { readonly id: string; readonly kind: "separator"; readonly order?: number; readonly disabled?: boolean }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    };

export type UtilityNode =
  | UtilityLeaf
  | {
      readonly id: string;
      readonly kind: "collection";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly children: readonly UtilityNode[];
    }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly onPress: ActionDescriptor;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly onChange: ActionDescriptor;
    };

export type UiSectionNode = {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly children: readonly UiNode[];
};

/** @emoji 🌳 One hover-revealed row action on a {@link UiTreeItemNode}; renderer-side addition on top of the base wasm tree-item shape. */
export type UiTreeItemAction = {
  readonly iconId: string;
  readonly label?: string;
  readonly action: ActionDescriptor;
  readonly revealOnHover?: boolean;
};

export type UiTreeItemNode = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: string;
  readonly iconId?: string;
  readonly selected?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly defaultOpen?: boolean;
  readonly action?: ActionDescriptor;
  readonly hoverAction?: ActionDescriptor;
  readonly unhoverAction?: ActionDescriptor;
  readonly actions?: readonly UiTreeItemAction[];
  readonly draggable?: boolean;
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly UiTreeItemNode[];
  readonly control?: UiControlNode;
  readonly isHidden?: boolean;
};

export type UiTreeSectionNode = {
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
};

export type UiControlNode = UiInputNode | UiSelectNode | UiToggleNode | UiButtonNode | UiKeyValueNode | UiSliderNode | UiNumberStepperNode | UiRingNode | UiIconSelectNode;

export type UiInputNode = {
  readonly type: "input";
  readonly id: string;
  readonly inputKind: string;
  readonly value: string;
  readonly placeholder?: string;
  readonly commit?: string;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly accept?: string;
  readonly onChange: ActionDescriptor;
};

export type UiSelectItem = {
  readonly value: string;
  readonly label: string;
};

export type UiSelectNode = {
  readonly type: "select";
  readonly id: string;
  readonly value: string;
  readonly items: readonly UiSelectItem[];
  readonly placeholder?: string;
  readonly onChange: ActionDescriptor;
};

export type UiToggleNode = {
  readonly type: "toggle";
  readonly id: string;
  readonly iconId: string;
  readonly pressed: boolean;
  readonly text?: string;
  readonly onChange: ActionDescriptor;
};

/** @emoji 🌿 A nestable labeled container of {@link UiNode} children — the declarative-tree mechanism
 * for subtrees like `Origin > X/Y/Z`: {@link uiDeclarativeChildToTreeItem} expands a `Group` into a
 * {@link UiTreeItemNode} whose `items` are its recursively-converted children, so depth composes to
 * any level (`Plane > Origin > X/Y/Z`). Unlike {@link UiSectionNode} (top-level tree sections only,
 * see `assertNoNestedTreeSections`), a `Group` may itself appear as another `Group`'s or
 * {@link UiFieldNode}'s child. */
export type UiGroupNode = {
  readonly type: "group";
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly children: readonly UiNode[];
};

export type UiKeyValueEntry = {
  readonly label: string;
  readonly value: string;
};

export type UiKeyValueNode = {
  readonly type: "keyValue";
  readonly entries: readonly UiKeyValueEntry[];
};

export type UiSliderNode = {
  readonly type: "slider";
  readonly id: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly unit?: string;
  readonly onChange: ActionDescriptor;
};

export type UiNumberStepperNode = {
  readonly type: "numberStepper";
  readonly id: string;
  readonly value: number;
  readonly step: number;
  readonly uniform: boolean;
  readonly onAbsolute: ActionDescriptor;
  readonly onDelta: ActionDescriptor;
};

export type UiRingNode = {
  readonly type: "ring";
  readonly id: string;
  readonly orbId: string;
  readonly t: number;
  readonly disabled?: boolean;
  readonly onChange: ActionDescriptor;
};

export type UiIconSelectNode = {
  readonly type: "iconSelect";
  readonly id: string;
  readonly value: string;
  readonly uniform: boolean;
  readonly classifierKind: string;
  readonly onChange: ActionDescriptor;
};

export type UiFieldNode = {
  readonly type: "field";
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly required?: boolean;
  readonly error?: string;
  readonly child: UiNode;
};

/** 🎨 Renderer-side visual variant/size/density hints on a {@link UiButtonNode} — no wasm/plugin equivalent, purely a display hint. */
export type StyleSpec = {
  readonly variant?: string;
  readonly size?: string;
  readonly density?: string;
};

export type UiButtonNode = {
  readonly type: "button";
  readonly id?: string;
  readonly iconId: string;
  readonly label: string;
  readonly action: ActionDescriptor;
  readonly style?: StyleSpec;
  readonly disabled?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
};

export type UiTextNode = {
  readonly type: "text";
  readonly value: string;
  readonly emphasize?: boolean;
  readonly dataAttributes?: Readonly<Record<string, string>>;
};

export type UiStackNode = {
  readonly type: "stack";
  readonly direction: string;
  readonly gap?: string;
  readonly padding?: string;
  readonly id?: string;
  readonly selected?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly activate?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly dropOverlay?: UiDropOverlaySpec;
  readonly children: readonly UiNode[];
};

/** 📥 Hover-state copy for a {@link UiStackNode}'s `dropOverlay` — shown while a drag is over the stack, ahead of `dropAction` firing on release. */
export type UiDropOverlaySpec = {
  readonly title: string;
  readonly hint: string;
  readonly accept?: string;
};

export type UiSeparatorNode = { readonly type: "separator" };

export type UiImageNode = {
  readonly type: "image";
  readonly id: string;
  readonly src: string;
  readonly alt?: string;
};

export type UiNode =
  | UiStackNode
  | UiTextNode
  | UiButtonNode
  | UiSeparatorNode
  | UiSectionNode
  | UiInputNode
  | UiSelectNode
  | UiToggleNode
  | UiKeyValueNode
  | UiSliderNode
  | UiNumberStepperNode
  | UiRingNode
  | UiIconSelectNode
  | UiFieldNode
  | UiGroupNode
  | UiTreeNode
  | UiImageNode
  | UiComponentSceneNode
  | UiExternalSlotNode;

export type UiInspectorFieldGroup = {
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly fields: readonly UiNode[];
};

//#region ComponentSceneProtocol
/** 🖼️ A 2D canvas surface scene payload — mirrors the wasm `componentScene` node's `canvas2d` field. */
export type Canvas2dScene = {
  readonly cameraX: number;
  readonly cameraY: number;
  readonly zoom: number;
  readonly layersJson: string;
};

/** 🌐 A 3D world surface scene payload — mirrors the wasm `componentScene` node's `world3d` field. */
/** 🖱️ Serializable context-menu entry emitted by plugins (`contextMenuJson`) and mapped to UI {@link ContextMenuItem} by hosts. */
export type ContextMenuItemSpec = {
  readonly id: string;
  readonly label?: string;
  readonly icon?: string;
  readonly color?: string;
  readonly shortcut?: string;
  readonly disabled?: boolean;
  readonly separator?: boolean;
  readonly checked?: boolean;
  readonly destructive?: boolean;
  readonly action?: string;
  readonly args?: Record<string, unknown>;
  readonly hoverAction?: string;
  readonly hoverArgs?: Record<string, unknown>;
  readonly children?: readonly ContextMenuItemSpec[];
};

export type World3dScene = {
  readonly cameraJson: string;
  readonly meshesJson: string;
  readonly instancesJson: string;
  readonly selectionJson: string;
  readonly vorticesJson?: string;
  readonly attractionsJson?: string;
  readonly targetVolumesJson?: string;
  readonly referencesJson?: string;
  readonly brushPreviewJson?: string;
  readonly interactionJson?: string;
  readonly engagementPreviewJson?: string;
  readonly lodJson?: string;
  readonly chunkingJson?: string;
  readonly contextMenuJson?: string;
  readonly environmentJson?: string;
  readonly frameJson?: string;
  readonly fitJson?: string;
  /** 🌐⛰️ GIS 3D terrain style/source descriptor, consumed by `WorldTerrainLayer`. */
  readonly terrainJson?: string;
  /** ☁️ Point-cloud rendering layers (10^5-10^6 points) — an array of `{ id, positionsB64 (base64 le
   * f32 xyz), colorsB64? (base64 u8 rgb), size, sizeAttenuation }`, consumed by `WorldPointCloudLayer`. */
  readonly pointsJson?: string;
  /** ⏳ Off-main-thread compute status (`{"computing": true, "label": "…"}`) shown as an overlay while
   * a `flowEvalTick` chain resolves the meshes this scene renders. */
  readonly statusJson?: string;
};

/** 🕸️ A node-graph surface scene payload — mirrors the wasm `componentScene` node's `nodeGraph` field. */
export type NodeGraphScene = {
  readonly nodesJson: string;
  readonly edgesJson: string;
  readonly viewportJson: string;
  readonly editable?: boolean;
  readonly operatorsJson?: string;
  readonly contextMenuJson?: string;
  readonly findItemsJson?: string;
  readonly selectionJson?: string;
  readonly hoverJson?: string;
  readonly previewOffJson?: string;
  readonly lodJson?: string;
  readonly catalogueJson?: string;
  readonly controlsJson?: string;
  readonly clustersJson?: string;
  readonly computingJson?: string;
  readonly capabilitiesJson?: string;
  readonly fixtureJson?: string;
  readonly presencePeersJson?: string;
  /** 🧵 Channel-structured eval outputs from an off-main-thread `flowEvalTick` chain, applied via
   * `FlowWasmSession.applyEvalOutputsJson` — lets the canvas session pick up results without ever
   * evaluating itself. */
  readonly evalJson?: string;
};

/** 👥 A live-collaboration cursor/selection peer shown on a shared surface. */
export type PresencePeer = {
  readonly clientId: string;
  readonly name: string;
  readonly selectionCount: number;
};

/** 📝 A text-editor surface scene payload — mirrors the wasm `componentScene` node's `textEditor` field. */
export type TextEditorScene = {
  readonly buffer: string;
  readonly language?: string;
  readonly selectionJson?: string;
  readonly tokensJson?: string;
  readonly diagnosticsJson?: string;
  readonly completionsJson?: string;
  readonly overlaysJson?: string;
  readonly occurrencesJson?: string;
  readonly placeholdersJson?: string;
  readonly extraCaretsJson?: string;
  readonly selectableSpansJson?: string;
  readonly settingsJson?: string;
  readonly cameraJson?: string;
  readonly hoverJson?: string;
  readonly newlineGatesJson?: string;
  readonly renameJson?: string;
};

export const nodeGraphActions = {
  select: "nodeGraphSelect",
  hover: "nodeGraphHover",
  edit: "nodeGraphEdit",
  viewport: "nodeGraphViewport",
  spotlightCommit: "spotlightCommit",
} as const;

export const textEditorActions = {
  edit: "textEdit",
  select: "textSelect",
  hover: "textHover",
  requestCompletions: "requestCompletions",
  commitRename: "commitRename",
  formatDocument: "formatDocument",
} as const;

/** 📋 A table surface scene payload — mirrors the wasm `componentScene` node's `table` field. */
export type TableScene = {
  readonly columnsJson: string;
  readonly rowsJson: string;
  readonly selectionJson?: string;
  readonly rowDragMime?: string;
  readonly dropAction?: ActionDescriptor;
  readonly sortJson?: string;
};

/** 🖌️ A 2D paint surface scene payload — mirrors the wasm `componentScene` node's `paint2d` field. */
export type Paint2dScene = {
  readonly documentSyncJson: string;
  readonly assetsJson: string;
  readonly cameraJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly brushSize: number;
  readonly brushOpacity: number;
  readonly viewMode: string;
  readonly compositeViewportJson?: string;
};

/** 🎨 An icon-render preview surface scene payload — mirrors the wasm `componentScene` node's `iconRender` field. */
export type IconRenderScene = {
  readonly requestJson: string;
  readonly footer?: string;
  readonly frameJson?: string;
};

/** 🗂️ A virtual-file-system browser surface scene payload — mirrors the wasm `componentScene` node's `virtualFileSystem` field. */
export type VirtualFileSystemScene = {
  readonly schemaJson: string;
  readonly rowsJson: string;
  readonly selectedRowIdsJson?: string;
  readonly hoveredRowId?: string;
  readonly emptyMessage?: string;
  readonly dragDropEnabled?: boolean;
};

/** 🗺️ A tiled map surface scene payload — mirrors the wasm `componentScene` node's `tiledMap` field. */
export type TiledMapScene = {
  readonly mapFixtureJson: string;
  readonly cameraJson: string;
  readonly renderMode: string;
  readonly vectorStyle: string;
  readonly lodMode: string;
  readonly tileUrlTemplate: string;
  readonly vectorTileUrlTemplate: string;
  readonly layerVisibilityJson: string;
  readonly layerStrokeScaleJson: string;
  readonly selectionJson: string;
  readonly hoverJson: string;
  readonly selectionMethod: string;
  readonly selectionMode: string;
  readonly contextMenuJson?: string;
};

/** 🧩 A 2D board surface scene payload — mirrors the wasm `componentScene` node's `board2d` field. */
export type Board2dScene = {
  readonly fixtureJson: string;
  readonly cameraJson: string;
  readonly glyphCatalogsJson: string;
  readonly selectionJson: string;
  readonly interactive: boolean;
  readonly hoveredId?: string;
  readonly activeUtility?: string;
  readonly selectionMethod: string;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  readonly suggestionOffset: number;
  readonly brushWeightsJson: string;
  readonly placementCompatibilityJson: string;
  readonly lodMode: string;
};

/** 🖊️ An ink-canvas surface scene payload — mirrors the wasm `componentScene` node's `inkCanvas` field. `documentJson` is opaque to the framework: the owning plugin defines its shape, conventionally an array of items (e.g. stroke | shape | text | image) each carrying its own transform; `selectionJson` is a `string[]` of selected item ids. */
export type InkCanvasScene = {
  readonly documentJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly viewMode: string;
  readonly interactive: boolean;
};

/** 🖊️ Renderer-to-plugin action names for ink-canvas surfaces (modeled after {@link nodeGraphActions}/{@link textEditorActions}). */
export const inkCanvasActions = {
  applyEvents: "inkApplyEvents",
  setSelection: "setSelection",
  setCamera: "setCamera",
  setHover: "setHover",
} as const;

/** 🗄️ A checkpoint ancestor-graph history view. `columnsJson` is a `HistoryColumn[]` array, newest checkpoint first. */
export type GraphTimelineScene = {
  readonly columnsJson: string;
};

/** 🧩 A palette entry for a block kind insertable into a {@link BlockListScene}, contributed either by the host app's own built-ins or by a `protocolBlockKind` module contribution. */
export type BlockPaletteEntry = {
  readonly blockKind: string;
  readonly label: string;
  readonly iconId: string;
};

/** 🧩 A strict, ordered list of steps/blocks for the Blockly-like list editor. `stepsJson` is a `ProtocolStep[]` array, `paletteJson` is a `BlockPaletteEntry[]` array of the block kinds available to insert. */
export type BlockListScene = {
  readonly stepsJson: string;
  readonly paletteJson: string;
  readonly selectedId?: string;
  readonly draggingId?: string;
};

/** 🆚 A before/after text diff surface scene payload — mirrors the wasm `componentScene` node's `diffView` field. */
export type DiffViewScene = {
  readonly before: string;
  readonly after: string;
  readonly language?: string;
  readonly mode?: "unified" | "split";
};

/** 📰 One entry of an {@link EventFeedScene}'s `entriesJson` array. */
export type EventFeedEntry = {
  readonly id: string;
  readonly timestampMs: number;
  readonly iconId: string;
  readonly title: string;
  readonly detail?: string;
  readonly tone?: string;
};

/** 📰 A scrolling event/log feed surface scene payload — mirrors the wasm `componentScene` node's `eventFeed` field. `entriesJson` is an {@link EventFeedEntry}`[]` array. */
export type EventFeedScene = {
  readonly entriesJson: string;
  readonly follow?: boolean;
  readonly activateAction?: string;
};

/** 🔌 A plugin-contributed external body rendered inline — mirrors the wasm `externalSlot` node. */
export type UiExternalSlotNode = {
  readonly type: "externalSlot";
  readonly pluginId: string;
  readonly appId: string;
  readonly bodyKey: string;
  readonly paramsJson: string;
};

/** 🧭 The dispatch key on {@link UiComponentSceneNode} — matches the lazy-loaded host component per `framework/renderer/react/index.tsx`. */
export type ComponentKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";

/** 🖥️ A native (non-declarative) rendering surface — mirrors the wasm `componentScene` node; the active `componentKind` selects which optional scene field is populated. */
export type UiComponentSceneNode = {
  readonly type: "componentScene";
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly componentKind: string;
  readonly paneId?: string;
  readonly bindingId?: string;
  readonly canvas2d?: Canvas2dScene;
  readonly world3d?: World3dScene;
  readonly nodeGraph?: NodeGraphScene;
  readonly textEditor?: TextEditorScene;
  readonly table?: TableScene;
  readonly paint2d?: Paint2dScene;
  readonly virtualFileSystem?: VirtualFileSystemScene;
  readonly tiledMap?: TiledMapScene;
  readonly board2d?: Board2dScene;
  readonly iconRender?: IconRenderScene;
  readonly inkCanvas?: InkCanvasScene;
  readonly graphTimeline?: GraphTimelineScene;
  readonly blockList?: BlockListScene;
  readonly diffView?: DiffViewScene;
  readonly eventFeed?: EventFeedScene;
};

/** 🧷 Shared prop shape for every `framework/renderer/react/index.tsx` host component. */
export type ComponentSceneHostProps = { readonly node: UiComponentSceneNode; readonly onAction: (action: ActionDescriptor) => void };
//#endregion ComponentSceneProtocol

export function canvasPickTargetKey(target: CanvasPickTarget): string {
  return `${target.domain}:${target.id}`;
}

/** @emoji 🪪 Parses a pick target key into domain and id. */
export function parseCanvasPickTargetKey(key: string): { readonly domain: string; readonly id: string } | null {
  const colon = key.indexOf(":");
  if (colon < 0) return null;
  return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}

export function sortCanvasPickTargetsGeneralFirst(targets: readonly CanvasPickTarget[]): readonly CanvasPickTarget[] {
  return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}

export function pickMostSpecificCanvasTarget(targets: readonly CanvasPickTarget[]): CanvasPickTarget | null {
  if (targets.length === 0) return null;
  return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}

export function canvasHoverFocusFromTarget(sourceId: string, target: CanvasPickTarget | null): CanvasHoverFocus {
  return { sourceId, target };
}

export function createWindowLayout(windowKindId: string, title?: string, options?: { readonly instanceId?: string; readonly templateId?: string }): WindowLayoutWindowNode {
  return {
    kind: "window",
    windowKindId,
    ...(title ? { title } : {}),
    ...(options?.instanceId ? { instanceId: options.instanceId } : {}),
    ...(options?.templateId ? { templateId: options.templateId } : {}),
  };
}

export function createStackLayout(windowKindIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: "stack",
      children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
    },
  };
}

export function createDefaultLayout(windowIds: readonly string[], direction: "row" | "column" = "row", sizes?: readonly number[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: direction,
      children: windowIds.map((id, index) => ({
        kind: "stack" as const,
        ...(sizes?.[index] !== undefined ? { size: sizes[index] } : {}),
        children: [createWindowLayout(id, titles?.[index] ?? id)],
      })),
    },
  };
}

export function createTabStackLayout(windowIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return createStackLayout(windowIds, titles);
}

export function createNamedLayout(id: string, label: string, layout: WindowLayout, origin: NamedLayout["origin"] = "builtin", iconId?: string, groupPath?: readonly string[]): NamedLayout {
  return {
    id,
    label,
    layout,
    origin,
    ...(iconId ? { iconId } : {}),
    ...(groupPath?.length ? { groupPath } : {}),
  };
}

export function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
  if (!base?.length && !extension?.length) return undefined;
  const merged = new Map<string, T>();
  base?.forEach((entry) => merged.set(entry.id, entry));
  extension?.forEach((entry) => merged.set(entry.id, entry));
  return [...merged.values()];
}

export function mergeNamedLayouts(base: readonly NamedLayout[] | undefined, extension: readonly NamedLayout[] | undefined): NamedLayout[] {
  return mergeById(base, extension) ?? [];
}

export type PlatformSubscriber = () => void;

export abstract class Store<TSnapshot> {
  private readonly listeners = new Set<PlatformSubscriber>();
  private disposed = false;

  abstract getSnapshot(): TSnapshot;

  subscribe(listener: PlatformSubscriber): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  protected notify(): void {
    if (this.disposed) return;
    for (const listener of this.listeners) listener();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
  }
}

export interface StoragePort {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
}

function namedLayoutStorageKey(appId: string): string {
  return `compose.display.layouts.${appId}`;
}

export class NamedLayoutStore extends Store<readonly NamedLayout[]> {
  private layouts: NamedLayout[] = [];

  constructor(
    private readonly appId: string,
    private readonly storage: StoragePort,
  ) {
    super();
    this.layouts = this.readPersisted();
  }

  getSnapshot(): readonly NamedLayout[] {
    return this.layouts;
  }

  save(layout: NamedLayout): void {
    const next = mergeNamedLayouts(
      this.layouts.filter((entry) => entry.id !== layout.id),
      [layout],
    );
    this.layouts = next;
    this.persist();
    this.notify();
  }

  remove(layoutId: string): void {
    const next = this.layouts.filter((entry) => entry.id !== layoutId);
    if (next.length === this.layouts.length) return;
    this.layouts = next;
    this.persist();
    this.notify();
  }

  private readPersisted(): NamedLayout[] {
    const raw = this.storage.get(namedLayoutStorageKey(this.appId));
    if (!raw) return [];
    try {
      const parsed = JSON.parse(raw) as unknown;
      if (!Array.isArray(parsed)) return [];
      return parsed.filter(
        (entry): entry is NamedLayout =>
          Boolean(entry) && typeof entry === "object" && typeof (entry as NamedLayout).id === "string" && typeof (entry as NamedLayout).label === "string" && (entry as NamedLayout).origin === "user" && Boolean((entry as NamedLayout).layout),
      );
    } catch {
      return [];
    }
  }

  private persist(): void {
    this.storage.set(namedLayoutStorageKey(this.appId), JSON.stringify(this.layouts));
  }
}

/** 🧭 The eight anchor ids, mirroring `Anchor` in `ui/js/react/index.tsx` (kept inline/private here to stay dependency-free of that package) — shared by every persisted anchor-keyed shape below so they can't drift apart from one another. */
type PersistedAnchor = "top-left" | "top-middle" | "top-right" | "right-middle" | "bottom-right" | "bottom-middle" | "bottom-left" | "left-middle";

//#region DockLayoutStore
/** 🐳 One tab (leaf or branch) in a persisted dock panel-arrangement tree; leaves carry `trees`, branches carry `children`. */
export interface DockTabSkeleton {
  id: string;
  children?: readonly DockTabSkeleton[];
  trees?: readonly string[];
}

/** 🐳 The full persisted dock arrangement, one tab tree per anchor — anchor ids mirror `Anchor` in `ui/js/react/index.tsx` (kept inline here to stay dependency-free of that package). */
export interface DockSkeleton {
  version: 3;
  anchors: Record<PersistedAnchor, readonly DockTabSkeleton[]>;
}

function dockOsStorageKey(): string {
  return "semio.os.dock";
}

function dockAppStorageKey(appId: string): string {
  return `semio.os.dock.${appId}`;
}

/** 🧪 Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readDockSkeleton(storage: StoragePort, key: string): DockSkeleton | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as DockSkeleton).version !== 3 || !(parsed as DockSkeleton).anchors || typeof (parsed as DockSkeleton).anchors !== "object") return null;
    return parsed as DockSkeleton;
  } catch {
    return null;
  }
}

/** 🐳 Persists the dock panel arrangement across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockLayoutStore extends Store<DockSkeleton | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): DockSkeleton | null {
    if (this.appId) {
      const app = readDockSkeleton(this.storage, dockAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockSkeleton(this.storage, dockOsStorageKey());
  }

  save(skeleton: DockSkeleton | null): void {
    this.writeOrRemove(this.appId ? dockAppStorageKey(this.appId) : dockOsStorageKey(), skeleton);
    this.notify();
  }

  saveOs(skeleton: DockSkeleton | null): void {
    this.writeOrRemove(dockOsStorageKey(), skeleton);
    this.notify();
  }

  reset(): void {
    this.storage.remove(dockOsStorageKey());
    if (this.appId) this.storage.remove(dockAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, skeleton: DockSkeleton | null): void {
    if (skeleton === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(skeleton));
  }
}
//#endregion DockLayoutStore

//#region DockUiStateStore
/** 🌱 Persisted per-anchor panel chrome — only the fields that differ from the shell's computed defaults are ever stored. */
export interface DockUiPanelState {
  visible?: boolean;
  size?: number;
  path?: readonly string[];
}

/** 🌱 The full persisted dock UI state: per-anchor visibility/size/active-path, per-branch drill-down memory, and tree section/group expansion. Anchor ids mirror `Anchor` (kept inline here to stay dependency-free of the `ui` package, same convention as {@link DockSkeleton}). */
export interface DockUiState {
  version: 3;
  anchors: Partial<Record<PersistedAnchor, DockUiPanelState>>;
  pathMemory?: Readonly<Record<string, string>>;
  treeOpen?: Readonly<Record<string, boolean>>;
}

function dockUiOsStorageKey(): string {
  return "semio.os.dockUi";
}

function dockUiAppStorageKey(appId: string): string {
  return `semio.os.dockUi.${appId}`;
}

/** 🧪 Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readDockUiState(storage: StoragePort, key: string): DockUiState | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as DockUiState).version !== 3 || !(parsed as DockUiState).anchors || typeof (parsed as DockUiState).anchors !== "object") return null;
    return parsed as DockUiState;
  } catch {
    return null;
  }
}

/** 🌱 Persists panel visibility/size/path, drill-down memory, and tree expansion across an "os" layer (global default) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockUiStateStore extends Store<DockUiState | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): DockUiState | null {
    if (this.appId) {
      const app = readDockUiState(this.storage, dockUiAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockUiState(this.storage, dockUiOsStorageKey());
  }

  save(state: DockUiState | null): void {
    this.writeOrRemove(this.appId ? dockUiAppStorageKey(this.appId) : dockUiOsStorageKey(), state);
    this.notify();
  }

  saveOs(state: DockUiState | null): void {
    this.writeOrRemove(dockUiOsStorageKey(), state);
    this.notify();
  }

  reset(): void {
    this.storage.remove(dockUiOsStorageKey());
    if (this.appId) this.storage.remove(dockUiAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, state: DockUiState | null): void {
    if (state === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(state));
  }
}
//#endregion DockUiStateStore

//#region WindowPaneStateStore
/** 🪟 Persisted state for one window-level pane (a {@link DockUiPanelState} sibling, but keyed per window INSTANCE id rather than globally) — only the fields that differ from the shell's computed defaults are ever stored. */
export interface WindowPaneState {
  anchor?: PersistedAnchor;
  folded?: boolean;
  size?: number;
}

/** 🪟 The full persisted window-pane arrangement: per-window-instance, per-pane anchor/fold/size — the pane-level analog of {@link DockUiState}, since panes float inside a window rather than docking to the shell's own edges. */
export interface WindowPaneUiState {
  version: 1;
  windows: Record<string, Record<string, WindowPaneState>>;
}

function windowPaneUiOsStorageKey(): string {
  return "semio.os.paneUi";
}

function windowPaneUiAppStorageKey(appId: string): string {
  return `semio.os.paneUi.${appId}`;
}

/** 🧪 Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readWindowPaneUiState(storage: StoragePort, key: string): WindowPaneUiState | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as WindowPaneUiState).version !== 1 || !(parsed as WindowPaneUiState).windows || typeof (parsed as WindowPaneUiState).windows !== "object") return null;
    return parsed as WindowPaneUiState;
  } catch {
    return null;
  }
}

/** 🪟 Persists window-pane anchor/fold/size across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class WindowPaneStateStore extends Store<WindowPaneUiState | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): WindowPaneUiState | null {
    if (this.appId) {
      const app = readWindowPaneUiState(this.storage, windowPaneUiAppStorageKey(this.appId));
      if (app) return app;
    }
    return readWindowPaneUiState(this.storage, windowPaneUiOsStorageKey());
  }

  save(state: WindowPaneUiState | null): void {
    this.writeOrRemove(this.appId ? windowPaneUiAppStorageKey(this.appId) : windowPaneUiOsStorageKey(), state);
    this.notify();
  }

  saveOs(state: WindowPaneUiState | null): void {
    this.writeOrRemove(windowPaneUiOsStorageKey(), state);
    this.notify();
  }

  reset(): void {
    this.storage.remove(windowPaneUiOsStorageKey());
    if (this.appId) this.storage.remove(windowPaneUiAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, state: WindowPaneUiState | null): void {
    if (state === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(state));
  }
}
//#endregion WindowPaneStateStore

export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
      } catch {
        /* ignore */
      }
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
      } catch {
        /* ignore */
      }
    },
  };
}

/** 🧠 In-memory {@link StoragePort} — used by ephemeral branded shells so nothing survives a window refresh. */
export function createMemoryStoragePort(): StoragePort {
  const map = new Map<string, string>();
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    },
  };
}

export function uiInspectorAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let index = 1; index < values.length; index += 1) {
    if (values[index] !== first) return false;
  }
  return true;
}

export function uiInspectorMixedText(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? "") : "", placeholder: uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER };
}

export function uiInspectorMixedNumber(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? 0) : Number.NaN, uniform };
}

export function uiInspectorMixedSelect(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  return uiInspectorMixedText(values);
}

export function uiInspectorMixedToggle(values: readonly boolean[]): { readonly pressed: boolean; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { pressed: uniform ? (values[0] ?? false) : false, uniform };
}

export function uiInspectorMixedSlider(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  return uiInspectorMixedNumber(values);
}

/** @emoji 🔢 Builds an editable number-stepper field row, computing the mixed/uniform display from
 * `values` via {@link uiInspectorMixedNumber}. `action` is merged into both `onAbsolute` (typed
 * entry, dispatched with `{value}`) and `onDelta` (nudge buttons, `{delta}`) — the patch handler
 * branches on whichever key the dispatched action actually carries. */
export function uiInspectorStepperField(id: string, label: string, values: readonly number[], step: number, action: ActionDescriptor): UiFieldNode {
  const mixed = uiInspectorMixedNumber(values);
  return {
    type: "field",
    id,
    label,
    child: { type: "numberStepper", id, value: mixed.value, step, uniform: mixed.uniform, onAbsolute: action, onDelta: action },
  };
}

/** @emoji 🔘 Builds an editable boolean toggle field row, computing the mixed/uniform display from
 * `values` via {@link uiInspectorMixedToggle}. */
export function uiInspectorToggleField(id: string, label: string, iconId: string, values: readonly boolean[], action: ActionDescriptor): UiFieldNode {
  const mixed = uiInspectorMixedToggle(values);
  return {
    type: "field",
    id,
    label,
    child: { type: "toggle", id, iconId, pressed: mixed.pressed, onChange: action },
  };
}

/** @emoji 📐 Builds a nested `Origin`-style group: a parent tree item labeled `label` containing
 * three {@link uiInspectorStepperField} children (`X`/`Y`/`Z`), each computing its own per-axis
 * mixed state independently — a multi-selection that agrees on X but not Y shows only Y as "Mixed".
 * `axisAction(axis)` builds the per-axis {@link ActionDescriptor}; callers typically merge
 * `{field: "<id>.x"}` etc. into its `args` so the patch handler can dot-path into the right
 * component with `value` (absolute) or `delta` (relative, offset-preserving across multi-select). */
export function uiInspectorVec3Group(
  id: string,
  label: string,
  values: readonly (readonly [number, number, number])[],
  step: number,
  axisAction: (axis: "x" | "y" | "z") => ActionDescriptor,
): UiGroupNode {
  const xs = values.map((v) => v[0]);
  const ys = values.map((v) => v[1]);
  const zs = values.map((v) => v[2]);
  return {
    type: "group",
    id,
    label,
    defaultOpen: true,
    children: [
      uiInspectorStepperField(`${id}.x`, "X", xs, step, axisAction("x")),
      uiInspectorStepperField(`${id}.y`, "Y", ys, step, axisAction("y")),
      uiInspectorStepperField(`${id}.z`, "Z", zs, step, axisAction("z")),
    ],
  };
}

export function uiInspectorGroupsToTree(groups: readonly UiInspectorFieldGroup[]): UiTreeNode {
  return uiDeclarativeSectionsToTree(
    groups
      .filter((group) => group.fields.length > 0)
      .map((group) => ({
        type: "section" as const,
        id: group.id,
        label: group.label,
        defaultOpen: group.defaultOpen ?? true,
        children: group.fields,
      })),
  );
}

const UI_CONTROL_NODE_TYPES = new Set(["input", "select", "toggle", "button", "keyValue", "slider", "numberStepper", "ring", "iconSelect"]);

function isUiControlNode(node: UiNode): node is UiControlNode {
  return UI_CONTROL_NODE_TYPES.has(node.type);
}

export function uiDeclarativeSectionsToTree(sections: readonly UiSectionNode[]): UiTreeNode {
  const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
    id: section.id,
    label: section.label,
    defaultOpen: section.defaultOpen ?? true,
    items: section.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${section.id}.${index}`)),
  }));
  return {
    type: "tree",
    sections: treeSections.length ? treeSections : [{ id: "empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function uiDeclarativeChildToTreeItem(node: UiNode, fallbackId: string): UiTreeItemNode {
  if (node.type === "text") return { id: `${fallbackId}.text`, label: node.value };
  if (node.type === "field") {
    if (node.child.type === "text") return { id: node.id, label: node.label, description: node.child.value };
    return { id: node.id, label: node.label, control: isUiControlNode(node.child) ? node.child : undefined };
  }
  if (node.type === "button") return { id: node.id ?? fallbackId, label: node.label, control: node };
  if (node.type === "group") {
    return {
      id: node.id,
      label: node.label,
      defaultOpen: node.defaultOpen,
      items: node.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${node.id}.${index}`)),
    };
  }
  if (node.type === "input" || node.type === "select" || node.type === "toggle" || node.type === "keyValue" || node.type === "slider" || node.type === "numberStepper" || node.type === "ring" || node.type === "iconSelect") {
    return { id: "id" in node ? String(node.id) : fallbackId, label: "", control: node };
  }
  if (node.type === "separator") return { id: `${fallbackId}.sep`, label: "—" };
  return { id: fallbackId, label: node.type };
}

//#region PluginRuntime
/** 🧬 Generated from Rust `ActionKind`/`ActionDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionKind = GeneratedActionKind;
export type ActionDefinition = GeneratedActionDefinition;
export type ActionArgDef = GeneratedActionArgDef;
export type ActionArgControl = GeneratedActionArgControl;
export type ActionArgOption = GeneratedActionArgOption;
export type UtilityDefinition = GeneratedUtilityDefinition;
export type UtilityRef = GeneratedUtilityRef;

/** 🛠️ Generated from Rust `ToolDefinition`/`ToolRef` (`framework/core/rs/lib.rs`) — a mode-level,
 * activatable capability (e.g. puzzle3d fill), distinct from a per-window `UtilityDefinition`. See
 * `js/generated/manifest.ts`. */
export type ToolDefinition = GeneratedToolDefinition;
export type ToolRef = GeneratedToolRef;

/** 🎛️ Generated from Rust `CommandScope`/`CommandDefinition`/`CommandRef` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type CommandScope = GeneratedCommandScope;
export type CommandDefinition = GeneratedCommandDefinition;
export type CommandRef = GeneratedCommandRef;

/** 🧰 The framework-owned action id apps dispatch to activate a utility — mirrors `SET_ACTIVE_UTILITY_ACTION_ID`. */
export const SET_ACTIVE_UTILITY_ACTION_ID = "setActiveUtility";

/** 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — mirrors Rust `SET_ACTIVE_TOOL_ACTION_ID`. */
export const SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool";

/** 🎓 The framework-owned action id apps dispatch (or the shell auto-injects into the command palette)
 * to (re)start an app's introduction — mirrors Rust `START_INTRODUCTION_ACTION_ID`. */
export const START_INTRODUCTION_ACTION_ID = "startIntroduction";

/** 🎓 Generated from Rust `Introduction*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type IntroductionDefinition = GeneratedIntroductionDefinition;
export type IntroductionStepDefinition = GeneratedIntroductionStepDefinition;
export type IntroductionPlacement = GeneratedIntroductionPlacement;
export type IntroductionAdvance = GeneratedIntroductionAdvance;
export type IntroductionLogo = GeneratedIntroductionLogo;

/** 🗨️ Generated from Rust `DialogDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type DialogDefinition = GeneratedDialogDefinition;

//#region 🏷️ShellBrand
/** 🌐 Locales the shell chrome ships a complete translation bundle for — the single source `UiLocale`
 * (`ui/js/react`), `ShellBrandLocks.locale`, and `resolveShellLocks` all derive from. Adding a locale
 * means adding it here, which the ui-react schema asserts force a matching bundle for. */
export const SHELL_LOCALES = ["en", "de"] as const;
export type ShellLocale = (typeof SHELL_LOCALES)[number];

/** 🗣️ Chrome-known terminology ids; declaring a new terminology means adding it here plus its
 * `ui.settings.terminology.*` labels (an app-declared id beyond this set is still accepted at
 * runtime, falling back to its raw id in the settings dropdown — only these two are chrome-known). */
export const SHELL_TERMINOLOGIES = ["native", "reuse"] as const;
export type ShellTerminology = (typeof SHELL_TERMINOLOGIES)[number];

/** 🔒 Shell preferences a brand pins at boot: each set axis is fixed and its in-app switcher hidden (validated by the renderer's `resolveShellLocks`). */
export type ShellBrandLocks = {
  readonly exampleId?: string;
  readonly locale?: ShellLocale;
  readonly terminology?: ShellTerminology;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🎛️ Shell preferences a brand seeds at boot without pinning them: the value applies on first launch but the in-app switcher stays visible. */
export type ShellBrandDefaults = {
  readonly exampleId?: string;
};

/** 🏷️ Boot-time branding for a standalone shell artifact — identity (window title, logo mark, favicon), locked and defaulted shell preferences, and an optional brand-owned {@link IntroductionDefinition} replacing the app's own (already localized, rendered verbatim). */
export type ShellBrand = {
  readonly id: string;
  readonly windowTitle: string;
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  readonly locks?: ShellBrandLocks;
  readonly defaults?: ShellBrandDefaults;
  readonly introduction?: IntroductionDefinition;
  /** 🎓 When true, auto-starts the brand introduction on every window load and never persists a device-local "seen" flag. */
  readonly replayIntroductionOnLoad?: boolean;
  /** 🧊 When true, the shell never reads or writes device-local shell state (dock, panes, named layouts, chrome prefs, introduction seen) — every refresh boots from brand locks/defaults only. */
  readonly ephemeral?: boolean;
  /** 🗂️ Repo-root-relative directory of this brand's own static assets (logos, etc.) — the dev/build server mounts it as a static route at `/<assetsDir>` alongside the shared `ui/asset` mount. */
  readonly assetsDir?: string;
  /** 📦 Repo-root-relative directory this brand's build output lands in instead of the shared playground `dist/` — keeps a brand's specialization (including its build artifact) self-contained. */
  readonly distDir?: string;
  /** 🌐 Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `CNAME` file at the build root. */
  readonly cnameHost?: string;
};
//#endregion 🏷️ShellBrand

/** @emoji 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every app receives, used by the shell to render the same set without a wasm round trip. */
export const HISTORY_ACTION_IDS = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"] as const;

export type PluginViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  /** 🧰 Per-call overlay: host-owned active utility for the window targeted by this render/action (`windowId`). */
  readonly activeUtilityId?: string;
  /** 🧰 Host-owned active utility per window instance (never a document field, never a VCS op). */
  readonly activeUtilityByWindowId?: Readonly<Record<string, string>>;
  /** 🛠️ Host-owned active tool of the active mode (never a document field, never a VCS op) — mutually
   * exclusive with `activeUtilityId`: activating one clears the other. */
  readonly activeToolId?: string;
  readonly selectionJson?: string;
  readonly panelJson?: string;
  readonly contributionsJson?: string;
  readonly locale?: string;
  readonly terminology?: string;
  /** 🪟 The window instance a render/action call targets — plugins key per-window option state off this, never off `activeWindowKindId`. */
  readonly windowId?: string;
  /** 🪟 The live set of open window instances (base + spawned/split), so `windowMeasures`/`windowEngagements` can return one entry per instance. */
  readonly windowInstances?: readonly { readonly id: string; readonly windowKindId: string }[];
};

export type PluginUiNode = Record<string, unknown> & { readonly type: string };

/** 🗣️ Locale/terminology-aware label patch for an app's window-kind/panel-tab/mode labels, resolved fresh per {@link PluginViewState} — merge over the static {@link PluginManifest} app labels by id. */
export type PluginAppLabelsOverlay = {
  readonly windowKindLabels: Readonly<Record<string, string>>;
  readonly panelTabLabels: Readonly<Record<string, string>>;
  readonly modeLabels: Readonly<Record<string, string>>;
  readonly actionLabels: Readonly<Record<string, string>>;
  readonly utilityLabels: Readonly<Record<string, string>>;
  readonly exampleLabels: Readonly<Record<string, string>>;
  readonly actionArgLabels: Readonly<Record<string, string>>;
  readonly dialogLabels: Readonly<Record<string, string>>;
  readonly introductionLabels: Readonly<Record<string, string>>;
  readonly groupLabels: Readonly<Record<string, string>>;
};

export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** 🗣️ Rust's `skip_serializing_if` omits empty maps entirely, so a parsed overlay may be missing keys — fill them back in. */
export function normalizeAppLabelsOverlay(raw: Partial<PluginAppLabelsOverlay> | null | undefined): PluginAppLabelsOverlay {
  return {
    windowKindLabels: raw?.windowKindLabels ?? {},
    panelTabLabels: raw?.panelTabLabels ?? {},
    modeLabels: raw?.modeLabels ?? {},
    actionLabels: raw?.actionLabels ?? {},
    utilityLabels: raw?.utilityLabels ?? {},
    exampleLabels: raw?.exampleLabels ?? {},
    actionArgLabels: raw?.actionArgLabels ?? {},
    dialogLabels: raw?.dialogLabels ?? {},
    introductionLabels: raw?.introductionLabels ?? {},
    groupLabels: raw?.groupLabels ?? {},
  };
}

export type PluginContribution =
  | {
      readonly kind: "protocolBlockKind";
      readonly appId: string;
      readonly blockKind: string;
      readonly label: string;
      readonly iconId: string;
      readonly defaultValueJson?: string;
      readonly paramsBodyKey: string;
      readonly previewBodyKey: string;
    }
  | {
      readonly kind: "sourcingModule";
      readonly appId: string;
      readonly moduleId: string;
      readonly label: string;
      readonly iconId: string;
      readonly typologyJson: string;
      readonly kindsJson: string;
    };

export type PluginContributionEntry = {
  readonly pluginId: string;
  readonly contribution: PluginContribution;
};

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly Record<string, unknown>[];
  readonly programs: readonly {
    readonly programId: string;
    readonly appId: string;
    readonly label: string;
    readonly yields: string;
  }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly PluginContribution[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

//#region AppManifestProtocol
/** 🧬 Generated from Rust `WindowMeasure`/`WindowEngagement*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type WindowMeasure = GeneratedWindowMeasure;
export type WindowEngagementOption = GeneratedWindowEngagementOption;
export type WindowEngagementInput = GeneratedWindowEngagementInput;
export type WindowEngagementStatus = GeneratedWindowEngagementStatus;
export type WindowEngagementPossible = GeneratedWindowEngagementPossible;
export type WindowEngagementRingOption = GeneratedWindowEngagementRingOption;
export type WindowEngagementToggleGroupOption = GeneratedWindowEngagementToggleGroupOption;
export type WindowEngagementSelectItem = GeneratedWindowEngagementSelectItem;
export type WindowEngagementControl = GeneratedWindowEngagementControl;
export type WindowEngagement = GeneratedWindowEngagement;

/** 🌳 Mirrors Rust `PanelTabKind` — closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention: every panel tab is either a framework-predefined kind (exhaustively switchable) or an app-declared custom tab (`{ kind: "app", id }`). */
export type PanelTabKind = GeneratedPanelTabKind;
/** 🔤 Flat string key for a `PanelTabKind` — mirrors Rust `PanelTabKind::id_str()`. Use for React `key=` props and legacy string-id matching. */
export function panelTabKindId(kind: PanelTabKind): string {
  switch (kind.kind) {
    case "workbenchCategory":
      return "framework.category.workbench";
    case "displayCategory":
      return "framework.category.display";
    case "detailsCategory":
      return "framework.category.details";
    case "settingsCategory":
      return "framework.category.settings";
    case "displayWindows":
      return "framework.display.windows";
    case "displayLayout":
      return "framework.display.layout";
    case "settingsGeneral":
      return "framework.settings.general";
    case "settingsTheme":
      return "framework.settings.theme";
    case "app":
      return kind.id;
  }
}

/** 🌳 Mirrors Rust `PanelTabDefinition` — a leaf carries `bodyKey`, a branch carries `children`; `group` is only meaningful on root entries. */
export type AppPanelTabDefinition = GeneratedPanelTabDefinition;

/** 📦 Mirrors Rust `AppDefinition` — generated 1:1 from `framework/core/rs/lib.rs` via ts-rs, except
 * `defaultLayout`/`namedLayouts` which keep this file's narrower hand-refined `WindowLayout` (ts-rs
 * widens `WindowLayoutAxisNode.kind`/`WindowLayoutStackNode.kind` to plain `string` since the Rust
 * field is a runtime `String`, not an enum — the narrower `"row" | "column" | "stack" | "window"`
 * literal unions here are domain knowledge worth keeping for exhaustive switches). */
export type AppDefinition = Omit<GeneratedAppDefinition, "defaultLayout" | "namedLayouts"> & {
  readonly defaultLayout?: WindowLayout;
  readonly namedLayouts: readonly NamedLayout[];
};
export type AppModeDefinition = GeneratedModeDefinition;
export type AppWindowKindDefinition = GeneratedWindowKindDefinition;
export type AppWindowOptions = GeneratedWindowOptions;
export type AppWindowEngagementSlot = GeneratedWindowEngagementSlot;
export type AppActionRef = GeneratedActionRef;
export type AppPanelGroup = GeneratedPanelGroup;

export type PluginHotSwapEvent = {
  readonly pluginId: string;
  readonly version: string;
  readonly addedApps: readonly string[];
  readonly removedApps: readonly string[];
};
//#endregion AppManifestProtocol

//#region UiRefresh
/** @emoji 🐢 One requested window/panel section — `bodyKey` only applies to windows/panels; `hash` is the host's known fnv1a-64 hex of that section's last payload, or absent on first fetch. */
export type PluginUiRefreshSectionRequest = { readonly key: string; readonly bodyKey?: string; readonly hash?: string };

/** @emoji 🐢 One batched, hash-conditional refresh request — one round trip for the window/panel/engagements/measures/labels sections. Utility bars are no longer a plugin section: the renderer derives them from the utility registry via {@link deriveUtilityNodes}. */
export type PluginUiRefreshRequest = {
  readonly viewState: PluginViewState;
  readonly windows?: readonly PluginUiRefreshSectionRequest[];
  readonly panels?: readonly PluginUiRefreshSectionRequest[];
  readonly engagements?: { readonly hash?: string };
  readonly measures?: { readonly hash?: string };
  /** 🛠️ Mode-level tool measures, keyed by tool id — see `DocumentApp::tool_measures`. */
  readonly tools?: { readonly hash?: string };
  readonly labels?: { readonly hash?: string };
};

/** @emoji 🐢 `value` is present only when `hash` differs from what the request supplied — an unchanged section costs one hash compare instead of a full re-serialize. */
export type PluginUiRefreshSectionResponse = { readonly key: string; readonly hash: string; readonly value?: unknown };

export type PluginUiRefreshResponse = {
  readonly windows?: readonly PluginUiRefreshSectionResponse[];
  readonly panels?: readonly PluginUiRefreshSectionResponse[];
  readonly engagements?: PluginUiRefreshSectionResponse;
  readonly measures?: PluginUiRefreshSectionResponse;
  readonly tools?: PluginUiRefreshSectionResponse;
  readonly labels?: PluginUiRefreshSectionResponse;
  /** ⏱️ See `DocumentApp::pending_effects` — background work (e.g. a `flowEvalTick` chain) the host
   * should dispatch right after this refresh, fed through the same `applyHostEffects` pass as an
   * action's own `requestedEffects`. */
  readonly requestedEffects?: readonly HostEffect[];
};
//#endregion UiRefresh

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: PluginViewState) => Promise<InvocationResponse>;
  readonly handleCommand?: (instanceId: number, commandJson: string, viewState: PluginViewState) => Promise<InvocationResponse>;
  readonly applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
  readonly render: (instanceId: number, bodyKey: string, viewState: PluginViewState) => Promise<PluginUiNode>;
  readonly renderWithDocument?: (instanceId: number, bodyKey: string, viewState: PluginViewState, documentJson: string) => Promise<PluginUiNode>;
  readonly refreshUi: (instanceId: number, request: PluginUiRefreshRequest) => Promise<PluginUiRefreshResponse>;
  readonly dispose: () => void;
};

export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: PluginContributionEntry[] = [];
  for (const entry of loaded) {
    for (const contribution of entry.manifest.contributions ?? []) {
      entries.push({ pluginId: entry.pluginId, contribution });
    }
  }
  return JSON.stringify(entries);
}

export function resolveLayoutForMode(
  app: { readonly defaultLayout?: WindowLayout; readonly namedLayouts?: readonly NamedLayout[]; readonly modes: readonly { readonly id: string; readonly layoutId?: string }[] },
  modeId: string,
): WindowLayout | undefined {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return app.defaultLayout;
}

//#region 🧰ActionArgsAndUtilities
/** 🧰 A resolved utility ready for the utility bar — the TS twin of Rust `DerivedUtilitySpec` in `ui_wgpu`. */
export type DerivedUtilitySpec = {
  readonly id: string;
  readonly label: string;
  readonly iconId: string;
  readonly group?: string;
  readonly groupLabel?: string;
  readonly category?: UtilityCategory;
};

/**
 * 🧰 Hand-written twin of Rust `derive_utility_nodes` (`ui/wgpu/rs/lib.rs`): builds the utility bar node tree
 * from resolved utilities + the host-owned active utility id. Each utility becomes a `toggle` whose `pressed`
 * reflects `activeUtilityId === id` and whose `onChange` dispatches `setActiveUtility { utilityId }`; utilities
 * sharing a `group` collapse into one `collection` placed where the group first appears. A group that ends
 * with exactly one child is hoisted to a top-level toggle (no nested Transform/Transform pair).
 */
export function deriveUtilityNodes(controllerId: string, utilities: readonly DerivedUtilitySpec[], activeUtilityId?: string): UtilityNode[] {
  const toggle = (utility: DerivedUtilitySpec): UtilityNode => ({
    id: utility.id,
    kind: "toggle",
    iconId: utility.iconId,
    label: utility.label,
    title: utility.label,
    pressed: activeUtilityId === utility.id,
    category: utility.category,
    onChange: { controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { utilityId: utility.id } },
  });
  const nodes: UtilityNode[] = [];
  const groupIndex = new Map<string, number>();
  for (const utility of utilities) {
    const node = toggle(utility);
    if (utility.group === undefined) {
      nodes.push(node);
      continue;
    }
    const existing = groupIndex.get(utility.group);
    if (existing !== undefined) {
      const collection = nodes[existing] as Extract<UtilityNode, { kind: "collection" }>;
      (collection.children as UtilityNode[]).push(node);
    } else {
      groupIndex.set(utility.group, nodes.length);
      const groupLabel = utility.groupLabel ?? utility.group;
      nodes.push({ id: `group:${utility.group}`, kind: "collection", iconId: utility.iconId, label: groupLabel, title: groupLabel, category: utility.category, children: [node] });
    }
  }
  return nodes.map((node) => (node.kind === "collection" && node.children.length === 1 ? node.children[0]! : node));
}

/**
 * 🎯 Hand-written twin of Rust `partition_window_measures` (`ui/wgpu/rs/lib.rs`): splits a window's
 * top-level measures into `general` and `utilityOptions`. A top-level `group` tagged with `activeUtilityId`
 * contributes its **children** to `utilityOptions` only when it equals the window's active utility (the
 * tagged wrapper is routing-only and never rendered), and is dropped from both buckets otherwise. Untagged
 * groups and non-group top-level measures stay in `general`, unchanged.
 */
export function partitionWindowMeasures(measures: readonly WindowMeasure[], activeUtilityId?: string): { readonly general: WindowMeasure[]; readonly utilityOptions: WindowMeasure[] } {
  const general: WindowMeasure[] = [];
  const utilityOptions: WindowMeasure[] = [];
  for (const measure of measures) {
    if (measure.kind === "group" && measure.activeUtilityId !== undefined) {
      if (measure.activeUtilityId === activeUtilityId) utilityOptions.push(...measure.children);
      continue;
    }
    general.push(measure);
  }
  return { general, utilityOptions };
}

/**
 * 🧮 Hand-written twin of Rust `effective_action_args`: for each declared arg, the staged value if
 * present, else its declared `default`, else omitted.
 */
export function effectiveActionArgs(defs: readonly ActionArgDef[], staged: Readonly<Record<string, unknown>>): Record<string, unknown> {
  const effective: Record<string, unknown> = {};
  for (const def of defs) {
    if (Object.prototype.hasOwnProperty.call(staged, def.id)) {
      effective[def.id] = staged[def.id];
    } else if (def.default !== undefined && def.default !== null) {
      effective[def.id] = def.default;
    }
  }
  return effective;
}

/**
 * ❗ Hand-written twin of Rust `missing_required_args`: ids of required args still unset in `effective`
 * (absent, null, or an empty string).
 */
export function missingRequiredArgs(defs: readonly ActionArgDef[], effective: Readonly<Record<string, unknown>>): string[] {
  return defs
    .filter((def) => def.required)
    .filter((def) => {
      const value = effective[def.id];
      return value === undefined || value === null || value === "";
    })
    .map((def) => def.id);
}

/**
 * 📇 Hand-written twin of Rust `resolve_window_actions`: explicit `windowKind.actions` refs resolve in
 * order, plus any panel-eligible app action referenced by no window kind (an orphan) appears on every
 * window — the scoping fallback that prevents blank panels mid-migration. History and `setActiveUtility`
 * are never panel-eligible orphans.
 */
export function resolveWindowActions(
  app: { readonly actions?: readonly ActionDefinition[]; readonly windowKinds: readonly { readonly actions?: readonly AppActionRef[] }[] },
  windowKind: { readonly actions?: readonly AppActionRef[] },
): ActionDefinition[] {
  const actions = app.actions ?? [];
  const referenced = new Set<string>();
  for (const kind of app.windowKinds) {
    for (const ref of kind.actions ?? []) referenced.add(ref);
  }
  const panelEligible = (action: ActionDefinition) => action.kind !== "history" && action.id !== SET_ACTIVE_UTILITY_ACTION_ID && action.id !== SET_ACTIVE_TOOL_ACTION_ID;
  const resolved: ActionDefinition[] = [];
  const seen = new Set<string>();
  for (const ref of windowKind.actions ?? []) {
    const action = actions.find((entry) => entry.id === ref);
    if (action && !seen.has(action.id)) {
      seen.add(action.id);
      resolved.push(action);
    }
  }
  for (const action of actions) {
    if (panelEligible(action) && !referenced.has(action.id) && !seen.has(action.id)) {
      seen.add(action.id);
      resolved.push(action);
    }
  }
  return resolved;
}

/**
 * 🛠️ Hand-written twin of Rust `resolve_mode_tools`: resolves the active mode's tools in declared
 * order. Unlike `resolveWindowActions`, unresolvable or unreferenced tools have no orphan fallback —
 * tools are opt-in per mode, never automatically shown everywhere.
 */
export function resolveModeTools(
  app: { readonly tools?: readonly ToolDefinition[]; readonly modes: readonly { readonly id: string; readonly tools?: readonly ToolRef[] }[] } | undefined,
  activeModeId: string | undefined,
): ToolDefinition[] {
  const tools = app?.tools ?? [];
  const mode = app?.modes.find((entry) => entry.id === activeModeId);
  if (!mode) return [];
  const resolved: ToolDefinition[] = [];
  const seen = new Set<string>();
  for (const ref of mode.tools ?? []) {
    const tool = tools.find((entry) => entry.id === ref);
    if (tool && !seen.has(tool.id)) {
      seen.add(tool.id);
      resolved.push(tool);
    }
  }
  return resolved;
}
//#endregion 🧰ActionArgsAndUtilities

/**
 * 🧩 Expands a plugin registry for a primary plugin: `primaryPluginId` is matched directly
 * against entry `pluginId` (no registry-id indirection), then every other entry whose
 * `contributes` intersects the primary entry's `consumes` is appended. Studio mode, or the
 * absence of a primary id, passes the full registry through unchanged.
 */
export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, studioMode = false): readonly PluginRegistryEntry[] {
  if (studioMode || !primaryPluginId) return plugins;
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  return [...primaryEntries, ...contributorEntries];
}

export type ExternalSlotResolverContext = {
  readonly plugins: ReadonlyMap<string, PluginWasmHandle>;
  readonly contributorInstances: Map<string, number>;
  readonly viewState: PluginViewState;
};

export async function ensureContributorInstance(pluginId: string, appId: string, context: ExternalSlotResolverContext): Promise<number | null> {
  const existing = context.contributorInstances.get(pluginId);
  if (existing != null) return existing;
  const handle = context.plugins.get(pluginId);
  if (!handle) return null;
  const instanceId = await handle.createApp(appId);
  context.contributorInstances.set(pluginId, instanceId);
  return instanceId;
}

export async function resolveExternalSlots(node: PluginUiNode, context: ExternalSlotResolverContext): Promise<PluginUiNode> {
  if (node.type === "externalSlot") {
    const pluginId = String(node.pluginId ?? "");
    const appId = String(node.appId ?? pluginId);
    const bodyKey = String(node.bodyKey ?? "");
    const paramsJson = String(node.paramsJson ?? "{}");
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const rendered = handle.renderWithDocument ? await handle.renderWithDocument(instanceId, bodyKey, context.viewState, paramsJson) : await handle.render(instanceId, bodyKey, context.viewState);
    return resolveExternalSlots(rendered, context);
  }
  if (node.type === "stack" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  if (node.type === "section" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  return node;
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
};

//#region InvocationResponse
/** @emoji 🕰️ Hybrid logical clock stamp carried by every kernel operation. */
export type HybridLogicalTimestamp = { readonly wall: number; readonly counter: number };

/** @emoji 🩹 A schema-tagged document mutation payload (forward diff or inverse diff). */
export type DocumentDiff = { readonly schemaId: string; readonly payload: unknown };

/** @emoji ↩️ Undo semantics for a single kernel operation. */
export type UndoPolicy = "exactBaseOnly" | "transformAgainstConcurrent" | "semanticUndo" | "compensatingAction";

/** @emoji ↩️ The true inverse of a kernel operation, recorded from the store's `Edit.backwards`. */
export type InverseOperation = {
  readonly targetOperation: string;
  readonly inverseDiff: DocumentDiff;
  readonly baseVersion: number;
  readonly dependencies?: readonly string[];
  readonly undoPolicy: UndoPolicy;
};

/** @emoji 🔁 One typed document operation with its true inverse — the CQRS wire unit. */
export type KernelOperation = {
  readonly id: string;
  readonly document: number;
  readonly baseVersion: number;
  readonly invocationId: string;
  readonly diff: DocumentDiff;
  readonly inverse: InverseOperation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** @emoji 🎁 The undo group binding an invocation (action or command) to its operations + inverses. */
export type UndoGroup = {
  readonly invocationId: string;
  readonly operations: readonly string[];
  readonly inverseOperations: readonly InverseOperation[];
};

/** @emoji 📣 An out-of-band app event surfaced to the shell (e.g. history changed). */
export type AppEvent = { readonly kind: string; readonly payload: unknown };

/** @emoji 🩺 A diagnostic emitted alongside an action result. */
export type Diagnostic = { readonly level: string; readonly message: string };

/**
 * @emoji 🐚 A typed side effect the shell performs on the app's behalf. Mirrors the Rust
 * `HostEffect` enum (externally tagged: unit variants are the plain tag string, struct variants are
 * a single-key object keyed by the camelCase variant name).
 */
export type HostEffect =
  | "requestSync"
  | { readonly openWindow: { readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  /** @emoji 📂 Replaces the active app instance's document with a VCS envelope JSON — host-owned
   * counterpart of `loadAppDocument` for catalog/example studio opens. */
  | { readonly loadDocument: { readonly documentJson: string } }
  | { readonly openExternalUrl: { readonly url: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly accept: string; readonly readAs?: string; readonly importAction: string; readonly multiple?: boolean } }
  /** @emoji 🎞️ Asks the shell to decode a video (file picker, or `payload` bytes already in hand)
   * and re-dispatch `frameAction` once per sampled frame with `{payload: dataUrl(image/jpeg), name,
   * frameIndex, timestampMs, index, total, width, height, ...args}`, then `doneAction` once with
   * `{name, durationMs, frameCount, sampledCount, width, height, codec, ...args}`; if the host can't
   * decode it, `fallbackAction` fires once with `{payload: dataUrl(raw bytes), name, ...args}`. The
   * numeric hints (`sampleStride`/`maxFrames`/`maxLongEdgePx`/`fpsHint`) are 0 when the caller wants
   * the host default. */
  | {
      readonly requestMediaFrames: {
        readonly accept: string;
        readonly frameAction: string;
        readonly doneAction: string;
        readonly fallbackAction: string;
        readonly sampleStride?: number;
        readonly maxFrames?: number;
        readonly maxLongEdgePx?: number;
        readonly fpsHint?: number;
        readonly payload?: string;
        readonly args?: unknown;
      };
    }
  | { readonly spawnPluginInstance: { readonly programId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly documentJson?: string } }
  | { readonly openPluginInstance: { readonly programId: string; readonly appId: string; readonly osInstanceId?: string } }
  | { readonly setActiveUtility: { readonly windowId: string; readonly utilityId: string } }
  /** 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect form of
   * `setActiveTool`. Empty `toolId` deactivates the current tool. */
  | { readonly setActiveTool: { readonly toolId: string } }
  | { readonly openDialog: { readonly dialogId: string; readonly args?: Record<string, unknown> } }
  /** @emoji 🔁 Re-dispatches `action` onto the same plugin instance after `delayMs` — lets a plugin
   * advance staged/progressive work over several ticks without blocking the host; the response's own
   * `requestedEffects` are fed back through `applyHostEffects` recursively. */
  | { readonly dispatchAction: { readonly action: string; readonly args?: unknown; readonly delayMs: number } };

/**
 * @emoji 🐢 Mirrors the Rust `UiDirtyScope` — which rendered UI sections an action actually
 * invalidates. Absent (`undefined`) on an `InvocationResponse` means the same as the Rust side's missing
 * field: treat as `{kind: "full"}` (see {@link resolveUiDirtyScope}) — every plugin that doesn't emit
 * this yet keeps today's whole-shell-refresh behavior.
 */
export type UiDirtyScope =
  | { readonly kind: "full" }
  | { readonly kind: "none" }
  | {
      readonly kind: "partial";
      readonly windowBodies?: readonly string[];
      readonly panelBodies?: readonly string[];
      readonly utilities?: boolean;
      readonly tools?: boolean;
      readonly engagements?: boolean;
      readonly measures?: boolean;
      readonly labels?: boolean;
    };

/** @emoji 🐢 Normalizes a possibly-absent `UiDirtyScope` — missing (older plugin, or a response built without one) means `full`. */
export function resolveUiDirtyScope(scope: UiDirtyScope | undefined): UiDirtyScope {
  return scope ?? { kind: "full" };
}

/**
 * @emoji 📤 Typed result of a plugin `handle-action`/`handle-command` call — mirrors the Rust
 * `InvocationResult`. Replaces the legacy `string[]` JSON-patch shape: operations are now typed
 * `KernelOperation`s with true inverses, and the shell applies `requestedEffects` through
 * `applyHostEffects` (WS-E).
 */
export type InvocationResponse = {
  readonly output: unknown;
  readonly operations: readonly KernelOperation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly HostEffect[];
  readonly events?: readonly AppEvent[];
  readonly uiScope?: UiDirtyScope;
};

// 🐢 `uiScope` deliberately left unset here (not `{kind: "none"}`) — `resolveUiDirtyScope` treats a
// missing scope as `full`, the safe default for the rare failure paths that return this constant
// (unparseable response, stub module missing `handleAction`/`handleCommand`).
const EMPTY_INVOCATION_RESPONSE: InvocationResponse = {
  output: null,
  operations: [],
  inverseGroup: { invocationId: "", operations: [], inverseOperations: [] },
};

/** @emoji 📥 Parses a raw plugin `handle-action`/`handle-command` response string into a typed {@link InvocationResponse}. */
export function parseInvocationResponse(raw: string): InvocationResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<InvocationResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.operations)) {
      return parsed as InvocationResponse;
    }
  } catch {
    // fall through to the empty response
  }
  return EMPTY_INVOCATION_RESPONSE;
}
//#endregion InvocationResponse

//#region SerializedPluginWasm
/** @emoji 🔒 Serializes wasm plugin entry points — the host keeps instances in one RefCell. */
export function withSerializedPluginWasmHandle(handle: PluginWasmHandle): PluginWasmHandle {
  let tail: Promise<void> = Promise.resolve();
  const runSerialized = <T>(fn: () => Promise<T>): Promise<T> => {
    const job = tail.then(async () => {
      for (let attempt = 0; attempt < 8; attempt += 1) {
        try {
          return await fn();
        } catch (error) {
          const message = error instanceof Error ? error.message : String(error);
          if (!message.includes("plugin instance busy") && !message.includes("plugin busy")) throw error;
          await new Promise((resolve) => setTimeout(resolve, attempt + 1));
        }
      }
      return fn();
    });
    tail = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  };
  return {
    pluginId: handle.pluginId,
    manifest: handle.manifest,
    createApp: (appId) => runSerialized(() => handle.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => handle.destroyApp(instanceId)),
    handleAction: (instanceId, actionJson, viewState) => runSerialized(() => handle.handleAction(instanceId, actionJson, viewState)),
    handleCommand: handle.handleCommand ? (instanceId, commandJson, viewState) => runSerialized(() => handle.handleCommand!(instanceId, commandJson, viewState)) : undefined,
    render: (instanceId, bodyKey, viewState) => runSerialized(() => handle.render(instanceId, bodyKey, viewState)),
    renderWithDocument: handle.renderWithDocument ? (instanceId, bodyKey, viewState, documentJson) => runSerialized(() => handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) : undefined,
    refreshUi: (instanceId, request) => runSerialized(() => handle.refreshUi(instanceId, request)),
    applyOperations: handle.applyOperations ? (instanceId, operationsJson) => runSerialized(() => handle.applyOperations!(instanceId, operationsJson)) : undefined,
    readAppDocument: handle.readAppDocument ? (instanceId) => runSerialized(() => handle.readAppDocument!(instanceId)) : undefined,
    loadAppDocument: handle.loadAppDocument ? (instanceId, documentJson) => runSerialized(() => handle.loadAppDocument!(instanceId, documentJson)) : undefined,
    attachBackbone: handle.attachBackbone ? (instanceId, uri) => runSerialized(() => handle.attachBackbone!(instanceId, uri)) : undefined,
    detachBackbone: handle.detachBackbone ? (instanceId) => runSerialized(() => handle.detachBackbone!(instanceId)) : undefined,
    dispose: handle.dispose,
  };
}
//#endregion SerializedPluginWasm

//#region PluginWorkerClient
/** @emoji 🧵 Message types the generated `plugin-worker.js` dispatches (framework/product/os/dev/script.ts `pluginWorkerSource`). */
type PluginWorkerMessageType = "init" | "manifest" | "createApp" | "handleAction" | "handleCommand" | "render" | "destroy" | "refreshUi" | "error";

/** @emoji ⏱️ Logs only, never kills the worker — a plugin action owns in-flight, possibly undo-relevant
 * state, so abandoning it mid-call (the wgpu renderer's timeout+restart policy) would corrupt it. */
const PLUGIN_WORKER_UNRESPONSIVE_MS = 10000;

function pluginWorkerUrl(moduleUrl: string): string {
  return moduleUrl.replace(/\/[^/]+\.js$/, "/plugin-worker.js");
}

/**
 * @emoji 🧵 Runs a component-model plugin's WASM inside a Web Worker so `handleAction` — including
 * long-running precompute — never blocks the UI thread. Mirrors `framework/renderer/wgpu/js/boot.ts`'s
 * `PluginWorkerClient`, minus its 5s timeout+restart.
 */
class PluginWorkerClient {
  private worker: Worker | null = null;
  private readonly pending = new Map<string, { resolve: (value: Record<string, unknown>) => void; reject: (error: Error) => void; watchdog: number }>();
  onBackboneOutbound?: (uri: string, messageJson: string) => void;

  constructor(
    private readonly pluginId: string,
    private readonly moduleUrl: string,
  ) {}

  private clearPending(error: Error): void {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.watchdog);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }

  private attachWorker(worker: Worker): void {
    worker.onmessage = (event: MessageEvent) => {
      const message = event.data as {
        requestId?: string;
        type?: PluginWorkerMessageType | "backboneOutbound";
        uri?: string;
        message?: string;
      };
      if (message.type === "backboneOutbound" && message.uri && message.message != null) {
        this.onBackboneOutbound?.(message.uri, message.message);
        return;
      }
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
      window.clearTimeout(entry.watchdog);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `plugin worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] plugin worker ${this.pluginId} crashed`, error);
      this.worker = null;
      this.clearPending(new Error(`plugin worker ${this.pluginId} crashed`));
    };
  }

  async start(): Promise<void> {
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    await this.request("init", { moduleUrl: this.moduleUrl });
  }

  private request(type: PluginWorkerMessageType, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`plugin worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const watchdog = window.setTimeout(() => {
        console.warn(`[DEBUG] plugin worker ${this.pluginId} unresponsive for ${PLUGIN_WORKER_UNRESPONSIVE_MS}ms: ${type}`);
      }, PLUGIN_WORKER_UNRESPONSIVE_MS);
      this.pending.set(requestId, { resolve, reject, watchdog });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }

  async manifest(): Promise<string> {
    return String((await this.request("manifest", {})).value ?? "");
  }

  async createApp(appId: string): Promise<number> {
    return Number((await this.request("createApp", { appId })).instanceId);
  }

  async destroyApp(instanceId: number): Promise<void> {
    await this.request("destroy", { instanceId });
  }

  async handleAction(instanceId: number, actionJson: string, contextJson: string): Promise<string> {
    return String((await this.request("handleAction", { instanceId, actionJson, contextJson })).value ?? "{}");
  }

  async handleCommand(instanceId: number, commandJson: string, contextJson: string): Promise<string> {
    return String((await this.request("handleCommand", { instanceId, commandJson, contextJson })).value ?? "{}");
  }

  async render(instanceId: number, bodyKey: string, viewStateJson: string, documentJson?: string): Promise<string> {
    return String((await this.request("render", { instanceId, bodyKey, viewStateJson, documentJson })).value ?? "{}");
  }

  async refreshUi(instanceId: number, requestJson: string): Promise<string> {
    return String((await this.request("refreshUi", { instanceId, requestJson })).value ?? "{}");
  }

  dispose(): void {
    this.clearPending(new Error(`plugin worker ${this.pluginId} disposed`));
    this.worker?.terminate();
    this.worker = null;
  }

  postBackboneInbound(uri: string, messages: readonly string[]): void {
    this.worker?.postMessage({ type: "backboneInbound", uri, messages });
  }
}

/**
 * @emoji 🧵 Worker-backed `PluginWasmHandle` for component-model plugins (the ABI the generated
 * `plugin-worker.js` supports). Caller falls back to the direct main-thread import on failure (no
 * `plugin-worker.js` alongside this module, wasm-bindgen-only plugin, or `Worker` unavailable).
 */
const pluginWorkerClients = new Map<string, PluginWorkerClient>();

async function loadPluginModuleViaWorker(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const client = new PluginWorkerClient(pluginId, moduleUrl);
  pluginWorkerClients.set(pluginId, client);
  client.onBackboneOutbound = (uri, messageJson) => relayPluginBackboneOutbound(uri, messageJson);
  await client.start();
  const manifest = JSON.parse(await client.manifest()) as PluginManifest;
  return withSerializedPluginWasmHandle({
    pluginId,
    manifest,
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    handleAction: async (instanceId, actionJson, viewState) => parseInvocationResponse(await client.handleAction(instanceId, actionJson, JSON.stringify({ viewState, actor: "local" }))),
    handleCommand: async (instanceId, commandJson, viewState) => parseInvocationResponse(await client.handleCommand(instanceId, commandJson, JSON.stringify({ viewState, actor: "local" }))),
    render: async (instanceId, bodyKey, viewState) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode,
    renderWithDocument: async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await client.render(instanceId, bodyKey, JSON.stringify(viewState), documentJson)) as PluginUiNode,
    refreshUi: async (instanceId, request) => JSON.parse(await client.refreshUi(instanceId, JSON.stringify(request))) as PluginUiRefreshResponse,
    dispose: () => {
      pluginWorkerClients.delete(pluginId);
      client.dispose();
    },
  });
}
//#endregion PluginWorkerClient

export function relayPluginBackboneOutbound(uri: string, messageJson: string): void {
  pluginBackboneOutboundRelay?.(uri, messageJson);
}

/** @emoji 🌉 A direct-import (main-thread, no-worker) plugin's generated `host-shim.js` runs in this
 * same realm but can't import from this module, so it reaches the outbound relay through this
 * well-known global instead — the same relay a worker-backed plugin reaches via `postMessage`. */
(globalThis as unknown as { __semioMainThreadPluginBackboneOutbound?: (uri: string, messageJson: string) => void }).__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;

/** @emoji 🌉 Inbound counterpart: pushes straight into the same global queue a direct-import plugin's
 * `host-shim.js` `backbonePoll` drains, keyed by `uri` (globally unique per document, so no pluginId
 * scoping is needed even though several plugins may share this realm). */
function pushMainThreadPluginBackboneInbound(uri: string, messages: readonly string[]): void {
  const bridge = globalThis as unknown as { __semioBackboneInbound?: Map<string, string[]> };
  const queue = bridge.__semioBackboneInbound ?? new Map<string, string[]>();
  queue.set(uri, [...(queue.get(uri) ?? []), ...messages]);
  bridge.__semioBackboneInbound = queue;
}

export function postPluginBackboneInbound(pluginId: string, uri: string, messages: readonly string[]): void {
  const client = pluginWorkerClients.get(pluginId);
  if (client) {
    client.postBackboneInbound(uri, messages);
    return;
  }
  pushMainThreadPluginBackboneInbound(uri, messages);
}

let pluginBackboneOutboundRelay: ((uri: string, messageJson: string) => void) | null = null;

export function setPluginBackboneOutboundRelay(relay: ((uri: string, messageJson: string) => void) | null): void {
  pluginBackboneOutboundRelay = relay;
}

const pluginModuleHandleCache = new Map<string, Promise<PluginWasmHandle>>();

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const cached = pluginModuleHandleCache.get(moduleUrl);
  if (cached) return cached;
  const pending = loadPluginModuleUncached(pluginId, moduleUrl);
  pluginModuleHandleCache.set(moduleUrl, pending);
  try {
    return await pending;
  } catch (error) {
    pluginModuleHandleCache.delete(moduleUrl);
    throw error;
  }
}

async function loadPluginModuleUncached(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  // 🧵 Worker-backed by default so a plugin's `handleAction` (e.g. puzzle-3d's collision precompute) can
  // never block the UI thread. Falls back to the direct main-thread import below when unavailable: no
  // `Worker` global (vitest/node), no `plugin-worker.js` alongside this module, or a wasm-bindgen-only
  // plugin (the worker template only supports the `createPluginApi` component-model ABI).
  if (typeof Worker !== "undefined") {
    try {
      return await loadPluginModuleViaWorker(pluginId, moduleUrl);
    } catch (error) {
      console.warn(`[DEBUG] plugin ${pluginId} worker-backed load failed, falling back to main thread: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  const module = (await import(/* @vite-ignore */ moduleUrl)) as {
    default?: () => Promise<void> | void;
    createPluginApi?: () => Promise<{
      manifest: () => Promise<string>;
      createApp: (appId: string) => Promise<number>;
      destroyApp?: (instanceId: number) => Promise<void>;
      handleAction: (instanceId: number, actionJson: string, contextJson: string) => Promise<string>;
      handleCommand?: (instanceId: number, commandJson: string, contextJson: string) => Promise<string>;
      render: (instanceId: number, bodyKey: string, viewStateJson: string) => Promise<string>;
      renderWithDocument?: (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => Promise<string>;
      refreshUi: (instanceId: number, requestJson: string) => Promise<string>;
      applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;
      readAppDocument?: (instanceId: number) => Promise<string>;
      loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
      attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
      detachBackbone?: (instanceId: number) => Promise<void>;
    }>;
    semio_plugin_manifest?: () => string;
    semio_plugin_create_app?: (appId: string) => number;
    semio_plugin_destroy_app?: (instanceId: number) => void;
    semio_plugin_handle_action?: (instanceId: number, actionJson: string, viewStateJson: string) => string;
    semio_plugin_handle_command?: (instanceId: number, commandJson: string, viewStateJson: string) => string;
    semio_plugin_render?: (instanceId: number, bodyKey: string, viewStateJson: string) => string;
    semio_plugin_refresh_ui?: (instanceId: number, requestJson: string) => string;
    semio_plugin_apply_operations?: (instanceId: number, operationsJson: string) => void;
    semio_plugin_read_app_document?: (instanceId: number) => string;
    semio_plugin_load_app_document?: (instanceId: number, documentJson: string) => void;
    semio_plugin_attach_backbone?: (instanceId: number, uri: string) => void;
    semio_plugin_detach_backbone?: (instanceId: number) => void;
  };
  if (module.default) await module.default();
  if (module.createPluginApi) {
    const api = await module.createPluginApi();
    const manifest = JSON.parse(await api.manifest()) as PluginManifest;
    return withSerializedPluginWasmHandle({
      pluginId,
      manifest,
      createApp: (appId) => api.createApp(appId),
      destroyApp: async (instanceId) => {
        await api.destroyApp?.(instanceId);
      },
      handleAction: async (instanceId, actionJson, viewState) => {
        const raw = await api.handleAction(instanceId, actionJson, JSON.stringify(viewState));
        return parseInvocationResponse(raw);
      },
      handleCommand: api.handleCommand
        ? async (instanceId, commandJson, viewState) => {
            const raw = await api.handleCommand!(instanceId, commandJson, JSON.stringify(viewState));
            return parseInvocationResponse(raw);
          }
        : undefined,
      render: async (instanceId, bodyKey, viewState) => JSON.parse(await api.render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode,
      renderWithDocument: api.renderWithDocument ? async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await api.renderWithDocument!(instanceId, bodyKey, JSON.stringify(viewState), documentJson)) as PluginUiNode : undefined,
      refreshUi: async (instanceId, request) => JSON.parse(await api.refreshUi(instanceId, JSON.stringify(request))) as PluginUiRefreshResponse,
      applyOperations: api.applyOperations ? (instanceId, operationsJson) => api.applyOperations!(instanceId, operationsJson) : undefined,
      readAppDocument: api.readAppDocument ? (instanceId) => api.readAppDocument!(instanceId) : undefined,
      loadAppDocument: api.loadAppDocument ? (instanceId, documentJson) => api.loadAppDocument!(instanceId, documentJson) : undefined,
      attachBackbone: api.attachBackbone ? (instanceId, uri) => api.attachBackbone!(instanceId, uri) : undefined,
      detachBackbone: api.detachBackbone ? (instanceId) => api.detachBackbone!(instanceId) : undefined,
      dispose() {},
    });
  }
  if (!module.semio_plugin_manifest) {
    throw new Error(`[DEBUG] plugin ${pluginId} missing semio_plugin_manifest export`);
  }
  const manifest = JSON.parse(module.semio_plugin_manifest()) as PluginManifest;
  return withSerializedPluginWasmHandle({
    pluginId,
    manifest,
    async createApp(appId: string) {
      const create = module.semio_plugin_create_app;
      if (!create) throw new Error(`plugin ${pluginId} missing create_app`);
      return create(appId);
    },
    async destroyApp(instanceId: number) {
      module.semio_plugin_destroy_app?.(instanceId);
    },
    async handleAction(instanceId: number, actionJson: string, viewState: PluginViewState) {
      const handle = module.semio_plugin_handle_action;
      if (!handle) return EMPTY_INVOCATION_RESPONSE;
      const raw = handle(instanceId, actionJson, JSON.stringify(viewState));
      return parseInvocationResponse(raw);
    },
    async handleCommand(instanceId: number, commandJson: string, viewState: PluginViewState) {
      const handle = module.semio_plugin_handle_command;
      if (!handle) return EMPTY_INVOCATION_RESPONSE;
      const raw = handle(instanceId, commandJson, JSON.stringify(viewState));
      return parseInvocationResponse(raw);
    },
    async render(instanceId: number, bodyKey: string, viewState: PluginViewState) {
      const render = module.semio_plugin_render;
      if (!render) throw new Error(`plugin ${pluginId} missing render`);
      return JSON.parse(render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode;
    },
    async refreshUi(instanceId: number, request: PluginUiRefreshRequest) {
      const refreshUi = module.semio_plugin_refresh_ui;
      if (!refreshUi) return {};
      return JSON.parse(refreshUi(instanceId, JSON.stringify(request))) as PluginUiRefreshResponse;
    },
    applyOperations: module.semio_plugin_apply_operations
      ? async (instanceId, operationsJson) => {
          module.semio_plugin_apply_operations!(instanceId, operationsJson);
        }
      : undefined,
    readAppDocument: module.semio_plugin_read_app_document ? async (instanceId) => module.semio_plugin_read_app_document!(instanceId) : undefined,
    loadAppDocument: module.semio_plugin_load_app_document
      ? async (instanceId, documentJson) => {
          module.semio_plugin_load_app_document!(instanceId, documentJson);
        }
      : undefined,
    attachBackbone: module.semio_plugin_attach_backbone
      ? async (instanceId, uri) => {
          module.semio_plugin_attach_backbone!(instanceId, uri);
        }
      : undefined,
    detachBackbone: module.semio_plugin_detach_backbone
      ? async (instanceId) => {
          module.semio_plugin_detach_backbone!(instanceId);
        }
      : undefined,
    dispose() {},
  });
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return loadPluginModule(pluginId, moduleUrl);
}

export function pluginHandleForBridge(handle: PluginWasmHandle) {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    createApp: (appId: string) => handle.createApp(appId),
    destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
    handleAction: (instanceId: number, actionJson: string, viewStateJson: string) => handle.handleAction(instanceId, actionJson, JSON.parse(viewStateJson) as PluginViewState).then((ops) => JSON.stringify(ops)),
    handleCommand: handle.handleCommand ? (instanceId: number, commandJson: string, viewStateJson: string) => handle.handleCommand!(instanceId, commandJson, JSON.parse(viewStateJson) as PluginViewState).then((ops) => JSON.stringify(ops)) : undefined,
    render: (instanceId: number, bodyKey: string, viewStateJson: string) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson) as PluginViewState).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument
      ? (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => handle.renderWithDocument!(instanceId, bodyKey, JSON.parse(viewStateJson) as PluginViewState, documentJson).then((node) => JSON.stringify(node))
      : undefined,
    refreshUi: (instanceId: number, requestJson: string) => handle.refreshUi(instanceId, JSON.parse(requestJson) as PluginUiRefreshRequest).then((response) => JSON.stringify(response)),
  };
}
//#endregion PluginRuntime

// #region 🎮PlaygroundResolution
/** @emoji 🎮 Finds the generated playground catalog row for a variant id or one of its aliases. */
function findPlaygroundVariant(playgroundPluginId: string): PlaygroundBuildTarget | undefined {
  return PLAYGROUND_BUILD_TARGETS.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}

/** @emoji 🎯 Resolves a playground filter/alias (e.g. "3d", "sourcing") to its underlying wasm component registry id. */
export function resolvePluginRegistryId(playgroundPluginId: string): string {
  return findPlaygroundVariant(playgroundPluginId)?.pluginId ?? playgroundPluginId;
}

/** @emoji 🎯 Resolves a playground filter/alias to the app id that should be instantiated by default within its plugin's manifest. */
export function resolvePlaygroundDefaultAppId(playgroundPluginId: string): string | undefined {
  return findPlaygroundVariant(playgroundPluginId)?.app;
}

export type PlaygroundBootSession = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

export type PlaygroundBoot = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

/** @emoji 🎮 Resolves the wasm plugin list and default app for one playground variant; when the on-disk
 * `generated/session.ts` was overwritten by another concurrent dev variant, rebuilds from the generated
 * plugin catalog instead of trusting the stale plugin rows. */
export function resolvePlaygroundBoot(variant: string, session?: PlaygroundBootSession): PlaygroundBoot {
  const defaultAppId = resolvePlaygroundDefaultAppId(variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins };
  }
  const registryPluginId = resolvePluginRegistryId(variant);
  const studioMode = resolvePluginHostConfig(variant) !== undefined;
  const catalogPlugins: PluginRegistryEntry[] = PLUGIN_BUILD_TARGETS.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
  }));
  return {
    variant,
    defaultAppId,
    plugins: expandPluginRegistry(catalogPlugins, studioMode ? undefined : registryPluginId, studioMode),
  };
}

//#region 🏠🧳PluginHostConfig
/** 🏠🧳 Declares, for a plugin whose manifest offers a host-style multi-app experience (one app is the
 * landing/default view, another hosts other apps as spawned sub-instances — e.g. "s"'s home/studio
 * pair), which app ids play which role. Callers resolve controller ids and default panel tabs from
 * the *loaded manifest*'s own `controllerId`/`panelTabs` on those apps rather than hardcoding separate
 * literals — this table only ever needs to carry app-id role assignments. A pluginFilter absent here
 * simply boots through the ordinary single-app path (`resolvePlaygroundDefaultAppId`). Mirrored by
 * `PLUGIN_HOST_CONFIGS`/`resolve_plugin_host_config` in `framework/renderer/wgpu/rs/lib.rs`'s
 * `plugin_bridge` module for the WGPU renderer. */
export type PluginHostConfig = {
  readonly pluginId: string;
  readonly landingAppId: string;
  readonly hostAppId: string;
};

/** 🎯 Resolves a playground filter/alias to its plugin's host config, or `undefined` when that plugin doesn't offer a host-style multi-app experience. */
export function resolvePluginHostConfig(playgroundPluginId: string): PluginHostConfig | undefined {
  const registryId = resolvePluginRegistryId(playgroundPluginId);
  return PLUGIN_HOST_CONFIGS.find((entry) => entry.pluginId === registryId);
}
//#endregion 🏠🧳PluginHostConfig
// #endregion 🎮PlaygroundResolution

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("DockLayoutStore", () => {
    const emptySkeleton = (): DockSkeleton => ({
      version: 3,
      anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
    });

    it("returns null when nothing persisted", () => {
      const store = new DockLayoutStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      const appSkeleton: DockSkeleton = { ...emptySkeleton(), anchors: { ...emptySkeleton().anchors, "top-left": [{ id: "a" }] } };
      store.saveOs(osSkeleton);
      store.save(appSkeleton);
      expect(store.getSnapshot()).toEqual(appSkeleton);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      store.saveOs(osSkeleton);
      expect(store.getSnapshot()).toEqual(osSkeleton);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.save(emptySkeleton());
      expect(storage.get("semio.os.dock.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.dock.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.saveOs(emptySkeleton());
      store.save(emptySkeleton());
      store.reset();
      expect(storage.get("semio.os.dock")).toBeNull();
      expect(storage.get("semio.os.dock.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", "{not json");
      const store = new DockLayoutStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", JSON.stringify({ version: 1, corners: { "top-left": [{ id: "a" }], "top-right": [], "bottom-left": [], "bottom-right": [] } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      storage.set(
        "semio.os.dock",
        JSON.stringify({ version: 2, anchors: { "top-left": [{ id: "a" }], "top-middle": [], "top-right": [], "bottom-left": [], "bottom-middle": [], "bottom-right": [] } }),
      );
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("DockUiStateStore", () => {
    const emptyUiState = (): DockUiState => ({ version: 3, anchors: {} });

    it("returns null when nothing persisted", () => {
      const store = new DockUiStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = emptyUiState();
      const appState: DockUiState = { ...emptyUiState(), anchors: { "top-left": { visible: true, size: 320 } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState: DockUiState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.document" } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.save(emptyUiState());
      expect(storage.get("semio.os.dockUi.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.dockUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.saveOs(emptyUiState());
      store.save(emptyUiState());
      store.reset();
      expect(storage.get("semio.os.dockUi")).toBeNull();
      expect(storage.get("semio.os.dockUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", "{not json");
      const store = new DockUiStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", JSON.stringify({ version: 1, corners: { "top-left": { visible: true, size: 320 } } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", JSON.stringify({ version: 2, anchors: { "top-left": { visible: true, size: 320 } } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it('uses a distinct key from DockLayoutStore for an app literally named "ui"', () => {
      const storage = createMemoryStoragePort();
      new DockLayoutStore(storage, "ui").save({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      });
      new DockUiStateStore(storage).saveOs(emptyUiState());
      expect(storage.get("semio.os.dock.ui")).not.toBeNull();
      expect(storage.get("semio.os.dockUi")).not.toBeNull();
      expect(storage.get("semio.os.dock.ui")).not.toEqual(storage.get("semio.os.dockUi"));
    });
  });

  describe("WindowPaneStateStore", () => {
    const emptyPaneState = (): WindowPaneUiState => ({ version: 1, windows: {} });

    it("returns null when nothing persisted", () => {
      const store = new WindowPaneStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = emptyPaneState();
      const appState: WindowPaneUiState = { version: 1, windows: { "puzzle3d.play": { utilities: { anchor: "bottom-left", folded: false, size: 280 } } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState: WindowPaneUiState = { version: 1, windows: { "puzzle3d.play": { measures: { anchor: "top-right", size: 320 } } } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.save(emptyPaneState());
      expect(storage.get("semio.os.paneUi.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.paneUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.saveOs(emptyPaneState());
      store.save(emptyPaneState());
      store.reset();
      expect(storage.get("semio.os.paneUi")).toBeNull();
      expect(storage.get("semio.os.paneUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.paneUi", "{not json");
      const store = new WindowPaneStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a foreign-version blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.paneUi", JSON.stringify({ version: 2, windows: {} }));
      const store = new WindowPaneStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("PlaygroundResolution", () => {
    it("resolves host config from generated plugin metadata", () => {
      expect(resolvePluginHostConfig("s")).toEqual({ pluginId: "s", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig("puzzle3d")).toBeUndefined();
    });

    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId("aggregator")).toBe("puzzle");
      expect(resolvePluginRegistryId("3d")).toBe("puzzle");
    });

    it("rebuilds plugin rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot("aggregator", {
        variant: "sourcing",
        defaultAppId: "sourcing-curate",
        plugins: [{ pluginId: "sourcing", moduleUrl: "/plugin-modules/sourcing/sourcing_plugin.js" }],
      });
      expect(boot.variant).toBe("aggregator");
      expect(boot.defaultAppId).toBe("puzzle3d-play");
      expect(boot.plugins).toEqual([{ pluginId: "puzzle", moduleUrl: "/plugin-modules/puzzle/puzzle_plugin.js", contributes: [], consumes: [] }]);
    });
  });
}
//#endregion 🧪Tests
