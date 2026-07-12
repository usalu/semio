// #region 🧲Header
/** @emoji 🧭 `@semio-tech/framework-core` — shared canvas pick helpers, layout factories, and inspector utilities for UI renderers. */
// #endregion 🧲Header

// #region 🧬GeneratedMirror
/** 🧬 Types generated from `framework/core/rs/lib.rs` via ts-rs (`bun nx run @semio-tech/framework-core:generate`); re-exported below alongside their hand-written neighbors so this stays the one import surface. */
import type {
  ActionDescriptor as GeneratedActionDescriptor,
  ActionKind as GeneratedActionKind,
  ActionDefinition as GeneratedActionDefinition,
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

export type ToolCategory = "selection" | "tools" | "actions" | "history" | "sync";

export type ToolLeaf =
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
      readonly category?: ToolCategory;
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
      readonly category?: ToolCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    };

export type ToolNode =
  | ToolLeaf
  | {
      readonly id: string;
      readonly kind: "collection";
      readonly iconId: string;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: ToolCategory;
      readonly children: readonly ToolNode[];
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
      readonly category?: ToolCategory;
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
      readonly category?: ToolCategory;
      readonly onChange: ActionDescriptor;
    };

export type UiSectionNode = {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
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
  readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
};

export type UiControlNode = UiInputNode | UiSelectNode | UiToggleNode | UiVec3Node | UiButtonNode | UiKeyValueNode | UiSliderNode | UiNumberStepperNode | UiRingNode | UiIconSelectNode;

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

export type UiVec3Node = {
  readonly type: "vec3";
  readonly id: string;
  readonly value: readonly [number, number, number] | null;
  readonly onChange: ActionDescriptor;
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
  readonly activate?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly children: readonly UiNode[];
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
  | UiVec3Node
  | UiKeyValueNode
  | UiSliderNode
  | UiNumberStepperNode
  | UiRingNode
  | UiIconSelectNode
  | UiFieldNode
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

/** 🖌️ A raster/paint surface scene payload — mirrors the wasm `componentScene` node's `raster` field. */
export type RasterScene = {
  readonly documentSyncJson: string;
  readonly assetsJson: string;
  readonly cameraJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeTool: string;
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

/** 🗺️ A GIS map surface scene payload — mirrors the wasm `componentScene` node's `gisMap` field. */
export type GisMapScene = {
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

/** 🧩 A puzzle-2d board surface scene payload — mirrors the wasm `componentScene` node's `puzzle2dBoard` field. */
export type Puzzle2dBoardScene = {
  readonly fixtureJson: string;
  readonly cameraJson: string;
  readonly kindCatalogsJson: string;
  readonly selectionJson: string;
  readonly interactive: boolean;
  readonly hoveredId?: string;
  readonly activeTool?: string;
  readonly selectionMethod: string;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  readonly suggestionOffset: number;
  readonly brushKindWeightsJson: string;
  readonly kindCompatibilityJson: string;
  readonly lodMode: string;
};

/** 🗒️ A note-canvas surface scene payload — mirrors the wasm `componentScene` node's `noteCanvas` field. */
export type NoteCanvasScene = {
  readonly documentJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeTool: string;
  readonly viewMode: string;
  readonly interactive: boolean;
};

/** 🗄️ A checkpoint ancestor-graph history view. `columnsJson` is a `HistoryColumn[]` array, newest checkpoint first. */
export type VcsHistoryScene = {
  readonly columnsJson: string;
};

/** 🧩 A palette entry for a block kind insertable into a {@link ProtocolListScene}, contributed either by the host app's own built-ins or by a `protocolBlockKind` module contribution. */
export type ProtocolPaletteEntry = {
  readonly blockKind: string;
  readonly label: string;
  readonly iconId: string;
};

/** 🧩 A strict, ordered list of steps/blocks for the Blockly-like list editor. `stepsJson` is a `ProtocolStep[]` array, `paletteJson` is a `ProtocolPaletteEntry[]` array of the block kinds available to insert. */
export type ProtocolListScene = {
  readonly stepsJson: string;
  readonly paletteJson: string;
  readonly selectedId?: string;
  readonly draggingId?: string;
};

/** 🔌 A plugin-contributed external body rendered inline — mirrors the wasm `externalSlot` node. */
export type UiExternalSlotNode = {
  readonly type: "externalSlot";
  readonly pluginId: string;
  readonly appId: string;
  readonly bodyKey: string;
  readonly paramsJson: string;
};

/** 🧭 The dispatch key on {@link UiComponentSceneNode} — matches the lazy-loaded host component per `framework/renderer/react/components/*-host.tsx`. */
export type ComponentKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "raster" | "gis2d-map" | "puzzle2d-board" | "icon-render" | "note-canvas" | "vcs-history" | "protocol-list";

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
  readonly raster?: RasterScene;
  readonly virtualFileSystem?: VirtualFileSystemScene;
  readonly gisMap?: GisMapScene;
  readonly puzzle2dBoard?: Puzzle2dBoardScene;
  readonly iconRender?: IconRenderScene;
  readonly noteCanvas?: NoteCanvasScene;
  readonly vcsHistory?: VcsHistoryScene;
  readonly protocolList?: ProtocolListScene;
};

/** 🧷 Shared prop shape for every `framework/renderer/react/components/*-host.tsx` component. */
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

export function uiInspectorMixedVec3(values: readonly (readonly [number, number, number])[]): { readonly value: readonly [number, number, number] | null; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values.map((row) => JSON.stringify(row)));
  return { value: uniform && values[0] ? values[0] : null, uniform };
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
    return { id: node.id, label: node.label, control: node.child };
  }
  if (node.type === "button") return { id: node.id ?? fallbackId, label: node.label, control: node };
  if (node.type === "input" || node.type === "select" || node.type === "toggle" || node.type === "vec3" || node.type === "keyValue" || node.type === "slider" || node.type === "numberStepper" || node.type === "ring" || node.type === "iconSelect") {
    return { id: "id" in node ? String(node.id) : fallbackId, label: "", control: node };
  }
  if (node.type === "separator") return { id: `${fallbackId}.sep`, label: "—" };
  return { id: fallbackId, label: node.type };
}

//#region PluginRuntime
/** 🧬 Generated from Rust `ActionKind`/`ActionDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionKind = GeneratedActionKind;
export type ActionDefinition = GeneratedActionDefinition;

/** @emoji 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every app receives, used by the shell to render the same set without a wasm round trip. */
export const HISTORY_ACTION_IDS = [
  "undo",
  "redo",
  "commitCheckpoint",
  "createAlternative",
  "switchAlternative",
  "checkoutCheckpoint",
] as const;

export type PluginViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  readonly selectionJson?: string;
  readonly panelJson?: string;
  readonly contributionsJson?: string;
  readonly locale?: string;
  readonly terminology?: string;
};

export type PluginUiNode = Record<string, unknown> & { readonly type: string };

/** 🗣️ Locale/terminology-aware label patch for an app's window-kind/panel-tab/mode labels, resolved fresh per {@link PluginViewState} — merge over the static {@link PluginManifest} app labels by id. */
export type PluginAppLabelsOverlay = {
  readonly appLabel?: string;
  readonly windowKindLabels: Readonly<Record<string, string>>;
  readonly panelTabLabels: Readonly<Record<string, string>>;
  readonly modeLabels: Readonly<Record<string, string>>;
};

const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {} };

/** 🗣️ Rust's `skip_serializing_if` omits empty maps entirely, so a parsed overlay may be missing keys — fill them back in. */
function normalizeAppLabelsOverlay(raw: Partial<PluginAppLabelsOverlay> | null | undefined): PluginAppLabelsOverlay {
  return {
    appLabel: raw?.appLabel,
    windowKindLabels: raw?.windowKindLabels ?? {},
    panelTabLabels: raw?.panelTabLabels ?? {},
    modeLabels: raw?.modeLabels ?? {},
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

/** 🌳 Mirrors Rust `PanelTabDefinition` — a leaf carries `bodyKey`, a branch carries `children`; `group` is only meaningful on root entries. */
export type AppPanelTabDefinition = {
  readonly id: string;
  readonly label: string;
  readonly group: string;
  readonly bodyKey?: string;
  readonly children?: readonly AppPanelTabDefinition[];
};

/** 📦 The richly-typed shape of one {@link PluginManifest} `apps` entry — additive superset kept alongside the loosely-typed wire shape so renderers can opt into precise field access. */
export type AppDefinition = {
  readonly id: string;
  readonly label: string;
  readonly document: readonly string[];
  readonly iconId?: string;
  readonly controllerId: string;
  readonly modes: readonly { readonly id: string; readonly label: string; readonly tools?: readonly ToolNode[] }[];
  readonly defaultModeId?: string;
  readonly windowKinds: readonly {
    readonly id: string;
    readonly label: string;
    readonly bodyKey: string;
    readonly iconId?: string;
    readonly measures?: readonly WindowMeasure[];
    readonly engagement?: WindowEngagement;
  }[];
  readonly panelTabs: readonly AppPanelTabDefinition[];
  readonly keybindings: readonly { readonly keys: string; readonly action: ActionDescriptor }[];
  readonly actions?: readonly ActionDefinition[];
  readonly namedLayouts?: readonly NamedLayout[];
  readonly defaultLayout?: WindowLayout;
  readonly terminologies?: readonly string[];
};

export type PluginHotSwapEvent = {
  readonly pluginId: string;
  readonly version: string;
  readonly addedApps: readonly string[];
  readonly removedApps: readonly string[];
};
//#endregion AppManifestProtocol

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: PluginViewState) => Promise<ActionResponse>;
  readonly applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
  readonly render: (instanceId: number, bodyKey: string, viewState: PluginViewState) => Promise<PluginUiNode>;
  readonly renderWithDocument?: (instanceId: number, bodyKey: string, viewState: PluginViewState, documentJson: string) => Promise<PluginUiNode>;
  readonly tools: (instanceId: number, viewState: PluginViewState) => Promise<readonly Record<string, unknown>[]>;
  readonly windowEngagements: (instanceId: number, viewState: PluginViewState) => Promise<Readonly<Record<string, Record<string, unknown>>>>;
  readonly windowMeasures: (instanceId: number, viewState: PluginViewState) => Promise<Readonly<Record<string, readonly Record<string, unknown>[]>>>;
  readonly appLabels: (instanceId: number, viewState: PluginViewState) => Promise<PluginAppLabelsOverlay>;
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

//#region ActionResponse
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
  readonly actionId: string;
  readonly diff: DocumentDiff;
  readonly inverse: InverseOperation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** @emoji 🎁 The undo group binding an action invocation to its operations + inverses. */
export type UndoGroup = {
  readonly actionId: string;
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
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly accept: string; readonly readAs?: string; readonly importAction: string } }
  | { readonly spawnPluginInstance: { readonly programId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly documentJson?: string } }
  | { readonly openPluginInstance: { readonly programId: string; readonly appId: string; readonly osInstanceId?: string } };

/**
 * @emoji 📤 Typed result of a plugin `handle-action` call — mirrors the Rust `ActionResult`. Replaces
 * the legacy `string[]` JSON-patch shape: operations are now typed `KernelOperation`s with true
 * inverses, and the shell applies `requestedEffects` through `applyHostEffects` (WS-E).
 */
export type ActionResponse = {
  readonly output: unknown;
  readonly operations: readonly KernelOperation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly HostEffect[];
  readonly events?: readonly AppEvent[];
};

const EMPTY_ACTION_RESPONSE: ActionResponse = {
  output: null,
  operations: [],
  inverseGroup: { actionId: "", operations: [], inverseOperations: [] },
};

/** @emoji 📥 Parses a raw plugin `handle-action` response string into a typed {@link ActionResponse}. */
export function parseActionResponse(raw: string): ActionResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<ActionResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.operations)) {
      return parsed as ActionResponse;
    }
  } catch {
    // fall through to the empty response
  }
  return EMPTY_ACTION_RESPONSE;
}
//#endregion ActionResponse

export const DEFAULT_PLUGIN_REGISTRY: readonly PluginRegistryEntry[] = [{ pluginId: "draw", moduleUrl: "/plugin-modules/draw/draw_plugin.js" }];

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
    render: (instanceId, bodyKey, viewState) => runSerialized(() => handle.render(instanceId, bodyKey, viewState)),
    renderWithDocument: handle.renderWithDocument ? (instanceId, bodyKey, viewState, documentJson) => runSerialized(() => handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) : undefined,
    tools: (instanceId, viewState) => runSerialized(() => handle.tools(instanceId, viewState)),
    windowEngagements: (instanceId, viewState) => runSerialized(() => handle.windowEngagements(instanceId, viewState)),
    windowMeasures: (instanceId, viewState) => runSerialized(() => handle.windowMeasures(instanceId, viewState)),
    appLabels: (instanceId, viewState) => runSerialized(() => handle.appLabels(instanceId, viewState)),
    applyOperations: handle.applyOperations ? (instanceId, operationsJson) => runSerialized(() => handle.applyOperations!(instanceId, operationsJson)) : undefined,
    readAppDocument: handle.readAppDocument ? (instanceId) => runSerialized(() => handle.readAppDocument!(instanceId)) : undefined,
    loadAppDocument: handle.loadAppDocument ? (instanceId, documentJson) => runSerialized(() => handle.loadAppDocument!(instanceId, documentJson)) : undefined,
    attachBackbone: handle.attachBackbone ? (instanceId, uri) => runSerialized(() => handle.attachBackbone!(instanceId, uri)) : undefined,
    detachBackbone: handle.detachBackbone ? (instanceId) => runSerialized(() => handle.detachBackbone!(instanceId)) : undefined,
    dispose: handle.dispose,
  };
}
//#endregion SerializedPluginWasm

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
  const module = (await import(/* @vite-ignore */ moduleUrl)) as {
    default?: () => Promise<void> | void;
    createPluginApi?: () => Promise<{
      manifest: () => Promise<string>;
      createApp: (appId: string) => Promise<number>;
      destroyApp?: (instanceId: number) => Promise<void>;
      handleAction: (instanceId: number, actionJson: string, contextJson: string) => Promise<string>;
      render: (instanceId: number, bodyKey: string, viewStateJson: string) => Promise<string>;
      renderWithDocument?: (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => Promise<string>;
      tools?: (instanceId: number, viewStateJson: string) => Promise<string>;
      windowEngagements?: (instanceId: number, viewStateJson: string) => Promise<string>;
      windowMeasures?: (instanceId: number, viewStateJson: string) => Promise<string>;
      appLabels?: (instanceId: number, viewStateJson: string) => Promise<string>;
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
    semio_plugin_render?: (instanceId: number, bodyKey: string, viewStateJson: string) => string;
    semio_plugin_tools?: (instanceId: number, viewStateJson: string) => string;
    semio_plugin_window_engagements?: (instanceId: number, viewStateJson: string) => string;
    semio_plugin_window_measures?: (instanceId: number, viewStateJson: string) => string;
    semio_plugin_app_labels?: (instanceId: number, viewStateJson: string) => string;
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
        return parseActionResponse(raw);
      },
      render: async (instanceId, bodyKey, viewState) => JSON.parse(await api.render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode,
      renderWithDocument: api.renderWithDocument ? async (instanceId, bodyKey, viewState, documentJson) => JSON.parse(await api.renderWithDocument!(instanceId, bodyKey, JSON.stringify(viewState), documentJson)) as PluginUiNode : undefined,
      tools: async (instanceId, viewState) => {
        if (!api.tools) return [];
        return JSON.parse(await api.tools(instanceId, JSON.stringify(viewState))) as Record<string, unknown>[];
      },
      windowEngagements: async (instanceId, viewState) => {
        if (!api.windowEngagements) return {};
        return JSON.parse(await api.windowEngagements(instanceId, JSON.stringify(viewState))) as Record<string, Record<string, unknown>>;
      },
      windowMeasures: async (instanceId, viewState) => {
        if (!api.windowMeasures) return {};
        return JSON.parse(await api.windowMeasures(instanceId, JSON.stringify(viewState))) as Record<string, readonly Record<string, unknown>[]>;
      },
      appLabels: async (instanceId, viewState) => {
        if (!api.appLabels) return EMPTY_APP_LABELS_OVERLAY;
        return normalizeAppLabelsOverlay(JSON.parse(await api.appLabels(instanceId, JSON.stringify(viewState))));
      },
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
      if (!handle) return EMPTY_ACTION_RESPONSE;
      const raw = handle(instanceId, actionJson, JSON.stringify(viewState));
      return parseActionResponse(raw);
    },
    async render(instanceId: number, bodyKey: string, viewState: PluginViewState) {
      const render = module.semio_plugin_render;
      if (!render) throw new Error(`plugin ${pluginId} missing render`);
      return JSON.parse(render(instanceId, bodyKey, JSON.stringify(viewState))) as PluginUiNode;
    },
    async tools(instanceId: number, viewState: PluginViewState) {
      const tools = module.semio_plugin_tools;
      if (!tools) return [];
      return JSON.parse(tools(instanceId, JSON.stringify(viewState))) as Record<string, unknown>[];
    },
    async windowEngagements(instanceId: number, viewState: PluginViewState) {
      const engagements = module.semio_plugin_window_engagements;
      if (!engagements) return {};
      return JSON.parse(engagements(instanceId, JSON.stringify(viewState))) as Record<string, Record<string, unknown>>;
    },
    async windowMeasures(instanceId: number, viewState: PluginViewState) {
      const measures = module.semio_plugin_window_measures;
      if (!measures) return {};
      return JSON.parse(measures(instanceId, JSON.stringify(viewState))) as Record<string, readonly Record<string, unknown>[]>;
    },
    async appLabels(instanceId: number, viewState: PluginViewState) {
      const labels = module.semio_plugin_app_labels;
      if (!labels) return EMPTY_APP_LABELS_OVERLAY;
      return normalizeAppLabelsOverlay(JSON.parse(labels(instanceId, JSON.stringify(viewState))));
    },
    applyOperations: module.semio_plugin_apply_operations ? async (instanceId, operationsJson) => { module.semio_plugin_apply_operations!(instanceId, operationsJson); } : undefined,
    readAppDocument: module.semio_plugin_read_app_document ? async (instanceId) => module.semio_plugin_read_app_document!(instanceId) : undefined,
    loadAppDocument: module.semio_plugin_load_app_document ? async (instanceId, documentJson) => { module.semio_plugin_load_app_document!(instanceId, documentJson); } : undefined,
    attachBackbone: module.semio_plugin_attach_backbone ? async (instanceId, uri) => { module.semio_plugin_attach_backbone!(instanceId, uri); } : undefined,
    detachBackbone: module.semio_plugin_detach_backbone ? async (instanceId) => { module.semio_plugin_detach_backbone!(instanceId); } : undefined,
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
    render: (instanceId: number, bodyKey: string, viewStateJson: string) => handle.render(instanceId, bodyKey, JSON.parse(viewStateJson) as PluginViewState).then((node) => JSON.stringify(node)),
    renderWithDocument: handle.renderWithDocument
      ? (instanceId: number, bodyKey: string, viewStateJson: string, documentJson: string) => handle.renderWithDocument!(instanceId, bodyKey, JSON.parse(viewStateJson) as PluginViewState, documentJson).then((node) => JSON.stringify(node))
      : undefined,
    tools: (instanceId: number, viewStateJson: string) => handle.tools(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((nodes) => JSON.stringify(nodes)),
    windowEngagements: (instanceId: number, viewStateJson: string) => handle.windowEngagements(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((engagements) => JSON.stringify(engagements)),
    windowMeasures: (instanceId: number, viewStateJson: string) => handle.windowMeasures(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((measures) => JSON.stringify(measures)),
    appLabels: (instanceId: number, viewStateJson: string) => handle.appLabels(instanceId, JSON.parse(viewStateJson) as PluginViewState).then((overlay) => JSON.stringify(overlay)),
  };
}
//#endregion PluginRuntime
