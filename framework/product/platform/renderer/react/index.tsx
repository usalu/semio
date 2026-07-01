// #region 🧲Header
/** @emoji ⚛️ `@semio-tech/framework-platform-renderer-react` — React renderer for {@link @semio-tech/framework-platform-core}: {@link ProductShell}, {@link PlatformView}, declarative {@link UiNode} host (monolith). */
// #endregion 🧲Header

export {
	Platform,
	Store,
	Table,
	VirtualFileSystem,
	buildVirtualFileSystemWindowBody,
	registerPlatformVirtualFileSystemDemo,
	PlatformVirtualFileSystemDemoController,
	PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
	virtualFileSystemSurfaceId,
	Puzzle2d,
	Puzzle3d,
	Puzzle5d,
	Cad,
	Panel,
	registerPlatformComponent,
	type WindowLayout,
	type ComponentKind,
	type TableModel,
	type VirtualFileSystemModel,
	type Puzzle2dModel,
	type Puzzle3dModel,
	type Puzzle5dModel,
	type CadModel,
	type PanelModel,
	type PlatformTopologyPayload,
	PlatformTopologyStore,
	getPlatformControllerById,
	platformTopologyStoreId,
	registerSidePanelBody,
	unregisterSidePanelBody,
} from "@semio-tech/framework-platform-core";

export type { Level } from "@semio-tech/ui-react";
export {
	LevelProvider,
	useLevel,
	getLevelBgClass,
	getLevelHoverClass,
	getLevelActiveHoverClass,
	getLevelZClass,
	getLevelBorderElementClass,
	getLevelDivideElementClass,
} from "@semio-tech/ui-react";

// #region 🔌Adapters
import {
	mergeAppTools,
	toolCollection,
	CommandBus,
	Controller,
	Store,
	Platform,
	PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
	resolveInitialPanelVisibility,
	LEFT_PANEL_KINDS,
	RIGHT_PANEL_KINDS,
	PANEL_KINDS,
	panelSide,
	type PanelKind,
	type NavigationLevel,
	AppRuntime,
	ModeRuntime,
	resolveCommandPaletteItems,
	WindowKindRuntime,
	createTabStackLayout,
	createWindowLayout,
	getSidePanelBodyFactory,
	getWindowBodyFactory,
	isEdgelessWindowBody,
	registerSidePanelBody,
	unregisterSidePanelBody,
	type ResolvedAppState,
	type AppTools as FrameworkAppTools,
	type FooterItem as DeclarativeFooterItem,
	type SidePanelBodyViewContext,
	type SideTabSpec,
	type ToolNode,
	type WindowBodyViewContext,
	type WindowMeasure,
	type NamedLayout,
	type StoragePort,
	NamedLayoutStore,
	createNamedLayout,
	type WindowTemplate,
	Cad,
	Panel,
	Puzzle2d,
	Puzzle3d,
	Puzzle5d,
	Table,
	VirtualFileSystem as VirtualFileSystemSurface,
	type CadModel,
	type Component,
	type ComponentKind,
	type VirtualFileSystemModel,
	type VirtualFileSystemSchemaModel,
	type PanelModel,
	type Puzzle2dModel,
	type Puzzle3dModel,
	type Puzzle5dModel,
	type TableModel,
	type UiButtonNode,
	type UiCadHostSurfaceNode,
	type UiComponentHostSurfaceNode,
	type UiNode,
	type UiPanelHostSurfaceNode,
	type UiPuzzle2dHostSurfaceNode,
	type UiPuzzle3dHostSurfaceNode,
	type UiPuzzle5dHostSurfaceNode,
	type UiSeparatorNode,
	type UiStackNode,
	type UiTableHostSurfaceNode,
	type UiEditorHostSurfaceNode,
	type UiVirtualFileSystemHostSurfaceNode,
	type UiTextNode,
	type UiTreeNode,
	type UiTreeSectionNode,
	type UiTreeItemNode,
	type UiControlNode,
	type UiInputNode,
	type UiSelectNode,
	type UiToggleNode,
	type UiVec3Node,
	type UiKeyValueNode,
	type UiSliderNode,
	type UiNumberStepperNode,
	type UiRingNode,
	type UiIconSelectNode,
	type UiFieldNode,
	type CommandDescriptor,
	collectUiTreeItemDragData,
	getPlatformControllerById,
	platformTopologyStoreId,
	registerPlatformVirtualFileSystemDemo,
	PlatformVirtualFileSystemDemoController,
	virtualFileSystemSurfaceId,
	PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
} from "@semio-tech/framework-platform-core";
import * as React from "react";
import { createRoot, type Root } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import Fuse, { type FuseResult } from "fuse.js";
import { Puzzle2dCanvas, parsePuzzle2dFixtureV1, type Puzzle2dPreselectSnapshot, type Puzzle2dSelectionSnapshot } from "@semio-tech/puzzle-2d-react";
import { parseFixtureV1, puzzle3dFixturePaletteTreeDragController, type SelectionSnapshot as Puzzle3dSelectionSnapshot } from "@semio-tech/puzzle-3d-react";
import { PUZZLE_2D_FIXTURE_DRAG_V1_MIME, puzzle2dFixturePaletteTreeDragController, classifyPuzzle2dIconSelectorMode } from "@semio-tech/puzzle-2d-react";
import { FiveD, StoreProvider, compose5d, createStore, prepareTopologyModel } from "@semio-tech/puzzle-5d-react";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import {
	BasicChatPanel,
	Breadcrumb,
	type BreadcrumbItemData,
  Button,
  ButtonCycle,
  ButtonGroup,
  ButtonGroupItem,
  CommandDialog,
	CommandEmpty,
	CommandGroup,
	CommandInput,
	CommandItem,
	CommandList,
	Combobox,
	ContextMenu,
	Footer,
	Input,
	Layout,
	Navbar,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Slider,
	Ring,
	IconSelector,
	staticSidePanelTabDefinition,
	staticTreePanelDefinition,
	Stepper,
	Textarea,
	Toggle,
	ToggleGroup,
	ActionGroup,
	ActionGroupItem,
	ToolbarDivider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	Window,
	App,
	Mode,
	collapseLayout,
	insertWindowAtDropZone,
	removeWindowFromLayout,
	COMPOSE_WINDOW_TEMPLATE_MIME,
	windowTemplatePaletteTreeDragController,
	type TreeDragAndDropController,
	Ui,
	type EngagementSpec,
	type ModeCanvasDropTarget,
	type ModeWindowDescriptor,
	type WindowLayoutNode as ShellWindowLayoutNode,
	type WindowTemplateDropPayload,
	cn,
	resolveTranslationLabel,
	useUiTranslation,
	useCommandHotkey,
	useMediaQuery,
	useSidePanelChromeHotkeys,
	type ContextMenuItem,
	type NavbarItem,
	Expertise,
	LevelProvider,
	getLevelBgClass,
	readStoredUiChromeCompact,
	readStoredUiChromeExpertise,
	readStoredUiChromeTheme,
	readStoredComputeWorkerCount,
	writeStoredComputeWorkerCount,
	isCrossOriginIsolatedRuntime,
	defaultComputeWorkerCount,
	UiChromeCompactProvider,
	UiChromeLabelPolicyProvider,
	useElementsSurfaceChrome,
	writeStoredUiChromeCompact,
	writeStoredUiChromeExpertise,
	writeStoredUiChromeTheme,
	type ElementsSurfaceTheme,
	reactHostPort,
	VirtualFileSystem as VirtualFileSystemView,
	type VirtualFileSystemRow,
	type VirtualFileSystemSchema,
	type SidePanelTabConfig,
	type TreeDataItem,
	type TreeDataSection,
	type TreePanelConfig,
	type TreePanelDefinition,
	type TreePanelSource,
  windowMeasureControlClass,
  windowMeasureLabelClass,
  windowMeasureTileClass,
  windowMeasureToggleClass,
  windowMeasureToggleCompactClass,
	borderNormalClass,
	navbarFillClassName,
	PanelToggleGroup,
	type PanelToggleItem,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
  WindowMeasuresTree,
	type AssertUiToolbarParentKeysCovered,
	Icon,
	renderControlIcon,
	type IconName,
	type IconSource,
	SemioLogo,
	interactiveActiveFillClass,
	shellChromeTitleClassName,
} from "@semio-tech/ui-react";
// #endregion 🔌Adapters

import { ICONS } from "@semio-tech/ui-asset";

//#region 📦shell-chrome-types.tsx

/** @emoji 👣 Footer row rendered by the product shell. */
export interface ChromeFooterRow {
	readonly id: string;
	readonly icon: React.ReactNode;
	readonly text?: string;
	readonly order?: number;
	readonly onClick?: () => void;
	readonly className?: string;
	readonly disabled?: boolean;
}

export type { SidePanelTabConfig, TreePanelConfig, TreePanelDefinition, TreePanelSource } from "@semio-tech/ui-react";

//#endregion 📦shell-chrome-types.tsx

//#region 📦shell-canvas.tsx
/** @emoji 🖼 Golden-layout shell: layouts, canvas portals, toolbar, search, and find. */
type DOMListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

function createDOMEventBinding() {
	const cleanups: Array<() => void> = [];
	return {
		listen(target: DOMListenerTarget | null | undefined, type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions) {
			if (!target) return;
			target.addEventListener(type, listener, options);
			cleanups.push(() => target.removeEventListener(type, listener, options));
		},
		dispose() {
			while (cleanups.length > 0) cleanups.pop()?.();
		},
	};
}



/**
 * A window control with kind, ID, icon, options, and change handler.
 **/
export interface UIWindowControl {
  kind: "toggle" | "dropdown";
  id: string;
  label?: string;
  text?: string;
  icon?: React.ReactNode;
  value?: string;
  options?: {
    id: string;
    value: string;
    icon?: React.ReactNode;
  }[];
  onChange?: (value: string) => void;
}

/**
 * 📐 Declarative `measure` entries for a window: read-only readouts (`display`, `reading`) or interactive controls; rendered as compact floats on the right.
 **/
export type UIWindowMeasure =
  | { kind: "display"; id: string; label?: string; content: React.ReactNode }
  | { kind: "reading"; id: string; label?: string; text: string; monospace?: boolean }
  | { kind: "section"; id: string; title: string }
  | { kind: "separator"; id: string }
  | { kind: "group"; id: string; label: string; defaultOpen?: boolean; children: UIWindowMeasure[] }
  | { kind: "toggle"; id: string; label?: string; pressed?: boolean; defaultPressed?: boolean; icon: React.ReactNode; text?: string; onPressedChange?: (pressed: boolean) => void }
  | { kind: "select"; id: string; label?: string; value?: string; defaultValue?: string; items: { id: string; value: string; label: string }[]; onValueChange?: (value: string) => void }
  | { kind: "combobox"; id: string; label?: string; value?: string; placeholder?: string; choices: { value: string; label: string }[]; onValueChange?: (value: string) => void }
  | { kind: "button"; id: string; label?: string; text: string; icon: React.ReactNode; onClick?: () => void }
  | { kind: "buttonCycle"; id: string; label?: string; value?: string; items: { value: string; label: string; icon: React.ReactNode; text?: string; id?: string }[]; onValueChange?: (value: string) => void }
  | { kind: "input"; id: string; label?: string; value?: string; placeholder?: string; onLazyChange?: (value: string) => void }
  | { kind: "textarea"; id: string; label?: string; value?: string; placeholder?: string; rows?: number; onLazyChange?: (value: string) => void }
  | { kind: "checkbox"; id: string; label?: string; checked?: boolean; defaultChecked?: boolean; onCheckedChange?: (checked: boolean) => void }
  | { kind: "radio"; id: string; label?: string; value: string; items: { value: string; label: string }[]; onChange?: (value: string) => void }
  | { kind: "slider"; id: string; label?: string; value?: number; min?: number; max?: number; step?: number; onValueChange?: (value: number) => void }
  | { kind: "number"; id: string; label?: string; value?: number; min?: number; max?: number; step?: number; onChange?: (value: number) => void }
  | { kind: "color"; id: string; label?: string; value?: string; onChange?: (value: string) => void };

/**
 * Definition of a window kind with label, icon, component, controls, and optional floating window measures.
 * Each app registers the window kinds it can render.
 **/
export interface UIWindowKindDefinition {
  id: string;
  windowKindId?: string;
  templateId?: string;
  label?: string;
  icon?: React.ReactNode;
  component: React.ComponentType<any>;
  controls?: UIWindowControl[];
  measures?: UIWindowMeasure[];
  engagement?: EngagementSpec;
  contextMenu?: ContextMenuItem[];
  variants?: {
    id: string;
    icon?: React.ReactNode;
    componentProps?: Record<string, any>;
  }[];
}

/**
 * A single window entry in the abstract UI layout tree.
 **/
export interface WindowLayoutWindowNode {
  kind: "window";
  windowKindId: string;
  title?: string;
  instanceId?: string;
  templateId?: string;
}

/**
 * A tab stack in the abstract UI layout tree.
 **/
export interface WindowLayoutStackNode {
  kind: "stack";
  size?: number;
  children: WindowLayoutWindowNode[];
}

/**
 * A row or column branch in the abstract UI layout tree.
 **/
export interface WindowLayoutAxisNode {
  kind: "row" | "column";
  size?: number;
  children: Array<WindowLayoutAxisNode | WindowLayoutStackNode>;
}

/**
 * Root layout wrapper owned by an app instead of the Golden Layout platform.
 **/
export interface WindowLayout {
  root: WindowLayoutAxisNode | WindowLayoutStackNode;
}

/**
 * Union of supported abstract UI layout nodes.
 **/
export type WindowLayoutNode = WindowLayout["root"];

function isWindowLayoutWindowNode(value: unknown): value is WindowLayoutWindowNode {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<WindowLayoutWindowNode>;
  return candidate.kind === "window" && typeof candidate.windowKindId === "string";
}

function isWindowLayoutStackNode(value: unknown): value is WindowLayoutStackNode {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<WindowLayoutStackNode>;
  return candidate.kind === "stack" && Array.isArray(candidate.children) && candidate.children.every(isWindowLayoutWindowNode);
}

function isWindowLayoutAxisNode(value: unknown): value is WindowLayoutAxisNode {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<WindowLayoutAxisNode>;
  return (candidate.kind === "row" || candidate.kind === "column") && Array.isArray(candidate.children) && candidate.children.every((child) => isWindowLayoutAxisNode(child) || isWindowLayoutStackNode(child));
}

function isWindowLayout(value: unknown): value is WindowLayout {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<WindowLayout>;
  return isWindowLayoutAxisNode(candidate.root) || isWindowLayoutStackNode(candidate.root);
}

function convertLegacyGoldenNodeToWindowLayoutNode(value: unknown): WindowLayoutNode | WindowLayoutWindowNode | undefined {
  if (!value || typeof value !== "object") return undefined;
  const node = value as Record<string, unknown>;

  if (node.type === "component") {
    const componentName = typeof node.componentName === "string" ? node.componentName : undefined;
    if (!componentName) return undefined;
    return createWindowLayout(componentName, typeof node.title === "string" ? node.title : componentName);
  }

  if (node.type === "stack") {
    const children = Array.isArray(node.content) ? node.content.map(convertLegacyGoldenNodeToWindowLayoutNode).filter(isWindowLayoutWindowNode) : [];
    if (children.length === 0) return undefined;
    return {
      kind: "stack",
      ...(typeof node.size === "string" ? { size: Number.parseFloat(node.size) } : typeof node.size === "number" ? { size: node.size } : {}),
      children,
    };
  }

  if (node.type === "row" || node.type === "column") {
    const children = Array.isArray(node.content)
      ? node.content.map(convertLegacyGoldenNodeToWindowLayoutNode).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => isWindowLayoutAxisNode(child) || isWindowLayoutStackNode(child))
      : [];
    if (children.length === 0) return undefined;
    return {
      kind: node.type,
      ...(typeof node.size === "string" ? { size: Number.parseFloat(node.size) } : typeof node.size === "number" ? { size: node.size } : {}),
      children,
    };
  }

  return undefined;
}

/**
 * Parses a window layout from a string, object, or undefined input.
 * MUST return undefined for null, empty, or unparseable inputs.
 **/
export function parseWindowLayout(layout: unknown): WindowLayout | undefined {
  if (layout === undefined || layout === null) return undefined;
  if (typeof layout === "string") {
    const trimmed = layout.trim();
    if (!trimmed) return undefined;
    try {
      return parseWindowLayout(JSON.parse(trimmed));
    } catch {
      return undefined;
    }
  }
  if (isWindowLayout(layout)) return layout;
  if (typeof layout === "object") {
    const candidate = layout as Record<string, unknown>;
    const legacyRoot = convertLegacyGoldenNodeToWindowLayoutNode(candidate.root);
    if (legacyRoot && (isWindowLayoutAxisNode(legacyRoot) || isWindowLayoutStackNode(legacyRoot))) {
      return { root: legacyRoot };
    }
  }
  return undefined;
}

/**
 * Serializes a window layout to a JSON string.
 * MUST return undefined when serialization fails.
 **/
export function stringifyWindowLayout(layout: unknown): string | undefined {
  const parsedLayout = parseWindowLayout(layout);
  if (!parsedLayout) return undefined;
  try {
    return JSON.stringify(parsedLayout);
  } catch {
    return undefined;
  }
}

/**
 * Removes duplicate and disallowed window components from a layout.
 **/
export function deduplicateWindowLayout(layout: unknown, allowedWindowIds: string[]): WindowLayout | undefined {
  const parsedLayout = parseWindowLayout(layout);
  if (!parsedLayout) return undefined;

  const seenComponents = new Set<string>();

  const deduplicateNode = (node: WindowLayoutNode): WindowLayoutNode | undefined => {
    if (node.kind === "stack") {
      const children = node.children.filter((child) => {
        if (seenComponents.has(child.windowKindId) || !allowedWindowIds.includes(child.windowKindId)) return false;
        seenComponents.add(child.windowKindId);
        return true;
      });

      if (children.length === 0) return undefined;
      return { ...node, children };
    }

    const children = node.children.map((child) => deduplicateNode(child)).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => Boolean(child));

    if (children.length === 0) return undefined;
    return { ...node, children };
  };

  const deduplicatedRoot = deduplicateNode(parsedLayout.root);
  if (!deduplicatedRoot || isWindowLayoutWindowNode(deduplicatedRoot)) return undefined;
  return { root: deduplicatedRoot };
}

function convertWindowLayoutNodeToGoldenConfig(node: WindowLayoutNode): Record<string, unknown> {
  if (node.kind === "stack") {
    return {
      type: "stack",
      ...(node.size !== undefined ? { size: `${node.size}%` } : {}),
      content: node.children.map((child) => ({
        type: "component",
        componentName: child.windowKindId,
        title: child.title ?? child.windowKindId,
        componentState: {},
      })),
    };
  }

  return {
    type: node.kind,
    ...(node.size !== undefined ? { size: `${node.size}%` } : {}),
    content: node.children.map((child) => convertWindowLayoutNodeToGoldenConfig(child)),
  };
}

function convertWindowLayoutToGoldenConfig(layout: WindowLayout): Record<string, unknown> {
  return { root: convertWindowLayoutNodeToGoldenConfig(layout.root) };
}

/** @emoji 🧩 Converts {@link WindowLayout} to legacy Golden Layout JSON (interop). */
export function layoutNodeToGoldenLayoutConfig(layout: WindowLayout): Record<string, unknown> {
  return convertWindowLayoutToGoldenConfig(layout);
}

/**
 * Window controls group component rendering toggle and dropdown controls.
 **/
const UIWindowControlsGroup: React.FC<{ controls: UIWindowControl[] }> = ({ controls }) => (
  <ActionGroup id="window-controls-group">
    {controls.map((control) => {
      if (control.kind === "toggle") {
        return (
          <ActionGroupItem
            key={control.id}
            id={control.id}
            icon={control.icon}
            text={control.text ?? control.label}
            onClick={() => control.onChange?.(control.value === "on" ? "off" : "on")}
          />
        );
      }
      return <ActionGroupItem key={control.id} id={control.id} icon={control.icon} text={control.text ?? control.label} />;
    })}
  </ActionGroup>
);

// #region 🪟WindowMeasuresOverlay

const UIWindowMeasureFloat: React.FC<{ measureId: string; label?: string; children: React.ReactNode }> = ({ measureId, label, children }) => (
  <div data-slot="window-measure-float" data-measure-id={measureId} className={windowMeasureTileClass}>
    {label ? <span className={windowMeasureLabelClass}>{label}</span> : null}
    <div className={windowMeasureControlClass}>{children}</div>
  </div>
);

function renderUIWindowMeasure(measure: UIWindowMeasure): React.ReactNode {
  switch (measure.kind) {
    case "group":
      return (
        <WindowMeasureTreeGroup id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen ?? true}>
          {measure.children.map((child) => (
            <React.Fragment key={child.id}>{renderUIWindowMeasure(child)}</React.Fragment>
          ))}
        </WindowMeasureTreeGroup>
      );
    case "display":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label} fullWidth>
          <div className="text-element max-w-full text-xs leading-snug break-words">{measure.content}</div>
        </WindowMeasureTreeLeaf>
      );
    case "reading":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label} fullWidth>
          <div className={cn("text-element text-xs tabular-nums", measure.monospace && "font-mono")}>{measure.text}</div>
        </WindowMeasureTreeLeaf>
      );
    case "section":
      return (
        <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.title} defaultOpen>
          {null}
        </WindowMeasureTreeGroup>
      );
    case "separator":
      return <div key={measure.id} data-slot="window-measure-separator" className="bg-muted-foreground/35 my-tiny h-px w-8 shrink-0 rounded-full" aria-hidden />;
    case "toggle":
      return (
        <WindowMeasureTreeLeaf key={measure.id} fullWidth>
          <Toggle
            id={measure.id}
            className={cn(windowMeasureToggleClass, windowMeasureToggleCompactClass)}
            pressed={measure.pressed}
            defaultPressed={measure.defaultPressed}
            onPressedChange={measure.onPressedChange}
            icon={measure.icon}
            text={measure.text}
          />
        </WindowMeasureTreeLeaf>
      );
    case "select":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <Select id={measure.id} value={measure.value} defaultValue={measure.defaultValue} onValueChange={measure.onValueChange}>
            <SelectTrigger id={measure.id} className="h-small w-full min-w-0 py-tiny" size="sm">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {measure.items.map((item) => (
                <SelectItem key={item.id} value={item.value}>
                  {item.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </WindowMeasureTreeLeaf>
      );
    case "combobox":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <Combobox id={measure.id} value={measure.value} options={measure.choices} placeholder={measure.placeholder} onValueChange={measure.onValueChange} className={windowMeasureControlClass} />
        </WindowMeasureTreeLeaf>
      );
    case "button":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label} fullWidth>
          <Button id={measure.id} text={measure.text} icon={measure.icon} onClick={measure.onClick} />
        </WindowMeasureTreeLeaf>
      );
    case "buttonCycle":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <ButtonCycle id={measure.id} value={measure.value} onValueChange={measure.onValueChange} items={measure.items} />
        </WindowMeasureTreeLeaf>
      );
    case "input":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <Input id={measure.id} lazy className={cn("h-small", windowMeasureControlClass)} value={measure.value} placeholder={measure.placeholder} onLazyChange={measure.onLazyChange} />
        </WindowMeasureTreeLeaf>
      );
    case "textarea":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label} fullWidth>
          <Textarea id={measure.id} lazy className={cn("min-h-[3rem]", windowMeasureControlClass)} value={measure.value} placeholder={measure.placeholder} rows={measure.rows} onLazyChange={measure.onLazyChange} />
        </WindowMeasureTreeLeaf>
      );
    case "checkbox":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <div className="text-element flex w-full min-w-0 items-center justify-end gap-single text-xs">
            <input
              id={measure.id}
              type="checkbox"
              className={cn("accent-foreground size-small shrink-0 rounded border", borderNormalClass)}
              {...(measure.checked !== undefined ? { checked: measure.checked } : { defaultChecked: measure.defaultChecked })}
              onChange={(event) => measure.onCheckedChange?.(event.target.checked)}
            />
          </div>
        </WindowMeasureTreeLeaf>
      );
    case "radio":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label} fullWidth>
          <div className="flex flex-col gap-tiny" role="radiogroup" aria-labelledby={measure.id}>
            {measure.items.map((item) => (
              <button
                key={item.value}
                type="button"
                data-slot="window-measure-radio-item"
                className={cn(
                  "border-normal/80 hover:bg-hover-interactive-fill w-full rounded border px-tiny py-tiny text-left text-xs transition-colors",
                  measure.value === item.value && "bg-active-base text-active-foreground",
                )}
                onClick={() => measure.onChange?.(item.value)}
              >
                {item.label}
              </button>
            ))}
          </div>
        </WindowMeasureTreeLeaf>
      );
    case "slider": {
      const min = measure.min ?? 0;
      const max = measure.max ?? 100;
      const v = measure.value ?? min;
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <Slider id={measure.id} value={[v]} min={min} max={max} step={measure.step} onValueChange={(vals) => measure.onValueChange?.(vals[0] ?? min)} />
        </WindowMeasureTreeLeaf>
      );
    }
    case "number":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <Stepper id={measure.id} value={measure.value} min={measure.min} max={measure.max} step={measure.step} onChange={measure.onChange} />
        </WindowMeasureTreeLeaf>
      );
    case "color":
      return (
        <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
          <Input id={measure.id} type="color" className={cn("h-small cursor-pointer", windowMeasureControlClass)} value={measure.value} onChange={(event) => measure.onChange?.(event.target.value)} />
        </WindowMeasureTreeLeaf>
      );
    default: {
      const _exhaustive: never = measure;
      return _exhaustive;
    }
  }
}

/**
 * 📐 Maps declarative `UIWindowMeasure` entries into compact floating tiles aligned to the right edge.
 **/
export const UIWindowMeasures: React.FC<{ measures: UIWindowMeasure[] }> = ({ measures }) => (
  <WindowMeasuresTree>
    {measures.map((measure) => (
      <React.Fragment key={measure.id}>{renderUIWindowMeasure(measure)}</React.Fragment>
    ))}
  </WindowMeasuresTree>
);

// #endregion 🪟WindowMeasuresOverlay

function convertFrameworkLayoutNodeToShellLayout(node: WindowLayoutNode): ShellWindowLayoutNode {
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => ({
        kind: "window",
        id: child.instanceId ?? child.windowKindId,
        title: child.title,
      })),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToShellLayout(child)),
  };
}

function collectFrameworkWindowNodes(node: WindowLayoutNode): WindowLayoutWindowNode[] {
  if (node.kind === "window") return [node];
  if (node.kind === "stack") return [...node.children];
  return node.children.flatMap((child) => collectFrameworkWindowNodes(child));
}

interface ShellWindowInstance {
  readonly instanceId: string;
  readonly windowKindId: string;
  readonly templateId?: string;
  readonly title: string;
}

function findWindowTemplateInList(templates: readonly WindowTemplate[], templateId: string): WindowTemplate | undefined {
	for (const template of templates) {
		if (template.id === templateId) {
			return template;
		}
		if (template.children?.length) {
			const nested = findWindowTemplateInList(template.children, templateId);
			if (nested) {
				return nested;
			}
		}
	}
	return undefined;
}

function findWindowTemplate(catalog: readonly WindowKindRuntime[], windowKindId: string, templateId: string): WindowTemplate | undefined {
	const kind = catalog.find((entry) => entry.id === windowKindId);
	if (!kind) {
		return undefined;
	}
	return findWindowTemplateInList(kind.templates, templateId);
}

function mapWindowTemplatesToTreeItems(windowKindId: string, templates: readonly WindowTemplate[]): import("@semio-tech/ui-react").TreeDataItem[] {
	return templates.map((template) => ({
		id: `framework.display.windows.${windowKindId}.${template.id}`,
		label: template.label,
		draggable: true,
		dragData: {
			[COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId, templateId: template.id } satisfies WindowTemplateDropPayload),
		},
		...(template.children?.length ? { defaultOpen: false, items: mapWindowTemplatesToTreeItems(windowKindId, template.children) } : {}),
	}));
}

function groupNamedLayoutsToTreeItems(
	layouts: readonly NamedLayout[],
	onApply: (layoutId: string) => void,
	onDeleteUser?: (layoutId: string) => void,
): import("@semio-tech/ui-react").TreeDataItem[] {
	const root: import("@semio-tech/ui-react").TreeDataItem[] = [];
	const folderByKey = new Map<string, import("@semio-tech/ui-react").TreeDataItem>();

	const layoutLeaf = (entry: NamedLayout): import("@semio-tech/ui-react").TreeDataItem => ({
		id: `framework.display.layout.${entry.id}`,
		label: entry.label,
		description: entry.origin === "user" ? resolveTranslationLabel("ui.display.deleteLayout") : undefined,
		onClick: () => onApply(entry.id),
		...(entry.origin === "user" && onDeleteUser
			? {
					actions: [
						{
							id: `framework.display.delete.${entry.id}`,
							icon: <Icon icon="trash-2" size="small" />,
							onClick: () => onDeleteUser(entry.id),
						},
					],
				}
			: {}),
	});

	for (const entry of layouts) {
		if (!entry.groupPath?.length) {
			root.push(layoutLeaf(entry));
			continue;
		}
		let siblings = root;
		let pathKey = "";
		for (let index = 0; index < entry.groupPath.length; index += 1) {
			const segment = entry.groupPath[index]!;
			pathKey = pathKey ? `${pathKey}/${segment}` : segment;
			let folder = folderByKey.get(pathKey);
			if (!folder) {
				folder = {
					id: `framework.display.layout.group.${pathKey}`,
					label: segment,
					defaultOpen: false,
					items: [],
				};
				folderByKey.set(pathKey, folder);
				siblings.push(folder);
			}
			const folderItems = folder.items ?? (folder.items = []);
			if (index === entry.groupPath.length - 1) {
				folder.items = [...folderItems, layoutLeaf(entry)];
			} else {
				siblings = folderItems;
			}
		}
	}
	return root;
}

function dispatchWindowTemplate(
  bus: CommandBus,
  catalog: readonly WindowKindRuntime[],
  windowKindId: string,
  templateId?: string,
  instanceId?: string,
): void {
  if (!templateId) return;
  const template = findWindowTemplate(catalog, windowKindId, templateId);
  if (!template?.controllerId || !template.command) return;
  const args =
    instanceId && template.args && typeof template.args === "object"
      ? { ...(template.args as Record<string, unknown>), instanceId }
      : instanceId
        ? { instanceId }
        : template.args;
  bus.dispatch(template.controllerId, template.command, args);
}

//#region 🪟ShellWindowInstance
export interface ShellWindowInstanceContextValue {
  readonly instanceId: string;
  readonly windowKindId: string;
  readonly templateId?: string;
}

const ShellWindowInstanceContext = reactHostPort.createContext<ShellWindowInstanceContextValue | null>(null);

/** @emoji 🪟 Active resizable shell window instance (camera, topology, template scope). */
export function useShellWindowInstance(): ShellWindowInstanceContextValue | null {
  return reactHostPort.useContext(ShellWindowInstanceContext);
}

/** @emoji 🪟 Stable scope key for per-instance viewport state (defaults to window kind id on bootstrap). */
export function shellWindowScopeId(instance: ShellWindowInstanceContextValue | null, fallbackWindowKindId: string): string {
  return instance?.instanceId ?? fallbackWindowKindId;
}

function ShellWindowInstanceProvider(props: { readonly value: ShellWindowInstanceContextValue; readonly children: React.ReactNode }): React.ReactElement {
  return <ShellWindowInstanceContext.Provider value={props.value}>{props.children}</ShellWindowInstanceContext.Provider>;
}
//#endregion 🪟ShellWindowInstance

function createShellInstanceId(windowKindId: string, index: number): string {
  const suffix = typeof crypto !== "undefined" && "randomUUID" in crypto ? crypto.randomUUID() : `${Date.now()}-${index}`;
  return `win-${windowKindId}-${suffix}`;
}

function bootstrapShellInstances(layout: WindowLayout, catalog: readonly WindowKindRuntime[], bus: CommandBus): ShellWindowInstance[] {
  return collectFrameworkWindowNodes(layout.root).map((node) => {
    const kind = catalog.find((entry) => entry.id === node.windowKindId);
    const template = node.templateId ? findWindowTemplate(catalog, node.windowKindId, node.templateId) : undefined;
    const instanceId = node.instanceId ?? node.windowKindId;
    dispatchWindowTemplate(bus, catalog, node.windowKindId, node.templateId, instanceId);
    return {
      instanceId,
      windowKindId: node.windowKindId,
      templateId: node.templateId,
      title: node.title ?? template?.label ?? kind?.label ?? node.windowKindId,
    };
  });
}

function shellLayoutNodeToFramework(node: ShellWindowLayoutNode, instancesById: ReadonlyMap<string, ShellWindowInstance>): WindowLayoutNode {
  if (node.kind === "window") {
    const instance = instancesById.get(node.id);
    if (!instance) return { kind: "window", windowKindId: node.id, title: node.title };
    return {
      kind: "window",
      windowKindId: instance.windowKindId,
      title: node.title ?? instance.title,
      instanceId: instance.instanceId,
      ...(instance.templateId ? { templateId: instance.templateId } : {}),
    };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => shellLayoutNodeToFramework(child, instancesById) as WindowLayoutWindowNode),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => shellLayoutNodeToFramework(child, instancesById) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

function shellLayoutToFrameworkLayout(shell: ShellWindowLayoutNode, instances: readonly ShellWindowInstance[]): WindowLayout {
  const instancesById = new Map(instances.map((instance) => [instance.instanceId, instance]));
  const root = shellLayoutNodeToFramework(shell, instancesById);
  if (root.kind === "window") {
    return { root: { kind: "stack", children: [root] } };
  }
  return { root };
}

function instantiateFrameworkLayout(
  layout: WindowLayout,
  catalog: readonly WindowKindRuntime[],
  bus: CommandBus,
): { instances: ShellWindowInstance[]; shellLayout: ShellWindowLayoutNode } {
  const instances: ShellWindowInstance[] = [];
  let counter = 0;
  const convert = (node: WindowLayoutNode): ShellWindowLayoutNode => {
    if (node.kind === "window") {
      const instanceId = createShellInstanceId(node.windowKindId, counter++);
      const kind = catalog.find((entry) => entry.id === node.windowKindId);
      const template = node.templateId ? findWindowTemplate(catalog, node.windowKindId, node.templateId) : undefined;
      const title = node.title ?? template?.label ?? kind?.label ?? node.windowKindId;
      instances.push({ instanceId, windowKindId: node.windowKindId, templateId: node.templateId, title });
      dispatchWindowTemplate(bus, catalog, node.windowKindId, node.templateId, instanceId);
      return { kind: "window", id: instanceId, title };
    }
    if (node.kind === "stack") {
      return {
        kind: "stack",
        ...(node.size !== undefined ? { size: node.size } : {}),
        children: node.children.map((child) => convert(child) as WindowLayoutWindowNode),
      };
    }
    return {
      kind: node.kind,
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => convert(child) as WindowLayoutStackNode | WindowLayoutAxisNode),
    };
  };
  return { instances, shellLayout: convert(layout.root) };
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

export interface DisplayHostApi {
  readonly windowKinds: readonly WindowKindRuntime[];
  readonly namedLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  saveCurrentLayout: (label: string) => void;
  applyNamedLayout: (layoutId: string) => void;
  deleteUserLayout: (layoutId: string) => void;
}

export const DisplayHostContext = React.createContext<DisplayHostApi | null>(null);

export function useDisplayHost(): DisplayHostApi | null {
  return reactHostPort.useContext(DisplayHostContext);
}

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";

let displayLayoutSaveLabel = "";

function dispatchUiCommand(bus: CommandBus, descriptor: CommandDescriptor, patch: Record<string, unknown>): void {
	bus.dispatch(descriptor.controllerId, descriptor.command, { ...(descriptor.args as object | undefined), ...patch });
}

/** @emoji 🎛️ Renders a declarative {@link UiControlNode} for tree item rows. */
export function renderUiControl(control: UiControlNode, commandBus: CommandBus, platform?: Platform): React.ReactElement {
	switch (control.type) {
		case "input": {
			const node = control;
			const commitOnBlur = node.commit === "blur";
			return (
				<Input
					id={node.id}
					type={node.inputKind === "number" ? "number" : "text"}
					className="h-medium w-full min-w-0"
					value={node.value}
					placeholder={node.placeholder}
					onChange={
						commitOnBlur
							? undefined
							: (event) => {
									const value = node.inputKind === "number" ? Number(event.target.value) : event.target.value;
									dispatchUiCommand(commandBus, node.onChange, { value });
								}
					}
					onBlur={
						commitOnBlur
							? (event) => {
									const value = node.inputKind === "number" ? Number(event.target.value) : event.target.value;
									dispatchUiCommand(commandBus, node.onChange, { value });
								}
							: undefined
					}
				/>
			);
		}
		case "select":
			return (
				<Select value={control.value || undefined} onValueChange={(value) => dispatchUiCommand(commandBus, control.onChange, { value })}>
					<SelectTrigger id={control.id} className="h-medium w-full min-w-0" size="sm">
						<SelectValue placeholder={control.placeholder ?? "Select"} />
					</SelectTrigger>
					<SelectContent>
						{control.items.map((item, index) => (
							<SelectItem key={`${control.id}:${index}:${item.value}`} value={item.value}>
								{item.label}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			);
		case "toggle":
			return <Toggle id={control.id} pressed={control.pressed} text={control.text} icon={resolveDeclarativeControlIcon(control.iconId)} onPressedChange={(pressed) => dispatchUiCommand(commandBus, control.onChange, { pressed })} />;
		case "vec3": {
			const tuple = control.value;
			const mixed = tuple == null || !Array.isArray(tuple) || tuple.length < 3;
			const axes = ["x", "y", "z"] as const;
			return (
				<div className="grid grid-cols-3 gap-1">
					{axes.map((axis, index) => (
						<Input
							key={`${control.id}.${axis}`}
							id={`${control.id}.${axis}`}
							type="number"
							className="h-medium w-full min-w-0"
							value={mixed ? "" : String(tuple[index] ?? 0)}
							placeholder={mixed ? "—" : axis}
							disabled={mixed}
							onChange={(event) => {
								if (mixed) return;
								const parsed = Number(event.target.value);
								if (!Number.isFinite(parsed)) return;
								const next: [number, number, number] = [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0];
								next[index] = parsed;
								dispatchUiCommand(commandBus, control.onChange, { value: next });
							}}
						/>
					))}
				</div>
			);
		}
		case "keyValue":
			return (
				<dl className="grid grid-cols-[auto_1fr] gap-x-2 gap-y-1 text-xs">
					{control.entries.map((entry) => (
						<React.Fragment key={entry.label}>
							<dt className="text-muted-foreground">{entry.label}</dt>
							<dd className="tabular-nums">{entry.value}</dd>
						</React.Fragment>
					))}
				</dl>
			);
		case "slider":
			return (
				<Slider
					id={control.id}
					className="w-full min-w-0"
					max={control.max}
					min={control.min}
					step={control.step}
					value={[control.value]}
					onValueChange={(values) => dispatchUiCommand(commandBus, control.onChange, { value: values[0] ?? control.value })}
				/>
			);
		case "numberStepper": {
			const node = control;
			return (
				<div className="flex min-w-0 w-full items-center gap-1">
					<Button className="h-medium shrink-0 px-2" onClick={() => dispatchUiCommand(commandBus, node.onDelta, { delta: -node.step })} type="button" variant="outline">
						−
					</Button>
					<Input
						className="h-medium min-w-0 flex-1 font-mono text-xs"
						id={node.id}
						onChange={(event) => {
							const parsed = Number(event.target.value);
							if (Number.isFinite(parsed)) {
								dispatchUiCommand(commandBus, node.onAbsolute, { value: parsed });
							}
						}}
						placeholder={node.uniform ? undefined : "Mixed"}
						type="number"
						value={node.uniform && Number.isFinite(node.value) ? String(node.value) : ""}
					/>
					<Button className="h-medium shrink-0 px-2" onClick={() => dispatchUiCommand(commandBus, node.onDelta, { delta: node.step })} type="button" variant="outline">
						+
					</Button>
				</div>
			);
		}
		case "ring":
			return (
				<Ring
					id={control.id}
					onOrbChange={(_orbId, _oldT, newT) => dispatchUiCommand(commandBus, control.onChange, { t: newT })}
					orbs={[{ disabled: control.disabled, id: control.orbId, selected: true, t: control.t }]}
				/>
			);
		case "iconSelect":
			return (
				<IconSelector
					classifyIconSelectorMode={control.classifierKind === "puzzle2d" ? classifyPuzzle2dIconSelectorMode : undefined}
					id={control.id}
					onChange={(next) => dispatchUiCommand(commandBus, control.onChange, { value: next })}
					uniform={control.uniform}
					value={control.value}
				/>
			);
		case "button":
			return (
				<Button
					id={control.id}
					text={control.label}
					icon={resolveDeclarativeControlIcon(control.iconId)}
					onClick={() => commandBus.dispatch(control.command.controllerId, control.command.command, control.command.args)}
				/>
			);
		case "field":
			return (
				<div className="flex flex-col gap-half" data-ui-field={control.id}>
					<label className="text-muted-foreground text-xs">{control.label}</label>
					{renderUiControl(control.child, commandBus, platform)}
				</div>
			);
		case "table":
		case "panel":
			return renderComponentHostSurface(control, "panel", platform);
		default:
			return <span className="text-muted-foreground text-xs">Unsupported control</span>;
	}
}

function uiTreeContextMenuToTreeData(items: UiTreeItemNode["contextMenu"]): TreeDataItem["contextMenu"] {
	if (!items?.length) {
		return undefined;
	}
	return items.map((item) => ({
		id: item.id,
		label: item.label,
		icon: item.icon,
		disabled: item.disabled,
		onSelect: item.onSelect ? () => item.onSelect!() : undefined,
		children: item.children ? uiTreeContextMenuToTreeData(item.children) : undefined,
	}));
}

function uiTreeItemsToTreeData(items: readonly UiTreeItemNode[], commandBus: CommandBus, platform: Platform | undefined): TreeDataItem[] {
	return items.map((item) => {
		const legacyActivate = (item as UiTreeItemNode & { readonly onClick?: () => void }).onClick;
		return {
			id: item.id,
			label: item.label,
			description: item.description,
			icon: item.icon ? renderControlIcon(item.icon, 12) : undefined,
			control: item.control ? renderUiControl(item.control, commandBus, platform) : undefined,
			defaultOpen: item.defaultOpen,
			isSelected: item.selected,
			isHidden: item.isHidden,
			draggable: item.draggable,
			dragData: item.dragData,
			className: item.draggable || item.dragData ? "cursor-grab active:cursor-grabbing" : undefined,
			items: item.items?.length ? uiTreeItemsToTreeData(item.items, commandBus, platform) : undefined,
			actions: item.actions?.map((action) => ({
				kind: "button" as const,
				id: action.id,
				icon: action.icon,
				title: action.title,
				onClick: action.onClick,
				revealOnHover: action.revealOnHover,
			})),
			contextMenu: uiTreeContextMenuToTreeData(item.contextMenu),
			onClick: legacyActivate ?? (item.command ? () => dispatchUiCommand(commandBus, item.command!, {}) : undefined),
			onPointerEnter: item.onPointerEnter,
			onPointerLeave: item.onPointerLeave,
		};
	});
}

function buildUiTreeDragAndDropController(sections: readonly UiTreeSectionNode[], commandBus: CommandBus): TreeDragAndDropController | undefined {
	void commandBus;
	const dragByItemId = collectUiTreeItemDragData(sections);
	if (dragByItemId.size === 0) return undefined;
	const sample = dragByItemId.values().next().value;
	if (sample && PUZZLE_2D_FIXTURE_DRAG_V1_MIME in sample) {
		return puzzle2dFixturePaletteTreeDragController(dragByItemId);
	}
	return puzzle3dFixturePaletteTreeDragController(dragByItemId);
}

/** @emoji 🌲 Maps a declarative {@link UiTreeNode} to a {@link TreePanelConfig}. */
export function uiTreeNodeToTreePanelConfig(treeNode: UiTreeNode, commandBus: CommandBus, platform?: Platform): TreePanelConfig {
	const sections: TreeDataSection[] = treeNode.sections.map((section) => ({
		id: section.id,
		label: section.label ?? "",
		defaultOpen: section.defaultOpen,
		items: uiTreeItemsToTreeData(section.items, commandBus, platform),
	}));
	return {
		sections,
		dragAndDropController: buildUiTreeDragAndDropController(treeNode.sections, commandBus),
		selectedIds: treeNode.selectedIds as string[] | undefined,
		highlightedIds: treeNode.highlightedIds,
		onSelectionChange: treeNode.selectionChange
			? (selectedIds) => dispatchUiCommand(commandBus, treeNode.selectionChange!, { ids: selectedIds })
			: undefined,
	};
}

class DeclarativeSidePanelTreeDefinition implements TreePanelDefinition {
	constructor(
		private readonly platform: Platform,
		private readonly tabId: string,
		private readonly bodyKey: string,
		private readonly bus: CommandBus,
	) {}

	resolveTree(): TreePanelConfig {
		const activeModeId = this.platform.getActiveApp()?.getActiveModeId() ?? null;
		const ctx: SidePanelBodyViewContext = {
			platform: this.platform,
			windowKindId: this.tabId,
			bodyKey: this.bodyKey,
			activeModeId,
			generation: this.platform.generation,
		};
		const node = getSidePanelBodyFactory(this.bodyKey)?.(ctx);
		if (!node) {
			return { sections: [{ id: `${this.tabId}.missing`, items: [{ id: "missing", label: `Missing panel ${this.bodyKey}` }] }] };
		}
		return uiTreeNodeToTreePanelConfig(node, this.bus, this.platform);
	}
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
	const sections: TreeDataSection[] = host.windowKinds.map((kind) => ({
		id: `framework.display.windows.${kind.id}`,
		label: kind.label,
		defaultOpen: false,
		items: [
			{
				id: `framework.display.windows.${kind.id}.kind`,
				label: kind.label,
				draggable: true,
				dragData: {
					[COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id } satisfies WindowTemplateDropPayload),
				},
			},
			...mapWindowTemplatesToTreeItems(kind.id, kind.templates),
		],
	}));
	const dragAndDropController = windowTemplatePaletteTreeDragController();
	return {
		sections: sections.length ? sections : [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
		dragAndDropController,
	};
}

function buildDisplayLayoutTree(host: DisplayHostApi, bus: CommandBus): TreePanelConfig {
	const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
	const userLayouts = host.userLayouts;
	const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
	const userItems = userLayouts.length
		? [
				{
					id: "framework.display.layout.group.saved",
					label: "Saved",
					defaultOpen: false,
					items: groupNamedLayoutsToTreeItems(userLayouts, (layoutId) => host.applyNamedLayout(layoutId), (layoutId) => host.deleteUserLayout(layoutId)),
				},
			]
		: [];
	return {
		sections: [
			{
				id: "framework.display.layout.save",
				label: resolveTranslationLabel("ui.display.saveLayout"),
				defaultOpen: false,
				items: [
					{
						id: "framework.display.layout.save.label",
						label: resolveTranslationLabel("ui.display.saveLayoutPlaceholder"),
						control: (
							<Input
								id="framework.display.save-label"
								value={displayLayoutSaveLabel}
								onChange={(event) => {
									displayLayoutSaveLabel = event.target.value;
								}}
								placeholder={resolveTranslationLabel("ui.display.saveLayoutPlaceholder")}
							/>
						),
					},
					{
						id: "framework.display.layout.save.action",
						label: resolveTranslationLabel("ui.display.saveLayout"),
						control: (
							<Button
								id="framework.display.save"
								size="sm"
								text={resolveTranslationLabel("ui.display.saveLayout")}
								disabled={!displayLayoutSaveLabel.trim()}
								onClick={() => {
									const label = displayLayoutSaveLabel.trim();
									if (!label) return;
									host.saveCurrentLayout(label);
									displayLayoutSaveLabel = "";
								}}
							/>
						),
					},
				],
			},
			{
				id: "framework.display.layout.list",
				label: resolveTranslationLabel("ui.display.tab.layout"),
				defaultOpen: false,
				items: [...builtinItems, ...userItems],
			},
		],
	};
}

class DisplayWindowsTreeDefinition implements TreePanelDefinition {
	constructor(private readonly getHost: () => DisplayHostApi | null) {}

	resolveTree(): TreePanelConfig {
		const host = this.getHost();
		if (!host) {
			return { sections: [{ id: "framework.display.unavailable", items: [{ id: "unavailable", label: "Display unavailable" }] }] };
		}
		return buildDisplayWindowsTree(host);
	}
}

class DisplayLayoutTreeDefinition implements TreePanelDefinition {
	constructor(
		private readonly getHost: () => DisplayHostApi | null,
		private readonly bus: CommandBus,
	) {}

	resolveTree(): TreePanelConfig {
		const host = this.getHost();
		if (!host) {
			return { sections: [{ id: "framework.display.unavailable", items: [{ id: "unavailable", label: "Display unavailable" }] }] };
		}
		return buildDisplayLayoutTree(host, this.bus);
	}
}

/** @emoji 🖥️ Framework display panel tabs (windows + layout), each with its own tree. */
export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null, bus: CommandBus): SidePanelTabConfig[] {
	return [
		{
			id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
			icon: shellTabIconComponent("framework.display.windows", "display"),
			name: "Windows",
			order: -100,
			tree: new DisplayWindowsTreeDefinition(getHost),
		},
		{
			id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
			icon: shellTabIconComponent("framework.display.layout", "display"),
			name: "Layout",
			order: -99,
			tree: new DisplayLayoutTreeDefinition(getHost, bus),
		},
	];
}

/** @emoji 🖥️ First display tab (windows); prefer {@link createFrameworkDisplayPanelTabs}. */
export function createFrameworkDisplayPanelTab(getHost: () => DisplayHostApi | null, bus: CommandBus): SidePanelTabConfig {
	return createFrameworkDisplayPanelTabs(getHost, bus)[0]!;
}

export interface SettingsHostModeEntry {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
}

export interface SettingsHostApi {
	readonly compact: boolean;
	setCompact: (compact: boolean) => void;
	readonly expertise: Expertise;
	setExpertise: (expertise: Expertise) => void;
	readonly computeWorkerCount: number;
	setComputeWorkerCount: (count: number) => void;
	readonly computeThreadsAvailable: boolean;
	readonly theme: ElementsSurfaceTheme;
	setTheme: (theme: ElementsSurfaceTheme) => void;
	readonly appId: string;
	readonly appLabel: string;
	readonly appIconId?: string;
	readonly modes: readonly SettingsHostModeEntry[];
	readonly activeModeId: string | null;
	setActiveModeId: (modeId: string) => void;
	readonly hasModeNav: boolean;
}

export const SettingsHostContext = React.createContext<SettingsHostApi | null>(null);

export function useSettingsHost(): SettingsHostApi | null {
	return reactHostPort.useContext(SettingsHostContext);
}

const FRAMEWORK_SETTINGS_MODE_TAB_ID = "framework.settings.mode";
const FRAMEWORK_SETTINGS_APP_TAB_ID = "framework.settings.app";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";

const SETTINGS_THEME_OPTIONS: readonly ElementsSurfaceTheme[] = ["system", "light", "dark"];

function settingsThemeLabel(theme: ElementsSurfaceTheme): string {
	switch (theme) {
		case "light":
			return resolveTranslationLabel("ui.settings.theme.light");
		case "dark":
			return resolveTranslationLabel("ui.settings.theme.dark");
		default:
			return resolveTranslationLabel("ui.settings.theme.system");
	}
}

const SETTINGS_EXPERTISE_OPTIONS: readonly Expertise[] = [Expertise.BEGINNER, Expertise.NORMAL, Expertise.EXPERT];

function settingsExpertiseLabel(expertise: Expertise): string {
	switch (expertise) {
		case Expertise.BEGINNER:
			return resolveTranslationLabel("settings.expertise.beginner");
		case Expertise.EXPERT:
			return resolveTranslationLabel("settings.expertise.expert");
		default:
			return resolveTranslationLabel("settings.expertise.normal");
	}
}

function buildFrameworkSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
	const items: TreeDataItem[] = [
		{
			id: "framework.settings.general.compact",
			label: resolveTranslationLabel("settings.compact"),
			control: (
				<Toggle
					id="framework.settings.compact"
					pressed={host.compact}
					onPressedChange={(pressed) => {
						host.setCompact(pressed);
						writeStoredUiChromeCompact(pressed);
					}}
					icon={<Icon icon="layout-grid" size="small" />}
				/>
			),
		},
		{
			id: "framework.settings.general.expertise",
			label: resolveTranslationLabel("ui.settings.tab.expertise"),
			control: (
				<Select value={host.expertise} onValueChange={(value) => host.setExpertise(value as Expertise)}>
					<SelectTrigger id="framework.settings.expertise" className="h-medium w-full min-w-0" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						{SETTINGS_EXPERTISE_OPTIONS.map((tier) => (
							<SelectItem key={tier} id={`framework.settings.expertise.${tier}`} value={tier}>
								{settingsExpertiseLabel(tier)}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			),
		},
		{
			id: "framework.settings.general.workers",
			label: "Compute workers",
			control: (
				<Input
					id="framework.settings.workers"
					type="number"
					min={1}
					max={128}
					disabled={!host.computeThreadsAvailable}
					value={String(host.computeWorkerCount)}
					onChange={(event) => {
						const parsed = Number.parseInt(event.target.value, 10);
						if (Number.isFinite(parsed) && parsed >= 1) host.setComputeWorkerCount(parsed);
					}}
					className="h-medium w-full min-w-0"
				/>
			),
		},
	];
	return {
		sections: [
			{
				id: "framework.settings.general.section",
				label: resolveTranslationLabel("ui.settings.tab.general"),
				defaultOpen: false,
				items,
			},
		],
	};
}

function resolveSettingsHostModes(host: SettingsHostApi): readonly SettingsHostModeEntry[] {
	if (host.modes.length > 0) return host.modes;
	return [{ id: host.appId, label: host.appLabel, iconId: host.appIconId }];
}

function buildFrameworkSettingsModeTree(host: SettingsHostApi): TreePanelConfig {
	const modes = resolveSettingsHostModes(host);
	const activeModeId = host.activeModeId ?? modes[0]?.id ?? host.appId;
	return {
		sections: [
			{
				id: "framework.settings.mode.section",
				label: resolveTranslationLabel("ui.settings.tab.mode"),
				defaultOpen: false,
				items: [
					{
						id: "framework.settings.mode.select",
						label: resolveTranslationLabel("ui.settings.tab.mode"),
						control: (
							<Select value={activeModeId} onValueChange={(modeId) => host.setActiveModeId(modeId)}>
								<SelectTrigger id="framework.settings.mode" className="h-medium w-full min-w-0" size="sm">
									<SelectValue placeholder={resolveTranslationLabel("ui.settings.tab.mode")} />
								</SelectTrigger>
								<SelectContent>
									{modes.map((mode) => (
										<SelectItem key={mode.id} id={`framework.settings.mode.${mode.id}`} value={mode.id}>
											{mode.label}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						),
					},
				],
			},
		],
	};
}

function buildFrameworkSettingsAppTree(host: SettingsHostApi, displayHost: DisplayHostApi | null, bus: CommandBus): TreePanelConfig {
	const identityItems: TreeDataItem[] = [
		{
			id: "framework.settings.app.id",
			label: "App id",
			control: <Input id="framework.settings.app.id" value={host.appId} readOnly className="h-medium w-full min-w-0" />,
		},
		{
			id: "framework.settings.app.label",
			label: "App label",
			control: <Input id="framework.settings.app.label" value={host.appLabel} readOnly className="h-medium w-full min-w-0" />,
		},
		{
			id: "framework.settings.app.theme",
			label: resolveTranslationLabel("ui.settings.tab.theme"),
			control: (
				<Select value={host.theme} onValueChange={(value) => host.setTheme(value as ElementsSurfaceTheme)}>
					<SelectTrigger id="framework.settings.theme" className="h-medium w-full min-w-0" size="sm">
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						{SETTINGS_THEME_OPTIONS.map((theme) => (
							<SelectItem key={theme} id={`framework.settings.theme.${theme}`} value={theme}>
								{settingsThemeLabel(theme)}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			),
		},
	];
	const layoutSections = displayHost ? buildDisplayLayoutTree(displayHost, bus).sections : [];
	return {
		sections: [
			{
				id: "framework.settings.app.identity",
				label: resolveTranslationLabel("ui.settings.tab.app"),
				defaultOpen: false,
				items: identityItems,
			},
			...layoutSections,
		],
	};
}

class FrameworkSettingsModeTreeDefinition implements TreePanelDefinition {
	constructor(private readonly getHost: () => SettingsHostApi | null) {}

	resolveTree(): TreePanelConfig {
		const host = this.getHost();
		if (!host) {
			return { sections: [{ id: "framework.settings.unavailable", items: [{ id: "unavailable", label: "Settings unavailable" }] }] };
		}
		return buildFrameworkSettingsModeTree(host);
	}
}

class FrameworkSettingsAppTreeDefinition implements TreePanelDefinition {
	constructor(
		private readonly getHost: () => SettingsHostApi | null,
		private readonly getDisplayHost: () => DisplayHostApi | null,
		private readonly getPlatform: () => Platform | null,
		private readonly bus: CommandBus,
	) {}

	resolveTree(): TreePanelConfig {
		const host = this.getHost();
		if (!host) {
			return { sections: [{ id: "framework.settings.unavailable", items: [{ id: "unavailable", label: "Settings unavailable" }] }] };
		}
		const base = buildFrameworkSettingsAppTree(host, this.getDisplayHost(), this.bus);
		const platform = this.getPlatform();
		const app = platform?.getActiveApp();
		if (!app?.appSettingsBodyKey || !platform) {
			return base;
		}
		const factory = getSidePanelBodyFactory(app.appSettingsBodyKey);
		if (!factory) {
			return base;
		}
		const ctx: SidePanelBodyViewContext = {
			platform,
			windowKindId: app.id,
			bodyKey: app.appSettingsBodyKey,
			activeModeId: app.getActiveModeId(),
			generation: platform.generation,
		};
		const node = factory(ctx);
		if (!node || node.type !== "tree") {
			return base;
		}
		const productConfig = uiTreeNodeToTreePanelConfig(node, this.bus, platform);
		return { sections: [...base.sections, ...productConfig.sections] };
	}
}

class FrameworkSettingsGeneralTreeDefinition implements TreePanelDefinition {
	constructor(private readonly getHost: () => SettingsHostApi | null) {}

	resolveTree(): TreePanelConfig {
		const host = this.getHost();
		if (!host) {
			return { sections: [{ id: "framework.settings.unavailable", items: [{ id: "unavailable", label: "Settings unavailable" }] }] };
		}
		return buildFrameworkSettingsGeneralTree(host);
	}
}

/** @emoji ⚙️ Framework settings panel tabs (mode, app, general chrome options). */
export function createFrameworkSettingsPanelTabs(
	getHost: () => SettingsHostApi | null,
	getDisplayHost: () => DisplayHostApi | null,
	getPlatform: () => Platform | null,
	bus: CommandBus,
): SidePanelTabConfig[] {
	return [
		{
			id: FRAMEWORK_SETTINGS_MODE_TAB_ID,
			icon: shellTabIconComponent("framework.settings.mode", "settings"),
			name: resolveTranslationLabel("ui.settings.tab.mode"),
			order: -300,
			tree: new FrameworkSettingsModeTreeDefinition(getHost),
		},
		{
			id: FRAMEWORK_SETTINGS_APP_TAB_ID,
			icon: shellTabIconComponent("framework.settings.app", "settings"),
			name: resolveTranslationLabel("ui.settings.tab.app"),
			order: -200,
			tree: new FrameworkSettingsAppTreeDefinition(getHost, getDisplayHost, getPlatform, bus),
		},
		{
			id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
			icon: shellTabIconComponent("framework.settings.general", "settings"),
			name: resolveTranslationLabel("ui.settings.tab.general"),
			order: -100,
			tree: new FrameworkSettingsGeneralTreeDefinition(getHost),
		},
	];
}

/** @emoji 🧭 Converts {@link WindowLayout} to the shell {@link Mode} layout tree. */
export function convertFrameworkLayoutToShellLayout(layout: WindowLayout): ShellWindowLayoutNode {
  return convertFrameworkLayoutNodeToShellLayout(layout.root);
}

function windowControlsToEngagement(controls: UIWindowControl[] | undefined): EngagementSpec | undefined {
  if (!controls?.length) return undefined;
  return {
    options: controls.map((control) => ({
      id: control.id,
      label: control.value ?? control.id,
      icon: control.icon,
      pressed: control.value === "on",
      onPress: () => control.onChange?.(control.value === "on" ? "off" : "on"),
    })),
  };
}

type ShellModeWindowBodyCacheEntry = {
  readonly component: UIWindowKindDefinition["component"];
  readonly contextMenu: UIWindowKindDefinition["contextMenu"];
  readonly body: React.ReactNode;
};

/** @emoji 🧷 Stable viewport body for one shell window kind (engagement chrome may refresh without remounting GL). */
export function resolveShellModeWindowBody(cache: Map<string, ShellModeWindowBodyCacheEntry>, windowKind: UIWindowKindDefinition): React.ReactNode {
  const existing = cache.get(windowKind.id);
  if (existing && existing.component === windowKind.component && existing.contextMenu === windowKind.contextMenu) {
    return existing.body;
  }
  const WindowComponent = windowKind.component;
  const body = (
    <ShellWindowInstanceProvider
      value={{ instanceId: windowKind.id, windowKindId: windowKind.windowKindId ?? windowKind.id, templateId: windowKind.templateId }}
    >
      <ContextMenu items={windowKind.contextMenu}>
        <div className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col">
          <WindowComponent />
        </div>
      </ContextMenu>
    </ShellWindowInstanceProvider>
  );
  cache.set(windowKind.id, { component: windowKind.component, contextMenu: windowKind.contextMenu, body });
  return body;
}

/** @emoji 🪟 Pure-React resizable mode canvas backed by {@link Mode}. */
export const ShellModeCanvas: React.FC<{
  windowKinds: UIWindowKindDefinition[];
  windowKindCatalog: readonly WindowKindRuntime[];
  defaultLayout: WindowLayout;
  namedLayouts: readonly NamedLayout[];
  namedLayoutStore: NamedLayoutStore;
  commandBus: CommandBus;
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string | null) => void;
  onDisplayHostReady?: (host: DisplayHostApi) => void;
}> = reactHostPort.memo(function ShellModeCanvas({
  windowKinds,
  windowKindCatalog,
  defaultLayout,
  namedLayouts,
  namedLayoutStore,
  commandBus,
  activeWindowId,
  onActiveWindowChange,
  onDisplayHostReady,
}) {
  const layoutKey = reactHostPort.useMemo(() => JSON.stringify(defaultLayout), [defaultLayout]);
  const catalogKey = reactHostPort.useMemo(() => windowKindCatalog.map((kind) => kind.id).join("|"), [windowKindCatalog]);
  const defaultLayoutRef = reactHostPort.useRef(defaultLayout);
  const windowKindCatalogRef = reactHostPort.useRef(windowKindCatalog);
  defaultLayoutRef.current = defaultLayout;
  windowKindCatalogRef.current = windowKindCatalog;
  const [instances, setInstances] = reactHostPort.useState<ShellWindowInstance[]>(() => bootstrapShellInstances(defaultLayout, windowKindCatalog, commandBus));
  const [shellLayout, setShellLayout] = reactHostPort.useState<ShellWindowLayoutNode>(() => convertFrameworkLayoutToShellLayout(defaultLayout));
  const liveFrameworkLayoutRef = reactHostPort.useRef<WindowLayout>(defaultLayout);

  reactHostPort.useEffect(() => {
    const layout = defaultLayoutRef.current;
    const catalog = windowKindCatalogRef.current;
    setInstances(bootstrapShellInstances(layout, catalog, commandBus));
    setShellLayout(convertFrameworkLayoutToShellLayout(layout));
    liveFrameworkLayoutRef.current = layout;
  }, [commandBus, layoutKey, catalogKey]);

  const windowBodyCacheRef = reactHostPort.useRef(new Map<string, ShellModeWindowBodyCacheEntry>());
  const kindById = reactHostPort.useMemo(() => new Map(windowKinds.map((kind) => [kind.id, kind])), [windowKinds]);

  const windows = reactHostPort.useMemo<ModeWindowDescriptor[]>(
    () =>
      instances.map((instance) => {
        const windowKind = kindById.get(instance.windowKindId);
        const component =
          windowKind?.component ??
          (() => (
            <div className="p-2 text-xs text-muted-foreground">
              Missing window kind &quot;{instance.windowKindId}&quot;
            </div>
          ));
        return {
          id: instance.instanceId,
          title: instance.title,
          fill: true,
          showControls: true,
          controls: windowKind?.controls ? <UIWindowControlsGroup controls={windowKind.controls} /> : undefined,
          measures: windowKind?.measures?.length ? <UIWindowMeasures measures={windowKind.measures} /> : undefined,
          engagement: windowKind?.engagement ?? windowControlsToEngagement(windowKind?.controls),
          children: resolveShellModeWindowBody(windowBodyCacheRef.current, {
            id: instance.instanceId,
            windowKindId: instance.windowKindId,
            templateId: instance.templateId,
            label: instance.title,
            component: component as React.ComponentType,
            controls: windowKind?.controls,
            measures: windowKind?.measures,
            engagement: windowKind?.engagement,
          }),
        };
      }),
    [instances, kindById],
  );

  const instancesRef = reactHostPort.useRef(instances);
  instancesRef.current = instances;

  const handleLayoutChange = reactHostPort.useCallback((layout: ShellWindowLayoutNode) => {
    setShellLayout(layout);
    liveFrameworkLayoutRef.current = shellLayoutToFrameworkLayout(layout, instancesRef.current);
  }, []);

  const handleWindowClose = reactHostPort.useCallback(
    (windowId: string) => {
      setInstances((prev) => {
        const next = prev.filter((instance) => instance.instanceId !== windowId);
        if (activeWindowId === windowId) onActiveWindowChange?.(next[0]?.instanceId ?? null);
        return next;
      });
      setShellLayout((prev) => collapseLayout(removeWindowFromLayout(prev, windowId)) ?? { kind: "stack", children: [] });
    },
    [activeWindowId, onActiveWindowChange],
  );

  const handleTemplateDrop = reactHostPort.useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      const kind = windowKindCatalog.find((entry) => entry.id === payload.windowKindId);
      const template = findWindowTemplate(windowKindCatalog, payload.windowKindId, payload.templateId);
      const instanceId = createShellInstanceId(payload.windowKindId, instances.length);
      const title = template?.label ?? kind?.label ?? payload.windowKindId;
      setInstances((prev) => [...prev, { instanceId, windowKindId: payload.windowKindId, templateId: payload.templateId, title }]);
      setShellLayout((prev) => insertWindowAtDropZone(prev, instanceId, target));
      dispatchWindowTemplate(commandBus, windowKindCatalog, payload.windowKindId, payload.templateId, instanceId);
      onActiveWindowChange?.(instanceId);
    },
    [commandBus, instances.length, onActiveWindowChange, windowKindCatalog],
  );

  const applyNamedLayout = reactHostPort.useCallback(
    (layoutId: string) => {
      const entry = [...namedLayouts, ...namedLayoutStore.getSnapshot()].find((layout) => layout.id === layoutId);
      if (!entry) return;
      const next = instantiateFrameworkLayout(entry.layout, windowKindCatalog, commandBus);
      setInstances(next.instances);
      setShellLayout(next.shellLayout);
      liveFrameworkLayoutRef.current = entry.layout;
      const active = next.instances[0]?.instanceId;
      if (active) onActiveWindowChange?.(active);
    },
    [commandBus, namedLayoutStore, namedLayouts, onActiveWindowChange, windowKindCatalog],
  );

  const saveCurrentLayout = reactHostPort.useCallback(
    (label: string) => {
      const layout = shellLayoutToFrameworkLayout(shellLayout, instances);
      liveFrameworkLayoutRef.current = layout;
      const id = `user-${label.toLowerCase().replace(/\s+/g, "-")}-${Date.now()}`;
      namedLayoutStore.save(createNamedLayout(id, label, layout, "user"));
    },
    [instances, namedLayoutStore, shellLayout],
  );

  const deleteUserLayout = reactHostPort.useCallback((layoutId: string) => namedLayoutStore.remove(layoutId), [namedLayoutStore]);

  const userLayouts = reactHostPort.useSyncExternalStore(namedLayoutStore.subscribe.bind(namedLayoutStore), () => namedLayoutStore.getSnapshot(), () => namedLayoutStore.getSnapshot());

  const onDisplayHostReadyRef = reactHostPort.useRef(onDisplayHostReady);
  onDisplayHostReadyRef.current = onDisplayHostReady;
  const namedLayoutsRef = reactHostPort.useRef(namedLayouts);
  namedLayoutsRef.current = namedLayouts;

  reactHostPort.useEffect(() => {
    onDisplayHostReadyRef.current?.({
      windowKinds: windowKindCatalogRef.current,
      namedLayouts: namedLayoutsRef.current,
      userLayouts,
      saveCurrentLayout,
      applyNamedLayout,
      deleteUserLayout,
    });
  }, [applyNamedLayout, catalogKey, deleteUserLayout, saveCurrentLayout, userLayouts]);

  return (
    <Mode
      windows={windows}
      layout={shellLayout}
      activeWindowId={activeWindowId}
      onActiveWindowChange={onActiveWindowChange}
      onWindowClose={handleWindowClose}
      onLayoutChange={handleLayoutChange}
      onTemplateDrop={handleTemplateDrop}
      className="h-full w-full"
    />
  );
});

// #region 🎼UISearch

/**
 * A searchable item for the global UI command palette.
 * Consumers provide items; the UI renders them in a CommandDialog with fuzzy search.
 **/
export interface UISearchItem {
  id: string;
  label: string;
  description?: string;
  icon?: React.ReactNode;
  category?: string;
  onSelect: () => void;
}

/**
 * Global search command palette for the UI (Ctrl+P / Cmd+P).
 * Uses Fuse.js for fuzzy matching and CommandDialog for rendering.
 **/
const UISearch: React.FC<{
  items: UISearchItem[];
  open: boolean;
  onOpenChange: (open: boolean) => void;
  placeholder?: string;
  emptyMessage?: string;
}> = ({ items, open, onOpenChange, placeholder, emptyMessage }) => {
  const { t } = useUiTranslation();
  const dialogTitle = resolveTranslationLabel(t("ui.search.title" as const));
  const dialogDescription = resolveTranslationLabel(t("ui.search.description" as const));
  const dialogPlaceholder = placeholder ?? resolveTranslationLabel(t("ui.search.placeholder" as const));
  const dialogEmpty = emptyMessage ?? resolveTranslationLabel(t("ui.search.empty" as const));
  const [query, setQuery] = reactHostPort.useState("");

  const fuse = reactHostPort.useMemo(
    () =>
      new Fuse(items, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [items],
  );

  const results = reactHostPort.useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return items.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UISearchItem>);
  }, [fuse, query, items]);

  const grouped = reactHostPort.useMemo(() => {
    const groups: Record<string, FuseResult<UISearchItem>[]> = {};
    results.forEach((result) => {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [results]);

  const handleSelect = reactHostPort.useCallback(
    (item: UISearchItem) => {
      onOpenChange(false);
      setQuery("");
      item.onSelect();
    },
    [onOpenChange],
  );

  return (
    <CommandDialog title={dialogTitle} description={dialogDescription} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.search.input" placeholder={dialogPlaceholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{dialogEmpty}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem
                key={`${result.item.id}-${idx}`}
                value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()}
                onSelect={() => handleSelect(result.item)}
              >
                <div className="flex items-center gap-single">
                  {result.item.icon}
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description && <span className="text-xs text-muted-foreground">{result.item.description}</span>}
                  </div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
};

// #endregion 🎼UISearch

// #region 🌧️UIFind

/**
 * A findable item scoped to an app for the per-app find palette.
 **/
export interface UIFindItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
}

/**
 * Context value for per-app find functionality.
 * Apps set find items and a callback; the UI renders the find palette.
 **/
export interface UIFindContextValue {
  findItems: UIFindItem[];
  setFindItems: (items: UIFindItem[]) => void;
  setOnFindItem: (callback: ((itemId: string) => void) | undefined) => void;
  triggerFindItem: (itemId: string) => void;
}

const UIFindContext = reactHostPort.createContext<UIFindContextValue | null>(null);
const EMPTY_UI_FIND_ITEMS: UIFindItem[] = [];

function areFindItemsShallowEqual(previousItems: UIFindItem[], nextItems: UIFindItem[]): boolean {
  if (previousItems === nextItems) return true;
  if (previousItems.length !== nextItems.length) return false;
  for (let i = 0; i < nextItems.length; i++) {
    if (previousItems[i] !== nextItems[i]) return false;
  }
  return true;
}

/**
 * Provider for per-app find functionality.
 * Wraps children and exposes find items + trigger via context.
 **/
export const UIFindProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [findItems, setFindItems] = reactHostPort.useState<UIFindItem[]>([]);
  const onFindItemCallbackRef = reactHostPort.useRef<((itemId: string) => void) | undefined>(undefined);

  const setFindItemsStable = reactHostPort.useCallback((items: UIFindItem[]) => {
    setFindItems((previousItems) => {
      return areFindItemsShallowEqual(previousItems, items) ? previousItems : items;
    });
  }, []);

  const setOnFindItem = reactHostPort.useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);

  const triggerFindItem = reactHostPort.useCallback((itemId: string) => {
    if (onFindItemCallbackRef.current) {
      onFindItemCallbackRef.current(itemId);
    }
  }, []);

  const contextValue = reactHostPort.useMemo(() => ({ findItems, setFindItems: setFindItemsStable, setOnFindItem, triggerFindItem }), [findItems, setFindItemsStable, setOnFindItem, triggerFindItem]);
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
};

/**
 * Hook to access the find context. Throws if used outside UIFindProvider.
 **/
export function useUIFind(): UIFindContextValue {
  const context = reactHostPort.useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

/**
 * Hook to access the find context. Returns null if outside UIFindProvider.
 **/
export function useUIFindSafe(): UIFindContextValue | null {
  return reactHostPort.useContext(UIFindContext);
}

/**
 * Per-app find command palette (Ctrl+F / Cmd+F).
 * Renders a CommandDialog with fuzzy search over the active app's find items.
 **/
const UIFind: React.FC<{
  open: boolean;
  onOpenChange: (open: boolean) => void;
  placeholder?: string;
  emptyMessage?: string;
}> = ({ open, onOpenChange, placeholder, emptyMessage }) => {
  const { t } = useUiTranslation();
  const dialogTitle = resolveTranslationLabel(t("ui.find.title" as const));
  const dialogDescription = resolveTranslationLabel(t("ui.find.description" as const));
  const dialogPlaceholder = placeholder ?? resolveTranslationLabel(t("ui.find.placeholder" as const));
  const dialogEmpty = emptyMessage ?? resolveTranslationLabel(t("ui.find.empty" as const));
  const [query, setQuery] = reactHostPort.useState("");
  const findContext = reactHostPort.useContext(UIFindContext);
  const findItems = findContext?.findItems || [];
  const triggerFindItem = findContext?.triggerFindItem;

  const fuse = reactHostPort.useMemo(
    () =>
      new Fuse(findItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [findItems],
  );

  const results = reactHostPort.useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return findItems.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UIFindItem>);
  }, [fuse, query, findItems]);

  const grouped = reactHostPort.useMemo(() => {
    const groups: Record<string, FuseResult<UIFindItem>[]> = {};
    results.forEach((result) => {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [results]);

  const handleSelect = reactHostPort.useCallback(
    (item: UIFindItem) => {
      onOpenChange(false);
      setQuery("");
      if (triggerFindItem) triggerFindItem(item.id);
    },
    [onOpenChange, triggerFindItem],
  );

  if (!findContext || findItems.length === 0) return null;

  return (
    <CommandDialog title={dialogTitle} description={dialogDescription} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.find.input" placeholder={dialogPlaceholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{dialogEmpty}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem
                key={`${result.item.id}-${idx}`}
                value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()}
                onSelect={() => handleSelect(result.item)}
              >
                <div className="flex flex-col">
                  <span>{result.item.label}</span>
                  {result.item.description && <span className="text-xs text-muted-foreground">{result.item.description}</span>}
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
};

// #endregion 🌧️UIFind

// #region 📔UIToolbar

/** @emoji 🎛 Resolved toolbar leaf for React rendering. */
export type UIToolLeaf =
	| { id: string; kind: "separator"; order?: number }
	| {
			id: string;
			icon: React.ReactNode;
			label?: string;
			text?: string;
			onClick?: () => void;
			kind?: "button";
			order?: number;
	  }
	| {
			id: string;
			icon: React.ReactNode;
			label?: string;
			text?: string;
			kind: "toggle";
			pressed?: boolean;
			onPressedChange?: (pressed: boolean) => void;
			order?: number;
	  };

/** @emoji 🌳 Resolved toolbar node for React rendering. */
export type UIToolNode =
	| UIToolLeaf
	| {
			id: string;
			kind: "collection";
			icon: React.ReactNode;
			label?: string;
			text?: string;
			order?: number;
			children: readonly UIToolNode[];
	  };

/** @emoji 🗂️ Root toolbar tree registered by an app or global UI shell (React view layer). */
export type ToolbarViewTools = readonly UIToolNode[];

function sortViewToolNodes(nodes: readonly UIToolNode[]): UIToolNode[] {
	return [...nodes].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

function isInteractiveViewToolNode(node: UIToolNode): boolean {
	if (node.kind === "separator") return false;
	if (node.kind === "collection") return hasInteractiveViewToolNodes(node.children);
	return true;
}

function hasInteractiveViewToolNodes(nodes?: readonly UIToolNode[]): boolean {
	return Boolean(nodes?.some((node) => isInteractiveViewToolNode(node)));
}

/** @emoji 🔢 Counts registered toolbar nodes recursively. */
export function countToolbarViewTools(tools?: ToolbarViewTools): number {
	if (!tools?.length) return 0;
	return tools.reduce((sum, node) => sum + (node.kind === "collection" ? 1 + countToolbarViewTools(node.children) : 1), 0);
}

function mergeViewToolSiblingLists(base: readonly UIToolNode[], extension: readonly UIToolNode[]): UIToolNode[] {
	const merged = new Map<string, UIToolNode>();
	for (const node of base) merged.set(node.id, node);
	for (const node of extension) {
		const existing = merged.get(node.id);
		if (existing?.kind === "collection" && node.kind === "collection") {
			merged.set(node.id, { ...node, children: mergeViewToolSiblingLists(existing.children, node.children) });
			continue;
		}
		merged.set(node.id, node);
	}
	return sortViewToolNodes([...merged.values()]);
}

/** @emoji 🔀 Merges toolbar trees by sibling id (collection children merge recursively). */
export function mergeToolbarViewTools(base?: ToolbarViewTools, extension?: ToolbarViewTools): ToolbarViewTools | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = mergeViewToolSiblingLists(base ?? [], extension ?? []);
	return merged.length > 0 ? merged : undefined;
}

function isLeafOnlyViewCollection(node: UIToolNode): boolean {
	if (node.kind !== "collection") return false;
	return node.children.every((child) => child.kind !== "collection");
}

function hasInteractiveViewToolLeaves(items: readonly UIToolLeaf[]): boolean {
	return items.some((node) => node.kind !== "separator");
}

function viewToolLeaves(nodes: readonly UIToolNode[]): UIToolLeaf[] {
	return sortViewToolNodes(nodes).filter((node): node is UIToolLeaf => node.kind !== "collection");
}

type ViewToolCollection = Extract<UIToolNode, { kind: "collection" }>;

type ToolbarRibbonSegment =
	| { kind: "picker"; collections: readonly ViewToolCollection[]; depth: number }
	| { kind: "tools"; items: readonly UIToolLeaf[] };

function buildToolbarRibbonSegments(nodes: readonly UIToolNode[], path: readonly string[], depth = 0): ToolbarRibbonSegment[] {
	const sorted = sortViewToolNodes(nodes);
	const collections = sorted.filter((node): node is ViewToolCollection => node.kind === "collection" && !node.disabled);
	const looseLeaves = sorted.filter((node): node is UIToolLeaf => node.kind !== "collection");
	const segments: ToolbarRibbonSegment[] = [];

	if (collections.length === 0) {
		if (hasInteractiveViewToolLeaves(looseLeaves)) segments.push({ kind: "tools", items: looseLeaves });
		return segments;
	}

	if (collections.length === 1) {
		if (hasInteractiveViewToolLeaves(looseLeaves)) segments.push({ kind: "tools", items: looseLeaves });
		segments.push(...buildToolbarRibbonSegments(collections[0].children, path, depth));
		return segments;
	}

	if (collections.every(isLeafOnlyViewCollection)) {
		for (const collection of collections) {
			const leaves = viewToolLeaves(collection.children);
			if (hasInteractiveViewToolLeaves(leaves)) segments.push({ kind: "tools", items: leaves });
		}
		if (hasInteractiveViewToolLeaves(looseLeaves)) segments.push({ kind: "tools", items: looseLeaves });
		return segments;
	}

	segments.push({ kind: "picker", collections, depth });
	const activeId = path[depth] ?? collections[0]?.id;
	const active = collections.find((node) => node.id === activeId) ?? collections[0];
	if (!active) return segments;
	segments.push(...buildToolbarRibbonSegments(active.children, path, depth + 1));
	return segments;
}

function reconcileViewToolPath(nodes: readonly UIToolNode[], path: readonly string[]): readonly string[] {
	let current = nodes;
	const reconciled: string[] = [];
	let pathIndex = 0;

	while (true) {
		const collections = sortViewToolNodes(current).filter(
			(node): node is ViewToolCollection => node.kind === "collection" && !node.disabled,
		);
		if (collections.length === 0) break;
		if (collections.length > 1 && collections.every(isLeafOnlyViewCollection)) break;
		if (collections.length === 1) {
			current = collections[0].children;
			continue;
		}

		let collectionId = path[pathIndex];
		if (!collectionId || !collections.some((node) => node.id === collectionId)) {
			collectionId = collections[0]?.id;
		}
		if (!collectionId) break;
		reconciled.push(collectionId);
		const active = collections.find((node) => node.id === collectionId);
		if (!active || active.kind !== "collection") break;
		current = active.children;
		pathIndex++;
	}

	return reconciled;
}

const UIToolbarItems: React.FC<{ items: readonly UIToolLeaf[] }> = ({ items }) => {
	const sorted = reactHostPort.useMemo(() => sortViewToolNodes(items) as UIToolLeaf[], [items]);
	const nodes = reactHostPort.useMemo(() => {
		const rendered: React.ReactNode[] = [];
		let buttonRun: UIToolLeaf[] = [];
		let toggleRun: UIToolLeaf[] = [];

		const flushButtons = () => {
			if (buttonRun.length === 0) return;
			const run = buttonRun;
			buttonRun = [];
			rendered.push(
				<ToolbarItem key={`buttons-${run.map((entry) => entry.id).join("-")}`}>
					<ButtonGroup>
						{run.map((entry) => (
							<ButtonGroupItem key={entry.id} id={entry.id} icon={entry.icon} text={entry.text ?? entry.label} onClick={entry.onClick} />
						))}
					</ButtonGroup>
				</ToolbarItem>,
			);
		};

		const flushToggles = () => {
			if (toggleRun.length === 0) return;
			const run = toggleRun;
			toggleRun = [];
			rendered.push(
				<ToolbarItem key={`toggles-${run.map((entry) => entry.id).join("-")}`}>
					<ToggleGroup
						kind="multiple"
						value={run.filter((entry) => entry.pressed).map((entry) => entry.id)}
						onValueChange={(values) => {
							for (const entry of run) {
								const pressed = values.includes(entry.id);
								if ((entry.pressed ?? false) !== pressed) entry.onPressedChange?.(pressed);
							}
						}}
						items={run.map((entry) => ({
							value: entry.id,
							id: entry.id,
							icon: entry.icon,
							text: entry.text ?? entry.label,
						}))}
					/>
				</ToolbarItem>,
			);
		};

		const flushRuns = () => {
			flushButtons();
			flushToggles();
		};

		for (const item of sorted) {
			if (item.kind === "separator") {
				flushRuns();
				rendered.push(<ToolbarDivider key={item.id} />);
				continue;
			}
			if (item.kind === "toggle") {
				flushButtons();
				toggleRun.push(item);
				continue;
			}
			flushToggles();
			buttonRun.push(item);
		}
		flushRuns();
		return rendered;
	}, [sorted]);

	return <ToolbarGroup>{nodes}</ToolbarGroup>;
};

/** @emoji ✅ True when the toolbar tree has at least one interactive leaf. */
export function hasToolbarViewTools(tools?: ToolbarViewTools): boolean {
	return hasInteractiveViewToolNodes(tools);
}

/** @emoji 🎀 Renders a drill-down ribbon: picker zones plus one zone per flattened leaf-only collection group. */
const UIToolbar: React.FC<{
	tools: ToolbarViewTools;
	className?: string;
}> = ({ tools, className }) => {
	const [activePath, setActivePath] = reactHostPort.useState<readonly string[]>([]);

	reactHostPort.useEffect(() => {
		setActivePath((previousPath) => reconcileViewToolPath(tools, previousPath));
	}, [tools]);

	const segments = reactHostPort.useMemo(() => buildToolbarRibbonSegments(tools, activePath), [tools, activePath]);

	if (!hasInteractiveViewToolNodes(tools)) return null;

	return (
		<UiChromeLabelPolicyProvider policy="always">
			<div
				role="toolbar"
				id="ui.toolbar"
				className={cn("pointer-events-auto flex w-fit max-w-full shrink-0 items-center justify-start gap-single", className)}
			>
				{segments.map((segment, index) => (
					<ToolbarZone
						key={
							segment.kind === "picker"
								? `picker-${segment.depth}-${segment.collections.map((entry) => entry.id).join("-")}`
								: `tools-${index}-${segment.items.map((entry) => entry.id).join("-")}`
						}
					>
						{segment.kind === "picker" ? (
							<ToolbarItem>
								<ToggleGroup
									kind="single"
									value={activePath[segment.depth] ?? ""}
									onValueChange={(value) => {
										if (value) setActivePath(reconcileViewToolPath(tools, [...activePath.slice(0, segment.depth), value]));
									}}
									items={segment.collections.map((entry) => ({
										value: entry.id,
										id: `ui.toolbar.group.${entry.id}`,
										icon: entry.icon,
										text: entry.text ?? entry.label,
									}))}
								/>
							</ToolbarItem>
						) : (
							<UIToolbarItems items={segment.items} />
						)}
					</ToolbarZone>
				))}
			</div>
		</UiChromeLabelPolicyProvider>
	);
};

export { UISearch, UIFind, UIToolbar };
export { App, Mode, Ui } from "@semio-tech/ui-react";

// #endregion 📔UIToolbar

//#region 🧪ShellCanvasTests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("toolbar ribbon", () => {
		const puzzle2dLike: ToolbarViewTools = [
			{
				id: "selection",
				kind: "collection",
				icon: null,
				children: [
					{ id: "methods", kind: "collection", icon: null, order: 0, children: [{ id: "rect", kind: "toggle", icon: null, pressed: true }] },
					{ id: "mode", kind: "collection", icon: null, order: 1, children: [{ id: "default", kind: "toggle", icon: null, pressed: true }] },
					{ id: "clear", kind: "button", icon: null, order: 20, onClick: () => undefined },
				],
			},
			{ id: "view", kind: "collection", icon: null, children: [{ id: "grid", kind: "toggle", icon: null, pressed: false }] },
		];

		it("flattens sibling leaf-only collections into separate tool zones", () => {
			const segments = buildToolbarRibbonSegments(puzzle2dLike, ["selection"]);
			expect(segments.map((segment) => segment.kind)).toEqual(["picker", "tools", "tools", "tools"]);
			expect(segments[1]?.kind === "tools" && segments[1].items.map((item) => item.id)).toEqual(["rect"]);
			expect(segments[2]?.kind === "tools" && segments[2].items.map((item) => item.id)).toEqual(["default"]);
			expect(segments[3]?.kind === "tools" && segments[3].items.map((item) => item.id)).toEqual(["clear"]);
		});

		it("reconciles picker path only through levels that need a collection choice", () => {
			expect(reconcileViewToolPath(puzzle2dLike, [])).toEqual(["selection"]);
			expect(reconcileViewToolPath(puzzle2dLike, ["view"])).toEqual(["view"]);
		});

		it("replaces downstream segments when a root collection changes", () => {
			const selectionSegments = buildToolbarRibbonSegments(puzzle2dLike, ["selection"]);
			const viewSegments = buildToolbarRibbonSegments(puzzle2dLike, ["view"]);
			expect(selectionSegments.length).toBeGreaterThan(viewSegments.length);
			expect(viewSegments.map((segment) => segment.kind)).toEqual(["picker", "tools"]);
		});
	});

	describe("layout helpers", () => {
		it("converts abstract layout nodes to GoldenLayout config", () => {
			expect(
				layoutNodeToGoldenLayoutConfig({
					root: {
						kind: "row",
						children: [
							{
								kind: "stack",
								size: 100,
								children: [{ kind: "window", windowKindId: "table", title: "table" }],
							},
						],
					},
				}),
			).toEqual({
				root: {
					type: "row",
					content: [
						{
							type: "stack",
							size: "100%",
							content: [{ type: "component", componentName: "table", title: "table", componentState: {} }],
						},
					],
				},
			});
		});

		it("merges toolbar trees and omits empty roots", () => {
			const merged = mergeToolbarViewTools(
				[{ id: "selection", kind: "collection", icon: null, children: [{ id: "a", onClick: () => undefined }] }],
				[{ id: "filter", kind: "collection", icon: null, children: [{ id: "b", onClick: () => undefined }] }],
			);
			expect(merged?.length).toBe(2);
			expect(hasInteractiveViewToolNodes([{ id: "filter", kind: "collection", icon: null, children: [{ id: "sep", kind: "separator" }] }])).toBe(false);
		});
	});
}
//#endregion 🧪ShellCanvasTests

//#endregion 📦shell-canvas.tsx

//#region 📦product-app-context.tsx
/** @emoji 🧭 Props for {@link PlatformView} (navbar, panels, golden-layout canvas). */
export interface PlatformViewProps {
	readonly platform: Platform;
	defaultAppId?: string;
	uri?: string;
	onNavigate?: (uri: string) => void;
	canGoBack?: boolean;
	onGoBack?: () => void;
	canGoForward?: boolean;
	onGoForward?: () => void;
	canGoUp?: boolean;
	onGoUp?: () => void;
	mobile?: boolean;
	mobileQuery?: string;
	className?: string;
	resolvedWindowKindsOverride?: UIWindowKindDefinition[];
	slotToolbar?: React.ReactNode;
	/** @emoji 🧪 Center navbar slot (e.g. sketchpad kit / open document picker). */
	slotNavbarCenter?: React.ReactNode;
	extraFooterItems?: ChromeFooterRow[];
	augmentPanelTabs?: Partial<Record<PanelKind, SidePanelTabConfig[]>>;
	initialPanelVisibility?: UIPanelVisibility;
}

export interface UIPanelVisibility {
	leftSidePanel: boolean;
	rightSidePanel: boolean;
}

export interface AppContextValue {
	platform: Platform;
	activeAppId: string;
	setActiveAppId: (id: string) => void;
	activeApp: ResolvedAppState;
	activeModeId: string | null;
	setActiveModeId: (id: string) => void;
	apps: AppRuntime[];
	panelVisibility: UIPanelVisibility;
	togglePanel: (panel: keyof UIPanelVisibility) => void;
	uri: string;
	navigate: (uri: string) => void;
	canGoBack: boolean;
	goBack: () => void;
	canGoForward: boolean;
	goForward: () => void;
	canGoUp: boolean;
	goUp: () => void;
}

export const AppContext = reactHostPort.createContext<AppContextValue | undefined>(undefined);

/** @emoji 🪝 Returns the active {@link Platform} shell context from the nearest {@link AppContext}. */
export function useApp(): AppContextValue {
	const ctx = reactHostPort.useContext(AppContext);
	if (!ctx) throw new Error("useApp must be used within a PlatformView");
	return ctx;
}

//#endregion ­ƒôªworkbench-app-context.tsx

//#region ­ƒôªworkbench-history.tsx

/** @emoji ­ƒöû Single URI stack entry. */
export interface UIHistoryEntry {
	readonly uri: string;
}

/** @emoji ­ƒöû URI navigation stack state. */
export interface UIHistory {
	readonly entries: readonly UIHistoryEntry[];
	readonly index: number;
}

/** @emoji ­ƒº¡ Manages URI history with back, forward, up, and navigate. */
export function useUIHistory(initialUri = "/"): {
	readonly history: UIHistory;
	readonly uri: string;
	readonly canGoBack: boolean;
	readonly canGoForward: boolean;
	readonly canGoUp: boolean;
	readonly parentUri: string | null;
	readonly goBack: () => void;
	readonly goForward: () => void;
	readonly goUp: () => void;
	readonly navigate: (uri: string) => void;
} {
	const [history, setHistory] = reactHostPort.useState<UIHistory>({
		entries: [{ uri: initialUri }],
		index: 0,
	});
	const uri = history.entries[history.index]?.uri ?? initialUri;
	const canGoBack = history.index > 0;
	const canGoForward = history.index < history.entries.length - 1;
	const segments = uri.split("/").filter(Boolean);
	const canGoUp = segments.length > 0;
	const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

	const goBack = reactHostPort.useCallback(() => {
		setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
	}, []);
	const goForward = reactHostPort.useCallback(() => {
		setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
	}, []);
	const goUp = reactHostPort.useCallback(() => {
		if (!canGoUp || parentUri === null) return;
		setHistory((prev) => {
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
		});
	}, [canGoUp, parentUri]);
	const navigate = reactHostPort.useCallback((targetUri: string) => {
		setHistory((prev) => {
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
		});
	}, []);
	const syncUri = reactHostPort.useCallback((targetUri: string) => {
		setHistory((prev) => {
			const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
			if (existingIndex >= 0) {
				return { ...prev, index: existingIndex };
			}
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
		});
	}, []);

	return { history, uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("useUIHistory types", () => {
		it("exports history entry shape", () => {
			const entry: UIHistoryEntry = { uri: "/test" };
			expect(entry.uri).toBe("/test");
		});
	});

	describe("useUIHistory navigate", () => {
		it("appends one stack entry per navigate call", () => {
			let history: UIHistory = { entries: [{ uri: "/" }], index: 0 };
			const navigate = (targetUri: string) => {
				history = {
					entries: [...history.entries.slice(0, history.index + 1), { uri: targetUri }],
					index: history.index + 1,
				};
			};
			navigate("/apps/a");
			navigate("/apps/b");
			expect(history.entries.map((entry) => entry.uri)).toEqual(["/", "/apps/a", "/apps/b"]);
			expect(history.index).toBe(2);
		});
	});
}
//#endregion ­ƒº¬Tests

//#endregion ­ƒôªworkbench-history.tsx

//#region ­ƒôªui-declarative-renderer.tsx

//#region 🔖ComponentKindRenderer
type ComponentKindRendererProps = {
	readonly component: Component<unknown>;
	readonly node: UiComponentHostSurfaceNode;
	readonly commandBus: CommandBus;
	readonly layout: "canvas" | "panel";
	readonly platform?: Platform;
};

type ComponentKindRenderer = React.ComponentType<ComponentKindRendererProps>;

const componentKindRenderers = new Map<ComponentKind, ComponentKindRenderer>();

/** @emoji 🧩 Registers a React renderer for a {@link ComponentKind} driven by {@link Component} view-models. */
export function registerComponentKindRenderer(kind: ComponentKind, Renderer: ComponentKindRenderer): void {
	componentKindRenderers.set(kind, Renderer);
}

/** @emoji 🗄️ Binds a renderer-neutral {@link Store} into React via `useSyncExternalStore`. */
export function useStore<TSnapshot>(store: Store<TSnapshot>): TSnapshot {
	return React.useSyncExternalStore(
		(listener) => store.subscribe(listener),
		() => store.getSnapshot(),
		() => store.getSnapshot(),
	);
}

/** @emoji 🎛 Resolves a controller-owned store by id and binds it with {@link useStore}. */
export function useControllerStore<TSnapshot>(controller: Controller | undefined, storeId: string): TSnapshot | undefined {
	const store = controller?.getStore<TSnapshot>(storeId);
	const subscribe = React.useCallback((listener: () => void) => store?.subscribe(listener) ?? (() => {}), [store]);
	const getSnapshot = React.useCallback(() => store?.getSnapshot(), [store]);
	return React.useSyncExternalStore(subscribe, getSnapshot, getSnapshot);
}

function virtualFileSystemSchemaFromModel(schema: VirtualFileSystemSchemaModel): VirtualFileSystemSchema {
	return schema as VirtualFileSystemSchema;
}

const BuiltinVirtualFileSystemKindRenderer: ComponentKindRenderer = ({ component, platform, commandBus }) => {
	const model = useStore(component as VirtualFileSystemSurface);
	const controllerId = component.controllerId;
	const hoverDispatchRowIdRef = React.useRef<string | null>(null);
	React.useEffect(() => {
		hoverDispatchRowIdRef.current = model.hoveredRowId ?? null;
	}, [model.hoveredRowId]);
	const dispatchRowHover = React.useCallback(
		(rowId: string | null) => {
			if (!platform) return;
			if (hoverDispatchRowIdRef.current === rowId) return;
			hoverDispatchRowIdRef.current = rowId;
			const vfs = component as VirtualFileSystemSurface;
			React.startTransition(() => {
				platform.commandBus.dispatch(controllerId, "virtualFileSystemRowHover", {
					appId: vfs.appId,
					rowId,
					surfaceId: component.surfaceId,
				});
			});
		},
		[component, controllerId, platform],
	);
	const schema = model.schema.fileNodeKinds && Object.keys(model.schema.fileNodeKinds).length
		? virtualFileSystemSchemaFromModel(model.schema)
		: virtualFileSystemSchemaFromModel(PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA);
	const rows: VirtualFileSystemRow[] = model.rows.map((row) => ({
		id: row.id,
		fileNodeKindId: row.fileNodeKindId,
		name: row.name,
		path: row.path,
		level: row.depth,
		hasChildren: row.hasChildren,
		isExpanded: row.expanded,
		parentId: undefined,
		...(row.icon ? { icon: row.icon } : {}),
		navigateUri: row.navigateUri,
		descriptorValues: row.descriptorValues,
	}));
	return (
		<div className="flex h-full min-h-0 w-full flex-col overflow-hidden p-2" data-component-kind="virtualFileSystem">
			<VirtualFileSystemView
				schema={schema}
				rows={rows}
				selectedRowIds={model.selectedRowIds ? new Set(model.selectedRowIds) : undefined}
				emptyMessage={model.emptyMessage ?? "No file system nodes"}
				rowClassName={(row) => (model.hoveredRowId === row.id ? "bg-hover-interactive-fill text-emphasized" : "")}
				onRowMouseEnter={(row) => {
					if (model.hoveredRowId === row.id) return;
					dispatchRowHover(row.id);
				}}
				onRowMouseLeave={(row) => {
					if (model.hoveredRowId !== row.id && hoverDispatchRowIdRef.current !== row.id) return;
					dispatchRowHover(null);
				}}
				onToggleExpand={(rowId) => {
					if (!platform) return;
					const vfs = component as VirtualFileSystemSurface;
					platform.commandBus.dispatch(controllerId, "toggleVirtualFileSystemExpand", {
						appId: vfs.appId,
						nodeId: rowId,
						surfaceId: component.surfaceId,
					});
				}}
				onSelectionChange={(selectedRowIds, { anchorRowId }) => {
					if (!platform) return;
					const vfs = component as VirtualFileSystemSurface;
					platform.commandBus.dispatch(controllerId, "setVirtualFileSystemRowSelection", {
						anchorRowId,
						appId: vfs.appId,
						rowIds: selectedRowIds,
						surfaceId: component.surfaceId,
					});
				}}
				onRowDoubleClick={(row) => {
					if (!row.navigateUri || !platform) return;
					if (platform.onNavigate) {
						platform.onNavigate(row.navigateUri);
						return;
					}
					platform.commandBus.dispatch(controllerId, "navigate", { path: row.navigateUri });
				}}
				dragDrop={
					model.dragDropEnabled
						? {
								enabled: true,
								pointerActivationDelayMs: 200,
								pointerActivationTolerancePx: 5,
								canDrag: (rowId) => {
									if (rowId === rows[0]?.id) return false;
									const modelRow = model.rows.find((entry) => entry.id === rowId);
									return modelRow?.canDrag !== false;
								},
								canDrop: (draggedId, targetId) => draggedId !== targetId,
								onDragEnd: ({ active, over }) => {
									if (!over) return;
									const vfs = component as VirtualFileSystemSurface;
									commandBus.dispatch(controllerId, "virtualFileSystemDragEnd", {
										appId: vfs.appId,
										active,
										over,
										surfaceId: component.surfaceId,
									});
								},
							}
						: undefined
				}
			/>
		</div>
	);
};

const BuiltinTableKindRenderer: ComponentKindRenderer = ({ component, platform }) => {
	const model = useStore(component as Table);
	const controllerId = component.controllerId;
	return (
		<div className="flex h-full min-h-0 w-full flex-col overflow-auto p-2" data-component-kind="table">
			{model.columns.length === 0 && model.rows.length === 0 ? (
				<div className="text-xs text-muted-foreground">{model.emptyMessage ?? "No rows"}</div>
			) : (
				<table className="w-full border-collapse text-xs">
					<thead>
						<tr>
							{model.columns.map((column) => {
								const active = model.sortColumnId === column.id;
								const indicator = active ? (model.sortDescending ? " ▾" : " ▴") : "";
								return (
									<th
										key={column.id}
										className={cn(
											"border-b px-2 py-1 text-left font-medium",
											column.sortable ? "cursor-pointer select-none hover:bg-muted/40" : undefined,
											active ? "text-emphasized" : "text-muted-foreground",
										)}
										onClick={
											column.sortable && platform
												? () =>
														platform.commandBus.dispatch(controllerId, "cycleTableSort", {
															columnId: column.id,
															surfaceId: component.surfaceId,
														})
												: undefined
										}
									>
										{column.label}
										{indicator}
									</th>
								);
							})}
						</tr>
					</thead>
					<tbody>
						{model.rows.map((row) => (
							<tr
								key={row.id}
								className={cn(
									model.selectedRowIds?.includes(row.id) ? "bg-muted/50" : undefined,
									row.navigateUri || row.expandToggle ? "cursor-pointer hover:bg-muted/40" : undefined,
								)}
								onClick={(event) => {
									if (row.expandToggle && platform && (event.target as HTMLElement).closest("[data-table-expand]")) {
										platform.commandBus.dispatch(controllerId, row.expandToggle.command, row.expandToggle.args);
										return;
									}
									if (!row.navigateUri || !platform?.onNavigate) return;
									if (event.metaKey || event.ctrlKey) {
										platform.commandBus.dispatch(controllerId, "toggleTableRowSelection", { rowId: row.id });
										return;
									}
									platform.onNavigate(row.navigateUri);
								}}
							>
								{model.columns.map((column, columnIndex) => (
									<td
										key={`${row.id}:${column.id}`}
										className="border-b px-2 py-1"
										style={columnIndex === 0 && row.depth ? { paddingLeft: 8 + row.depth * 14 } : undefined}
									>
										{columnIndex === 0 && row.hasChildren ? (
											<span className="inline-flex items-center gap-1">
												<button
													type="button"
													data-table-expand
													className="inline-flex size-4 shrink-0 items-center justify-center rounded hover:bg-muted"
													aria-label={row.expanded ? "Collapse" : "Expand"}
													onClick={(event) => {
														event.stopPropagation();
														if (!row.expandToggle || !platform) return;
														platform.commandBus.dispatch(controllerId, row.expandToggle.command, row.expandToggle.args);
													}}
												>
													{row.expanded ? "▾" : "▸"}
												</button>
												<span>{String(row.cells[column.id] ?? "")}</span>
											</span>
										) : (
											String(row.cells[column.id] ?? "")
										)}
									</td>
								))}
							</tr>
						))}
					</tbody>
				</table>
			)}
		</div>
	);
};

const BuiltinPuzzle2dKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Puzzle2d);
	if (model.nodes.length === 0 && model.edges.length === 0) {
		return (
			<div className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground" data-surface-id={node.surfaceId}>
				{model.emptyMessage ?? "Empty puzzle 2d"}
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId}>
			<Puzzle2dCanvas className="h-full w-full" />
		</div>
	);
};

const BuiltinPuzzle3dKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Puzzle3d);
	return (
		<div className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground" data-surface-id={node.surfaceId}>
			{model.emptyMessage ?? `3D scene${model.instanceId ? ` · ${model.instanceId}` : ""}`}
		</div>
	);
};

/** @emoji 🔑 Stable topology identity ignoring flat node positions (live force updates positions locally). */
export function platformTopologyStructureKey(flat: Record<string, unknown>, volume: Record<string, unknown>): string {
	const parsed = parsePuzzle2dFixtureV1(flat);
	if (!parsed) return "";
	const nodes = [...parsed.nodes]
		.map((node) => node.id)
		.sort()
		.join(",");
	const edges = [...parsed.edges]
		.map((edge) => `${edge.id}:${edge.source}:${edge.target}`)
		.sort()
		.join(";");
	return `${nodes}|${edges}|${JSON.stringify(parsed.camera)}|${JSON.stringify(volume)}`;
}

function usePlatformTopologyStore(
	controller: Controller | undefined,
	instanceId: string,
): ReturnType<typeof createStore> | null {
	const payload = useControllerStore<PlatformTopologyPayload>(controller, platformTopologyStoreId(instanceId));
	const topologyStoreRef = React.useRef<ReturnType<typeof createStore> | null>(null);
	const lastStructureKeyRef = React.useRef<string | null>(null);
	const flatPayloadRef = React.useRef(payload?.flat);
	const volumePayloadRef = React.useRef(payload?.volume);
	flatPayloadRef.current = payload?.flat;
	volumePayloadRef.current = payload?.volume;
	const structureKey =
		flatPayloadRef.current && volumePayloadRef.current
			? platformTopologyStructureKey(flatPayloadRef.current, volumePayloadRef.current)
			: null;
	const [, setTopologyEpoch] = React.useState(0);
	React.useEffect(() => {
		if (!structureKey) {
			if (topologyStoreRef.current !== null) {
				topologyStoreRef.current = null;
				lastStructureKeyRef.current = null;
				setTopologyEpoch((epoch) => epoch + 1);
			}
			return;
		}
		const flatPayload = flatPayloadRef.current;
		const volumePayload = volumePayloadRef.current;
		if (!flatPayload || !volumePayload) {
			return;
		}
		const model = prepareTopologyModel(compose5d(parsePuzzle2dFixtureV1(flatPayload)!, parseFixtureV1(volumePayload)!));
		const existing = topologyStoreRef.current;
		if (existing) {
			if (lastStructureKeyRef.current !== structureKey) {
				existing.replaceModel(model);
				lastStructureKeyRef.current = structureKey;
			}
			return;
		}
		topologyStoreRef.current = createStore(model);
		lastStructureKeyRef.current = structureKey;
		setTopologyEpoch((epoch) => epoch + 1);
	}, [structureKey]);
	return topologyStoreRef.current;
}

/** @emoji 🎯 Maps FiveD flat/volume selection to `puzzle5dSelection` command payload. */
export function puzzle5dSelectionPayload(
	instanceId: string,
	presentation: Puzzle5dModel["presentation"],
	snapshot: Puzzle2dSelectionSnapshot | Puzzle3dSelectionSnapshot,
): { readonly instanceId: string; readonly puzzle2dIds: readonly string[] } {
	if (presentation === "flat") {
		return { instanceId, puzzle2dIds: (snapshot as Puzzle2dSelectionSnapshot).ids };
	}
	const volume = snapshot as Puzzle3dSelectionSnapshot;
	return { instanceId, puzzle2dIds: [...volume.objectIds, ...volume.vortexIds, ...volume.attractionIds] };
}

const BuiltinPuzzle5dKindRenderer: ComponentKindRenderer = ({ component, node, commandBus, platform }) => {
	const model = useStore(component as Puzzle5d);
	const instanceId = model.instanceId || node.surfaceId;
	const controller = platform ? getPlatformControllerById(platform, component.controllerId) : undefined;
	const topologyStore = usePlatformTopologyStore(controller, instanceId);
	const puzzle2dSelect = React.useMemo(
		() =>
			model.presentation === "flat"
				? {
						...(model.puzzle2dSelection !== undefined ? { selection: { ids: [...model.puzzle2dSelection] } } : {}),
						...(model.puzzle2dHoveredId !== undefined ? { hoveredId: model.puzzle2dHoveredId } : {}),
						onSelect: (snapshot: Puzzle2dSelectionSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dSelection", puzzle5dSelectionPayload(instanceId, "flat", snapshot));
						},
						...(instanceId.endsWith(":kit:wires")
							? {
									onActivate: (snapshot: Puzzle2dSelectionSnapshot) => {
										commandBus.dispatch(component.controllerId, "puzzle5dActivate", {
											instanceId,
											puzzle2dIds: snapshot.ids,
										});
									},
								}
							: {}),
						onHover: (payload: { readonly id: string | null }) => {
							commandBus.dispatch(component.controllerId, "puzzle5dHover", { instanceId, nodeId: payload.id });
						},
						onPreselect: (snapshot: Puzzle2dPreselectSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dPreselect", {
								instanceId,
								preselect: { ids: [...snapshot.ids], removedIds: [...snapshot.removedIds] },
							});
						},
					}
				: undefined,
		[commandBus, component.controllerId, instanceId, model.presentation, model.puzzle2dHoveredId, model.puzzle2dSelection],
	);
	const puzzle3dSelect = React.useMemo(
		() =>
			model.presentation === "volume"
				? {
						onSelect: (snapshot: Puzzle3dSelectionSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dSelection", puzzle5dSelectionPayload(instanceId, "volume", snapshot));
						},
					}
				: undefined,
		[commandBus, component.controllerId, instanceId, model.presentation],
	);
	const fiveDMode = model.presentation === "volume" ? "3d" : "2d";
	if (model.emptyMessage) {
		return (
			<div
				className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
				data-surface-id={node.surfaceId}
			>
				{model.emptyMessage}
			</div>
		);
	}
	if (!topologyStore) {
		return (
			<div
				className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
				data-surface-id={node.surfaceId}
			>
				Topology loading…
			</div>
		);
	}
	return (
		<div
			className="absolute inset-0 min-h-0 min-w-0"
			data-surface-id={node.surfaceId}
			data-testid={`platform-five-d-${instanceId}`}
		>
			<StoreProvider store={topologyStore}>
				<FiveD
					instanceId={instanceId}
					graphPortMode={instanceId.endsWith(":kit:wires") || instanceId.endsWith(":diagram") ? "normal" : undefined}
					liveForceGraph={instanceId.endsWith(":kit:wires")}
					mode={fiveDMode}
					puzzle2d={puzzle2dSelect}
					puzzle3d={puzzle3dSelect}
				/>
			</StoreProvider>
		</div>
	);
};

const BuiltinCadKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Cad);
	return (
		<div className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground" data-surface-id={node.surfaceId}>
			{model.emptyMessage ?? `CAD${model.instanceId ? ` · ${model.instanceId}` : ""}`}
		</div>
	);
};

const BuiltinPanelKindRenderer: ComponentKindRenderer = ({ component, commandBus }) => {
	const model = useStore(component as Panel);
	return <UiRenderer node={model.body} commandBus={commandBus} />;
};

function ensureBuiltinComponentKindRenderers(): void {
	if (componentKindRenderers.size > 0) return;
	registerComponentKindRenderer("table", BuiltinTableKindRenderer);
	registerComponentKindRenderer("virtualFileSystem", BuiltinVirtualFileSystemKindRenderer);
	registerComponentKindRenderer("puzzle2d", BuiltinPuzzle2dKindRenderer);
	registerComponentKindRenderer("puzzle3d", BuiltinPuzzle3dKindRenderer);
	registerComponentKindRenderer("puzzle5d", BuiltinPuzzle5dKindRenderer);
	registerComponentKindRenderer("cad", BuiltinCadKindRenderer);
	registerComponentKindRenderer("panel", BuiltinPanelKindRenderer);
}

ensureBuiltinComponentKindRenderers();
//#endregion 🔖ComponentKindRenderer

//#region 🔖CodeEditor
export interface CodeEditorToken {
	readonly class: string;
	readonly start: number;
	readonly end: number;
}

export interface CodeEditorCompletion {
	readonly label: string;
	readonly kind: string;
	readonly detail?: string;
	readonly insert: string;
}

export interface CodeEditorProps {
	readonly value: string;
	readonly onChange: (value: string) => void;
	readonly onSubmit?: () => void;
	readonly tokenize: (text: string) => readonly CodeEditorToken[];
	readonly complete: (text: string, cursor: number) => readonly CodeEditorCompletion[];
	readonly className?: string;
	readonly placeholder?: string;
}

/** @emoji 🎨 Maps editor token classes to semantic surface colors. */
export function codeEditorTokenClassName(tokenClass: string): string {
	switch (tokenClass) {
		case "keyword":
			return "text-accent";
		case "string":
			return "text-emphasized";
		case "number":
			return "text-accent";
		case "operator":
			return "text-muted-foreground";
		case "punctuation":
			return "text-muted-foreground";
		case "error":
			return "text-destructive";
		default:
			return "text-foreground";
	}
}

function escapeHtml(text: string): string {
	return text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

function renderHighlightedMarkup(value: string, tokens: readonly CodeEditorToken[]): string {
	if (!value) return "\n";
	let html = "";
	let cursor = 0;
	for (const token of tokens) {
		if (token.start > cursor) {
			html += escapeHtml(value.slice(cursor, token.start));
		}
		html += `<span class="${codeEditorTokenClassName(token.class)}">${escapeHtml(value.slice(token.start, token.end))}</span>`;
		cursor = token.end;
	}
	if (cursor < value.length) {
		html += escapeHtml(value.slice(cursor));
	}
	return `${html}\n`;
}

function caretOffsetCoordinates(textarea: HTMLTextAreaElement, position: number): { top: number; left: number } {
	const mirror = document.createElement("div");
	const style = window.getComputedStyle(textarea);
	const properties = [
		"boxSizing",
		"width",
		"height",
		"overflowX",
		"overflowY",
		"borderTopWidth",
		"borderRightWidth",
		"borderBottomWidth",
		"borderLeftWidth",
		"paddingTop",
		"paddingRight",
		"paddingBottom",
		"paddingLeft",
		"fontStyle",
		"fontVariant",
		"fontWeight",
		"fontStretch",
		"fontSize",
		"fontFamily",
		"lineHeight",
		"letterSpacing",
		"textTransform",
		"textIndent",
		"whiteSpace",
		"wordBreak",
		"wordWrap",
	] as const;
	for (const key of properties) {
		mirror.style.setProperty(key, style.getPropertyValue(key.replace(/[A-Z]/g, (m) => `-${m.toLowerCase()}`)));
	}
	mirror.style.position = "absolute";
	mirror.style.visibility = "hidden";
	mirror.style.whiteSpace = "pre-wrap";
	mirror.style.wordWrap = "break-word";
	mirror.style.top = "0";
	mirror.style.left = "0";
	mirror.textContent = textarea.value.slice(0, position);
	const marker = document.createElement("span");
	marker.textContent = textarea.value.slice(position) || ".";
	mirror.appendChild(marker);
	document.body.appendChild(mirror);
	const top = marker.offsetTop - textarea.scrollTop;
	const left = marker.offsetLeft - textarea.scrollLeft;
	document.body.removeChild(mirror);
	return { top, left };
}

/** @emoji ✍️ Handcrafted overlay code editor with syntax highlighting and autocomplete. */
export function CodeEditor({ value, onChange, onSubmit, tokenize, complete, className, placeholder }: CodeEditorProps): React.ReactElement {
	const textareaRef = React.useRef<HTMLTextAreaElement>(null);
	const preRef = React.useRef<HTMLPreElement>(null);
	const [suggestions, setSuggestions] = React.useState<readonly CodeEditorCompletion[]>([]);
	const [activeSuggestion, setActiveSuggestion] = React.useState(0);
	const [popup, setPopup] = React.useState<{ top: number; left: number } | null>(null);
	const tokens = React.useMemo(() => tokenize(value), [tokenize, value]);
	const lineCount = Math.max(1, value.split("\n").length);

	const syncScroll = React.useCallback(() => {
		const textarea = textareaRef.current;
		const pre = preRef.current;
		if (!textarea || !pre) return;
		pre.scrollTop = textarea.scrollTop;
		pre.scrollLeft = textarea.scrollLeft;
	}, []);

	const refreshSuggestions = React.useCallback(
		(nextValue: string, nextCursor: number) => {
			const items = complete(nextValue, nextCursor);
			setSuggestions(items);
			setActiveSuggestion(0);
			const textarea = textareaRef.current;
			if (!textarea || items.length === 0) {
				setPopup(null);
				return;
			}
			const coords = caretOffsetCoordinates(textarea, nextCursor);
			setPopup({ top: coords.top + 20, left: coords.left });
		},
		[complete],
	);

	const applySuggestion = React.useCallback(
		(item: CodeEditorCompletion) => {
			const textarea = textareaRef.current;
			if (!textarea) return;
			const start = textarea.selectionStart;
			let scan = start;
			while (scan > 0) {
				const ch = value[scan - 1];
				if (/[A-Za-z0-9_]/.test(ch)) scan -= 1;
				else break;
			}
			const next = `${value.slice(0, scan)}${item.insert}${value.slice(start)}`;
			onChange(next);
			const nextCursor = scan + item.insert.length;
			window.requestAnimationFrame(() => {
				textarea.focus();
				textarea.setSelectionRange(nextCursor, nextCursor);
				refreshSuggestions(next, nextCursor);
			});
			setSuggestions([]);
			setPopup(null);
		},
		[onChange, refreshSuggestions, value],
	);

	const onKeyDown = React.useCallback(
		(event: React.KeyboardEvent<HTMLTextAreaElement>) => {
			if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
				event.preventDefault();
				setSuggestions([]);
				setPopup(null);
				onSubmit?.();
				return;
			}
			if (suggestions.length > 0) {
				if (event.key === "ArrowDown") {
					event.preventDefault();
					setActiveSuggestion((row) => (row + 1) % suggestions.length);
					return;
				}
				if (event.key === "ArrowUp") {
					event.preventDefault();
					setActiveSuggestion((row) => (row - 1 + suggestions.length) % suggestions.length);
					return;
				}
				if (event.key === "Enter" || event.key === "Tab") {
					event.preventDefault();
					applySuggestion(suggestions[activeSuggestion]!);
					return;
				}
				if (event.key === "Escape") {
					event.preventDefault();
					setSuggestions([]);
					setPopup(null);
					return;
				}
			}
			if (event.key === "Tab" && !event.shiftKey) {
				event.preventDefault();
				const textarea = event.currentTarget;
				const start = textarea.selectionStart;
				const end = textarea.selectionEnd;
				const next = `${value.slice(0, start)}  ${value.slice(end)}`;
				onChange(next);
				window.requestAnimationFrame(() => {
					textarea.setSelectionRange(start + 2, start + 2);
				});
			}
		},
		[activeSuggestion, applySuggestion, onChange, onSubmit, suggestions, value],
	);

	return (
		<div className={cn("relative flex h-full min-h-0 w-full min-w-0 bg-canvas font-mono text-xs", className)} data-code-editor>
			<div className="select-none border-r border-border px-2 py-2 text-right text-muted-foreground tabular-nums">
				{Array.from({ length: lineCount }, (_, index) => (
					<div key={index}>{index + 1}</div>
				))}
			</div>
			<div className="relative min-h-0 min-w-0 flex-1">
				<pre
					ref={preRef}
					className="pointer-events-none absolute inset-0 m-0 overflow-hidden whitespace-pre-wrap break-words p-2"
					aria-hidden
					dangerouslySetInnerHTML={{ __html: renderHighlightedMarkup(value, tokens) }}
				/>
				<textarea
					ref={textareaRef}
					className="absolute inset-0 m-0 resize-none overflow-auto bg-transparent p-2 text-transparent caret-foreground outline-none"
					value={value}
					placeholder={placeholder}
					spellCheck={false}
					onChange={(event) => {
						onChange(event.target.value);
						const nextCursor = event.target.selectionStart ?? event.target.value.length;
						refreshSuggestions(event.target.value, nextCursor);
					}}
					onKeyDown={onKeyDown}
					onScroll={syncScroll}
					onClick={(event) => {
						const nextCursor = event.currentTarget.selectionStart ?? value.length;
						refreshSuggestions(value, nextCursor);
					}}
					onKeyUp={(event) => {
						const nextCursor = event.currentTarget.selectionStart ?? value.length;
						refreshSuggestions(value, nextCursor);
					}}
				/>
				{popup && suggestions.length > 0 ? (
					<div
						className="absolute z-20 max-h-48 min-w-40 overflow-auto rounded-md border border-border bg-popover shadow-md"
						style={{ top: popup.top, left: popup.left }}
					>
						{suggestions.map((item, index) => (
							<button
								key={`${item.label}-${index}`}
								type="button"
								className={cn(
									"flex w-full flex-col items-start px-2 py-1 text-left text-xs",
									index === activeSuggestion ? "bg-accent text-accent-foreground" : "text-foreground hover:bg-muted",
								)}
								onMouseDown={(event) => {
									event.preventDefault();
									applySuggestion(item);
								}}
							>
								<span>{item.label}</span>
								{item.detail ? <span className="text-muted-foreground">{item.detail}</span> : null}
							</button>
						))}
					</div>
				) : null}
			</div>
		</div>
	);
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("codeEditorTokenClassName", () => {
		it("maps keyword tokens", () => {
			expect(codeEditorTokenClassName("keyword")).toBe("text-accent");
		});
	});

	describe("renderHighlightedMarkup", () => {
		it("wraps token spans", () => {
			const html = renderHighlightedMarkup("MATCH", [{ class: "keyword", start: 0, end: 5 }]);
			expect(html).toContain('class="text-accent"');
			expect(html).toContain("MATCH");
		});
	});
}
//#endregion 🔖CodeEditor

//#region 🔖SurfaceBinding
type SurfaceBindingHost = React.ComponentType<{ readonly node: UiComponentHostSurfaceNode; readonly platform?: Platform }>;

const surfaceBindingHosts = new Map<string, SurfaceBindingHost>();

/** @emoji 🔗 Binds `surfaceId` to a host React implementation for any {@link ComponentKind} surface node. */
export function registerSurfaceBinding(surfaceId: string, Component: SurfaceBindingHost): void {
	surfaceBindingHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a surface binding (tests / hot reload). */
export function unregisterSurfaceBinding(surfaceId: string): void {
	surfaceBindingHosts.delete(surfaceId);
}

const PlatformComponentPlaceholder: React.FC<{ readonly kind: ComponentKind; readonly surfaceId: string }> = ({ kind, surfaceId }) => (
	<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
		Register a surface binding for <span className="mx-1 font-mono">{kind}</span> · <span className="font-mono">{surfaceId}</span>
	</div>
);

const BuiltinPuzzle2dCanvas: React.FC<{ readonly node: UiPuzzle2dHostSurfaceNode }> = ({ node }) => (
	<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId}>
		<Puzzle2dCanvas className="h-full w-full" />
	</div>
);

const BuiltinPuzzle5dCanvas: React.FC<{ readonly node: UiPuzzle5dHostSurfaceNode; readonly platform?: Platform }> = ({
	node,
	platform,
}) => {
	if (platform) {
		const registered = platform.getComponent(node.surfaceId);
		if (registered?.componentKind === "puzzle5d") {
			const KindRenderer = componentKindRenderers.get("puzzle5d");
			if (KindRenderer) {
				return (
					<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId}>
						<KindRenderer
							component={registered as Component<unknown>}
							node={node}
							commandBus={platform.commandBus}
							layout="canvas"
							platform={platform}
						/>
					</div>
				);
			}
		}
	}
	return (
		<div
			className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
			data-surface-id={node.surfaceId}
		>
			Loading…
		</div>
	);
};

const defaultComponentHosts: Partial<Record<ComponentKind, SurfaceBindingHost>> = {
	puzzle2d: BuiltinPuzzle2dCanvas as SurfaceBindingHost,
	puzzle5d: BuiltinPuzzle5dCanvas as SurfaceBindingHost,
};

function renderBoundComponent(
	node: UiComponentHostSurfaceNode,
	layout: "canvas" | "panel",
	platform?: Platform,
	commandBus?: CommandBus,
): React.ReactElement {
	const wrapperClass =
		layout === "canvas" ? "absolute inset-0 min-h-0 min-w-0" : "relative min-h-0 min-w-0 flex-1 overflow-auto";
	const ExplicitHost = surfaceBindingHosts.get(node.surfaceId);
	if (ExplicitHost) {
		return (
			<div className={wrapperClass}>
				<ExplicitHost node={node} platform={platform} />
			</div>
		);
	}
	if (platform) {
		const registered = platform.getComponent(node.surfaceId);
		if (registered && registered.componentKind === node.componentKind) {
			const KindRenderer = componentKindRenderers.get(node.componentKind);
			if (KindRenderer) {
				return (
					<div className={wrapperClass}>
						<KindRenderer
							component={registered as Component<unknown>}
							node={node}
							commandBus={commandBus ?? platform.commandBus}
							layout={layout}
							platform={platform}
						/>
					</div>
				);
			}
		}
	}
	const Host = defaultComponentHosts[node.componentKind];
	if (!Host) {
		return (
			<div className={wrapperClass}>
				<PlatformComponentPlaceholder kind={node.componentKind} surfaceId={node.surfaceId} />
			</div>
		);
	}
	return (
		<div className={wrapperClass}>
			<Host node={node} platform={platform} />
		</div>
	);
}

/** @emoji 🖼 Renders a {@link UiComponentHostSurfaceNode} using {@link registerSurfaceBinding} (shared with playground shell). */
export function renderComponentHostSurface(
	node: UiComponentHostSurfaceNode,
	layout: "canvas" | "panel" = "canvas",
	platform?: Platform,
): React.ReactElement {
	return renderBoundComponent(node, layout, platform, platform?.commandBus);
}

/** @emoji 📑 Binds a table `surfaceId` (alias for {@link registerSurfaceBinding}). */
export function registerUiTableSurfaceHost(surfaceId: string, Component: React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>): void {
	registerSurfaceBinding(surfaceId, Component as SurfaceBindingHost);
}

export function unregisterUiTableSurfaceHost(surfaceId: string): void {
	unregisterSurfaceBinding(surfaceId);
}

/** @emoji ✍️ Binds an editor `surfaceId` (alias for {@link registerSurfaceBinding}). */
export function registerUiEditorSurfaceHost(surfaceId: string, Component: React.ComponentType<{ readonly node: UiEditorHostSurfaceNode }>): void {
	registerSurfaceBinding(surfaceId, Component as SurfaceBindingHost);
}

export function unregisterUiEditorSurfaceHost(surfaceId: string): void {
	unregisterSurfaceBinding(surfaceId);
}

/** @emoji 📁 Binds a virtual file system `surfaceId` (alias for {@link registerSurfaceBinding}). */
export function registerUiVirtualFileSystemSurfaceHost(
	surfaceId: string,
	Component: React.ComponentType<{ readonly node: UiVirtualFileSystemHostSurfaceNode }>,
): void {
	registerSurfaceBinding(surfaceId, Component as SurfaceBindingHost);
}

export function unregisterUiVirtualFileSystemSurfaceHost(surfaceId: string): void {
	unregisterSurfaceBinding(surfaceId);
}

/** @emoji 🧩 Binds a panel `surfaceId` (alias for {@link registerSurfaceBinding}). */
export function registerUiPanelSurfaceHost(surfaceId: string, Component: React.ComponentType<{ readonly node: UiPanelHostSurfaceNode }>): void {
	registerSurfaceBinding(surfaceId, Component as SurfaceBindingHost);
}

export function unregisterUiPanelSurfaceHost(surfaceId: string): void {
	unregisterSurfaceBinding(surfaceId);
}
//#endregion 🔖SurfaceBinding

//#region ­ƒöûStackLayout
function stackClass(spec: UiStackNode): string {
	const dir = spec.direction === "horizontal" ? "flex-row" : "flex-col";
	const gap =
		spec.gap === "none"
			? "gap-0"
			: spec.gap === "tight"
				? "gap-1"
				: spec.gap === "relaxed"
					? "gap-4"
					: "gap-2";
	const pad = spec.padding === "none" ? "p-0" : "p-2";
	return cn("flex", dir, gap, pad, spec.direction === "vertical" ? "min-h-0 min-w-0" : "min-w-0");
}
//#endregion ­ƒöûStackLayout

//#region ­ƒöûRenderer
export interface UiRendererProps {
	readonly node: UiNode;
	readonly commandBus: CommandBus;
	readonly platform?: Platform;
}

function renderText(node: UiTextNode): React.ReactElement {
	const dataProps = node.dataAttributes
		? Object.fromEntries(Object.entries(node.dataAttributes).map(([k, v]) => [`data-${k}`, v]))
		: {};
	return (
		<span
			className={cn(
				"text-muted-foreground px-1 text-xs",
				node.emphasize && "font-semibold uppercase tracking-wide",
			)}
			{...dataProps}
		>
			{node.value}
		</span>
	);
}

function renderButton(node: UiButtonNode, commandBus: CommandBus): React.ReactElement {
	const variant = node.style?.variant ?? "default";
	if (variant === "default") {
		return (
			<Button type="button" id={node.id} variant="outline" size="sm" onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}>
				{node.label}
			</Button>
		);
	}
	return (
		<button
			type="button"
			id={node.id}
			className={cn(
				"rounded-md border px-2 py-1 text-sm",
				variant === "danger" && "border-destructive text-destructive",
				variant === "success" && "border-success text-success",
				variant === "subtle" && "border-transparent bg-muted/60",
			)}
			onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}
		>
			{node.label}
		</button>
	);
}

function renderSeparator(_node: UiSeparatorNode, horizontalParent: boolean): React.ReactElement {
	return (
		<span
			role="separator"
			className={cn(
				"shrink-0 bg-border",
				horizontalParent ? "mx-1 h-4 w-px self-center" : "my-1 h-px w-full",
			)}
			aria-hidden
		/>
	);
}

function renderTable(node: UiTableHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "panel", platform, commandBus);
}

function renderEditor(node: UiEditorHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "panel", platform, commandBus);
}

function renderVirtualFileSystem(
	node: UiVirtualFileSystemHostSurfaceNode,
	platform: Platform | undefined,
	commandBus: CommandBus,
): React.ReactElement {
	return renderBoundComponent(node, "panel", platform, commandBus);
}

function renderPanel(node: UiPanelHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "panel", platform, commandBus);
}

function renderPuzzle2d(node: UiPuzzle2dHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "canvas", platform, commandBus);
}

function renderPuzzle3d(node: UiPuzzle3dHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "canvas", platform, commandBus);
}

function renderPuzzle5d(node: UiPuzzle5dHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "canvas", platform, commandBus);
}

function renderCad(node: UiCadHostSurfaceNode, platform: Platform | undefined, commandBus: CommandBus): React.ReactElement {
	return renderBoundComponent(node, "canvas", platform, commandBus);
}

function renderNode(node: UiNode, commandBus: CommandBus, horizontalParent: boolean, platform?: Platform): React.ReactElement {
	switch (node.type) {
		case "stack":
			return (
				<div
					className={cn(
						stackClass(node),
						node.direction === "vertical" &&
							node.children.some((c) => c.type === "puzzle2d" || c.type === "puzzle3d" || c.type === "puzzle5d" || c.type === "cad" || c.type === "gismap") &&
							"relative min-h-0 flex-1",
					)}
				>
					{node.children.map((child, index) => (
						<React.Fragment key={index}>{renderNode(child, commandBus, node.direction === "horizontal", platform)}</React.Fragment>
					))}
				</div>
			);
		case "text":
			return renderText(node);
		case "button":
			return renderButton(node, commandBus);
		case "separator":
			return renderSeparator(node, horizontalParent);
		case "table":
			return renderTable(node, platform, commandBus);
		case "editor":
			return renderEditor(node, platform, commandBus);
		case "virtualFileSystem":
			return renderVirtualFileSystem(node, platform, commandBus);
		case "panel":
			return renderPanel(node, platform, commandBus);
		case "puzzle2d":
			return renderPuzzle2d(node, platform, commandBus);
		case "puzzle3d":
			return renderPuzzle3d(node, platform, commandBus);
		case "puzzle5d":
			return renderPuzzle5d(node, platform, commandBus);
		case "cad":
			return renderCad(node, platform, commandBus);
		case "gismap":
			return renderBoundComponent(node, "canvas", platform, commandBus);
		case "forms":
		case "raster":
		case "writer":
		case "flow":
		case "dag":
		case "trinity":
		case "shooting":
			return renderBoundComponent(node, "canvas", platform, commandBus);
		default:
			return (
				<div className="p-2 text-xs text-destructive">
					Unsupported UiNode {(node as { type?: string }).type ?? "unknown"}
				</div>
			);
	}
}

/** @emoji ­ƒº® Host entry: turns declarative {@link UiNode} trees into mounted React structure. */
export function UiRenderer({ node, commandBus, platform }: UiRendererProps): React.ReactElement {
	return renderNode(node, commandBus, false, platform);
}
//#endregion ­ƒöûRenderer

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("platformTopologyStructureKey", () => {
		it("ignores node position changes", () => {
			const flatA = {
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "a", x: 0, y: 0, shape: "circle", radius: 8, handles: [] }],
				edges: [],
			};
			const flatB = {
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "a", x: 40, y: 50, shape: "circle", radius: 8, handles: [] }],
				edges: [],
			};
			const volume = { schema: "puzzle.3d.fixture/v1", objects: [], attractions: [], cables: [] };
			expect(platformTopologyStructureKey(flatA, volume)).toBe(platformTopologyStructureKey(flatB, volume));
		});

		it("changes when node ids change", () => {
			const volume = { schema: "puzzle.3d.fixture/v1", objects: [], attractions: [], cables: [] };
			const flatA = {
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "a", x: 0, y: 0, shape: "circle", radius: 8, handles: [] }],
				edges: [],
			};
			const flatB = {
				schema: "puzzle.2d.fixture/v1",
				camera: { x: 0, y: 0, zoom: 1 },
				nodes: [{ id: "b", x: 0, y: 0, shape: "circle", radius: 8, handles: [] }],
				edges: [],
			};
			expect(platformTopologyStructureKey(flatA, volume)).not.toBe(platformTopologyStructureKey(flatB, volume));
		});
	});

	describe("ShellModeCanvas", () => {
		it("resolveShellModeWindowBody reuses the same body node when only engagement chrome changes", () => {
			const cache = new Map<string, ShellModeWindowBodyCacheEntry>();
			const HostA: React.FC = () => <div data-testid="host-a" />;
			const HostB: React.FC = () => <div data-testid="host-b" />;
			const first = resolveShellModeWindowBody(cache, {
				id: "shape",
				label: "Shape",
				component: HostA,
				engagement: { input: { id: "in", value: "" } },
			});
			const second = resolveShellModeWindowBody(cache, {
				id: "shape",
				label: "Shape",
				component: HostA,
				engagement: { input: { id: "in", value: "box" } },
			});
			const third = resolveShellModeWindowBody(cache, {
				id: "shape",
				label: "Shape",
				component: HostB,
				engagement: { input: { id: "in", value: "box" } },
			});
			expect(second).toBe(first);
			expect(third).not.toBe(first);
		});

		it("resolveShellModeWindowBody isolates bodies per shell instance id", () => {
			const cache = new Map<string, ShellModeWindowBodyCacheEntry>();
			const Host: React.FC = () => <div data-testid="viewport-host" />;
			const overviewA = resolveShellModeWindowBody(cache, {
				id: "2d-overview",
				windowKindId: "2d-overview",
				label: "Overview",
				component: Host,
			});
			const overviewB = resolveShellModeWindowBody(cache, {
				id: "win-2d-overview-extra",
				windowKindId: "2d-overview",
				label: "Overview copy",
				component: Host,
			});
			expect(overviewA).not.toBe(overviewB);
		});

		it("removeWindowFromLayout and collapseLayout produce an empty stack for the last closed window", () => {
			const layout: ShellWindowLayoutNode = {
				kind: "stack",
				children: [{ kind: "window", id: "gis-map-main" }],
			};
			const next = collapseLayout(removeWindowFromLayout(layout, "gis-map-main")) ?? { kind: "stack", children: [] };
			expect(next).toEqual({ kind: "stack", children: [] });
		});
	});

	describe("display windows tree", () => {
		it("uses pointer palette drag for window-template rows", () => {
			const host: DisplayHostApi = {
				windowKinds: [{ id: "puzzle-3d-main", label: "Puzzle 3D", bodyKey: "puzzle.3d.play.main", templates: [] } as WindowKindRuntime],
				namedLayouts: [],
				userLayouts: [],
				saveCurrentLayout: () => {},
				applyNamedLayout: () => {},
				deleteUserLayout: () => {},
			};
			const tree = new DisplayWindowsTreeDefinition(() => host).resolveTree();
			expect(tree.dragAndDropController?.pointerPaletteDrag).toBeTruthy();
		});

		it("lists a draggable kind row and nested template children", () => {
			const host: DisplayHostApi = {
				windowKinds: [
					{
						id: "cad-play-shape",
						label: "Shape",
						bodyKey: "cad.play.shape",
						templates: [
							{
								id: "orthographic",
								label: "Orthographic",
								controllerId: "cad",
								command: "setView",
								args: {},
								children: [{ id: "top", label: "Top", controllerId: "cad", command: "setView", args: {} }],
							},
						],
					} as WindowKindRuntime,
				],
				namedLayouts: [],
				userLayouts: [],
				saveCurrentLayout: () => {},
				applyNamedLayout: () => {},
				deleteUserLayout: () => {},
			};
			const tree = new DisplayWindowsTreeDefinition(() => host).resolveTree();
			const items = tree.sections[0]?.items ?? [];
			expect(items).toHaveLength(2);
			const kindRow = JSON.parse(items[0]!.dragData![COMPOSE_WINDOW_TEMPLATE_MIME]!) as WindowTemplateDropPayload;
			expect(kindRow.windowKindId).toBe("cad-play-shape");
			expect(kindRow.templateId).toBeUndefined();
			const templateRow = JSON.parse(items[1]!.dragData![COMPOSE_WINDOW_TEMPLATE_MIME]!) as WindowTemplateDropPayload;
			expect(templateRow.templateId).toBe("orthographic");
			expect(items[1]?.defaultOpen).toBe(false);
			expect(items[1]?.items?.[0]?.label).toBe("Top");
		});

		it("groups builtin layouts by groupPath", () => {
			const host: DisplayHostApi = {
				windowKinds: [],
				namedLayouts: [createNamedLayout("view-quad-standard", "Standard", { root: { kind: "stack", children: [] } }, "builtin", undefined, ["Quad", "Mixed"])],
				userLayouts: [],
				saveCurrentLayout: () => {},
				applyNamedLayout: () => {},
				deleteUserLayout: () => {},
			};
			const tree = new DisplayLayoutTreeDefinition(() => host, new CommandBus()).resolveTree();
			const listItems = tree.sections[1]?.items ?? [];
			expect(listItems[0]?.label).toBe("Quad");
			expect(listItems[0]?.defaultOpen).toBe(false);
			expect(listItems[0]?.items?.[0]?.label).toBe("Mixed");
			expect(listItems[0]?.items?.[0]?.defaultOpen).toBe(false);
			expect(listItems[0]?.items?.[0]?.items?.[0]?.label).toBe("Standard");
		});
	});

	describe("settings general tree", () => {
		it("exposes compact, expertise, and compute worker rows", () => {
			const host: SettingsHostApi = {
				compact: false,
				setCompact: () => {},
				expertise: Expertise.NORMAL,
				setExpertise: () => {},
				computeWorkerCount: 4,
				setComputeWorkerCount: () => {},
				computeThreadsAvailable: true,
				modes: [
					{ id: "edit", label: "Edit" },
					{ id: "inspect", label: "Inspect" },
				],
				activeModeId: "edit",
				setActiveModeId: () => {},
				hasModeNav: true,
			};
			const tree = new FrameworkSettingsGeneralTreeDefinition(() => host).resolveTree();
			const items = tree.sections[0]?.items ?? [];
			expect(items.length).toBeGreaterThanOrEqual(3);
			expect(items.some((item) => item.id === "framework.settings.general.compact")).toBe(true);
			expect(items.some((item) => item.id === "framework.settings.general.expertise")).toBe(true);
			expect(items.some((item) => item.id === "framework.settings.general.workers")).toBe(true);
			const modeTree = new FrameworkSettingsModeTreeDefinition(() => host).resolveTree();
			const modeItems = modeTree.sections[0]?.items ?? [];
			expect(modeItems.some((item) => item.id === "framework.settings.mode.select")).toBe(true);
		});

		it("registers mode, app, and general settings tabs as tree definitions", () => {
			const host: SettingsHostApi = {
				compact: false,
				setCompact: () => {},
				expertise: Expertise.NORMAL,
				setExpertise: () => {},
				computeWorkerCount: 4,
				setComputeWorkerCount: () => {},
				computeThreadsAvailable: true,
				appId: "demo",
				appLabel: "Demo",
				modes: [{ id: "edit", label: "Edit" }],
				activeModeId: "edit",
				setActiveModeId: () => {},
				hasModeNav: true,
			};
			const wb = new Platform();
			const tabs = createFrameworkSettingsPanelTabs(() => host, () => null, () => wb, wb.commandBus);
			expect(tabs.map((tab) => tab.id)).toEqual([
				"framework.settings.mode",
				"framework.settings.app",
				"framework.settings.general",
			]);
			for (const tab of tabs) {
				expect(tab.tree && typeof tab.tree === "object" && "resolveTree" in tab.tree).toBe(true);
				if (tab.tree && typeof tab.tree === "object" && "resolveTree" in tab.tree) {
					const config = tab.tree.resolveTree();
					expect(config.sections.length).toBeGreaterThan(0);
					expect(config.sections[0]?.items?.length).toBeGreaterThan(0);
				}
			}
		});
	});

	describe("component kind renderers", () => {
		it("renders registered table components from platform registry", () => {
			class DemoTable extends Table {
				override buildSnapshot(): TableModel {
					return {
						columns: [{ id: "name", label: "Name" }],
						rows: [{ id: "1", cells: { name: "row-alpha" } }],
					};
				}
			}
			const platform = new Platform({ id: "demo", name: "Demo" });
			const table = new DemoTable("surface/demo-table", "ctrl");
			table.refresh();
			platform.registerComponent(table);
			const markup = renderToStaticMarkup(
				renderComponentHostSurface(
					{ type: "table", componentKind: "table", surfaceId: "surface/demo-table", controllerId: "ctrl" },
					"panel",
					platform,
				),
			);
			expect(markup).toContain("row-alpha");
		});

		it("renders per-app virtual file system surfaces independently", () => {
			const platform = new Platform({ id: "demo", name: "Demo" });
			registerPlatformVirtualFileSystemDemo(platform);
			const surfaceIdA = virtualFileSystemSurfaceId(PlatformVirtualFileSystemDemoController.APP_A);
			const markupA = renderToStaticMarkup(
				renderComponentHostSurface(
					{
						type: "virtualFileSystem",
						componentKind: "virtualFileSystem",
						surfaceId: surfaceIdA,
						controllerId: "platform-vfs-demo-ctrl",
					},
					"panel",
					platform,
				),
			);
			expect(markupA).toContain("Alpha");
			expect(markupA).toContain("Models");
			expect(markupA).not.toContain("Capsule");
			const surfaceIdB = virtualFileSystemSurfaceId(PlatformVirtualFileSystemDemoController.APP_B);
			const markupB = renderToStaticMarkup(
				renderComponentHostSurface(
					{
						type: "virtualFileSystem",
						componentKind: "virtualFileSystem",
						surfaceId: surfaceIdB,
						controllerId: "platform-vfs-demo-ctrl",
					},
					"panel",
					platform,
				),
			);
			expect(markupB).toContain("Beta Branch");
			expect(markupB).not.toContain("Alpha");
		});

		it("maps puzzle5d flat and volume selections to puzzle5dSelection payload", () => {
			expect(puzzle5dSelectionPayload("inst-1", "flat", { ids: ["a", "b"] })).toEqual({
				instanceId: "inst-1",
				puzzle2dIds: ["a", "b"],
			});
			expect(
				puzzle5dSelectionPayload("inst-2", "volume", {
					objectIds: ["o1"],
					vortexIds: ["v1"],
					attractionIds: ["c1"],
				}),
			).toEqual({
				instanceId: "inst-2",
				puzzle2dIds: ["o1", "v1", "c1"],
			});
		});

		it("renders registerSurfaceBinding hosts with PascalCase dynamic components", () => {
			const surfaceId = "cad.play.scene3d/test-binding";
			const TestCadHost: React.FC<{ readonly node: UiCadHostSurfaceNode }> = ({ node }) => (
				<div data-testid="cad-surface-host">{node.surfaceId}</div>
			);
			registerSurfaceBinding(surfaceId, TestCadHost);
			try {
				const markup = renderToStaticMarkup(
					<UiRenderer
						commandBus={new CommandBus()}
						node={{ type: "cad", componentKind: "cad", surfaceId, controllerId: "ctrl" }}
					/>,
				);
				expect(markup).toContain('data-testid="cad-surface-host"');
				expect(markup).toContain(surfaceId);
			} finally {
				unregisterSurfaceBinding(surfaceId);
			}
		});
	});

	describe("UiRenderer", () => {
		it("renders text and dispatches button commands", () => {
			const bus = new CommandBus();
			let dispatched = "";
			class TCtrl extends Controller {
				constructor() {
					super("ctrl", bus, () => undefined);
				}
				override run(command: string): void {
					dispatched = command;
				}
			}
			new TCtrl();
			const markup = renderToStaticMarkup(
				<UiRenderer
					commandBus={bus}
					node={{
						type: "stack",
						direction: "vertical",
						children: [
							{ type: "text", value: "hello" },
							{
								type: "button",
								label: "Go",
								command: { controllerId: "ctrl", command: "go" },
							},
						],
					}}
				/>,
			);
			expect(markup).toContain("hello");
			expect(markup).toContain("Go");
			bus.dispatch("ctrl", "go");
			expect(dispatched).toBe("go");
		});
	});
}
//#endregion ­ƒº¬Tests

//#endregion ­ƒôªui-declarative-renderer.tsx

//#region ­ƒôªshell-bridge.tsx
const registeredIcons = new Map<string, IconSource>();

/** @emoji ­ƒû╝ Registers a static icon node resolved by `iconId` for toolbars, footers, and tabs. */
export function registerIcon(iconId: string, source: IconSource): void {
	registeredIcons.set(iconId, source);
}

export function registerElementIcon(iconId: string, node: React.ReactNode): void {
	registerIcon(iconId, { node });
}

export function registerTabIcon(iconId: string, name: IconName): void {
	registerIcon(iconId, name);
}

/** @emoji ­ƒöì Returns a registered element icon node for navbar/search rows. */
export function resolveElementIcon(iconId: string, size = 16): React.ReactNode | undefined {
	const source = registeredIcons.get(iconId);
	if (!source) return undefined;
	return (
		<span className="inline-flex items-center justify-center" style={{ width: size, height: size }}>
			<Icon icon={source} size={size} />
		</span>
	);
}

const PANEL_KIND_ICON: Record<PanelKind, IconName> = {
	display: "layout-grid",
	overview: "folder-open",
	workbench: "folder",
	details: "info",
	settings: "settings-2",
	chat: "message-square",
};

function renderPanelKindIcon(kind: PanelKind, size: number | "tiny" | "small" | "base" | "large" = 16): React.ReactNode {
	return <Icon icon={PANEL_KIND_ICON[kind]} size={size} />;
}

function resolveTabIconNode(iconId: string, panelKind: PanelKind, size = 16): React.ReactNode {
	const registered = resolveElementIcon(iconId, size);
	if (registered) return registered;
	return renderPanelKindIcon(panelKind, size);
}

const windowBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji ­ƒ¬ƒ Binds a `bodyKey` from {@link WindowKindRuntime} to a React window body component. */
export function registerWindowBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	windowBodyByKey.set(bodyKey, Component);
}

/** @emoji 🎯 Picks the initial active window kind from layout or the first registered kind. */
export function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
	const allowed = new Set(windowKinds.map((windowKind) => windowKind.id));
	const visit = (node: WindowLayout["root"]): string | null => {
		if (node.kind === "stack") {
			for (const child of node.children) {
				if (allowed.has(child.windowKindId)) return child.windowKindId;
			}
			return null;
		}
		for (const child of node.children) {
			const match = visit(child);
			if (match) return match;
		}
		return null;
	};
	if (layout) {
		const match = visit(layout.root);
		if (match) return match;
	}
	return windowKinds[0]?.id ?? null;
}

const declarativeWindowBodyComponents = new Map<string, React.FC>();

function getDeclarativeWindowBodyComponent(windowKindId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${windowKindId}`;
	let component = declarativeWindowBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeWindowBody() {
			const { platform, activeModeId } = useApp();
			const generation = reactHostPort.useSyncExternalStore(
				(listener) => platform.subscribe(listener),
				() => platform.generation,
				() => 0,
			);
			const ctx: WindowBodyViewContext = {
				platform,
				windowKindId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getWindowBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative body "${bodyKey}"` };
			return (
				<div data-window-content-layout="edgeless" className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
					<UiRenderer node={node} commandBus={platform.commandBus} platform={platform} />
				</div>
			);
		};
		declarativeWindowBodyComponents.set(cacheKey, component);
	}
	return component;
}

const declarativeSidePanelBodyComponents = new Map<string, React.FC>();

function getDeclarativeSidePanelBodyComponent(tabId: string, bodyKey: string): React.FC {
	const cacheKey = `${bodyKey}\0${tabId}`;
	let component = declarativeSidePanelBodyComponents.get(cacheKey);
	if (!component) {
		component = function ShellDeclarativeSidePanelBody() {
			const { platform, activeModeId } = useApp();
			const generation = reactHostPort.useSyncExternalStore(
				(listener) => platform.subscribe(listener),
				() => platform.generation,
				() => 0,
			);
			const ctx: SidePanelBodyViewContext = {
				platform,
				windowKindId: tabId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getSidePanelBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={platform.commandBus} platform={platform} />;
		};
		declarativeSidePanelBodyComponents.set(cacheKey, component);
	}
	return component;
}

function mapWindowMeasureToGolden(measure: WindowMeasure, bus: CommandBus): UIWindowMeasure {
	if (measure.kind === "group") {
		return {
			id: measure.id,
			kind: "group",
			label: measure.label,
			defaultOpen: measure.defaultOpen,
			children: measure.children.map((child) => mapWindowMeasureToGolden(child, bus)),
		};
	}
	if (measure.kind === "select") {
		return {
			id: measure.id,
			kind: "select",
			label: measure.label,
			value: measure.value,
			items: measure.items.map((item) => ({ id: item.id, value: item.value, label: item.label })),
			onValueChange: (value: string) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { ...(measure.onChange.args as object | undefined), value }),
		};
	}
	if (measure.kind === "slider") {
		return {
			id: measure.id,
			kind: "slider",
			label: measure.label,
			value: measure.value,
			min: measure.min,
			max: measure.max,
			step: measure.step,
			onValueChange: (value: number) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { ...(measure.onChange.args as object | undefined), value }),
		};
	}
	if (measure.kind === "toggle") {
		return {
			id: measure.id,
			kind: "toggle",
			label: measure.label,
			text: measure.text,
			pressed: measure.pressed,
			icon: <Icon icon={measure.iconId in ICONS ? (measure.iconId as IconName) : "circle-dot"} size="small" />,
			onPressedChange: (pressed: boolean) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { ...(measure.onChange.args as object | undefined), pressed }),
		};
	}
	return { id: measure.id, kind: "display", content: null };
}

/** @emoji 📐 Maps {@link WindowMeasure} controller rows to {@link UIWindowMeasure} tiles for {@link ShellModeCanvas}. */
export function windowMeasuresToGolden(measures: readonly WindowMeasure[], bus: CommandBus): UIWindowMeasure[] | undefined {
	if (!measures.length) return undefined;
	return measures.map((measure) => mapWindowMeasureToGolden(measure, bus));
}

/** @emoji ­ƒ¬ƒ Converts framework window kinds into golden-layout window definitions. */
export function windowKindsToGolden(windowKinds: readonly WindowKindRuntime[], bus: CommandBus): UIWindowKindDefinition[] {
	const goldenMeasures = (wk: WindowKindRuntime) => windowMeasuresToGolden(wk.measures, bus);
	return windowKinds.map((wk) => {
		const declarativeFactory = getWindowBodyFactory(wk.bodyKey);
		if (declarativeFactory) {
			return { id: wk.id, label: wk.label, component: getDeclarativeWindowBodyComponent(wk.id, wk.bodyKey), measures: goldenMeasures(wk) };
		}
		const Body =
			windowBodyByKey.get(wk.bodyKey) ??
			(() => (
				<div className="p-2 text-xs text-muted-foreground">
					Missing window body &quot;{wk.bodyKey}&quot;
				</div>
			));
		return { id: wk.id, label: wk.label, component: Body as React.ComponentType, measures: goldenMeasures(wk) };
	});
}

export function shellTabIconComponent(iconId: string, panelKind: PanelKind): React.ComponentType<{ size?: number }> {
	return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
		return <>{resolveTabIconNode(iconId, panelKind, size)}</>;
	};
}

/** @emoji 📑 Converts framework side tabs into panel tab configs with declarative trees. */
export function sideTabsToPanelTabs(tabs: readonly SideTabSpec[], platform: Platform, bus: CommandBus): SidePanelTabConfig[] {
	return tabs.map((tab, orderIndex) => {
		const declarativeFactory = getSidePanelBodyFactory(tab.bodyKey);
		if (declarativeFactory) {
			return {
				id: tab.id,
				icon: shellTabIconComponent(tab.iconId, tab.panel),
				name: tab.label,
				order: tab.order ?? orderIndex,
				tree: new DeclarativeSidePanelTreeDefinition(platform, tab.id, tab.bodyKey, bus),
			};
		}
		return {
			id: tab.id,
			icon: shellTabIconComponent(tab.iconId, tab.panel),
			name: tab.label,
			order: tab.order ?? orderIndex,
			tree: { sections: [{ id: `${tab.id}.missing`, items: [{ id: "missing", label: `Missing panel ${tab.bodyKey}` }] }] },
		};
	});
}

/** @emoji ­ƒæú Converts framework footer items into React footer rows. */
export function declarativeFooterToChromeRows(items: readonly DeclarativeFooterItem[], bus: CommandBus): ChromeFooterRow[] {
	return items.map((item) => ({
		id: item.id,
		text: item.text,
		order: item.order,
		className: item.className,
		disabled: item.disabled,
		icon: resolveDeclarativeControlIcon(item.iconId),
		onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
	}));
}

/** @emoji 👣 Merges platform-wide, app, and extra footer rows for {@link ProductShell}. */
export function mergePlatformFooterChromeRows(
	platform: Platform,
	activeApp: { readonly footerItems: readonly DeclarativeFooterItem[] },
	extraFooterItems: readonly ChromeFooterRow[] = [],
): ChromeFooterRow[] {
	return [
		...declarativeFooterToChromeRows(platform.globalFooterItems, platform.commandBus),
		...declarativeFooterToChromeRows(activeApp.footerItems, platform.commandBus),
		...extraFooterItems,
	].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
}

function resolveDeclarativeControlIcon(iconId: string, size: number | "tiny" | "small" | "base" | "large" = 16): React.ReactNode {
	return resolveElementIcon(iconId, typeof size === "number" ? size : 16) ?? <Icon icon={iconId in ICONS ? (iconId as IconName) : "circle-dot"} size={size} />;
}

export { resolveDeclarativeControlIcon };

function resolveToolItemIcon(iconId: string, size = 16): React.ReactNode {
	return resolveDeclarativeControlIcon(iconId, size);
}

function shellToolToToolNode(item: ToolNode, bus: CommandBus): UIToolNode {
	if (item.kind === "collection") {
		return {
			id: item.id,
			kind: "collection",
			icon: resolveToolItemIcon(item.iconId),
			label: item.label,
			text: item.text,
			order: item.order,
			children: item.children.map((child) => shellToolToToolNode(child, bus)),
		};
	}
	if (item.kind === "separator") {
		return { id: item.id, kind: "separator", order: item.order };
	}
	const iconNode = resolveToolItemIcon(item.iconId);
	if (item.kind === "toggle") {
		return {
			id: item.id,
			kind: "toggle",
			icon: iconNode,
			label: item.label,
			text: item.text,
			order: item.order,
			pressed: item.pressed,
			onPressedChange: (pressed: boolean) => {
				if (item.controllerId && item.command) bus.dispatch(item.controllerId, item.command, { ...((item.args as object | undefined) ?? {}), pressed });
			},
		};
	}
	return {
		id: item.id,
		icon: iconNode,
		label: item.label,
		text: item.text,
		order: item.order,
		onClick: item.controllerId && item.command ? () => bus.dispatch(item.controllerId!, item.command!, item.args) : undefined,
	};
}

/** @emoji 🎛 Converts declarative {@link FrameworkAppTools} into mounted toolbar nodes. */
export function declareToolsToViewTools(tools: FrameworkAppTools | undefined, bus: CommandBus): ToolbarViewTools | undefined {
	if (!tools?.length) return undefined;
	const mapped = tools.map((entry) => shellToolToToolNode(entry, bus));
	return mapped.length > 0 ? mapped : undefined;
}

/** @emoji ­ƒöÇ Merges config rows by `id` (extension overrides base). */
export function mergeConfigEntries<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
	if (!base?.length && !extension?.length) return undefined;
	const merged = new Map<string, T>();
	base?.forEach((entry) => merged.set(entry.id, entry));
	extension?.forEach((entry) => merged.set(entry.id, entry));
	return [...merged.values()];
}

//#endregion ­ƒôªshell-bridge.tsx

//#region ­ƒôªworkbench-view.tsx
const ProductFindItemsSync: React.FC<{
	findItems?: UIFindItem[];
	onFindSelect?: (itemId: string) => void;
}> = ({ findItems, onFindSelect }) => {
	const { setFindItems, setOnFindItem } = useUIFind();
	const resolvedFindItems = findItems ?? EMPTY_UI_FIND_ITEMS;
	reactHostPort.useEffect(() => {
		setFindItems(resolvedFindItems);
		setOnFindItem(onFindSelect);
	}, [findItems, onFindSelect, resolvedFindItems, setFindItems, setOnFindItem]);
	return null;
};
function panelKindToggleIcon(kind: PanelKind, tabs: SidePanelTabConfig[]): React.ReactNode {
	void tabs;
	return renderPanelKindIcon(kind, 16);
}

function resolveAppPanelTabsByKind(
	app: ResolvedAppState,
	platform: Platform,
	bus: CommandBus,
	getDisplayHost: () => DisplayHostApi | null,
	getSettingsHost: () => SettingsHostApi | null,
	augment?: Partial<Record<PanelKind, SidePanelTabConfig[]>>,
): Record<PanelKind, SidePanelTabConfig[]> {
	const grouped = new Map<PanelKind, SideTabSpec[]>();
	for (const kind of PANEL_KINDS) grouped.set(kind, []);
	for (const tab of app.panelTabs) {
		grouped.get(tab.panel)?.push(tab);
	}
	const result = {} as Record<PanelKind, SidePanelTabConfig[]>;
	for (const kind of PANEL_KINDS) {
		const resolved = sideTabsToPanelTabs(grouped.get(kind) ?? [], platform, bus);
		result[kind] = mergeConfigEntries(resolved, augment?.[kind]) ?? resolved;
	}
	if (app.windowKinds.length > 0) {
		const displayTabs = createFrameworkDisplayPanelTabs(getDisplayHost, bus);
		result.display = mergeConfigEntries(result.display, displayTabs) ?? displayTabs;
	}
	const settingsTabs = createFrameworkSettingsPanelTabs(getSettingsHost, getDisplayHost, () => platform, bus);
	result.settings = mergeConfigEntries(result.settings, settingsTabs) ?? settingsTabs;
	return result;
}

function uriToBreadcrumbItems(uri: string, onNavigate: (href: string) => void): BreadcrumbItemData[] {
	if (uri === "/" || uri === "") {
		return [{ id: "breadcrumb.root", content: "Home", onNavigate }];
	}
	const segments = uri.split("/").filter(Boolean);
	const items: BreadcrumbItemData[] = [{ id: "breadcrumb.root", content: "Home", onNavigate: () => onNavigate("/") }];
	let path = "";
	for (const segment of segments) {
		path += `/${segment}`;
		const href = path;
		items.push({ id: `breadcrumb.${href}`, content: segment, onNavigate: () => onNavigate(href) });
	}
	return items;
}

function navigationTrailToBreadcrumbItems(trail: readonly NavigationLevel[], onNavigate: (href: string) => void): BreadcrumbItemData[] {
	return trail.map((level, index) => ({
		id: level.node.id ?? `breadcrumb.${index}`,
		content: level.node.label as React.ReactNode,
		options: level.alternatives.map((alternative) => ({
			id: alternative.id,
			label: alternative.label as React.ReactNode,
			href: alternative.uri,
		})),
		onNavigate: (href: string) => onNavigate(href || level.node.uri),
	}));
}

function readBrowserUri(): string {
	if (typeof window === "undefined") return "/";
	return `${window.location.pathname}${window.location.search}`;
}

//#region 🪨ProductShell

/** @emoji 🪨 Shared product base layout: navbar, floating side panels, window canvas, footer, optional toolbar/search/find. */
export interface ProductShellProps {
	readonly platform: Platform;
	readonly defaultAppId?: string;
	readonly className?: string;
	readonly mobile?: boolean;
	readonly mobileQuery?: string;
	readonly initialPanelVisibility?: UIPanelVisibility;
	readonly navbarItems: NavbarItem[];
	readonly footerItems?: ChromeFooterRow[];
	readonly slotToolbar?: React.ReactNode;
	readonly leftSidePanelTabs: SidePanelTabConfig[];
	readonly rightSidePanelTabs: SidePanelTabConfig[];
	readonly mobilePanel?: {
		readonly visible: boolean;
		readonly tabs: SidePanelTabConfig[];
		readonly activeTabId?: string;
		readonly onActiveTabChange?: (tabId: string) => void;
	};
	readonly panelVisibility: UIPanelVisibility;
	readonly leftPanelSize: number;
	readonly onLeftPanelSizeChange: (size: number) => void;
	readonly rightPanelSize: number;
	readonly onRightPanelSizeChange: (size: number) => void;
	readonly goldenWindowKinds: UIWindowKindDefinition[];
	readonly windowKindCatalog: readonly WindowKindRuntime[];
	readonly namedLayouts: readonly NamedLayout[];
	readonly namedLayoutStore: NamedLayoutStore;
	readonly commandBus: CommandBus;
	readonly onDisplayHostReady?: (host: DisplayHostApi) => void;
	readonly defaultLayout: WindowLayout;
	readonly activeWindowKindId: string | null;
	readonly onActiveWindowKindChange: (windowKindId: string) => void;
	readonly multiApp?: boolean;
	readonly activeModeId?: string | null;
	readonly onActiveModeChange?: (modeId: string) => void;
	readonly searchItems?: UISearchItem[];
	readonly searchOpen?: boolean;
	readonly onSearchOpenChange?: (open: boolean) => void;
	readonly findOpen?: boolean;
	readonly onFindOpenChange?: (open: boolean) => void;
	readonly onToggleLastActiveLeftSidePanel?: () => void;
	readonly onToggleLastActiveRightSidePanel?: () => void;
}

/** @emoji 🪨 Navbar + canvas (windows) + footer with left/right panels floating over the canvas. */
export const ProductShell: React.FC<ProductShellProps> = ({
	platform,
	defaultAppId,
	className,
	mobile,
	mobileQuery = "(max-width: 767px)",
	navbarItems,
	footerItems,
	slotToolbar,
	leftSidePanelTabs,
	rightSidePanelTabs,
	mobilePanel,
	panelVisibility,
	leftPanelSize,
	onLeftPanelSizeChange,
	rightPanelSize,
	onRightPanelSizeChange,
	goldenWindowKinds,
	windowKindCatalog,
	namedLayouts,
	namedLayoutStore,
	commandBus,
	onDisplayHostReady,
	defaultLayout,
	activeWindowKindId,
	onActiveWindowKindChange,
	multiApp = true,
	activeModeId,
	onActiveModeChange,
	searchItems,
	searchOpen: searchOpenProp,
	onSearchOpenChange,
	findOpen: findOpenProp,
	onFindOpenChange,
	onToggleLastActiveLeftSidePanel,
	onToggleLastActiveRightSidePanel,
}) => {
	reactHostPort.useEffect(() => {
		if (defaultAppId) platform.setActiveAppId(defaultAppId);
	}, [defaultAppId, platform]);

	const detectedMobile = useMediaQuery(mobileQuery);
	const resolvedMobile = mobile ?? detectedMobile ?? platform.mobile;
	const [internalSearchOpen, setInternalSearchOpen] = reactHostPort.useState(false);
	const searchOpen = searchOpenProp ?? internalSearchOpen;
	const setSearchOpen = onSearchOpenChange ?? setInternalSearchOpen;
	const findOpen = findOpenProp ?? false;

	useSidePanelChromeHotkeys({
		onToggleLeft: leftSidePanelTabs.length > 0 ? onToggleLastActiveLeftSidePanel : undefined,
		onToggleRight: rightSidePanelTabs.length > 0 ? onToggleLastActiveRightSidePanel : undefined,
	});

	useCommandHotkey(
		"ctrl+p,meta+p",
		() => {
			const activeEl = document.activeElement as HTMLElement | null;
			if (!searchOpen && activeEl && (activeEl.tagName === "INPUT" || activeEl.tagName === "TEXTAREA" || activeEl.isContentEditable)) {
				return;
			}
			setSearchOpen(!searchOpen);
		},
		{ preventDefault: true, enableOnFormTags: true },
		[searchOpen, setSearchOpen],
	);
	useCommandHotkey(
		"ctrl+f,meta+f",
		() => {
			if (onFindOpenChange) onFindOpenChange(!findOpen);
		},
		{ preventDefault: true, enableOnFormTags: true },
		[findOpen, onFindOpenChange],
	);

	const resolvedApps = platform.apps;
	const activeAppId = platform.activeAppId;
	const setActiveAppId = reactHostPort.useCallback(
		(id: string) => {
			platform.setActiveAppId(id);
		},
		[platform],
	);

	const activeAppBase = platform.getActiveApp();
	if (!activeAppBase) return null;

	const activeModeIdResolved = activeModeId ?? activeAppBase.getActiveModeId();
	const shellDataGeneration = platform.generation;
	const activeApp = reactHostPort.useMemo(
		() => activeAppBase.resolve(activeModeIdResolved),
		[activeAppBase, activeModeIdResolved, shellDataGeneration],
	);

	const canvasNode = reactHostPort.useMemo(
		() =>
			multiApp ? (
				<Ui
					apps={resolvedApps.map((app) => ({
						id: app.id,
						label: app.label,
						icon: app.iconId ? resolveElementIcon(app.iconId) : undefined,
						children: (
							<App
								modes={app.modes.length > 0 ? app.modes.map((mode) => ({ id: mode.id, label: mode.label, children: null })) : [{ id: app.id, label: app.label, children: null }]}
								activeModeId={app.id === activeAppId ? (activeModeId ?? app.modes[0]?.id ?? app.id) : (app.modes[0]?.id ?? app.id)}
								onActiveModeChange={app.id === activeAppId ? onActiveModeChange : undefined}
								chrome={false}
							>
								{app.id === activeAppId ? (
									<ShellModeCanvas
										windowKinds={goldenWindowKinds}
										windowKindCatalog={windowKindCatalog}
										namedLayouts={namedLayouts}
										namedLayoutStore={namedLayoutStore}
										commandBus={commandBus}
										onDisplayHostReady={onDisplayHostReady}
										defaultLayout={defaultLayout}
										activeWindowId={activeWindowKindId}
										onActiveWindowChange={onActiveWindowKindChange}
									/>
								) : null}
							</App>
						),
					}))}
					activeAppId={activeAppId}
					onActiveAppChange={setActiveAppId}
					chrome={false}
				/>
			) : (
				<Ui
					apps={[
						{
							id: activeApp.id,
							label: activeApp.label,
							icon: activeApp.iconId ? resolveElementIcon(activeApp.iconId) : undefined,
							children: (
								<App
									modes={activeAppBase.modes.length > 0 ? activeAppBase.modes.map((mode) => ({ id: mode.id, label: mode.label, children: null })) : [{ id: activeApp.id, label: activeApp.label, children: null }]}
									activeModeId={activeModeId ?? activeAppBase.modes[0]?.id ?? activeApp.id}
									onActiveModeChange={onActiveModeChange}
									chrome={false}
								>
									<ShellModeCanvas
										windowKinds={goldenWindowKinds}
										windowKindCatalog={windowKindCatalog}
										namedLayouts={namedLayouts}
										namedLayoutStore={namedLayoutStore}
										commandBus={commandBus}
										onDisplayHostReady={onDisplayHostReady}
										defaultLayout={defaultLayout}
										activeWindowId={activeWindowKindId}
										onActiveWindowChange={onActiveWindowKindChange}
									/>
								</App>
							),
						},
					]}
					activeAppId={activeAppId}
					chrome={false}
				/>
			),
		[
			activeApp,
			activeAppBase.modes,
			activeAppId,
			activeModeId,
			activeWindowKindId,
			defaultLayout,
			goldenWindowKinds,
			windowKindCatalog,
			namedLayouts,
			namedLayoutStore,
			commandBus,
			onDisplayHostReady,
			multiApp,
			onActiveModeChange,
			onActiveWindowKindChange,
			resolvedApps,
			setActiveAppId,
		],
	);

	return (
		<>
			<Layout
				className={className}
				mobile={resolvedMobile}
				navbar={<Navbar items={navbarItems} />}
				footer={<Footer items={footerItems ?? []} toolbar={slotToolbar} />}
				mobilePanel={
					resolvedMobile && mobilePanel
						? {
								visible: mobilePanel.visible,
								activeTabId: mobilePanel.activeTabId,
								onActiveTabChange: mobilePanel.onActiveTabChange,
								tabs: mobilePanel.tabs,
							}
						: undefined
				}
				leftSidePanel={
					!resolvedMobile && leftSidePanelTabs.length > 0
						? {
								position: "left" as const,
								visible: panelVisibility.leftSidePanel,
								size: leftPanelSize,
								onSizeChange: onLeftPanelSizeChange,
								tabs: leftSidePanelTabs,
							}
						: undefined
				}
				rightSidePanel={
					!resolvedMobile && rightSidePanelTabs.length > 0
						? {
								position: "right" as const,
								visible: panelVisibility.rightSidePanel,
								size: rightPanelSize,
								onSizeChange: onRightPanelSizeChange,
								tabs: rightSidePanelTabs,
							}
						: undefined
				}
				canvas={canvasNode}
			/>
			<UISearch items={searchItems ?? []} open={searchOpen} onOpenChange={setSearchOpen} />
			{onFindOpenChange ? <UIFind open={findOpen} onOpenChange={onFindOpenChange} /> : null}
		</>
	);
};

//#endregion 🪨ProductShell

/** @emoji 🧭 Wraps {@link PlatformView} with browser History API sync and {@link useUIHistory}. */
/** @emoji 🧭 {@link PlatformView} with browser History API sync and {@link useUIHistory}. */
export const PlatformViewWithHistory: React.FC<Omit<PlatformViewProps, "uri" | "onNavigate" | "canGoBack" | "onGoBack" | "canGoForward" | "onGoForward" | "canGoUp" | "onGoUp">> = ({
	platform,
	...rest
}) => {
	const { uri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate, syncUri } = useUIHistory(readBrowserUri());

	reactHostPort.useEffect(() => {
		platform.applyUri?.(uri);
		platform.uri = uri;
		platform.canGoBack = canGoBack;
		platform.canGoForward = canGoForward;
		platform.canGoUp = canGoUp;
		if (typeof window !== "undefined") {
			const current = `${window.location.pathname}${window.location.search}`;
			if (current !== uri) {
				window.history.pushState(null, "", uri);
			}
		}
		platform.notify();
	}, [uri, canGoBack, canGoForward, canGoUp, platform]);

	reactHostPort.useEffect(() => {
		if (typeof window === "undefined") return;
		const onPopState = () => {
			const browserUri = readBrowserUri();
			syncUri(browserUri);
		};
		window.addEventListener("popstate", onPopState);
		return () => window.removeEventListener("popstate", onPopState);
	}, [syncUri]);

	const handleNavigate = reactHostPort.useCallback(
		(targetUri: string) => {
			platform.applyUri?.(targetUri);
			navigate(targetUri);
		},
		[navigate, platform],
	);

	return (
		<PlatformView
			platform={platform}
			uri={uri}
			onNavigate={handleNavigate}
			canGoBack={canGoBack}
			onGoBack={goBack}
			canGoForward={canGoForward}
			onGoForward={goForward}
			canGoUp={canGoUp}
			onGoUp={goUp}
			{...rest}
		/>
	);
};

/**
 * Domain-neutral composite component providing a full application shell.
 * The UI only has apps. An app has window kinds (rendered with golden-layout)
 * and registers left/right side panel tabs, footer items, toolbar items, and find items.
 * Every UI has: toolbar, search (Ctrl+P), panel toggles, back/forward/up navigation.
 * Every app has: find (Ctrl+F).
 * Every panel has: tree.
 * Fixed navbar layout: [mode (if >1 mode)] [nav history group] [breadcrumb (flex-1)] [search] [find] [panel toggles].
 **/
export const PlatformView: React.FC<PlatformViewProps> = ({
	platform,
	defaultAppId,
	uri: uriProp = "/",
	onNavigate,
	canGoBack: canGoBackProp = false,
	onGoBack,
	canGoForward: canGoForwardProp = false,
	onGoForward,
	canGoUp: canGoUpProp = false,
	onGoUp,
	mobile,
	mobileQuery = "(max-width: 767px)",
	className,
	initialPanelVisibility,
	resolvedWindowKindsOverride,
	slotToolbar,
	slotNavbarCenter,
	extraFooterItems,
	augmentPanelTabs,
}) => {
	const shellGen = reactHostPort.useSyncExternalStore(
		(onStoreChange) => {
			const unsubData = platform.subscribe(onStoreChange);
			const unsubChrome = platform.subscribeChrome(onStoreChange);
			return () => {
				unsubData();
				unsubChrome();
			};
		},
		() => platform.generation * 1_000_000 + platform.chromeGeneration,
		() => 0,
	);
	void shellGen;

	reactHostPort.useEffect(() => {
		if (defaultAppId) {
			platform.setActiveAppId(defaultAppId);
		}
	}, [defaultAppId, platform]);

	reactHostPort.useEffect(() => {
		platform.uri = uriProp;
		platform.onNavigate = onNavigate;
		platform.onGoBack = onGoBack;
		platform.onGoForward = onGoForward;
		platform.onGoUp = onGoUp;
		platform.canGoBack = canGoBackProp;
		platform.canGoForward = canGoForwardProp;
		platform.canGoUp = canGoUpProp;
		platform.mobile = mobile;
		platform.mobileQuery = mobileQuery;
		platform.className = className ?? "";
	}, [uriProp, onNavigate, onGoBack, onGoForward, onGoUp, canGoBackProp, canGoForwardProp, canGoUpProp, mobile, mobileQuery, className, platform]);

	const [leftPanelSize, setLeftPanelSize] = reactHostPort.useState(280);
	const [rightPanelSize, setRightPanelSize] = reactHostPort.useState(300);
	const [panelVisibility, setPanelVisibilityState] = reactHostPort.useState<UIPanelVisibility>(() =>
		resolveInitialPanelVisibility(initialPanelVisibility ?? PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY, platform),
	);
	const setPanelVisibility = reactHostPort.useCallback(
		(next: UIPanelVisibility | ((prev: UIPanelVisibility) => UIPanelVisibility)) => {
			setPanelVisibilityState((prev) => {
				const resolved = typeof next === "function" ? next(prev) : next;
				platform.assignPanelVisibility(resolved);
				return resolved;
			});
		},
		[platform],
	);
	const [mobilePanelVisible, setMobilePanelVisible] = reactHostPort.useState(false);
	const [activeDesktopLeftPanelKind, setActiveDesktopLeftPanelKind] = reactHostPort.useState<PanelKind>("workbench");
	const [activeDesktopRightPanelKind, setActiveDesktopRightPanelKind] = reactHostPort.useState<PanelKind>("details");
	const [activeMobilePanelKind, setActiveMobilePanelKind] = reactHostPort.useState<PanelKind>("workbench");
	const [mobilePanelActiveTabId, setMobilePanelActiveTabId] = reactHostPort.useState<string | undefined>(undefined);
	const [searchOpen, setSearchOpen] = reactHostPort.useState(false);
	const [findOpen, setFindOpen] = reactHostPort.useState(false);
	const [uiCompact, setUiCompact] = reactHostPort.useState(readStoredUiChromeCompact);
	const [uiExpertise, setUiExpertise] = reactHostPort.useState(readStoredUiChromeExpertise);
	const [uiTheme, setUiTheme] = reactHostPort.useState(readStoredUiChromeTheme);
	const [computeWorkerCount, setComputeWorkerCount] = reactHostPort.useState(readStoredComputeWorkerCount);
	const detectedMobile = useMediaQuery(mobileQuery);
	const resolvedMobile = mobile ?? detectedMobile ?? platform.mobile;

	useElementsSurfaceChrome({ ...PLATFORM_SYSTEM_SURFACE_CHROME, theme: uiTheme, compact: uiCompact, expertise: uiExpertise });

	const togglePanel = reactHostPort.useCallback((panel: keyof UIPanelVisibility) => {
		setPanelVisibility((prev) => ({ ...prev, [panel]: !prev[panel] }));
	}, [setPanelVisibility]);

	const resolvedApps = platform.apps;
	const activeAppId = platform.activeAppId;
	const setActiveAppId = reactHostPort.useCallback(
		(id: string) => {
			platform.setActiveAppId(id);
		},
		[platform],
	);

	const activeAppBase = platform.getActiveApp();
	if (!activeAppBase) return null;

	const activeModeId = activeAppBase.getActiveModeId();
	const activeApp = reactHostPort.useMemo(
		() => activeAppBase.resolve(activeModeId),
		[activeAppBase, activeModeId, shellGen],
	);
	const [displayHost, setDisplayHost] = reactHostPort.useState<DisplayHostApi | null>(null);
	const handleDisplayHostReady = reactHostPort.useCallback((host: DisplayHostApi) => {
		setDisplayHost((previous) => (previous?.windowKinds === host.windowKinds ? previous : host));
	}, []);

	const hasModeNav = activeAppBase.modes.length > 1;
	const setActiveModeId = reactHostPort.useCallback(
		(id: string) => {
			activeAppBase.setActiveModeId(id);
			platform.notifyChrome();
		},
		[activeAppBase, platform],
	);

	const settingsHostApi = reactHostPort.useMemo<SettingsHostApi>(
		() => ({
			compact: uiCompact,
			setCompact: (compact: boolean) => {
				setUiCompact(compact);
				writeStoredUiChromeCompact(compact);
			},
			expertise: uiExpertise,
			setExpertise: (expertise: Expertise) => {
				setUiExpertise(expertise);
				writeStoredUiChromeExpertise(expertise);
			},
			computeWorkerCount,
			setComputeWorkerCount: (count: number) => {
				const clamped = Math.max(1, Math.floor(count));
				setComputeWorkerCount(clamped);
				writeStoredComputeWorkerCount(clamped);
			},
			computeThreadsAvailable: isCrossOriginIsolatedRuntime(),
			theme: uiTheme,
			setTheme: (theme: ElementsSurfaceTheme) => {
				setUiTheme(theme);
				writeStoredUiChromeTheme(theme);
			},
			appId: activeApp.id,
			appLabel: activeApp.label,
			appIconId: activeApp.iconId,
			modes: activeAppBase.modes.map((mode) => ({ id: mode.id, label: mode.label, iconId: mode.iconId })),
			activeModeId,
			setActiveModeId,
			hasModeNav,
		}),
		[activeApp.iconId, activeApp.id, activeApp.label, activeModeId, activeAppBase.modes, computeWorkerCount, hasModeNav, setActiveModeId, uiCompact, uiExpertise, uiTheme],
	);

	const panelTabsByKind = reactHostPort.useMemo(
		() => resolveAppPanelTabsByKind(activeApp, platform, platform.commandBus, () => displayHost, () => settingsHostApi, augmentPanelTabs),
		[activeApp, platform, augmentPanelTabs, displayHost, settingsHostApi],
	);
	const leftKindsWithTabs = LEFT_PANEL_KINDS.filter((kind) => panelTabsByKind[kind].length > 0);
	const rightKindsWithTabs = RIGHT_PANEL_KINDS.filter((kind) => panelTabsByKind[kind].length > 0);
	const panelKindsWithTabs = PANEL_KINDS.filter((kind) => panelTabsByKind[kind].length > 0);

	reactHostPort.useEffect(() => {
		if (!leftKindsWithTabs.includes(activeDesktopLeftPanelKind)) {
			setActiveDesktopLeftPanelKind(leftKindsWithTabs[0] ?? "workbench");
		}
		if (!rightKindsWithTabs.includes(activeDesktopRightPanelKind)) {
			setActiveDesktopRightPanelKind(rightKindsWithTabs[0] ?? "details");
		}
		if (!panelKindsWithTabs.includes(activeMobilePanelKind)) {
			setActiveMobilePanelKind(panelKindsWithTabs[0] ?? "workbench");
		}
	}, [activeApp.id, activeModeId, leftKindsWithTabs.join(","), rightKindsWithTabs.join(","), panelKindsWithTabs.join(",")]);

	const leftSidePanelTabs = panelTabsByKind[activeDesktopLeftPanelKind] ?? [];
	const activeDesktopRightPanelTabs = panelTabsByKind[activeDesktopRightPanelKind] ?? [];
	const activeMobilePanelTabs = panelTabsByKind[activeMobilePanelKind] ?? [];

	const activeWindowLayoutKey = reactHostPort.useMemo(() => JSON.stringify(activeApp.defaultLayout), [activeApp.defaultLayout]);
	const activeWindowKindsKey = reactHostPort.useMemo(
		() => activeApp.windowKinds.map((windowKind) => windowKind.id).join("|"),
		[activeApp.windowKinds],
	);
	const [activeWindowKindId, setActiveWindowKindId] = reactHostPort.useState<string | null>(() => findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds));

	reactHostPort.useEffect(() => {
		setActiveWindowKindId((previous) => {
			if (previous && activeApp.windowKinds.some((windowKind) => windowKind.id === previous)) return previous;
			return findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds);
		});
	}, [activeApp, activeWindowKindsKey, activeWindowLayoutKey]);

	const handleActiveWindowChange = reactHostPort.useCallback(
		(windowKindId: string | null) => {
			setActiveWindowKindId(windowKindId);
			if (windowKindId) activeApp.onActiveWindowChange?.(windowKindId);
		},
		[activeApp],
	);

	const mergedTools = reactHostPort.useMemo(
		() => mergeToolbarViewTools(declareToolsToViewTools(platform.globalTools, platform.commandBus), declareToolsToViewTools(activeApp.tools, platform.commandBus)),
		[activeApp.tools, platform, shellGen],
	);
	const hasToolbarTools = hasToolbarViewTools(mergedTools);

	const openDesktopLeftPanel = reactHostPort.useCallback(
		(kind: PanelKind, pressed: boolean) => {
			if (pressed) {
				setActiveDesktopLeftPanelKind(kind);
				setPanelVisibility((prev) => ({ ...prev, leftSidePanel: true }));
				return;
			}
			setPanelVisibility((prev) => ({ ...prev, leftSidePanel: kind === activeDesktopLeftPanelKind ? false : prev.leftSidePanel }));
		},
		[activeDesktopLeftPanelKind, setPanelVisibility],
	);

	const openDesktopRightPanel = reactHostPort.useCallback(
		(kind: PanelKind, pressed: boolean) => {
			if (pressed) {
				setActiveDesktopRightPanelKind(kind);
				setPanelVisibility((prev) => ({ ...prev, rightSidePanel: true }));
				return;
			}
			setPanelVisibility((prev) => ({ ...prev, rightSidePanel: kind === activeDesktopRightPanelKind ? false : prev.rightSidePanel }));
		},
		[activeDesktopRightPanelKind, setPanelVisibility],
	);

	const openMobilePanel = reactHostPort.useCallback(
		(kind: PanelKind, pressed: boolean) => {
			if (pressed) {
				setActiveMobilePanelKind(kind);
				if (panelSide(kind) === "left") {
					setActiveDesktopLeftPanelKind(kind);
				} else {
					setActiveDesktopRightPanelKind(kind);
				}
				setMobilePanelVisible(true);
				return;
			}
			if (activeMobilePanelKind === kind) {
				setMobilePanelVisible(false);
			}
		},
		[activeMobilePanelKind],
	);

	const toggleLastActiveLeftSidePanel = reactHostPort.useCallback(() => {
		if (leftKindsWithTabs.length === 0) return;
		const kind = activeDesktopLeftPanelKind;
		if (resolvedMobile) {
			openMobilePanel(kind, !(mobilePanelVisible && activeMobilePanelKind === kind));
			return;
		}
		openDesktopLeftPanel(kind, !panelVisibility.leftSidePanel);
	}, [
		activeDesktopLeftPanelKind,
		activeMobilePanelKind,
		leftKindsWithTabs.length,
		mobilePanelVisible,
		openDesktopLeftPanel,
		openMobilePanel,
		panelVisibility.leftSidePanel,
		resolvedMobile,
	]);

	const toggleLastActiveRightSidePanel = reactHostPort.useCallback(() => {
		if (rightKindsWithTabs.length === 0) return;
		const kind = activeDesktopRightPanelKind;
		if (resolvedMobile) {
			openMobilePanel(kind, !(mobilePanelVisible && activeMobilePanelKind === kind));
			return;
		}
		openDesktopRightPanel(kind, !panelVisibility.rightSidePanel);
	}, [
		activeDesktopRightPanelKind,
		activeMobilePanelKind,
		mobilePanelVisible,
		openDesktopRightPanel,
		openMobilePanel,
		panelVisibility.rightSidePanel,
		resolvedMobile,
		rightKindsWithTabs.length,
	]);

	const navbarItems: NavbarItem[] = [];

	navbarItems.push({
		key: "logoAndTitle",
		className: "min-w-0 shrink-0 flex items-center gap-single",
		content: (
			<div className="flex items-center gap-single">
				<SemioLogo className="shrink-0 size-workbench" />
				<span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>{activeAppBase.label}</span>
			</div>
		),
	});

	navbarItems.push({
		key: "navHistory",
		content: (
			<ButtonGroup id="ui.nav">
				<ButtonGroupItem id="ui.nav.back" onClick={onGoBack} className={cn(!canGoBackProp && "opacity-30 pointer-events-none")} icon={<Icon icon="arrow-left" size="small" />} />
				<ButtonGroupItem id="ui.nav.forward" onClick={onGoForward} className={cn(!canGoForwardProp && "opacity-30 pointer-events-none")} icon={<Icon icon="arrow-right" size="small" />} />
				<ButtonGroupItem id="ui.nav.up" onClick={onGoUp} className={cn(!canGoUpProp && "opacity-30 pointer-events-none")} icon={<Icon icon="arrow-up" size="small" />} />
			</ButtonGroup>
		),
	});

	const breadcrumbNavigate = reactHostPort.useCallback(
		(href: string) => {
			onNavigate?.(href);
		},
		[onNavigate],
	);
	const breadcrumbItems = reactHostPort.useMemo(() => {
		const trail = platform.navigation?.(uriProp);
		if (trail?.length) return navigationTrailToBreadcrumbItems(trail, breadcrumbNavigate);
		return uriToBreadcrumbItems(uriProp, breadcrumbNavigate);
	}, [platform, uriProp, breadcrumbNavigate]);

	navbarItems.push({
		key: "breadcrumb",
		className: slotNavbarCenter ? "min-w-0 shrink-0 max-w-[40%]" : navbarFillClassName,
		content: <Breadcrumb className="min-w-0" items={breadcrumbItems} />,
	});

	if (slotNavbarCenter) {
		navbarItems.push({
			key: "fixture",
			className: cn(navbarFillClassName, "flex justify-center"),
			content: slotNavbarCenter,
		});
	}

	navbarItems.push({
		key: "search",
		content: <Toggle id="ui.search.toggle" pressed={searchOpen} onPressedChange={setSearchOpen} icon={<Icon icon="search" size={16} />} />,
	});

	navbarItems.push({
		key: "find",
		content: <Toggle id="ui.find.toggle" pressed={findOpen} onPressedChange={setFindOpen} icon={<Icon icon="text-search" size={16} />} />,
	});

	const { t } = useUiTranslation();
	const panelToggleItems = reactHostPort.useMemo<PanelToggleItem[]>(
		() =>
			panelKindsWithTabs.map((kind) => {
				const tabs = panelTabsByKind[kind];
				const icon = panelKindToggleIcon(kind, tabs);
				const side = panelSide(kind);
				const text = resolveTranslationLabel(t(`ui.panelToggle.${kind}` as const));
				if (resolvedMobile) {
					return {
						id: `ui.panelToggle.${kind}`,
						icon,
						text,
						pressed: mobilePanelVisible && activeMobilePanelKind === kind,
						onPressedChange: (pressed: boolean) => openMobilePanel(kind, pressed),
					};
				}
				if (side === "left") {
					return {
						id: `ui.panelToggle.${kind}`,
						icon,
						text,
						pressed: panelVisibility.leftSidePanel && activeDesktopLeftPanelKind === kind,
						onPressedChange: (pressed: boolean) => openDesktopLeftPanel(kind, pressed),
					};
				}
				return {
					id: `ui.panelToggle.${kind}`,
					icon,
					text,
					pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === kind,
					onPressedChange: (pressed: boolean) => openDesktopRightPanel(kind, pressed),
				};
			}),
		[
			activeDesktopLeftPanelKind,
			activeDesktopRightPanelKind,
			activeMobilePanelKind,
			mobilePanelVisible,
			openDesktopLeftPanel,
			openDesktopRightPanel,
			openMobilePanel,
			panelKindsWithTabs,
			panelTabsByKind,
			resolvedMobile,
			t,
		],
	);

	if (panelToggleItems.length > 0) {
		navbarItems.push({
			key: "panelToggles",
			content: <PanelToggleGroup items={panelToggleItems} />,
		});
	}

	navbarItems.push({
		key: "modes",
		content: (
			<ButtonGroup id="platform.navbar.modes">
				{activeAppBase.modes.map((mode) => {
					const isActive = activeModeId === mode.id;
					return (
						<ButtonGroupItem
							key={mode.id}
							id={`platform.navbar.modes.${mode.id}`}
							className={cn(isActive && interactiveActiveFillClass)}
							data-state={isActive ? "on" : undefined}
							onClick={() => {
								activeAppBase.setActiveModeId(mode.id);
								platform.notifyChrome();
							}}
							icon={mode.iconId || <span className="hidden" />}
							text={mode.label}
						/>
					);
				})}
			</ButtonGroup>
		),
	});

	const mergedFooterItems = reactHostPort.useMemo(
		() => mergePlatformFooterChromeRows(platform, activeApp, [...(extraFooterItems ?? [])]),
		[activeApp, extraFooterItems, platform],
	);

	const searchItemsResolved = reactHostPort.useMemo(
		() =>
			resolveCommandPaletteItems(platform, activeApp, activeWindowKindId).map((row) => ({
				id: row.id,
				label: row.label,
				description: row.description,
				category: row.category,
				icon: row.iconId ? resolveElementIcon(row.iconId) : undefined,
				onSelect: () => platform.commandBus.dispatch(row.controllerId, row.command, row.args),
			})),
		[platform, activeApp, activeWindowKindId, shellGen],
	);

	const goldenWindowKinds = reactHostPort.useMemo(
		() => resolvedWindowKindsOverride ?? windowKindsToGolden(activeApp.windowKinds, platform.commandBus),
		[activeApp, resolvedWindowKindsOverride, platform.commandBus],
	);

	const namedLayoutStore = reactHostPort.useMemo(
		() => new NamedLayoutStore(activeApp.id, createBrowserStoragePort()),
		[activeApp.id],
	);

	const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined);

	return (
		<UiChromeCompactProvider compact={uiCompact}>
			<DisplayHostContext.Provider value={displayHost}>
			<SettingsHostContext.Provider value={settingsHostApi}>
			<AppContext.Provider
				value={{
					platform,
					activeAppId,
					setActiveAppId,
					activeApp,
					activeModeId,
					setActiveModeId,
					apps: resolvedApps,
					panelVisibility,
					togglePanel,
					uri: uriProp,
					navigate: onNavigate ?? (() => {}),
					canGoBack: canGoBackProp,
					goBack: onGoBack ?? (() => {}),
					canGoForward: canGoForwardProp,
					goForward: onGoForward ?? (() => {}),
					canGoUp: canGoUpProp,
					goUp: onGoUp ?? (() => {}),
				}}
			>
				<UIFindProvider>
					<ProductFindItemsSync findItems={activeApp.findItems} onFindSelect={activeApp.onFindSelect} />
					<ProductShell
					platform={platform}
					defaultAppId={defaultAppId}
					className={className}
					mobile={resolvedMobile}
					mobileQuery={mobileQuery}
					navbarItems={navbarItems}
					footerItems={mergedFooterItems}
					slotToolbar={toolbarElement}
					leftSidePanelTabs={leftSidePanelTabs}
					rightSidePanelTabs={activeDesktopRightPanelTabs}
					mobilePanel={
						resolvedMobile
							? {
									visible: mobilePanelVisible,
									activeTabId: mobilePanelActiveTabId,
									onActiveTabChange: setMobilePanelActiveTabId,
									tabs: activeMobilePanelTabs,
								}
							: undefined
					}
					panelVisibility={panelVisibility}
					leftPanelSize={leftPanelSize}
					onLeftPanelSizeChange={setLeftPanelSize}
					rightPanelSize={rightPanelSize}
					onRightPanelSizeChange={setRightPanelSize}
					goldenWindowKinds={goldenWindowKinds}
					windowKindCatalog={activeApp.windowKinds}
					namedLayouts={activeApp.namedLayouts}
					namedLayoutStore={namedLayoutStore}
					commandBus={platform.commandBus}
					onDisplayHostReady={handleDisplayHostReady}
					defaultLayout={activeApp.defaultLayout as WindowLayout}
					activeWindowKindId={activeWindowKindId}
					onActiveWindowKindChange={handleActiveWindowChange}
					multiApp
					activeModeId={activeModeId}
					onActiveModeChange={setActiveModeId}
					searchItems={searchItemsResolved}
					searchOpen={searchOpen}
					onSearchOpenChange={setSearchOpen}
					findOpen={findOpen}
					onFindOpenChange={setFindOpen}
					onToggleLastActiveLeftSidePanel={toggleLastActiveLeftSidePanel}
					onToggleLastActiveRightSidePanel={toggleLastActiveRightSidePanel}
				/>
				</UIFindProvider>
			</AppContext.Provider>
			</SettingsHostContext.Provider>
			</DisplayHostContext.Provider>
		</UiChromeCompactProvider>
	);
};

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	function attachTestPanelTabs(app: AppRuntime): void {
		app.panelTabs = [
			{ id: "workbench", iconId: "folder", panel: "workbench", order: 0, bodyKey: "test.platform.panel.workbench", label: "Workbench" },
			{ id: "details", iconId: "info", panel: "details", order: 0, bodyKey: "test.platform.panel.details", label: "Details" },
		];
		registerSidePanelBody("test.platform.panel.workbench", () => ({
			type: "tree",
			sections: [{ id: "workbench", items: [{ id: "item", label: "Workbench" }] }],
		}));
		registerSidePanelBody("test.platform.panel.details", () => ({
			type: "tree",
			sections: [{ id: "details", items: [{ id: "item", label: "Details" }] }],
		}));
	}

	describe("registerSidePanelBody", () => {
		it("rejects non-tree declarative bodies when the factory is invoked", () => {
			const key = "test.side-panel.non-tree";
			registerSidePanelBody(key, () => ({ type: "text", value: "x" }) as UiTreeNode);
			const factory = getSidePanelBodyFactory(key);
			expect(factory).toBeDefined();
			expect(() =>
				factory?.({
					platform: new Platform(),
					windowKindId: "tab",
					bodyKey: key,
					activeModeId: null,
					generation: 0,
				}),
			).toThrow(/must be type "tree"/);
			unregisterSidePanelBody(key);
		});
	});

	describe("sideTabsToPanelTabs", () => {
		it("maps registered declarative bodies to tree definitions", () => {
			const key = "test.side-tabs.declarative";
			registerSidePanelBody(key, () => ({
				type: "tree",
				sections: [{ id: "section", items: [{ id: "item", label: "Panel item" }] }],
			}));
			const wb = new Platform();
			const tabs = sideTabsToPanelTabs([{ id: "tab", iconId: "folder", panel: "workbench", order: 0, bodyKey: key, label: "Tab" }], wb, wb.commandBus);
			expect(tabs).toHaveLength(1);
			expect(tabs[0]?.tree && typeof tabs[0].tree === "object" && "resolveTree" in tabs[0].tree).toBe(true);
			unregisterSidePanelBody(key);
		});

		it("falls back to a missing-panel tree when no factory is registered", () => {
			const wb = new Platform();
			const tabs = sideTabsToPanelTabs(
				[{ id: "tab", iconId: "folder", panel: "workbench", order: 0, bodyKey: "test.side-tabs.missing", label: "Tab" }],
				wb,
				wb.commandBus,
			);
			const tree = tabs[0]?.tree;
			expect(tree && typeof tree === "object" && "sections" in tree).toBe(true);
			if (tree && typeof tree === "object" && "sections" in tree) {
				expect(tree.sections[0]?.items[0]?.label).toContain("Missing panel");
			}
		});
	});

	describe("UIWindowMeasures", () => {
		it("renders compact measure tiles that fill the rail width", () => {
			const markup = renderToStaticMarkup(
				<UIWindowMeasures
					measures={[
						{
							id: "lod",
							kind: "select",
							label: "LOD",
							value: "automatic",
							items: [{ id: "automatic", value: "automatic", label: "Auto (LOD 2)" }],
							onValueChange: () => {},
						},
						{
							id: "auto",
							kind: "toggle",
							label: "LOD",
							icon: <Icon icon="zoom-in" size="small" />,
							text: "Auto zoom",
							pressed: true,
							onPressedChange: () => {},
						},
					]}
				/>,
			);
			expect(markup).toContain('data-slot="window-measures-tree"');
			expect(markup).toContain('data-slot="window-measure-tree-row"');
			expect(markup).toContain("w-full");
			expect(markup).not.toContain("shadow-md");
		});

		it("renders nested measure groups compactly", () => {
			const markup = renderToStaticMarkup(
				<UIWindowMeasures
					measures={[
						{
							id: "brush",
							kind: "group",
							label: "Brush",
							defaultOpen: true,
							children: [
								{
									id: "tolerance",
									kind: "slider",
									label: "Tolerance 0.50",
									value: 0.5,
									min: 0,
									max: 1,
									onValueChange: () => {},
								},
							],
						},
					]}
				/>,
			);
			expect(markup).toContain('data-slot="window-measures-tree"');
			expect(markup).toContain("Brush");
			expect(markup).toContain('data-slot="tree-guide"');
			expect(markup).toContain('data-slot="window-measure-tree-content"');
			expect(markup).toContain('data-slot="slider"');
			expect(markup).not.toContain('data-slot="tree-section-row"');
		});
	});

	describe("windowMeasuresToGolden", () => {
		it("maps nested measure groups recursively", () => {
			const bus = new CommandBus();
			const golden = windowMeasuresToGolden(
				[
					{
						kind: "group",
						id: "brush",
						label: "Brush",
						defaultOpen: false,
						children: [
							{
								kind: "toggle",
								id: "tool",
								iconId: "circle-dot",
								text: "On",
								pressed: true,
								onChange: { controllerId: "test", command: "toggleTool" },
							},
						],
					},
				],
				bus,
			);
			expect(golden?.[0]?.kind).toBe("group");
			if (golden?.[0]?.kind !== "group") {
				return;
			}
			expect(golden[0].defaultOpen).toBe(false);
			expect(golden[0].children[0]?.kind).toBe("toggle");
		});
	});

	describe("navigationTrailToBreadcrumbItems", () => {
		it("maps navigation alternatives to breadcrumb separator options", () => {
			const items = navigationTrailToBreadcrumbItems(
				[
					{
						node: { id: "home", label: "Home", uri: "/" },
						alternatives: [
							{ id: "kits", label: "Kits", uri: "/" },
							{ id: "docs", label: "Documentation", uri: "/docs" },
						],
					},
					{
						node: { id: "kits", label: "Kits", uri: "/" },
						alternatives: [{ id: "k1", label: "Demo", uri: "/kits/k1" }],
					},
				],
				(href) => href,
			);
			expect(items).toHaveLength(2);
			expect(items[0]?.options).toHaveLength(2);
			expect(items[0]?.options?.[0]?.href).toBe("/");
			expect(items[0]?.options?.[1]?.label).toBe("Documentation");
			expect(items[1]?.content).toBe("Kits");
		});
	});

	describe("PlatformView", () => {
		it("opens side panels when PlatformSpec initialPanelVisibility is set", () => {
			const wb = new Platform({
				id: "panels",
				name: "Panels",
				initialPanelVisibility: { leftSidePanel: true, rightSidePanel: true },
			});
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.panel-spec.main"),
			]);
			registerWindowBody("test.panel-spec.main", () => <div>Main</div>);
			attachTestPanelTabs(app);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain('data-panel="leftSidePanel"');
			expect(markup).toContain('data-slot="side-panel-tabs"');
			expect(markup).toContain('data-slot="tree-section-wrapper"');
		});

		it("hides panel toggles when the active app registers no panel tabs", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.notoggles"),
			]);
			registerWindowBody("test.workbench-view.notoggles", () => <div>Main</div>);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} />);
			expect(markup).not.toContain('id="ui.panelToggle.workbench"');
		});

		it("renders breadcrumb navigation for the current uri", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.breadcrumb"),
			]);
			registerWindowBody("test.workbench-view.breadcrumb", () => <div>Main</div>);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} uri="/apps/demo" />);
			expect(markup).toContain('aria-label="breadcrumb"');
			expect(markup).toContain("apps");
		});

		it("groups back, forward, and up in one nav button group", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.nav-group"),
			]);
			registerWindowBody("test.workbench-view.nav-group", () => <div>Main</div>);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} uri="/apps/demo" />);
			expect(markup).toContain('id="ui.nav"');
			expect(markup).toContain('id="ui.nav.back"');
			expect(markup).toContain('id="ui.nav.forward"');
			expect(markup).toContain('id="ui.nav.up"');
			const navGroupCount = (markup.match(/data-slot="button-group"[^>]*id="ui\.nav"/g) ?? []).length;
			expect(navGroupCount).toBe(1);
			expect(markup).not.toMatch(/id="ui\.nav\.back"[^>]*>[\s\S]*data-slot="button-group"[^>]*id="ui\.nav\.back"/);
		});

		it("does not render app switcher tabs when multiple apps are registered", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const mkApp = (id: string) => {
				const app = new AppRuntime(id, id, undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
					new WindowKindRuntime("main", "Main", `test.workbench-view.${id}`),
				]);
				registerWindowBody(`test.workbench-view.${id}`, () => <div>{id}</div>);
				return app;
			};
			wb.addApp(mkApp("home"));
			wb.addApp(mkApp("secondary"));
			const markup = renderToStaticMarkup(<PlatformView platform={wb} uri="/apps/demo" />);
			expect(markup).toContain('aria-label="breadcrumb"');
			expect(markup).not.toContain('id="ui.appNav"');
		});

		it("shows panel toggles only for registered panel kinds", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.main"),
			]);
			registerWindowBody("test.workbench-view.main", () => <div>Main</div>);
			attachTestPanelTabs(app);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain('data-panel="leftSidePanel"');
			expect(markup).toContain('id="ui.panelToggle.workbench"');
			expect(markup).toContain('id="ui.panelToggle.details"');
			expect(markup).toContain('id="ui.panelToggle.settings"');
			expect(markup).toContain('id="ui.panelToggle.display"');
			expect(markup).not.toContain("data-missing-icon");
			expect(markup).toContain('data-icon="layout-grid"');
			expect(markup).toContain('data-icon="folder"');
			expect(markup).toContain('data-icon="info"');
			expect(markup).toContain('data-icon="settings-2"');
		});

		it("renders navbar buttons and toggles with inline labels even when compact chrome is on", () => {
			if (typeof localStorage !== "undefined") {
				localStorage.setItem("ui.chrome.compact", "true");
			}
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.navbar-labels"),
			]);
			registerWindowBody("test.workbench-view.navbar-labels", () => <div>Main</div>);
			attachTestPanelTabs(app);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain("Go back");
			expect(markup).toContain("Search");
			expect(markup).toContain('id="ui.panelToggle.workbench"');
			expect(markup).toContain("Workbench");
			expect(markup).toContain('id="ui.panelToggle.details"');
			expect(markup).toContain("Details");
			expect(markup).toContain("Settings");
		});

		it("renders panel kind icons for unregistered tab iconIds", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.icons"),
			]);
			registerWindowBody("test.workbench-view.icons", () => <div>Main</div>);
			app.panelTabs = [
				{ id: "workbench", iconId: "lucide.folder", panel: "workbench", order: 0, bodyKey: "test.platform.panel.workbench", label: "Workbench" },
				{ id: "details", iconId: "lucide.info", panel: "details", order: 0, bodyKey: "test.platform.panel.details", label: "Details" },
			];
			registerSidePanelBody("test.platform.panel.workbench", () => ({
				type: "tree",
				sections: [{ id: "workbench", items: [{ id: "item", label: "Workbench" }] }],
			}));
			registerSidePanelBody("test.platform.panel.details", () => ({
				type: "tree",
				sections: [{ id: "details", items: [{ id: "item", label: "Details" }] }],
			}));
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain('data-icon="folder"');
			expect(markup).toContain('data-icon="info"');
			expect(markup).not.toContain("data-missing-icon");
		});

		it("merges appwide tools, selection, options, and window kinds with the active mode", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("app", "App", undefined, new TCtrl(), createTabStackLayout(["base"], ["Base"]), [
				new WindowKindRuntime("base", "Base", "test.workbench-view.base"),
			]);
			app.tools = [toolCollection("selection", "mouse-pointer-2", [{ id: "base-tool", kind: "button", iconId: "circle-dot", label: "Base", controllerId: "tctrl", command: "x" }])];
			app.selection = { base: true };
			app.options = { snap: true };
			const inspect = new ModeRuntime("inspect", "Inspect", undefined);
			inspect.tools = [toolCollection("actions", "more-horizontal", [{ id: "mode-tool", kind: "button", iconId: "circle-dot", label: "Mode", controllerId: "tctrl", command: "y" }])];
			inspect.selection = { mode: true };
			inspect.options = { isolate: true };
			inspect.windowKinds = [new WindowKindRuntime("mode", "Mode", "test.workbench-view.mode")];
			app.addMode(inspect);
			app.defaultModeId = "inspect";
			const resolved = app.resolve("inspect");

			expect(resolved.activeModeId).toBe("inspect");
			const selectionCollection = resolved.tools?.find((node) => node.kind === "collection" && node.id === "selection");
			const actionsCollection = resolved.tools?.find((node) => node.kind === "collection" && node.id === "actions");
			expect(selectionCollection?.kind === "collection" ? selectionCollection.children.map((tool) => tool.id) : []).toEqual(["base-tool"]);
			expect(actionsCollection?.kind === "collection" ? actionsCollection.children.map((tool) => tool.id) : []).toEqual(["mode-tool"]);
			expect(resolved.selection).toEqual({ base: true, mode: true });
			expect(resolved.options).toEqual({ snap: true, isolate: true });
			expect(resolved.windowKinds.map((windowKind) => windowKind.id)).toEqual(["base", "mode"]);
		});

		it("always renders the product footer strip even with no footer items", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("test", "Test", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.footer.main"),
			]);
			registerWindowBody("test.workbench-view.footer.main", () => <div>Main</div>);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} />);

			expect(markup).toContain('data-slot="footer"');
		});

		it("renders a leading mode dropdown when an app has multiple modes", () => {
			const wb = new Platform();
			class TCtrl extends Controller {
				constructor() {
					super("tctrl", wb.commandBus, () => wb.notify());
				}
				run(): void {}
			}
			const app = new AppRuntime("app", "App", undefined, new TCtrl(), createTabStackLayout(["main"], ["Main"]), [
				new WindowKindRuntime("main", "Main", "test.workbench-view.mm.main"),
			]);
			registerWindowBody("test.workbench-view.mm.main", () => <div>Main</div>);
			app.addMode(new ModeRuntime("inspect", "Inspect", undefined));
			app.addMode(new ModeRuntime("edit", "Edit", undefined));
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} />);

			expect(markup).toContain('data-slot="app-name"');
			expect(markup).toContain(">App</span>");
			expect(markup).toContain('id="platform.navbar.modes.inspect"');
			expect(markup).toContain('id="platform.navbar.modes.edit"');
		});
	});
}
//#endregion ­ƒº¬Tests

//#endregion ­ƒôªworkbench-view.tsx

//#region 🔖PlatformShell
/** @emoji 🌓 Fixed surface chrome for every product shell (system theme, desktop device). */
export const PLATFORM_SYSTEM_SURFACE_CHROME = {
	theme: "system" as const,
	device: "desktop" as const,
	expertise: Expertise.NORMAL,
};

/** @emoji 🛝 Applies system theme, level chrome, and full-viewport layout for {@link PlatformView}. */
export function PlatformShell({ children, compact }: { readonly children: React.ReactNode; readonly compact?: boolean }): React.ReactElement {
	const resolvedCompact = compact ?? readStoredUiChromeCompact();
	useElementsSurfaceChrome({ ...PLATFORM_SYSTEM_SURFACE_CHROME, compact: resolvedCompact });
	return (
		<UiChromeCompactProvider compact={resolvedCompact}>
			<LevelProvider level="window">
				<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>{children}</div>
			</LevelProvider>
		</UiChromeCompactProvider>
	);
}
//#endregion 🔖PlatformShell

//#region ­ƒôªworkbench-mount.tsx
type ElementsDomRoot = HTMLElement & { __elementsReactRoot?: Root };

function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
	return document.getElementById(id) as T | null;
}

/** @emoji ÔÜø´©Å Imperative React root helpers for workbench shells. */
export class ReactUI {
	private static mountedRoot: Root | null = null;

	/** @emoji 🖥️ Mounts a {@link Platform} shell into `#root` (or `rootId`) with {@link PlatformView}. */
	static mount(platform: Platform, rootId = "root"): void {
		if (typeof document === "undefined") return;
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		if (!rootElement) {
			throw new Error(`React root #${rootId} missing.`);
		}
		rootElement.__elementsReactRoot ??= createRoot(rootElement);
		ReactUI.mountedRoot = rootElement.__elementsReactRoot;
		rootElement.__elementsReactRoot.render(
			<PlatformShell>
				<PlatformViewWithHistory platform={platform} />
			</PlatformShell>,
		);
	}

	static unmount(rootId = "root"): void {
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		rootElement?.__elementsReactRoot?.unmount();
		if (rootElement) {
			delete rootElement.__elementsReactRoot;
		}
		ReactUI.mountedRoot = null;
	}
}

/** @emoji ­ƒûÑ´©Å Mounts an arbitrary React tree into `#root` (or `rootId`). */
export function mountReactApp(element: React.ReactElement, rootId = "root"): void {
	if (typeof document === "undefined") return;
	const rootElement = getElementById<ElementsDomRoot>(rootId);
	if (!rootElement) {
		throw new Error(`React root #${rootId} missing.`);
	}
	rootElement.__elementsReactRoot ??= createRoot(rootElement);
	rootElement.__elementsReactRoot.render(element);
}

/** @emoji 🖥️ Loads a {@link Platform} asynchronously then mounts {@link PlatformView}. */
export async function mountAsyncReactApp(loadRuntime: () => Promise<Platform>, rootId = "root"): Promise<void> {
	ReactUI.mount(await loadRuntime(), rootId);
}

/** @emoji 🖥️ Alias for {@link mountAsyncReactApp} — mounts a product {@link Platform} shell. */
export const mountPlatform = mountAsyncReactApp;

//#endregion ­ƒôªworkbench-mount.tsx
