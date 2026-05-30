// #region 🧲Header
/** @emoji ⚛️ `@framework/platform/renderer/react` — React renderer for {@link @framework/platform/core}: declarative {@link UiNode} host (monolith). */
// #endregion 🧲Header

export { ProductRuntime, APP_TOOL_CATEGORY_ORDER, type WindowLayout, type AppToolCategory } from "@framework/platform/core";

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
	ProductRuntime,
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
	type UiBoardHostSurfaceNode,
	type UiButtonNode,
	type UiNode,
	type UiPanelHostSurfaceNode,
	type UiScene3DHostSurfaceNode,
	type UiSeparatorNode,
	type UiStackNode,
	type UiTableHostSurfaceNode,
	type UiTextNode,
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
	MoreHorizontal as MoreHorizontalIcon,
	MousePointer2 as MousePointerIcon,
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
import { useTranslation } from "react-i18next";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import {
	BasicChatPanel,
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
	ActionGroup,
	ActionGroupItem,
	ToolbarDivider,
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
	useCommandHotkey,
	useMediaQuery,
	type ContextMenuItem,
	type NavbarItem,
	reactHostPort,
} from "@ui/react";
// #endregion 🔌Adapters

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

/** @emoji 📑 Side panel tab registration consumed by {@link ProductView}. */
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
 * Root layout wrapper owned by an app instead of the Golden Layout runtime.
 **/
export interface WindowLayout {
  root: WindowLayoutAxisNode | WindowLayoutStackNode;
}

/**
 * Union of supported abstract UI layout nodes.
 **/
export type WindowLayoutNode = WindowLayout["root"];

/**
 * Alias for WindowLayout used by the sketchpad layer.
 **/
export type LayoutNode = WindowLayout;

/**
 * Alias for WindowLayoutStackNode used by the sketchpad layer.
 **/
export type LayoutStack = WindowLayoutStackNode;

/**
 * Alias for WindowLayoutAxisNode with kind "row" used by the sketchpad layer.
 **/
export type LayoutRow = WindowLayoutAxisNode & { kind: "row" };

/**
 * Alias for WindowLayoutAxisNode with kind "column" used by the sketchpad layer.
 **/
export type LayoutColumn = WindowLayoutAxisNode & { kind: "column" };

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

/**
 * Alias for convertWindowLayoutToGoldenConfig used by the sketchpad layer.
 **/
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
          <ActionGroupItem key={control.id} id={control.id} onClick={() => control.onChange?.(control.value === "on" ? "off" : "on")}>
            {control.icon}
          </ActionGroupItem>
        );
      }
      return (
        <ActionGroupItem key={control.id} id={control.id}>
          {control.icon}
        </ActionGroupItem>
      );
    })}
  </ActionGroup>
);

// #region 🪟WindowMeasuresOverlay

const UIWindowMeasureFloat: React.FC<{ measureId: string; label?: string; children: React.ReactNode }> = ({ measureId, label, children }) => (
  <div
    data-slot="window-measure-float"
    data-measure-id={measureId}
    className="border-element/80 bg-window/90 max-w-[11rem] min-w-0 rounded-md border px-single py-half shadow-md backdrop-blur-sm"
  >
    {label ? <span className="text-muted-foreground mb-half block max-w-full truncate text-[10px] font-semibold uppercase tracking-wide">{label}</span> : null}
    <div className="min-w-0 w-full">{children}</div>
  </div>
);

/**
 * 📐 Maps declarative `UIWindowMeasure` entries into compact floating tiles aligned to the right edge.
 **/
export const UIWindowMeasures: React.FC<{ measures: UIWindowMeasure[] }> = ({ measures }) => (
  <div data-slot="window-measures-stack-inner" className="flex flex-col items-end gap-half">
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
            <div
              key={measure.id}
              data-slot="window-measure-heading"
              className="border-element/60 bg-window/85 max-w-[11rem] rounded-md border px-single py-tiny text-center shadow-sm backdrop-blur-sm"
            >
              <span className="text-muted-foreground text-[10px] font-semibold uppercase tracking-wide">{measure.title}</span>
            </div>
          );
        case "separator":
          return <div key={measure.id} data-slot="window-measure-separator" className="bg-muted-foreground/35 my-half h-px w-8 shrink-0 rounded-full" aria-hidden />;
        case "toggle":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Toggle id={measure.id} pressed={measure.pressed} defaultPressed={measure.defaultPressed} onPressedChange={measure.onPressedChange} icon={measure.icon ?? <CheckIcon className="size-small" />} text={measure.text} />
            </UIWindowMeasureFloat>
          );
        case "select":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Select id={measure.id} value={measure.value} defaultValue={measure.defaultValue} onValueChange={measure.onValueChange}>
                <SelectTrigger id={measure.id} className="h-medium w-full min-w-0 max-w-[9.5rem]" size="sm">
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
              <Combobox id={measure.id} value={measure.value} options={measure.choices} placeholder={measure.placeholder} onValueChange={measure.onValueChange} className="w-full min-w-0 max-w-[9.5rem]" />
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
              <Input id={measure.id} lazy className="h-medium w-full min-w-0 max-w-[9.5rem]" value={measure.value} placeholder={measure.placeholder} onLazyChange={measure.onLazyChange} />
            </UIWindowMeasureFloat>
          );
        case "textarea":
          return (
            <UIWindowMeasureFloat key={measure.id} measureId={measure.id} label={measure.label}>
              <Textarea id={measure.id} lazy className="min-h-[4rem] w-full min-w-0 max-w-[9.5rem]" value={measure.value} placeholder={measure.placeholder} rows={measure.rows} onLazyChange={measure.onLazyChange} />
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
                      "border-element/80 hover:bg-hover-window rounded border px-single py-half text-left text-xs transition-colors",
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
              <Input id={measure.id} type="color" className="h-medium w-full min-w-0 max-w-[9.5rem] cursor-pointer" value={measure.value} onChange={(event) => measure.onChange?.(event.target.value)} />
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

function convertFrameworkLayoutToShellLayout(layout: WindowLayout): ShellWindowLayoutNode {
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

/** @emoji 🪟 Pure-React resizable mode canvas backed by {@link Mode}. */
const ShellModeCanvas: React.FC<{
  windowKinds: UIWindowKindDefinition[];
  defaultLayout: WindowLayout;
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string) => void;
}> = ({ windowKinds, defaultLayout, activeWindowId, onActiveWindowChange }) => {
  const windows = reactHostPort.useMemo<ModeWindowDescriptor[]>(
    () =>
      windowKinds.map((windowKind) => {
        const WindowComponent = windowKind.component;
        return {
          id: windowKind.id,
          title: windowKind.label,
          showControls: true,
          controls: windowKind.controls ? <UIWindowControlsGroup controls={windowKind.controls} /> : undefined,
          measures: windowKind.measures?.length ? <UIWindowMeasures measures={windowKind.measures} /> : undefined,
          engagement: windowKind.engagement ?? windowControlsToEngagement(windowKind.controls),
          children: (
            <ContextMenu items={windowKind.contextMenu}>
              <div className="flex min-h-0 min-w-0 flex-1 flex-col">
                <WindowComponent />
              </div>
            </ContextMenu>
          ),
        };
      }),
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
    <CommandDialog title="Search" description="Search for items..." open={open} onOpenChange={onOpenChange}>
      <CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} onSelect={() => handleSelect(result.item)}>
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
    <CommandDialog title="Find" description="Find items in this app..." open={open} onOpenChange={onOpenChange}>
      <CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} onSelect={() => handleSelect(result.item)}>
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
    case "transform":
      return <ArrowRightLeftIcon className="size-tiny" aria-hidden />;
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
  return (
    <>
      {sorted.map((item) => {
        if (item.kind === "separator") {
          return <ToolbarDivider key={item.id} />;
        }
        if (item.kind === "toggle") {
          return (
            <ToolbarItem key={item.id}>
              <Toggle
                kind={item.icon && !item.text && !item.label ? "icon" : "default"}
                id={item.id}
                pressed={item.pressed ?? false}
                onPressedChange={(pressed) => item.onPressedChange?.(pressed)}
                icon={item.icon}
                text={item.text ?? item.label}
              />
            </ToolbarItem>
          );
        }
        return (
          <ToolbarItem key={item.id}>
            <button onClick={item.onClick} className="flex items-center gap-single px-single py-tiny hover:bg-hover-panel rounded text-sm cursor-pointer">
              {item.icon}
              {(item.text ?? item.label) && <span>{item.text ?? item.label}</span>}
            </button>
          </ToolbarItem>
        );
      })}
    </>
  );
};

/**
 * Renders a floating toolbar with category toggles; only categories with registered tools are shown.
 **/
const UIToolbar: React.FC<{
  tools: ToolbarViewTools;
  className?: string;
}> = ({ tools, className }) => {
  const { t } = useTranslation();
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
        className={cn(
          "pointer-events-auto flex max-w-full items-center gap-single",
          showCategoryNav && "relative h-[var(--toolbar-item-height)] w-full max-w-[min(100%,48rem)] px-2",
        )}
      >
        {showCategoryNav ? (
          <>
            <ToolbarZone id="ui.toolbar.zone.categories" className="shrink-0">
              {populatedCategories.map((category) => (
                <Toggle
                  key={category}
                  kind="single"
                  id={`ui.toolbar.group.${category}`}
                  pressed={activeCategory === category}
                  onPressedChange={() => setActiveCategory((previousValue) => (previousValue === category ? null : category))}
                  icon={resolveAppToolCategoryIcon(category)}
                  text={resolveTranslationLabel(t(`semio.sketchpad.toolbar.parent.${category}`))}
                />
              ))}
            </ToolbarZone>
            {activeCategory && hasAppToolCategoryItems(activeItems) ? (
              <ToolbarZone id="ui.toolbar.zone.tools" className="min-w-0 flex-1 flex-wrap h-auto min-h-[var(--toolbar-item-height)] overflow-visible p-half">
                <UIToolbarItems items={activeItems} />
              </ToolbarZone>
            ) : null}
          </>
        ) : (
          <ToolbarZone className="max-w-full flex-wrap h-auto min-h-[var(--toolbar-item-height)] overflow-visible p-half">
            <UIToolbarItems items={tools[populatedCategories[0]!] ?? []} />
          </ToolbarZone>
        )}
      </div>
    </div>
  );
};

export { ShellModeCanvas, UISearch, UIFind, UIToolbar };
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
/** @emoji 🧭 Props for {@link ProductView} (navbar, panels, golden-layout canvas). */
export interface ProductViewProps {
	runtime: ProductRuntime;
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
	augmentPanelTabs?: Partial<Record<"workbench" | "details", SidePanelTabConfig[]>>;
	initialPanelVisibility?: UIPanelVisibility;
}

/** @emoji 🧭 @deprecated Use {@link ProductViewProps}. */
export type AppProps = ProductViewProps;

export interface UIPanelVisibility {
	leftSidePanel: boolean;
	rightSidePanel: boolean;
}

export interface AppContextValue {
	runtime: ProductRuntime;
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

/** @emoji 🪝 Returns the active {@link ProductRuntime} shell context from the nearest {@link AppContext}. */
export function useApp(): AppContextValue {
	const ctx = reactHostPort.useContext(AppContext);
	if (!ctx) throw new Error("useApp must be used within a ProductView");
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

	return { history, uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate };
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
}
//#endregion ­ƒº¬Tests

//#endregion ­ƒôªworkbench-history.tsx

//#region ­ƒôªui-declarative-renderer.tsx

//#region ­ƒöûScene3DRegistry
type Scene3DSurfaceHost = React.ComponentType<{ readonly node: UiScene3DHostSurfaceNode }>;

const scene3dSurfaceHosts = new Map<string, Scene3DSurfaceHost>();

/** @emoji ­ƒº¡ Binds a `surfaceId` from {@link UiScene3DHostSurfaceNode} to a host React canvas implementation. */
export function registerUiScene3DSurfaceHost(surfaceId: string, Component: Scene3DSurfaceHost): void {
	scene3dSurfaceHosts.set(surfaceId, Component);
}

/** @emoji ­ƒº╣ Drops a surface binding (tests). */
export function unregisterUiScene3DSurfaceHost(surfaceId: string): void {
	scene3dSurfaceHosts.delete(surfaceId);
}
//#endregion ­ƒöûScene3DRegistry

//#region ­ƒöûBoardRegistry
type BoardSurfaceHost = React.ComponentType<{ readonly node: UiBoardHostSurfaceNode }>;

const boardSurfaceHosts = new Map<string, BoardSurfaceHost>();

/** @emoji ­ƒôï Binds `surfaceId` from {@link UiBoardHostSurfaceNode} to a host board canvas. */
export function registerUiBoardSurfaceHost(surfaceId: string, Component: BoardSurfaceHost): void {
	boardSurfaceHosts.set(surfaceId, Component);
}

/** @emoji ­ƒº╣ Drops a board surface binding (tests). */
export function unregisterUiBoardSurfaceHost(surfaceId: string): void {
	boardSurfaceHosts.delete(surfaceId);
}
//#endregion ­ƒöûBoardRegistry

//#region ­ƒöûTableRegistry
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;

const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

/** @emoji ­ƒôæ Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
	tableSurfaceHosts.set(surfaceId, Component);
}

/** @emoji ­ƒº╣ Drops a table surface binding (tests). */
export function unregisterUiTableSurfaceHost(surfaceId: string): void {
	tableSurfaceHosts.delete(surfaceId);
}
//#endregion ­ƒöûTableRegistry

//#region ­ƒöûPanelRegistry
type PanelSurfaceHost = React.ComponentType<{ readonly node: UiPanelHostSurfaceNode }>;

const panelSurfaceHosts = new Map<string, PanelSurfaceHost>();

/** @emoji ­ƒº® Binds `surfaceId` from {@link UiPanelHostSurfaceNode} to a host side-panel body. */
export function registerUiPanelSurfaceHost(surfaceId: string, Component: PanelSurfaceHost): void {
	panelSurfaceHosts.set(surfaceId, Component);
}

/** @emoji ­ƒº╣ Drops a panel surface binding (tests). */
export function unregisterUiPanelSurfaceHost(surfaceId: string): void {
	panelSurfaceHosts.delete(surfaceId);
}
//#endregion ­ƒöûPanelRegistry

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

function renderScene3d(node: UiScene3DHostSurfaceNode): React.ReactElement {
	const Host = scene3dSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported scene3d surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<Host node={node} />
		</div>
	);
}

function renderBoard(node: UiBoardHostSurfaceNode): React.ReactElement {
	const Host = boardSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported board surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<Host node={node} />
		</div>
	);
}

function renderTable(node: UiTableHostSurfaceNode): React.ReactElement {
	const Host = tableSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported table surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
			<Host node={node} />
		</div>
	);
}

function renderPanel(node: UiPanelHostSurfaceNode): React.ReactElement {
	const Host = panelSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported panel surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
			<Host node={node} />
		</div>
	);
}

function renderNode(node: UiNode, commandBus: CommandBus, horizontalParent: boolean): React.ReactElement {
	switch (node.type) {
		case "stack":
			return (
				<div className={cn(stackClass(node), node.direction === "vertical" && node.children.some((c) => c.type === "scene3d" || c.type === "board") && "relative min-h-0 flex-1")}>
					{node.children.map((child, index) => (
						<React.Fragment key={index}>{renderNode(child, commandBus, node.direction === "horizontal")}</React.Fragment>
					))}
				</div>
			);
		case "text":
			return renderText(node);
		case "button":
			return renderButton(node, commandBus);
		case "separator":
			return renderSeparator(node, horizontalParent);
		case "scene3d":
			return renderScene3d(node);
		case "board":
			return renderBoard(node);
		case "table":
			return renderTable(node);
		case "panel":
			return renderPanel(node);
		default:
			return (
				<div className="p-2 text-xs text-destructive">
					Unsupported UiNode {(node as { type?: string }).type ?? "unknown"}
				</div>
			);
	}
}

/** @emoji ­ƒº® Host entry: turns declarative {@link UiNode} trees into mounted React structure. */
export function UiRenderer({ node, commandBus }: UiRendererProps): React.ReactElement {
	return renderNode(node, commandBus, false);
}
//#endregion ­ƒöûRenderer

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

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

/** @emoji ­ƒû╝ Registers a Lucide icon constructor for side-panel tab headers keyed by `iconId`. */
export function registerTabIcon(iconId: string, Icon: LucideIcon): void {
	shellTabIcons.set(iconId, Icon);
}

const windowBodyByKey = new Map<string, React.ComponentType<unknown>>();

/** @emoji ­ƒ¬ƒ Binds a `bodyKey` from {@link WindowKindRuntime} to a React window body component. */
export function registerWindowBody(bodyKey: string, Component: React.ComponentType<unknown>): void {
	windowBodyByKey.set(bodyKey, Component);
}

const sidePanelBodyByKey = new Map<string, React.ComponentType<unknown>>();

function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
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
			const { runtime, activeModeId } = useApp();
			const generation = reactHostPort.useSyncExternalStore(
				(listener) => runtime.subscribe(listener),
				() => runtime.generation,
				() => 0,
			);
			const ctx: WindowBodyViewContext = {
				runtime,
				windowKindId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getWindowBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative body "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={runtime.commandBus} />;
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
			const { runtime, activeModeId } = useApp();
			const generation = reactHostPort.useSyncExternalStore(
				(listener) => runtime.subscribe(listener),
				() => runtime.generation,
				() => 0,
			);
			const ctx: SidePanelBodyViewContext = {
				runtime,
				windowKindId: tabId,
				bodyKey,
				activeModeId: activeModeId ?? null,
				generation,
			};
			const factory = getSidePanelBodyFactory(bodyKey);
			const node = factory?.(ctx) ?? { type: "text", value: `Missing declarative panel "${bodyKey}"` };
			return <UiRenderer node={node} commandBus={runtime.commandBus} />;
		};
		declarativeSidePanelBodyComponents.set(cacheKey, component);
	}
	return component;
}

function windowMeasuresToGolden(measures: readonly WindowMeasure[], bus: CommandBus): UIWindowMeasure[] | undefined {
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

function shellTabIconComponent(iconId: string): React.ComponentType<{ size?: number }> {
	return function ShellResolvedTabIcon({ size = 16 }: { size?: number }) {
		const node = elementIconNodes.get(iconId);
		if (node) {
			return (
				<span className="inline-flex items-center justify-center" style={{ width: size, height: size }}>
					{node}
				</span>
			);
		}
		const Lucide = shellTabIcons.get(iconId);
		return Lucide ? <Lucide size={size} /> : <span style={{ display: "inline-block", width: size }} data-missing-icon={iconId} />;
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
			icon: shellTabIconComponent(tab.iconId),
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
const APP_WORKBENCH_TAB_ID = "workbench";
const APP_DETAILS_TAB_ID = "details";
const APP_OPTIONS_TAB_ID = "options";
const APP_CHAT_TAB_ID = "chat";
type AppPanelKind = "workbench" | "details" | "options" | "chat";

function hasAppPanelValue(value: unknown): boolean {
  if (value === null || value === undefined) return false;
  if (typeof value === "string") return value.trim().length > 0;
  if (Array.isArray(value)) return value.length > 0;
  if (typeof value === "object") return Object.keys(value as Record<string, unknown>).length > 0;
  return true;
}

const AppPanelStatePreview: React.FC<{
  emptyMessage: string;
  testId: string;
  value: unknown;
}> = ({ emptyMessage, testId, value }) => {
  if (!hasAppPanelValue(value)) {
    return <div data-testid={`${testId}.empty`} className="text-sm text-muted-foreground">{emptyMessage}</div>;
  }

  return (
    <pre data-testid={testId} className="text-xs leading-relaxed whitespace-pre-wrap break-words rounded-[3px] border bg-window p-small overflow-x-auto">
      {JSON.stringify(value, null, 2)}
    </pre>
  );
};

const AppSummaryPanel: React.FC<{
  activeModeLabel?: string | null;
  app: ResolvedAppState;
}> = ({ activeModeLabel, app }) => {
  return (
    <div data-testid="app-panel.workbench" className="flex min-h-0 flex-col gap-small text-sm">
      <div>
        <div className="font-medium">{app.label}</div>
        <div className="text-muted-foreground">{activeModeLabel ? `Mode: ${activeModeLabel}` : "Single-mode app"}</div>
      </div>
      <div className="grid gap-single text-muted-foreground">
        <div>{`Windows: ${app.windowKinds.length}`}</div>
        <div>{`Tools: ${countAppTools(app.tools)}`}</div>
        <div>{`Left tabs: ${app.leftPanelTabs?.length ?? 0}`}</div>
        <div>{`Right tabs: ${app.rightPanelTabs?.length ?? 0}`}</div>
      </div>
    </div>
  );
};

function createDefaultAppHostTabs(app: ResolvedAppState, activeModeLabel?: string | null): SidePanelTabConfig[] {
  return [
    staticSidePanelTabDefinition({
      id: APP_WORKBENCH_TAB_ID,
      icon: Folder,
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [{ id: `${APP_WORKBENCH_TAB_ID}.summary`, content: <AppSummaryPanel activeModeLabel={activeModeLabel} app={app} /> }],
      }),
    }).resolveTab(),
  ];
}

function createDefaultAppDetailsTabs(app: ResolvedAppState): SidePanelTabConfig[] {
  return [
    staticSidePanelTabDefinition({
      id: APP_DETAILS_TAB_ID,
      icon: Info,
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: `${APP_DETAILS_TAB_ID}.state`,
            content: <AppPanelStatePreview emptyMessage="No detail state is available for this app." testId="app-panel.details" value={{ selection: app.selection ?? {}, hover: app.hover ?? {} }} />,
          },
        ],
      }),
    }).resolveTab(),
  ];
}

function createDefaultAppOptionsTabs(app: ResolvedAppState): SidePanelTabConfig[] {
  return [
    staticSidePanelTabDefinition({
      id: APP_OPTIONS_TAB_ID,
      icon: Settings2,
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [{ id: `${APP_OPTIONS_TAB_ID}.state`, content: <AppPanelStatePreview emptyMessage="No options are available for this app." testId="app-panel.options" value={app.options ?? {}} /> }],
      }),
    }).resolveTab(),
  ];
}

function createDefaultAppChatTabs(app: ResolvedAppState): SidePanelTabConfig[] {
  return [
    staticSidePanelTabDefinition({
      id: APP_CHAT_TAB_ID,
      icon: MessageSquare,
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [{ id: `${APP_CHAT_TAB_ID}.content`, content: <BasicChatPanel id={`app.chat.${app.id}`} title={app.label} /> }],
      }),
    }).resolveTab(),
  ];
}

function withDefaultAppPanelTabs(app: ResolvedAppState, bus: CommandBus, activeModeLabel?: string | null): Record<AppPanelKind, SidePanelTabConfig[]> {
	const defaultHostTabs = createDefaultAppHostTabs(app, activeModeLabel);
	const defaultDetailsTabs = createDefaultAppDetailsTabs(app);
	const defaultOptionsTabs = createDefaultAppOptionsTabs(app);
	const defaultChatTabs = createDefaultAppChatTabs(app);
	const shellLeft = sideTabsToPanelTabs(app.leftTabs, bus);
	const shellRight = sideTabsToPanelTabs(app.rightTabs, bus);
	return {
		workbench: mergeConfigEntries(defaultHostTabs, shellLeft.length ? shellLeft : undefined) ?? defaultHostTabs,
		details: mergeConfigEntries(defaultDetailsTabs, shellRight.length ? shellRight : undefined) ?? defaultDetailsTabs,
		options: defaultOptionsTabs,
		chat: defaultChatTabs,
	};
}

/**
 * Left panel toggle for the navbar.
 * Uses the first tab icon as the toggle icon.
 * Styled to match sketchpad: border border-element, h-medium.
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
        kind="icon"
        id={item.id}
        pressed={item.pressed}
        onPressedChange={item.onPressedChange}
        className={cn("border-0 rounded-none", index > 0 && "border-l")}
        icon={item.icon}
      />
    ))}
  </div>
);

/**
 * Domain-neutral composite component providing a full application shell.
 * The UI only has apps. An app has window kinds (rendered with golden-layout)
 * and registers left/right side panel tabs, footer items, toolbar items, and find items.
 * Every UI has: toolbar, search (Ctrl+P), panel toggles, back/forward/up navigation.
 * Every app has: find (Ctrl+F).
 * Every panel has: tree.
 * Fixed navbar layout: [mode (if >1 mode)] [back] [forward] [up] [app nav (if >1 app)] [uri (flex-1)] [search] [find] [panel toggles].
 **/
export const ProductView: React.FC<ProductViewProps> = ({
	runtime,
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
		(onStoreChange) => runtime.subscribe(onStoreChange),
		() => runtime.generation,
		() => 0,
	);
	void shellGen;

	reactHostPort.useEffect(() => {
		if (defaultAppId) {
			runtime.setActiveAppId(defaultAppId);
		}
	}, [defaultAppId, runtime]);

	reactHostPort.useEffect(() => {
		runtime.uri = uriProp;
		runtime.onNavigate = onNavigate;
		runtime.onGoBack = onGoBack;
		runtime.onGoForward = onGoForward;
		runtime.onGoUp = onGoUp;
		runtime.canGoBack = canGoBackProp;
		runtime.canGoForward = canGoForwardProp;
		runtime.canGoUp = canGoUpProp;
		runtime.mobile = mobile;
		runtime.mobileQuery = mobileQuery;
		runtime.className = className ?? "";
		runtime.notify();
	}, [uriProp, onNavigate, onGoBack, onGoForward, onGoUp, canGoBackProp, canGoForwardProp, canGoUpProp, mobile, mobileQuery, className, runtime]);

	const [leftPanelSize, setLeftPanelSize] = reactHostPort.useState(280);
	const [rightPanelSize, setRightPanelSize] = reactHostPort.useState(300);
	const [panelVisibility, setPanelVisibility] = reactHostPort.useState<UIPanelVisibility>(() => ({
		leftSidePanel: initialPanelVisibility?.leftSidePanel ?? false,
		rightSidePanel: initialPanelVisibility?.rightSidePanel ?? false,
	}));
	const [mobilePanelVisible, setMobilePanelVisible] = reactHostPort.useState(false);
	const [activeDesktopRightPanelKind, setActiveDesktopRightPanelKind] = reactHostPort.useState<Exclude<AppPanelKind, "workbench">>("details");
	const [activeMobilePanelKind, setActiveMobilePanelKind] = reactHostPort.useState<AppPanelKind>("workbench");
	const [mobilePanelActiveTabId, setMobilePanelActiveTabId] = reactHostPort.useState<string | undefined>(undefined);
	const [searchOpen, setSearchOpen] = reactHostPort.useState(false);
	const [findOpen, setFindOpen] = reactHostPort.useState(false);
	const detectedMobile = useMediaQuery(mobileQuery);
	const resolvedMobile = mobile ?? detectedMobile ?? runtime.mobile;

	useCommandHotkey(
		"ctrl+p,meta+p",
		() => {
			const activeEl = document.activeElement as HTMLElement | null;
			if (!searchOpen && activeEl && (activeEl.tagName === "INPUT" || activeEl.tagName === "TEXTAREA" || activeEl.isContentEditable)) {
				return;
			}
			setSearchOpen((previousValue) => !previousValue);
		},
		{ preventDefault: true, enableOnFormTags: true },
		[searchOpen],
	);
	useCommandHotkey(
		"ctrl+f,meta+f",
		() => {
			setFindOpen((previousValue) => !previousValue);
		},
		{ preventDefault: true, enableOnFormTags: true },
		[],
	);

	const togglePanel = reactHostPort.useCallback((panel: keyof UIPanelVisibility) => {
		setPanelVisibility((prev) => ({ ...prev, [panel]: !prev[panel] }));
	}, []);

	const resolvedApps = runtime.apps;
	const activeAppId = runtime.activeAppId;
	const setActiveAppId = reactHostPort.useCallback(
		(id: string) => {
			runtime.setActiveAppId(id);
		},
		[runtime],
	);

	const activeAppBase = runtime.getActiveApp();
	if (!activeAppBase) return null;

	const activeModeId = activeAppBase.getActiveModeId();
	const activeApp = activeAppBase.resolve(activeModeId);
	const activeModeLabel = activeAppBase.modes.find((mode) => mode.id === activeModeId)?.label ?? null;
	const panelTabsBase = withDefaultAppPanelTabs(activeApp, runtime.commandBus, activeModeLabel);
	const panelTabs = {
		...panelTabsBase,
		workbench: mergeConfigEntries(panelTabsBase.workbench, augmentPanelTabs?.workbench) ?? panelTabsBase.workbench,
		details: mergeConfigEntries(panelTabsBase.details, augmentPanelTabs?.details) ?? panelTabsBase.details,
	};
	const workbenchTabs = panelTabs.workbench;
	const detailsTabs = panelTabs.details;
	const optionsTabs = panelTabs.options;
	const chatTabs = panelTabs.chat;
	const activeDesktopRightPanelTabs = activeDesktopRightPanelKind === "details" ? detailsTabs : activeDesktopRightPanelKind === "options" ? optionsTabs : chatTabs;
	const activeMobilePanelTabs = activeMobilePanelKind === "workbench" ? workbenchTabs : activeMobilePanelKind === "details" ? detailsTabs : activeMobilePanelKind === "options" ? optionsTabs : chatTabs;

	const hasModeNav = activeAppBase.modes.length > 1;
	const setActiveModeId = (id: string) => {
		activeAppBase.setActiveModeId(id);
		runtime.notify();
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
		() => mergeToolbarViewTools(declareToolsToViewTools(runtime.globalTools, runtime.commandBus), declareToolsToViewTools(activeApp.tools, runtime.commandBus)),
		[activeApp.tools, runtime, shellGen],
	);
	const hasToolbarTools = listPopulatedToolbarViewCategories(mergedTools).length > 0;

	const openDesktopLeftPanel = reactHostPort.useCallback((pressed: boolean) => {
		setPanelVisibility((prev) => ({ ...prev, leftSidePanel: pressed }));
	}, []);

	const openDesktopRightPanel = reactHostPort.useCallback(
		(kind: Exclude<AppPanelKind, "workbench">, pressed: boolean) => {
			if (pressed) {
				setActiveDesktopRightPanelKind(kind);
				setPanelVisibility((prev) => ({ ...prev, rightSidePanel: true }));
				return;
			}
			setPanelVisibility((prev) => ({ ...prev, rightSidePanel: kind === activeDesktopRightPanelKind ? false : prev.rightSidePanel }));
		},
		[activeDesktopRightPanelKind],
	);

	const openMobilePanel = reactHostPort.useCallback(
		(kind: AppPanelKind, pressed: boolean) => {
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

	const workbenchIcon = workbenchTabs[0]?.icon ? React.createElement(workbenchTabs[0].icon, { size: 16 }) : <Folder size={16} />;
	const detailsIcon = detailsTabs[0]?.icon ? React.createElement(detailsTabs[0].icon, { size: 16 }) : <Info size={16} />;
	const optionsIcon = optionsTabs[0]?.icon ? React.createElement(optionsTabs[0].icon, { size: 16 }) : <Settings2 size={16} />;
	const chatIcon = chatTabs[0]?.icon ? React.createElement(chatTabs[0].icon, { size: 16 }) : <MessageSquare size={16} />;

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

	if (resolvedApps.length > 1) {
		navbarItems.push({
			key: "appNav",
			content: (
				<ButtonGroup id="ui.appNav">
					{resolvedApps.map((app) => (
						<ButtonGroupItem key={app.id} id={`ui.appNav.${app.id}`} className={cn(activeAppId === app.id && "bg-active-base")} onClick={() => setActiveAppId(app.id)}>
							{app.iconId ? resolveElementIcon(app.iconId) ?? <span className="text-xs">{app.label}</span> : <span className="text-xs">{app.label}</span>}
						</ButtonGroupItem>
					))}
				</ButtonGroup>
			),
		});
	}

	navbarItems.push({
		key: "uri",
		className: "flex-1 min-w-0",
		content: <span className="text-sm text-muted-foreground truncate px-single select-all">{uriProp}</span>,
	});

	navbarItems.push({
		key: "search",
		content: <Toggle kind="icon" id="ui.search.toggle" pressed={searchOpen} onPressedChange={setSearchOpen} icon={<Search size={16} />} />,
	});

	navbarItems.push({
		key: "find",
		content: <Toggle kind="icon" id="ui.find.toggle" pressed={findOpen} onPressedChange={setFindOpen} icon={<Search size={16} />} />,
	});

	navbarItems.push({
		key: "panelToggles",
		content: (
			<UIPanelToggleGroup
				items={
					resolvedMobile
						? [
								{ id: "ui.panelToggle.workbench", icon: workbenchIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "workbench", onPressedChange: (pressed) => openMobilePanel("workbench", pressed) },
								{ id: "ui.panelToggle.details", icon: detailsIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "details", onPressedChange: (pressed) => openMobilePanel("details", pressed) },
								{ id: "ui.panelToggle.options", icon: optionsIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "options", onPressedChange: (pressed) => openMobilePanel("options", pressed) },
								{ id: "ui.panelToggle.chat", icon: chatIcon, pressed: mobilePanelVisible && activeMobilePanelKind === "chat", onPressedChange: (pressed) => openMobilePanel("chat", pressed) },
						  ]
						: [
								{ id: "ui.panelToggle.workbench", icon: workbenchIcon, pressed: panelVisibility.leftSidePanel, onPressedChange: openDesktopLeftPanel },
								{ id: "ui.panelToggle.details", icon: detailsIcon, pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === "details", onPressedChange: (pressed) => openDesktopRightPanel("details", pressed) },
								{ id: "ui.panelToggle.options", icon: optionsIcon, pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === "options", onPressedChange: (pressed) => openDesktopRightPanel("options", pressed) },
								{ id: "ui.panelToggle.chat", icon: chatIcon, pressed: panelVisibility.rightSidePanel && activeDesktopRightPanelKind === "chat", onPressedChange: (pressed) => openDesktopRightPanel("chat", pressed) },
						  ]
				}
			/>
		),
	});

	const mergedFooterItems = [
		...declarativeFooterToChromeRows(runtime.globalFooterItems, runtime.commandBus),
		...declarativeFooterToChromeRows(activeApp.footerItems, runtime.commandBus),
		...(extraFooterItems ?? []),
	].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

	const searchItemsResolved = reactHostPort.useMemo(
		() =>
			resolveCommandPaletteItems(runtime, activeApp, activeWindowKindId).map((row) => ({
				id: row.id,
				label: row.label,
				description: row.description,
				category: row.category,
				icon: row.iconId ? resolveElementIcon(row.iconId) : undefined,
				onSelect: () => runtime.commandBus.dispatch(row.controllerId, row.command, row.args),
			})),
		[runtime, activeApp, activeWindowKindId, shellGen],
	);

	const goldenWindowKinds = reactHostPort.useMemo(
		() => resolvedWindowKindsOverride ?? windowKindsToGolden(activeApp.windowKinds, runtime.commandBus),
		[activeApp.windowKinds, resolvedWindowKindsOverride, runtime.commandBus],
	);

	const toolbarElement = slotToolbar ?? (hasToolbarTools && mergedTools ? <UIToolbar tools={mergedTools} /> : undefined);

	return (
		<AppContext.Provider
			value={{
				runtime,
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
				<Layout
					className={className}
					mobile={resolvedMobile}
					navbar={<Navbar items={navbarItems} />}
					footer={mergedFooterItems.length > 0 ? <Footer items={mergedFooterItems} /> : undefined}
					toolbar={toolbarElement}
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
					leftSidePanel={
						!resolvedMobile
							? {
									position: "left" as const,
									visible: panelVisibility.leftSidePanel,
									size: leftPanelSize,
									onSizeChange: setLeftPanelSize,
									tabs: workbenchTabs,
							  }
							: undefined
					}
					rightSidePanel={
						!resolvedMobile
							? {
									position: "right" as const,
									visible: panelVisibility.rightSidePanel,
									size: rightPanelSize,
									onSizeChange: setRightPanelSize,
									tabs: activeDesktopRightPanelTabs,
							  }
							: undefined
					}
					canvas={
						<Ui
							apps={resolvedApps.map((app) => ({
								id: app.id,
								label: app.label,
								icon: app.iconId ? resolveElementIcon(app.iconId) : undefined,
								children: (
									<App
										modes={
											app.modes.length > 0
												? app.modes.map((mode) => ({ id: mode.id, label: mode.label, children: null }))
												: [{ id: app.id, label: app.label, children: null }]
										}
										activeModeId={app.id === activeAppId ? (activeModeId ?? app.modes[0]?.id ?? app.id) : (app.modes[0]?.id ?? app.id)}
										onActiveModeChange={app.id === activeAppId ? setActiveModeId : undefined}
										chrome={false}
									>
										{app.id === activeAppId ? (
											<ShellModeCanvas
												windowKinds={goldenWindowKinds}
												defaultLayout={activeApp.defaultLayout as WindowLayout}
												activeWindowId={activeWindowKindId}
												onActiveWindowChange={handleActiveWindowChange}
											/>
										) : null}
									</App>
								),
							}))}
							activeAppId={activeAppId}
							onActiveAppChange={setActiveAppId}
							chrome={false}
						/>
					}
				/>
				{searchItemsResolved.length > 0 && <UISearch items={searchItemsResolved} open={searchOpen} onOpenChange={setSearchOpen} />}
				<UIFind open={findOpen} onOpenChange={setFindOpen} />
			</UIFindProvider>
		</AppContext.Provider>
	);
};

//#region ­ƒº¬Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("ProductView", () => {
		it("synthesizes default panel toggles for a single-app workbench", () => {
			const wb = new ProductRuntime();
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
			wb.addApp(app);
			const markup = renderToStaticMarkup(<ProductView runtime={wb} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />);

			expect(markup).toContain('data-panel="leftSidePanel"');
			expect(markup).toContain('id="ui.panelToggle.workbench"');
			expect(markup).toContain('id="ui.panelToggle.details"');
		});

		it("merges appwide tools, selection, options, and window kinds with the active mode", () => {
			const wb = new ProductRuntime();
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

		it("renders a leading mode dropdown when an app has multiple modes", () => {
			const wb = new ProductRuntime();
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
			const markup = renderToStaticMarkup(<ProductView runtime={wb} />);

			expect(markup).toContain('id="ui.mode.select.app.trigger"');
			expect(markup).not.toContain("ui.modeNav.app");
		});
	});
}
//#endregion ­ƒº¬Tests

//#endregion ­ƒôªworkbench-view.tsx

//#region ­ƒôªworkbench-mount.tsx
type ElementsDomRoot = HTMLElement & { __elementsReactRoot?: Root };

function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
	return document.getElementById(id) as T | null;
}

/** @emoji ÔÜø´©Å Imperative React root helpers for workbench shells. */
export class ReactUI {
	private static mountedRoot: Root | null = null;

	/** @emoji 🖥️ Mounts a {@link ProductRuntime} shell into `#root` (or `rootId`) with {@link ProductView}. */
	static mount(runtime: ProductRuntime, rootId = "root"): void {
		if (typeof document === "undefined") return;
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		if (!rootElement) {
			throw new Error(`React root #${rootId} missing.`);
		}
		rootElement.__elementsReactRoot ??= createRoot(rootElement);
		ReactUI.mountedRoot = rootElement.__elementsReactRoot;
		rootElement.__elementsReactRoot.render(<ProductView runtime={runtime} />);
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

/** @emoji 🖥️ Loads a {@link ProductRuntime} asynchronously then mounts {@link ProductView}. */
export async function mountAsyncReactApp(loadRuntime: () => Promise<ProductRuntime>, rootId = "root"): Promise<void> {
	ReactUI.mount(await loadRuntime(), rootId);
}

//#endregion ­ƒôªworkbench-mount.tsx
