// #region 🧲Header
/** @emoji ⚛️ `@framework/platform/renderer/react` — React renderer for {@link @framework/platform/core}: {@link ProductShell}, {@link PlatformView}, declarative {@link UiNode} host (monolith). */
// #endregion 🧲Header

export {
	Platform,
	Store,
	APP_TOOL_CATEGORY_ORDER,
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
	type AppToolCategory,
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
} from "@framework/platform/core";

export type { Level } from "@ui/react";
export {
	LevelProvider,
	useLevel,
	getLevelBgClass,
	getLevelHoverClass,
	getLevelActiveHoverClass,
	getLevelZClass,
	getLevelBorderElementClass,
	getLevelDivideElementClass,
} from "@ui/react";

// #region 🔌Adapters
import {
	APP_TOOL_CATEGORY_ORDER,
	countAppTools,
	CommandBus,
	Controller,
	Store,
	Platform,
	resolveInitialPanelVisibility,
	LEFT_PANEL_KINDS,
	RIGHT_PANEL_KINDS,
	PANEL_KINDS,
	panelSide,
	type PanelKind,
	type PlatformBreadcrumbItem,
	AppRuntime,
	ModeRuntime,
	resolveCommandPaletteItems,
	WindowKindRuntime,
	createTabStackLayout,
	createWindowLayout,
	getSidePanelBodyFactory,
	getWindowBodyFactory,
	type ResolvedAppState,
	type AppTools as FrameworkAppTools,
	type FooterItem as DeclarativeFooterItem,
	type SidePanelBodyViewContext,
	type SideTabSpec,
	type ToolItem,
	type WindowBodyViewContext,
	type WindowMeasure,
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
	type UiVirtualFileSystemHostSurfaceNode,
	type UiTextNode,
	getPlatformControllerById,
	platformTopologyStoreId,
	registerPlatformVirtualFileSystemDemo,
	PlatformVirtualFileSystemDemoController,
	virtualFileSystemSurfaceId,
	PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
} from "@framework/platform/core";
import {
	ArrowLeft,
	ArrowRight,
	ArrowUp,
	ArrowRightLeft as ArrowRightLeftIcon,
	Check as CheckIcon,
	Filter as FilterIcon,
	Folder,
	FolderOpen as FolderOpenIcon,
	Hand as HandIcon,
	Info,
	Lasso as LassoIcon,
	LayoutGrid as LayoutGridIcon,
	MessageSquare,
	Minimize2,
	MoreHorizontal as MoreHorizontalIcon,
	MousePointer2 as MousePointerIcon,
	Move3d as Move3dIcon,
	Plus as PlusIcon,
	Save as SaveIcon,
	Search,
	Search as SearchIcon,
	Settings2,
	Settings2 as Settings2Icon,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import * as React from "react";
import { createRoot, type Root } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import Fuse, { type FuseResult } from "fuse.js";
import { Puzzle2dCanvas, parsePuzzle2dFixtureV1, type Puzzle2dSelectionSnapshot } from "@puzzle/2d/react";
import { parseFixtureV1, type SelectionSnapshot as Puzzle3dSelectionSnapshot } from "@puzzle/3d/react";
import { FiveD, StoreProvider, compose5d, createStore } from "@puzzle/5d/react";
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
	Ui,
	type EngagementSpec,
	type ModeWindowDescriptor,
	type WindowLayoutNode as ShellWindowLayoutNode,
	cn,
	resolveTranslationLabel,
	useUiTranslation,
	useCommandHotkey,
	useMediaQuery,
	type ContextMenuItem,
	type NavbarItem,
	Expertise,
	LevelProvider,
	getLevelBgClass,
	readStoredUiChromeCompact,
	UiChromeCompactProvider,
	useElementsSurfaceChrome,
	writeStoredUiChromeCompact,
	reactHostPort,
	VirtualFileSystem as VirtualFileSystemView,
	type VirtualFileSystemRow,
	type VirtualFileSystemSchema,
	windowMeasureControlClass,
	windowMeasureLabelClass,
	windowMeasureSectionClass,
	windowMeasureTileClass,
	windowMeasureToggleClass,
	type AssertUiToolbarParentKeysCovered,
} from "@ui/react";
// #endregion 🔌Adapters

import type { AppToolCategory } from "@framework/core";

const _assertFrameworkToolbarParentKeys: AssertUiToolbarParentKeysCovered<AppToolCategory> = true;

//#region 📦shell-chrome-types.tsx

/** @emoji 👣 Footer row rendered by the product shell. */
export interface ChromeFooterRow {
	readonly id: string;
	readonly icon?: React.ReactNode;
	readonly text?: string;
	readonly content?: React.ReactNode;
	readonly order?: number;
	readonly onClick?: () => void;
	readonly className?: string;
	readonly disabled?: boolean;
}

/** @emoji 🌲 Minimal tree panel payload for declarative side tabs. */
export interface ChromeTreePanelConfig {
	readonly sections: readonly { readonly id: string; readonly content: React.ReactNode }[];
}

/** @emoji 📑 Side panel tab registration consumed by {@link PlatformView}. */
export interface SidePanelTabConfig {
	readonly id: string;
	readonly icon: React.ComponentType<{ readonly size?: number }>;
	readonly order?: number;
	readonly tree: ChromeTreePanelConfig;
}

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
  | { kind: "toggle"; id: string; label?: string; pressed?: boolean; defaultPressed?: boolean; icon?: React.ReactNode; text?: string; onPressedChange?: (pressed: boolean) => void }
  | { kind: "select"; id: string; label?: string; value?: string; defaultValue?: string; items: { id: string; value: string; label: string }[]; onValueChange?: (value: string) => void }
  | { kind: "combobox"; id: string; label?: string; value?: string; placeholder?: string; choices: { value: string; label: string }[]; onValueChange?: (value: string) => void }
  | { kind: "button"; id: string; label?: string; text: string; icon?: React.ReactNode; onClick?: () => void }
  | { kind: "buttonCycle"; id: string; label?: string; value?: string; items: { value: string; label: string; icon?: React.ReactNode; text?: string; id?: string }[]; onValueChange?: (value: string) => void }
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

/**
 * 📐 Maps declarative `UIWindowMeasure` entries into compact floating tiles aligned to the right edge.
 **/
export const UIWindowMeasures: React.FC<{ measures: UIWindowMeasure[] }> = ({ measures }) => (
  <div data-slot="window-measures-stack-inner" className="flex w-full min-w-0 flex-col gap-half">
    {measures.map((measure) => {
      switch (measure.kind) {
        case "display":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <div className="text-foreground max-w-full text-xs leading-snug break-words">{measure.content}</div>
            </UIWindowMeasureFloat>
          );
        case "reading":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <div className={cn("text-foreground text-xs tabular-nums", measure.monospace && "font-mono")}>{measure.text}</div>
            </UIWindowMeasureFloat>
          );
        case "section":
          return (
            <span key={measure.id} data-slot="window-measure-heading" className={windowMeasureSectionClass}>
              {measure.title}
            </span>
          );
        case "separator":
          return <div key={measure.id} data-slot="window-measure-separator" className="bg-muted-foreground/35 my-half h-px w-8 shrink-0 rounded-full" aria-hidden />;
        case "toggle":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Toggle
                id={measure.id}
                className={windowMeasureToggleClass}
                pressed={measure.pressed}
                defaultPressed={measure.defaultPressed}
                onPressedChange={measure.onPressedChange}
                icon={measure.icon ?? <CheckIcon className="size-small" />}
                text={measure.text}
              />
            </UIWindowMeasureFloat>
          );
        case "select":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Select id={measure.id} value={measure.value} defaultValue={measure.defaultValue} onValueChange={measure.onValueChange}>
                <SelectTrigger id={measure.id} className="h-medium w-full min-w-0" size="sm">
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
            </UIWindowMeasureFloat>
          );
        case "combobox":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Combobox id={measure.id} value={measure.value} options={measure.choices} placeholder={measure.placeholder} onValueChange={measure.onValueChange} className={windowMeasureControlClass} />
            </UIWindowMeasureFloat>
          );
        case "button":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Button id={measure.id} text={measure.text} icon={measure.icon} onClick={measure.onClick} />
            </UIWindowMeasureFloat>
          );
        case "buttonCycle":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <ButtonCycle id={measure.id} value={measure.value} onValueChange={measure.onValueChange} items={measure.items} />
            </UIWindowMeasureFloat>
          );
        case "input":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Input id={measure.id} lazy className={cn("h-medium", windowMeasureControlClass)} value={measure.value} placeholder={measure.placeholder} onLazyChange={measure.onLazyChange} />
            </UIWindowMeasureFloat>
          );
        case "textarea":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Textarea id={measure.id} lazy className={cn("min-h-[4rem]", windowMeasureControlClass)} value={measure.value} placeholder={measure.placeholder} rows={measure.rows} onLazyChange={measure.onLazyChange} />
            </UIWindowMeasureFloat>
          );
        case "checkbox":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id}>
              <div className="text-foreground flex w-full min-w-0 items-center gap-single text-xs">
                <input
                  id={measure.id}
                  type="checkbox"
                  className="border-element accent-foreground size-small shrink-0 rounded border"
                  {...(measure.checked !== undefined ? { checked: measure.checked } : { defaultChecked: measure.defaultChecked })}
                  onChange={(event) => measure.onCheckedChange?.(event.target.checked)}
                />
                {measure.label ? (
                  <label htmlFor={measure.id} className="cursor-pointer select-none">
                    {measure.label}
                  </label>
                ) : null}
              </div>
            </UIWindowMeasureFloat>
          );
        case "radio":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <div className="flex flex-col gap-half" role="radiogroup" aria-labelledby={measure.id}>
                {measure.items.map((item) => (
                  <button
                    key={item.value}
                    type="button"
                    data-slot="window-measure-radio-item"
                    className={cn(
                      "border-element/80 hover:bg-hover-window w-full rounded border px-single py-half text-left text-xs transition-colors",
                      measure.value === item.value && "bg-active-base text-active-foreground",
                    )}
                    onClick={() => measure.onChange?.(item.value)}
                  >
                    {item.label}
                  </button>
                ))}
              </div>
            </UIWindowMeasureFloat>
          );
        case "slider": {
          const min = measure.min ?? 0;
          const max = measure.max ?? 100;
          const v = measure.value ?? min;
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Slider id={measure.id} value={[v]} min={min} max={max} step={measure.step} onValueChange={(vals) => measure.onValueChange?.(vals[0] ?? min)} />
            </UIWindowMeasureFloat>
          );
        }
        case "number":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Stepper id={measure.id} value={measure.value} min={measure.min} max={measure.max} step={measure.step} onChange={measure.onChange} />
            </UIWindowMeasureFloat>
          );
        case "color":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Input id={measure.id} type="color" className={cn("h-medium cursor-pointer", windowMeasureControlClass)} value={measure.value} onChange={(event) => measure.onChange?.(event.target.value)} />
            </UIWindowMeasureFloat>
          );
        default: {
          const _exhaustive: never = measure;
          return _exhaustive;
        }
      }
    })}
  </div>
);

// #endregion 🪟WindowMeasuresOverlay

function convertFrameworkLayoutNodeToShellLayout(node: WindowLayoutNode): ShellWindowLayoutNode {
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => ({ kind: "window", id: child.windowKindId, title: child.title })),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToShellLayout(child)),
  };
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
    <ContextMenu items={windowKind.contextMenu}>
      <div className="flex min-h-0 min-w-0 flex-1 flex-col">
        <WindowComponent />
      </div>
    </ContextMenu>
  );
  cache.set(windowKind.id, { component: windowKind.component, contextMenu: windowKind.contextMenu, body });
  return body;
}

/** @emoji 🪟 Pure-React resizable mode canvas backed by {@link Mode}. */
export const ShellModeCanvas: React.FC<{
  windowKinds: UIWindowKindDefinition[];
  defaultLayout: WindowLayout;
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string) => void;
}> = ({ windowKinds, defaultLayout, activeWindowId, onActiveWindowChange }) => {
  const windowBodyCacheRef = reactHostPort.useRef(new Map<string, ShellModeWindowBodyCacheEntry>());
  reactHostPort.useLayoutEffect(() => {
    const liveIds = new Set(windowKinds.map((windowKind) => windowKind.id));
    for (const windowKindId of windowBodyCacheRef.current.keys()) {
      if (!liveIds.has(windowKindId)) windowBodyCacheRef.current.delete(windowKindId);
    }
  }, [windowKinds]);
  const windows = reactHostPort.useMemo<ModeWindowDescriptor[]>(
    () =>
      windowKinds.map((windowKind) => ({
        id: windowKind.id,
        title: windowKind.label,
        showControls: true,
        controls: windowKind.controls ? <UIWindowControlsGroup controls={windowKind.controls} /> : undefined,
        measures: windowKind.measures?.length ? <UIWindowMeasures measures={windowKind.measures} /> : undefined,
        engagement: windowKind.engagement ?? windowControlsToEngagement(windowKind.controls),
        children: resolveShellModeWindowBody(windowBodyCacheRef.current, windowKind),
      })),
    [windowKinds],
  );
  const shellLayout = reactHostPort.useMemo(() => convertFrameworkLayoutToShellLayout(defaultLayout), [defaultLayout]);

  return (
    <Mode
      windows={windows}
      layout={shellLayout}
      activeWindowId={activeWindowId}
      onActiveWindowChange={onActiveWindowChange}
      className="h-full w-full"
    />
  );
};

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
}> = ({ items, open, onOpenChange, placeholder = "Search...", emptyMessage = "No results found." }) => {
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
    <CommandDialog title="Search" description="Search for items..." open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
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
}> = ({ open, onOpenChange, placeholder = "Find...", emptyMessage = "No results found." }) => {
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
    <CommandDialog title="Find" description="Find items in this app..." open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
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

/**
 * A toolbar action item registered by an app or the UI.
 **/
export interface UIToolbarItem {
  id: string;
  icon?: React.ReactNode;
  label?: string;
  text?: string;
  onClick?: () => void;
  kind?: "button" | "toggle" | "separator";
  pressed?: boolean;
  onPressedChange?: (pressed: boolean) => void;
  order?: number;
}

/** @emoji 🗂️ Per-category toolbar tools registered by an app or global UI shell (React view layer). */
export type ToolbarViewTools = Partial<Record<AppToolCategory, UIToolbarItem[]>>;

function sortToolbarItems(items: readonly UIToolbarItem[]): UIToolbarItem[] {
  return [...items].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

function hasAppToolCategoryItems(items: readonly UIToolbarItem[] | undefined): boolean {
  return Boolean(items?.some((item) => item.kind !== "separator"));
}

/** @emoji 🔢 Counts registered toolbar items across all populated categories. */
export function countToolbarViewTools(tools?: ToolbarViewTools): number {
  if (!tools) return 0;
  return APP_TOOL_CATEGORY_ORDER.reduce((sum, category) => sum + (tools[category]?.length ?? 0), 0);
}

/** @emoji 🔀 Merges base and extension tool maps per category (extension appends within each category). */
export function mergeToolbarViewTools(base?: ToolbarViewTools, extension?: ToolbarViewTools): ToolbarViewTools | undefined {
  if (!base && !extension) return undefined;
  const merged: ToolbarViewTools = {};
  for (const category of APP_TOOL_CATEGORY_ORDER) {
    const combined = [...(base?.[category] ?? []), ...(extension?.[category] ?? [])];
    if (combined.length > 0) merged[category] = combined;
  }
  return Object.keys(merged).length > 0 ? merged : undefined;
}

/** @emoji 📂 Lists categories that have at least one non-separator tool. */
export function listPopulatedToolbarViewCategories(tools?: ToolbarViewTools): AppToolCategory[] {
  if (!tools) return [];
  return APP_TOOL_CATEGORY_ORDER.filter((category) => hasAppToolCategoryItems(tools[category]));
}

function resolveAppToolCategoryIcon(category: AppToolCategory): React.ReactNode {
  switch (category) {
    case "hand":
      return <HandIcon className="size-tiny" aria-hidden />;
    case "selection":
      return <MousePointerIcon className="size-tiny" aria-hidden />;
    case "lasso":
      return <LassoIcon className="size-tiny" aria-hidden />;
    case "filter":
      return <FilterIcon className="size-tiny" aria-hidden />;
    case "open":
      return <FolderOpenIcon className="size-tiny" aria-hidden />;
    case "save":
      return <SaveIcon className="size-tiny" aria-hidden />;
    case "transfer":
      return <ArrowRightLeftIcon className="size-tiny" aria-hidden />;
    case "transform":
      return <Move3dIcon className="size-tiny" aria-hidden />;
    case "create":
      return <PlusIcon className="size-tiny" aria-hidden />;
    case "view":
      return <LayoutGridIcon className="size-tiny" aria-hidden />;
    case "actions":
      return <MoreHorizontalIcon className="size-tiny" aria-hidden />;
    case "settings":
      return <Settings2Icon className="size-tiny" aria-hidden />;
    default:
      return <SearchIcon className="size-tiny" aria-hidden />;
  }
}

const UIToolbarItems: React.FC<{ items: readonly UIToolbarItem[] }> = ({ items }) => {
  const sorted = reactHostPort.useMemo(() => sortToolbarItems(items), [items]);
  const nodes = reactHostPort.useMemo(() => {
    const rendered: React.ReactNode[] = [];
    let buttonRun: UIToolbarItem[] = [];
    let toggleRun: UIToolbarItem[] = [];

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

/**
 * Renders a floating toolbar with category toggles; only categories with registered tools are shown.
 **/
const UIToolbar: React.FC<{
  tools: ToolbarViewTools;
  className?: string;
}> = ({ tools, className }) => {
  const { t } = useUiTranslation();
  const populatedCategories = reactHostPort.useMemo(() => listPopulatedToolbarViewCategories(tools), [tools]);
  const [activeCategory, setActiveCategory] = reactHostPort.useState<AppToolCategory | null>(null);

  reactHostPort.useEffect(() => {
    if (populatedCategories.length === 0) {
      setActiveCategory(null);
      return;
    }
    setActiveCategory((previousValue) => {
      if (previousValue && populatedCategories.includes(previousValue)) return previousValue;
      return populatedCategories.find((category) => category !== "history" && category !== "hand") ?? populatedCategories[0] ?? null;
    });
  }, [populatedCategories]);

  if (populatedCategories.length === 0) return null;

  const activeItems = activeCategory ? (tools[activeCategory] ?? []) : [];
  const showCategoryNav = populatedCategories.length > 1;

  return (
    <div className={cn("flex items-center justify-center pointer-events-none", className)}>
      <div
        role="toolbar"
        id="ui.toolbar"
        className={cn("pointer-events-auto flex max-w-full items-stretch gap-single", showCategoryNav && "w-full max-w-[min(100%,48rem)] px-2")}
      >
        {showCategoryNav ? (
          <>
            <ToolbarZone id="ui.toolbar.zone.categories" className="shrink-0">
              <ToggleGroup
                kind="single"
                value={activeCategory ?? ""}
                onValueChange={(value) => setActiveCategory(value ? (value as AppToolCategory) : null)}
                items={populatedCategories.map((category) => ({
                  value: category,
                  id: `ui.toolbar.group.${category}`,
                  icon: resolveAppToolCategoryIcon(category),
                  text: resolveTranslationLabel(t(`ui.toolbar.parent.${category}` as const)),
                }))}
              />
            </ToolbarZone>
            {activeCategory && hasAppToolCategoryItems(activeItems) ? (
              <ToolbarZone id="ui.toolbar.zone.tools" className="min-w-0 flex-1">
                <UIToolbarItems items={activeItems} />
              </ToolbarZone>
            ) : null}
          </>
        ) : (
          <ToolbarZone className="max-w-full">
            <UIToolbarItems items={tools[populatedCategories[0]!] ?? []} />
          </ToolbarZone>
        )}
      </div>
    </div>
  );
};

export { UISearch, UIFind, UIToolbar };
export { App, Mode, Ui } from "@ui/react";

// #endregion 📔UIToolbar

//#region 🧪ShellCanvasTests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

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

		it("merges categorized tools and omits empty categories", () => {
			expect(mergeToolbarViewTools({ selection: [{ id: "a", onClick: () => undefined }] }, { filter: [{ id: "b", onClick: () => undefined }] })).toEqual({
				selection: [{ id: "a", onClick: expect.any(Function) }],
				filter: [{ id: "b", onClick: expect.any(Function) }],
			});
			expect(listPopulatedToolbarViewCategories({ selection: [], filter: [{ id: "b", onClick: () => undefined }] })).toEqual(["filter"]);
			expect(listPopulatedToolbarViewCategories({ filter: [{ id: "sep", kind: "separator" }] })).toEqual([]);
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
	return store ? useStore(store) : undefined;
}

function virtualFileSystemSchemaFromModel(schema: VirtualFileSystemSchemaModel): VirtualFileSystemSchema {
	return schema as VirtualFileSystemSchema;
}

const BuiltinVirtualFileSystemKindRenderer: ComponentKindRenderer = ({ component, platform, commandBus }) => {
	const model = useStore(component as VirtualFileSystemSurface);
	const controllerId = component.controllerId;
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
				onToggleExpand={(rowId) => {
					if (!platform) return;
					const vfs = component as VirtualFileSystemSurface;
					platform.commandBus.dispatch(controllerId, "toggleVirtualFileSystemExpand", {
						appId: vfs.appId,
						nodeId: rowId,
						surfaceId: component.surfaceId,
					});
				}}
				onRowClick={(row, _index, event) => {
					if (!platform) return;
					const vfs = component as VirtualFileSystemSurface;
					if (event.metaKey || event.ctrlKey) {
						platform.commandBus.dispatch(controllerId, "toggleVirtualFileSystemRowSelection", {
							appId: vfs.appId,
							rowId: row.id,
							surfaceId: component.surfaceId,
						});
						return;
					}
					if (row.navigateUri && platform.onNavigate) {
						platform.onNavigate(row.navigateUri);
						return;
					}
					platform.commandBus.dispatch(controllerId, "toggleVirtualFileSystemRowSelection", {
						appId: vfs.appId,
						rowId: row.id,
						surfaceId: component.surfaceId,
					});
				}}
				dragDrop={
					model.dragDropEnabled
						? {
								enabled: true,
								canDrag: (rowId) => rowId !== rows[0]?.id,
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
											active ? "text-foreground" : "text-muted-foreground",
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

function usePlatformTopologyStore(
	controller: Controller | undefined,
	instanceId: string,
): ReturnType<typeof createStore> | null {
	const payload = useControllerStore<PlatformTopologyPayload>(controller, platformTopologyStoreId(instanceId));
	const [topologyStore, setTopologyStore] = React.useState<ReturnType<typeof createStore> | null>(null);
	React.useEffect(() => {
		if (!payload) {
			setTopologyStore(null);
			return;
		}
		const model = compose5d(parsePuzzle2dFixtureV1(payload.flat)!, parseFixtureV1(payload.volume)!);
		setTopologyStore((previous) => {
			if (previous) {
				previous.replaceModel(model);
				return previous;
			}
			return createStore(model);
		});
	}, [payload]);
	return topologyStore;
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
						onSelect: (snapshot: Puzzle2dSelectionSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dSelection", puzzle5dSelectionPayload(instanceId, "flat", snapshot));
						},
					}
				: undefined,
		[commandBus, component.controllerId, instanceId, model.presentation],
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
				<FiveD mode={fiveDMode} instanceId={instanceId} puzzle2d={puzzle2dSelect} puzzle3d={puzzle3dSelect} />
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
	return (
		<button
			type="button"
			id={node.id}
			className={cn(
				"rounded-md border px-2 py-1 text-sm",
				variant === "danger" && "border-destructive text-destructive",
				variant === "success" && "border-green-600 text-green-700",
				variant === "subtle" && "border-transparent bg-muted/60",
				variant === "default" && "border-border bg-background",
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
							node.children.some((c) => c.type === "puzzle2d" || c.type === "puzzle3d" || c.type === "puzzle5d" || c.type === "cad") &&
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
			expect(markupA).toContain("Alpha Workspace");
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
			expect(markupB).toContain("Beta Workspace");
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
const elementIconNodes = new Map<string, React.ReactNode>();

/** @emoji ­ƒû╝ Registers a static icon node resolved by `iconId` for toolbars, footers, and tabs. */
export function registerElementIcon(iconId: string, node: React.ReactNode): void {
	elementIconNodes.set(iconId, node);
}

/** @emoji ­ƒöì Returns a registered element icon node for navbar/search rows. */
export function resolveElementIcon(iconId: string): React.ReactNode | undefined {
	return elementIconNodes.get(iconId);
}

const shellTabIcons = new Map<string, LucideIcon>();

const PANEL_KIND_LUCIDE: Record<PanelKind, LucideIcon> = {
	windows: LayoutGridIcon,
	overview: FolderOpenIcon,
	workbench: Folder,
	details: Info,
	settings: Settings2,
	chat: MessageSquare,
};

/** @emoji 🖼️ Ready-made Lucide icon for a {@link PanelKind} navbar toggle or tab fallback. */
function renderPanelKindIcon(kind: PanelKind, size = 16): React.ReactNode {
	const Icon = PANEL_KIND_LUCIDE[kind];
	return Icon ? <Icon size={size} /> : null;
}

/** @emoji 🔍 Resolves a tab icon from registry, then falls back to the panel kind default. */
function resolveTabIconNode(iconId: string, panelKind: PanelKind, size = 16): React.ReactNode {
	const node = elementIconNodes.get(iconId);
	if (node) {
		return (
			<span className="inline-flex items-center justify-center" style={{ width: size, height: size }}>
				{node}
			</span>
		);
	}
	const Lucide = shellTabIcons.get(iconId);
	if (Lucide) return <Lucide size={size} />;
	return renderPanelKindIcon(panelKind, size);
}

/** @emoji ­ƒû� Registers a Lucide icon constructor for side-panel tab headers keyed by `iconId`. */
export function registerTabIcon(iconId: string, Icon: LucideIcon): void {
	shellTabIcons.set(iconId, Icon);
}

const windowBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji ­ƒ¬ƒ Binds a `bodyKey` from {@link WindowKindRuntime} to a React window body component. */
export function registerWindowBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	windowBodyByKey.set(bodyKey, Component);
}

const sidePanelBodyByKey = new Map<string, React.ComponentType<unknown>>();

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

/** @emoji 📑 Binds a `bodyKey` from {@link SideTabSpec} to a React panel body component. */
export function registerSidePanelBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	sidePanelBodyByKey.set(bodyKey, Component);
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
			return <UiRenderer node={node} commandBus={platform.commandBus} platform={platform} />;
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

/** @emoji 📐 Maps {@link WindowMeasure} controller rows to {@link UIWindowMeasure} tiles for {@link ShellModeCanvas}. */
export function windowMeasuresToGolden(measures: readonly WindowMeasure[], bus: CommandBus): UIWindowMeasure[] | undefined {
	if (!measures.length) return undefined;
	return measures.map((measure) => {
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
				onPressedChange: (pressed: boolean) => bus.dispatch(measure.onChange.controllerId, measure.onChange.command, { ...(measure.onChange.args as object | undefined), pressed }),
			};
		}
		return { id: measure.id, kind: "display", content: null };
	});
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

function shellTabIconComponent(iconId: string, panelKind: PanelKind): React.ComponentType<{ size?: number }> {
	return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
		return <>{resolveTabIconNode(iconId, panelKind, size)}</>;
	};
}

/** @emoji ­ƒôæ Converts framework side tabs into panel tab configs. */
export function sideTabsToPanelTabs(tabs: readonly SideTabSpec[], bus: CommandBus): SidePanelTabConfig[] {
	void bus;
	return tabs.map((tab, orderIndex) => {
		const declarativeFactory = getSidePanelBodyFactory(tab.bodyKey);
		const Body = declarativeFactory
			? getDeclarativeSidePanelBodyComponent(tab.id, tab.bodyKey)
			: (sidePanelBodyByKey.get(tab.bodyKey) ?? (() => <div className="p-2 text-xs">Missing panel {tab.bodyKey}</div>));
		return {
			id: tab.id,
			icon: shellTabIconComponent(tab.iconId, tab.panel),
			order: tab.order ?? orderIndex,
			tree: { sections: [{ id: `${tab.id}.body`, content: <Body /> }] },
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
		icon: item.iconId ? elementIconNodes.get(item.iconId) : undefined,
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

function shellToolToToolbarItem(item: ToolItem, bus: CommandBus): UIToolbarItem {
	if (item.kind === "separator") {
		return { id: item.id, kind: "separator", order: item.order };
	}
	const iconNode = item.iconId ? elementIconNodes.get(item.iconId) : undefined;
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

/** @emoji 🎛 Converts declarative {@link FrameworkAppTools} into mounted toolbar items. */
export function declareToolsToViewTools(tools: FrameworkAppTools | undefined, bus: CommandBus): ToolbarViewTools | undefined {
	if (!tools) return undefined;
	const merged: ToolbarViewTools = {};
	for (const category of APP_TOOL_CATEGORY_ORDER) {
		const list = tools[category];
		if (!list?.length) continue;
		merged[category] = list.map((entry) => shellToolToToolbarItem(entry, bus));
	}
	return Object.keys(merged).length > 0 ? merged : undefined;
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
	const resolvedFindItems = findItems ?? [];
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
	bus: CommandBus,
	augment?: Partial<Record<PanelKind, SidePanelTabConfig[]>>,
): Record<PanelKind, SidePanelTabConfig[]> {
	const grouped = new Map<PanelKind, SideTabSpec[]>();
	for (const kind of PANEL_KINDS) grouped.set(kind, []);
	for (const tab of app.panelTabs) {
		grouped.get(tab.panel)?.push(tab);
	}
	const result = {} as Record<PanelKind, SidePanelTabConfig[]>;
	for (const kind of PANEL_KINDS) {
		const resolved = sideTabsToPanelTabs(grouped.get(kind) ?? [], bus);
		result[kind] = mergeConfigEntries(resolved, augment?.[kind]) ?? resolved;
	}
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

function platformBreadcrumbToUiItems(items: readonly PlatformBreadcrumbItem[], onNavigate: (href: string) => void): BreadcrumbItemData[] {
	return items.map((item, index) => ({
		id: item.id ?? `breadcrumb.${index}`,
		content: item.content as React.ReactNode,
		options: item.options,
		onNavigate: item.onNavigate ?? onNavigate,
	}));
}

function readBrowserUri(): string {
	if (typeof window === "undefined") return "/";
	return `${window.location.pathname}${window.location.search}`;
}

/**
 * Left panel toggle for the navbar.
 * Uses the first tab icon as the toggle icon.
 * Panel toggle strip: border border-element, h-medium.
 **/
const UIPanelToggleGroup: React.FC<{
  items: Array<{
    icon: React.ReactNode;
    id: string;
    onPressedChange: (pressed: boolean) => void;
    pressed: boolean;
  }>;
}> = ({ items }) => (
  <div data-slot="app-panel-toggle-group" className="flex items-stretch border border-element overflow-hidden h-medium">
    {items.map((item, index) => (
      <Toggle
        key={item.id}
        id={item.id}
        pressed={item.pressed}
        onPressedChange={item.onPressedChange}
        className={cn("border-0 rounded-none", index > 0 && "border-l")}
        icon={item.icon}
      />
    ))}
  </div>
);

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

	const activeApp = activeAppBase.resolve(activeModeId ?? activeAppBase.getActiveModeId());

	const canvasNode = multiApp ? (
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
	);

	return (
		<>
			<Layout
				className={className}
				mobile={resolvedMobile}
				navbar={<Navbar items={navbarItems} />}
				footer={<Footer items={footerItems ?? []} />}
				toolbar={slotToolbar}
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
const PlatformViewWithHistory: React.FC<Omit<PlatformViewProps, "uri" | "onNavigate" | "canGoBack" | "onGoBack" | "canGoForward" | "onGoForward" | "canGoUp" | "onGoUp">> = ({
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
 * Fixed navbar layout: [mode (if >1 mode)] [back] [forward] [up] [breadcrumb (flex-1)] [search] [find] [panel toggles].
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
	extraFooterItems,
	augmentPanelTabs,
}) => {
	const shellGen = reactHostPort.useSyncExternalStore(
		(onStoreChange) => platform.subscribe(onStoreChange),
		() => platform.generation,
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
		platform.notify();
	}, [uriProp, onNavigate, onGoBack, onGoForward, onGoUp, canGoBackProp, canGoForwardProp, canGoUpProp, mobile, mobileQuery, className, platform]);

	const [leftPanelSize, setLeftPanelSize] = reactHostPort.useState(280);
	const [rightPanelSize, setRightPanelSize] = reactHostPort.useState(300);
	const [panelVisibility, setPanelVisibilityState] = reactHostPort.useState<UIPanelVisibility>(() =>
		resolveInitialPanelVisibility(initialPanelVisibility, platform),
	);
	const setPanelVisibility = reactHostPort.useCallback(
		(next: UIPanelVisibility | ((prev: UIPanelVisibility) => UIPanelVisibility)) => {
			setPanelVisibilityState((prev) => {
				const resolved = typeof next === "function" ? next(prev) : next;
				platform.setPanelVisibility(resolved);
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
	const detectedMobile = useMediaQuery(mobileQuery);
	const resolvedMobile = mobile ?? detectedMobile ?? platform.mobile;

	useElementsSurfaceChrome({ ...PLATFORM_SYSTEM_SURFACE_CHROME, compact: uiCompact });

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
	const activeApp = activeAppBase.resolve(activeModeId);
	const panelTabsByKind = resolveAppPanelTabsByKind(activeApp, platform.commandBus, augmentPanelTabs);
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

	const hasModeNav = activeAppBase.modes.length > 1;
	const setActiveModeId = (id: string) => {
		activeAppBase.setActiveModeId(id);
		platform.notify();
	};
	const [activeWindowKindId, setActiveWindowKindId] = reactHostPort.useState<string | null>(() => findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds));

	reactHostPort.useEffect(() => {
		setActiveWindowKindId((previous) => {
			if (previous && activeApp.windowKinds.some((windowKind) => windowKind.id === previous)) return previous;
			return findDefaultActiveWindowKindId(activeApp.defaultLayout, activeApp.windowKinds);
		});
	}, [activeApp.defaultLayout, activeApp.windowKinds]);

	const handleActiveWindowChange = reactHostPort.useCallback(
		(windowKindId: string) => {
			setActiveWindowKindId(windowKindId);
			activeApp.onActiveWindowChange?.(windowKindId);
		},
		[activeApp],
	);

	const mergedTools = reactHostPort.useMemo(
		() => mergeToolbarViewTools(declareToolsToViewTools(platform.globalTools, platform.commandBus), declareToolsToViewTools(activeApp.tools, platform.commandBus)),
		[activeApp.tools, platform, shellGen],
	);
	const hasToolbarTools = listPopulatedToolbarViewCategories(mergedTools).length > 0;

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
				setMobilePanelVisible(true);
				return;
			}
			if (activeMobilePanelKind === kind) {
				setMobilePanelVisible(false);
			}
		},
		[activeMobilePanelKind],
	);

	const navbarItems: NavbarItem[] = [];

	if (hasModeNav) {
		navbarItems.push({
			key: "modeNav",
			content: (
				<Select id={`ui.mode.select.${activeAppBase.id}`} onValueChange={setActiveModeId} value={activeModeId ?? undefined}>
					<SelectTrigger className="h-medium w-30" id={`ui.mode.select.${activeAppBase.id}.trigger`} size="sm">
						<SelectValue placeholder="Mode" />
					</SelectTrigger>
					<SelectContent>
						{activeAppBase.modes.map((mode) => (
							<SelectItem key={mode.id} id={`ui.mode.select.${activeAppBase.id}.${mode.id}`} value={mode.id}>
								<span className="flex items-center gap-single">
									{mode.iconId ? resolveElementIcon(mode.iconId) ?? null : null}
									<span>{mode.label}</span>
								</span>
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			),
		});
	}

	navbarItems.push({
		key: "navBack",
		content: (
			<ButtonGroup id="ui.nav.back">
				<ButtonGroupItem id="ui.nav.back" onClick={onGoBack} className={cn(!canGoBackProp && "opacity-30 pointer-events-none")}>
					<ArrowLeft className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});
	navbarItems.push({
		key: "navForward",
		content: (
			<ButtonGroup id="ui.nav.forward">
				<ButtonGroupItem id="ui.nav.forward" onClick={onGoForward} className={cn(!canGoForwardProp && "opacity-30 pointer-events-none")}>
					<ArrowRight className="size-small" />
				</ButtonGroupItem>
			</ButtonGroup>
		),
	});
	navbarItems.push({
		key: "navUp",
		content: (
			<ButtonGroup id="ui.nav.up">
				<ButtonGroupItem id="ui.nav.up" onClick={onGoUp} className={cn(!canGoUpProp && "opacity-30 pointer-events-none")}>
					<ArrowUp className="size-small" />
				</ButtonGroupItem>
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
		const override = platform.breadcrumb?.(uriProp);
		if (override?.length) return platformBreadcrumbToUiItems(override, breadcrumbNavigate);
		return uriToBreadcrumbItems(uriProp, breadcrumbNavigate);
	}, [platform, uriProp, breadcrumbNavigate]);

	navbarItems.push({
		key: "breadcrumb",
		className: "flex-1 min-w-0",
		content: <Breadcrumb className="min-w-0" items={breadcrumbItems} />,
	});

	navbarItems.push({
		key: "search",
		content: <Toggle id="ui.search.toggle" pressed={searchOpen} onPressedChange={setSearchOpen} icon={<Search size={16} />} />,
	});

	navbarItems.push({
		key: "find",
		content: <Toggle id="ui.find.toggle" pressed={findOpen} onPressedChange={setFindOpen} icon={<Search size={16} />} />,
	});

	const panelToggleItems = panelKindsWithTabs.map((kind) => {
		const tabs = panelTabsByKind[kind];
		const icon = panelKindToggleIcon(kind, tabs);
		const side = panelSide(kind);
		if (resolvedMobile) {
			return {
				id: `ui.panelToggle.${kind}`,
				icon,
				pressed: mobilePanelVisible && activeMobilePanelKind === kind,
				onPressedChange: (pressed: boolean) => openMobilePanel(kind, pressed),
			};
		}
		if (side === "left") {
			return {
				id: `ui.panelToggle.${kind}`,
				icon,
				pressed: panelVisibility.leftSidePanel && activeDesktopLeftPanelKind === kind,
				onPressedChange: (pressed: boolean) => openDesktopLeftPanel(kind, pressed),
			};
		}
		return {
			id: `ui.panelToggle.${kind}`,
			icon,
			pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === kind,
			onPressedChange: (pressed: boolean) => openDesktopRightPanel(kind, pressed),
		};
	});

	if (panelToggleItems.length > 0) {
		navbarItems.push({
			key: "panelToggles",
			content: <UIPanelToggleGroup items={panelToggleItems} />,
		});
	}

	const mergedFooterItems = mergePlatformFooterChromeRows(platform, activeApp, [
		{
			id: "settings.compact",
			order: -20,
			content: (
				<Toggle
					id="settings.compact"
					pressed={uiCompact}
					onPressedChange={(pressed) => {
						setUiCompact(pressed);
						writeStoredUiChromeCompact(pressed);
					}}
					icon={<Minimize2 className="size-small" aria-hidden />}
				/>
			),
		},
		...(extraFooterItems ?? []),
	]);

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
		[activeApp.windowKinds, resolvedWindowKindsOverride, platform.commandBus],
	);

	const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined);

	return (
		<UiChromeCompactProvider compact={uiCompact}>
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
					/>
				</UIFindProvider>
			</AppContext.Provider>
		</UiChromeCompactProvider>
	);
};

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	function attachTestPanelTabs(app: AppRuntime): void {
		app.panelTabs = [
			{ id: "workbench", iconId: "folder", panel: "workbench", order: 0, bodyKey: "test.platform.panel.workbench" },
			{ id: "details", iconId: "info", panel: "details", order: 0, bodyKey: "test.platform.panel.details" },
		];
		registerSidePanelBody("test.platform.panel.workbench", () => <div data-testid="test-panel.workbench" />);
		registerSidePanelBody("test.platform.panel.details", () => <div data-testid="test-panel.details" />);
	}

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
							text: "Auto zoom",
							pressed: true,
							onPressedChange: () => {},
						},
					]}
				/>,
			);
			expect(markup).toContain('data-slot="window-measure-float"');
			expect(markup).toContain('data-slot="window-measures-stack-inner"');
			expect(markup).toContain("w-full");
			expect(markup).not.toContain("shadow-md");
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
			expect(markup).not.toContain('id="ui.panelToggle.settings"');
			expect(markup).not.toContain("data-missing-icon");
			expect(markup).toContain('lucide lucide-folder');
			expect(markup).toContain('lucide lucide-info');
		});

		it("renders navbar buttons and toggles with inline labels when compact is off", () => {
			if (typeof localStorage !== "undefined") {
				localStorage.setItem("ui.chrome.compact", "false");
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
			expect(markup).toContain("Compact");
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
				{ id: "workbench", iconId: "lucide.folder", panel: "workbench", order: 0, bodyKey: "test.platform.panel.workbench" },
				{ id: "details", iconId: "lucide.info", panel: "details", order: 0, bodyKey: "test.platform.panel.details" },
			];
			registerSidePanelBody("test.platform.panel.workbench", () => <div />);
			registerSidePanelBody("test.platform.panel.details", () => <div />);
			wb.addApp(app);
			const markup = renderToStaticMarkup(<PlatformView platform={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain('lucide lucide-folder');
			expect(markup).toContain('lucide lucide-info');
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
			app.tools = { selection: [{ id: "base-tool", kind: "button", label: "Base", controllerId: "tctrl", command: "x" }] };
			app.selection = { base: true };
			app.options = { snap: true };
			const inspect = new ModeRuntime("inspect", "Inspect", undefined);
			inspect.tools = { actions: [{ id: "mode-tool", kind: "button", label: "Mode", controllerId: "tctrl", command: "y" }] };
			inspect.selection = { mode: true };
			inspect.options = { isolate: true };
			inspect.windowKinds = [new WindowKindRuntime("mode", "Mode", "test.workbench-view.mode")];
			app.addMode(inspect);
			app.defaultModeId = "inspect";
			const resolved = app.resolve("inspect");

			expect(resolved.activeModeId).toBe("inspect");
			expect(resolved.tools?.selection?.map((tool) => tool.id)).toEqual(["base-tool"]);
			expect(resolved.tools?.actions?.map((tool) => tool.id)).toEqual(["mode-tool"]);
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

			expect(markup).toContain('id="ui.mode.select.app.trigger"');
			expect(markup).not.toContain("ui.modeNav.app");
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
