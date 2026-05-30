// #region 🧲Header

// 💻 ui/react/index.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Shared export surface for elements ui primitives.

// #endregion 🧲Header

// #region 🔌Adapters

import * as AccordionPrimitive from "@radix-ui/react-accordion";
import * as AvatarPrimitive from "@radix-ui/react-avatar";
import * as CollapsiblePrimitive from "@radix-ui/react-collapsible";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import * as DropdownMenuPrimitive from "@radix-ui/react-dropdown-menu";
import * as HoverCardPrimitive from "@radix-ui/react-hover-card";
import * as PopoverPrimitive from "@radix-ui/react-popover";
import * as ScrollAreaPrimitive from "@radix-ui/react-scroll-area";
import * as SelectPrimitive from "@radix-ui/react-select";
import * as SliderPrimitive from "@radix-ui/react-slider";
import * as TabsPrimitive from "@radix-ui/react-tabs";
import * as TogglePrimitive from "@radix-ui/react-toggle";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import * as dagre from "dagre";
import Fuse, { type FuseResult } from "fuse.js";
import i18next from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import * as React from "react";
import * as ResizablePrimitive from "react-resizable-panels";
import * as THREE from "three";

import { closestCenter, DndContext, DragEndEvent, PointerSensor, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
import { SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Slot } from "@radix-ui/react-slot";
import {
  Clone,
  Edges,
  GizmoHelper,
  GizmoViewport,
  Grid,
  Line as DreiLine,
  OrbitControls,
  Outlines,
  PerspectiveCamera,
  Text as DreiText,
  TransformControls,
  useGLTF,
} from "@react-three/drei";
import {
  Canvas as ThreeCanvas,
  createPortal as r3fCreatePortal,
  ThreeEvent,
  useFrame,
  useStore,
  useThree,
} from "@react-three/fiber";
import {
  applyNodeChanges,
  Background,
  BackgroundVariant,
  BaseEdge,
  ConnectionMode,
  getBezierPath,
  Handle,
  MiniMap,
  Position,
  ReactFlow,
  ReactFlowProvider,
  SelectionMode,
  useInternalNode,
  useReactFlow,
  useStoreApi,
  ViewportPortal,
} from "@xyflow/react";
import { cva, type VariantProps } from "class-variance-authority";
import { ClassValue, clsx } from "clsx";
import { Command as CommandPrimitive } from "cmdk";
import { forceCenter, forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY, Simulation, SimulationLinkDatum, SimulationNodeDatum } from "d3-force";
import type { LucideIcon } from "lucide-react";
import {
  Plus as AddIcon,
  AlertCircle as AlertCircleIcon,
  BookOpen as BookIcon,
  Camera as CameraIcon,
  Check as CheckIcon,
  CheckIcon as CheckIconAlt,
  ChevronDown as ChevronDownIcon,
  ChevronDownIcon as ChevronDownIconAlt,
  ChevronLeft as ChevronLeftIcon,
  ChevronRight as ChevronRightIcon,
  ChevronsUpDown as ChevronsUpDownIcon,
  X as CloseIcon,
  XIcon as CloseIconAlt,
  FileText as DocumentIcon,
  ExternalLink as ExternalLinkIcon,
  Folder as FolderIcon,
  GripVertical as GripVerticalIcon,
  Image as BoardIconRasterGlyphIcon,
  ImagePlus as BoardIconFileImportIcon,
  Info as InfoIcon,
  Lightbulb as LightbulbIcon,
  Maximize2 as Maximize2Icon,
  MessageSquare as MessageSquareIcon,
  Minimize2 as Minimize2Icon,
  ArrowLeft as NavigateBackIcon,
  ArrowRight as NavigateForwardIcon,
  ArrowUp as NavigateUpIcon,
  Minus as RemoveIcon,
  SearchIcon,
  Hand as HandIcon,
  Lasso as LassoIcon,
  LayoutGrid as LayoutGridIcon,
  MousePointer2 as MousePointerIcon,
  MoreHorizontal as MoreHorizontalIcon,
  FolderOpen as FolderOpenIcon,
  Plus as PlusIcon,
  Filter as FilterIcon,
  Settings2 as Settings2Icon,
  Shapes as BoardIconCatalogGlyphIcon,
  Sigma as BoardIconMathGlyphIcon,
  Smile as BoardIconEmojiGlyphIcon,
  TriangleAlert as TriangleAlertIcon,
  GraduationCap as TutorialIcon,
} from "lucide-react";
import { createPortal } from "react-dom";
import { createRoot, type Root } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import { useHotkeys } from "react-hotkeys-hook";
import { initReactI18next, useTranslation } from "react-i18next";
import { Link, useNavigate } from "react-router";
import { twMerge } from "tailwind-merge";
// #endregion 🔌Adapters

// #region 🔌Ports
/** @emoji ⚛️ Host surface for React runtime wiring (implemented by 🔌Adapters). */
export interface ReactHostPort {
  readonly createElement: typeof React.createElement;
  readonly useState: typeof React.useState;
  readonly useEffect: typeof React.useEffect;
  readonly useMemo: typeof React.useMemo;
  readonly useCallback: typeof React.useCallback;
  readonly useRef: typeof React.useRef;
  readonly useContext: typeof React.useContext;
  readonly useLayoutEffect: typeof React.useLayoutEffect;
  readonly useSyncExternalStore: typeof React.useSyncExternalStore;
  readonly useId: typeof React.useId;
  readonly useImperativeHandle: typeof React.useImperativeHandle;
  readonly memo: typeof React.memo;
  readonly forwardRef: typeof React.forwardRef;
  readonly lazy: typeof React.lazy;
  readonly Suspense: typeof React.Suspense;
  readonly createContext: typeof React.createContext;
}

/** @emoji 🕸️ Host surface for diagram runtime (implemented by 🔌Adapters). */
export interface FlowHostPort {
  readonly flow: typeof ReactFlow;
  readonly provider: typeof ReactFlowProvider;
}

/** @emoji 🧊 Host surface for three.js / R3F (implemented by 🔌Adapters). */
export interface ThreeHostPort {
  readonly canvas: typeof ThreeCanvas;
  readonly drei: { OrbitControls: typeof OrbitControls; Grid: typeof Grid };
}

/** @emoji 🧊 Scene host surface for puzzle/cad R3F + three.js (implemented by 🔌Adapters). */
export interface SceneHostPort {
  readonly fiber: {
    readonly canvas: typeof ThreeCanvas;
    readonly createPortal: typeof r3fCreatePortal;
    readonly useFrame: typeof useFrame;
    readonly useStore: typeof useStore;
    readonly useThree: typeof useThree;
  };
  readonly drei: {
    readonly Clone: typeof Clone;
    readonly Line: typeof DreiLine;
    readonly OrbitControls: typeof OrbitControls;
    readonly Outlines: typeof Outlines;
    readonly PerspectiveCamera: typeof PerspectiveCamera;
    readonly Text: typeof DreiText;
    readonly TransformControls: typeof TransformControls;
    readonly useGLTF: typeof useGLTF;
    readonly Grid: typeof Grid;
  };
  readonly three: typeof THREE;
}
// #endregion 🔌Ports

// #region 🔌PortWiring
/** @emoji 🔌 Default host ports — inject test doubles via module assignment before render. */
export let reactHostPort: ReactHostPort = {
  createElement: React.createElement,
  useState: React.useState,
  useEffect: React.useEffect,
  useMemo: React.useMemo,
  useCallback: React.useCallback,
  useRef: React.useRef,
  useContext: React.useContext,
  useLayoutEffect: React.useLayoutEffect,
  useSyncExternalStore: React.useSyncExternalStore,
  useId: React.useId,
  useImperativeHandle: React.useImperativeHandle,
  memo: React.memo,
  forwardRef: React.forwardRef,
  lazy: React.lazy,
  Suspense: React.Suspense,
  createContext: React.createContext,
};

/** @emoji 🔌 Default diagram host port wired to @xyflow/react adapters. */
export let flowHostPort: FlowHostPort = {
  flow: ReactFlow,
  provider: ReactFlowProvider,
};

/** @emoji 🔌 Default R3F host port wired to fiber/drei adapters. */
export let threeHostPort: ThreeHostPort = {
  canvas: ThreeCanvas,
  drei: { OrbitControls, Grid },
};

/** @emoji 🔌 Default scene host port wired to fiber/drei/three adapters. */
export let sceneHostPort: SceneHostPort = {
  fiber: {
    canvas: ThreeCanvas,
    createPortal: r3fCreatePortal,
    useFrame,
    useStore,
    useThree,
  },
  drei: {
    Clone,
    Line: DreiLine,
    OrbitControls,
    Outlines,
    PerspectiveCamera,
    Text: DreiText,
    TransformControls,
    useGLTF,
    Grid,
  },
  three: THREE,
};

/** @emoji 🔌 JSX aliases for diagram / R3F hosts (use instead of adapter imports in domain JSX). */
export const HostReactFlow = flowHostPort.flow;
export const HostReactFlowProvider = flowHostPort.provider;
export const HostThreeCanvas = threeHostPort.canvas;
export const HostSceneCanvas = sceneHostPort.fiber.canvas;
export type { ThreeEvent };
// #endregion 🔌PortWiring

// #region 🎼Utilities

// Generic utility and type definitions that make @ui/react self-contained.
// These MUST NOT depend on any external semio package.

/**
 * Merges CSS class names using Tailwind merge.
 **/
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// #region 🖱️ContextMenu

const contextMenuContentClassName =
  "bg-transparent backdrop-blur-sm w-auto min-w-[10rem] overflow-hidden border p-single z-temporary text-foreground";
const contextMenuItemClassName =
  "text-foreground hover:bg-hover-temporary focus:bg-hover-temporary relative flex items-center gap-single p-single text-sm outline-none whitespace-nowrap cursor-default select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50";
const contextMenuShortcutClassName = "ml-auto text-xs text-muted-foreground pl-tiny";

/**
 * 🧩 Serializable right-click entry for {@link ContextMenu} and board/window surfaces.
 **/
export interface ContextMenuItem {
  id: string;
  label?: string;
  icon?: LucideIcon | string;
  shortcut?: string;
  disabled?: boolean;
  separator?: boolean;
  checked?: boolean;
  destructive?: boolean;
  onSelect?: (event: Event) => void;
  children?: ContextMenuItem[];
}

function renderContextMenuIcon(icon: ContextMenuItem["icon"]): React.ReactNode {
  if (!icon) {
    return null;
  }
  if (typeof icon === "string") {
    return <span className="text-base shrink-0">{icon}</span>;
  }
  const Icon = icon;
  return <Icon className="size-small shrink-0" />;
}

/**
 * 🧩 Recursively renders {@link ContextMenuItem} rows for Radix dropdown menu surfaces (right-click host).
 **/
export function renderContextMenuItems(items: ContextMenuItem[] | undefined, onClose?: () => void): React.ReactNode {
  if (!items?.length) {
    return null;
  }
  const rows: React.ReactNode[] = [];
  for (const item of items) {
    if (item.separator) {
      rows.push(<DropdownMenuPrimitive.Separator key={`${item.id}-sep`} className="h-px bg-border my-single" />);
      continue;
    }
    if (item.children?.length) {
      rows.push(
        <DropdownMenuPrimitive.Sub key={item.id}>
          <DropdownMenuPrimitive.SubTrigger
            disabled={item.disabled}
            className={cn(contextMenuItemClassName, item.destructive && "text-destructive focus:bg-destructive/10")}
          >
            {renderContextMenuIcon(item.icon)}
            <span className="truncate">{item.label ?? item.id}</span>
            <span className={contextMenuShortcutClassName}>{item.shortcut}</span>
          </DropdownMenuPrimitive.SubTrigger>
          <DropdownMenuPrimitive.Portal>
            <DropdownMenuPrimitive.SubContent className={contextMenuContentClassName}>{renderContextMenuItems(item.children, onClose)}</DropdownMenuPrimitive.SubContent>
          </DropdownMenuPrimitive.Portal>
        </DropdownMenuPrimitive.Sub>,
      );
      continue;
    }
    if (item.checked !== undefined) {
      rows.push(
        <DropdownMenuPrimitive.Item
          key={item.id}
          disabled={item.disabled}
          className={cn(contextMenuItemClassName, item.destructive && "text-destructive focus:bg-destructive/10")}
          onSelect={(event) => {
            item.onSelect?.(event as unknown as Event);
            onClose?.();
          }}
        >
          <span className="size-small shrink-0 text-center">{item.checked ? "✓" : ""}</span>
          {renderContextMenuIcon(item.icon)}
          <span className="truncate">{item.label ?? item.id}</span>
          {item.shortcut ? <span className={contextMenuShortcutClassName}>{item.shortcut}</span> : null}
        </DropdownMenuPrimitive.Item>,
      );
      continue;
    }
    rows.push(
      <DropdownMenuPrimitive.Item
        key={item.id}
        disabled={item.disabled}
        className={cn(contextMenuItemClassName, item.destructive && "text-destructive focus:bg-destructive/10")}
        onSelect={(event) => {
          item.onSelect?.(event as unknown as Event);
          onClose?.();
        }}
      >
        {renderContextMenuIcon(item.icon)}
        <span className="truncate">{item.label ?? item.id}</span>
        {item.shortcut ? <span className={contextMenuShortcutClassName}>{item.shortcut}</span> : null}
      </DropdownMenuPrimitive.Item>,
    );
  }
  return <>{rows}</>;
}

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

function getDocumentBody(): HTMLElement | null {
  return typeof document === "undefined" ? null : document.body;
}

function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
  return typeof document === "undefined" ? null : (document.getElementById(id) as T | null);
}

function queryElement<T extends Element = HTMLElement>(selector: string, root?: ParentNode | null): T | null {
  return (root ?? (typeof document === "undefined" ? null : document))?.querySelector(selector) as T | null;
}

function renderPortalInto(children: React.ReactNode, container: Element | DocumentFragment | null | undefined): React.ReactNode {
  return container ? createPortal(children, container) : null;
}

function renderContextMenuTrigger(point: { x: number; y: number } | null): React.ReactNode {
  const trigger = (
    <DropdownMenuPrimitive.Trigger asChild>
      <span
        aria-hidden
        style={{
          height: 1,
          left: point?.x ?? 0,
          opacity: 0,
          pointerEvents: "none",
          position: "fixed",
          top: point?.y ?? 0,
          width: 1,
        }}
      />
    </DropdownMenuPrimitive.Trigger>
  );
  return renderPortalInto(trigger, getDocumentBody()) ?? trigger;
}

export interface ContextMenuProps {
  items?: ContextMenuItem[];
  children: React.ReactNode;
}

/**
 * 🧩 Right-click menu via Radix dropdown primitives; passes children through when `items` is empty.
 **/
export const ContextMenu: React.FC<ContextMenuProps> = ({ items, children }) => {
  const [open, setOpen] = reactHostPort.useState(false);
  const [point, setPoint] = reactHostPort.useState<{ x: number; y: number } | null>(null);
  const close = reactHostPort.useCallback(() => setOpen(false), []);
  if (!items?.length) {
    return <>{children}</>;
  }
  return (
    <DropdownMenuPrimitive.Root modal={false} onOpenChange={setOpen} open={open}>
      <div
        className="contents"
        onContextMenu={(event) => {
          event.preventDefault();
          setPoint({ x: event.clientX, y: event.clientY });
          setOpen(true);
        }}
      >
        {children}
      </div>
      {renderContextMenuTrigger(point)}
      <DropdownMenuPrimitive.Portal>
        <DropdownMenuPrimitive.Content
          align="start"
          avoidCollisions={false}
          className={contextMenuContentClassName}
          onCloseAutoFocus={(event) => event.preventDefault()}
          side="bottom"
          sideOffset={0}
          style={point ? { left: point.x, position: "fixed", top: point.y } : undefined}
        >
          {renderContextMenuItems(items, close)}
        </DropdownMenuPrimitive.Content>
      </DropdownMenuPrimitive.Portal>
    </DropdownMenuPrimitive.Root>
  );
};

export interface ContextMenuControllerProps {
  open: boolean;
  position: { x: number; y: number } | null;
  items: ContextMenuItem[];
  onOpenChange: (open: boolean) => void;
}

function renderFixedContextMenuItems(items: ContextMenuItem[], onClose: () => void): React.ReactNode {
  return items.map((item) => {
    if (item.separator) {
      return <div key={`${item.id}-sep`} className="h-px bg-border my-single" role="separator" />;
    }
    const role = item.checked === undefined ? "menuitem" : "menuitemcheckbox";
    return (
      <button
        key={item.id}
        aria-checked={item.checked}
        aria-disabled={item.disabled}
        className={cn(
          contextMenuItemClassName,
          "w-full bg-transparent text-left",
          item.destructive && "text-destructive focus:bg-destructive/10",
        )}
        data-disabled={item.disabled ? "" : undefined}
        disabled={item.disabled}
        onClick={(event) => {
          item.onSelect?.(event.nativeEvent);
          onClose();
        }}
        role={role}
        type="button"
      >
        {item.checked !== undefined ? <span className="size-small shrink-0 text-center">{item.checked ? "✓" : ""}</span> : null}
        {renderContextMenuIcon(item.icon)}
        <span className="truncate">{item.label ?? item.id}</span>
        {item.shortcut ? <span className={contextMenuShortcutClassName}>{item.shortcut}</span> : null}
      </button>
    );
  });
}

/**
 * 🧩 Controlled right-click menu anchored at viewport coordinates (board canvas bridge). Portals to `document.body` for correct `fixed` placement under transformed UI; outside-dismiss uses `window` bubble listeners so they run after the board `eventSurface` bubble path and after `window` capture (441–442 used `document` capture and swallowed input).
 **/
export const ContextMenuController: React.FC<ContextMenuControllerProps> = ({ open, position, items, onOpenChange }) => {
  const close = reactHostPort.useCallback(() => onOpenChange(false), [onOpenChange]);
  const menuRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  reactHostPort.useEffect(() => {
    if (!open || !items.length || !position) {
      return undefined;
    }
    const handlePointerDown = (event: PointerEvent): void => {
      const target = event.target;
      if (target instanceof Node && menuRef.current?.contains(target)) {
        return;
      }
      onOpenChange(false);
    };
    const handleKeyDown = (event: KeyboardEvent): void => {
      if (event.key === "Escape") {
        onOpenChange(false);
      }
    };
    const bindings = createDOMEventBinding();
    bindings.listen(window, "pointerdown", handlePointerDown, false);
    bindings.listen(window, "keydown", handleKeyDown, false);
    return () => bindings.dispose();
  }, [items.length, onOpenChange, open, position?.x, position?.y]);
  if (!items.length) {
    return null;
  }
  if (!open || !position) {
    return null;
  }
  return renderPortalInto(
    <div
      className={contextMenuContentClassName}
      onContextMenu={(event) => event.preventDefault()}
      ref={menuRef}
      role="menu"
      style={{ left: position.x, position: "fixed", top: position.y }}
    >
      {renderFixedContextMenuItems(items, close)}
    </div>,
    getDocumentBody(),
  );
};

// #endregion 🖱️ContextMenu

/** @emoji 🎚 Surface expertise tier for chrome and label resolution. */
export enum Expertise {
	BEGINNER = "beginner",
	NORMAL = "normal",
	EXPERT = "expert",
}

let _expertiseProvider: (() => Expertise) | undefined;

/**
 * Registers a function that returns the current expertise level.
 **/
export function setExpertiseProvider(fn: () => Expertise) {
  _expertiseProvider = fn;
}

// #region 🌈SurfaceChrome
/** @emoji 🌈 Document-level UI chrome shared by Elements shells: theme (system/light/dark), device (desktop/tablet/mobile), and tooltip expertise — mirrors sketchpad `Theme` / `Device` behavior on `documentElement`. */
export type ElementsSurfaceTheme = "system" | "light" | "dark";
export type ElementsSurfaceDevice = "desktop" | "tablet" | "mobile";

export interface ElementsSurfaceChromeInput {
  theme: ElementsSurfaceTheme;
  device: ElementsSurfaceDevice;
  expertise: Expertise;
}

function applyDocumentBodyBaseColors(): void {
  if (typeof document === "undefined") return;
  document.body.style.backgroundColor = "var(--base)";
  document.body.style.color = "var(--foreground)";
}

/**
 * @emoji 🌈 Imperative surface chrome controller for class-based shells; returns a cleanup that reverts DOM state and resets tooltip expertise.
 */
export function applyElementsSurfaceChrome({ theme, device, expertise }: ElementsSurfaceChromeInput): () => void {
  setExpertiseProvider(() => expertise);
  const cleanups: Array<() => void> = [() => setExpertiseProvider(() => Expertise.NORMAL)];

  if (typeof window !== "undefined" && typeof document !== "undefined") {
    const root = document.documentElement;
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const bindings = createDOMEventBinding();
    const applyTheme = (): void => {
      const prefersDark = mq.matches;
      const dark = theme === "dark" || (theme === "system" && prefersDark);
      root.classList.toggle("dark", dark);
      applyDocumentBodyBaseColors();
    };
    applyTheme();
    bindings.listen(mq, "change", applyTheme);
    cleanups.push(() => {
      bindings.dispose();
      root.classList.remove("dark");
      document.body.style.backgroundColor = "";
      document.body.style.color = "";
    });
  }

  if (typeof document !== "undefined") {
    const root = document.documentElement;
    root.dataset.uiDevice = device;
    if (device === "tablet") {
      root.classList.add("touch");
    } else {
      root.classList.remove("touch");
    }
    cleanups.push(() => {
      delete root.dataset.uiDevice;
      root.classList.remove("touch");
    });
  }

  return () => {
    while (cleanups.length > 0) {
      cleanups.pop()?.();
    }
  };
}

/**
 * @emoji 🌓 Syncs `document.documentElement` (`dark`, `touch`, `data-ui-device`), body base colors, and {@link setExpertiseProvider} for tooltips; returns `mobile` for {@link AppProps.mobile}.
 */
export function useElementsSurfaceChrome({ theme, device, expertise }: ElementsSurfaceChromeInput): { mobile: boolean } {
  reactHostPort.useEffect(() => applyElementsSurfaceChrome({ theme, device, expertise }), [device, expertise, theme]);

  return { mobile: device === "mobile" };
}
// #endregion 🌈SurfaceChrome

// #region 🪁I18n Resources

// Shared UI translation bundles and initialization for all multilingual UI surfaces.
// UI bundles MUST keep translation resources in source code and MUST not rely on sketchpad-local JSON files.

const elementUiTranslationBundles = {
  de: {
    translation: JSON.parse(String.raw`{
  "semio": {
    "label": {
      "normal": "",
      "beginner": ""
    },
    "file": {
      "name": "Name",
      "size": "Groesse",
      "created": "Erstellt",
      "updated": "Aktualisiert"
    },
    "folder": {
      "created": "Erstellt",
      "updated": "Aktualisiert"
    },
    "sketchpad": {
      "label": {
        "normal": "",
        "beginner": ""
      },
      "navbar": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "back": {
          "label": {
            "normal": "Zurueck",
            "beginner": "Klicken, um zurueckzugehen, halten um Historie zu sehen"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Links"
        },
        "forward": {
          "label": {
            "normal": "Vorwaerts",
            "beginner": "Klicken, um vorwaerts zu gehen, halten um Historie zu sehen"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Rechts"
        },
        "up": {
          "label": {
            "normal": "Eine Ebene hoch",
            "beginner": "Klicken, um eine Ebene hoeher in der Navigationshierarchie zu gehen"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Oben"
        },
        "kits": {
          "label": {
            "normal": "Kits",
            "beginner": "Klicken, um alle Kits zu sehen"
          }
        },
        "navigationButtons": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigationsknoepfe"
          }
        },
        "docs": {
          "label": {
            "normal": "Dokumentation",
            "beginner": "Klicken, um Dokumentation anzuzeigen"
          },
          "hotkey": "Ctrl+Shift+D"
        },
        "search": {
          "label": {
            "normal": "Suche",
            "beginner": "Nach Inhalten suchen"
          },
          "open": {
            "label": {
              "normal": "Suche",
              "beginner": "Klicken, um Suche zu oeffnen und schnell zu jedem Element zu navigieren"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Ctrl+K"
          },
          "close": {
            "label": {
              "normal": "Suche schliessen",
              "beginner": "Klicken, um den Suchdialog zu schliessen"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Escape"
          },
          "title": {
            "label": {
              "normal": "Suche",
              "beginner": "Suche"
            }
          },
          "description": {
            "label": {
              "normal": "Nach Kits, Entwuerfen, Typen und mehr suchen",
              "beginner": "Nach Kits, Entwuerfen, Typen und mehr suchen"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Suchen...",
              "beginner": "Suchen..."
            }
          },
          "noResults": {
            "label": {
              "normal": "Keine Ergebnisse gefunden",
              "beginner": "Keine Ergebnisse gefunden"
            }
          }
        },
        "focus": {
          "label": {
            "normal": "Fokusmodus",
            "beginner": "Fokusmodus umschalten um Ablenkungen auszublenden"
          },
          "open": {
            "label": {
              "normal": "Fokus",
              "beginner": "Klicken, um in den Fokusmodus zu wechseln und Ablenkungen auszublenden"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Ctrl+Shift+F"
          },
          "close": {
            "label": {
              "normal": "Fokus verlassen",
              "beginner": "Klicken, um den Fokusmodus zu verlassen und alle UI-Elemente anzuzeigen"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Escape"
          },
          "input": {
            "label": {
              "normal": "Fokus-Eingabe",
              "beginner": "Tippen, um nach einem Element zum Fokussieren zu suchen"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Nach einem Element suchen...",
              "beginner": "Nach einem Element suchen..."
            }
          },
          "title": {
            "label": {
              "normal": "Fokus",
              "beginner": "Fokus"
            }
          },
          "description": {
            "label": {
              "normal": "Auf ein Element in der aktuellen Ansicht fokussieren",
              "beginner": "Auf ein Element in der aktuellen Ansicht fokussieren"
            }
          },
          "other": {
            "label": {
              "normal": "Andere",
              "beginner": "Andere"
            }
          }
        },
        "copyJsonToClipboard": {
          "label": {
            "normal": "JSON kopieren",
            "beginner": "Den aktuellen Sketchpad-JSON-Status in die Zwischenablage kopieren"
          },
          "hotkey": "Ctrl+Shift+J"
        },
        "breadcrumb": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "designs": {
            "label": {
              "normal": "Entwuerfe",
              "beginner": "Entwuerfe"
            }
          },
          "types": {
            "label": {
              "normal": "Typen",
              "beginner": "Typen"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualitaeten",
              "beginner": "Qualitaeten"
            }
          },
          "temporary": {
            "label": {
              "normal": "Temporär",
              "beginner": "Temporär"
            }
          },
          "local": {
            "label": {
              "normal": "Lokal",
              "beginner": "Lokal"
            }
          },
          "remote": {
            "label": {
              "normal": "Remote",
              "beginner": "Remote"
            }
          },
          "files": {
            "label": {
              "normal": "Files",
              "beginner": "Files"
            }
          },
          "authors": {
            "label": {
              "normal": "Autoren",
              "beginner": "Autoren"
            }
          }
        },
        "tutorials": {
          "label": {
            "normal": "Tutorials",
            "beginner": "Tutorials"
          }
        },
        "tutorial": {
          "controls": {
            "stop": {
              "label": {
                "normal": "Tutorial beenden",
                "beginner": "Klicken, um das aktuelle Tutorial zu beenden"
              }
            },
            "previous": {
              "label": {
                "normal": "Vorheriger Schritt",
                "beginner": "Zum vorherigen Schritt im Tutorial gehen"
              }
            },
            "playPause": {
              "label": {
                "normal": "Abspielen/Pause",
                "beginner": "Tutorial abspielen oder pausieren"
              }
            },
            "next": {
              "label": {
                "normal": "Nächster Schritt",
                "beginner": "Zum nächsten Schritt im Tutorial gehen"
              }
            }
          }
        },
        "recording": {
          "controls": {
            "playPause": {
              "label": {
                "normal": "Aufnahme abspielen/pausieren",
                "beginner": "Aufnahme abspielen oder pausieren"
              }
            },
            "stop": {
              "label": {
                "normal": "Aufnahme beenden",
                "beginner": "Aufnahme beenden und speichern"
              }
            }
          }
        },
        "createKit": {
          "label": {
            "normal": "Kit erstellen",
            "beginner": "Klicken, um ein neues Kit zu erstellen"
          }
        },
        "createDesign": {
          "label": {
            "normal": "Entwurf erstellen",
            "beginner": "Klicken, um einen neuen Entwurf zu erstellen"
          }
        },
        "createChild": {
          "label": {
            "normal": "Kind erstellen",
            "beginner": "Klicken, um ein Kind-Artefakt zu erstellen"
          }
        },
        "createType": {
          "label": {
            "normal": "Typ erstellen",
            "beginner": "Klicken, um einen neuen Typ zu erstellen"
          }
        },
        "createVersion": {
          "label": {
            "normal": "Version erstellen",
            "beginner": "Klicken, um eine neue Version zu erstellen"
          }
        },
        "searchInput": {
          "label": {
            "normal": "Such-Eingabe",
            "beginner": "Tippen, um nach Elementen zu suchen"
          }
        },
        "panelToggle": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "workbench": {
            "label": {
              "normal": "Workbench umschalten",
              "beginner": "Das Workbench-Panel auf der linken Seite ein- oder ausblenden"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "hud": {
            "label": {
              "normal": "HUD umschalten",
              "beginner": "Das HUD-Panel in der Mitte ein- oder ausblenden"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "right": {
            "label": {
              "normal": "Rechtes Panel umschalten",
              "beginner": "Das rechte Panel fuer Details und Einstellungen ein- oder ausblenden"
            }
          },
          "tools": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "toolbar": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "stats": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "details": {
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "chat": {
            "label": {
              "normal": "Chat umschalten",
              "beginner": "Chat-Panel umschalten"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen umschalten",
              "beginner": "Einstellungen-Panel umschalten"
            },
            "show": {
              "label": {
                "normal": "Anzeigen",
                "beginner": "Anzeigen"
              }
            }
          },
          "leftSidePanel": {
            "label": {
              "normal": "Linkes Panel umschalten",
              "beginner": "Das linke Seitenfeld mit Workbench-Tabs umschalten"
            }
          },
          "rightSidePanel": {
            "label": {
              "normal": "Rechtes Panel umschalten",
              "beginner": "Das rechte Seitenfeld mit Detail-Tabs umschalten"
            }
          },
          "hudPanel": {
            "label": {
              "normal": "HUD-Panel umschalten",
              "beginner": "Das zentrale HUD-Panel umschalten"
            }
          }
        },
        "home": {
          "label": {
            "normal": "Home",
            "beginner": "Home"
          }
        },
        "kitName": {
          "label": {
            "normal": "Kit Name",
            "beginner": "Kit Name"
          }
        },
        "kitVersion": {
          "label": {
            "normal": "Kit Version",
            "beginner": "Kit Version"
          }
        },
        "name": {
          "label": {
            "normal": "Name",
            "beginner": "Name"
          }
        },
        "design": {
          "label": {
            "normal": "Design",
            "beginner": "Design"
          }
        },
        "type": {
          "label": {
            "normal": "Typ",
            "beginner": "Typ"
          }
        },
        "quality": {
          "label": {
            "normal": "Quality",
            "beginner": "Quality"
          }
        },
        "navigation": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigation"
          }
        },
        "panelToggles": {
          "label": {
            "normal": "Panel-Umschalter",
            "beginner": "Panel-Umschalter"
          }
        },
        "fullscreenToggle": {
          "label": {
            "normal": "Fullscreen Toggle",
            "beginner": "Fullscreen Toggle"
          }
        }
      },
      "panel": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "chat": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "placeholder": {
            "label": {
              "normal": "Fragen Sie etwas...",
              "beginner": "Fragen Sie etwas..."
            }
          }
        },
        "settings": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "theme": {
            "label": {
              "normal": "Design",
              "beginner": "Wählen Sie das Farbschema für die Anwendung"
            },
            "dark": {
              "label": {
                "normal": "Dunkel",
                "beginner": "Dunkles Farbschema verwenden"
              }
            },
            "light": {
              "label": {
                "normal": "Hell",
                "beginner": "Helles Farbschema verwenden"
              }
            },
            "system": {
              "label": {
                "normal": "System",
                "beginner": "Systemdesign-Einstellung folgen"
              }
            }
          },
          "device": {
            "label": {
              "normal": "Geraet",
              "beginner": "Geraetemodus fuer die Interaktion waehlen"
            },
            "desktop": {
              "label": {
                "normal": "Desktop",
                "beginner": "Optimiertes Layout für Desktop-Computer"
              }
            },
            "tablet": {
              "label": {
                "normal": "Tablet",
                "beginner": "Optimiertes Layout für Tablets"
              }
            },
            "mobile": {
              "label": {
                "normal": "Mobil",
                "beginner": "Optimiertes Layout für mobile Geräte"
              }
            }
          },
          "mode": {
            "label": {
              "normal": "Modus",
              "beginner": "Wählen Sie den Benutzeroberflächenmodus: Experte (minimale Tooltips), Normal (Standard) oder Anfänger (detaillierte Hilfe)"
            },
            "dev": {
              "label": {
                "normal": "Entwickler",
                "beginner": "Entwicklermodus mit erweiterten Werkzeugen und Debugging-Funktionen"
              }
            },
            "user": {
              "label": {
                "normal": "Benutzer",
                "beginner": "Standardbenutzermodus für reguläre Operationen"
              }
            }
          },
          "expertise": {
            "label": {
              "normal": "Erfahrung",
              "beginner": "Wählen Sie Ihre Erfahrungsstufe um die Komplexität der Oberfläche anzupassen"
            },
            "beginner": {
              "label": {
                "normal": "Anfänger",
                "beginner": "Detaillierte Erklärungen und Tutorials anzeigen"
              }
            },
            "normal": {
              "label": {
                "normal": "Normal",
                "beginner": "Standard-Tooltips und Hilfe anzeigen"
              }
            },
            "expert": {
              "label": {
                "normal": "Experte",
                "beginner": "Minimale Tooltips für erfahrene Benutzer"
              }
            }
          }
        }
      },
      "common": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "selectVariant": "Variante auswaehlen...",
        "selectView": "Ansicht auswaehlen...",
        "search": {
          "label": {
            "normal": "Suche",
            "beginner": "Suche"
          }
        },
        "mixedValues": {
          "label": {
            "normal": "Gemischte Werte",
            "beginner": "Gemischte Werte"
          }
        },
        "selectDesign": {
          "label": {
            "normal": "Entwurf auswaehlen",
            "beginner": "Entwurf auswaehlen"
          }
        },
        "selectType": {
          "label": {
            "normal": "Typ auswaehlen",
            "beginner": "Typ auswaehlen"
          }
        },
        "no": {
          "label": {
            "normal": "Nein",
            "beginner": "Nein"
          }
        },
        "yes": {
          "label": {
            "normal": "Ja",
            "beginner": "Ja"
          }
        },
        "add": {
          "label": {
            "normal": "Hinzufügen",
            "beginner": "Hinzufügen"
          }
        },
        "remove": {
          "label": {
            "normal": "Entfernen",
            "beginner": "Entfernen"
          }
        },
        "addChild": {
          "label": {
            "normal": "Kind hinzufügen",
            "beginner": "Kind hinzufügen"
          }
        },
        "duplicateType": {
          "label": {
            "normal": "Typ duplizieren (Hover)",
            "beginner": "Typ duplizieren (Hover)"
          }
        },
        "addType": {
          "label": {
            "normal": "Typ hinzufügen",
            "beginner": "Typ hinzufügen"
          }
        },
        "addDesign": {
          "label": {
            "normal": "Entwurf hinzufügen",
            "beginner": "Entwurf hinzufügen"
          }
        },
        "settings": {
          "label": {
            "normal": "Sketchpad",
            "beginner": "Globale Sketchpad-Einstellungen"
          },
          "theme": {
            "label": {
              "normal": "Design",
              "beginner": "Farbschema wählen"
            }
          },
          "layout": {
            "label": {
              "normal": "Layout",
              "beginner": "Layoutmodus wählen"
            }
          },
          "mode": {
            "label": {
              "normal": "Modus",
              "beginner": "Oberflächenmodus wählen"
            }
          },
          "expertise": {
            "label": {
              "normal": "Erfahrungsstufe",
              "beginner": "Erfahrungsstufe wählen"
            }
          }
        }
      },
      "footer": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Feedback senden, um semio zu verbessern"
          }
        }
      },
      "app": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "home": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "title": "Home",
          "fileInput": {
            "label": {
              "normal": "Kit-Datei waehlen",
              "beginner": "Eine .zip-Kit-Datei von Ihrem Geraet auswaehlen"
            }
          },
          "searchPlaceholder": {
            "label": {
              "normal": "Kits suchen...",
              "beginner": "Kits suchen..."
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "kind": {
            "label": {
              "normal": "Art",
              "beginner": "Art"
            }
          },
          "lastUpdated": {
            "label": {
              "normal": "Zuletzt aktualisiert",
              "beginner": "Zuletzt aktualisiert"
            }
          },
          "created": {
            "label": {
              "normal": "Erstellt",
              "beginner": "Erstellt"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "kind": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "show": {
                "label": {
                  "normal": "Nach Art filtern",
                  "beginner": "Kits nach ihrem Speichertyp filtern (temporaer, lokal oder remote)"
                },
                "hotkey": "Ctrl+K"
              },
              "create": {
                "label": {
                  "normal": "Neues Kit erstellen",
                  "beginner": "Ein neues Kit dieses Typs erstellen"
                },
                "hotkey": "Ctrl+Shift+K"
              },
              "temporary": {
                "label": {
                  "normal": "Temporaere Kits anzeigen",
                  "beginner": "Kits anzeigen, die im Browser-Speicher gespeichert sind (gehen beim Neuladen verloren)"
                },
                "hotkey": "Ctrl+1"
              },
              "createTemporary": {
                "label": {
                  "normal": "Temporaeres Kit erstellen",
                  "beginner": "Ein neues temporaeres Kit erstellen, das im Browser-Speicher gespeichert wird"
                },
                "hotkey": "Ctrl+Shift+1"
              },
              "local": {
                "label": {
                  "normal": "Lokale Kits anzeigen",
                  "beginner": "Kits anzeigen, die lokal auf Ihrem Geraet gespeichert sind"
                },
                "hotkey": "Ctrl+2"
              },
              "createLocal": {
                "label": {
                  "normal": "Lokales Kit erstellen",
                  "beginner": "Ein neues Kit erstellen, das lokal auf Ihrem Geraet gespeichert wird"
                },
                "hotkey": "Ctrl+Shift+2"
              },
              "remote": {
                "label": {
                  "normal": "Remote-Kits anzeigen",
                  "beginner": "Kits anzeigen, die mit Remote-Speicher synchronisiert sind"
                },
                "hotkey": "Ctrl+3"
              },
              "createRemote": {
                "label": {
                  "normal": "Remote-Kit erstellen",
                  "beginner": "Ein neues Kit erstellen, das mit Remote-Speicher synchronisiert wird"
                },
                "hotkey": "Ctrl+Shift+3"
              }
            },
            "name": {
              "label": {
                "normal": "Nach Name filtern",
                "beginner": "Kits nach diesem spezifischen Namen filtern"
              },
              "hotkey": "Ctrl+N"
            },
            "version": {
              "label": {
                "normal": "Nach Version filtern",
                "beginner": "Kits nach dieser spezifischen Version filtern"
              },
              "hotkey": "Ctrl+V"
            },
            "band": {
              "label": {
                "normal": "Filterleiste",
                "beginner": "Filterleiste ein- oder ausblenden"
              },
              "hotkey": "Ctrl+F"
            }
          },
          "search": {
            "label": {
              "normal": "Suche",
              "beginner": "Nach Kits suchen"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Home-Einstellungen"
            },
            "theme": {
              "label": {
                "normal": "Design",
                "beginner": "Waehlen Sie das Farbschema fuer die Anwendung"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Waehlen Sie die Sprache fuer die Anwendungsoberflaeche"
              },
              "placeholder": {
                "label": {
                  "normal": "Sprache waehlen...",
                  "beginner": "Waehlen Sie die Sprache, in der die Anwendung angezeigt wird"
                }
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Waehlen Sie den Benutzeroberflaechen-Modus"
              }
            },
            "expertise": {
              "label": {
                "normal": "Erfahrung",
                "beginner": "Waehlen Sie Ihr Erfahrungsniveau"
              }
            },
            "device": {
              "label": {
                "normal": "Geraet",
                "beginner": "Waehlen Sie das Eingabegeraet"
              }
            },
            "layout": {
              "label": {
                "normal": "Layout",
                "beginner": "Waehlen Sie das Layout fuer die Kit-Uebersicht"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Konfigurieren Sie das kraftgerichtete Diagramm-Layout"
              },
              "chargeStrength": {
                "label": {
                  "normal": "Ladungsstaerke",
                  "beginner": "Steuert, wie stark Knoten sich abstossen. Negativere Werte druecken Knoten weiter auseinander."
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Verbindungsabstand",
                  "beginner": "Der Zielabstand zwischen verbundenen Knoten. Groessere Werte verteilen das Diagramm."
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Kollisionsradius",
                  "beginner": "Der Mindestabstand zwischen Knotenzentren zur Vermeidung von Ueberlappung."
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Zentrierungsstaerke",
                  "beginner": "Wie stark Knoten zur Mitte des Diagramms gezogen werden."
                }
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "createKit": {
                "label": {
                  "normal": "Kit erstellen",
                  "beginner": "Neues Kit aus der Home-Tabelle anlegen"
                }
              },
              "createVersion": {
                "label": {
                  "normal": "Version erstellen",
                  "beginner": "Neue Kit-Version aus der Home-Tabelle anlegen"
                }
              },
              "hover": {
                "label": {
                  "normal": "Kit hervorheben",
                  "beginner": "Ueber eine Kit-Zeile fahren, um sie im Diagramm hervorzuheben"
                }
              },
              "toggleSort": {
                "label": {
                  "normal": "Sortierung umschalten",
                  "beginner": "Sortierrichtung fuer diese Spalte aendern"
                }
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "kit": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "Der Name des Kits"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "Die Version des Kits"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Beschreibung",
                    "beginner": "Eine Beschreibung des Kits"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Symbol",
                    "beginner": "Das Symbol des Kits"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Bild",
                    "beginner": "Das Vorschaubild des Kits"
                  }
                }
              },
              "kits": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "Der Name der ausgewählten Kits"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "Die Version der ausgewählten Kits"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Beschreibung",
                    "beginner": "Eine Beschreibung der ausgewählten Kits"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Symbol",
                    "beginner": "Das Symbol der ausgewählten Kits"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Bild",
                    "beginner": "Das Vorschaubild der ausgewählten Kits"
                  }
                }
              }
            }
          },
          "dropzone": {
            "label": {
              "normal": "Zip-Datei zum Importieren eines Kits ablegen",
              "beginner": "Legen Sie eine Zip-Datei hier ab, um sie als neues Kit zu importieren"
            },
            "description": {
              "normal": "Nur Kits mit .semio-Ordner können importiert werden",
              "beginner": "Die Zip-Datei muss einen .semio-Ordner mit kit.db enthalten, um als Kit importiert zu werden."
            }
          },
          "noKits": {
            "label": {
              "normal": "Keine Kits",
              "beginner": "Keine Kits"
            }
          },
          "sortByName": {
            "label": {
              "normal": "Sort By Name",
              "beginner": "Sort By Name"
            }
          },
          "toggleRow": {
            "label": {
              "normal": "Zeile umschalten",
              "beginner": "Zeile umschalten"
            }
          },
          "createVersion": {
            "label": {
              "normal": "Version erstellen",
              "beginner": "Version erstellen"
            }
          },
          "hideKind": {
            "label": {
              "normal": "Art ausblenden",
              "beginner": "Art ausblenden"
            }
          },
          "showTemporary": {
            "label": {
              "normal": "Temporär anzeigen",
              "beginner": "Temporär anzeigen"
            }
          },
          "showLocal": {
            "label": {
              "normal": "Lokal anzeigen",
              "beginner": "Lokal anzeigen"
            }
          },
          "showRemote": {
            "label": {
              "normal": "Remote anzeigen",
              "beginner": "Remote anzeigen"
            }
          },
          "sortByType": {
            "label": {
              "normal": "Sort By Type",
              "beginner": "Sort By Type"
            }
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Sort By Updated At",
              "beginner": "Sort By Updated At"
            }
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Sort By Created At",
              "beginner": "Sort By Created At"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Home-Chat"
            }
          },
          "createKit": {
            "label": {
              "normal": "Kit erstellen",
              "beginner": "Kit erstellen"
            }
          },
          "createTemporary": {
            "label": {
              "normal": "Temporär erstellen",
              "beginner": "Temporär erstellen"
            }
          },
          "createLocal": {
            "label": {
              "normal": "Lokal erstellen",
              "beginner": "Lokal erstellen"
            }
          },
          "createRemote": {
            "label": {
              "normal": "Remote erstellen",
              "beginner": "Remote erstellen"
            }
          },
          "importKit": {
            "label": {
              "normal": "Kit importieren",
              "beginner": "Kit importieren"
            }
          },
          "toolbar": {
            "showTemporary": {
              "label": {
                "normal": "Temporär",
                "beginner": "Temporär"
              }
            },
            "showLocal": {
              "label": {
                "normal": "Lokal",
                "beginner": "Lokal"
              }
            },
            "showRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "createTemporary": {
              "label": {
                "normal": "Temporär",
                "beginner": "Temporär"
              }
            },
            "createLocal": {
              "label": {
                "normal": "Lokal",
                "beginner": "Lokal"
              }
            },
            "createRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Kits nach Standort filtern"
              }
            },
            "create": {
              "label": {
                "normal": "Erstellen",
                "beginner": "Neues Kit erstellen"
              }
            },
            "createKit": {
              "label": {
                "normal": "Neues Kit",
                "beginner": "Ein neues leeres Kit erstellen"
              }
            },
            "openFolder": {
              "label": {
                "normal": "Ordner oeffnen",
                "beginner": "Ein Kit aus einem Ordner oeffnen"
              }
            },
            "openFile": {
              "label": {
                "normal": "Datei oeffnen",
                "beginner": "Ein Kit aus einer Zip-Datei oeffnen"
              }
            },
            "openRemote": {
              "label": {
                "normal": "Remote oeffnen",
                "beginner": "Ein Kit von einer Remote-URL oeffnen"
              }
            },
            "createFile": {
              "label": {
                "normal": "Neues Datei-Kit",
                "beginner": "Ein Kit anlegen, das in einer Datei gespeichert wird"
              }
            },
            "createFolder": {
              "label": {
                "normal": "Neues Ordner-Kit",
                "beginner": "Ein Kit anlegen, das in einem Ordner gespeichert wird"
              }
            },
            "showFile": {
              "label": {
                "normal": "Datei-Kits anzeigen",
                "beginner": "Kits anzeigen, die als Datei gespeichert sind"
              }
            },
            "showFolder": {
              "label": {
                "normal": "Ordner-Kits anzeigen",
                "beginner": "Kits anzeigen, die in Ordnern gespeichert sind"
              }
            },
            "exportArchive": {
              "label": {
                "normal": "Archiv exportieren",
                "beginner": "Das ausgewaehlte Kit als Zip-Archiv exportieren"
              }
            }
          }
        },
        "kit": {
          "label": {
            "normal": "Kit",
            "beginner": "Kit"
          },
          "properties": {
            "label": {
              "normal": "Kit-Eigenschaften",
              "beginner": "Kit-Eigenschaften"
            }
          },
          "notFound": {
            "label": {
              "normal": "Kit nicht gefunden",
              "beginner": "Das angeforderte Kit wurde nicht gefunden"
            },
            "description": {
              "normal": "Das Kit wurde moeglicherweise entfernt oder der Link ist ungueltig.",
              "beginner": "Zurueck zur Startseite und ein anderes Kit oeffnen oder ein neues erstellen."
            }
          },
          "noKitLoaded": {
            "label": {
              "normal": "Kein Kit geladen",
              "beginner": "Kein Kit geladen"
            }
          },
          "loading": {
            "label": {
              "normal": "Kit wird geladen...",
              "beginner": "Kit wird geladen..."
            }
          },
          "notAvailable": {
            "label": {
              "normal": "Kit nicht verfuegbar",
              "beginner": "Kit nicht verfuegbar"
            }
          },
          "dropzone": {
            "label": {
              "normal": "Zip-Datei zum Importieren ablegen",
              "beginner": "Legen Sie eine Zip-Datei hier ab, um sie als Kit zu importieren oder Dateien zum aktuellen Kit hinzuzufuegen"
            },
            "description": {
              "normal": "Kits mit .semio-Ordner werden importiert, andere als Dateien hinzugefuegt",
              "beginner": "Wenn die Zip-Datei einen .semio-Ordner enthaelt, wird sie als vollstaendiges Kit importiert. Andernfalls werden die Dateien zum aktuellen Kit hinzugefuegt."
            }
          },
          "versionPlaceholder": {
            "label": {
              "label": {
                "normal": "z.B., 1.0.0",
                "beginner": "z.B., 1.0.0"
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie den Inhalt dieses Kits...",
                "beginner": "Beschreiben Sie den Inhalt dieses Kits..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "🎨 oder URL zum Icon",
                "beginner": "🎨 oder URL zum Icon"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "URL zum Vorschaubild",
                "beginner": "URL zum Vorschaubild"
              }
            }
          },
          "homepagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://beispiel.de",
                "beginner": "https://beispiel.de"
              }
            }
          },
          "licensePlaceholder": {
            "label": {
              "label": {
                "normal": "z.B., MIT, GPL-3.0, Apache-2.0",
                "beginner": "z.B., MIT, GPL-3.0, Apache-2.0"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neues Kit",
              "beginner": "Neues Kit"
            }
          },
          "defaultDesignName": {
            "label": {
              "normal": "Neuer Entwurf",
              "beginner": "Neuer Entwurf"
            }
          },
          "defaultTypeName": {
            "label": {
              "normal": "Neuer Typ",
              "beginner": "Neuer Typ"
            }
          },
          "newVersion": {
            "label": {
              "normal": "Neue Version",
              "beginner": "Neue Version"
            }
          },
          "defaultVersion": {
            "label": {
              "normal": "Standard",
              "beginner": "Standard"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "name": {
              "label": {
                "normal": "Nach Name filtern",
                "beginner": "Klicken, um Artefakte nach diesem Namen zu filtern"
              },
              "manual": "manuals/semio/kit",
              "tutorial": "hello-semio/model-design",
              "hotkey": "Ctrl+N",
              "hide": {
                "label": {
                  "normal": "Namensfilter ausblenden",
                  "beginner": "Klicken, um den Namensfilter auszublenden"
                },
                "hotkey": "Ctrl+Shift+N"
              }
            },
            "band": {
              "label": {
                "normal": "Filterleiste",
                "beginner": "Filterleiste ein- oder ausblenden"
              },
              "hotkey": "Ctrl+F"
            },
            "search": {
              "label": {
                "normal": "Suchfilter",
                "beginner": "Nach bestimmten Filtern suchen"
              },
              "hotkey": "Ctrl+Shift+F"
            }
          },
          "pieces": {
            "label": {
              "normal": "Teile",
              "beginner": "Typen und Entwuerfe in diesem Kit"
            }
          },
          "designs": {
            "label": {
              "normal": "Entwuerfe",
              "beginner": "Entwuerfe in diesem Kit"
            },
            "manual": "manuals/semio/kit#designs",
            "tutorial": "hello-semio/model-design",
            "multipleSelected": {
              "label": {
                "normal": "Mehrere ausgewählt",
                "beginner": "Mehrere ausgewählt"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "types": {
            "label": {
              "normal": "Typen",
              "beginner": "Typen in diesem Kit"
            },
            "manual": "manuals/semio/kit#types",
            "tutorial": "hello-semio/model-brick-set",
            "multipleSelected": {
              "label": {
                "normal": "Mehrere ausgewählt",
                "beginner": "Mehrere ausgewählt"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "sortByArtifact": {
            "label": {
              "normal": "Nach Artefakt sortieren",
              "beginner": "Artefakte nach ihrem Namen sortieren"
            },
            "hotkey": "Ctrl+Shift+A"
          },
          "sortByKind": {
            "label": {
              "normal": "Nach Art sortieren",
              "beginner": "Artefakte nach ihrem Typ sortieren (Entwurf, Typ, Qualitaet, usw.)"
            },
            "hotkey": "Ctrl+Shift+K"
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Nach Erstellungsdatum sortieren",
              "beginner": "Artefakte nach ihrem Erstellungsdatum sortieren"
            },
            "hotkey": "Ctrl+Shift+C"
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Nach Aenderungsdatum sortieren",
              "beginner": "Artefakte nach ihrem letzten Aenderungsdatum sortieren"
            },
            "hotkey": "Ctrl+Shift+U"
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showDesigns": {
              "label": {
                "normal": "Entwuerfe anzeigen",
                "beginner": "Klicken, um alle Entwuerfe in diesem Kit anzuzeigen"
              },
              "manual": "manuals/semio/kit#designs",
              "tutorial": "hello-semio/model-design"
            },
            "createDesign": {
              "label": {
                "normal": "Entwurf erstellen",
                "beginner": "Klicken, um einen neuen Entwurf in diesem Kit zu erstellen"
              },
              "manual": "manuals/semio/kit#designs",
              "tutorial": "hello-semio/model-design"
            },
            "showTypes": {
              "label": {
                "normal": "Typen anzeigen",
                "beginner": "Klicken, um alle Typen in diesem Kit anzuzeigen"
              },
              "manual": "manuals/semio/kit#types",
              "tutorial": "hello-semio/model-brick-set"
            },
            "createType": {
              "label": {
                "normal": "Typ erstellen",
                "beginner": "Klicken, um einen neuen Typ in diesem Kit zu erstellen"
              },
              "manual": "manuals/semio/kit#types",
              "tutorial": "hello-semio/model-brick-set"
            },
            "showQualities": {
              "label": {
                "normal": "Qualitaeten anzeigen",
                "beginner": "Klicken, um alle Qualitaeten in diesem Kit anzuzeigen"
              },
              "manual": "manuals/semio/kit#qualities",
              "tutorial": "getting-started/intro#quality"
            },
            "createQuality": {
              "label": {
                "normal": "Qualitaet erstellen",
                "beginner": "Klicken, um eine neue Qualitaetsdefinition in diesem Kit zu erstellen"
              },
              "manual": "manuals/semio/kit#qualities",
              "tutorial": "getting-started/intro#quality"
            },
            "showPorts": {
              "label": {
                "normal": "Schnittstellen anzeigen",
                "beginner": "Klicken, um alle Schnittstellen in diesem Kit anzuzeigen"
              },
              "manual": "manuals/semio/kit#ports",
              "tutorial": "getting-started/intro#port"
            },
            "createPort": {
              "label": {
                "normal": "Schnittstelle erstellen",
                "beginner": "Klicken, um eine neue Schnittstellendefinition in diesem Kit zu erstellen"
              },
              "manual": "manuals/semio/kit#ports",
              "tutorial": "getting-started/intro#port"
            },
            "showFiles": {
              "label": {
                "normal": "Dateien anzeigen",
                "beginner": "Klicken, um alle Dateien in diesem Kit anzuzeigen"
              },
              "manual": "manuals/semio/kit#files",
              "tutorial": "getting-started/intro#files"
            },
            "createFile": {
              "label": {
                "normal": "Datei erstellen",
                "beginner": "Klicken, um eine neue Datei zu diesem Kit hinzuzufuegen"
              },
              "manual": "manuals/semio/kit#files",
              "tutorial": "getting-started/intro#files"
            },
            "showFolders": {
              "label": {
                "normal": "Ordner anzeigen",
                "beginner": "Klicken, um alle Ordner in diesem Kit anzuzeigen"
              },
              "manual": "manuals/semio/kit#folders",
              "tutorial": "hello-semio/model-design"
            },
            "createFolder": {
              "label": {
                "normal": "Ordner erstellen",
                "beginner": "Klicken, um einen neuen Ordner in diesem Kit zu erstellen"
              },
              "manual": "manuals/semio/kit#folders",
              "tutorial": "hello-semio/model-design"
            },
            "reset": {
              "label": {
                "normal": "Zuruecksetzen",
                "beginner": "Klicken, um das Kit auf den urspruenglichen Zustand zurueckzusetzen"
              }
            },
            "showAuthors": {
              "label": {
                "normal": "Autoren anzeigen",
                "beginner": "Klicken, um alle Autoren dieses Kits anzuzeigen"
              },
              "manual": "manuals/semio/kit#authors",
              "tutorial": "getting-started/intro#authors"
            },
            "createAuthor": {
              "label": {
                "normal": "Autor erstellen",
                "beginner": "Klicken, um einen neuen Autor zu diesem Kit hinzuzufuegen"
              },
              "manual": "manuals/semio/kit#authors",
              "tutorial": "getting-started/intro#authors"
            },
            "hideKind": {
              "label": {
                "normal": "Ausblenden",
                "beginner": "Klicken, um diese Artefaktkategorie auszublenden"
              }
            },
            "createArtifact": {
              "label": {
                "normal": "Erstellen",
                "beginner": "Klicken, um ein neues Artefakt dieses Typs zu erstellen"
              }
            },
            "createChild": {
              "label": {
                "normal": "Kind erstellen",
                "beginner": "Ein Kindelement erstellen"
              }
            },
            "showTags": {
              "label": {
                "normal": "Tags anzeigen",
                "beginner": "Tags anzeigen"
              }
            },
            "showConcepts": {
              "label": {
                "normal": "Konzepte anzeigen",
                "beginner": "Konzepte anzeigen"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Artefakte nach Typ filtern"
              }
            },
            "resetFilters": {
              "label": {
                "normal": "Filter Zurücksetzen",
                "beginner": "Aktive Artefaktfilter entfernen und alle Artefaktarten anzeigen"
              }
            },
            "selection": {
              "label": {
                "normal": "Auswahl",
                "beginner": "Auswahlwerkzeuge"
              }
            },
            "create": {
              "label": {
                "normal": "Erstellen",
                "beginner": "Neue Artefakte erstellen"
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "search": {
                "label": {
                  "normal": "Tabelle durchsuchen",
                  "beginner": "Nach Artefakten in der Tabelle suchen"
                },
                "hotkey": "Ctrl+F"
              },
              "header": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kind": {
                  "label": {
                    "normal": "Art",
                    "beginner": "Art des Artefakts"
                  }
                },
                "artifact": {
                  "label": {
                    "normal": "Name",
                    "beginner": "Der Name"
                  }
                },
                "updatedAt": {
                  "label": {
                    "normal": "Aktualisiert",
                    "beginner": "Letzte Aktualisierungszeit"
                  }
                },
                "createdAt": {
                  "label": {
                    "normal": "Erstellt",
                    "beginner": "Erstellungszeit"
                  }
                }
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Ein kraftgerichteter Graph, der alle Kit-Artefakte und ihre Beziehungen zeigt"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kit": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Kit-Name",
                      "beginner": "Der Name des Kits. Dies ist die primaere Kennung fuer Ihr Kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "version": {
                    "label": {
                      "normal": "Version",
                      "beginner": "Die Version des Kits im semantischen Versionierungsformat (z.B. 1.0.0)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine detaillierte Beschreibung dessen, was dieses Kit enthaelt und wie es verwendet werden sollte."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "icon": {
                    "label": {
                      "normal": "Symbol",
                      "beginner": "Ein Icon zur Darstellung dieses Kits. Kann ein Emoji oder eine URL zu einem Bild sein."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "image": {
                    "label": {
                      "normal": "Bild",
                      "beginner": "URL zu einem Vorschaubild, das dieses Kit praesentiert."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "homepage": {
                    "label": {
                      "normal": "Homepage",
                      "beginner": "URL zur Homepage oder Dokumentation fuer dieses Kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "license": {
                    "label": {
                      "normal": "Lizenz",
                      "beginner": "Die Lizenz, unter der dieses Kit verteilt wird (z.B. MIT, GPL)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  }
                },
                "folder": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Anzeigename des Ordners."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Optionale Beschreibung, die den Zweck dieses Ordners erlaeutert."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-semio/save-kit"
                  }
                },
                "port": {
                  "compatible": {
                    "label": {
                      "normal": "Kompatibel",
                      "beginner": "Kompatibel"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Beschreibung"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  }
                },
                "tag": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Beschreibung"
                    }
                  }
                },
                "concept": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Beschreibung"
                    }
                  }
                }
              }
            }
          },
          "notFound": {
            "label": {
              "normal": "Kit nicht gefunden",
              "beginner": "Das angeforderte Kit wurde nicht gefunden"
            },
            "description": {
              "normal": "Das Kit wurde moeglicherweise entfernt oder der Link ist ungueltig.",
              "beginner": "Zurueck zur Startseite und ein anderes Kit oeffnen oder ein neues erstellen."
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Kit-Einstellungen"
            },
            "diagram": {
              "chargeStrength": {
                "label": {
                  "normal": "Ladungsstärke",
                  "beginner": "Abstoßungskraft zwischen Knoten"
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Verbindungsabstand",
                  "beginner": "Zielabstand zwischen verbundenen Knoten"
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Kollisionsradius",
                  "beginner": "Kollisionsradius zur Vermeidung von Knotenüberlappung"
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Zentrierungsstärke",
                  "beginner": "Kraft, die Knoten zur Mitte zieht"
                }
              }
            },
            "theme": {
              "label": {
                "normal": "Thema",
                "beginner": "Farbthema"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Oberflächensprache"
              }
            },
            "device": {
              "label": {
                "normal": "Gerät",
                "beginner": "Eingabegerätetyp"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Benutzerexpertise-Stufe"
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Benutzer- oder Entwicklermodus"
              }
            }
          },
          "port": {
            "allCompatible": {
              "label": {
                "normal": "Alle kompatibel",
                "beginner": "Alle kompatibel"
              }
            },
            "compatiblePorts": {
              "label": {
                "normal": "Kompatible Schnittstellen",
                "beginner": "Kompatible Schnittstellen"
              }
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Label",
                  "beginner": "Label"
                }
              }
            }
          },
          "ports": {
            "multipleSelected": {
              "label": {
                "normal": "Mehrere ausgewählt",
                "beginner": "Mehrere ausgewählt"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "{{count}} Schnittstellen",
                "beginner": "{{count}} Schnittstellen ausgewaehlt"
              }
            }
          },
          "qualities": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "files": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "authors": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "tag": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie diesen Tag...",
                "beginner": "Beschreiben Sie diesen Tag..."
              }
            }
          },
          "tags": {
            "multipleSelected": {
              "label": {
                "normal": "Mehrere Tags ausgewaehlt",
                "beginner": "Mehrere Tags ausgewaehlt"
              }
            },
            "multipleTitle": "{{count}} Tags"
          },
          "concept": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie dieses Konzept...",
                "beginner": "Beschreiben Sie dieses Konzept..."
              }
            }
          },
          "concepts": {
            "multipleSelected": {
              "label": {
                "normal": "Mehrere Konzepte ausgewaehlt",
                "beginner": "Mehrere Konzepte ausgewaehlt"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Titel"
            }
          },
          "tools": {
            "label": {
              "normal": "Werkzeuge",
              "beginner": "Werkzeuge"
            },
            "select": {
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additiv",
                    "beginner": "Additiver Auswahlmodus - zur vorhandenen Auswahl hinzufügen"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtraktiv",
                    "beginner": "Subtraktiver Auswahlmodus - aus vorhandener Auswahl entfernen"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Schnittmenge",
                    "beginner": "Schnittmengen-Auswahlmodus - nur überlappende Elemente auswählen"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rechteckig",
                    "beginner": "Rechteckige Auswahl - ziehen Sie, um in einem Rechteck auszuwählen"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freiform-Lasso-Auswahl - zeichnen Sie eine Freiform-Form"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand-Werkzeug - schwenken und navigieren Sie auf der Leinwand"
                  }
                }
              }
            }
          },
          "folder": {
            "label": {
              "normal": "Ordner",
              "beginner": "Ordner"
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Describe this folder...",
                  "beginner": "Describe this folder..."
                }
              }
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Kit-Chat"
            }
          }
        },
        "port": {
          "label": {
            "normal": "Schnittstelle",
            "beginner": "Schnittstelle"
          },
          "defaultName": {
            "label": {
              "normal": "Neue Schnittstelle",
              "beginner": "Neue Schnittstelle"
            }
          }
        },
        "tag": {
          "label": {
            "normal": "Tag",
            "beginner": "Tag"
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Tag",
              "beginner": "Neuer Tag"
            }
          }
        },
        "concept": {
          "label": {
            "normal": "Konzept",
            "beginner": "Konzept"
          },
          "defaultName": {
            "label": {
              "normal": "Neues Konzept",
              "beginner": "Neues Konzept"
            }
          }
        },
        "folder": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Ordner",
              "beginner": "Neuer Ordner"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diesen Ordner...",
                "beginner": "Beschreiben Sie diesen Ordner..."
              }
            }
          }
        },
        "design": {
          "label": {
            "normal": "Entwurf",
            "beginner": "Entwurf"
          },
          "properties": {
            "label": {
              "normal": "Entwurfs-Eigenschaften",
              "beginner": "Entwurfs-Eigenschaften"
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Entwurf",
              "beginner": "Neuer Entwurf"
            }
          },
          "windowLibrary": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "scene": {
              "label": {
                "normal": "Szenen-Fenster",
                "beginner": "Durchsuchen und Hinzufuegen von 3D-Szenen-Fenstern zum Betrachten Ihres Entwurfs im 3D-Raum."
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm-Fenster",
                "beginner": "Durchsuchen und Hinzufuegen von 2D-Diagramm-Fenstern zum Betrachten der Verbindungstopologie."
              }
            },
            "table": {
              "label": {
                "normal": "Tabellen-Fenster",
                "beginner": "Durchsuchen und Hinzufuegen von Tabellen-Fenstern zum Betrachten von Entwurfsdaten in tabellarischer Form."
              }
            }
          },
          "diagram": {
            "clusterMenu": {
              "cluster": {
                "label": {
                  "normal": "Gruppieren",
                  "beginner": "Ausgewählte Entwurfsteile zu einem einzelnen Cluster gruppieren"
                }
              }
            },
            "expandMenu": {
              "expand": {
                "label": {
                  "normal": "Erweitern",
                  "beginner": "Das Entwurfsteil erweitern, um seine internen Komponenten anzuzeigen"
                }
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "normal": "Beschreiben Sie diesen Entwurf...",
              "beginner": "Beschreiben Sie diesen Entwurf..."
            }
          },
          "iconPlaceholder": {
            "label": {
              "normal": "???",
              "beginner": "???"
            }
          },
          "imagePlaceholder": {
            "label": {
              "normal": "https://example.com/image.png",
              "beginner": "https://example.com/image.png"
            }
          },
          "variantPlaceholder": {
            "label": {
              "normal": "z.B. klein, mittel, gross",
              "beginner": "z.B. klein, mittel, gross"
            }
          },
          "viewPlaceholder": {
            "label": {
              "normal": "z.B. vorne, seite, oben",
              "beginner": "z.B. vorne, seite, oben"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "variant": {
            "label": {
              "normal": "Variante",
              "beginner": "Variante"
            }
          },
          "view": {
            "label": {
              "normal": "Ansicht",
              "beginner": "Ansicht"
            }
          },
          "location": {
            "label": {
              "normal": "Standort",
              "beginner": "Standort"
            }
          },
          "authors": {
            "label": {
              "normal": "Autoren",
              "beginner": "Autoren"
            }
          },
          "author": {
            "label": {
              "normal": "Autor",
              "beginner": "Autor"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribut",
              "beginner": "Attribut"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "normal": "Wert...",
              "beginner": "Wert..."
            }
          },
          "attributeUnitPlaceholder": {
            "label": {
              "normal": "Einheit...",
              "beginner": "Einheit..."
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "normal": "Definition oder URL...",
              "beginner": "Definition oder URL..."
            }
          },
          "piece": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "id": {
              "label": {
                "normal": "ID",
                "beginner": "ID"
              }
            },
            "type": {
              "label": {
                "normal": "Typ",
                "beginner": "Typ"
              }
            },
            "center": {
              "label": {
                "normal": "Zentrum",
                "beginner": "Zentrum"
              }
            },
            "plane": {
              "label": {
                "normal": "Ebene",
                "beginner": "Ebene"
              }
            },
            "planeOrigin": {
              "label": {
                "normal": "Ursprung",
                "beginner": "Ursprung"
              }
            },
            "planeXAxis": {
              "label": {
                "normal": "X-Achse",
                "beginner": "X-Achse"
              }
            },
            "planeYAxis": {
              "label": {
                "normal": "Y-Achse",
                "beginner": "Y-Achse"
              }
            },
            "mixedSelectionMessage": {
              "label": {
                "normal": "Mehrere Teile mit unterschiedlichen Werten ausgewaehlt",
                "beginner": "Mehrere Teile mit unterschiedlichen Werten ausgewaehlt"
              }
            },
            "connectedPieceInfo": {
              "label": {
                "normal": "Dieses Teil ist mit einem anderen Teil verbunden. Seine Position und Ausrichtung werden aus der Verbindung berechnet. Um es unabhängig zu machen, klicken Sie auf 'Teil fixieren'.",
                "beginner": "Dieses Teil ist mit einem anderen Teil verbunden. Seine Position und Ausrichtung werden aus der Verbindung berechnet. Um es unabhängig zu machen, klicken Sie auf 'Teil fixieren'."
              }
            },
            "fixPiece": {
              "label": {
                "normal": "Teil fixieren",
                "beginner": "Teil fixieren"
              }
            }
          },
          "connection": {
            "label": {},
            "rotation": {
              "label": {
                "normal": "Rotation",
                "beginner": "Rotation"
              }
            },
            "turn": {
              "label": {
                "normal": "Drehung",
                "beginner": "Drehung"
              }
            },
            "tilt": {
              "label": {
                "normal": "Neigung",
                "beginner": "Neigung"
              }
            },
            "plane": {
              "label": {
                "normal": "Ebene",
                "beginner": "Ebene"
              }
            },
            "translation": {
              "label": {
                "normal": "Verschiebung",
                "beginner": "Verschiebung"
              }
            },
            "orientation": {
              "label": {
                "normal": "Ausrichtung",
                "beginner": "Ausrichtung"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Diagramm"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "design": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name des Entwurfs. Dies ist die primaere Kennung fuer Ihre Komposition."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine detaillierte Beschreibung dessen, was dieser Entwurf darstellt und wie er verwendet werden sollte."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "icon": {
                    "label": {
                      "normal": "Symbol",
                      "beginner": "URL oder Pfad zu einem Icon, das diesen Entwurf in Listen und Vorschauen darstellt."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "image": {
                    "label": {
                      "normal": "Bild",
                      "beginner": "URL oder Pfad zu einem Vorschaubild, das diesen Entwurf praesentiert."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "variant": {
                    "label": {
                      "normal": "Variante",
                      "beginner": "Eine Variantenkennung fuer verschiedene Versionen oder Konfigurationen dieses Entwurfs."
                    },
                    "manual": "design#variants",
                    "tutorial": "hello-semio/model-design"
                  },
                  "view": {
                    "label": {
                      "normal": "Ansicht",
                      "beginner": "Die Ansichtsperspektive oder Kamerawinkel zur Anzeige dieses Entwurfs."
                    },
                    "manual": "design#views",
                    "tutorial": "hello-semio/model-design"
                  },
                  "unit": {
                    "label": {
                      "normal": "Einheit",
                      "beginner": "Die Masseinheit fuer alle Abmessungen in diesem Entwurf (z.B. mm, cm, m)."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "createdAt": {
                    "label": {
                      "normal": "Erstellt am",
                      "beginner": "Das Datum und die Uhrzeit, wann dieser Entwurf erstmals erstellt wurde."
                    }
                  },
                  "updatedAt": {
                    "label": {
                      "normal": "Aktualisiert am",
                      "beginner": "Das Datum und die Uhrzeit, wann dieser Entwurf zuletzt geaendert wurde."
                    }
                  },
                  "pieceCount": {
                    "label": {
                      "normal": "Bauteile",
                      "beginner": "Die Gesamtanzahl der Bauteile in diesem Entwurf."
                    }
                  },
                  "connectionCount": {
                    "label": {
                      "normal": "Verbindungen",
                      "beginner": "Die Gesamtanzahl der Verbindungen in diesem Entwurf."
                    }
                  }
                },
                "location": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "longitude": {
                    "label": {
                      "normal": "Laengengrad",
                      "beginner": "Die Ost-West-Position dieses Entwurfsstandorts in Dezimalgrad."
                    },
                    "manual": "design#location"
                  },
                  "latitude": {
                    "label": {
                      "normal": "Breitengrad",
                      "beginner": "Die Nord-Sued-Position dieses Entwurfsstandorts in Dezimalgrad."
                    },
                    "manual": "design#location"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der vollstaendige Name der Person, die zu diesem Entwurf beigetragen hat."
                    },
                    "manual": "design#authors"
                  },
                  "email": {
                    "label": {
                      "normal": "E-Mail",
                      "beginner": "Kontakt-E-Mail-Adresse fuer diesen Autor."
                    },
                    "manual": "design#authors"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Die eindeutige Kennung fuer dieses Attribut im Kebab-Case-Format (z.B. 'material.type')."
                    },
                    "manual": "design#attributes"
                  },
                  "value": {
                    "label": {
                      "normal": "Wert",
                      "beginner": "Der Wert, der mit diesem Attribut verbunden ist. Leer lassen, um als Kategorie-Flag zu verwenden."
                    },
                    "manual": "design#attributes"
                  },
                  "unit": {
                    "label": {
                      "normal": "Einheit",
                      "beginner": "Die Masseinheit fuer den Wert dieses Attributs (z.B. mm, kg, °C)."
                    },
                    "manual": "design#attributes"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "Eine URL oder ein Text, der definiert, was dieses Attribut bedeutet und wie es interpretiert werden sollte."
                    },
                    "manual": "design#attributes"
                  }
                },
                "piece": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Bauteil",
                      "beginner": "Eigenschaften des ausgewaehlten Bauteils."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Bauteile",
                      "beginner": "Eigenschaften der ausgewaehlten Bauteile."
                    }
                  },
                  "pieceInfo": {
                    "label": {
                      "normal": "Bauteil",
                      "beginner": "Grundinformationen ueber das Bauteil."
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Ein optionaler Name für dieses Bauteil zur Identifikation innerhalb des Entwurfs."
                    }
                  },
                  "namePlaceholder": {
                    "label": {
                      "normal": "Bauteilname eingeben...",
                      "beginner": "Bauteilname eingeben..."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung des Zwecks oder der Rolle dieses Bauteils im Entwurf."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Bauteilbeschreibung eingeben...",
                      "beginner": "Bauteilbeschreibung eingeben..."
                    }
                  },
                  "scale": {
                    "label": {
                      "normal": "Skalierung",
                      "beginner": "Der Skalierungsfaktor für dieses Bauteil. Standard ist 1.0."
                    }
                  },
                  "color": {
                    "label": {
                      "normal": "Farbe",
                      "beginner": "Eine optionale Farbüberschreibung für dieses Bauteil (z.B. #FF0000)."
                    }
                  },
                  "colorPlaceholder": {
                    "label": {
                      "normal": "Farbe eingeben...",
                      "beginner": "Farbe eingeben..."
                    }
                  },
                  "attributes": {
                    "label": {
                      "normal": "Attribute",
                      "beginner": "Benutzerdefinierte Schlüssel-Wert-Attribute für dieses Bauteil."
                    },
                    "name": {
                      "label": {
                        "normal": "Name",
                        "beginner": "Der Schlüsselbezeichner für dieses Attribut."
                      }
                    },
                    "value": {
                      "label": {
                        "normal": "Wert",
                        "beginner": "Der Wert für dieses Attribut."
                      }
                    },
                    "unit": {
                      "label": {
                        "normal": "Einheit",
                        "beginner": "Die Maßeinheit für dieses Attribut."
                      }
                    },
                    "definition": {
                      "label": {
                        "normal": "Definition",
                        "beginner": "Eine URL oder ein Text, der dieses Attribut definiert."
                      }
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribut",
                      "beginner": "Ein benutzerdefiniertes Attribut dieses Bauteils."
                    }
                  },
                  "center": {
                    "label": {
                      "normal": "Zentrum",
                      "beginner": "Die Zentrumsposition des Bauteils im 2D-Diagramm-Layout."
                    },
                    "manual": "design#diagram",
                    "tutorial": "metabolism/thinking-about-the-diagram",
                    "x": {
                      "label": {
                        "normal": "U",
                        "beginner": "U-Diagrammkoordinate des Zentrums des Bauteils im 2D-Layoutraum."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    },
                    "y": {
                      "label": {
                        "normal": "V",
                        "beginner": "V-Diagrammkoordinate des Zentrums des Bauteils im 2D-Layoutraum."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    }
                  },
                  "plane": {
                    "label": {
                      "normal": "Ebene",
                      "beginner": "Die 3D-Platzierungsebene fuer dieses Bauteil. Definiert Position und Ausrichtung im 3D-Raum."
                    },
                    "manual": "design#pieces",
                    "tutorial": "hello-semio/model-design#pieces",
                    "origin": {
                      "label": {
                        "normal": "Ursprung",
                        "beginner": "Ursprung"
                      },
                      "x": {
                        "label": {
                          "normal": "Ursprung X",
                          "beginner": "X-Koordinate des Ursprungs"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Ursprung Y",
                          "beginner": "Y-Koordinate des Ursprungs"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Ursprung Z",
                          "beginner": "Z-Koordinate des Ursprungs"
                        }
                      }
                    },
                    "xaxis": {
                      "label": {
                        "normal": "X-Achse",
                        "beginner": "X-Achse"
                      },
                      "x": {
                        "label": {
                          "normal": "X-Achse X",
                          "beginner": "X-Komponente der X-Achse"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "X-Achse Y",
                          "beginner": "Y-Komponente der X-Achse"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "X-Achse Z",
                          "beginner": "Z-Komponente der X-Achse"
                        }
                      }
                    },
                    "yaxis": {
                      "label": {
                        "normal": "Y-Achse",
                        "beginner": "Y-Achse"
                      },
                      "x": {
                        "label": {
                          "normal": "Y-Achse X",
                          "beginner": "X-Komponente der Y-Achse"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Y-Achse Y",
                          "beginner": "Y-Komponente der Y-Achse"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Y-Achse Z",
                          "beginner": "Z-Komponente der Y-Achse"
                        }
                      }
                    }
                  }
                },
                "connection": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Verbindung",
                      "beginner": "Eigenschaften der ausgewaehlten Verbindung."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Verbindungen",
                      "beginner": "Eigenschaften der ausgewaehlten Verbindungen."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung des Zwecks dieser Verbindung."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Verbindungsbeschreibung eingeben...",
                      "beginner": "Verbindungsbeschreibung eingeben..."
                    }
                  },
                  "multipleEditing": {
                    "label": {
                      "normal": "{{count}} Verbindungen werden gleichzeitig bearbeitet",
                      "beginner": "{{count}} Verbindungen werden gleichzeitig bearbeitet"
                    }
                  },
                  "connecting": {
                    "label": {
                      "normal": "Verbindend",
                      "beginner": "Verbindend"
                    }
                  },
                  "connectingPieceId": {
                    "label": {
                      "normal": "Verbindendes Stück",
                      "beginner": "Verbindendes Stück"
                    }
                  },
                  "connectingPortId": {
                    "label": {
                      "normal": "Verbindender Anschluss",
                      "beginner": "Verbindender Anschluss"
                    }
                  },
                  "connectingDesignPieceId": {
                    "label": {
                      "normal": "Verbindendes Designstück",
                      "beginner": "Verbindendes Designstück"
                    }
                  },
                  "connected": {
                    "label": {
                      "normal": "Verbunden",
                      "beginner": "Verbunden"
                    }
                  },
                  "connectedPieceId": {
                    "label": {
                      "normal": "Verbundenes Stück",
                      "beginner": "Verbundenes Stück"
                    }
                  },
                  "connectedPortId": {
                    "label": {
                      "normal": "Verbundener Anschluss",
                      "beginner": "Verbundener Anschluss"
                    }
                  },
                  "connectedDesignPieceId": {
                    "label": {
                      "normal": "Verbundenes Designstück",
                      "beginner": "Verbundenes Designstück"
                    }
                  },
                  "gap": {
                    "label": {
                      "normal": "Abstand",
                      "beginner": "Abstand"
                    }
                  },
                  "shift": {
                    "label": {
                      "normal": "Verschiebung",
                      "beginner": "Verschiebung"
                    }
                  },
                  "rise": {
                    "label": {
                      "normal": "Anstieg",
                      "beginner": "Anstieg"
                    }
                  },
                  "rotation": {
                    "label": {
                      "normal": "Rotation",
                      "beginner": "Rotation"
                    }
                  },
                  "turn": {
                    "label": {
                      "normal": "Drehung",
                      "beginner": "Drehung"
                    }
                  },
                  "tilt": {
                    "label": {
                      "normal": "Neigung",
                      "beginner": "Neigung"
                    }
                  },
                  "x": {
                    "label": {
                      "normal": "Diagramm X-Versatz",
                      "beginner": "X-Versatz im Diagramm"
                    }
                  },
                  "y": {
                    "label": {
                      "normal": "Diagramm Y-Versatz",
                      "beginner": "Y-Versatz im Diagramm"
                    }
                  },
                  "u": {
                    "label": {
                      "normal": "X-Versatz",
                      "beginner": "X-Versatz"
                    }
                  },
                  "v": {
                    "label": {
                      "normal": "Y-Versatz",
                      "beginner": "Y-Versatz"
                    }
                  }
                },
                "connector": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "id": {
                    "label": {
                      "normal": "Connector-ID",
                      "beginner": "Die eindeutige Kennung des Connectors"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name dieses Connectors."
                    }
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "Der Parameter t entlang der Kurve des Typs für diesen Connector."
                    }
                  },
                  "position": {
                    "label": {
                      "normal": "Position",
                      "beginner": "Die Position des Connectors"
                    }
                  },
                  "direction": {
                    "label": {
                      "normal": "Richtung",
                      "beginner": "Der Richtungsvektor des Connectors"
                    }
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Erforderlich",
                      "beginner": "Ob dieser Connector verbunden sein muss"
                    }
                  },
                  "port": {
                    "label": {
                      "normal": "Familie",
                      "beginner": "Die Connectorfamilie für Kompatibilitätsprüfung"
                    }
                  },
                  "compatiblePort": {
                    "label": {
                      "normal": "Kompatible Familie",
                      "beginner": "Die Familien, mit denen dieser Connector kompatibel ist"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung des Connectors"
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribut",
                      "beginner": "Benutzerdefinierte Attribute des Connectors"
                    }
                  },
                  "notFound": {
                    "label": {
                      "normal": "Nicht gefunden",
                      "beginner": "Nicht gefunden"
                    }
                  }
                }
              },
              "parentConnection": {
                "label": {
                  "normal": "Elternverbindung",
                  "beginner": "Elternverbindung"
                }
              },
              "parentConnections": {
                "label": {
                  "normal": "Elternverbindungen",
                  "beginner": "Elternverbindungen"
                }
              }
            },
            "hud": {
              "overview": {
                "label": {
                  "normal": "HUD-Uebersicht",
                  "beginner": "HUD-Uebersicht"
                }
              },
              "selection": {
                "pieces": {
                  "label": {
                    "normal": "Ausgewaehlte Bauteile",
                    "beginner": "Ausgewaehlte Bauteile"
                  }
                },
                "connections": {
                  "label": {
                    "normal": "Ausgewaehlte Verbindungen",
                    "beginner": "Ausgewaehlte Verbindungen"
                  }
                },
                "connector": {
                  "label": {
                    "normal": "Connector ausgewaehlt",
                    "beginner": "Connector ausgewaehlt"
                  }
                }
              }
            },
            "stats": {
              "overview": {
                "label": {
                  "normal": "Statistik",
                  "beginner": "Statistik"
                }
              },
              "pieces": {
                "label": {
                  "normal": "Bauteile gesamt",
                  "beginner": "Bauteile gesamt"
                }
              },
              "connections": {
                "label": {
                  "normal": "Verbindungen gesamt",
                  "beginner": "Verbindungen gesamt"
                }
              },
              "windows": {
                "label": {
                  "normal": "Fensterlayout geladen",
                  "beginner": "Fensterlayout geladen"
                }
              }
            },
            "workbench": {
              "types": {
                "addPiece": {
                  "label": {
                    "normal": "Bauteil hinzufügen",
                    "beginner": "Ein neues Bauteil dieses Typs zum Entwurf hinzufügen"
                  }
                },
                "duplicateType": {
                  "label": {
                    "normal": "Typ duplizieren",
                    "beginner": "Eine Kopie dieses Typs erstellen"
                  }
                }
              },
              "designs": {
                "addPiece": {
                  "label": {
                    "normal": "Bauteil hinzufügen",
                    "beginner": "Ein neues Bauteil dieses Entwurfs zum aktuellen Entwurf hinzufügen"
                  }
                }
              }
            }
          },
          "gridSize": {
            "label": {
              "normal": "Rastergröße",
              "beginner": "Rastergröße"
            }
          },
          "proximityConnectDistance": {
            "label": {
              "normal": "Näherungsverbindungs-Abstand",
              "beginner": "Näherungsverbindungs-Abstand"
            }
          },
          "selectOnlyPiecesOrConnections": {
            "label": {
              "normal": "Nur Bauteile oder Verbindungen auswählen",
              "beginner": "Nur Bauteile oder Verbindungen auswählen"
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Titel"
            }
          },
          "tools": {
            "label": {
              "normal": "Werkzeuge",
              "beginner": "Werkzeuge"
            },
            "select": {
              "label": {
                "normal": "Auswählen",
                "beginner": "Auswahlwerkzeug"
              },
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additiv",
                    "beginner": "Additiver Auswahlmodus - zur vorhandenen Auswahl hinzufügen"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtraktiv",
                    "beginner": "Subtraktiver Auswahlmodus - von vorhandener Auswahl entfernen"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Schnittmenge",
                    "beginner": "Schnittmengen-Auswahlmodus - nur überlappende Elemente auswählen"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rechteckig",
                    "beginner": "Rechteckige Auswahl - ziehen Sie, um in einem Rechteck auszuwählen"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freiform-Lasso-Auswahl - zeichnen Sie eine Freiformform"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand-Werkzeug - die Leinwand schwenken und navigieren"
                  }
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rechteck-Lasso",
                  "beginner": "Rechteck fuer Lasso-Auswahl ziehen"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freiform-Lasso",
                  "beginner": "Freihand-Lasso-Pfad zeichnen"
                }
              }
            }
          },
          "windows": {
            "label": {
              "normal": "Fenster",
              "beginner": "Fenster"
            }
          },
          "appTitle": {
            "label": {
              "normal": "App Title",
              "beginner": "App Title"
            }
          },
          "canvas": {
            "diagram": {
              "label": {
                "normal": "Diagramm",
                "beginner": "Diagramm"
              },
              "pieceNode": {
                "label": {
                  "normal": "Bauteil",
                  "beginner": "Diagramm-Bauteilknoten"
                }
              }
            },
            "label": {
              "normal": "Leinwand",
              "beginner": "Leinwand"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showPieces": {
              "label": {
                "normal": "Bauteile",
                "beginner": "Sichtbarkeit von Bauteilen umschalten"
              }
            },
            "showConnections": {
              "label": {
                "normal": "Verbindungen",
                "beginner": "Sichtbarkeit von Verbindungen zwischen Bauteilen umschalten"
              }
            },
            "showPorts": {
              "label": {
                "normal": "Anschlüsse",
                "beginner": "Sichtbarkeit von Anschlüssen an Bauteilen umschalten"
              }
            },
            "addPiece": {
              "label": {
                "normal": "Bauteil hinzufügen",
                "beginner": "Ein neues Bauteil zum Entwurf hinzufügen"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Design-Elemente nach Sichtbarkeit filtern"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Entwurfs-Einstellungen"
            },
            "theme": {
              "label": {
                "normal": "Design",
                "beginner": "Wählen Sie das Farbschema für die Anwendung"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Wählen Sie die Sprache für die Anwendungsoberfläche"
              },
              "placeholder": {
                "label": {
                  "normal": "Sprache wählen...",
                  "beginner": "Wählen Sie die Sprache, in der die Anwendung angezeigt wird"
                }
              }
            },
            "device": {
              "label": {
                "normal": "Gerät",
                "beginner": "Wählen Sie das Eingabegerät"
              }
            },
            "expertise": {
              "label": {
                "normal": "Erfahrung",
                "beginner": "Wählen Sie Ihr Erfahrungsniveau"
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Wählen Sie den Benutzeroberflächenmodus"
              }
            },
            "panel": {
              "label": {
                "normal": "Panels",
                "beginner": "Panel-Sichtbarkeit konfigurieren"
              },
              "toolbar": {
                "label": {
                  "normal": "Toolbar anzeigen",
                  "beginner": "Toolbar-Panel umschalten"
                }
              },
              "workbench": {
                "label": {
                  "normal": "Werkbank anzeigen",
                  "beginner": "Werkbank-Panel umschalten"
                }
              },
              "windows": {
                "label": {
                  "normal": "Fenster anzeigen",
                  "beginner": "Fenster-Panel umschalten"
                }
              },
              "details": {
                "label": {
                  "normal": "Details anzeigen",
                  "beginner": "Details-Panel umschalten"
                }
              }
            }
          }
        },
        "docs": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "noHeadings": {
            "label": {
              "normal": "Keine Ueberschriften gefunden",
              "beginner": "Keine Ueberschriften gefunden"
            }
          },
          "docs": {
            "label": {
              "normal": "Dokumentation",
              "beginner": "Dokumentation"
            }
          },
          "overview": {
            "label": {
              "normal": "Übersicht",
              "beginner": "Übersicht"
            }
          },
          "page": {
            "label": {
              "normal": "Seite",
              "beginner": "Seite"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Einstellungen"
            },
            "theme": {
              "label": {
                "normal": "Design",
                "beginner": "Farbschema waehlen"
              }
            },
            "language": {
              "label": {
                "normal": "Sprache",
                "beginner": "Sprache auswaehlen"
              }
            },
            "device": {
              "label": {
                "normal": "Geraet",
                "beginner": "Eingabegeraet waehlen"
              }
            },
            "expertise": {
              "label": {
                "normal": "Erfahrung",
                "beginner": "Erfahrungsniveau waehlen"
              }
            },
            "mode": {
              "label": {
                "normal": "Modus",
                "beginner": "Benutzer- oder Entwicklermodus waehlen"
              }
            }
          },
          "navigation": {
            "previous": {
              "label": {
                "normal": "Zurück",
                "beginner": "Zur vorherigen Seite navigieren"
              }
            },
            "next": {
              "label": {
                "normal": "Weiter",
                "beginner": "Zur nächsten Seite navigieren"
              }
            }
          }
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Feedback"
          },
          "form": {
            "label": {
              "normal": "Feedback-Formular",
              "beginner": "Teilen Sie Ihre Gedanken mit uns"
            }
          },
          "kind": {
            "label": {
              "normal": "Art",
              "beginner": "Welche Art von Feedback möchten Sie geben?"
            }
          },
          "bugReport": {
            "label": {
              "normal": "Fehlerbericht",
              "beginner": "Einen Fehler oder ein Problem melden"
            }
          },
          "featureIdea": {
            "label": {
              "normal": "Feature-Idee",
              "beginner": "Ein neues Feature oder eine Verbesserung vorschlagen"
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Eine kurze Zusammenfassung Ihres Feedbacks"
            },
            "placeholder": {
              "label": {
                "normal": "Kurze Zusammenfassung eingeben...",
                "beginner": "Kurze Zusammenfassung eingeben..."
              }
            }
          },
          "description": {
            "label": {
              "normal": "Beschreibung",
              "beginner": "Detaillierte Beschreibung Ihres Feedbacks"
            },
            "bugPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie, was passiert ist und wie es reproduziert werden kann...",
                "beginner": "Beschreiben Sie, was passiert ist und wie es reproduziert werden kann..."
              }
            },
            "ideaPlaceholder": {
              "label": {
                "normal": "Beschreiben Sie Ihre Feature-Idee oder Verbesserung...",
                "beginner": "Beschreiben Sie Ihre Feature-Idee oder Verbesserung..."
              }
            }
          },
          "app": {
            "label": {
              "normal": "App",
              "beginner": "In welcher App ist der Fehler aufgetreten?"
            },
            "placeholder": {
              "label": {
                "normal": "App auswählen...",
                "beginner": "App auswählen..."
              }
            },
            "options": {
              "home": {
                "label": {
                  "normal": "Home",
                  "beginner": "Home"
                }
              },
              "kit": {
                "label": {
                  "normal": "Kit",
                  "beginner": "Kit"
                }
              },
              "design": {
                "label": {
                  "normal": "Design",
                  "beginner": "Design"
                }
              },
              "type": {
                "label": {
                  "normal": "Typ",
                  "beginner": "Typ"
                }
              },
              "quality": {
                "label": {
                  "normal": "Qualität",
                  "beginner": "Qualität"
                }
              },
              "docs": {
                "label": {
                  "normal": "Dokumentation",
                  "beginner": "Dokumentation"
                }
              },
              "feedback": {
                "label": {
                  "normal": "Feedback",
                  "beginner": "Feedback"
                }
              }
            }
          },
          "name": {
            "label": {
              "normal": "Ihr Name",
              "beginner": "Optional: Ihr Name für Rückmeldungen"
            },
            "placeholder": {
              "label": {
                "normal": "Name (optional)",
                "beginner": "Name (optional)"
              }
            }
          },
          "email": {
            "label": {
              "normal": "E-Mail",
              "beginner": "Optional: Ihre E-Mail für Rückmeldungen"
            },
            "placeholder": {
              "label": {
                "normal": "email@beispiel.de (optional)",
                "beginner": "email@beispiel.de (optional)"
              }
            }
          },
          "submit": {
            "label": {
              "normal": "Feedback senden",
              "beginner": "Klicken, um Ihr Feedback zu senden"
            }
          },
          "submitting": {
            "label": {
              "normal": "Wird gesendet...",
              "beginner": "Ihr Feedback wird gesendet"
            }
          },
          "success": {
            "title": {
              "label": {
                "normal": "Vielen Dank!",
                "beginner": "Vielen Dank!"
              }
            },
            "message": {
              "label": {
                "normal": "Ihr Feedback wurde erfolgreich gesendet. Wir schätzen Ihren Beitrag zur Verbesserung von Semio.",
                "beginner": "Ihr Feedback wurde erfolgreich gesendet. Wir schätzen Ihren Beitrag zur Verbesserung von Semio."
              }
            },
            "sendAnother": {
              "label": {
                "normal": "Weiteres Feedback senden",
                "beginner": "Klicken, um ein weiteres Feedback zu senden"
              }
            }
          },
          "error": {
            "title": {
              "label": {
                "normal": "Fehler",
                "beginner": "Fehler"
              }
            },
            "message": {
              "label": {
                "normal": "Beim Senden Ihres Feedbacks ist ein Fehler aufgetreten. Bitte versuchen Sie es erneut.",
                "beginner": "Beim Senden Ihres Feedbacks ist ein Fehler aufgetreten. Bitte versuchen Sie es erneut."
              }
            },
            "retry": {
              "label": {
                "normal": "Erneut versuchen",
                "beginner": "Klicken, um erneut zu versuchen"
              }
            }
          },
          "validation": {
            "titleRequired": {
              "label": {
                "normal": "Titel ist erforderlich",
                "beginner": "Titel ist erforderlich"
              }
            },
            "descriptionRequired": {
              "label": {
                "normal": "Beschreibung ist erforderlich",
                "beginner": "Beschreibung ist erforderlich"
              }
            },
            "appRequired": {
              "label": {
                "normal": "Bitte wählen Sie die App aus, in der der Fehler aufgetreten ist",
                "beginner": "Bitte wählen Sie die App aus, in der der Fehler aufgetreten ist"
              }
            },
            "invalidEmail": {
              "label": {
                "normal": "Bitte geben Sie eine gültige E-Mail-Adresse ein",
                "beginner": "Bitte geben Sie eine gültige E-Mail-Adresse ein"
              }
            }
          },
          "toolbar": {
            "send": {
              "label": {
                "normal": "Senden",
                "beginner": "Feedback senden"
              }
            }
          }
        },
        "type": {
          "label": {
            "normal": "Typ",
            "beginner": "Typ"
          },
          "properties": {
            "label": {
              "normal": "Typ-Eigenschaften",
              "beginner": "Typ-Eigenschaften"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showConnectors": {
              "label": {
                "normal": "Connectors",
                "beginner": "Connectors"
              }
            },
            "showModels": {
              "label": {
                "normal": "Darstellungen",
                "beginner": "Darstellungen"
              }
            },
            "filters": {
              "label": {
                "normal": "Filter",
                "beginner": "Typ-Elemente nach Sichtbarkeit filtern"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neuer Typ",
              "beginner": "Neuer Typ"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diesen Typ...",
                "beginner": "Beschreiben Sie diesen Typ..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "??",
                "beginner": "??"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://example.com/image.png",
                "beginner": "https://example.com/image.png"
              }
            }
          },
          "variantPlaceholder": {
            "label": {
              "label": {
                "normal": "z.B. gross, klein",
                "beginner": "z.B. gross, klein"
              }
            }
          },
          "parentPlaceholder": {
            "label": {
              "label": {
                "normal": "Elterntyp auswaehlen...",
                "beginner": "Elterntyp auswaehlen..."
              }
            }
          },
          "variant": {
            "label": {
              "normal": "Variante",
              "beginner": "Variante"
            }
          },
          "models": {
            "label": {
              "normal": "Darstellungen",
              "beginner": "Verschiedene 3D-Modelle, Bilder und visuelle Darstellungen fuer diesen Typ verwalten."
            },
            "manual": "type#models",
            "tutorial": "hello-semio/model-brick-set#models"
          },
          "model": {
            "label": {
              "normal": "Darstellung",
              "beginner": "Darstellung"
            }
          },
          "modelDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diese Darstellung...",
                "beginner": "Beschreiben Sie diese Darstellung..."
              }
            }
          },
          "modelTagsPlaceholder": {
            "label": {
              "label": {
                "normal": "tag1, tag2, tag3",
                "beginner": "tag1, tag2, tag3"
              }
            }
          },
          "connectors": {
            "label": {
              "normal": "Connectors",
              "beginner": "Verbindungsports fuer diesen Typ verwalten. Connectors definieren, wo und wie Bauteile verbunden werden koennen."
            },
            "manual": "type#connectors",
            "tutorial": "hello-semio/model-brick-set#connectors"
          },
          "connector": {
            "label": {
              "normal": "Connector",
              "beginner": "Connector"
            },
            "properties": {
              "label": {
                "normal": "Connector-Eigenschaften",
                "beginner": "Connector-Eigenschaften"
              }
            },
            "title": {
              "label": {
                "normal": "Titel",
                "beginner": "Titel"
              }
            }
          },
          "connectorPortPlaceholder": {
            "label": {
              "label": {
                "normal": "z.B. elektrisch, mechanisch",
                "beginner": "z.B. elektrisch, mechanisch"
              }
            }
          },
          "connectorNamePlaceholder": {
            "label": {
              "label": {
                "normal": "Name hinzufuegen",
                "beginner": "Name hinzufuegen"
              }
            }
          },
          "connectorDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Beschreiben Sie diesen Connector...",
                "beginner": "Beschreiben Sie diesen Connector..."
              }
            }
          },
          "connectorPoint": {
            "label": {
              "normal": "Punkt",
              "beginner": "Die 3D-Position des Connectors in lokalen Koordinaten."
            },
            "manual": "type#connectors",
            "tutorial": "hello-semio/model-brick-set#connectors"
          },
          "connectorDirection": {
            "label": {
              "normal": "Richtung",
              "beginner": "Der Auswaerts-Richtungsvektor des Connectors in lokalen Koordinaten."
            },
            "manual": "type#connectors",
            "tutorial": "hello-semio/model-brick-set#connectors"
          },
          "connectorCompatiblePortsPlaceholder": {
            "label": {
              "label": {
                "normal": "familie1, familie2",
                "beginner": "familie1, familie2"
              }
            }
          },
          "connectorNotFound": {
            "label": {
              "normal": "Connector nicht gefunden",
              "beginner": "Connector nicht gefunden"
            }
          },
          "connectorsNotFound": {
            "label": {
              "normal": "Keine Connectors gefunden",
              "beginner": "Keine Connectors gefunden"
            }
          },
          "authors": {
            "label": {
              "normal": "Autoren",
              "beginner": "Autoren"
            }
          },
          "author": {
            "label": {
              "normal": "Autor",
              "beginner": "Autor"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribut",
              "beginner": "Attribut"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "label": {
                "normal": "Wert...",
                "beginner": "Wert..."
              }
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "label": {
                "normal": "Definition oder URL...",
                "beginner": "Definition oder URL..."
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "type": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name des Typs. Dies ist die primaere Kennung fuer die Komponente."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine detaillierte Beschreibung dessen, was dieser Typ darstellt und wie er verwendet werden sollte."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "icon": {
                    "label": {
                      "normal": "Symbol",
                      "beginner": "Ein Icon zur visuellen Darstellung dieses Typs. Kann ein Emoji, Iconname oder URL zu einem Bild sein."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "image": {
                    "label": {
                      "normal": "Bild",
                      "beginner": "URL zu einem Bild, das diesen Typ darstellt. Wird fuer Vorschauen und visuelle Identifikation verwendet."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "unit": {
                    "label": {
                      "normal": "Einheit",
                      "beginner": "Die Masseinheit fuer diesen Typ (z.B. mm, m, ft)."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "parent": {
                    "label": {
                      "normal": "Elterntyp",
                      "beginner": "Der Elterntyp, von dem dieser Typ erbt"
                    }
                  },
                  "abstract": {
                    "label": {
                      "normal": "Abstrakt",
                      "beginner": "Ob dies ein abstrakter Typ ist"
                    }
                  }
                },
                "models": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "url": {
                    "label": {
                      "normal": "URL",
                      "beginner": "URL zu einem 3D-Modell, Bild oder einer anderen Ressource, die diesen Typ darstellt."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung dessen, was diese Darstellung zeigt oder wie sie verwendet werden sollte."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "tags": {
                    "label": {
                      "normal": "Tags",
                      "beginner": "Tags zum Kategorisieren und Filtern von Darstellungen (z.B. 'detailliert', 'vereinfacht', 'lod1')."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                },
                "connectors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "port": {
                    "label": {
                      "normal": "Familie",
                      "beginner": "Connector-Familienname. Connectors derselben Familie koennen miteinander verbunden werden."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "compatiblePorts": {
                    "label": {
                      "normal": "Kompatible Familien",
                      "beginner": "Liste anderer Connector-Familien, mit denen dieser Connector verbunden werden kann. Leer lassen, um alle Familien zuzulassen."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Beschreibung",
                      "beginner": "Eine Beschreibung dessen, was dieser Connector darstellt und wie er fuer Verbindungen verwendet werden sollte."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "Position auf dem Diagrammring (0-1). Steuert, wo der Connector in der 2D-Diagrammansicht erscheint."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position auf dem Diagrammring (0-1). Steuert, wo der Connector in der 2D-Diagrammansicht erscheint."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "direction": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X-Koordinate des Connector-Richtungsvektors. Dies definiert, in welche Richtung der Connector im 3D-Raum zeigt."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y-Koordinate des Connector-Richtungsvektors. Dies definiert, in welche Richtung der Connector im 3D-Raum zeigt."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z-Koordinate des Connector-Richtungsvektors. Dies definiert, in welche Richtung der Connector im 3D-Raum zeigt."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Ein optionaler Name fuer diesen Connector."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Pflichtfeld",
                      "beginner": "Ob dieser Connector fuer eine gueltige Verbindung zwingend erforderlich ist."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "maxChildren": {
                    "label": {
                      "normal": "Max. Kinder",
                      "beginner": "Die maximale Anzahl von Verbindungen, die an diesem Connector erlaubt sind."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "point": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X-Position des Connectors im 3D-Raum relativ zum Ursprung des Typs."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y-Position des Connectors im 3D-Raum relativ zum Ursprung des Typs."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z-Position des Connectors im 3D-Raum relativ zum Ursprung des Typs."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    }
                  }
                },
                "connector": {
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position auf dem Diagrammring (0-1). Steuert, wo der Connector in der 2D-Diagrammansicht erscheint."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Der Name des Attributs im Kebab-Case (z.B. 'material.holz', 'kosten.arbeit')."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "value": {
                    "label": {
                      "normal": "Wert",
                      "beginner": "Der Wert des Attributs. Leer lassen fuer boolesche Attribute (Anwesenheit = wahr)."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "Optionale Definition oder Dokumentation fuer dieses Attribut. Kann Text oder eine URL sein."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Vollstaendiger Name des Autors oder Mitwirkenden."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "email": {
                    "label": {
                      "normal": "E-Mail",
                      "beginner": "E-Mail-Adresse zur Kontaktaufnahme mit dem Autor."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                }
              }
            }
          },
          "footer": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "someAction": {
              "label": {
                "normal": "Some Action",
                "beginner": "Some Action"
              }
            }
          },
          "tools": {
            "label": {
              "normal": "Werkzeuge",
              "beginner": "Werkzeuge"
            },
            "select": {
              "normal": {
                "label": {
                  "normal": "Normal",
                  "beginner": "Normale Auswahl"
                }
              },
              "additive": {
                "label": {
                  "normal": "Additiv",
                  "beginner": "Additive Auswahl - zur bestehenden Auswahl hinzufügen"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtraktiv",
                  "beginner": "Subtraktive Auswahl - von bestehender Auswahl entfernen"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Schnittmenge",
                  "beginner": "Schnittmengen-Auswahl - nur überlappende Elemente auswählen"
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rechteckig",
                  "beginner": "Rechteckige Lasso-Auswahl"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freihand",
                  "beginner": "Freihand-Lasso-Auswahl"
                }
              }
            },
            "hand": {
              "label": {
                "normal": "Hand",
                "beginner": "Hand-Werkzeug - Schwenken und Navigieren"
              }
            },
            "connector": {
              "label": {
                "normal": "Konnektor",
                "beginner": "Konnektor-Erstellungswerkzeug"
              }
            },
            "selection": {
              "label": {
                "normal": "Auswahl",
                "beginner": "Auswahlwerkzeug"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Titel",
              "beginner": "Titel"
            }
          }
        },
        "quality": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "description": {
            "label": {
              "normal": "Messqualitaeten definieren",
              "beginner": "Messqualitaeten definieren"
            }
          },
          "tools": {
            "select": {
              "additive": {
                "label": {
                  "normal": "Additiv",
                  "beginner": "Additive Auswahl - zur bestehenden Auswahl hinzufügen"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtraktiv",
                  "beginner": "Subtraktive Auswahl - von bestehender Auswahl entfernen"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Schnittmenge",
                  "beginner": "Schnittmengen-Auswahl - nur überlappende Elemente auswählen"
                }
              }
            },
            "selection": {
              "label": {
                "normal": "Auswahl",
                "beginner": "Auswahlwerkzeug"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "Neue Qualitaet",
              "beginner": "Neue Qualitaet"
            }
          },
          "numericFunctions": {
            "label": {
              "normal": "Numerische Funktionen",
              "beginner": "Numerische Funktionen"
            }
          },
          "add": {
            "label": {
              "normal": "Addieren",
              "beginner": "Addieren"
            }
          },
          "subtract": {
            "label": {
              "normal": "Subtrahieren",
              "beginner": "Subtrahieren"
            }
          },
          "multiply": {
            "label": {
              "normal": "Multiplizieren",
              "beginner": "Multiplizieren"
            }
          },
          "divide": {
            "label": {
              "normal": "Dividieren",
              "beginner": "Dividieren"
            }
          },
          "branchingFunctions": {
            "label": {
              "normal": "Verzweigungsfunktionen",
              "beginner": "Verzweigungsfunktionen"
            }
          },
          "if": {
            "label": {
              "normal": "Wenn",
              "beginner": "Wenn"
            }
          },
          "switch": {
            "label": {
              "normal": "Schalter",
              "beginner": "Schalter"
            }
          },
          "dataStructures": {
            "label": {
              "normal": "Datenstrukturen",
              "beginner": "Datenstrukturen"
            }
          },
          "list": {
            "label": {
              "normal": "Liste",
              "beginner": "Liste"
            }
          },
          "dictionary": {
            "label": {
              "normal": "Woerterbuch",
              "beginner": "Woerterbuch"
            }
          },
          "noQualities": {
            "label": {
              "normal": "Keine Qualitaeten definiert",
              "beginner": "Keine Qualitaeten definiert"
            }
          },
          "key": {
            "label": {
              "normal": "Schluessel",
              "beginner": "Der eindeutige Bezeichner fuer diese Qualitaet"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Der Anzeigename fuer diese Qualitaet"
            }
          },
          "kind": {
            "label": {
              "normal": "Art",
              "beginner": "Der Entitaetstyp, auf den diese Qualitaet anwendbar ist"
            }
          },
          "formula": {
            "label": {
              "normal": "Formel",
              "beginner": "Die Formel zur Berechnung dieser Qualitaet"
            }
          },
          "formulaPlaceholder": "Formel eingeben...",
          "defaultValue": {
            "label": {
              "normal": "Standardwert",
              "beginner": "Der Standardwert fuer diese Qualitaet"
            }
          },
          "defaultSiUnit": {
            "label": {
              "normal": "Standard-SI-Einheit",
              "beginner": "Die Standardeinheit im SI-System"
            }
          },
          "defaultImperialUnit": {
            "label": {
              "normal": "Standard-Imperial-Einheit",
              "beginner": "Die Standardeinheit im Imperial-System"
            }
          },
          "min": {
            "label": {
              "normal": "Minimum",
              "beginner": "Der minimal zulaessige Wert"
            }
          },
          "isMinExcluded": {
            "label": {
              "normal": "Minimum ausschliessen",
              "beginner": "Ob der Minimalwert ausgeschlossen ist"
            }
          },
          "max": {
            "label": {
              "normal": "Maximum",
              "beginner": "Der maximal zulaessige Wert"
            }
          },
          "isMaxExcluded": {
            "label": {
              "normal": "Maximum ausschliessen",
              "beginner": "Ob der Maximalwert ausgeschlossen ist"
            }
          },
          "canScale": {
            "label": {
              "normal": "Kann skalieren",
              "beginner": "Ob diese Qualitaet mit der Teilegroesse skaliert"
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "key": {
                "label": {
                  "normal": "Schlüssel",
                  "beginner": "Die eindeutige Schlüsselkennung der Qualität"
                }
              },
              "name": {
                "label": {
                  "normal": "Name",
                  "beginner": "Der Anzeigename der Qualität"
                }
              },
              "description": {
                "label": {
                  "normal": "Beschreibung",
                  "beginner": "Eine Beschreibung dessen, was diese Qualität misst"
                }
              },
              "formula": {
                "label": {
                  "normal": "Formel",
                  "beginner": "Die Formel zur Berechnung dieser Qualität"
                }
              },
              "defaultSiUnit": {
                "label": {
                  "normal": "Standard-SI-Einheit",
                  "beginner": "Die Standardeinheit im SI-System"
                }
              },
              "defaultImperialUnit": {
                "label": {
                  "normal": "Standard-Imperial-Einheit",
                  "beginner": "Die Standardeinheit im imperialen System"
                }
              },
              "kind": {
                "label": {
                  "normal": "Art",
                  "beginner": "Der Entitätstyp, auf den diese Qualität anwendbar ist"
                }
              },
              "canScale": {
                "label": {
                  "normal": "Skalierbar",
                  "beginner": "Ob diese Qualität mit der Teilgröße skaliert"
                }
              },
              "defaultValue": {
                "label": {
                  "normal": "Standardwert",
                  "beginner": "Der Standardwert für diese Qualität"
                }
              },
              "min": {
                "label": {
                  "normal": "Minimum",
                  "beginner": "Der minimal zulässige Wert"
                }
              },
              "max": {
                "label": {
                  "normal": "Maximum",
                  "beginner": "Der maximal zulässige Wert"
                }
              },
              "isMinExcluded": {
                "label": {
                  "normal": "Minimum ausgeschlossen",
                  "beginner": "Ob der Minimalwert exklusiv ist"
                }
              },
              "isMaxExcluded": {
                "label": {
                  "normal": "Maximum ausgeschlossen",
                  "beginner": "Ob der Maximalwert exklusiv ist"
                }
              }
            }
          },
          "title": {
            "label": {
              "normal": "Qualität",
              "beginner": "Qualität"
            }
          },
          "functions": {
            "label": {
              "normal": "Funktionen",
              "beginner": "Funktionen"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualitäten",
              "beginner": "Qualitäten"
            }
          },
          "toolbar": {
            "view": {
              "label": {
                "normal": "Ansicht",
                "beginner": "Ansichtsoptionen"
              }
            },
            "actions": {
              "label": {
                "normal": "Aktionen",
                "beginner": "Qualitätsaktionen"
              }
            }
          },
          "workbench": {
            "nodes": {
              "label": {
                "normal": "Knoten",
                "beginner": "Qualitätsformel-Knoten"
              }
            },
            "qualities": {
              "label": {
                "normal": "Qualitäten",
                "beginner": "Verfügbare Qualitäten"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Qualitäts-Einstellungen"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Qualitäts-Chat"
            }
          }
        }
      },
      "settings": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "theme": {
          "label": {
            "normal": "Design",
            "beginner": "Farbschema für die Oberfläche"
          },
          "system": {
            "label": {
              "normal": "System",
              "beginner": "System"
            }
          },
          "light": {
            "label": {
              "normal": "Hell",
              "beginner": "Hell"
            }
          },
          "dark": {
            "label": {
              "normal": "Dunkel",
              "beginner": "Dunkel"
            }
          }
        },
        "device": {
          "label": {
            "normal": "Geraet",
            "beginner": "Layoutmodus für die Oberfläche"
          },
          "desktop": {
            "label": {
              "normal": "Desktop",
              "beginner": "Desktop"
            }
          },
          "tablet": {
            "label": {
              "normal": "Tablet",
              "beginner": "Tablet"
            }
          }
        },
        "mode": {
          "label": {
            "normal": "Modus",
            "beginner": "Oberflächenmodus"
          },
          "user": {
            "label": {
              "normal": "Benutzer",
              "beginner": "Benutzer"
            }
          },
          "dev": {
            "label": {
              "normal": "Dev",
              "beginner": "Dev"
            }
          }
        },
        "expertise": {
          "label": {
            "normal": "Erfahrung",
            "beginner": "Ihre Erfahrungsstufe"
          },
          "beginner": {
            "label": {
              "normal": "Anfänger",
              "beginner": "Detaillierte Hilfe und Tutorials anzeigen"
            }
          },
          "normal": {
            "label": {
              "normal": "Normal",
              "beginner": "Standard-Tooltips anzeigen"
            }
          },
          "expert": {
            "label": {
              "normal": "Experte",
              "beginner": "Experte"
            }
          }
        },
        "language": {
          "label": {
            "normal": "Sprache",
            "beginner": "Waehlen Sie die Sprache fuer die Anwendungsoberflaeche"
          },
          "placeholder": {
            "label": {
              "normal": "Sprache waehlen...",
              "beginner": "Waehlen Sie die Sprache, in der die Anwendung angezeigt wird"
            }
          },
          "de": {
            "label": {
              "normal": "Deutsch",
              "beginner": "Deutsch"
            }
          },
          "en": {
            "label": {
              "normal": "Englisch",
              "beginner": "Englisch"
            }
          }
        }
      },
      "tool": {
        "label": {
          "normal": "Werkzeuge",
          "beginner": "Werkzeuge"
        },
        "selection": {
          "label": {
            "normal": "Auswahl",
            "beginner": "Auswahl"
          },
          "normal": {
            "label": {
              "normal": "Normale Auswahl",
              "beginner": "Klicken Sie, um jeweils ein Element auszuwählen"
            },
            "manual": "selection",
            "hotkey": "1"
          },
          "additive": {
            "label": {
              "normal": "Zur Auswahl hinzufügen",
              "beginner": "Klicken Sie, um Elemente zur Auswahl hinzuzufügen, ohne Strg zu halten"
            },
            "manual": "selection",
            "hotkey": "2"
          },
          "subtractive": {
            "label": {
              "normal": "Von Auswahl entfernen",
              "beginner": "Klicken Sie, um Elemente aus der Auswahl zu entfernen, ohne Alt zu halten"
            },
            "manual": "selection",
            "hotkey": "3"
          }
        }
      },
      "sort": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "ascending": {
          "label": {
            "normal": "Aufsteigend",
            "beginner": "Aufsteigend"
          }
        },
        "descending": {
          "label": {
            "normal": "Absteigend",
            "beginner": "Absteigend"
          }
        }
      },
      "docs": {
        "navigation": {
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "toolbar": {
        "label": {
          "normal": "Werkzeugleiste",
          "beginner": "Werkzeugleiste"
        },
        "group": {
          "hand": {
            "label": {
              "normal": "Hand",
              "beginner": "Hand"
            }
          },
          "selection": {
            "label": {
              "normal": "Auswahl",
              "beginner": "Auswahl"
            }
          },
          "lasso": {
            "label": {
              "normal": "Lasso",
              "beginner": "Lasso"
            }
          },
          "filter": {
            "label": {
              "normal": "Filter",
              "beginner": "Filter"
            }
          },
          "open": {
            "label": {
              "normal": "Öffnen",
              "beginner": "Öffnen"
            }
          },
          "create": {
            "label": {
              "normal": "Erstellen",
              "beginner": "Erstellen"
            }
          },
          "view": {
            "label": {
              "normal": "Ansicht",
              "beginner": "Ansicht"
            }
          },
          "actions": {
            "label": {
              "normal": "Aktionen",
              "beginner": "Aktionen"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Einstellungen"
            }
          }
        },
        "parent": {
          "selection": {
            "label": {
              "normal": "Auswahl",
              "beginner": "Auswahl"
            }
          },
          "hand": {
            "label": {
              "normal": "Hand",
              "beginner": "Hand"
            }
          },
          "lasso": {
            "label": {
              "normal": "Lasso",
              "beginner": "Lasso"
            }
          },
          "filter": {
            "label": {
              "normal": "Filter",
              "beginner": "Filter"
            }
          },
          "open": {
            "label": {
              "normal": "Öffnen",
              "beginner": "Öffnen"
            }
          },
          "create": {
            "label": {
              "normal": "Erstellen",
              "beginner": "Erstellen"
            }
          },
          "view": {
            "label": {
              "normal": "Ansicht",
              "beginner": "Ansicht"
            }
          },
          "actions": {
            "label": {
              "normal": "Aktionen",
              "beginner": "Aktionen"
            }
          },
          "settings": {
            "label": {
              "normal": "Einstellungen",
              "beginner": "Einstellungen"
            }
          }
        }
      },
      "tutorial": {
        "controls": {
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          },
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "recording": {
        "controls": {
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          }
        }
      }
    }
  },
  "tooltip": {
    "manual": {
      "label": {
        "normal": "Handbuch",
        "beginner": "Handbuch"
      }
    },
    "tutorial": {
      "label": {
        "normal": "Tutorial",
        "beginner": "Tutorial"
      }
    }
  },
  "settings": {
    "layout": {
      "normal": {
        "label": {
          "normal": "Normal layout",
          "beginner": "Use the standard layout optimized for mouse and keyboard."
        }
      },
      "desktop": {
        "label": {
          "normal": "Desktop layout",
          "beginner": "Use the desktop layout optimized for large screens."
        }
      },
      "tablet": {
        "label": {
          "normal": "Tablet layout",
          "beginner": "Use the tablet layout optimized for medium screens."
        }
      },
      "mobile": {
        "label": {
          "normal": "Mobile layout",
          "beginner": "Use the mobile layout optimized for small screens."
        }
      }
    },
    "mode": {
      "dev": {
        "label": {
          "normal": "Developer mode",
          "beginner": "Show developer tools and advanced options."
        }
      },
      "user": {
        "label": {
          "normal": "User mode",
          "beginner": "Show standard user port."
        }
      },
      "beginner": {
        "label": {
          "normal": "Beginner mode",
          "beginner": "Show full guidance, tutorials, and detailed tooltips."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal mode",
          "beginner": "Show contextual help without tutorials."
        }
      }
    },
    "expertise": {
      "beginner": {
        "label": {
          "normal": "Anfänger",
          "beginner": "Show full guidance and tutorials."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal",
          "beginner": "Show contextual help."
        }
      },
      "expert": {
        "label": {
          "normal": "Experte",
          "beginner": "Hide guidance."
        }
      }
    }
  }
}
`),
  },
  en: {
    translation: JSON.parse(String.raw`{
  "semio": {
    "label": {
      "normal": "",
      "beginner": ""
    },
    "file": {
      "name": "Name",
      "size": "Size",
      "created": "Created",
      "updated": "Updated"
    },
    "folder": {
      "created": "Created",
      "updated": "Updated"
    },
    "sketchpad": {
      "label": {
        "normal": "",
        "beginner": ""
      },
      "navbar": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "back": {
          "label": {
            "normal": "Go back",
            "beginner": "Click to go back, hold to see history"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Left"
        },
        "forward": {
          "label": {
            "normal": "Go forward",
            "beginner": "Click to go forward, hold to see history"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Right"
        },
        "up": {
          "label": {
            "normal": "Go up one level",
            "beginner": "Click to go up one level in the navigation hierarchy"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Up"
        },
        "kits": {
          "label": {
            "normal": "Kits",
            "beginner": "Click to see all kits"
          }
        },
        "navigationButtons": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigation buttons"
          }
        },
        "docs": {
          "label": {
            "normal": "Documentation",
            "beginner": "Click to view documentation"
          },
          "hotkey": "Ctrl+Shift+D"
        },
        "search": {
          "label": {
            "normal": "Search",
            "beginner": "Search for content"
          },
          "open": {
            "label": {
              "normal": "Search",
              "beginner": "Click to open search to quickly find and navigate to any element"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Ctrl+K"
          },
          "close": {
            "label": {
              "normal": "Close Search",
              "beginner": "Click to close the search dialog"
            },
            "manual": "navigation#search",
            "tutorial": "getting-started/intro#search",
            "hotkey": "Escape"
          },
          "title": {
            "label": {
              "normal": "Search",
              "beginner": "Search"
            }
          },
          "description": {
            "label": {
              "normal": "Search for kits, designs, types, and more",
              "beginner": "Search for kits, designs, types, and more"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Search...",
              "beginner": "Search..."
            }
          },
          "noResults": {
            "label": {
              "normal": "No results found",
              "beginner": "No results found"
            }
          }
        },
        "focus": {
          "label": {
            "normal": "Focus Mode",
            "beginner": "Toggle focus mode to hide distractions"
          },
          "open": {
            "label": {
              "normal": "Focus",
              "beginner": "Click to enter focus mode and hide distractions"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Ctrl+Shift+F"
          },
          "close": {
            "label": {
              "normal": "Exit Focus",
              "beginner": "Click to exit focus mode and show all UI elements"
            },
            "manual": "navigation#focus",
            "tutorial": "getting-started/intro#focus",
            "hotkey": "Escape"
          },
          "input": {
            "label": {
              "normal": "Focus Input",
              "beginner": "Type to search for an element to focus on"
            }
          },
          "placeholder": {
            "label": {
              "normal": "Search for an element...",
              "beginner": "Search for an element..."
            }
          },
          "title": {
            "label": {
              "normal": "Focus",
              "beginner": "Focus"
            }
          },
          "description": {
            "label": {
              "normal": "Focus on an element in the current view",
              "beginner": "Focus on an element in the current view"
            }
          },
          "other": {
            "label": {
              "normal": "Other",
              "beginner": "Other"
            }
          }
        },
        "copyJsonToClipboard": {
          "label": {
            "normal": "Copy JSON",
            "beginner": "Copy the current sketchpad JSON state to clipboard"
          },
          "hotkey": "Ctrl+Shift+J"
        },
        "breadcrumb": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "designs": {
            "label": {
              "normal": "Designs",
              "beginner": "Designs"
            }
          },
          "types": {
            "label": {
              "normal": "Types",
              "beginner": "Types"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualities",
              "beginner": "Qualities"
            }
          },
          "temporary": {
            "label": {
              "normal": "Temporary",
              "beginner": "Temporary"
            }
          },
          "local": {
            "label": {
              "normal": "Local",
              "beginner": "Local"
            }
          },
          "remote": {
            "label": {
              "normal": "Remote",
              "beginner": "Remote"
            }
          },
          "files": {
            "label": {
              "normal": "Files",
              "beginner": "Files"
            }
          },
          "authors": {
            "label": {
              "normal": "Authors",
              "beginner": "Authors"
            }
          }
        },
        "tutorials": {
          "label": {
            "normal": "Tutorials",
            "beginner": "Tutorials"
          }
        },
        "tutorial": {
          "controls": {
            "stop": {
              "label": {
                "normal": "Stop Tutorial",
                "beginner": "Click to stop the current tutorial"
              }
            },
            "previous": {
              "label": {
                "normal": "Previous Step",
                "beginner": "Go to the previous step in the tutorial"
              }
            },
            "playPause": {
              "label": {
                "normal": "Play/Pause",
                "beginner": "Play or pause the tutorial"
              }
            },
            "next": {
              "label": {
                "normal": "Next Step",
                "beginner": "Go to the next step in the tutorial"
              }
            }
          }
        },
        "recording": {
          "controls": {
            "playPause": {
              "label": {
                "normal": "Play/Pause Recording",
                "beginner": "Play or pause the recording"
              }
            },
            "stop": {
              "label": {
                "normal": "Stop Recording",
                "beginner": "Stop the recording and save it"
              }
            }
          }
        },
        "createKit": {
          "label": {
            "normal": "Create Kit",
            "beginner": "Click to create a new kit"
          }
        },
        "createDesign": {
          "label": {
            "normal": "Create Design",
            "beginner": "Click to create a new design"
          }
        },
        "createChild": {
          "label": {
            "normal": "Create Child",
            "beginner": "Click to create a child artifact"
          }
        },
        "createType": {
          "label": {
            "normal": "Create Type",
            "beginner": "Click to create a new type"
          }
        },
        "createVersion": {
          "label": {
            "normal": "Create Version",
            "beginner": "Click to create a new version"
          }
        },
        "searchInput": {
          "label": {
            "normal": "Search Input",
            "beginner": "Type to search for elements"
          }
        },
        "panelToggle": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "workbench": {
            "label": {
              "normal": "Toggle Workbench",
              "beginner": "Toggle the Workbench panel on the left side"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "hud": {
            "label": {
              "normal": "Toggle HUD",
              "beginner": "Toggle the HUD panel in the middle"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "right": {
            "label": {
              "normal": "Toggle Right Panel",
              "beginner": "Toggle the right panel for details and settings"
            }
          },
          "tools": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "toolbar": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "stats": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "details": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "chat": {
            "label": {
              "normal": "Toggle Chat",
              "beginner": "Toggle the chat panel"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "console": {
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Toggle Settings",
              "beginner": "Toggle the settings panel"
            },
            "show": {
              "label": {
                "normal": "Show",
                "beginner": "Show"
              }
            }
          },
          "leftSidePanel": {
            "label": {
              "normal": "Toggle Left Panel",
              "beginner": "Toggle the left side panel with workbench tabs"
            }
          },
          "rightSidePanel": {
            "label": {
              "normal": "Toggle Right Panel",
              "beginner": "Toggle the right side panel with details tabs"
            }
          },
          "hudPanel": {
            "label": {
              "normal": "Toggle HUD Panel",
              "beginner": "Toggle the center HUD panel"
            }
          }
        },
        "home": {
          "label": {
            "normal": "Home",
            "beginner": "Home"
          }
        },
        "kitName": {
          "label": {
            "normal": "Kit Name",
            "beginner": "Kit Name"
          }
        },
        "kitVersion": {
          "label": {
            "normal": "Kit Version",
            "beginner": "Kit Version"
          }
        },
        "name": {
          "label": {
            "normal": "Name",
            "beginner": "Name"
          }
        },
        "design": {
          "label": {
            "normal": "Design",
            "beginner": "Design"
          }
        },
        "type": {
          "label": {
            "normal": "Type",
            "beginner": "Type"
          }
        },
        "quality": {
          "label": {
            "normal": "Quality",
            "beginner": "Quality"
          }
        },
        "navigation": {
          "label": {
            "normal": "Navigation",
            "beginner": "Navigation"
          }
        },
        "panelToggles": {
          "label": {
            "normal": "Panel Toggles",
            "beginner": "Panel Toggles"
          }
        },
        "fullscreenToggle": {
          "label": {
            "normal": "Fullscreen Toggle",
            "beginner": "Fullscreen Toggle"
          }
        }
      },
      "panel": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "chat": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "placeholder": {
            "label": {
              "normal": "Ask anything...",
              "beginner": "Ask anything..."
            }
          }
        },
        "settings": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "theme": {
            "label": {
              "normal": "Theme",
              "beginner": "Choose the color theme for the application"
            },
            "dark": {
              "label": {
                "normal": "Dark",
                "beginner": "Use dark color scheme"
              }
            },
            "light": {
              "label": {
                "normal": "Light",
                "beginner": "Use light color scheme"
              }
            },
            "system": {
              "label": {
                "normal": "System",
                "beginner": "Follow system theme preference"
              }
            }
          },
          "device": {
            "label": {
              "normal": "Device",
              "beginner": "Choose the device mode for interaction"
            },
            "desktop": {
              "label": {
                "normal": "Desktop",
                "beginner": "Optimized for mouse and keyboard"
              }
            },
            "tablet": {
              "label": {
                "normal": "Tablet",
                "beginner": "Optimized for touch interaction"
              }
            },
            "mobile": {
              "label": {
                "normal": "Mobile",
                "beginner": "Optimized for touch interaction on small screens"
              }
            }
          },
          "mode": {
            "label": {
              "normal": "Mode",
              "beginner": "Select the user port mode: Expert (minimal tooltips), Normal (standard), or Beginner (detailed help)"
            },
            "dev": {
              "label": {
                "normal": "Developer",
                "beginner": "Developer mode with advanced tools and debugging features"
              }
            },
            "user": {
              "label": {
                "normal": "User",
                "beginner": "Standard user mode for regular operations"
              }
            }
          },
          "expertise": {
            "label": {
              "normal": "Expertise",
              "beginner": "Select your expertise level to adjust the port complexity"
            },
            "beginner": {
              "label": {
                "normal": "Beginner",
                "beginner": "Show detailed explanations and tutorials"
              }
            },
            "normal": {
              "label": {
                "normal": "Normal",
                "beginner": "Show standard tooltips and help"
              }
            },
            "expert": {
              "label": {
                "normal": "Expert",
                "beginner": "Minimal tooltips for experienced users"
              }
            }
          }
        }
      },
      "common": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "selectVariant": "Select variant...",
        "selectView": "Select view...",
        "search": {
          "label": {
            "normal": "Search",
            "beginner": "Search"
          }
        },
        "mixedValues": {
          "label": {
            "normal": "Mixed values",
            "beginner": "Mixed values"
          }
        },
        "selectDesign": {
          "label": {
            "normal": "Select design",
            "beginner": "Select design"
          }
        },
        "selectType": {
          "label": {
            "normal": "Select type",
            "beginner": "Select type"
          }
        },
        "settings": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "theme": {
            "label": {
              "normal": "Theme",
              "beginner": "Choose the color theme"
            }
          },
          "layout": {
            "label": {
              "normal": "Layout",
              "beginner": "Choose the layout mode"
            }
          },
          "mode": {
            "label": {
              "normal": "Mode",
              "beginner": "Choose the port mode"
            }
          },
          "expertise": {
            "label": {
              "normal": "Expertise Level",
              "beginner": "Choose your expertise level"
            }
          }
        },
        "no": {
          "label": {
            "normal": "No",
            "beginner": "No"
          }
        },
        "yes": {
          "label": {
            "normal": "Yes",
            "beginner": "Yes"
          }
        },
        "add": {
          "label": {
            "normal": "Add",
            "beginner": "Add"
          }
        },
        "remove": {
          "label": {
            "normal": "Remove",
            "beginner": "Remove"
          }
        },
        "addChild": {
          "label": {
            "normal": "Add Child",
            "beginner": "Add Child"
          }
        },
        "duplicateType": {
          "label": {
            "normal": "Duplicate Type by Hover",
            "beginner": "Duplicate Type by Hover"
          }
        },
        "addType": {
          "label": {
            "normal": "Add Type",
            "beginner": "Add Type"
          }
        },
        "addDesign": {
          "label": {
            "normal": "Add Design",
            "beginner": "Add Design"
          }
        }
      },
      "footer": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Send feedback to help improve Semio"
          }
        }
      },
      "settings": {
        "label": {
          "normal": "Sketchpad",
          "beginner": "Global sketchpad settings"
        },
        "theme": {
          "label": {
            "normal": "Theme",
            "beginner": "Color scheme for the port"
          },
          "system": {
            "label": {
              "normal": "System",
              "beginner": "System"
            }
          },
          "light": {
            "label": {
              "normal": "Light",
              "beginner": "Light"
            }
          },
          "dark": {
            "label": {
              "normal": "Dark",
              "beginner": "Dark"
            }
          }
        },
        "device": {
          "label": {
            "normal": "Device",
            "beginner": "Device mode for the port"
          },
          "desktop": {
            "label": {
              "normal": "Desktop",
              "beginner": "Desktop"
            }
          },
          "tablet": {
            "label": {
              "normal": "Tablet",
              "beginner": "Tablet"
            }
          }
        },
        "mode": {
          "label": {
            "normal": "Mode",
            "beginner": "Port mode"
          },
          "user": {
            "label": {
              "normal": "User",
              "beginner": "User"
            }
          },
          "dev": {
            "label": {
              "normal": "Dev",
              "beginner": "Dev"
            }
          }
        },
        "expertise": {
          "label": {
            "normal": "Expertise",
            "beginner": "Your expertise level"
          },
          "beginner": {
            "label": {
              "normal": "Beginner",
              "beginner": "Show detailed help and tutorials"
            }
          },
          "normal": {
            "label": {
              "normal": "Normal",
              "beginner": "Show standard tooltips"
            }
          },
          "expert": {
            "label": {
              "normal": "Expert",
              "beginner": "Expert"
            }
          }
        },
        "language": {
          "label": {
            "normal": "Language",
            "beginner": "Select the language for the application port"
          },
          "placeholder": {
            "label": {
              "normal": "Select language...",
              "beginner": "Select the port language"
            }
          },
          "de": {
            "label": {
              "normal": "Deutsch",
              "beginner": "Deutsch"
            }
          },
          "en": {
            "label": {
              "normal": "English",
              "beginner": "English"
            }
          }
        }
      },
      "tool": {
        "label": {
          "normal": "Tools",
          "beginner": "Tools"
        },
        "selection": {
          "label": {
            "normal": "Selection",
            "beginner": "Selection"
          },
          "normal": {
            "label": {
              "normal": "Normal Selection",
              "beginner": "Click to select one item at a time"
            },
            "manual": "selection",
            "hotkey": "1"
          },
          "additive": {
            "label": {
              "normal": "Add to Selection",
              "beginner": "Click to add items to your selection without holding Ctrl"
            },
            "manual": "selection",
            "hotkey": "2"
          },
          "subtractive": {
            "label": {
              "normal": "Remove from Selection",
              "beginner": "Click to remove items from your selection without holding Alt"
            },
            "manual": "selection",
            "hotkey": "3"
          }
        }
      },
      "sort": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "ascending": {
          "label": {
            "normal": "Ascending",
            "beginner": "Ascending"
          }
        },
        "descending": {
          "label": {
            "normal": "Descending",
            "beginner": "Descending"
          }
        }
      },
      "app": {
        "label": {
          "normal": "",
          "beginner": ""
        },
        "home": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "title": "Home",
          "fileInput": {
            "label": {
              "normal": "Choose kit file",
              "beginner": "Select a .zip kit file from your device"
            }
          },
          "searchPlaceholder": {
            "label": {
              "normal": "Search kits...",
              "beginner": "Search kits..."
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "kind": {
            "label": {
              "normal": "Kind",
              "beginner": "Kind"
            }
          },
          "lastUpdated": {
            "label": {
              "normal": "Last Updated",
              "beginner": "Last Updated"
            }
          },
          "created": {
            "label": {
              "normal": "Created",
              "beginner": "Created"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "band": {
              "label": {
                "normal": "Filter band",
                "beginner": "Toggle the filter band to show or hide filter options"
              },
              "hotkey": "Ctrl+F"
            },
            "kind": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "show": {
                "label": {
                  "normal": "Filter by kind",
                  "beginner": "Filter kits by their storage type (temporary, local, or remote)"
                },
                "hotkey": "Ctrl+K"
              },
              "create": {
                "label": {
                  "normal": "Create new kit",
                  "beginner": "Create a new kit of this type"
                },
                "hotkey": "Ctrl+Shift+K"
              },
              "temporary": {
                "label": {
                  "normal": "Show temporary kits",
                  "beginner": "Show kits stored in browser memory (lost on refresh)"
                },
                "hotkey": "Ctrl+1"
              },
              "createTemporary": {
                "label": {
                  "normal": "Create temporary kit",
                  "beginner": "Create a new temporary kit stored in browser memory"
                },
                "hotkey": "Ctrl+Shift+1"
              },
              "local": {
                "label": {
                  "normal": "Show local kits",
                  "beginner": "Show kits stored locally on your device"
                },
                "hotkey": "Ctrl+2"
              },
              "createLocal": {
                "label": {
                  "normal": "Create local kit",
                  "beginner": "Create a new kit stored locally on your device"
                },
                "hotkey": "Ctrl+Shift+2"
              },
              "remote": {
                "label": {
                  "normal": "Show remote kits",
                  "beginner": "Show kits synced with remote storage"
                },
                "hotkey": "Ctrl+3"
              },
              "createRemote": {
                "label": {
                  "normal": "Create remote kit",
                  "beginner": "Create a new kit synced with remote storage"
                },
                "hotkey": "Ctrl+Shift+3"
              }
            },
            "name": {
              "label": {
                "normal": "Filter by name",
                "beginner": "Filter kits by this specific name"
              },
              "hotkey": "Ctrl+N"
            },
            "version": {
              "label": {
                "normal": "Filter by version",
                "beginner": "Filter kits to this specific version"
              },
              "hotkey": "Ctrl+V"
            }
          },
          "search": {
            "label": {
              "normal": "Search",
              "beginner": "Search for kits"
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Home settings"
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Choose the color theme for the application"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Select the language for the application port"
              },
              "placeholder": {
                "label": {
                  "normal": "Select language...",
                  "beginner": "Select the port language"
                }
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "Select the user port mode"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Select your expertise level"
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Select the input device"
              }
            },
            "layout": {
              "label": {
                "normal": "Layout",
                "beginner": "Choose the layout for the kit overview"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "Configure the force-directed diagram layout"
              },
              "chargeStrength": {
                "label": {
                  "normal": "Charge Strength",
                  "beginner": "Controls how strongly nodes repel each other. More negative values push nodes further apart."
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Link Distance",
                  "beginner": "The target distance between connected nodes. Larger values spread the diagram out."
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Collision Radius",
                  "beginner": "The minimum distance between node centers to prevent overlap."
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Center Strength",
                  "beginner": "How strongly nodes are pulled toward the center of the diagram."
                }
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "createKit": {
                "label": {
                  "normal": "Create kit",
                  "beginner": "Create a new kit from the home table"
                }
              },
              "createVersion": {
                "label": {
                  "normal": "Create version",
                  "beginner": "Create a new kit version from the home table"
                }
              },
              "hover": {
                "label": {
                  "normal": "Highlight kit",
                  "beginner": "Hover a kit row to highlight it in the diagram"
                }
              },
              "toggleSort": {
                "label": {
                  "normal": "Toggle sort",
                  "beginner": "Change sort direction for this column"
                }
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "kit": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "The name of the kit"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "The version of the kit"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Description",
                    "beginner": "A description of the kit"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Icon",
                    "beginner": "The icon of the kit"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Image",
                    "beginner": "The preview image of the kit"
                  }
                }
              },
              "kits": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "name": {
                  "label": {
                    "normal": "Name",
                    "beginner": "The name of the selected kits"
                  }
                },
                "version": {
                  "label": {
                    "normal": "Version",
                    "beginner": "The version of the selected kits"
                  }
                },
                "description": {
                  "label": {
                    "normal": "Description",
                    "beginner": "A description of the selected kits"
                  }
                },
                "icon": {
                  "label": {
                    "normal": "Icon",
                    "beginner": "The icon of the selected kits"
                  }
                },
                "image": {
                  "label": {
                    "normal": "Image",
                    "beginner": "The preview image of the selected kits"
                  }
                }
              }
            }
          },
          "dropzone": {
            "label": {
              "normal": "Drop zip file to import kit",
              "beginner": "Drop a zip file here to import it as a new kit"
            },
            "description": {
              "normal": "Only kits with .semio folder can be imported",
              "beginner": "The zip file must contain a .semio folder with kit.db to be imported as a kit."
            }
          },
          "noKits": {
            "label": {
              "normal": "No Kits",
              "beginner": "No Kits"
            }
          },
          "sortByName": {
            "label": {
              "normal": "Sort By Name",
              "beginner": "Sort By Name"
            }
          },
          "toggleRow": {
            "label": {
              "normal": "Toggle Row",
              "beginner": "Toggle Row"
            }
          },
          "createVersion": {
            "label": {
              "normal": "Create Version",
              "beginner": "Create Version"
            }
          },
          "hideKind": {
            "label": {
              "normal": "Hide Kind",
              "beginner": "Hide Kind"
            }
          },
          "showTemporary": {
            "label": {
              "normal": "Show Temporary",
              "beginner": "Show Temporary"
            }
          },
          "showLocal": {
            "label": {
              "normal": "Show Local",
              "beginner": "Show Local"
            }
          },
          "showRemote": {
            "label": {
              "normal": "Show Remote",
              "beginner": "Show Remote"
            }
          },
          "sortByType": {
            "label": {
              "normal": "Sort By Type",
              "beginner": "Sort By Type"
            }
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Sort By Updated At",
              "beginner": "Sort By Updated At"
            }
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Sort By Created At",
              "beginner": "Sort By Created At"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Chat"
            }
          },
          "createKit": {
            "label": {
              "normal": "Create Kit",
              "beginner": "Create Kit"
            }
          },
          "createTemporary": {
            "label": {
              "normal": "Create Temporary",
              "beginner": "Create Temporary"
            }
          },
          "createLocal": {
            "label": {
              "normal": "Create Local",
              "beginner": "Create Local"
            }
          },
          "createRemote": {
            "label": {
              "normal": "Create Remote",
              "beginner": "Create Remote"
            }
          },
          "importKit": {
            "label": {
              "normal": "Import Kit",
              "beginner": "Import Kit"
            }
          },
          "toolbar": {
            "showTemporary": {
              "label": {
                "normal": "Temporary",
                "beginner": "Temporary"
              }
            },
            "showLocal": {
              "label": {
                "normal": "Local",
                "beginner": "Local"
              }
            },
            "showRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "createTemporary": {
              "label": {
                "normal": "Temporary",
                "beginner": "Temporary"
              }
            },
            "createLocal": {
              "label": {
                "normal": "Local",
                "beginner": "Local"
              }
            },
            "createRemote": {
              "label": {
                "normal": "Remote",
                "beginner": "Remote"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter kits by location"
              }
            },
            "create": {
              "label": {
                "normal": "Create",
                "beginner": "Create a new kit"
              }
            },
            "createKit": {
              "label": {
                "normal": "New kit",
                "beginner": "Create a new empty kit"
              }
            },
            "openFolder": {
              "label": {
                "normal": "Open folder",
                "beginner": "Open a kit from a folder on disk"
              }
            },
            "openFile": {
              "label": {
                "normal": "Open file",
                "beginner": "Open a kit from a .zip file"
              }
            },
            "openRemote": {
              "label": {
                "normal": "Open remote",
                "beginner": "Open a kit from a remote URL"
              }
            },
            "createFile": {
              "label": {
                "normal": "New file kit",
                "beginner": "Create a kit backed by a file"
              }
            },
            "createFolder": {
              "label": {
                "normal": "New folder kit",
                "beginner": "Create a kit backed by a folder"
              }
            },
            "showFile": {
              "label": {
                "normal": "Show file kits",
                "beginner": "Show kits stored as files"
              }
            },
            "showFolder": {
              "label": {
                "normal": "Show folder kits",
                "beginner": "Show kits stored in folders"
              }
            },
            "exportArchive": {
              "label": {
                "normal": "Export archive",
                "beginner": "Export the selected kit as a .zip archive"
              }
            }
          }
        },
        "kit": {
          "label": {
            "normal": "Kit",
            "beginner": "Kit"
          },
          "properties": {
            "label": {
              "normal": "Kit Properties",
              "beginner": "Kit properties"
            }
          },
          "notFound": {
            "label": {
              "normal": "Kit not found",
              "beginner": "The requested kit could not be found"
            },
            "description": {
              "normal": "The kit may have been removed or the link is invalid.",
              "beginner": "Return home and open another kit, or create a new one."
            }
          },
          "noKitLoaded": {
            "label": {
              "normal": "No kit loaded",
              "beginner": "No kit loaded"
            }
          },
          "loading": {
            "label": {
              "normal": "Loading kit...",
              "beginner": "Loading kit..."
            }
          },
          "notAvailable": {
            "label": {
              "normal": "Kit not available",
              "beginner": "Kit not available"
            }
          },
          "search": {
            "placeholder": {
              "label": {
                "normal": "Filter...",
                "beginner": "Search for artifacts..."
              }
            }
          },
          "dropzone": {
            "label": {
              "normal": "Drop zip file to import",
              "beginner": "Drop a zip file here to import it as a kit or add files to the current kit"
            },
            "description": {
              "normal": "Kits with .semio folder will be imported, others will be added as files",
              "beginner": "If the zip contains a .semio folder, it will be imported as a complete kit. Otherwise, the files will be added to the current kit."
            }
          },
          "versionPlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., 1.0.0",
                "beginner": "e.g., 1.0.0"
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe what this kit contains...",
                "beginner": "Describe what this kit contains..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "\ud83c\udfa8 or URL to icon",
                "beginner": "\ud83c\udfa8 or URL to icon"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "URL to preview image",
                "beginner": "URL to preview image"
              }
            }
          },
          "homepagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://example.com",
                "beginner": "https://example.com"
              }
            }
          },
          "licensePlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., MIT, GPL-3.0, Apache-2.0",
                "beginner": "e.g., MIT, GPL-3.0, Apache-2.0"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Kit",
              "beginner": "New Kit"
            }
          },
          "defaultDesignName": {
            "label": {
              "normal": "New Design",
              "beginner": "New Design"
            }
          },
          "defaultTypeName": {
            "label": {
              "normal": "New Type",
              "beginner": "New Type"
            }
          },
          "newVersion": {
            "label": {
              "normal": "New Version",
              "beginner": "New Version"
            }
          },
          "defaultVersion": {
            "label": {
              "normal": "Default",
              "beginner": "Default"
            }
          },
          "filter": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "name": {
              "label": {
                "normal": "Filter by name",
                "beginner": "Click to filter artifacts by this name"
              },
              "manual": "manuals/semio/kit",
              "tutorial": "hello-semio/model-design",
              "hotkey": "Ctrl+N",
              "hide": {
                "label": {
                  "normal": "Hide name filter",
                  "beginner": "Click to hide the name filter"
                },
                "hotkey": "Ctrl+Shift+N"
              }
            },
            "band": {
              "label": {
                "normal": "Filter band",
                "beginner": "Toggle the filter band to show or hide filter options"
              },
              "hotkey": "Ctrl+F"
            },
            "search": {
              "label": {
                "normal": "Search filter",
                "beginner": "Search for specific filters"
              },
              "hotkey": "Ctrl+Shift+F"
            }
          },
          "pieces": {
            "label": {
              "normal": "Pieces",
              "beginner": "Types and designs in this kit"
            }
          },
          "designs": {
            "label": {
              "normal": "Designs",
              "beginner": "Designs in this kit"
            },
            "manual": "manuals/semio/kit#designs",
            "tutorial": "hello-semio/model-design",
            "multipleSelected": {
              "label": {
                "normal": "Multiple Selected",
                "beginner": "Multiple Selected"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "types": {
            "label": {
              "normal": "Types",
              "beginner": "Types in this kit"
            },
            "manual": "manuals/semio/kit#types",
            "tutorial": "hello-semio/model-brick-set",
            "multipleSelected": {
              "label": {
                "normal": "Multiple Selected",
                "beginner": "Multiple Selected"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "folder": {
            "label": {
              "normal": "Folder",
              "beginner": "Folder"
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Describe this folder...",
                  "beginner": "Describe this folder..."
                }
              }
            }
          },
          "sortByArtifact": {
            "label": {
              "normal": "Sort by artifact",
              "beginner": "Sort artifacts by their name"
            },
            "hotkey": "Ctrl+Shift+A"
          },
          "sortByKind": {
            "label": {
              "normal": "Sort by kind",
              "beginner": "Sort artifacts by their type (design, type, quality, etc.)"
            },
            "hotkey": "Ctrl+Shift+K"
          },
          "sortByCreatedAt": {
            "label": {
              "normal": "Sort by creation date",
              "beginner": "Sort artifacts by when they were created"
            },
            "hotkey": "Ctrl+Shift+C"
          },
          "sortByUpdatedAt": {
            "label": {
              "normal": "Sort by update date",
              "beginner": "Sort artifacts by when they were last updated"
            },
            "hotkey": "Ctrl+Shift+U"
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showDesigns": {
              "label": {
                "normal": "Designs",
                "beginner": "Designs"
              }
            },
            "createDesign": {
              "label": {
                "normal": "Design",
                "beginner": "Create Design"
              },
              "manual": "manuals/semio/kit#designs",
              "tutorial": "hello-semio/model-design"
            },
            "showTypes": {
              "label": {
                "normal": "Types",
                "beginner": "Types"
              }
            },
            "createType": {
              "label": {
                "normal": "Type",
                "beginner": "Create Type"
              },
              "manual": "manuals/semio/kit#types",
              "tutorial": "hello-semio/model-brick-set"
            },
            "showQualities": {
              "label": {
                "normal": "Qualities",
                "beginner": "Qualities"
              }
            },
            "createQuality": {
              "label": {
                "normal": "Quality",
                "beginner": "Create Quality"
              },
              "manual": "manuals/semio/kit#qualities",
              "tutorial": "getting-started/intro#quality"
            },
            "showPorts": {
              "label": {
                "normal": "Ports",
                "beginner": "Ports"
              }
            },
            "createPort": {
              "label": {
                "normal": "Port",
                "beginner": "Create Port"
              },
              "manual": "manuals/semio/kit#ports",
              "tutorial": "getting-started/intro#port"
            },
            "showFiles": {
              "label": {
                "normal": "Files",
                "beginner": "Files"
              }
            },
            "createFile": {
              "label": {
                "normal": "Create File",
                "beginner": "Click to add a new file to this kit"
              },
              "manual": "manuals/semio/kit#files",
              "tutorial": "getting-started/intro#files"
            },
            "showFolders": {
              "label": {
                "normal": "Folders",
                "beginner": "Folders"
              }
            },
            "createFolder": {
              "label": {
                "normal": "Folder",
                "beginner": "Create Folder"
              },
              "manual": "manuals/semio/kit#folders",
              "tutorial": "hello-semio/model-design"
            },
            "reset": {
              "label": {
                "normal": "Reset",
                "beginner": "Click to reset the kit to its original state"
              }
            },
            "showAuthors": {
              "label": {
                "normal": "Authors",
                "beginner": "Authors"
              }
            },
            "createAuthor": {
              "label": {
                "normal": "Create Author",
                "beginner": "Click to add a new author to this kit"
              },
              "manual": "manuals/semio/kit#authors",
              "tutorial": "getting-started/intro#authors"
            },
            "hideKind": {
              "label": {
                "normal": "Hide",
                "beginner": "Click to hide this artifact category"
              }
            },
            "createArtifact": {
              "label": {
                "normal": "Create",
                "beginner": "Click to create a new artifact of this type"
              }
            },
            "createChild": {
              "label": {
                "normal": "Create Child",
                "beginner": "Create a child element"
              }
            },
            "showTags": {
              "label": {
                "normal": "Tags",
                "beginner": "Tags"
              }
            },
            "showConcepts": {
              "label": {
                "normal": "Concepts",
                "beginner": "Concepts"
              }
            },
            "selection": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection tools"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter artifacts by type"
              }
            },
            "create": {
              "label": {
                "normal": "Create",
                "beginner": "Create new artifacts"
              }
            },
            "resetFilters": {
              "label": {
                "normal": "Reset Filters",
                "beginner": "Clear active artifact filters and show all artifact kinds"
              }
            },
            "createTag": {
              "label": {
                "normal": "Tag",
                "beginner": "Create Tag"
              }
            },
            "createConcept": {
              "label": {
                "normal": "Concept",
                "beginner": "Create Concept"
              }
            }
          },
          "canvas": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "table": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "search": {
                "label": {
                  "normal": "Search table",
                  "beginner": "Search for artifacts in the table"
                },
                "hotkey": "Ctrl+F"
              },
              "header": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kind": {
                  "label": {
                    "normal": "Kind",
                    "beginner": "Type of artifact"
                  }
                },
                "artifact": {
                  "label": {
                    "normal": "Name",
                    "beginner": "The name"
                  }
                },
                "updatedAt": {
                  "label": {
                    "normal": "Updated",
                    "beginner": "Last update time"
                  }
                },
                "createdAt": {
                  "label": {
                    "normal": "Created",
                    "beginner": "Creation time"
                  }
                }
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "A force-directed graph showing all kit artifacts and their relationships"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "kit": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Kit name",
                      "beginner": "The name of the kit. This is the primary identifier for your kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "version": {
                    "label": {
                      "normal": "Version",
                      "beginner": "The version of the kit in semantic versioning format (e.g., 1.0.0)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A detailed description of what this kit contains and how it should be used."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "icon": {
                    "label": {
                      "normal": "Icon",
                      "beginner": "An icon to represent this kit. Can be an emoji or URL to an image."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "image": {
                    "label": {
                      "normal": "Image",
                      "beginner": "URL to a preview image that showcases this kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "homepage": {
                    "label": {
                      "normal": "Homepage",
                      "beginner": "URL to the homepage or documentation for this kit."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "license": {
                    "label": {
                      "normal": "License",
                      "beginner": "The license under which this kit is distributed (e.g., MIT, GPL)."
                    },
                    "manual": "kit#metadata",
                    "tutorial": "hello-semio/save-kit"
                  }
                },
                "folder": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The display name of the folder."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-semio/save-kit"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Optional description that explains the purpose of this folder."
                    },
                    "manual": "kit#folders",
                    "tutorial": "hello-semio/save-kit"
                  }
                },
                "port": {
                  "compatible": {
                    "label": {
                      "normal": "Compatible",
                      "beginner": "Compatible"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Description"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  }
                },
                "tag": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Description"
                    }
                  }
                },
                "concept": {
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Name"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "Description"
                    }
                  }
                }
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Kit settings"
            },
            "diagram": {
              "chargeStrength": {
                "label": {
                  "normal": "Charge Strength",
                  "beginner": "Repulsion force between nodes"
                }
              },
              "linkDistance": {
                "label": {
                  "normal": "Link Distance",
                  "beginner": "Target distance between connected nodes"
                }
              },
              "collideRadius": {
                "label": {
                  "normal": "Collide Radius",
                  "beginner": "Collision radius preventing node overlap"
                }
              },
              "centerStrength": {
                "label": {
                  "normal": "Center Strength",
                  "beginner": "Force pulling nodes toward center"
                }
              }
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Color theme"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Port language"
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Input device type"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "User expertise level"
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "User or developer mode"
              }
            }
          },
          "port": {
            "allCompatible": {
              "label": {
                "normal": "All Compatible",
                "beginner": "All Compatible"
              }
            },
            "compatiblePorts": {
              "label": {
                "normal": "Compatible Ports",
                "beginner": "Compatible Ports"
              }
            },
            "descriptionPlaceholder": {
              "label": {
                "label": {
                  "normal": "Label",
                  "beginner": "Label"
                }
              }
            }
          },
          "ports": {
            "multipleSelected": {
              "label": {
                "normal": "Multiple Selected",
                "beginner": "Multiple Selected"
              }
            },
            "multipleTitle": {
              "label": {
                "normal": "{{count}} ports",
                "beginner": "{{count}} ports selected"
              }
            }
          },
          "qualities": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "files": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "authors": {
            "multipleTitle": {
              "label": {
                "normal": "Multiple Title",
                "beginner": "Multiple Title"
              }
            }
          },
          "tag": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Describe this tag...",
                "beginner": "Describe this tag..."
              }
            }
          },
          "tags": {
            "multipleSelected": {
              "label": {
                "normal": "Multiple tags selected",
                "beginner": "Multiple tags selected"
              }
            },
            "multipleTitle": "{{count}} tags"
          },
          "concept": {
            "descriptionPlaceholder": {
              "label": {
                "normal": "Describe this concept...",
                "beginner": "Describe this concept..."
              }
            }
          },
          "concepts": {
            "multipleSelected": {
              "label": {
                "normal": "Multiple concepts selected",
                "beginner": "Multiple concepts selected"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Title",
              "beginner": "Title"
            }
          },
          "tools": {
            "label": {
              "normal": "Tools",
              "beginner": "Tools"
            },
            "select": {
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additive",
                    "beginner": "Additive selection mode - add to existing selection"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtractive",
                    "beginner": "Subtractive selection mode - remove from existing selection"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Intersect",
                    "beginner": "Intersect selection mode - select only overlapping items"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rectangular",
                    "beginner": "Rectangular selection - drag to select in a rectangle"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freeform lasso selection - draw a freeform shape"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand tool - pan and navigate the canvas"
                  }
                }
              }
            }
          },
          "tool": {
            "pointer": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection"
              }
            },
            "hand": {
              "label": {
                "normal": "Hand",
                "beginner": "Hand"
              }
            }
          }
        },
        "port": {
          "label": {
            "normal": "Port",
            "beginner": "Port"
          },
          "defaultName": {
            "label": {
              "normal": "New Port",
              "beginner": "New Port"
            }
          }
        },
        "tag": {
          "label": {
            "normal": "Tag",
            "beginner": "Tag"
          },
          "defaultName": {
            "label": {
              "normal": "New Tag",
              "beginner": "New Tag"
            }
          }
        },
        "concept": {
          "label": {
            "normal": "Concept",
            "beginner": "Concept"
          },
          "defaultName": {
            "label": {
              "normal": "New Concept",
              "beginner": "New Concept"
            }
          }
        },
        "folder": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "defaultName": {
            "label": {
              "normal": "New Folder",
              "beginner": "New Folder"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this folder...",
                "beginner": "Describe this folder..."
              }
            }
          }
        },
        "design": {
          "label": {
            "normal": "Design",
            "beginner": "Design"
          },
          "properties": {
            "label": {
              "normal": "Design Properties",
              "beginner": "Design properties"
            }
          },
          "console": {
            "label": {
              "normal": "Console",
              "beginner": "Console"
            },
            "empty": {
              "label": {
                "normal": "Console output will appear here.",
                "beginner": "Console output will appear here."
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Design",
              "beginner": "New Design"
            }
          },
          "windowLibrary": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "scene": {
              "label": {
                "normal": "Scene Windows",
                "beginner": "Scene Windows"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram Windows",
                "beginner": "Diagram Windows"
              }
            },
            "table": {
              "label": {
                "normal": "Table Windows",
                "beginner": "Table Windows"
              }
            }
          },
          "diagram": {
            "clusterMenu": {
              "cluster": {
                "label": {
                  "normal": "Cluster",
                  "beginner": "Group selected design pieces into a single cluster"
                }
              }
            },
            "expandMenu": {
              "expand": {
                "label": {
                  "normal": "Expand",
                  "beginner": "Expand the design piece to show its internal components"
                }
              }
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "normal": "Describe this design...",
              "beginner": "Describe this design..."
            }
          },
          "iconPlaceholder": {
            "label": {
              "normal": "???",
              "beginner": "???"
            }
          },
          "imagePlaceholder": {
            "label": {
              "normal": "https://example.com/image.png",
              "beginner": "https://example.com/image.png"
            }
          },
          "variantPlaceholder": {
            "label": {
              "normal": "e.g., small, medium, large",
              "beginner": "e.g., small, medium, large"
            }
          },
          "viewPlaceholder": {
            "label": {
              "normal": "e.g., front, side, top",
              "beginner": "e.g., front, side, top"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "Name"
            }
          },
          "variant": {
            "label": {
              "normal": "Variant",
              "beginner": "Variant"
            }
          },
          "view": {
            "label": {
              "normal": "View",
              "beginner": "View"
            }
          },
          "location": {
            "label": {
              "normal": "Location",
              "beginner": "Location"
            }
          },
          "authors": {
            "label": {
              "normal": "Authors",
              "beginner": "Authors"
            }
          },
          "author": {
            "label": {
              "normal": "Author",
              "beginner": "Author"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attributes",
              "beginner": "Attributes"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "normal": "Value...",
              "beginner": "Value..."
            }
          },
          "attributeUnitPlaceholder": {
            "label": {
              "normal": "Unit...",
              "beginner": "Unit..."
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "normal": "Definition or URL...",
              "beginner": "Definition or URL..."
            }
          },
          "piece": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "id": {
              "label": {
                "normal": "ID",
                "beginner": "ID"
              }
            },
            "type": {
              "label": {
                "normal": "Type",
                "beginner": "Type"
              }
            },
            "center": {
              "label": {
                "normal": "Center",
                "beginner": "Center"
              }
            },
            "plane": {
              "label": {
                "normal": "Plane",
                "beginner": "Plane"
              }
            },
            "planeOrigin": {
              "label": {
                "normal": "Origin",
                "beginner": "Origin"
              }
            },
            "planeXAxis": {
              "label": {
                "normal": "X Axis",
                "beginner": "X Axis"
              }
            },
            "planeYAxis": {
              "label": {
                "normal": "Y Axis",
                "beginner": "Y Axis"
              }
            },
            "mixedSelectionMessage": {
              "label": {
                "normal": "Multiple pieces selected with different values",
                "beginner": "Multiple pieces selected with different values"
              }
            },
            "connectedPieceInfo": {
              "label": {
                "normal": "This piece is connected to another piece. Its position and orientation are computed from the connection. To make it independent, click 'Fix Piece'.",
                "beginner": "This piece is connected to another piece. Its position and orientation are computed from the connection. To make it independent, click 'Fix Piece'."
              }
            },
            "fixPiece": {
              "label": {
                "normal": "Fix Piece",
                "beginner": "Fix Piece"
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "design": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of the design. This is the primary identifier for your composition."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A detailed description of what this design represents and how it should be used."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "icon": {
                    "label": {
                      "normal": "Icon",
                      "beginner": "URL or path to an icon that represents this design in listings and previews."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "image": {
                    "label": {
                      "normal": "Image",
                      "beginner": "URL or path to a preview image that showcases this design."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "variant": {
                    "label": {
                      "normal": "Variant",
                      "beginner": "A variant identifier for different versions or configurations of this design."
                    },
                    "manual": "design#variants",
                    "tutorial": "hello-semio/model-design"
                  },
                  "view": {
                    "label": {
                      "normal": "View",
                      "beginner": "The viewing perspective or camera angle for displaying this design."
                    },
                    "manual": "design#views",
                    "tutorial": "hello-semio/model-design"
                  },
                  "unit": {
                    "label": {
                      "normal": "Unit",
                      "beginner": "The measurement unit used for all dimensions in this design (e.g., mm, cm, m)."
                    },
                    "manual": "design#metadata",
                    "tutorial": "hello-semio/model-design"
                  },
                  "createdAt": {
                    "label": {
                      "normal": "Created At",
                      "beginner": "The date and time when this design was first created."
                    }
                  },
                  "updatedAt": {
                    "label": {
                      "normal": "Updated At",
                      "beginner": "The date and time when this design was last modified."
                    }
                  },
                  "pieceCount": {
                    "label": {
                      "normal": "Pieces",
                      "beginner": "The total number of pieces in this design."
                    }
                  },
                  "connectionCount": {
                    "label": {
                      "normal": "Connections",
                      "beginner": "The total number of connections in this design."
                    }
                  }
                },
                "location": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "longitude": {
                    "label": {
                      "normal": "Longitude",
                      "beginner": "The east-west position of this design's location in decimal degrees."
                    },
                    "manual": "design#location"
                  },
                  "latitude": {
                    "label": {
                      "normal": "Latitude",
                      "beginner": "The north-south position of this design's location in decimal degrees."
                    },
                    "manual": "design#location"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The full name of the person who contributed to this design."
                    },
                    "manual": "design#authors"
                  },
                  "email": {
                    "label": {
                      "normal": "Email",
                      "beginner": "Contact email address for this author."
                    },
                    "manual": "design#authors"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The unique identifier for this attribute in kebab-case format (e.g., 'material.type')."
                    },
                    "manual": "design#attributes"
                  },
                  "value": {
                    "label": {
                      "normal": "Value",
                      "beginner": "The value associated with this attribute. Leave empty to use as a category flag."
                    },
                    "manual": "design#attributes"
                  },
                  "unit": {
                    "label": {
                      "normal": "Unit",
                      "beginner": "The measurement unit for this attribute's value (e.g., mm, kg, \u00b0C)."
                    },
                    "manual": "design#attributes"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "A URL or text that defines what this attribute means and how it should be interpreted."
                    },
                    "manual": "design#attributes"
                  }
                },
                "piece": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Piece",
                      "beginner": "Properties of the selected piece."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Pieces",
                      "beginner": "Properties of the selected pieces."
                    }
                  },
                  "pieceInfo": {
                    "label": {
                      "normal": "Piece",
                      "beginner": "Basic information about the piece."
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "An optional name for this piece to identify it within the design."
                    }
                  },
                  "namePlaceholder": {
                    "label": {
                      "normal": "Enter piece name...",
                      "beginner": "Enter piece name..."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of this piece's purpose or role in the design."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Enter piece description...",
                      "beginner": "Enter piece description..."
                    }
                  },
                  "scale": {
                    "label": {
                      "normal": "Scale",
                      "beginner": "The scaling factor applied to this piece. Default is 1.0."
                    }
                  },
                  "color": {
                    "label": {
                      "normal": "Color",
                      "beginner": "An optional color override for this piece (e.g., #FF0000)."
                    }
                  },
                  "colorPlaceholder": {
                    "label": {
                      "normal": "Enter color...",
                      "beginner": "Enter color..."
                    }
                  },
                  "attributes": {
                    "label": {
                      "normal": "Attributes",
                      "beginner": "Custom key-value attributes for this piece."
                    },
                    "name": {
                      "label": {
                        "normal": "Name",
                        "beginner": "The key identifier for this attribute."
                      }
                    },
                    "value": {
                      "label": {
                        "normal": "Value",
                        "beginner": "The value for this attribute."
                      }
                    },
                    "unit": {
                      "label": {
                        "normal": "Unit",
                        "beginner": "The measurement unit for this attribute."
                      }
                    },
                    "definition": {
                      "label": {
                        "normal": "Definition",
                        "beginner": "A URL or text defining this attribute."
                      }
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribute",
                      "beginner": "A custom attribute of this piece."
                    }
                  },
                  "center": {
                    "label": {
                      "normal": "Center",
                      "beginner": "The center position of the piece in the 2D diagram layout."
                    },
                    "manual": "design#diagram",
                    "tutorial": "metabolism/thinking-about-the-diagram",
                    "x": {
                      "label": {
                        "normal": "U",
                        "beginner": "U diagram coordinate of the center of the piece in 2D layout space."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    },
                    "y": {
                      "label": {
                        "normal": "V",
                        "beginner": "V diagram coordinate of the center of the piece in 2D layout space."
                      },
                      "manual": "design#diagram",
                      "tutorial": "metabolism/thinking-about-the-diagram"
                    }
                  },
                  "plane": {
                    "label": {
                      "normal": "Plane",
                      "beginner": "The 3D placement plane for this piece. Defines position and orientation in 3D space."
                    },
                    "manual": "design#pieces",
                    "tutorial": "hello-semio/model-design#pieces",
                    "origin": {
                      "label": {
                        "normal": "Origin",
                        "beginner": "Origin"
                      },
                      "x": {
                        "label": {
                          "normal": "Origin X",
                          "beginner": "Origin X"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Origin Y",
                          "beginner": "Origin Y"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Origin Z",
                          "beginner": "Origin Z"
                        }
                      }
                    },
                    "xaxis": {
                      "label": {
                        "normal": "X Axis",
                        "beginner": "X Axis"
                      },
                      "x": {
                        "label": {
                          "normal": "Axis X",
                          "beginner": "Axis X"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Axis Y",
                          "beginner": "Axis Y"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Axis Z",
                          "beginner": "Axis Z"
                        }
                      }
                    },
                    "yaxis": {
                      "label": {
                        "normal": "Y Axis",
                        "beginner": "Y Axis"
                      },
                      "x": {
                        "label": {
                          "normal": "Axis X",
                          "beginner": "Axis X"
                        }
                      },
                      "y": {
                        "label": {
                          "normal": "Axis Y",
                          "beginner": "Axis Y"
                        }
                      },
                      "z": {
                        "label": {
                          "normal": "Axis Z",
                          "beginner": "Axis Z"
                        }
                      }
                    }
                  }
                },
                "connection": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "properties": {
                    "label": {
                      "normal": "Connection",
                      "beginner": "Properties of the selected connection."
                    }
                  },
                  "multipleTitle": {
                    "label": {
                      "normal": "Connections",
                      "beginner": "Properties of the selected connections."
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of this connection's purpose."
                    }
                  },
                  "descriptionPlaceholder": {
                    "label": {
                      "normal": "Enter connection description...",
                      "beginner": "Enter connection description..."
                    }
                  },
                  "multipleEditing": {
                    "label": {
                      "normal": "Editing {{count}} connections simultaneously",
                      "beginner": "Editing {{count}} connections simultaneously"
                    }
                  },
                  "connecting": {
                    "label": {
                      "normal": "Connecting",
                      "beginner": "Connecting"
                    }
                  },
                  "connectingPieceId": {
                    "label": {
                      "normal": "Connecting Piece",
                      "beginner": "Connecting Piece"
                    }
                  },
                  "connectingPortId": {
                    "label": {
                      "normal": "Connecting Port",
                      "beginner": "Connecting Port"
                    }
                  },
                  "connectingDesignPieceId": {
                    "label": {
                      "normal": "Connecting Design Piece",
                      "beginner": "Connecting Design Piece"
                    }
                  },
                  "connected": {
                    "label": {
                      "normal": "Connected",
                      "beginner": "Connected"
                    }
                  },
                  "connectedPieceId": {
                    "label": {
                      "normal": "Connected Piece",
                      "beginner": "Connected Piece"
                    }
                  },
                  "connectedPortId": {
                    "label": {
                      "normal": "Connected Port",
                      "beginner": "Connected Port"
                    }
                  },
                  "connectedDesignPieceId": {
                    "label": {
                      "normal": "Connected Design Piece",
                      "beginner": "Connected Design Piece"
                    }
                  },
                  "gap": {
                    "label": {
                      "normal": "Gap",
                      "beginner": "Gap"
                    }
                  },
                  "shift": {
                    "label": {
                      "normal": "Shift",
                      "beginner": "Shift"
                    }
                  },
                  "rise": {
                    "label": {
                      "normal": "Rise",
                      "beginner": "Rise"
                    }
                  },
                  "rotation": {
                    "label": {
                      "normal": "Rotation",
                      "beginner": "Rotation"
                    }
                  },
                  "turn": {
                    "label": {
                      "normal": "Turn",
                      "beginner": "Turn"
                    }
                  },
                  "tilt": {
                    "label": {
                      "normal": "Tilt",
                      "beginner": "Tilt"
                    }
                  },
                  "x": {
                    "label": {
                      "normal": "Diagram X Offset",
                      "beginner": "Diagram X Offset"
                    }
                  },
                  "y": {
                    "label": {
                      "normal": "Diagram Y Offset",
                      "beginner": "Diagram Y Offset"
                    }
                  },
                  "u": {
                    "label": {
                      "normal": "X Offset",
                      "beginner": "X Offset"
                    }
                  },
                  "v": {
                    "label": {
                      "normal": "Y Offset",
                      "beginner": "Y Offset"
                    }
                  }
                },
                "connector": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "id": {
                    "label": {
                      "normal": "Connector ID",
                      "beginner": "The unique identifier of the connector"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of this connector."
                    }
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "The parameter t along the type's curve for this connector."
                    }
                  },
                  "position": {
                    "label": {
                      "normal": "Position",
                      "beginner": "The position of the connector"
                    }
                  },
                  "direction": {
                    "label": {
                      "normal": "Direction",
                      "beginner": "The direction vector of the connector"
                    }
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Mandatory",
                      "beginner": "Whether this connector must be connected"
                    }
                  },
                  "port": {
                    "label": {
                      "normal": "Port",
                      "beginner": "The connector port for compatibility checking"
                    }
                  },
                  "compatiblePort": {
                    "label": {
                      "normal": "Compatible Port",
                      "beginner": "The ports this connector is compatible with"
                    }
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of the connector"
                    }
                  },
                  "attribute": {
                    "label": {
                      "normal": "Attribute",
                      "beginner": "Custom attributes of the connector"
                    }
                  },
                  "notFound": {
                    "label": {
                      "normal": "Not Found",
                      "beginner": "Not Found"
                    }
                  }
                }
              },
              "parentConnection": {
                "label": {
                  "normal": "Parent Connection",
                  "beginner": "Parent Connection"
                }
              },
              "parentConnections": {
                "label": {
                  "normal": "Parent Connections",
                  "beginner": "Parent Connections"
                }
              }
            },
            "hud": {
              "overview": {
                "label": {
                  "normal": "HUD Overview",
                  "beginner": "HUD Overview"
                }
              },
              "selection": {
                "pieces": {
                  "label": {
                    "normal": "Selected Pieces",
                    "beginner": "Selected Pieces"
                  }
                },
                "connections": {
                  "label": {
                    "normal": "Selected Connections",
                    "beginner": "Selected Connections"
                  }
                },
                "connector": {
                  "label": {
                    "normal": "Connector Selected",
                    "beginner": "Connector Selected"
                  }
                }
              }
            },
            "stats": {
              "overview": {
                "label": {
                  "normal": "Statistics",
                  "beginner": "Statistics"
                }
              },
              "pieces": {
                "label": {
                  "normal": "Total Pieces",
                  "beginner": "Total Pieces"
                }
              },
              "connections": {
                "label": {
                  "normal": "Total Connections",
                  "beginner": "Total Connections"
                }
              },
              "windows": {
                "label": {
                  "normal": "Window Layout Loaded",
                  "beginner": "Window Layout Loaded"
                }
              }
            },
            "workbench": {
              "types": {
                "addPiece": {
                  "label": {
                    "normal": "Add Piece",
                    "beginner": "Add a new piece of this type to the design"
                  }
                },
                "duplicateType": {
                  "label": {
                    "normal": "Duplicate Type",
                    "beginner": "Create a duplicate of this type"
                  }
                }
              },
              "designs": {
                "addPiece": {
                  "label": {
                    "normal": "Add Piece",
                    "beginner": "Add a new piece of this design to the current design"
                  }
                }
              }
            }
          },
          "gridSize": {
            "label": {
              "normal": "Grid Size",
              "beginner": "Grid Size"
            }
          },
          "proximityConnectDistance": {
            "label": {
              "normal": "Proximity Connect Distance",
              "beginner": "Proximity Connect Distance"
            }
          },
          "selectOnlyPiecesOrConnections": {
            "label": {
              "normal": "Select Only Pieces Or Connections",
              "beginner": "Select Only Pieces Or Connections"
            }
          },
          "connection": {
            "rotation": {
              "label": {
                "normal": "Rotation",
                "beginner": "Rotation"
              }
            },
            "tilt": {
              "label": {
                "normal": "Tilt",
                "beginner": "Tilt"
              }
            },
            "turn": {
              "label": {
                "normal": "Turn",
                "beginner": "Turn"
              }
            },
            "plane": {
              "label": {
                "normal": "Plane",
                "beginner": "Plane"
              }
            },
            "translation": {
              "label": {
                "normal": "Translation",
                "beginner": "Translation"
              }
            },
            "orientation": {
              "label": {
                "normal": "Orientation",
                "beginner": "Orientation"
              }
            },
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "Diagram"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Title",
              "beginner": "Title"
            }
          },
          "tools": {
            "label": {
              "normal": "Tools",
              "beginner": "Tools"
            },
            "select": {
              "label": {
                "normal": "Select",
                "beginner": "Selection tool"
              },
              "mode": {
                "additive": {
                  "label": {
                    "normal": "Additive",
                    "beginner": "Additive selection mode - add to existing selection"
                  }
                },
                "subtractive": {
                  "label": {
                    "normal": "Subtractive",
                    "beginner": "Subtractive selection mode - remove from existing selection"
                  }
                },
                "intersect": {
                  "label": {
                    "normal": "Intersect",
                    "beginner": "Intersect selection mode - select only overlapping items"
                  }
                }
              },
              "shape": {
                "rectangular": {
                  "label": {
                    "normal": "Rectangular",
                    "beginner": "Rectangular selection - drag to select in a rectangle"
                  }
                },
                "lasso": {
                  "label": {
                    "normal": "Lasso",
                    "beginner": "Freeform lasso selection - draw a freeform shape"
                  }
                }
              },
              "navigation": {
                "hand": {
                  "label": {
                    "normal": "Hand",
                    "beginner": "Hand tool - pan and navigate the canvas"
                  }
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rectangular lasso",
                  "beginner": "Drag a rectangle for lasso selection"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freeform lasso",
                  "beginner": "Draw a freehand lasso path"
                }
              }
            }
          },
          "windows": {
            "label": {
              "normal": "Windows",
              "beginner": "Windows"
            }
          },
          "appTitle": {
            "label": {
              "normal": "App Title",
              "beginner": "App Title"
            }
          },
          "canvas": {
            "diagram": {
              "label": {
                "normal": "Diagram",
                "beginner": "Diagram"
              },
              "pieceNode": {
                "label": {
                  "normal": "Piece",
                  "beginner": "Diagram piece node"
                }
              }
            },
            "label": {
              "normal": "Canvas",
              "beginner": "Canvas"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showPieces": {
              "label": {
                "normal": "Pieces",
                "beginner": "Toggle visibility of design pieces"
              }
            },
            "showConnections": {
              "label": {
                "normal": "Connections",
                "beginner": "Toggle visibility of connections between pieces"
              }
            },
            "showPorts": {
              "label": {
                "normal": "Ports",
                "beginner": "Toggle visibility of ports on pieces"
              }
            },
            "addPiece": {
              "label": {
                "normal": "Add Piece",
                "beginner": "Add a new piece to the design"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter design elements by visibility"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Design settings"
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Choose the color scheme for the application"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Choose the language for the application interface"
              },
              "placeholder": {
                "label": {
                  "normal": "Select language...",
                  "beginner": "Select the language in which the application is displayed"
                }
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Choose the input device"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Choose your experience level"
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "Choose the user interface mode"
              }
            },
            "panel": {
              "label": {
                "normal": "Panels",
                "beginner": "Configure panel visibility"
              },
              "toolbar": {
                "label": {
                  "normal": "Show Toolbar",
                  "beginner": "Toggle the toolbar panel"
                }
              },
              "workbench": {
                "label": {
                  "normal": "Show Workbench",
                  "beginner": "Toggle the workbench panel"
                }
              },
              "windows": {
                "label": {
                  "normal": "Show Windows",
                  "beginner": "Toggle the windows panel"
                }
              },
              "details": {
                "label": {
                  "normal": "Show Details",
                  "beginner": "Toggle the details panel"
                }
              }
            }
          }
        },
        "type": {
          "label": {
            "normal": "Type",
            "beginner": "Type"
          },
          "properties": {
            "label": {
              "normal": "Type Properties",
              "beginner": "Type properties"
            }
          },
          "toolbar": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "showConnectors": {
              "label": {
                "normal": "Connectors",
                "beginner": "Connectors"
              }
            },
            "showModels": {
              "label": {
                "normal": "Models",
                "beginner": "Models"
              }
            },
            "filters": {
              "label": {
                "normal": "Filters",
                "beginner": "Filter type elements by visibility"
              }
            }
          },
          "footer": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "someAction": {
              "label": {
                "normal": "Some Action",
                "beginner": "Some Action"
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Type",
              "beginner": "New Type"
            }
          },
          "descriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this type...",
                "beginner": "Describe this type..."
              }
            }
          },
          "iconPlaceholder": {
            "label": {
              "label": {
                "normal": "??",
                "beginner": "??"
              }
            }
          },
          "imagePlaceholder": {
            "label": {
              "label": {
                "normal": "https://example.com/image.png",
                "beginner": "https://example.com/image.png"
              }
            }
          },
          "variantPlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., large, small",
                "beginner": "e.g., large, small"
              }
            }
          },
          "parentPlaceholder": {
            "label": {
              "label": {
                "normal": "Select parent type...",
                "beginner": "Select parent type..."
              }
            }
          },
          "variant": {
            "label": {
              "normal": "Variant",
              "beginner": "Variant"
            }
          },
          "models": {
            "label": {
              "normal": "Models",
              "beginner": "Manage different 3D models, images, and visual models for this type."
            },
            "manual": "type#models",
            "tutorial": "hello-semio/model-brick-set#models"
          },
          "model": {
            "label": {
              "normal": "Model",
              "beginner": "Model"
            }
          },
          "modelDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this model...",
                "beginner": "Describe this model..."
              }
            }
          },
          "modelTagsPlaceholder": {
            "label": {
              "label": {
                "normal": "tag1, tag2, tag3",
                "beginner": "tag1, tag2, tag3"
              }
            }
          },
          "connectors": {
            "label": {
              "normal": "Connectors",
              "beginner": "Manage connection connectors for this type. Connectors define where and how pieces can connect."
            },
            "manual": "type#connectors",
            "tutorial": "hello-semio/model-brick-set#connectors"
          },
          "connector": {
            "label": {
              "normal": "Connector",
              "beginner": "Connector"
            },
            "properties": {
              "label": {
                "normal": "Connector Properties",
                "beginner": "Connector properties"
              }
            },
            "title": {
              "label": {
                "normal": "Title",
                "beginner": "Title"
              }
            }
          },
          "connectorPortPlaceholder": {
            "label": {
              "label": {
                "normal": "e.g., electrical, mechanical",
                "beginner": "e.g., electrical, mechanical"
              }
            }
          },
          "connectorNamePlaceholder": {
            "label": {
              "label": {
                "normal": "Add a name",
                "beginner": "Add a name"
              }
            }
          },
          "connectorDescriptionPlaceholder": {
            "label": {
              "label": {
                "normal": "Describe this connector...",
                "beginner": "Describe this connector..."
              }
            }
          },
          "connectorPoint": {
            "label": {
              "normal": "Point",
              "beginner": "The 3D position of the connector in local coordinates."
            },
            "manual": "type#connectors",
            "tutorial": "hello-semio/model-brick-set#connectors"
          },
          "connectorDirection": {
            "label": {
              "normal": "Direction",
              "beginner": "The outward direction vector of the connector in local coordinates."
            },
            "manual": "type#connectors",
            "tutorial": "hello-semio/model-brick-set#connectors"
          },
          "connectorCompatiblePortsPlaceholder": {
            "label": {
              "label": {
                "normal": "port1, port2",
                "beginner": "port1, port2"
              }
            }
          },
          "connectorNotFound": {
            "label": {
              "normal": "Connector not found",
              "beginner": "Connector not found"
            }
          },
          "connectorsNotFound": {
            "label": {
              "normal": "No connectors found",
              "beginner": "No connectors found"
            }
          },
          "authors": {
            "label": {
              "normal": "Authors",
              "beginner": "Authors"
            }
          },
          "author": {
            "label": {
              "normal": "Author",
              "beginner": "Author"
            }
          },
          "attributes": {
            "label": {
              "normal": "Attributes",
              "beginner": "Attributes"
            }
          },
          "attribute": {
            "label": {
              "normal": "Attribute",
              "beginner": "Attribute"
            }
          },
          "attributeValuePlaceholder": {
            "label": {
              "label": {
                "normal": "Value...",
                "beginner": "Value..."
              }
            }
          },
          "attributeDefinitionPlaceholder": {
            "label": {
              "label": {
                "normal": "Definition or URL...",
                "beginner": "Definition or URL..."
              }
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "section": {
                "label": {
                  "normal": "",
                  "beginner": ""
                },
                "type": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of the type. This is the primary identifier for the component."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A detailed description of what this type represents and how it should be used."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "icon": {
                    "label": {
                      "normal": "Icon",
                      "beginner": "An icon to visually represent this type. Can be an emoji, icon name, or URL to an image."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "image": {
                    "label": {
                      "normal": "Image",
                      "beginner": "URL to an image that represents this type. Used for previews and visual identification."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "unit": {
                    "label": {
                      "normal": "Unit",
                      "beginner": "The unit of measurement used for this type (e.g., mm, m, ft)."
                    },
                    "manual": "type#metadata",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "parent": {
                    "label": {
                      "normal": "Parent",
                      "beginner": "The parent type this type inherits from"
                    }
                  },
                  "abstract": {
                    "label": {
                      "normal": "Abstract",
                      "beginner": "Whether this is an abstract type"
                    }
                  }
                },
                "models": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "url": {
                    "label": {
                      "normal": "URL",
                      "beginner": "URL to a 3D model, image, or other resource representing this type."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of what this model shows or how it should be used."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "tags": {
                    "label": {
                      "normal": "Tags",
                      "beginner": "Tags to categorize and filter models (e.g., 'detailed', 'simplified', 'lod1')."
                    },
                    "manual": "type#models",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                },
                "connectors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "port": {
                    "label": {
                      "normal": "Port",
                      "beginner": "Connector port name. Connectors of the same port can connect to each other."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "compatiblePorts": {
                    "label": {
                      "normal": "Compatible Ports",
                      "beginner": "List of other connector ports this connector can connect to. Leave empty to allow all ports."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "description": {
                    "label": {
                      "normal": "Description",
                      "beginner": "A description of what this connector represents and how it should be used for connections."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "t": {
                    "label": {
                      "normal": "T",
                      "beginner": "Position on the diagram ring (0-1). Controls where the connector appears in the 2D diagram view."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position on the diagram ring (0-1). Controls where the connector appears in the 2D diagram view."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "direction": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X coordinate of the connector direction vector. This defines which direction the connector points in 3D space."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y coordinate of the connector direction vector. This defines which direction the connector points in 3D space."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z coordinate of the connector direction vector. This defines which direction the connector points in 3D space."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    }
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "An optional name for this connector."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "mandatory": {
                    "label": {
                      "normal": "Mandatory",
                      "beginner": "Whether this connector is required for a valid connection."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "maxChildren": {
                    "label": {
                      "normal": "Max Children",
                      "beginner": "The maximum number of connections allowed at this connector."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "point": {
                    "label": {
                      "normal": "",
                      "beginner": ""
                    },
                    "x": {
                      "label": {
                        "normal": "X",
                        "beginner": "X position of the connector in 3D space relative to the type's origin."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "y": {
                      "label": {
                        "normal": "Y",
                        "beginner": "Y position of the connector in 3D space relative to the type's origin."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    },
                    "z": {
                      "label": {
                        "normal": "Z",
                        "beginner": "Z position of the connector in 3D space relative to the type's origin."
                      },
                      "manual": "type#connectors",
                      "tutorial": "hello-semio/model-brick-set"
                    }
                  }
                },
                "connector": {
                  "ring": {
                    "label": {
                      "normal": "Ring",
                      "beginner": "Position on the diagram ring (0-1). Controls where the connector appears in the 2D diagram view."
                    },
                    "manual": "type#connectors",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                },
                "attributes": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "The name of the attribute in kebab-case (e.g., 'material.wood', 'cost.labor')."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "value": {
                    "label": {
                      "normal": "Value",
                      "beginner": "The value of the attribute. Leave empty for boolean attributes (presence = true)."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "definition": {
                    "label": {
                      "normal": "Definition",
                      "beginner": "Optional definition or documentation for this attribute. Can be text or a URL."
                    },
                    "manual": "type#attributes",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                },
                "authors": {
                  "label": {
                    "normal": "",
                    "beginner": ""
                  },
                  "name": {
                    "label": {
                      "normal": "Name",
                      "beginner": "Full name of the author or contributor."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-semio/model-brick-set"
                  },
                  "email": {
                    "label": {
                      "normal": "Email",
                      "beginner": "Email address for contacting the author."
                    },
                    "manual": "type#authors",
                    "tutorial": "hello-semio/model-brick-set"
                  }
                }
              }
            }
          },
          "tools": {
            "label": {
              "normal": "Tools",
              "beginner": "Tools"
            },
            "select": {
              "normal": {
                "label": {
                  "normal": "Normal",
                  "beginner": "Normal selection"
                }
              },
              "additive": {
                "label": {
                  "normal": "Additive",
                  "beginner": "Additive selection - add to existing selection"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtractive",
                  "beginner": "Subtractive selection - remove from existing selection"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Intersect",
                  "beginner": "Intersect selection - select only overlapping items"
                }
              }
            },
            "lasso": {
              "rectangular": {
                "label": {
                  "normal": "Rectangular",
                  "beginner": "Rectangular lasso selection"
                }
              },
              "freeform": {
                "label": {
                  "normal": "Freeform",
                  "beginner": "Freeform lasso selection"
                }
              }
            },
            "selection": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection tool"
              }
            },
            "hand": {
              "label": {
                "normal": "Hand",
                "beginner": "Hand tool - pan and navigate"
              }
            },
            "connector": {
              "label": {
                "normal": "Connector",
                "beginner": "Connector creation tool"
              }
            }
          },
          "title": {
            "label": {
              "normal": "Title",
              "beginner": "Title"
            }
          }
        },
        "quality": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "description": {
            "label": {
              "normal": "Define measurement qualities",
              "beginner": "Define measurement qualities"
            }
          },
          "title": {
            "label": {
              "normal": "Quality",
              "beginner": "Quality"
            }
          },
          "toolbar": {
            "view": {
              "label": {
                "normal": "View",
                "beginner": "View options"
              }
            },
            "actions": {
              "label": {
                "normal": "Actions",
                "beginner": "Quality actions"
              }
            }
          },
          "workbench": {
            "nodes": {
              "label": {
                "normal": "Nodes",
                "beginner": "Quality formula nodes"
              }
            },
            "qualities": {
              "label": {
                "normal": "Qualities",
                "beginner": "Available qualities"
              }
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Quality settings"
            }
          },
          "chat": {
            "label": {
              "normal": "Chat",
              "beginner": "Quality chat"
            }
          },
          "tools": {
            "selection": {
              "label": {
                "normal": "Selection",
                "beginner": "Selection tool"
              }
            },
            "select": {
              "additive": {
                "label": {
                  "normal": "Additive",
                  "beginner": "Additive selection - add to existing selection"
                }
              },
              "subtractive": {
                "label": {
                  "normal": "Subtractive",
                  "beginner": "Subtractive selection - remove from existing selection"
                }
              },
              "intersect": {
                "label": {
                  "normal": "Intersect",
                  "beginner": "Intersect selection - select only overlapping items"
                }
              }
            }
          },
          "defaultName": {
            "label": {
              "normal": "New Quality",
              "beginner": "New Quality"
            }
          },
          "numericFunctions": {
            "label": {
              "normal": "Numeric Functions",
              "beginner": "Numeric Functions"
            }
          },
          "add": {
            "label": {
              "normal": "Add",
              "beginner": "Add"
            }
          },
          "subtract": {
            "label": {
              "normal": "Subtract",
              "beginner": "Subtract"
            }
          },
          "multiply": {
            "label": {
              "normal": "Multiply",
              "beginner": "Multiply"
            }
          },
          "divide": {
            "label": {
              "normal": "Divide",
              "beginner": "Divide"
            }
          },
          "branchingFunctions": {
            "label": {
              "normal": "Branching Functions",
              "beginner": "Branching Functions"
            }
          },
          "if": {
            "label": {
              "normal": "If",
              "beginner": "If"
            }
          },
          "switch": {
            "label": {
              "normal": "Switch",
              "beginner": "Switch"
            }
          },
          "dataStructures": {
            "label": {
              "normal": "Data Structures",
              "beginner": "Data Structures"
            }
          },
          "list": {
            "label": {
              "normal": "List",
              "beginner": "List"
            }
          },
          "dictionary": {
            "label": {
              "normal": "Dictionary",
              "beginner": "Dictionary"
            }
          },
          "noQualities": {
            "label": {
              "normal": "No qualities defined",
              "beginner": "No qualities defined"
            }
          },
          "key": {
            "label": {
              "normal": "Key",
              "beginner": "The unique identifier for this quality"
            }
          },
          "name": {
            "label": {
              "normal": "Name",
              "beginner": "The display name for this quality"
            }
          },
          "kind": {
            "label": {
              "normal": "Kind",
              "beginner": "The type of entity this quality applies to"
            }
          },
          "formula": {
            "label": {
              "normal": "Formula",
              "beginner": "The formula to calculate this quality"
            }
          },
          "formulaPlaceholder": "Enter formula...",
          "defaultValue": {
            "label": {
              "normal": "Default value",
              "beginner": "The default value for this quality"
            }
          },
          "defaultSiUnit": {
            "label": {
              "normal": "Default SI unit",
              "beginner": "The default unit in the SI system"
            }
          },
          "defaultImperialUnit": {
            "label": {
              "normal": "Default Imperial unit",
              "beginner": "The default unit in the Imperial system"
            }
          },
          "min": {
            "label": {
              "normal": "Minimum",
              "beginner": "The minimum allowed value"
            }
          },
          "isMinExcluded": {
            "label": {
              "normal": "Exclude minimum",
              "beginner": "Whether the minimum value is excluded"
            }
          },
          "max": {
            "label": {
              "normal": "Maximum",
              "beginner": "The maximum allowed value"
            }
          },
          "isMaxExcluded": {
            "label": {
              "normal": "Exclude maximum",
              "beginner": "Whether the maximum value is excluded"
            }
          },
          "canScale": {
            "label": {
              "normal": "Can scale",
              "beginner": "Whether this quality scales with piece size"
            }
          },
          "panel": {
            "label": {
              "normal": "",
              "beginner": ""
            },
            "details": {
              "label": {
                "normal": "",
                "beginner": ""
              },
              "key": {
                "label": {
                  "normal": "Key",
                  "beginner": "The unique key identifier of the quality"
                }
              },
              "name": {
                "label": {
                  "normal": "Name",
                  "beginner": "The display name of the quality"
                }
              },
              "description": {
                "label": {
                  "normal": "Description",
                  "beginner": "A description of what this quality measures"
                }
              },
              "formula": {
                "label": {
                  "normal": "Formula",
                  "beginner": "The formula to calculate this quality"
                }
              },
              "defaultSiUnit": {
                "label": {
                  "normal": "Default SI Unit",
                  "beginner": "The default unit in the SI system"
                }
              },
              "defaultImperialUnit": {
                "label": {
                  "normal": "Default Imperial Unit",
                  "beginner": "The default unit in the Imperial system"
                }
              },
              "kind": {
                "label": {
                  "normal": "Kind",
                  "beginner": "The type of entity this quality applies to"
                }
              },
              "canScale": {
                "label": {
                  "normal": "Can Scale",
                  "beginner": "Whether this quality scales with piece size"
                }
              },
              "defaultValue": {
                "label": {
                  "normal": "Default Value",
                  "beginner": "The default value for this quality"
                }
              },
              "min": {
                "label": {
                  "normal": "Minimum",
                  "beginner": "The minimum allowed value"
                }
              },
              "max": {
                "label": {
                  "normal": "Maximum",
                  "beginner": "The maximum allowed value"
                }
              },
              "isMinExcluded": {
                "label": {
                  "normal": "Minimum Excluded",
                  "beginner": "Whether the minimum value is exclusive"
                }
              },
              "isMaxExcluded": {
                "label": {
                  "normal": "Maximum Excluded",
                  "beginner": "Whether the maximum value is exclusive"
                }
              }
            }
          },
          "functions": {
            "label": {
              "normal": "Functions",
              "beginner": "Functions"
            }
          },
          "qualities": {
            "label": {
              "normal": "Qualities",
              "beginner": "Qualities"
            }
          }
        },
        "docs": {
          "label": {
            "normal": "",
            "beginner": ""
          },
          "noHeadings": {
            "label": {
              "normal": "No headings found",
              "beginner": "No headings found"
            }
          },
          "docs": {
            "label": {
              "normal": "Docs",
              "beginner": "Docs"
            }
          },
          "overview": {
            "label": {
              "normal": "Overview",
              "beginner": "Overview"
            }
          },
          "page": {
            "label": {
              "normal": "Page",
              "beginner": "Page"
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Settings"
            },
            "theme": {
              "label": {
                "normal": "Theme",
                "beginner": "Choose the color theme"
              }
            },
            "language": {
              "label": {
                "normal": "Language",
                "beginner": "Select the language"
              }
            },
            "device": {
              "label": {
                "normal": "Device",
                "beginner": "Select the input device"
              }
            },
            "expertise": {
              "label": {
                "normal": "Expertise",
                "beginner": "Select your expertise level"
              }
            },
            "mode": {
              "label": {
                "normal": "Mode",
                "beginner": "Select user or developer mode"
              }
            }
          },
          "navigation": {
            "previous": {
              "label": {
                "normal": "Previous",
                "beginner": "Navigate to the previous page"
              }
            },
            "next": {
              "label": {
                "normal": "Next",
                "beginner": "Navigate to the next page"
              }
            }
          }
        },
        "feedback": {
          "label": {
            "normal": "Feedback",
            "beginner": "Send feedback to help improve Semio"
          },
          "title": {
            "label": {
              "normal": "Feedback",
              "beginner": "Feedback"
            }
          },
          "subtitle": {
            "label": {
              "normal": "Help us improve semio by reporting bugs or sharing ideas.",
              "beginner": "Help us improve semio by reporting bugs or sharing ideas."
            }
          },
          "form": {
            "kind": {
              "label": {
                "normal": "Type",
                "beginner": "Select bug report or feature idea"
              }
            },
            "title": {
              "label": {
                "normal": "Title",
                "beginner": "A brief summary of your feedback"
              }
            },
            "titlePlaceholder": {
              "label": {
                "normal": "Enter a brief title...",
                "beginner": "Enter a brief title..."
              }
            },
            "description": {
              "label": {
                "normal": "Description",
                "beginner": "Detailed description of the problem or idea"
              }
            },
            "bugDescriptionPlaceholder": {
              "label": {
                "normal": "Describe what happened...",
                "beginner": "Describe what happened..."
              }
            },
            "ideaDescriptionPlaceholder": {
              "label": {
                "normal": "Describe your idea...",
                "beginner": "Describe your idea..."
              }
            },
            "app": {
              "label": {
                "normal": "App",
                "beginner": "Which app did the bug occur in?"
              }
            },
            "appPlaceholder": {
              "label": {
                "normal": "Select app...",
                "beginner": "Select app..."
              }
            },
            "name": {
              "label": {
                "normal": "Name",
                "beginner": "Your name (optional)"
              }
            },
            "namePlaceholder": {
              "label": {
                "normal": "Your name (optional)",
                "beginner": "Your name (optional)"
              }
            },
            "email": {
              "label": {
                "normal": "Email",
                "beginner": "Your email address (optional)"
              }
            },
            "emailPlaceholder": {
              "label": {
                "normal": "your@email.com (optional)",
                "beginner": "your@email.com (optional)"
              }
            },
            "submit": {
              "label": {
                "normal": "Submit Feedback",
                "beginner": "Submit your feedback"
              }
            },
            "submitting": {
              "label": {
                "normal": "Submitting...",
                "beginner": "Submitting..."
              }
            }
          },
          "kind": {
            "bug": {
              "label": {
                "normal": "Bug Report",
                "beginner": "Report a problem or error"
              }
            },
            "idea": {
              "label": {
                "normal": "Feature Idea",
                "beginner": "Suggest a new feature or improvement"
              }
            }
          },
          "appOption": {
            "home": {
              "label": {
                "normal": "Home",
                "beginner": "Home"
              }
            },
            "kit": {
              "label": {
                "normal": "Kit",
                "beginner": "Kit"
              }
            },
            "design": {
              "label": {
                "normal": "Design",
                "beginner": "Design"
              }
            },
            "type": {
              "label": {
                "normal": "Type",
                "beginner": "Type"
              }
            },
            "quality": {
              "label": {
                "normal": "Quality",
                "beginner": "Quality"
              }
            },
            "docs": {
              "label": {
                "normal": "Docs",
                "beginner": "Docs"
              }
            },
            "feedback": {
              "label": {
                "normal": "Feedback",
                "beginner": "Feedback"
              }
            }
          },
          "optional": {
            "label": {
              "normal": "Optional contact information",
              "beginner": "Optional contact information"
            }
          },
          "success": {
            "thankYou": {
              "label": {
                "normal": "Thank You!",
                "beginner": "Thank You!"
              }
            },
            "message": {
              "label": {
                "normal": "Your feedback has been received. We appreciate your contribution!",
                "beginner": "Your feedback has been received. We appreciate your contribution!"
              }
            },
            "sendAnother": {
              "label": {
                "normal": "Send Another",
                "beginner": "Submit another feedback"
              }
            },
            "goHome": {
              "label": {
                "normal": "Go Home",
                "beginner": "Return to home page"
              }
            }
          },
          "error": {
            "titleRequired": {
              "label": {
                "normal": "Title is required",
                "beginner": "Title is required"
              }
            },
            "descriptionRequired": {
              "label": {
                "normal": "Description is required",
                "beginner": "Description is required"
              }
            },
            "appRequired": {
              "label": {
                "normal": "Please select which app the bug occurred in",
                "beginner": "Please select which app the bug occurred in"
              }
            },
            "submitFailed": {
              "label": {
                "normal": "Failed to submit feedback. Please try again.",
                "beginner": "Failed to submit feedback. Please try again."
              }
            }
          },
          "toolbar": {
            "send": {
              "label": {
                "normal": "Send",
                "beginner": "Send feedback"
              }
            }
          }
        }
      },
      "docs": {
        "navigation": {
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "toolbar": {
        "label": {
          "normal": "Toolbar",
          "beginner": "Toolbar"
        },
        "group": {
          "hand": {
            "label": {
              "normal": "Hand",
              "beginner": "Hand"
            }
          },
          "selection": {
            "label": {
              "normal": "Selection",
              "beginner": "Selection"
            }
          },
          "lasso": {
            "label": {
              "normal": "Lasso",
              "beginner": "Lasso"
            }
          },
          "filter": {
            "label": {
              "normal": "Filter",
              "beginner": "Filter"
            }
          },
          "open": {
            "label": {
              "normal": "Open",
              "beginner": "Open"
            }
          },
          "create": {
            "label": {
              "normal": "Create",
              "beginner": "Create"
            }
          },
          "view": {
            "label": {
              "normal": "View",
              "beginner": "View"
            }
          },
          "actions": {
            "label": {
              "normal": "Actions",
              "beginner": "Actions"
            }
          },
          "settings": {
            "label": {
              "normal": "Settings",
              "beginner": "Settings"
            }
          }
        },
        "parent": {
          "hand": "Hand",
          "selection": "Selection",
          "lasso": "Lasso",
          "filter": "Filter",
          "open": "Open",
          "create": "Create",
          "view": "View",
          "actions": "Actions",
          "settings": "Settings"
        }
      },
      "tutorial": {
        "controls": {
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          },
          "previous": {
            "label": {
              "normal": "Previous",
              "beginner": "Previous"
            }
          },
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "next": {
            "label": {
              "normal": "Next",
              "beginner": "Next"
            }
          }
        }
      },
      "recording": {
        "controls": {
          "playPause": {
            "label": {
              "normal": "Play Pause",
              "beginner": "Play Pause"
            }
          },
          "stop": {
            "label": {
              "normal": "Stop",
              "beginner": "Stop"
            }
          }
        }
      }
    }
  },
  "settings": {
    "layout": {
      "normal": {
        "label": {
          "normal": "Normal layout",
          "beginner": "Use the standard layout optimized for mouse and keyboard."
        }
      },
      "desktop": {
        "label": {
          "normal": "Desktop layout",
          "beginner": "Use the desktop layout optimized for large screens."
        }
      },
      "tablet": {
        "label": {
          "normal": "Tablet layout",
          "beginner": "Use the tablet layout optimized for medium screens."
        }
      },
      "mobile": {
        "label": {
          "normal": "Mobile layout",
          "beginner": "Use the mobile layout optimized for small screens."
        }
      }
    },
    "mode": {
      "dev": {
        "label": {
          "normal": "Developer mode",
          "beginner": "Show developer tools and advanced options."
        }
      },
      "user": {
        "label": {
          "normal": "User mode",
          "beginner": "Show standard user port."
        }
      },
      "beginner": {
        "label": {
          "normal": "Beginner mode",
          "beginner": "Show full guidance, tutorials, and detailed tooltips."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal mode",
          "beginner": "Show contextual help without tutorials."
        }
      }
    },
    "expertise": {
      "beginner": {
        "label": {
          "normal": "Beginner",
          "beginner": "Show full guidance and tutorials."
        }
      },
      "normal": {
        "label": {
          "normal": "Normal",
          "beginner": "Show contextual help."
        }
      },
      "expert": {
        "label": {
          "normal": "Expert",
          "beginner": "Hide guidance."
        }
      }
    }
  },
  "tooltip": {
    "manual": {
      "label": {
        "normal": "Manual",
        "beginner": "Manual"
      }
    },
    "tutorial": {
      "label": {
        "normal": "Tutorial",
        "beginner": "Tutorial"
      }
    }
  }
}
`),
  },
} as const;

type ElementUiLocaleCode = keyof typeof elementUiTranslationBundles;

function normalizeElementUiLocale(language?: string): ElementUiLocaleCode {
  return language?.toLowerCase().startsWith("de") ? "de" : "en";
}

function resolveRequestedElementUiLocale(): ElementUiLocaleCode {
  return normalizeElementUiLocale(i18next.resolvedLanguage || i18next.language || (typeof navigator !== "undefined" ? navigator.language : undefined));
}

function registerElementUiTranslationBundles() {
  for (const [language, resource] of Object.entries(elementUiTranslationBundles)) {
    if (!i18next.hasResourceBundle(language, "translation")) {
      i18next.addResourceBundle(language, "translation", resource.translation, true, true);
    }
  }
}

function initializeElementUiI18n() {
  const requestedLocale = resolveRequestedElementUiLocale();

  if (i18next.isInitialized) {
    registerElementUiTranslationBundles();
    if (i18next.language !== requestedLocale) {
      void i18next.changeLanguage(requestedLocale);
    }
    return i18next;
  }

  i18next.use(LanguageDetector).use(initReactI18next);

  void i18next.init({
    resources: elementUiTranslationBundles,
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: requestedLocale,
    showSupportNotice: false,
    returnObjects: true,
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
      bindI18n: "languageChanged",
      bindI18nStore: "added removed",
    },
  });

  return i18next;
}

export const elementUiI18n = initializeElementUiI18n();

// #endregion 🪁I18n Resources

/**
 * React hook that resolves a localized label by i18n key and expertise level.
 **/
export function useLabel(id: string): string | undefined {
  const { t } = useTranslation();
  const expertise = _expertiseProvider ? _expertiseProvider() : Expertise.NORMAL;
  const value = t(id as any) as any;

  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object" && "label" in value) {
    const label = value.label;

    if (typeof label === "string") {
      return label;
    }

    if (label && typeof label === "object") {
      if (expertise === Expertise.BEGINNER && "beginner" in label && label.beginner !== undefined) {
        return String(label.beginner);
      }
      if ("normal" in label && label.normal !== undefined) {
        return String(label.normal);
      }
      if ("beginner" in label && label.beginner !== undefined) {
        return String(label.beginner);
      }
    }
  }

  return undefined;
}

/**
 * Resolves a localized string from a raw translation value and expertise level.
 * Pure function (non-hook) variant of useLabel for use outside React render context.
 * Handles: string, {label: string}, {label: {normal, beginner}}, {normal, beginner}.
 **/
export function resolveTranslationLabel(value: unknown): string | undefined {
  const expertise = _expertiseProvider ? _expertiseProvider() : Expertise.NORMAL;

  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object") {
    const obj = value as Record<string, unknown>;

    if ("label" in obj) {
      const label = obj.label;

      if (typeof label === "string") {
        return label;
      }

      if (label && typeof label === "object") {
        const labelObj = label as Record<string, unknown>;
        if (expertise === Expertise.BEGINNER && "beginner" in labelObj && labelObj.beginner !== undefined) {
          return String(labelObj.beginner);
        }
        if ("normal" in labelObj && labelObj.normal !== undefined) {
          return String(labelObj.normal);
        }
        if ("beginner" in labelObj && labelObj.beginner !== undefined) {
          return String(labelObj.beginner);
        }
      }
    }

    if ("normal" in obj || "beginner" in obj) {
      if (expertise === Expertise.BEGINNER && "beginner" in obj && obj.beginner !== undefined) {
        return String(obj.beginner);
      }
      if ("normal" in obj && obj.normal !== undefined) {
        return String(obj.normal);
      }
      if ("beginner" in obj && obj.beginner !== undefined) {
        return String(obj.beginner);
      }
    }
  }

  return undefined;
}

/**
 * Resolves a localized hotkey string from a translation value.
 **/
export function resolveHotkeyValue(value: unknown): string | undefined {
  if (typeof value === "string") {
    return value;
  }

  if (value && typeof value === "object" && "hotkey" in value) {
    const hotkey = (value as { hotkey?: unknown }).hotkey;
    return typeof hotkey === "string" ? hotkey : undefined;
  }

  return undefined;
}

/**
 * React hook that resolves a localized hotkey by i18n key.
 **/
export function useTranslatedHotkey(id: string): string | undefined {
  const { t } = useTranslation();
  const directHotkey = resolveHotkeyValue(t(id as any));

  if (directHotkey) {
    return directHotkey;
  }

  return resolveHotkeyValue(t(`${id}.hotkey` as any));
}

/**
 * Hook binding a keyboard shortcut with optional translation and overrides.
 **/
export function useCommandHotkey(
  hotkeyOrId: string,
  callback: () => void,
  options?: Parameters<typeof useHotkeys>[2],
  deps?: React.DependencyList,
  configuration?: {
    overrides?: Record<string, string> | undefined;
    translatedHotkey?: string | undefined;
  },
) {
  const inferredTranslatedHotkey = useTranslatedHotkey(hotkeyOrId);
  const translatedHotkey = configuration?.translatedHotkey ?? inferredTranslatedHotkey;
  const finalHotkey = reactHostPort.useMemo(() => configuration?.overrides?.[hotkeyOrId] ?? translatedHotkey ?? hotkeyOrId, [configuration?.overrides, hotkeyOrId, translatedHotkey]);

  useHotkeys(finalHotkey, callback, options || {}, deps || []);
}

/**
 * Hook returning whether a CSS media query currently matches.
 **/
export function useMediaQuery(query: string, defaultValue = false): boolean {
  const getMatches = reactHostPort.useCallback(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return defaultValue;
    }

    return window.matchMedia(query).matches;
  }, [defaultValue, query]);

  const [matches, setMatches] = reactHostPort.useState<boolean>(getMatches);

  reactHostPort.useEffect(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return undefined;
    }

    const mediaQueryList = window.matchMedia(query);
    const bindings = createDOMEventBinding();
    const handleChange = (event: MediaQueryListEvent) => setMatches(event.matches);
    setMatches(mediaQueryList.matches);
    bindings.listen(mediaQueryList, "change", handleChange);

    return () => {
      bindings.dispose();
    };
  }, [query]);

  return matches;
}

/**
 * 3D point with x, y, z coordinates.
 **/
export interface Point {
  x: number;
  y: number;
  z: number;
}

/**
 * 3D direction vector with x, y, z components.
 **/
export interface Vector {
  x: number;
  y: number;
  z: number;
}

/**
 * 3D coordinate plane defined by an origin point and two axis vectors.
 **/
export interface Plane {
  origin: Point;
  xAxis: Vector;
  yAxis: Vector;
}

/**
 * 3D camera defined by position, forward direction, and up direction.
 **/
export interface Camera {
  position: Point;
  forward: Vector;
  up: Vector;
}

// #endregion 🎼Utilities

// #region 🔊Section Specificity
// Enum defining priority levels for section content ownership.
// Consumers MUST use these constants for section precedence.

/**
 * Priority enum for section content ownership across apps.
 **/
export enum SectionSpecificity {
  SKETCHPAD = 0,
  KIT = 10,
  QUALITY = 20,
  TYPE = 20,
  DESIGN = 20,
  DOCS = 20,
  SELECTION = 30,
}

// #endregion 🔊Section Specificity

// #region 🔤Interaction Context
// React context for tracking active UI interactions.
// Consumers MUST wrap interactive elements with InteractionProvider.

/**
 * InteractionCommands holds the data fields for a InteractionCommands record.
 **/
interface InteractionCommands {
  setActiveInteraction: (elementId?: string, interactionId?: string) => void;
}

/**
 * InteractionContext holds the data fields for a InteractionContext record.
 **/
const InteractionContext = reactHostPort.createContext<InteractionCommands | undefined>(undefined);
/**
 * ActiveInteractionContext holds the data fields for a ActiveInteractionContext record.
 **/
const ActiveInteractionContext = reactHostPort.createContext<string | undefined>(undefined);

/**
 * Context provider for UI interaction commands and active state.
 **/
export const InteractionProvider: React.FC<{
  commands?: InteractionCommands;
  activeInteraction?: string;
  children: React.ReactNode;
}> = ({ commands, activeInteraction, children }) => {
  return (
    <InteractionContext.Provider value={commands}>
      <ActiveInteractionContext.Provider value={activeInteraction}>{children}</ActiveInteractionContext.Provider>
    </InteractionContext.Provider>
  );
};

/**
 * useInteractionCommands holds the data fields for a useInteractionCommands record.
 **/
const useInteractionCommands = () => reactHostPort.useContext(InteractionContext);
/** useActiveInteraction holds the data fields for a useActiveInteraction record.
 **/
/**
 **/
const useActiveInteraction = () => reactHostPort.useContext(ActiveInteractionContext);

// #endregion 🔤Interaction Context

// #region 🎈Level Context
/** @emoji 📚 Semantic UI depth layer for background, hover, and z-index tokens. */
export type Level = "base" | "canvas" | "window" | "panel" | "overlay" | "temporary";

const LevelContext = reactHostPort.createContext<Level>("base");

/** @emoji 🎈 Sets the current UI depth level for descendant chrome. */
export const LevelProvider: React.FC<{
	readonly level: Level;
	readonly children: React.ReactNode;
}> = ({ level, children }) => <LevelContext.Provider value={level}>{children}</LevelContext.Provider>;

/** @emoji 🪝 Returns the nearest {@link LevelProvider} level. */
export function useLevel(): Level {
	return reactHostPort.useContext(LevelContext);
}

/** @emoji 🎨 Tailwind background class for a {@link Level}. */
export function getLevelBgClass(level: Level): string {
	switch (level) {
		case "canvas":
			return "bg-canvas";
		case "window":
			return "bg-window";
		case "panel":
			return "bg-panel";
		case "overlay":
			return "bg-overlay";
		case "temporary":
			return "bg-temporary";
		default:
			return "bg-base";
	}
}

/** @emoji 🎨 Tailwind hover background class for a {@link Level}. */
export function getLevelHoverClass(level: Level): string {
	switch (level) {
		case "canvas":
			return "hover:bg-hover-canvas";
		case "window":
			return "hover:bg-hover-window";
		case "panel":
			return "hover:bg-hover-panel";
		case "overlay":
			return "hover:bg-hover-overlay";
		case "temporary":
			return "hover:bg-hover-temporary";
		default:
			return "hover:bg-hover-base";
	}
}

/** @emoji 🎨 Tailwind active-hover class for a {@link Level}. */
export function getLevelActiveHoverClass(level: Level): string {
	switch (level) {
		case "canvas":
			return "data-[state=active]:bg-hover-canvas";
		case "window":
			return "data-[state=active]:bg-hover-window";
		case "panel":
			return "data-[state=active]:bg-hover-panel";
		case "overlay":
			return "data-[state=active]:bg-hover-overlay";
		case "temporary":
			return "data-[state=active]:bg-hover-temporary";
		default:
			return "data-[state=active]:bg-hover-base";
	}
}

/** @emoji 🎨 Tailwind z-index class for a {@link Level}. */
export function getLevelZClass(level: Level): string {
	switch (level) {
		case "canvas":
			return "z-canvas";
		case "window":
			return "z-window";
		case "panel":
			return "z-panel";
		case "overlay":
			return "z-overlay";
		case "temporary":
			return "z-temporary";
		default:
			return "z-base";
	}
}

/** @emoji 📏 Secondary chrome line (`border-element`) for window frames and in-window controls. */
export const secondaryLineClass = "border-element";

/** @emoji 📏 Primary chrome line (`border-active-base`) recolors the window U-frame when that stack is globally active. */
export const activeLineClass = "border-active-base";

/** @emoji 📏 Secondary chrome frame (`border border-element`) wrapping window body or caps. */
export const windowFrameClass = `border ${secondaryLineClass}`;

/** @emoji 📏 Top cap sides only — no bottom edge; z-index covers the body top stroke under the cap. */
export const windowCapFrameClass = `relative z-[2] border-t border-x !border-b-0 ${secondaryLineClass} bg-window`;

/** @emoji 📏 Top cap with primary chrome line when the stack owns the globally active window. */
export const windowCapFrameActiveClass = `relative z-[2] border-t border-x !border-b-0 ${activeLineClass} bg-window`;

/** @emoji 📏 Canvas gap stroke — horizontal segment of the U between tab and fullscreen caps. */
export const windowGapFrameClass = `border-x-0 border-t-0 border-b ${secondaryLineClass}`;

/** @emoji 📏 Canvas gap stroke with primary chrome line when the stack is globally active. */
export const windowGapFrameActiveClass = `border-x-0 border-t-0 border-b ${activeLineClass}`;

/** @emoji 📏 Bottom of U-shaped window chrome; sides and bottom only (top stroke is gap + cap corners). */
export const windowBodyFrameClass = `relative z-0 -mt-px border-x border-t-0 border-b ${secondaryLineClass} bg-canvas`;

/** @emoji 📏 U-shaped body frame with primary chrome line when the stack owns the globally active window. */
export const windowBodyFrameActiveClass = `relative z-0 -mt-px border-b border-l border-r border-t-0 ${activeLineClass} bg-canvas`;

/** @emoji 📏 L-shaped joint at the bottom-outer corner of a window cap (connects side into gap baseline). */
export const windowCapCornerClass = (lineClass: string, side: "left" | "right", behindTabs = false) =>
  cn(
    "pointer-events-none absolute bottom-0 box-border size-px border-b",
    behindTabs ? "z-0" : "z-[3]",
    side === "left" ? "left-0 border-l" : "right-0 border-r",
    lineClass,
  );

/** @emoji 📐 Grid tracks for multi-tab active chrome: one column per tab, then flex gap, then controls. */
export interface ModeDockChromeGrid {
  readonly templateColumns: string;
  readonly activeCol: number;
  readonly gapCol: number;
  readonly controlsCol: number;
  readonly bodyColumnSpan: string;
  readonly activeTabIndex: number;
  readonly tabCol: (tabIndex: number) => number;
}

/** @emoji 📐 Computes {@link ModeDockChromeGrid} column indices for a tab stack. */
export function modeDockChromeGridPlacement(
  tabs: readonly { id: string; title: string }[],
  activeId: string | undefined,
): ModeDockChromeGrid {
  const activeTabIndex = Math.max(0, tabs.findIndex((tab) => tab.id === activeId));
  const gapCol = tabs.length + 1;
  const controlsCol = tabs.length + 2;
  const activeCol = activeTabIndex + 1;
  const templateParts = [...tabs.map(() => "max-content"), "minmax(0, 1fr)", "max-content"];
  return {
    templateColumns: templateParts.join(" "),
    activeCol,
    gapCol,
    controlsCol,
    bodyColumnSpan: `${activeCol} / ${gapCol + 1}`,
    activeTabIndex,
    tabCol: (tabIndex) => tabIndex + 1,
  };
}

/** @emoji 📏 Inactive sibling tab — gray pill resting on the U-frame baseline; its bottom stroke color is applied per-stack (active vs secondary) so the chrome reads as one continuous outline. */
export const modeDockInactiveTabClass =
  "relative z-30 box-border min-h-medium shrink-0 border border-element bg-window";

/** @emoji 📏 Filled primary for the globally active dock tab (matches single-tab selection). */
export const modeDockActiveTabFillClass =
  "bg-active-base text-active-foreground hover:bg-active-base hover:text-active-foreground";

/** @emoji 📏 Stack-active tab — three-sided U-cap above body; open bottom merges into stack body (no bottom stroke). */
export const modeDockActiveTabClass =
  `relative z-20 box-border min-h-medium shrink-0 border-t border-l border-r !border-b-0 border-active-base ${modeDockActiveTabFillClass}`;

/** @emoji 📏 Maximize cap on the right of the gap (secondary chrome line). */
export const windowControlsCapClass = `relative z-[2] flex shrink-0 items-stretch border-t border-x !border-b-0 ${secondaryLineClass} bg-window`;

/** @emoji 📏 Maximize cap on the right when the stack owns the globally active window. */
export const windowControlsCapActiveClass = `relative z-[2] flex shrink-0 items-stretch border-t border-x !border-b-0 ${activeLineClass} bg-window`;

/** @emoji 📏 Maximize cap on the right of the gap when multi-tab chrome uses a split column layout. */
export const windowControlsCapActiveSplitClass = `relative flex shrink-0 items-stretch border-t border-x !border-b-0 ${activeLineClass} bg-window`;

/** @emoji 🎨 Tailwind border token class for a {@link Level}. */
export function getLevelBorderElementClass(level: Level): string {
	switch (level) {
		case "canvas":
			return "border-hover-canvas";
		case "window":
			return "border-hover-window";
		case "panel":
			return "border-hover-panel";
		case "overlay":
			return "border-hover-overlay";
		case "temporary":
			return "border-hover-temporary";
		default:
			return "border-hover-base";
	}
}

/** @emoji 🎨 Tailwind divide token class for a {@link Level}. */
export function getLevelDivideElementClass(level: Level): string {
	switch (level) {
		case "canvas":
			return "divide-hover-canvas";
		case "window":
			return "divide-hover-window";
		case "panel":
			return "divide-hover-panel";
		case "overlay":
			return "divide-hover-overlay";
		case "temporary":
			return "divide-hover-temporary";
		default:
			return "divide-hover-base";
	}
}
// #endregion 🎈Level Context

// #region 🐹Element
// Core element types, transaction context, and level-based CSS class helpers.
// Consumers MUST use level functions for consistent styling.

/**
 * Interface for start/finalize/abort lifecycle of a UI transaction.
 **/
export interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

/**
 * TransactionContext holds the data fields for a TransactionContext record.
 **/
const TransactionContext = reactHostPort.createContext<Transaction | undefined>(undefined);

/**
 * Context provider that supplies a Transaction to descendants.
 **/
export const TransactionProvider: React.FC<{
  transaction?: Transaction;
  children: React.ReactNode;
}> = ({ transaction, children }) => {
  return <TransactionContext.Provider value={transaction}>{children}</TransactionContext.Provider>;
};

/**
 * Hook returning the current Transaction context.
 **/
export const useTransaction = (): Transaction | undefined => reactHostPort.useContext(TransactionContext);

/**
 * Base props interface requiring an id string.
 **/
export interface ElementBaseProps {
  id: string;
}

export interface ElementProps extends ElementBaseProps {}

// #endregion 🐹Element

// #region 🪆Command
// Command palette UI built on cmdk primitives.
// Consumers MUST use CommandInput for search functionality.

/**
 * Command holds the data fields for a Command record.
 **/
function Command({ className, ...props }: React.ComponentProps<typeof CommandPrimitive>) {
  return <CommandPrimitive data-slot="command" className={cn("bg-popover text-popover-foreground flex h-full w-full flex-col overflow-hidden", className)} {...props} />;
}

/**
 * CommandDialog holds the data fields for a CommandDialog record.
 **/
function CommandDialog({
  title = "Command Palette",
  description = "Search for a command to run...",
  children,
  className,
  showCloseButton = true,
  ...props
}: React.ComponentProps<typeof Dialog> & {
  title?: string;
  description?: string;
  className?: string;
  showCloseButton?: boolean;
}) {
  return (
    <Dialog {...props}>
      <DialogHeader className="sr-only">
        <DialogTitle>{title}</DialogTitle>
        <DialogDescription>{description}</DialogDescription>
      </DialogHeader>
      <DialogContent className={cn("overflow-hidden p-0", className)} showCloseButton={showCloseButton}>
        <Command className="[&_[cmdk-group-heading]]:text-muted-foreground **:data-[slot=command-input-wrapper]:h-large [&_[cmdk-group-heading]]:px-single [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group]:px-single [&_[cmdk-group]:not([hidden])_~[cmdk-group]]:pt-0 [&_[cmdk-input-wrapper]_svg]:h-small [&_[cmdk-input-wrapper]_svg]:w-small [&_[cmdk-input]]:h-large [&_[cmdk-item]]:px-single [&_[cmdk-item]]:py-tiny [&_[cmdk-item]_svg]:h-small [&_[cmdk-item]_svg]:w-small">
          {children}
        </Command>
      </DialogContent>
    </Dialog>
  );
}

/**
 * CommandInput holds the data fields for a CommandInput record.
 **/
function CommandInput({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Input>) {
  return (
    <div data-slot="command-input-wrapper" className="flex h-medium items-center gap-single border-b border-element px-tiny">
      <SearchIcon className="size-small shrink-0 opacity-50" />
      <CommandPrimitive.Input data-slot="command-input" className={cn("placeholder:text-muted-foreground flex h-medium w-full bg-transparent text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50", className)} {...props} />
    </div>
  );
}

/**
 * CommandList holds the data fields for a CommandList record.
 **/
function CommandList({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.List>) {
  return <CommandPrimitive.List data-slot="command-list" className={cn("max-h-[300px] scroll-py-single overflow-x-hidden overflow-y-auto", className)} {...props} />;
}

/**
 * CommandEmpty holds the data fields for a CommandEmpty record.
 **/
function CommandEmpty({ ...props }: React.ComponentProps<typeof CommandPrimitive.Empty>) {
  return <CommandPrimitive.Empty data-slot="command-empty" className="py-medium text-center text-sm" {...props} />;
}

/**
 * CommandGroup holds the data fields for a CommandGroup record.
 **/
function CommandGroup({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Group>) {
  return (
    <CommandPrimitive.Group
      data-slot="command-group"
      className={cn(
        "text-foreground [&_[cmdk-group-heading]]:text-muted-foreground overflow-hidden p-single [&_[cmdk-group-heading]]:px-single [&_[cmdk-group-heading]]:py-single [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium",
        className,
      )}
      {...props}
    />
  );
}

function CommandSeparator({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Separator>) {
  return <CommandPrimitive.Separator data-slot="command-separator" className={cn("bg-border -mx-single h-px", className)} {...props} />;
}

/**
 * CommandItem holds the data fields for a CommandItem record.
 **/
function CommandItem({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Item>) {
  return (
    <CommandPrimitive.Item
      data-slot="command-item"
      className={cn(
        "data-[selected=true]:bg-hover-temporary data-[selected=true]:text-foreground [&_svg:not([class*='text-'])]:text-muted-foreground relative flex items-center gap-single p-single text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny cursor-selectable",
        className,
      )}
      {...props}
    />
  );
}

/**
 * CommandShortcut holds the data fields for a CommandShortcut record.
 **/
function CommandShortcut({ className, ...props }: React.ComponentProps<"span">) {
  return <span data-slot="command-shortcut" className={cn("text-muted-foreground ml-auto text-xs tracking-widest", className)} {...props} />;
}

// #endregion 🪆Command

export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };

// #region 🎮Footer
// Status bar component at the bottom of the layout.
// Consumers MUST provide FooterItem entries for each action.

/**
 * Configuration interface for a single footer action item.
 **/
export interface FooterItem {
  id: string;
  icon?: React.ReactNode;
  text?: string;
  content?: React.ReactNode;
  order?: number;
  onClick?: () => void;
  className?: string;
  disabled?: boolean;
}

/**
 * Props interface for the Footer component.
 **/
export interface FooterProps {
  items?: FooterItem[];
  className?: string;
  isVisible?: boolean;
}

/**
 * Footer holds the data fields for a Footer record.
 **/
const Footer: React.FC<FooterProps> = ({ items = [], className = "", isVisible = true }) => {
  const level = useLevel();
  const sortedItems = [...items].sort((a, b) => (a.order || 0) - (b.order || 0));
  const bgClass = getLevelBgClass(level);
  return (
    <footer id="semio.sketchpad.footer" data-slot="footer" className={cn("border-t flex items-center h-medium transition-transform duration-200", bgClass, isVisible ? "translate-y-0" : "translate-y-full", className)}>
      <div className="flex items-center h-full px-single min-w-0">
        <ActionGroup className="border">
          {sortedItems.map((item) => (
            <ActionGroupItem key={item.id} as={item.onClick ? "button" : "div"} id={item.id} text={item.text} onClick={item.onClick} disabled={item.disabled} className={cn(item.content && !item.text && "aspect-auto", item.className)}>
              {item.content ?? item.icon}
            </ActionGroupItem>
          ))}
        </ActionGroup>
      </div>
    </footer>
  );
};

export { Footer };

// #endregion 🎮Footer

// #region 🪨Layout
// Top-level layout orchestrating navbar, panels, canvas, and footer.
// Consumers MUST provide a canvas element.

/**
 * Props interface for the top-level Layout component.
 **/
export interface LayoutProps {
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  bottomPanel?: BottomPanelProps;
  leftSidePanel?: SidePanelProps;
  rightSidePanel?: SidePanelProps;
  mobilePanel?: MobilePanelProps;
  canvas: React.ReactNode;
  toolbar?: React.ReactNode;
  mobile?: boolean;
  className?: string;
}

const Layout: React.FC<LayoutProps> = ({ navbar, footer, bottomPanel, leftSidePanel, rightSidePanel, mobilePanel, canvas, toolbar, mobile = false, className = "" }) => (
  <div className={cn("flex flex-col overflow-hidden", mobile ? "touch h-full w-full" : "h-screen w-screen", className)}>
    {navbar && <div className="flex-shrink-0">{navbar}</div>}
    {mobile ? (
      <div className="flex flex-col flex-1 min-h-0">
        {mobilePanel && mobilePanel.visible && <MobilePanel {...mobilePanel} />}
        <div className="flex-1 min-w-0 min-h-0 relative">{canvas}</div>
      </div>
    ) : (
      <div className="flex flex-1 min-h-0 relative">
        {leftSidePanel && leftSidePanel.visible && <SidePanel {...leftSidePanel} position="left" />}
        <div className="flex flex-col flex-1 min-w-0 relative">
          <div className="flex flex-1 min-h-0 relative">
            <div className="flex-1 min-w-0 min-h-0 relative">{canvas}</div>
            {rightSidePanel && rightSidePanel.visible && <SidePanel {...rightSidePanel} position="right" />}
          </div>
          {bottomPanel && bottomPanel.visible && <BottomPanel {...bottomPanel} />}
        </div>
      </div>
    )}
    {(footer || toolbar) && (
      <div className="flex-shrink-0 relative">
        {toolbar && <div className="pointer-events-none absolute bottom-[calc(100%+var(--spacing-double))] left-0 right-0 z-panel flex justify-center">{toolbar}</div>}
        {footer}
      </div>
    )}
  </div>
);

export { Layout };

// #endregion 🪨Layout

// #region 🌐Popover
// Floating popover component built on Radix primitives.

/**
 * Popover holds the data fields for a Popover record.
/**
 * Popover holds the data fields for a Popover record.
 **/
function Popover({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root data-slot="popover" {...props} />;
}

/**
 * PopoverTrigger holds the data fields for a PopoverTrigger record.
 **/
function PopoverTrigger({ className, ...props }: React.ComponentProps<typeof PopoverPrimitive.Trigger>) {
  return <PopoverPrimitive.Trigger data-slot="popover-trigger" className={cn(className)} {...props} />;
}

/**
 * PopoverContent holds the data fields for a PopoverContent record.
 **/
function PopoverContent({ className, align = "center", sideOffset = 4, ...props }: React.ComponentProps<typeof PopoverPrimitive.Content>) {
  return (
    <PopoverPrimitive.Portal>
      <PopoverPrimitive.Content
        data-slot="popover-content"
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-temporary w-72 origin-(--radix-popover-content-transform-origin) border p-1 outline-hidden",
          className,
        )}
        {...props}
      />
    </PopoverPrimitive.Portal>
  );
}

/**
 * PopoverAnchor holds the data fields for a PopoverAnchor record.
 **/
function PopoverAnchor({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Anchor>) {
  return <PopoverPrimitive.Anchor data-slot="popover-anchor" {...props} />;
}

export { Popover, PopoverAnchor, PopoverContent, PopoverTrigger };

// #endregion 🌐Popover

// #region 🎙️Tooltip
// Tooltip components with expertise-level adaptive content.
// Consumers MUST configure the expertise mode provider.

/**
 * Configuration for enhanced tooltip with label, paths, and hotkey.
 **/
export interface TooltipConfig {
  labelKey: string;
  manualPath?: string;
  tutorialPath?: string;
  hotkey?: string;
}

/**
 * Data interface for description-based tooltip content.
 **/
export interface DescriptionTooltipData {
  label?: string;
  description?: string;
  descriptionBeginner?: string;
  manual?: string;
  tutorial?: string;
  hotkey?: string;
}

/**
 * Registers the expertise provider function for tooltips.
 **/
export function setTooltipModeProvider(fn: () => Expertise) {
  setExpertiseProvider(fn);
}

/**
 * Hook returning the current expertise level for tooltips.
 **/
export function useTooltipMode(): Expertise {
  if (!_expertiseProvider) return Expertise.BEGINNER;
  return _expertiseProvider();
}

/**
 * TooltipProvider holds the data fields for a TooltipProvider record.
 **/
function TooltipProvider({ delayDuration = 400, ...props }: React.ComponentProps<typeof TooltipPrimitive.Provider>) {
  return <TooltipPrimitive.Provider data-slot="tooltip-provider" delayDuration={delayDuration} {...props} />;
}

/**
 * Tooltip holds the data fields for a Tooltip record.
 **/
function Tooltip({ ...props }: React.ComponentProps<typeof TooltipPrimitive.Root>) {
  return (
    <TooltipProvider>
      <TooltipPrimitive.Root data-slot="tooltip" {...props} />
    </TooltipProvider>
  );
}

/**
 * TooltipTrigger holds the data fields for a TooltipTrigger record.
 **/
function TooltipTrigger({ className, asChild, ...props }: React.ComponentProps<typeof TooltipPrimitive.Trigger>) {
  return <TooltipPrimitive.Trigger data-slot="tooltip-trigger" asChild={asChild} className={cn(className)} {...props} />;
}

/**
 * TooltipContent holds the data fields for a TooltipContent record.
 **/
function TooltipContent({ className, sideOffset = 8, children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Content>) {
  return (
    <TooltipPrimitive.Portal>
      <TooltipPrimitive.Content
        data-slot="tooltip-content"
        sideOffset={sideOffset}
        className={cn(
          "bg-temporary border border-accent-foreground text-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-temporary origin-(--radix-tooltip-content-transform-origin) p-single text-xs text-balance w-max max-w-fit",
          className,
        )}
        {...props}
      >
        {children}
      </TooltipPrimitive.Content>
    </TooltipPrimitive.Portal>
  );
}

/**
 * EnhancedTooltipContentProps holds the data fields for a EnhancedTooltipContentProps record.
 **/
interface EnhancedTooltipContentProps {
  config: TooltipConfig;
}

/** EnhancedTooltipContent holds the data fields for a EnhancedTooltipContent record.
 **/
/**
 **/
function EnhancedTooltipContent({ config }: EnhancedTooltipContentProps) {
  const { t } = useTranslation();
  const mode = useTooltipMode();

  if (mode === Expertise.EXPERT) return null;

  const { labelKey, manualPath, tutorialPath, hotkey } = config;
  const showManual = mode === Expertise.BEGINNER || mode === Expertise.NORMAL;
  const showTutorial = mode === Expertise.BEGINNER;

  const label = useLabel(labelKey);

  const fullManualPath = manualPath ? `/docs/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/docs/tutorials/${tutorialPath}` : undefined;

  const handleHotkeyClick = () => {
    if (labelKey) {
      window.dispatchEvent(
        new CustomEvent("navigate-to-hotkey", {
          detail: { path: labelKey },
        }),
      );
    }
  };

  return (
    <div className="flex flex-col gap-single">
      <span>{label}</span>
      {(showManual && fullManualPath) || (showTutorial && fullTutorialPath) || hotkey ? (
        <div className="grid w-full grid-cols-3 items-center border-t border-accent-foreground pt-single gap-single">
          {showManual && fullManualPath ? (
            <Link to={fullManualPath} className="flex items-center gap-single cursor-pointer text-foreground transition-colors p-single hover:bg-hover-temporary">
              <BookIcon className="size-tiny" />
              <span>{useLabel("tooltip.manual")}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {showTutorial && fullTutorialPath ? (
            <Link to={fullTutorialPath} className="flex items-center gap-single cursor-pointer text-foreground transition-colors p-single hover:bg-hover-temporary">
              <TutorialIcon className="size-tiny" />
              <span className="block text-center">{useLabel("tooltip.tutorial")}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {hotkey ? (
            <kbd onClick={handleHotkeyClick} className="border border-accent-foreground text-muted-foreground p-single text-2xs font-mono justify-self-end cursor-pointer">
              {hotkey}
            </kbd>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

/**
 * DescriptionTooltipContentProps holds the data fields for a DescriptionTooltipContentProps record.
 **/
interface DescriptionTooltipContentProps {
  id: string;
}

/**
 * DescriptionTooltipContent holds the data fields for a DescriptionTooltipContent record.
 **/
function DescriptionTooltipContent({ id }: DescriptionTooltipContentProps) {
  const { t } = useTranslation();
  const mode = useTooltipMode();

  if (mode === Expertise.EXPERT) return null;

  const manualLabel = useLabel("tooltip.manual");
  const tutorialLabel = useLabel("tooltip.tutorial");
  const value = t(id as any) as any;
  const manualPath = typeof value === "object" && value?.manual ? value.manual : undefined;
  const tutorialPath = typeof value === "object" && value?.tutorial ? value.tutorial : undefined;
  const label =
    typeof value === "string"
      ? value
      : typeof value === "object" && value?.label
        ? typeof value.label === "string"
          ? value.label
          : typeof value.label === "object"
            ? mode === Expertise.BEGINNER && value.label.beginner !== undefined
              ? String(value.label.beginner)
              : value.label.normal !== undefined
                ? String(value.label.normal)
                : value.label.beginner !== undefined
                  ? String(value.label.beginner)
                  : undefined
            : undefined
        : undefined;

  let hotkey: string | undefined;
  if (typeof value === "object" && value?.hotkey) {
    hotkey = typeof value.hotkey === "string" ? value.hotkey : undefined;
  } else {
    const hotkeyKey = `${id}.hotkey`;
    const hotkeyValue = t(hotkeyKey as any) as any;
    if (typeof hotkeyValue === "string" && hotkeyValue !== hotkeyKey) {
      hotkey = hotkeyValue;
    }
  }

  const showManual = (mode === Expertise.BEGINNER || mode === Expertise.NORMAL) && manualPath;
  const showTutorial = mode === Expertise.BEGINNER && tutorialPath;

  const fullManualPath = manualPath ? `/docs/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/docs/tutorials/${tutorialPath}` : undefined;

  const hasLinks = showManual || showTutorial || hotkey;

  const handleHotkeyClick = () => {
    window.dispatchEvent(
      new CustomEvent("navigate-to-hotkey", {
        detail: { path: id },
      }),
    );
  };

  return (
    <div className="flex flex-col gap-single">
      {label && <span className="text-sm">{label}</span>}
      {hasLinks ? (
        <div className="flex w-full items-center border-t border-accent-foreground pt-single gap-single">
          {showManual && fullManualPath && (
            <Link to={fullManualPath} className="flex items-center gap-single cursor-pointer text-foreground transition-colors p-single hover:bg-hover-temporary">
              <BookIcon className="size-3" />
              <span>{manualLabel}</span>
            </Link>
          )}
          {showTutorial && fullTutorialPath && (
            <Link to={fullTutorialPath} className="flex items-center gap-single cursor-pointer text-foreground transition-colors p-single hover:bg-hover-temporary">
              <TutorialIcon className="size-3" />
              <span className="block text-center">{tutorialLabel}</span>
            </Link>
          )}
          {hotkey && (
            <kbd onClick={handleHotkeyClick} className="border border-accent-foreground text-muted-foreground p-single text-2xs font-mono ml-auto cursor-pointer">
              {hotkey}
            </kbd>
          )}
        </div>
      ) : null}
    </div>
  );
}

// #endregion 🎙️Tooltip

// #region 🌥️Base Components
// Foundational internal components like Label.
// Consumers MUST use these as building blocks for inputs.

/**
 * LabelProps holds the data fields for a LabelProps record.
 **/
interface LabelProps {
  id: string;
  rowId?: string;
  label?: React.ReactNode;
  labelElementId?: string;
  className?: string;
  /**
   * Property rows use the label/value grid; tree group headers mirror TreeItem header geometry
   * (gutter, tree-label slot, trailing control) so collection rows do not drift into the value column.
   */
  labelLayoutKind?: "property" | "treeGroupHeader";
  children: React.ReactNode;
}
// [🏘️semio📚js🗃️sketchpad💻elements🔖basecomponents🪨label](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Base%20Components/d/i/Label)
export function Label({ id, rowId, label, labelElementId, className, children, labelLayoutKind = "property" }: LabelProps) {
  const localizedLabel = useLabel(id);
  const resolvedLabel = label ?? localizedLabel;
  const fallbackLabel = reactHostPort.useMemo(() => {
    const trailingToken = id.split(".").pop() ?? id;
    const normalizedToken = trailingToken.replace(/[-_]+/g, " ").trim();
    if (!normalizedToken) return id;
    return normalizedToken
      .split(/\s+/)
      .map((word) => (word.length > 0 ? `${word[0].toUpperCase()}${word.slice(1)}` : word))
      .join(" ");
  }, [id]);
  const displayLabel = resolvedLabel ?? fallbackLabel;
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const isInsideTreeRow = reactHostPort.useContext(TreeRowAlignmentContext);
  const treePropertyRowOffsetPx = detailPanelIndentPx(level, indentMultiplier);
  const propertyRowRef = reactHostPort.useRef<HTMLDivElement>(null);
  const propertyLabelRef = reactHostPort.useRef<HTMLDivElement>(null);
  const propertyControlRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [propertyRowStacked, setPropertyRowStacked] = reactHostPort.useState(false);

  reactHostPort.useEffect(() => {
    const rowElement = propertyRowRef.current;
    const labelElement = propertyLabelRef.current;
    const controlElement = propertyControlRef.current;
    if (!rowElement || !labelElement || !controlElement) {
      return;
    }

    let animationFrame = 0;
    const resolvePropertyLayout = () => {
      animationFrame = 0;
      const rowWidthPx = rowElement.clientWidth;
      const labelWidthPx = Math.ceil(labelElement.scrollWidth);
      const controlMinWidthPx = Math.ceil(controlElement.scrollWidth);
      const minimumInlineWidthPx = labelWidthPx + controlMinWidthPx + detailPanelPropertyInlineGapPx;
      const labelRect = labelElement.getBoundingClientRect();
      const controlRect = controlElement.getBoundingClientRect();
      const overlaps = labelRect.right + detailPanelPropertyInlineGapPx > controlRect.left;
      const shouldStack = propertyRowStacked ? overlaps || minimumInlineWidthPx > rowWidthPx - detailPanelPropertyStackedToInlineHysteresisPx : overlaps || minimumInlineWidthPx > rowWidthPx;
      setPropertyRowStacked((current) => (current === shouldStack ? current : shouldStack));
    };

    const scheduleResolvePropertyLayout = () => {
      if (animationFrame !== 0) {
        cancelAnimationFrame(animationFrame);
      }
      animationFrame = requestAnimationFrame(resolvePropertyLayout);
    };

    const observer = new ResizeObserver(() => scheduleResolvePropertyLayout());
    observer.observe(rowElement);
    observer.observe(labelElement);
    observer.observe(controlElement);
    scheduleResolvePropertyLayout();

    return () => {
      observer.disconnect();
      if (animationFrame !== 0) {
        cancelAnimationFrame(animationFrame);
      }
    };
  }, [id, label, level, treePropertyRowOffsetPx, children, propertyRowStacked]);

  if (labelLayoutKind === "treeGroupHeader") {
    const treeGroupHeaderLabel = id ? (
      <Tooltip>
        <TooltipTrigger asChild>
          <span data-slot="tree-label" id={labelElementId} className="flex min-w-0 flex-1 items-center text-xs font-normal text-left truncate text-foreground h-[22px]" style={treeItemLabelStyle}>
            {displayLabel}
          </span>
        </TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    ) : (
      <span data-slot="tree-label" id={labelElementId} className="flex min-w-0 flex-1 items-center text-xs font-normal text-left truncate text-foreground h-[22px]">
        {displayLabel}
      </span>
    );

    const treeGroupHeaderInner = (
      <div id={rowId} data-slot="tree-group-header-row" className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName, className)}>
        <div className={cn(treeHeaderMainClassName, "min-h-[22px] items-center")}>
          {treeGroupHeaderLabel}
          <div data-slot="tree-group-header-control" className="ml-auto flex min-w-0 shrink-0 items-center justify-end">
            {children}
          </div>
        </div>
      </div>
    );

    if (!isTree) {
      return <TreeRowAlignmentContext.Provider value={false}>{treeGroupHeaderInner}</TreeRowAlignmentContext.Provider>;
    }

    if (isInsideTreeRow) {
      return <TreeRowAlignmentContext.Provider value={false}>{treeGroupHeaderInner}</TreeRowAlignmentContext.Provider>;
    }

    return (
      <TreeRowAlignmentContext.Provider value={false}>
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0">
          {treeGroupHeaderInner}
        </TreeAlignedRow>
      </TreeRowAlignmentContext.Provider>
    );
  }

  const propertyLabelElement = (
    <Tooltip>
      <TooltipTrigger asChild>
        {isTree ? (
          <div ref={propertyLabelRef} data-slot="property-label-tree" className="min-w-0" style={{ paddingLeft: `${treePropertyRowOffsetPx}px` }}>
            <div className="inline-flex min-w-0 h-[22px]">
              <span data-slot="property-label" id={labelElementId} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-left truncate cursor-pointer transition-colors hover:bg-hover-panel h-[22px] pl-[4px]">
                {resolvedLabel}
              </span>
            </div>
          </div>
        ) : (
          <div ref={propertyLabelRef} data-slot="property-label-inline" className="min-w-0">
            <span data-slot="property-label" id={labelElementId} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-left truncate cursor-pointer transition-colors hover:bg-hover-panel h-[22px]">
              {resolvedLabel}
            </span>
          </div>
        )}
      </TooltipTrigger>
      <TooltipContent>
        <DescriptionTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );

  const propertyRowElement = (
    <div
      ref={propertyRowRef}
      id={rowId}
      data-slot="property-row"
      data-property-layout={propertyRowStacked ? "stacked" : "inline"}
      style={{
        ...(isTree ? { marginLeft: `${-treePropertyRowOffsetPx}px`, width: treePropertyRowOffsetPx > 0 ? `calc(100% + ${treePropertyRowOffsetPx}px)` : "100%" } : {}),
        gridTemplateColumns: propertyRowStacked ? "minmax(0, 1fr)" : `${detailPanelPropertyLabelColumnWidthPx}px minmax(0, 1fr)`,
        rowGap: `${propertyRowStacked ? detailPanelPropertyStackedRowGapPx : 0}px`,
      }}
      className={cn(detailPanelPropertyRowClassName, !isTree && "w-full", className)}
    >
      {propertyLabelElement}
      <div ref={propertyControlRef} data-slot="property-control" className={detailPanelPropertyControlClassName} style={propertyRowStacked ? { paddingLeft: `${detailPanelPropertyLabelColumnWidthPx + detailPanelPropertyInlineGapPx}px` } : undefined}>
        <PropertyValueColumnContext.Provider value={true}>{children}</PropertyValueColumnContext.Provider>
      </div>
    </div>
  );

  if (isTree) {
    if (isInsideTreeRow) {
      return propertyRowElement;
    }
    return (
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0} anchorOffsetPx={detailPanelHeaderLineCenterPx}>
        {propertyRowElement}
      </TreeAlignedRow>
    );
  }

  return propertyRowElement;
}

// #endregion 🌥️Base Components

// #region 🏷️Display Components
// Read-only display wrappers for tooltips and callouts.
// Consumers MUST pass valid config objects.

/**
 * SemioTooltipProps holds the data fields for a SemioTooltipProps record.
 **/
interface SemioTooltipProps {
  children: React.ReactElement;
  config: TooltipConfig;
}

/**
 * SemioTooltip holds the data fields for a SemioTooltip record.
 **/
function SemioTooltip({ children, config }: SemioTooltipProps) {
  const mode = useTooltipMode();
  if (mode === Expertise.EXPERT) return children;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{children}</TooltipTrigger>
      <TooltipContent>
        <EnhancedTooltipContent config={config} />
      </TooltipContent>
    </Tooltip>
  );
}

/**
 * IdSemioTooltipProps holds the data fields for a IdSemioTooltipProps record.
 **/
interface IdSemioTooltipProps {
  id: string;
  children: React.ReactNode;
}

/**
 **/
function IdSemioTooltip({ id, children }: IdSemioTooltipProps) {
  const mode = useTooltipMode();
  if (mode === Expertise.EXPERT) return children;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{children}</TooltipTrigger>
      <TooltipContent>
        <DescriptionTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );
}

export { DescriptionTooltipContent, EnhancedTooltipContent, IdSemioTooltip, SemioTooltip, Tooltip, TooltipContent, TooltipProvider, TooltipTrigger };

// #region 📣Aside
// Callout boxes for notes, tips, cautions, and dangers.
// Consumers MUST specify a valid kind prop.

/**
 * Props interface for the Aside callout component.
 **/
export interface AsideProps {
  kind?: "note" | "tip" | "caution" | "danger";
  title?: string;
  children: React.ReactNode;
}

/**
 * iconMap holds the data fields for a iconMap record.
 **/
const iconMap = {
  note: InfoIcon,
  tip: LightbulbIcon,
  caution: TriangleAlertIcon,
  danger: AlertCircleIcon,
};

/**
 * colorMap holds the data fields for a colorMap record.
 **/
const colorMap = {
  note: "border-info-border bg-info-bg text-info-foreground",
  tip: "border-success-border bg-success-bg text-success-foreground",
  caution: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

/**
 * Callout component rendering note, tip, caution, or danger boxes.
 **/
export const Aside: React.FC<AsideProps> = ({ kind = "note", title, children }) => {
  const Icon = iconMap[kind];
  const colorClass = colorMap[kind];

  return (
    <aside className={`my-small p-single border ${colorClass}`}>
      <div className="flex items-start gap-single">
        <Icon className="size-small mt-0.5 flex-shrink-0" />
        <div className="flex-1">
          {title && <div className="font-semibold mb-1">{title}</div>}
          <div>{children}</div>
        </div>
      </div>
    </aside>
  );
};

// #endregion 📣Aside

// #region 📔Avatar
// User avatar components with image, fallback, drag, and table variants.
// Consumers MUST provide content for the fallback.

/**
 * Avatar holds the data fields for a Avatar record.
 **/
const Avatar = reactHostPort.forwardRef<React.ElementRef<typeof AvatarPrimitive.Root>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>>(({ className, style, ...props }, ref) => {
  const isSizeClass = className && (className.includes("size-") || className.includes("w-") || className.includes("h-"));
  const isFullSize = className && className.includes("size-full");
  const hasExplicitSize = style && (style.width || style.height);
  return (
    <AvatarPrimitive.Root
      ref={ref}
      data-slot="avatar"
      style={style}
      className={cn("relative flex overflow-hidden rounded-full", !hasExplicitSize && "shrink-0", !isFullSize && "border border-element", !isSizeClass && !hasExplicitSize && "size-small", className)}
      {...props}
    />
  );
});
Avatar.displayName = "Avatar";

/**
 * AvatarImage holds the data fields for a AvatarImage record.
 **/
const AvatarImage = reactHostPort.forwardRef<React.ElementRef<typeof AvatarPrimitive.Image>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Image ref={ref} data-slot="avatar-image" className={cn("aspect-square size-full", className)} {...props} />
));
AvatarImage.displayName = "AvatarImage";

/**
 * AvatarFallback holds the data fields for a AvatarFallback record.
 **/
const AvatarFallback = reactHostPort.forwardRef<React.ElementRef<typeof AvatarPrimitive.Fallback>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Fallback ref={ref} data-slot="avatar-fallback" className={cn("bg-muted flex size-full items-center justify-center rounded-full", className)} {...props} />
));
AvatarFallback.displayName = "AvatarFallback";

/**
 * Props interface for the DraggableAvatar component.
 **/
export interface DraggableAvatarProps {
  content: string;
  isSelected?: boolean;
  isHovered?: boolean;
  shouldFade?: boolean;
  title?: string;
  dragRef?: (element: HTMLElement | null) => void;
  dragListeners?: any;
  dragAttributes?: any;
  onClick?: () => void;
  onPointerDown?: () => void;
  onMouseDown?: () => void;
  onDoubleClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  className?: string;
  avatarClassName?: string;
  dataDragKind?: "type" | "design";
  dataDragGuid?: string;
}

/**
 * Avatar component with drag-and-drop support and selection styling.
 **/
export const DraggableAvatar = reactHostPort.forwardRef<HTMLDivElement, DraggableAvatarProps>(
  ({ content, isSelected, isHovered, shouldFade, title, dragRef, dragListeners, dragAttributes, onClick, onPointerDown, onMouseDown, onDoubleClick, onPointerEnter, onPointerLeave, className, avatarClassName, dataDragKind, dataDragGuid }, ref) => {
    const dragPointerDown = dragListeners?.onPointerDown as ((event: React.PointerEvent<HTMLDivElement>) => void) | undefined;
    const dragMouseDown = dragListeners?.onMouseDown as ((event: React.MouseEvent<HTMLDivElement>) => void) | undefined;
    const mergedDragListeners = { ...(dragListeners ?? {}) };
    delete mergedDragListeners.onPointerDown;
    delete mergedDragListeners.onMouseDown;
    return (
      <div
        data-slot="avatar"
        ref={dragRef || ref}
        {...mergedDragListeners}
        {...dragAttributes}
        onClick={onClick}
        onPointerDown={(event) => {
          dragPointerDown?.(event);
          onPointerDown?.();
        }}
        onMouseDown={(event) => {
          dragMouseDown?.(event);
          onMouseDown?.();
        }}
        onDoubleClick={onDoubleClick}
        onPointerEnter={onPointerEnter}
        onPointerLeave={onPointerLeave}
        title={title}
        className={className}
        data-drag-kind={dataDragKind}
        data-drag-guid={dataDragGuid}
      >
        <Avatar
          className={cn("cursor-grab active:cursor-grabbing select-none", avatarClassName, isSelected && "ring-1 ring-[color:var(--active-base)]", isHovered && !isSelected && "ring-1 ring-[color:var(--hover-base)]")}
          style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}
        >
          <AvatarFallback className={cn("select-none", isSelected && "bg-[var(--active-base)] text-[var(--active-foreground)]", isHovered && !isSelected && "bg-[var(--hover-base)] text-foreground", !isSelected && !isHovered && "bg-muted")}>
            {content}
          </AvatarFallback>
        </Avatar>
      </div>
    );
  },
);
DraggableAvatar.displayName = "DraggableAvatar";

/**
 * Props interface for the TableAvatar component.
 **/
export interface TableAvatarProps {
  id?: string;
  icon?: string | React.ReactNode;
  name?: string;
  className?: string;
  isSelected?: boolean;
  isHovered?: boolean;
  style?: React.CSSProperties;
  fallbackStyle?: React.CSSProperties;
}

/**
 * Avatar component optimized for table row display.
 **/
export const TableAvatar: React.FC<TableAvatarProps> = ({ id, icon, name, className, isSelected, isHovered, style, fallbackStyle }) => {
  const normalizedName = (name ?? "").trim();
  const initials = normalizedName
    ? normalizedName
        .split(" ")
        .slice(0, 2)
        .map((word: string) => word.charAt(0))
        .join("")
        .toUpperCase()
        .substring(0, 2)
    : "";
  const isImageIcon = typeof icon === "string";
  const isReactIcon = icon && !isImageIcon;
  return (
    <Avatar id={id} style={style} className={cn("shrink-0", className, isSelected && "ring-1 ring-[color:var(--active-base)]", isHovered && "ring-1 ring-[color:var(--hover-base)]")}>
      {isImageIcon ? <AvatarImage src={icon} alt={normalizedName} /> : null}
      <AvatarFallback style={fallbackStyle} className={cn("text-xs", isSelected ? "bg-[color:var(--active-base)] text-[color:var(--active-foreground)]" : isHovered ? "bg-[color:var(--hover-base)]" : "")}>
        {isReactIcon ? icon : initials}
      </AvatarFallback>
    </Avatar>
  );
};
TableAvatar.displayName = "TableAvatar";

export { Avatar, AvatarFallback, AvatarImage };

// #endregion 📔Avatar

// #region 🎬Card
// Card container and grid layout for content blocks.
/**
 * Props interface for the Card component.
 *
 **/
export interface CardProps {
  title: string;
  icon?: string | LucideIcon;
  children: React.ReactNode;
  className?: string;
  contextMenu?: ContextMenuItem[];
}

/**
 * Content card with title, icon, and children.
 **/
export const Card: React.FC<CardProps> = ({ title, icon, children, className = "", contextMenu }) => {
  const IconComponent = typeof icon === "string" ? null : icon;
  return (
    <ContextMenu items={contextMenu}>
      <div className={`border p-single ${className}`}>
        <div className="flex items-start gap-tiny mb-single">
          {IconComponent && <IconComponent className="size-small flex-shrink-0 mt-0.5" />}
          {typeof icon === "string" && <span className="text-xl flex-shrink-0">{icon}</span>}
          <h3 className="font-semibold text-base">{title}</h3>
        </div>
        <div className="text-sm">{children}</div>
      </div>
    </ContextMenu>
  );
};

/**
 * Props interface for the CardGrid component.
 **/
export interface CardGridProps {
  stagger?: boolean;
  className?: string;
  children: React.ReactNode;
}

/** 📐 Lays out children in a responsive card grid (1-2 columns). */
export const CardGrid: React.FC<CardGridProps> = ({ stagger = false, children, className = "" }) => {
  return <div className={`grid grid-cols-1 md:grid-cols-2 gap-medium my-medium ${className}`}>{children}</div>;
};

// #endregion 🎬Card

// #region 🎹Spinner
// Animated loading spinner in small, medium, or large sizes.
// Consumers MUST choose an appropriate size for the context.

/**
 * Props interface for the Spinner component.
 **/
export interface SpinnerProps {
  size?: "small" | "medium" | "large";
  className?: string;
}

/**
 * Animated SVG loading spinner.
 **/
export const Spinner: React.FC<SpinnerProps> = ({ size = "medium", className = "" }) => {
  const sizeClass = size === "small" ? "size-small" : size === "large" ? "size-large" : "size-medium";
  return (
    <svg className={`animate-spin ${sizeClass} ${className}`} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
    </svg>
  );
};

// #endregion 🎹Spinner

// #region 🎍NotFound
// 404-style placeholder with icon, title, and back navigation.
// Consumers MUST provide a title for the error.

/**
 * Props interface for the NotFound component.
 **/
export interface NotFoundProps {
  title: string;
  description?: string;
  parentPath?: string;
  parentLabel?: string;
  icon?: React.ReactNode;
}

/**
 * Not-found placeholder page with navigation link.
 **/
export const NotFound: React.FC<NotFoundProps> = ({ title, description, parentPath, parentLabel, icon }) => {
  const navigate = useNavigate();
  return (
    <div className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-muted-foreground">{icon || <AlertCircleIcon className="size-huge" />}</div>
      <h1 className="text-xl font-semibold">{title}</h1>
      {description && <p className="text-muted-foreground max-w-md">{description}</p>}
      {parentPath && (
        <button onClick={() => navigate(parentPath)} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <ChevronLeftIcon className="size-small" />
          <span>{parentLabel || "Go back"}</span>
        </button>
      )}
    </div>
  );
};

// #endregion 🎍NotFound

// #region 🎺LoadingRow
// Skeleton loading row with pulsing icon and name.
// Consumers MUST provide a name for the placeholder.

/**
 * Props interface for the LoadingRow component.
/**
 **/
/**
 **/
export interface LoadingRowProps {
  name: string;
  icon?: React.ReactNode;
  className?: string;
}

/** LoadingRow holds the data fields for a LoadingRow record.
 **/
export const LoadingRow: React.FC<LoadingRowProps> = ({ name, icon, className = "" }) => {
  return (
    <div className={`flex items-center gap-single p-single opacity-50 pointer-events-none ${className}`}>
      {icon && <span className="shrink-0">{icon}</span>}
      <span className="flex-1 truncate">{name}</span>
    </div>
  );
};

// #endregion 🎺LoadingRow

// #region 🔓DiagramNode
// Individual diagram node element with selection and hover states.
// Consumers MUST provide content for the node.

/**
 * Props interface for the DiagramNode component.
 **/
export interface DiagramNodeProps {
  content: React.ReactNode;
  selected?: boolean;
  hovered?: boolean;
  isPlaceholder?: boolean;
  showTopHandle?: boolean;
  showBottomHandle?: boolean;
  className?: string;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
  onClick?: () => void;
  contextMenu?: ContextMenuItem[];
}

/**
 * Individual node element within a diagram graph.
 **/
export const DiagramNode: React.FC<DiagramNodeProps> = ({
  content,
  selected = false,
  hovered = false,
  isPlaceholder = false,
  showTopHandle = false,
  showBottomHandle = false,
  className = "",
  onMouseEnter,
  onMouseLeave,
  onClick,
  contextMenu,
}) => {
  return (
    <ContextMenu items={contextMenu}>
      <div
        className={`
        relative flex items-center justify-center
        size-large size-large rounded-full
        ${isPlaceholder ? "border-2 border-dashed" : "border-2 border-solid"}
        ${selected ? "ring-2 ring-[color:var(--active-base)]" : ""}
        ${hovered ? "ring-2 ring-[color:var(--hover-base)]" : ""}
        ${isPlaceholder ? "border-[color:var(--disabled-base)] bg-[color:var(--disabled-panel)]" : "border-[color:var(--foreground-panel)] bg-[color:var(--background-panel)]"}
        transition-all duration-150
        ${onClick ? "cursor-selectable" : "cursor-default"}
        ${className}
      `}
        onMouseEnter={onMouseEnter}
        onMouseLeave={onMouseLeave}
        onClick={onClick}
      >
        {showTopHandle && <Handle type="target" position={Position.Top as any} className="size-dot !bg-[color:var(--foreground-panel)] !border-[color:var(--background-panel)]" />}

        <div className="text-sm font-medium text-[color:var(--foreground-panel)] truncate px-single">{content}</div>

        {showBottomHandle && <Handle type="source" position={Position.Bottom as any} className="size-dot !bg-[color:var(--foreground-panel)] !border-[color:var(--background-panel)]" />}
      </div>
    </ContextMenu>
  );
};
/**
 * PlaceholderDiagramNode holds the data fields for a PlaceholderDiagramNode record.
 **/
export const PlaceholderDiagramNode: React.FC<{ id?: string; onClick?: () => void }> = ({ id = "diagram.placeholder", onClick }) => {
  return <DiagramNode content={useLabel(id)} isPlaceholder showTopHandle onClick={onClick} className="hover:border-[color:var(--hover-base)] hover:bg-[color:var(--hover-panel)]" />;
};

// #endregion 🔓DiagramNode

// #region 🔧HoverCard
// Hover-triggered card built on Radix primitives.
// Consumers MUST use HoverCardTrigger to activate.

/**
 * HoverCard holds the data fields for a HoverCard record.
 **/
function HoverCard({ ...props }: React.ComponentProps<typeof HoverCardPrimitive.Root>) {
  return <HoverCardPrimitive.Root data-slot="hover-card" {...props} />;
}

/**
 * HoverCardTrigger holds the data fields for a HoverCardTrigger record.
 **/
function HoverCardTrigger({ className, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Trigger>) {
  return <HoverCardPrimitive.Trigger data-slot="hover-card-trigger" className={cn(className)} {...props} />;
}

/**
 * HoverCardContent holds the data fields for a HoverCardContent record.
 **/
function HoverCardContent({ className, align = "center", sideOffset = 4, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Content>) {
  return (
    <HoverCardPrimitive.Portal data-slot="hover-card-portal">
      <HoverCardPrimitive.Content
        data-slot="hover-card-content"
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-temporary w-64 origin-(--radix-hover-card-content-transform-origin) border p-single outline-hidden",
          className,
        )}
        {...props}
      />
    </HoverCardPrimitive.Portal>
  );
}

export { HoverCard, HoverCardContent, HoverCardTrigger };

// #endregion 🔧HoverCard

// #region 🛒Icons
// Cursor icon component for collaborative pointer display.
// Consumers MUST provide position data for rendering.

/**
 * CursorProps holds the data fields for a CursorProps record.
 **/
interface CursorProps {
  color: string;
  x?: number;
  y?: number;
}

/**
 **/
const Cursor: React.FC<CursorProps> = ({ color, x = 0, y = 0 }) => {
  return (
    <svg
      style={{
        position: "absolute",
        left: 0,
        top: 0,
        transform: `translateX(${x}px) translateY(${y}px)`,
      }}
      width="24"
      height="36"
      viewBox="0 0 24 36"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M5.65376 12.3673H5.46026L5.31717 12.4976L0.500002 16.8829L0.500002 1.19841L11.7841 12.3673H5.65376Z" fill={color} />
    </svg>
  );
};

export { Cursor };

// #endregion 🛒Icons

// #region 🖲️Section
// Collapsible section container with heading and specificity.
// Consumers MUST provide a heading string.

/**
 * Props interface for the Section component.
 **/
export interface SectionProps {
  id?: string;
  title?: string;
  children: React.ReactNode;
  className?: string;
}

/**
 **/
const Section: React.FC<SectionProps> = ({ id, title, children, className = "" }) => {
  return (
    <section id={id} className={`mb-8 ${className}`}>
      {title && (
        <h2 className="text-2xl font-semibold mb-4" id={id}>
          {title}
        </h2>
      )}
      <div>{children}</div>
    </section>
  );
};

export { Section };

// #endregion 🖲️Section

// #region 🪬Steps
// Ordered step list container for tutorial or wizard flows.
// Consumers MUST provide step children in order.

/**
 * Props interface for the Steps component.
 **/
export interface StepsProps {
  children: React.ReactNode;
  className?: string;
}

/**
 * Ordered step list container rendering numbered children.
 **/
export const Steps: React.FC<StepsProps> = ({ children, className = "" }) => {
  return <ol className={`flex flex-col gap-medium ${className}`}>{children}</ol>;
};

// #endregion 🪬Steps

// #endregion 🏷️Display Components

// #region 🛒Input Components

// #region 🌩️ActionGroup
// Compact action button group with dropdown support.
// Consumers MUST provide action items for the group.

/**
 * actionGroupItemVariants holds the data fields for a actionGroupItemVariants record.
 **/
const actionGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center shrink-0 transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg]:size-tiny [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive overflow-hidden aspect-square p-single",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
        canvas: "hover:bg-hover-canvas",
        window: "hover:bg-hover-window",
        panel: "hover:bg-hover-panel",
        overlay: "hover:bg-hover-overlay",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      level: "base",
    },
  },
);

/**
 * ActionGroupContext holds the data fields for a ActionGroupContext record.
 **/
const ActionGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ActionGroupProps holds the data fields for a ActionGroupProps record.
 **/
interface ActionGroupProps extends Omit<React.ComponentProps<"div">, "children"> {
  children: React.ReactNode;
}

/**
 * ActionGroup holds the data fields for a ActionGroup record.
 **/
function ActionGroup({ className, children, ...props }: ActionGroupProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const divideClass = getLevelDivideElementClass(level);
  return (
    <div data-slot="action-group" data-detail-panel-control="fit" data-level={level} className={cn("group/action-group flex h-small items-center border divide-x overflow-hidden", borderClass, divideClass, className)} {...props}>
      <ActionGroupContext.Provider value={{ level }}>{children}</ActionGroupContext.Provider>
    </div>
  );
}

/**
 * ActionGroupItem holds the data fields for a ActionGroupItem record.
 **/
function ActionGroupItem({
  className,
  children,
  id,
  text,
  as: Component = "button",
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  text?: string;
  as?: "button" | "div";
}) {
  const context = reactHostPort.useContext(ActionGroupContext);
  const level = context.level ?? "base";
  const hasText = Boolean(text);

  const actionGroupItemElement = (
    <Component
      data-slot="action-group-item"
      id={id}
      type={Component === "button" ? "button" : undefined}
      role={Component === "div" && (props as any).onClick ? "button" : undefined}
      tabIndex={Component === "div" && (props as any).onClick ? 0 : undefined}
      data-level={context.level || level}
      className={cn(
        actionGroupItemVariants({
          level: context.level || level,
        }),
        "min-w-0 shrink-0 focus:z-panel focus-visible:z-panel",
        !id && "flex-1",
        hasText && "aspect-auto gap-single",
        className,
      )}
      {...(props as any)}
    >
      {children}
      {text && <span className="text-tiny whitespace-nowrap">{text}</span>}
    </Component>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{actionGroupItemElement}</TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return actionGroupItemElement;
}

/**
 * ActionDropdownOption holds the data fields for a ActionDropdownOption record.
 **/
interface ActionDropdownOption {
  value: string;
  icon: React.ReactNode;
  label?: string;
}

/**
 * ActionDropdownProps holds the data fields for a ActionDropdownProps record.
 **/
interface ActionDropdownProps extends Omit<React.ComponentProps<"button">, "children" | "id"> {
  id: string;
  options: ActionDropdownOption[];
  value: string;
  onValueChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
}

/**
 * ActionDropdown holds the data fields for a ActionDropdown record.
 **/
function ActionDropdown({ className, id, options, value, onValueChange, startTransaction, finalizeTransaction, ...props }: ActionDropdownProps) {
  const transaction = useTransaction();
  const [open, setOpen] = reactHostPort.useState(false);
  const level = useLevel();

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    const start = startTransaction ?? transaction?.start;
    const finalize = finalizeTransaction ?? transaction?.finalize;
    if (isOpen && start) start();
    setOpen(isOpen);
    if (!isOpen && finalize) finalize();
  };

  const handleSelect = (optionValue: string) => {
    if (onValueChange) onValueChange(optionValue);
    setOpen(false);
  };

  const buttonElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <ActionGroup id={id} className={className}>
          <ActionGroupItem id={id} {...props}>
            {selectedOption?.icon}
          </ActionGroupItem>
        </ActionGroup>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-single min-w-[120px]" align="start">
        <div className="flex flex-col">
          {options.map((option) => (
            <button
              key={option.value}
              onClick={() => handleSelect(option.value)}
              className={cn("flex items-center gap-single p-single text-xs cursor-selectable transition-colors", "hover:bg-hover-temporary outline-none focus-visible:bg-hover-temporary", value === option.value && "bg-active-temporary")}
            >
              <span className="flex items-center justify-center size-3">{option.icon}</span>
              {option.label && <span className="flex-1 text-left">{option.label}</span>}
              {value === option.value && <CheckIcon className="size-tiny ml-auto" />}
            </button>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  );

  return buttonElement;
}

/**
 * ActionProps holds the data fields for a ActionProps record.
 **/
interface ActionProps extends Omit<React.ComponentProps<"button">, "children"> {
  as?: "button" | "div";
  loading?: boolean;
  icon?: React.ReactNode;
  text?: string;
  id?: string;
}

/**
 * Action holds the data fields for a Action record.
 **/
function Action({ className, id, icon, text, as = "button", ...props }: ActionProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const Comp = as;
  const hasText = Boolean(text);

  const actionElement = (
    <Comp
      data-slot="action"
      type={Comp === "button" ? "button" : undefined}
      role={Comp === "div" && (props as any).onClick ? "button" : undefined}
      tabIndex={Comp === "div" && (props as any).onClick ? 0 : undefined}
      id={id}
      className={cn(
        "text-foreground inline-flex items-center justify-center shrink-0 transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg]:size-tiny [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive overflow-hidden aspect-square p-single h-medium border",
        hasText && "aspect-auto gap-single",
        level === "base" && "hover:bg-hover-base",
        level === "window" && "hover:bg-hover-window",
        level === "panel" && "hover:bg-hover-panel",
        level === "overlay" && "hover:bg-hover-overlay",
        level === "temporary" && "hover:bg-hover-temporary",
        borderClass,
        className,
      )}
      {...(props as any)}
    >
      {icon}
      {text && <span className="text-tiny whitespace-nowrap">{text}</span>}
    </Comp>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{actionElement}</TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return actionElement;
}

export { Action, ActionDropdown, ActionGroup, ActionGroupItem, actionGroupItemVariants };
export type { ActionDropdownOption, ActionDropdownProps, ActionProps };

// #endregion 🌩️ActionGroup

/**
 * buttonGroupItemVariants holds the data fields for a buttonGroupItemVariants record.
 **/
const buttonGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap h-medium aspect-square p-single overflow-hidden",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
        canvas: "hover:bg-hover-canvas",
        window: "hover:bg-hover-window",
        panel: "hover:bg-hover-panel",
        overlay: "hover:bg-hover-overlay",
        temporary: "hover:bg-hover-temporary",
      },
      variant: {
        default: "",
        ghost: "border-transparent bg-transparent",
        outline: "border border-element",
      },
    },
    defaultVariants: {
      level: "base",
      variant: "default",
    },
  },
);

/**
 * ButtonGroupContext holds the data fields for a ButtonGroupContext record.
 **/
const ButtonGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ButtonGroupProps holds the data fields for a ButtonGroupProps record.
 **/
interface ButtonGroupProps extends Omit<React.ComponentProps<"div">, "id"> {
  detailPanelWidthMode?: "fit" | "fill";
  id?: string;
  showLabel?: boolean;
  children: React.ReactNode;
}

/**
 * ButtonGroup holds the data fields for a ButtonGroup record.
 **/
function ButtonGroup({ className, detailPanelWidthMode = "fit", id, showLabel, children, ...props }: ButtonGroupProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const divideClass = getLevelDivideElementClass(level);
  const buttonGroupElement = (
    <div
      data-slot="button-group"
      data-detail-panel-control={detailPanelWidthMode}
      id={id}
      data-level={level}
      className={cn("group/button-group flex items-center border divide-x overflow-hidden h-medium", detailPanelWidthMode === "fill" ? "w-full min-w-0" : "w-fit shrink-0", borderClass, divideClass, className)}
      {...props}
    >
      {children}
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {buttonGroupElement}
      </Label>
    );
  }

  return buttonGroupElement;
}

/**
 * ButtonGroupItem holds the data fields for a ButtonGroupItem record.
 **/
function ButtonGroupItem({
  className,
  children,
  id,
  icon,
  text,
  asChild = false,
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  icon?: React.ReactNode;
  text?: string;
  asChild?: boolean;
}) {
  const context = reactHostPort.useContext(ButtonGroupContext);
  const level = context.level ?? "base";
  const Comp = asChild ? Slot : "button";

  const buttonGroupItemElement = (
    <Comp
      data-slot="button-group-item"
      id={id}
      data-level={context.level || level}
      className={cn(
        buttonGroupItemVariants({
          level: context.level || level,
        }),
        text ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel" : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel",
        text && "flex items-center gap-single py-single px-double w-auto aspect-auto",
        className,
      )}
      {...(props as any)}
    >
      {icon || children}
      {text && <span className="text-xs whitespace-nowrap">{text}</span>}
    </Comp>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{buttonGroupItemElement}</TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return buttonGroupItemElement;
}

/**
 * ButtonProps holds the data fields for a ButtonProps record.
 **/
type ButtonProps = React.ComponentProps<"button"> &
  Omit<VariantProps<typeof buttonGroupItemVariants>, "level"> & {
    asChild?: boolean;
    id?: string;
    icon?: React.ReactNode;
    text?: string;
    children?: React.ReactNode;
  };

/**
 * ButtonCycleItem holds the data fields for a ButtonCycleItem record.
 **/
interface ButtonCycleItem<T extends string> {
  value: T;
  label: string;
  icon?: React.ReactNode;
  text?: string;
  id?: string;
}

/**
 * ButtonCycleProps holds the data fields for a ButtonCycleProps record.
 **/
interface ButtonCycleProps<T extends string> extends Omit<React.ComponentProps<"button">, "children" | "id">, ElementProps {
  value?: T;
  onValueChange?: (value: T) => void;
  items: ButtonCycleItem<T>[];
  showLabel?: boolean;
}

/**
 **/
function Button({ className, asChild = false, id, icon, text, children, ...props }: ButtonProps) {
  const level = useLevel();
  return (
    <ButtonGroup className={className}>
      <ButtonGroupItem id={id} asChild={asChild} text={text} {...props}>
        {icon || children}
      </ButtonGroupItem>
    </ButtonGroup>
  );
}

/**
 * ButtonCycle holds the data fields for a ButtonCycle record.
 **/
function ButtonCycle<T extends string = string>({ className, id, showLabel, value, onValueChange, items, ...props }: ButtonCycleProps<T>) {
  const level = useLevel();
  const currentIndex = items.findIndex((item) => item.value === value);
  const currentItem = currentIndex >= 0 ? items[currentIndex] : items[0];

  const handleCycle = () => {
    const nextIndex = (currentIndex + 1) % items.length;
    if (onValueChange) onValueChange(items[nextIndex].value);
  };

  return (
    <ButtonGroup id={id} showLabel={showLabel} className={className}>
      <ButtonGroupItem id={id} onClick={handleCycle} text={currentItem?.label} {...props}>
        {currentItem?.icon}
      </ButtonGroupItem>
    </ButtonGroup>
  );
}

export { Button, ButtonCycle, ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
export type { ButtonCycleProps, ButtonProps };

// #region 📧Combobox
// Searchable dropdown with popover options list.
// Consumers MUST provide options and onValueChange handler.

/**
 * ComboboxOption holds the data fields for a ComboboxOption record.
 **/
interface ComboboxOption {
  value: string;
  label: string;
}

/**
 * ComboboxProps holds the data fields for a ComboboxProps record.
 **/
interface ComboboxProps extends ElementProps {
  options: ComboboxOption[];
  value?: string;
  placeholder?: string;
  placeholderId?: string;
  emptyMessage?: string;
  onValueChange?: (value: string) => void;
  className?: string;
  allowClear?: boolean;
  showLabel?: boolean;
}

/**
 * Searchable combobox dropdown with autocomplete filtering.
 **/
export const Combobox: React.FC<ComboboxProps> = ({ options, value = "", placeholder = "Select option...", placeholderId, emptyMessage = "No options found.", onValueChange, className, allowClear = false, showLabel, id }) => {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [open, setOpen] = reactHostPort.useState(false);
  const { t } = useTranslation();
  const computedPlaceholder = placeholderId ? useLabel(placeholderId) : placeholder;

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    setOpen(isOpen);
    if (isOpen) {
      transaction?.start?.();
    } else {
      transaction?.finalize?.();
    }
  };

  const handleSelect = (optionValue: string) => {
    if (allowClear && optionValue === value) {
      onValueChange?.("");
    } else {
      onValueChange?.(optionValue);
    }
    setOpen(false);
    transaction?.finalize?.();
  };

  const comboboxEmptyOpacity = isInPropertyValueColumn && !selectedOption && !open ? 0.6 : 1;

  const comboboxElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <ButtonGroup detailPanelWidthMode="fill" style={{ opacity: comboboxEmptyOpacity, transition: "opacity 150ms" }}>
          <ButtonGroupItem id={id} role="combobox" aria-expanded={open} className="w-full min-w-0 justify-between">
            {selectedOption ? selectedOption.label : computedPlaceholder}
            <ChevronsUpDownIcon className="ml-2 size-tiny shrink-0 opacity-50" />
          </ButtonGroupItem>
        </ButtonGroup>
      </PopoverTrigger>
      <PopoverContent className="w-full" align="start">
        <Command>
          <CommandInput placeholder="Search..." />
          <CommandList>
            <CommandEmpty>{emptyMessage}</CommandEmpty>
            <CommandGroup>
              {allowClear && value && (
                <CommandItem value="" onSelect={() => handleSelect("")}>
                  <div className="mr-2 size-tiny" />
                  <span className="text-muted-foreground italic">Clear selection</span>
                </CommandItem>
              )}
              {options.map((option) => (
                <CommandItem key={option.value} value={option.value} onSelect={() => handleSelect(option.value)}>
                  <CheckIcon className={cn("mr-2 size-small", value === option.value ? "opacity-100" : "opacity-0")} />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className={cn("h-medium", className)}>
        {comboboxElement}
      </Label>
    );
  }

  return comboboxElement;
};

// #endregion 📧Combobox

// #region 🩺Input
// Text input field with label, validation, and clear support.
// Consumers MUST provide an id for accessibility.

// #region 📨Input Collapse Helpers

const COLLAPSED_FIELD_ELLIPSIS = "...";
const collapsedFieldOverflowEpsilonPx = 0.5;
const collapsedFieldWhitespacePattern = /\s+/g;
const nonCollapsibleInputTypes = new Set(["button", "checkbox", "color", "file", "hidden", "image", "password", "radio", "range", "reset", "submit"]);
const stackedOverflowInputTypes = new Set(["email", "search", "tel", "text", "url"]);

interface FitCollapsedFieldTextOptions {
  value: string;
  maxWidth: number;
  ellipsis?: string;
  appendEllipsis?: boolean;
  measureText: (value: string) => number;
}

function normalizeCollapsedFieldText(value: string) {
  return value.replace(collapsedFieldWhitespacePattern, " ").trim();
}

function getCollapsedFieldGraphemes(value: string) {
  if (typeof Intl !== "undefined" && "Segmenter" in Intl) {
    return Array.from(new Intl.Segmenter(undefined, { granularity: "grapheme" }).segment(value), (segment) => segment.segment);
  }
  return Array.from(value);
}

function fitCollapsedFieldText({ value, maxWidth, ellipsis = COLLAPSED_FIELD_ELLIPSIS, appendEllipsis = true, measureText }: FitCollapsedFieldTextOptions) {
  const normalizedValue = normalizeCollapsedFieldText(value);
  if (!normalizedValue || maxWidth <= 0) {
    return normalizedValue;
  }
  if (measureText(normalizedValue) <= maxWidth) {
    return normalizedValue;
  }

  if (measureText(ellipsis) >= maxWidth) {
    return ellipsis;
  }

  const words = normalizedValue.split(" ");
  if (words.length > 1) {
    let low = 1;
    let high = words.length;
    let bestWordCount = 0;

    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      const prefix = words.slice(0, mid).join(" ");
      const candidate = appendEllipsis ? `${prefix}${ellipsis}` : prefix;
      if (measureText(candidate) <= maxWidth) {
        bestWordCount = mid;
        low = mid + 1;
      } else {
        high = mid - 1;
      }
    }

    if (bestWordCount > 0 && bestWordCount < words.length) {
      const prefix = words.slice(0, bestWordCount).join(" ");
      return appendEllipsis ? `${prefix}${ellipsis}` : prefix;
    }
  }

  const graphemes = getCollapsedFieldGraphemes(normalizedValue);
  let low = 1;
  let high = graphemes.length;
  let bestCharacterCount = 0;

  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    const prefix = graphemes.slice(0, mid).join("").trimEnd();
    const candidate = appendEllipsis ? `${prefix}${ellipsis}` : prefix;
    if (measureText(candidate) <= maxWidth) {
      bestCharacterCount = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  if (bestCharacterCount <= 0) {
    return appendEllipsis ? ellipsis : (graphemes[0] ?? "");
  }

  const prefix = graphemes.slice(0, bestCharacterCount).join("").trimEnd();
  return appendEllipsis ? `${prefix}${ellipsis}` : prefix;
}

function isCollapsibleInputType(type?: string) {
  return !type || !nonCollapsibleInputTypes.has(type);
}

function isStackedOverflowInputType(type?: string) {
  return !type || stackedOverflowInputTypes.has(type);
}

interface ResolveCollapsedFieldDisplayStateOptions {
  allowStackedOverflow?: boolean;
  value: string;
  maxWidth: number;
  measureText: (value: string) => number;
}

interface CollapsedFieldDisplayState {
  value: string;
  normalizedValue: string;
  isOverflowing: boolean;
  layoutKind: "single-line" | "stacked-overflow";
}

function resolveCollapsedFieldDisplayState({ allowStackedOverflow = false, value, maxWidth, measureText }: ResolveCollapsedFieldDisplayStateOptions): CollapsedFieldDisplayState {
  const normalizedValue = normalizeCollapsedFieldText(value);
  if (!normalizedValue || maxWidth <= 0) {
    return {
      value: normalizedValue,
      normalizedValue,
      isOverflowing: false,
      layoutKind: "single-line",
    };
  }

  const measuredValueWidth = measureText(normalizedValue);
  const isOverflowing = measuredValueWidth > maxWidth + collapsedFieldOverflowEpsilonPx;
  if (!isOverflowing) {
    return {
      value: normalizedValue,
      normalizedValue,
      isOverflowing: false,
      layoutKind: "single-line",
    };
  }

  const collapsedValue = fitCollapsedFieldText({ value: normalizedValue, maxWidth, appendEllipsis: !allowStackedOverflow, measureText });

  return {
    value: collapsedValue,
    normalizedValue,
    isOverflowing,
    layoutKind: allowStackedOverflow && isOverflowing ? "stacked-overflow" : "single-line",
  };
}

interface CollapsedFieldDisplayProps {
  allowStackedOverflow?: boolean;
  className?: string;
  disabled?: boolean;
  id?: string;
  mixed?: boolean;
  onActivate: () => void;
  placeholder?: string;
  slot: "input" | "textarea";
  value: string;
}

function CollapsedFieldDisplay({ allowStackedOverflow = false, className, disabled, id, mixed, onActivate, placeholder, slot, value }: CollapsedFieldDisplayProps) {
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const displayRef = reactHostPort.useRef<HTMLDivElement>(null);
  const lineRef = reactHostPort.useRef<HTMLSpanElement>(null);
  const normalizedValue = reactHostPort.useMemo(() => normalizeCollapsedFieldText(value), [value]);
  const stackedOverflowEnabled = isInPropertyValueColumn && allowStackedOverflow;
  const [displayState, setDisplayState] = reactHostPort.useState<CollapsedFieldDisplayState>({
    value: normalizedValue,
    normalizedValue,
    isOverflowing: false,
    layoutKind: "single-line",
  });

  const updateCollapsedValue = reactHostPort.useCallback(() => {
    const element = displayRef.current;
    const lineElement = lineRef.current;
    if (!element || !lineElement) {
      return;
    }
    if (!normalizedValue) {
      setDisplayState({
        value: "",
        normalizedValue,
        isOverflowing: false,
        layoutKind: "single-line",
      });
      return;
    }

    const computedStyle = window.getComputedStyle(element);
    const maxWidth = lineElement.clientWidth;
    if (maxWidth <= 0) {
      setDisplayState({
        value: normalizedValue,
        normalizedValue,
        isOverflowing: false,
        layoutKind: "single-line",
      });
      return;
    }

    const measurementElement = document.createElement("span");
    measurementElement.style.position = "absolute";
    measurementElement.style.visibility = "hidden";
    measurementElement.style.pointerEvents = "none";
    measurementElement.style.whiteSpace = "nowrap";
    measurementElement.style.font = computedStyle.font || `${computedStyle.fontStyle} ${computedStyle.fontVariant} ${computedStyle.fontWeight} ${computedStyle.fontSize} / ${computedStyle.lineHeight} ${computedStyle.fontFamily}`;
    measurementElement.style.letterSpacing = computedStyle.letterSpacing;
    measurementElement.style.textTransform = computedStyle.textTransform;
    measurementElement.style.textRendering = computedStyle.textRendering;
    document.body.appendChild(measurementElement);

    const measureText = (candidate: string) => {
      measurementElement.textContent = candidate;
      return measurementElement.getBoundingClientRect().width;
    };

    const nextState = resolveCollapsedFieldDisplayState({ allowStackedOverflow: stackedOverflowEnabled, value: normalizedValue, maxWidth, measureText });
    measurementElement.remove();

    setDisplayState((previousState) =>
      previousState.value === nextState.value && previousState.normalizedValue === nextState.normalizedValue && previousState.isOverflowing === nextState.isOverflowing && previousState.layoutKind === nextState.layoutKind ? previousState : nextState,
    );
  }, [normalizedValue, stackedOverflowEnabled]);

  reactHostPort.useEffect(() => {
    updateCollapsedValue();
  }, [updateCollapsedValue]);

  reactHostPort.useEffect(() => {
    const fontSet = document.fonts;
    if (!fontSet?.ready) {
      return;
    }

    let isCancelled = false;
    void fontSet.ready.then(() => {
      if (!isCancelled) {
        updateCollapsedValue();
      }
    });

    return () => {
      isCancelled = true;
    };
  }, [updateCollapsedValue]);

  reactHostPort.useEffect(() => {
    const element = displayRef.current;
    if (!element || typeof ResizeObserver === "undefined") {
      return;
    }
    const resizeObserver = new ResizeObserver(() => updateCollapsedValue());
    resizeObserver.observe(element);
    return () => resizeObserver.disconnect();
  }, [updateCollapsedValue]);

  const activate = () => {
    if (!disabled) {
      onActivate();
    }
  };

  const showStackedOverflow = stackedOverflowEnabled && displayState.layoutKind === "stacked-overflow";

  return (
    <div
      ref={displayRef}
      data-slot={slot}
      data-collapsed="true"
      data-overflowing={displayState.isOverflowing ? "true" : undefined}
      data-overflow-layout={showStackedOverflow ? "stacked" : "single-line"}
      id={id}
      className={cn(
        "text-foreground flex w-full min-w-0 overflow-hidden border bg-transparent text-base transition-[color,border-color] outline-none md:text-sm",
        showStackedOverflow ? "h-auto min-h-0 flex-col px-single" : "h-medium items-center px-single whitespace-nowrap",
        "aria-invalid:border-destructive flex-1 cursor-text",
        disabled && "cursor-not-allowed opacity-50",
        mixed && !displayState.value && "italic text-muted-foreground/70",
        className,
      )}
      tabIndex={disabled ? -1 : 0}
      role="textbox"
      aria-readonly="true"
      aria-disabled={disabled ? "true" : undefined}
      onClick={activate}
      onFocus={activate}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          activate();
        }
      }}
    >
      <span ref={lineRef} data-slot="collapsed-field-line" className={cn("flex min-w-0 overflow-hidden whitespace-nowrap", showStackedOverflow ? "h-medium w-full items-center" : "w-full items-center")}>
        {displayState.value ? (
          <span className={cn("block min-w-0 overflow-hidden whitespace-nowrap", !showStackedOverflow && "text-ellipsis")}>{displayState.value}</span>
        ) : (
          <span className={cn("block min-w-0 truncate", mixed ? "italic text-muted-foreground/70" : "text-muted-foreground")}>{placeholder}</span>
        )}
      </span>
      {showStackedOverflow ? (
        <span data-slot="collapsed-field-overflow" aria-hidden="true" className="flex h-[10px] min-w-0 items-center justify-center overflow-hidden leading-none">
          <span data-slot="collapsed-field-indicator" className="inline-flex items-center justify-center text-muted-foreground/75 leading-none">
            <ChevronDownIcon data-slot="collapsed-field-indicator-chevron" className="size-[10px] shrink-0 stroke-[2.5]" />
          </span>
        </span>
      ) : null}
    </div>
  );
}

// #endregion 📨Input Collapse Helpers

/**
 * InputProps holds the data fields for a InputProps record.
 **/
interface InputProps extends Omit<React.ComponentProps<"input">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onLazyChange?: (value: string) => void;
  interactionId?: string;
  placeholderId?: string;
  showLabel?: boolean;
  mixed?: boolean;
}

/**
 * Input holds the data fields for a Input record.
 **/
function Input({ className, type, lazy, value: externalValue, onChange, onLazyChange, interactionId, id, placeholderId, placeholder, showLabel, mixed, ...props }: InputProps) {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [localValue, setLocalValue] = reactHostPort.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isFocused, setIsFocused] = reactHostPort.useState(false);
  const inputRef = reactHostPort.useRef<HTMLInputElement>(null);
  /** @emoji 🧾 Enter key already runs {@link onLazyChange} + blur; skip duplicate commit on the subsequent blur event. */
  const skipLazyBlurCommitRef = reactHostPort.useRef(false);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const placeholderLabel = useLabel(placeholderId || "");
  const mixedLabel = useLabel("semio.sketchpad.common.mixedValues");
  const computedPlaceholder = mixed ? mixedLabel || "—" : placeholderId ? placeholderLabel : placeholder;

  reactHostPort.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  reactHostPort.useEffect(() => {
    if (isFocused && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isFocused]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsFocused(true);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (lazy) {
      setIsEditing(true);
      transaction?.start?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    setIsFocused(false);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (lazy) {
      setIsEditing(false);
      if (skipLazyBlurCommitRef.current) {
        skipLazyBlurCommitRef.current = false;
        props.onBlur?.(e);
        return;
      }
      onLazyChange?.(localValue);
      transaction?.finalize?.();
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (lazy) {
      if (e.key === "Enter") {
        if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
        setIsEditing(false);
        skipLazyBlurCommitRef.current = true;
        onLazyChange?.(localValue);
        transaction?.finalize?.();
        (e.target as HTMLInputElement).blur();
      } else if (e.key === "Escape") {
        if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
        setIsEditing(false);
        setLocalValue(externalValue?.toString() || "");
        transaction?.abort?.();
        (e.target as HTMLInputElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const inputValue = lazy ? localValue : externalValue;

  const activeInteraction = useActiveInteraction();
  const isInteracting = interactionId && activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;
  const inputDisplayValue = inputValue?.toString() || "";
  const showCollapsedDisplay = !!showLabel && !isFocused && isCollapsibleInputType(type);
  const allowStackedOverflow = isStackedOverflowInputType(type);

  const inputEmptyOpacity = isInPropertyValueColumn && !inputDisplayValue && !isFocused ? 0.6 : 1;
  const inputFinalOpacity = shouldFade ? 0 : inputEmptyOpacity;

  const inputElement = (
    <div data-slot="input-root" data-detail-panel-control="fill" className="flex min-w-0 w-full flex-1 items-stretch" style={{ opacity: inputFinalOpacity, transition: "opacity 150ms" }}>
      {showCollapsedDisplay ? (
        <CollapsedFieldDisplay
          allowStackedOverflow={allowStackedOverflow}
          className={className}
          disabled={props.disabled}
          id={id}
          mixed={mixed}
          onActivate={() => setIsFocused(true)}
          placeholder={computedPlaceholder}
          slot="input"
          value={mixed && !inputDisplayValue ? "" : inputDisplayValue}
        />
      ) : (
        <input
          ref={inputRef}
          type={type}
          data-slot="input"
          data-mixed={mixed ? "true" : undefined}
          id={id}
          className={cn(
            "file:text-foreground placeholder:text-muted-foreground text-foreground flex h-medium w-full min-w-0 border bg-transparent p-single text-base transition-[color,border-color] outline-none file:inline-flex file:h-medium file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
            "focus-visible:border-accent",
            "aria-invalid:ring-destructive/20 aria-invalid:border-destructive flex-1",
            mixed && "placeholder:italic placeholder:text-muted-foreground/70",
            type === "number" && "[&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]",
            className,
          )}
          value={mixed && !isFocused && !inputValue ? "" : inputValue}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          placeholder={computedPlaceholder}
          {...props}
        />
      )}
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {inputElement}
      </Label>
    );
  }

  return inputElement;
}

export { Input };

// #endregion 🩺Input

// #region 🔎Select
// Dropdown select built on Radix primitives.
// Consumers MUST use SelectItem children for options.

/**
 * Select holds the data fields for a Select record.
 **/
function Select({ id, showLabel, children, value, defaultValue, onOpenChange, ...props }: React.ComponentProps<typeof SelectPrimitive.Root> & ElementProps & { showLabel?: boolean }) {
  const transaction = useTransaction();
  const fallbackValue = reactHostPort.useMemo(() => {
    const findValue = (nodes: React.ReactNode[]): string | undefined => {
      for (const node of nodes) {
        if (!React.isValidElement(node)) {
          continue;
        }
        const nodeProps = node.props as { "data-slot"?: string; value?: string; children?: React.ReactNode };
        if ((node.type === SelectPrimitive.Item || nodeProps["data-slot"] === "select-item") && nodeProps.value !== undefined) {
          return nodeProps.value as string;
        }
        const nested = React.Children.toArray(nodeProps.children);
        if (nested.length) {
          const nestedValue = findValue(nested);
          if (nestedValue !== undefined) {
            return nestedValue;
          }
        }
      }
      return undefined;
    };
    return findValue(React.Children.toArray(children));
  }, [children]);

  const handleOpenChange = (open: boolean) => {
    if (open) {
      transaction?.start?.();
    } else {
      transaction?.finalize?.();
    }
    onOpenChange?.(open);
  };

  const selectElement = (
    <SelectPrimitive.Root
      onOpenChange={handleOpenChange}
      data-slot="select"
      {...(value !== null && value !== undefined ? { value } : defaultValue !== null && defaultValue !== undefined ? { defaultValue } : fallbackValue !== undefined ? { defaultValue: fallbackValue } : {})}
      {...props}
    >
      {children}
    </SelectPrimitive.Root>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {selectElement}
      </Label>
    );
  }

  return selectElement;
}

/**
 * SelectGroup holds the data fields for a SelectGroup record.
 **/
function SelectGroup({ ...props }: React.ComponentProps<typeof SelectPrimitive.Group>) {
  return <SelectPrimitive.Group data-slot="select-group" {...props} />;
}

/**
 * SelectValue holds the data fields for a SelectValue record.
 **/
function SelectValue({ ...props }: React.ComponentProps<typeof SelectPrimitive.Value>) {
  return <SelectPrimitive.Value data-slot="select-value" {...props} />;
}

/**
 * SelectTrigger holds the data fields for a SelectTrigger record.
 **/
function SelectTrigger({
  className,
  size = "default",
  children,
  id,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Trigger> & {
  size?: "sm" | "default";
  id?: string;
}) {
  const level = useLevel();
  const hoverClass = getLevelHoverClass(level);

  return (
    <SelectPrimitive.Trigger
      data-slot="select-trigger"
      data-detail-panel-control="fill"
      id={id}
      data-size={size}
      className={cn(
        "border-input data-[placeholder]:text-muted-foreground [&_svg:not([class*='text-'])]:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive flex w-fit items-center justify-between gap-single border bg-transparent px-tiny py-single text-sm whitespace-nowrap transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 h-medium *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-single [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny cursor-foldable",
        hoverClass,
        className,
      )}
      {...props}
    >
      {children as React.ReactNode}
      <SelectPrimitive.Icon asChild>
        <ChevronDownIconAlt className="size-small opacity-50" />
      </SelectPrimitive.Icon>
    </SelectPrimitive.Trigger>
  );
}

/**
 * SelectContent holds the data fields for a SelectContent record.
 **/
function SelectContent({ className, children, position = "popper", ...props }: React.ComponentProps<typeof SelectPrimitive.Content>) {
  return (
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        data-slot="select-content"
        className={cn(
          "bg-transparent backdrop-blur-sm text-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 relative z-temporary max-h-(--radix-select-content-available-height) min-w-32 origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto border",
          position === "popper" && "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1",
          className,
        )}
        position={position}
        {...props}
      >
        <SelectScrollUpButton />
        <SelectPrimitive.Viewport className={cn("p-single", position === "popper" && "h-[var(--radix-select-trigger-height)] w-full min-w-[var(--radix-select-trigger-width)] scroll-my-single")}>{children}</SelectPrimitive.Viewport>
        <SelectScrollDownButton />
      </SelectPrimitive.Content>
    </SelectPrimitive.Portal>
  );
}

/**
 * SelectLabel holds the data fields for a SelectLabel record.
 **/
function SelectLabel({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Label>) {
  return <SelectPrimitive.Label data-slot="select-label" className={cn("text-muted-foreground p-single text-xs", className)} {...props} />;
}

/**
 * SelectItem holds the data fields for a SelectItem record.
 **/
function SelectItem({ className, children, id, ...props }: React.ComponentProps<typeof SelectPrimitive.Item> & { id?: string }) {
  return (
    <SelectPrimitive.Item
      data-slot="select-item"
      id={id}
      className={cn(
        "focus:bg-hover-temporary focus:text-foreground [&_svg:not([class*='text-'])]:text-muted-foreground relative flex w-full items-center gap-single rounded-sm py-single pr-medium pl-single text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-tiny *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-single",
        "cursor-selectable",
        className,
      )}
      {...props}
    >
      <span className="absolute right-2 flex size-tiny.5 items-center justify-center">
        <SelectPrimitive.ItemIndicator>
          <CheckIconAlt className="size-tiny" />
        </SelectPrimitive.ItemIndicator>
      </span>
      <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
    </SelectPrimitive.Item>
  );
}

/**
 * SelectSeparator holds the data fields for a SelectSeparator record.
 **/
function SelectSeparator({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Separator>) {
  return <SelectPrimitive.Separator data-slot="select-separator" className={cn("bg-border pointer-events-none -mx-single my-single h-px", className)} {...props} />;
}

/**
 * SelectScrollUpButton holds the data fields for a SelectScrollUpButton record.
 **/
function SelectScrollUpButton({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.ScrollUpButton>) {
  return (
    <SelectPrimitive.ScrollUpButton data-slot="select-scroll-up-button" className={cn("flex cursor-default items-center justify-center py-single", className)} {...props}>
      <ChevronUpIcon className="size-tiny" />
    </SelectPrimitive.ScrollUpButton>
  );
}

/**
 * SelectScrollDownButton holds the data fields for a SelectScrollDownButton record.
 **/
function SelectScrollDownButton({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.ScrollDownButton>) {
  return (
    <SelectPrimitive.ScrollDownButton data-slot="select-scroll-down-button" className={cn("flex cursor-default items-center justify-center py-single", className)} {...props}>
      <ChevronDownIconAlt className="size-tiny" />
    </SelectPrimitive.ScrollDownButton>
  );
}

/**
 * ChevronUpIcon holds the data fields for a ChevronUpIcon record.
 **/
const ChevronUpIcon = ChevronDownIconAlt;

export { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton, SelectSeparator, SelectTrigger, SelectValue };

// #endregion 🔎Select

// #region 🏩Slider
// Range slider built on Radix primitives.
// Consumers MUST provide min and max values.

/**
 * Slider holds the data fields for a Slider record.
 **/
function Slider({
  className,
  defaultValue,
  value,
  min = 0,
  max = 100,
  showLabel,
  onValueChange,
  onPointerDown,
  onPointerUp,
  onPointerCancel,
  interactionId,
  id,
  snapValues,
  ...props
}: React.ComponentProps<typeof SliderPrimitive.Root> &
  ElementProps & {
    showLabel?: boolean;
    onPointerDown?: () => void;
    onPointerUp?: () => void;
    onPointerCancel?: () => void;
    interactionId?: string;
    snapValues?: number[];
  }) {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isSliding, setIsSliding] = reactHostPort.useState(false);
  const [editValue, setEditValue] = reactHostPort.useState("");
  const [hasBeenEdited, setHasBeenEdited] = reactHostPort.useState(false);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const activeInteraction = useActiveInteraction();
  const isInteracting = interactionId && activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const _values = reactHostPort.useMemo(() => (Array.isArray(value) ? value : Array.isArray(defaultValue) ? defaultValue : [min, max]), [value, defaultValue, min, max]);

  const displayValue = _values[0] ?? min;

  const findNearestSnapValue = reactHostPort.useCallback(
    (val: number): number => {
      if (!snapValues || snapValues.length === 0) return val;
      let nearest = snapValues[0];
      let minDistance = Math.abs(val - nearest);
      for (const snapValue of snapValues) {
        const distance = Math.abs(val - snapValue);
        if (distance < minDistance) {
          minDistance = distance;
          nearest = snapValue;
        }
      }
      return nearest;
    },
    [snapValues],
  );

  const handleValueChange = reactHostPort.useCallback(
    (values: number[]) => {
      if (snapValues && snapValues.length > 0) {
        const snappedValues = values.map(findNearestSnapValue);
        onValueChange?.(snappedValues);
      } else {
        onValueChange?.(values);
      }
    },
    [snapValues, findNearestSnapValue, onValueChange],
  );

  const handleValueClick = () => {
    if (!hasBeenEdited) setHasBeenEdited(true);
    setEditValue(displayValue.toString());
    setIsEditing(true);
    transaction?.start?.();
  };

  const handleEditKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      const newValue = parseFloat(editValue);
      if (!isNaN(newValue) && newValue >= min && newValue <= max) {
        handleValueChange([newValue]);
      }
      setIsEditing(false);
      transaction?.finalize?.();
    } else if (e.key === "Escape") {
      setIsEditing(false);
      transaction?.abort?.();
    }
  };

  const handleEditBlur = () => {
    setIsEditing(false);
    transaction?.finalize?.();
  };

  const handlePointerDown = (e: React.PointerEvent) => {
    if (!hasBeenEdited) setHasBeenEdited(true);
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (!isSliding) {
      setIsSliding(true);
      transaction?.start?.();
    }
    onPointerDown?.();
  };

  const handlePointerUp = (e: React.PointerEvent) => {
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isSliding) {
      setIsSliding(false);
      transaction?.finalize?.();
    }
    onPointerUp?.();
  };

  const handlePointerCancel = (e: React.PointerEvent) => {
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isSliding) {
      setIsSliding(false);
      transaction?.abort?.();
    }
    onPointerCancel?.();
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowLeft" || e.key === "ArrowRight" || e.key === "ArrowUp" || e.key === "ArrowDown") {
      if (!isSliding) {
        setIsSliding(true);
        transaction?.start?.();
      }
    }
  };

  const handleKeyUp = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowLeft" || e.key === "ArrowRight" || e.key === "ArrowUp" || e.key === "ArrowDown") {
      if (isSliding) {
        setIsSliding(false);
        transaction?.finalize?.();
      }
    } else if (e.key === "Escape") {
      if (isSliding) {
        setIsSliding(false);
        transaction?.abort?.();
      }
    }
  };

  const sliderElement = (
    <SliderPrimitive.Root
      data-slot="slider"
      id={id}
      defaultValue={defaultValue}
      value={value}
      min={min}
      max={max}
      onValueChange={handleValueChange}
      onPointerDown={handlePointerDown}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerCancel}
      onKeyDown={handleKeyDown}
      onKeyUp={handleKeyUp}
      className={cn(
        "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
      )}
      {...props}
    >
      <SliderPrimitive.Track
        data-slot="slider-track"
        className={cn("bg-muted relative grow overflow-hidden rounded-full data-[orientation=horizontal]:h-single data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-single")}
      >
        <SliderPrimitive.Range data-slot="slider-range" className={cn("bg-foreground absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full")} />
      </SliderPrimitive.Track>
      {Array.from({ length: _values.length }, (_, index) => (
        <SliderPrimitive.Thumb
          data-slot="slider-thumb"
          key={index}
          className="border-foreground bg-foreground ring-ring/50 block size-small shrink-0 rounded-full border transition-colors focus-visible:bg-accent focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 active:bg-accent"
        />
      ))}
    </SliderPrimitive.Root>
  );

  const wrappedSlider = (
    <Tooltip>
      <TooltipTrigger asChild>{sliderElement}</TooltipTrigger>
      <TooltipContent>
        <DescriptionTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );

  const sliderContent = (
    <div data-slot="slider-content" data-detail-panel-control="fill" style={{ opacity: shouldFade ? 0 : isInPropertyValueColumn && !hasBeenEdited ? 0.6 : 1, transition: "opacity 150ms" }} className="flex-1 min-w-0">
      <div data-slot="slider-row" className="grid h-[22px] grid-cols-[minmax(0,1fr)_28px] items-center gap-x-[8px]">
        <div data-slot="slider-track-cell" className="min-w-0">
          {wrappedSlider}
        </div>
        {isEditing ? (
          <Input
            type="number"
            value={editValue}
            onChange={(e) => setEditValue(e.target.value)}
            onKeyDown={handleEditKeyDown}
            onBlur={handleEditBlur}
            className="w-[28px] min-w-[28px] border-0 px-0 text-right text-xs"
            min={min}
            max={max}
            autoFocus
            id={id}
          />
        ) : (
          <span data-slot="slider-value" className="w-[28px] text-right text-xs leading-none select-none" role="button" onDoubleClick={handleValueClick} title="Double-click to edit">
            {displayValue}
          </span>
        )}
      </div>
    </div>
  );

  if (showLabel) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className={className}>
        {sliderContent}
      </Label>
    );
  }

  return sliderContent;
}

export { Slider };

// #endregion 🏩Slider

// #region 🏬Stepper
// Numeric stepper with increment/decrement and drag adjustment.
// Consumers MUST provide min and max bounds.

/**
 * StepperProps holds the data fields for a StepperProps record.
 **/
interface StepperProps extends ElementProps {
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  onChange?: (value: number) => void;
  onPointerDown?: () => void;
  onPointerUp?: () => void;
  onPointerCancel?: () => void;
  interactionId?: string;
  showLabel?: boolean;
}

/**
 * Numeric stepper with increment, decrement, and drag-to-adjust.
 **/
export const Stepper: React.FC<StepperProps> = ({ value, defaultValue = 0, min, max, step = 1, onChange, onPointerDown, onPointerUp, onPointerCancel, interactionId, id, showLabel }) => {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const [internalValue, setInternalValue] = reactHostPort.useState(value ?? defaultValue);
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [hasBeenEdited, setHasBeenEdited] = reactHostPort.useState(false);
  const intervalRef = reactHostPort.useRef<NodeJS.Timeout | null>(null);
  const timeoutRef = reactHostPort.useRef<NodeJS.Timeout | null>(null);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const activeInteraction = useActiveInteraction();

  reactHostPort.useEffect(() => {
    if (value !== undefined) {
      setInternalValue(value);
    }
  }, [value]);

  const clampValue = reactHostPort.useCallback(
    (val: number): number => {
      let clampedValue = val;
      if (min !== undefined) clampedValue = Math.max(clampedValue, min);
      if (max !== undefined) clampedValue = Math.min(clampedValue, max);
      return clampedValue;
    },
    [min, max],
  );

  const updateValue = reactHostPort.useCallback(
    (newValue: number) => {
      const clampedValue = clampValue(newValue);
      setInternalValue(clampedValue);
      onChange?.(clampedValue);
    },
    [clampValue, onChange],
  );

  const startContinuousChange = reactHostPort.useCallback(
    (increment: number) => {
      if (intervalRef.current) clearInterval(intervalRef.current);
      if (timeoutRef.current) clearTimeout(timeoutRef.current);

      timeoutRef.current = setTimeout(() => {
        intervalRef.current = setInterval(() => {
          setInternalValue((prev) => {
            const newValue = clampValue(prev + increment);
            return newValue;
          });
        }, 100);
      }, 500);
    },
    [clampValue, onChange],
  );

  const stopContinuousChange = reactHostPort.useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  reactHostPort.useEffect(() => {
    return () => {
      stopContinuousChange();
    };
  }, [stopContinuousChange]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = parseFloat(e.target.value);
    if (!isNaN(newValue)) {
      updateValue(newValue);
    }
  };

  const handleStepUp = () => {
    updateValue(internalValue + step);
  };

  const handleStepDown = () => {
    updateValue(internalValue - step);
  };

  const handleMouseDown = (increment: number) => {
    return () => {
      if (!hasBeenEdited) setHasBeenEdited(true);
      if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
      if (!isEditing) {
        setIsEditing(true);
        transaction?.start?.();
      }
      onPointerDown?.();
      if (increment > 0) {
        handleStepUp();
      } else {
        handleStepDown();
      }
      startContinuousChange(increment);
    };
  };

  const handleMouseUp = () => {
    stopContinuousChange();
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isEditing) {
      setIsEditing(false);
      transaction?.finalize?.();
    }
    onPointerUp?.();
  };

  const handleMouseLeave = () => {
    stopContinuousChange();
    if (interactionId && setActiveInteraction) setActiveInteraction(id, undefined);
    if (isEditing) {
      setIsEditing(false);
      transaction?.finalize?.();
    }
    onPointerCancel?.();
  };

  const canStepDown = min === undefined || internalValue > min;
  const canStepUp = max === undefined || internalValue < max;
  const displayedValue = Number.isFinite(internalValue) ? internalValue : defaultValue;

  const labelElementId = id ? `${id.split(".").join("-")}-label` : undefined;

  const stepperEmptyOpacity = isInPropertyValueColumn && value === undefined && !hasBeenEdited ? 0.6 : 1;

  const stepperElement = (
    <div
      data-slot="stepper-group"
      data-detail-panel-control="fill"
      className={cn("flex h-[22px] w-full min-w-0 items-stretch overflow-hidden rounded-[3px] border transition-[border-color] focus-within:border-accent", borderClass)}
      style={{ opacity: stepperEmptyOpacity, transition: "opacity 150ms" }}
    >
      <button
        data-slot="stepper-minus"
        type="button"
        onMouseDown={handleMouseDown(-step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(-step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepDown}
        className={cn("flex h-[22px] w-[22px] shrink-0 cursor-pointer items-center justify-center border-r hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted", borderClass)}
      >
        <RemoveIcon className="size-tiny" />
      </button>
      <input
        type="number"
        data-slot="input"
        data-stepper-input="true"
        value={displayedValue}
        onChange={handleInputChange}
        onFocus={() => {
          if (!hasBeenEdited) setHasBeenEdited(true);
          if (!isEditing) {
            setIsEditing(true);
            transaction?.start?.();
          }
          onPointerDown?.();
        }}
        onBlur={() => {
          if (isEditing) {
            setIsEditing(false);
            transaction?.finalize?.();
          }
          onPointerUp?.();
        }}
        onKeyDown={(e) => {
          if (e.key === "ArrowUp" || e.key === "ArrowDown") {
            e.preventDefault();
            if (!isEditing) {
              setIsEditing(true);
              transaction?.start?.();
            }
            if (e.key === "ArrowUp") {
              handleStepUp();
            } else {
              handleStepDown();
            }
          } else if (e.key === "Escape") {
            if (isEditing) {
              setIsEditing(false);
              setInternalValue(value ?? defaultValue);
              transaction?.abort?.();
              (e.target as HTMLInputElement).blur();
            }
          } else if (e.key === "Enter") {
            if (isEditing) {
              setIsEditing(false);
              transaction?.finalize?.();
              (e.target as HTMLInputElement).blur();
            }
          }
        }}
        className="file:text-foreground placeholder:text-muted-foreground text-foreground flex h-[22px] min-w-0 flex-1 border-0 bg-transparent px-[6px] text-center text-base transition-[color,border-color] outline-none file:inline-flex file:h-[22px] file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 focus-visible:border-0 md:text-sm [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]"
        step={step}
        min={min}
        max={max}
        aria-labelledby={labelElementId}
        id={id}
        inputMode="decimal"
      />
      <button
        data-slot="stepper-plus"
        type="button"
        onMouseDown={handleMouseDown(step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepUp}
        className={cn("flex h-[22px] w-[22px] shrink-0 cursor-pointer items-center justify-center border-l hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted", borderClass)}
      >
        <AddIcon className="size-[10px]" />
      </button>
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={labelElementId}>
        {stepperElement}
      </Label>
    );
  }

  return stepperElement;
};

// #endregion 🏬Stepper

// #region 🎏Textarea
// Multi-line text input with label and validation.
// Consumers MUST provide an id for the field.

/**
 * TextareaProps holds the data fields for a TextareaProps record.
 **/
interface TextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onLazyChange?: (value: string) => void;
  showLabel?: boolean;
  placeholderId?: string;
  readOnly?: boolean;
  mixed?: boolean;
}

/**
 **/
function Textarea({ className, lazy, value: externalValue, onChange, onLazyChange, id, showLabel, placeholderId, placeholder, mixed, rows, ...props }: TextareaProps) {
  const transaction = useTransaction();
  const isInPropertyValueColumn = reactHostPort.useContext(PropertyValueColumnContext);
  const [localValue, setLocalValue] = reactHostPort.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = reactHostPort.useState(false);
  const [isFocused, setIsFocused] = reactHostPort.useState(false);
  const textareaRef = reactHostPort.useRef<HTMLTextAreaElement>(null);
  const computedPlaceholder = placeholderId ? useLabel(placeholderId) : placeholder;
  const mixedLabel = useLabel("semio.sketchpad.common.mixedValues");
  const effectivePlaceholder = mixed ? mixedLabel || "—" : computedPlaceholder;

  reactHostPort.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  reactHostPort.useEffect(() => {
    if (isFocused && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [isFocused]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    setIsFocused(true);
    if (lazy) {
      setIsEditing(true);
      transaction?.start?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    setIsFocused(false);
    if (lazy) {
      setIsEditing(false);
      onLazyChange?.(localValue);
      transaction?.finalize?.();
    }
    props.onBlur?.(e);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      if (e.key === "Escape") {
        setIsEditing(false);
        setLocalValue(externalValue?.toString() || "");
        transaction?.abort?.();
        (e.target as HTMLTextAreaElement).blur();
      }
    }
    props.onKeyDown?.(e);
  };

  const textareaValue = lazy ? localValue : externalValue;
  const displayValue = textareaValue?.toString() || "";
  const showCollapsedDisplay = !!showLabel && !isFocused;
  const useSingleRowPropertyEditor = isInPropertyValueColumn && !!showLabel;

  const textareaEmptyOpacity = isInPropertyValueColumn && !displayValue && !isFocused ? 0.6 : 1;

  const textareaElement = (
    <div data-slot="textarea-root" data-detail-panel-control="fill" className="flex min-w-0 w-full flex-1 items-stretch" style={{ opacity: textareaEmptyOpacity, transition: "opacity 150ms" }}>
      {!showCollapsedDisplay ? (
        <textarea
          ref={textareaRef}
          data-slot="textarea"
          data-mixed={mixed ? "true" : undefined}
          id={id}
          className={cn(
            "placeholder:text-muted-foreground text-foreground flex w-full border bg-transparent text-base transition-[color,border-color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
            "focus-visible:border-accent",
            "aria-invalid:border-destructive flex-1",
            useSingleRowPropertyEditor ? "h-medium min-h-[22px] max-h-[22px] resize-none overflow-y-auto px-single py-single leading-normal" : "field-sizing-content min-h-huge px-tiny py-single",
            className,
          )}
          rows={useSingleRowPropertyEditor ? 1 : rows}
          value={textareaValue}
          onChange={handleChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          placeholder={effectivePlaceholder}
          {...props}
        />
      ) : (
        <CollapsedFieldDisplay
          allowStackedOverflow={true}
          className={className}
          disabled={props.disabled}
          id={id}
          mixed={mixed}
          onActivate={() => setIsFocused(true)}
          placeholder={effectivePlaceholder}
          slot="textarea"
          value={mixed && !displayValue ? "" : displayValue}
        />
      )}
    </div>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className="items-start">
        {textareaElement}
      </Label>
    );
  }

  return textareaElement;
}

export { Textarea };

// #endregion 🎏Textarea

// #region 🗡️Toggle
// Toggle button with pressed/unpressed states.
// Consumers MUST handle onPressedChange events.

/**
 * toggleVariants holds the data fields for a toggleVariants record.
 **/
const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap data-[state=on]:bg-active-base data-[state=on]:text-active-foreground data-[state=on]:hover:bg-active-base/90 data-[state=on]:hover:text-active-foreground h-medium aspect-square p-single leading-none overflow-hidden",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
        canvas: "hover:bg-hover-canvas",
        window: "hover:bg-hover-window",
        panel: "hover:bg-hover-panel",
        overlay: "hover:bg-hover-overlay",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      level: "base",
    },
  },
);

/**
 * Configuration interface for a single toggle option with value and label.
 **/
export interface ToggleItem<T extends string> {
  value: T;
  label: React.ReactNode;
  text?: string;
  dropdownText?: string;
  id?: string;
}

/**
 * ToggleStandardProps holds the data fields for a ToggleStandardProps record.
 **/
interface ToggleStandardProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind?: "default" | "icon" | "single";
  i18nPressed?: string;
  showLabel?: boolean;
  icon?: React.ReactNode;
  text?: string;
}

/**
 * ToggleWithActionProps holds the data fields for a ToggleWithActionProps record.
 **/
interface ToggleWithActionProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind: "withAction";
  actionIcon: React.ReactNode;
  onActionClick: () => void;
  showLabel?: boolean;
  actionId?: string;
  icon: React.ReactNode;
  text?: string;
}

/**
 * ToggleDropdownProps holds the data fields for a ToggleDropdownProps record.
 **/
interface ToggleDropdownProps<T extends string> extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind: "dropdown";
  value?: T;
  defaultValue?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  showLabel?: boolean;
  placeholder?: string;
  dropdownId?: string;
  dropdownSide?: "top" | "right" | "bottom" | "left";
  dropdownAlign?: "start" | "center" | "end";
  dropdownSideOffset?: number;
  dropdownAvoidCollisions?: boolean;
  dropdownInstant?: boolean;
  dropdownContentClassName?: string;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}
type ToggleProps<T extends string = string> = ToggleStandardProps | ToggleWithActionProps | ToggleDropdownProps<T>;

export type { ToggleProps };

// #endregion 🗡️Toggle

// #region 🧩ToggleGroup
// Group of mutually exclusive or multi-select toggles.
// Consumers MUST provide items with distinct values.

/**
 * ToggleGroupContext holds the data fields for a ToggleGroupContext record.
 **/
const ToggleGroupContext = reactHostPort.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ToggleGroupItemProps holds the data fields for a ToggleGroupItemProps record.
 **/
type ToggleGroupItemProps = Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Item>, "children"> & {
  id?: string;
  icon: React.ReactNode;
  text?: string;
  action?: React.ReactNode;
  value: string;
};

/**
 * ToggleGroupProps holds the data fields for a ToggleGroupProps record.
 **/
interface ToggleGroupProps extends Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Root>, "children" | "type" | "id"> {
  id?: string;
  showLabel?: boolean;
  kind?: "single" | "multiple";
  items: ToggleGroupItemProps[];
}

/**
 * ToggleGroup holds the data fields for a ToggleGroup record.
 **/
function ToggleGroup({ className, id, showLabel, items, kind = "single", ...restProps }: ToggleGroupProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const divideClass = getLevelDivideElementClass(level);

  const controlledValue = (restProps as any).value;
  const rootDataState = kind === "single" && controlledValue !== undefined ? (controlledValue ? "on" : "off") : undefined;

  const toggleGroupElement = (
    <ToggleGroupPrimitive.Root
      data-slot="toggle-group"
      data-detail-panel-control="fit"
      data-state={rootDataState}
      id={id}
      type={kind}
      className={cn("group/toggle-group flex w-fit shrink-0 items-center border overflow-hidden h-medium divide-x", borderClass, divideClass, className)}
      {...(restProps as any)}
    >
      <ToggleGroupContext.Provider value={{ level }}>
        {items.map((item) => (
          <ToggleGroupItem key={item.value} {...item} />
        ))}
      </ToggleGroupContext.Provider>
    </ToggleGroupPrimitive.Root>
  );

  if (showLabel && id) {
    return (
      <Label id={id} labelElementId={`${id}-label`}>
        {toggleGroupElement}
      </Label>
    );
  }

  return toggleGroupElement;
}

/**
 * ToggleGroupItem holds the data fields for a ToggleGroupItem record.
 **/
function ToggleGroupItem({ className, id, icon, text, action, ...props }: ToggleGroupItemProps) {
  const context = reactHostPort.useContext(ToggleGroupContext);
  const level = context.level ?? "base";

  const toggleGroupItemElement = (
    <ToggleGroupPrimitive.Item
      data-slot="toggle-group-item"
      id={id}
      className={cn(
        toggleVariants({
          level,
        }),
        text
          ? "w-auto shrink-0 focus:z-panel focus-visible:z-panel data-[state=on]:bg-active-base data-[state=on]:hover:bg-active-base/90"
          : "min-w-0 flex-1 shrink-0 focus:z-panel focus-visible:z-panel data-[state=on]:bg-active-base data-[state=on]:hover:bg-active-base/90",
        (text || action) && "flex items-center gap-single py-single px-double aspect-auto",
        text && "w-auto",
        className,
      )}
      {...props}
    >
      <span className={action ? "flex-1 flex items-center justify-center" : undefined}>{icon as React.ReactNode}</span>
      {text && <span className="text-xs whitespace-nowrap">{text}</span>}
      {action && (
        <div
          className={cn("flex items-center justify-center aspect-square h-full flex-shrink-0", getLevelBgClass(level), text && "ml-single")}
          onClick={(e) => e.stopPropagation()}
          onPointerDown={(e) => e.stopPropagation()}
          onMouseDown={(e) => e.stopPropagation()}
          onMouseUp={(e) => e.stopPropagation()}
          onDoubleClick={(e) => e.stopPropagation()}
        >
          {action}
        </div>
      )}
    </ToggleGroupPrimitive.Item>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{toggleGroupItemElement}</TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }
  return toggleGroupItemElement;
}

/**
 **/
const addIconSize = (element: React.ReactNode): React.ReactNode => {
  if (React.isValidElement(element)) {
    const existingClassName = (element.props as any).className || "";
    if (!existingClassName.includes("size-")) {
      return React.cloneElement(element, {
        className: cn(existingClassName, "size-small"),
      } as any);
    }
  }
  return element;
};

/**
 * Toggle holds the data fields for a Toggle record.
 **/
function Toggle<T extends string = string>(props: ToggleProps<T>) {
  if ("kind" in props && props.kind === "withAction") {
    const { actionIcon, onActionClick, icon, text, pressed, defaultPressed, onPressedChange, id, showLabel, className, actionId } = props as ToggleWithActionProps;
    const value = pressed !== undefined ? (pressed ? "on" : undefined) : undefined;
    return (
      <ToggleGroup
        showLabel={showLabel}
        kind="multiple"
        value={value ? [value] : []}
        defaultValue={pressed === undefined && defaultPressed ? ["on"] : []}
        onValueChange={(val: string[]) => onPressedChange?.(val.includes("on"))}
        className={className}
        items={[
          {
            value: "on",
            icon: addIconSize(icon),
            text: text,
            action: <Action as="div" id={actionId} icon={addIconSize(actionIcon)} onClick={onActionClick} />,
            id: id,
          },
        ]}
      />
    );
  }

  if ("kind" in props && props.kind === "dropdown" && "items" in props) {
    const dropdownProps = props as ToggleDropdownProps<T>;
    const {
      items,
      value: controlledValue,
      defaultValue,
      pressed,
      defaultPressed,
      onPressedChange,
      id,
      showLabel,
      className,
      dropdownId,
      dropdownSide = "bottom",
      dropdownAlign = "start",
      dropdownSideOffset = 4,
      dropdownAvoidCollisions = true,
      dropdownInstant = false,
      dropdownContentClassName,
      open: controlledOpen,
      onOpenChange,
      onValueChange,
    } = dropdownProps;
    const [internalValue, setInternalValue] = reactHostPort.useState<T | undefined>(defaultValue);
    const [internalOpen, setInternalOpen] = reactHostPort.useState(false);

    const isControlled = controlledValue !== undefined;
    const value = isControlled ? controlledValue : internalValue;
    const selectedItem = items.find((item) => item.value === value) || items[0];
    const isOpenControlled = controlledOpen !== undefined;
    const open = isOpenControlled ? controlledOpen : internalOpen;
    const setOpen = (nextOpen: boolean) => {
      if (!isOpenControlled) {
        setInternalOpen(nextOpen);
      }
      onOpenChange?.(nextOpen);
    };

    const handleSelect = (itemValue: string) => {
      if (!isControlled) {
        setInternalValue(itemValue as T);
      }
      if (onValueChange) onValueChange(itemValue as T);
      setOpen(false);
    };

    const handleToggleGroupValueChange = (toggleValue: string) => {
      const isPressed = toggleValue === selectedItem.value;
      if (onPressedChange) {
        onPressedChange(isPressed);
      }
    };

    const availableItems = items;

    const dropdownAction = (
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Action as="div" id={dropdownId} icon={<ChevronDownIcon className="size-small" />} />
        </PopoverTrigger>
        <PopoverContent
          side={dropdownSide}
          align={dropdownAlign}
          sideOffset={dropdownSideOffset}
          avoidCollisions={dropdownAvoidCollisions}
          className={cn(
            "w-auto p-single min-w-[120px]",
            dropdownInstant ? "data-[state=open]:animate-none data-[state=closed]:animate-none data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0 data-[state=open]:zoom-in-100 data-[state=closed]:zoom-out-100" : "",
            dropdownContentClassName,
          )}
        >
          <div className="flex flex-col">
            {availableItems.map((item) => {
              const dropdownText = item.dropdownText || item.text;
              const buttonElement = (
                <button key={item.value} onClick={() => handleSelect(item.value)} className={cn("flex items-center p-single text-xs cursor-selectable transition-colors", "hover:bg-hover-temporary outline-none focus-visible:bg-hover-temporary")}>
                  <span className="flex flex-1 items-center gap-single text-left">
                    <span className="flex items-center">{addIconSize(item.label)}</span>
                    {dropdownText ? <span className="text-xs">{dropdownText}</span> : null}
                  </span>
                </button>
              );

              if (item.id) {
                return (
                  <Tooltip key={item.value}>
                    <TooltipTrigger asChild>{buttonElement}</TooltipTrigger>
                    <TooltipContent side="left">
                      <DescriptionTooltipContent id={item.id} />
                    </TooltipContent>
                  </Tooltip>
                );
              }

              return buttonElement;
            })}
          </div>
        </PopoverContent>
      </Popover>
    );

    const isPressedControlled = pressed !== undefined;
    const toggleGroupProps: any = {
      id,
      showLabel,
      kind: "single" as const,
      onValueChange: handleToggleGroupValueChange,
      className,
      items: [
        {
          value: selectedItem.value,
          icon: addIconSize(selectedItem.label),
          text: selectedItem.text,
          action: dropdownAction,

          id: selectedItem.id,
        },
      ],
    };

    if (isPressedControlled) {
      toggleGroupProps.value = pressed ? selectedItem.value : "";
    } else if (defaultPressed !== undefined) {
      toggleGroupProps.defaultValue = defaultPressed ? selectedItem.value : undefined;
    }

    return <ToggleGroup {...toggleGroupProps} />;
  }

  const { id, showLabel, className, icon, text, pressed, defaultPressed, onPressedChange } = props as ToggleStandardProps;
  const value = pressed !== undefined ? (pressed ? "on" : "") : undefined;
  return (
    <ToggleGroup
      id={id}
      showLabel={showLabel}
      className={className}
      kind="single"
      value={value}
      defaultValue={pressed === undefined && defaultPressed ? "on" : undefined}
      onValueChange={(val: string) => onPressedChange?.(val === "on")}
      items={[
        {
          value: "on",
          icon: addIconSize(icon),
          text: text,
        },
      ]}
    />
  );
}
export { Toggle, ToggleGroup, ToggleGroupItem, toggleVariants };

// #endregion 🧩ToggleGroup

// #region 🎄Orb
// Circular position indicator on a Ring. t ∈ [0,1[ maps to an angle on the ring.

interface OrbProps {
  id: string;
  t: number;
  disabled?: boolean;
  selected?: boolean;
  hovered?: boolean;
  dragging?: boolean;
  radius?: number;
  onPointerDown?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerMove?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerUp?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerEnter?: (e: React.PointerEvent<SVGCircleElement>) => void;
  onPointerLeave?: (e: React.PointerEvent<SVGCircleElement>) => void;
}

function Orb({ id, t, disabled = false, selected = false, hovered = false, radius = 40, dragging = false, onPointerDown, onPointerMove, onPointerUp, onPointerEnter, onPointerLeave }: OrbProps) {
  const angle = t * 2 * Math.PI - Math.PI / 2;
  const cx = Math.cos(angle) * radius;
  const cy = Math.sin(angle) * radius;
  const orbRadius = selected ? 7 : 5;
  return (
    <circle
      data-slot="orb"
      data-orb-id={id}
      cx={cx}
      cy={cy}
      r={orbRadius}
      className={cn(
        dragging ? "" : "transition-all duration-150",
        disabled ? "fill-muted-foreground/40 cursor-not-allowed" : "fill-foreground cursor-grab active:cursor-grabbing",
        selected && !disabled && "fill-accent stroke-accent-foreground stroke-1",
        hovered && !disabled && !selected && "fill-accent-foreground",
      )}
      style={{ pointerEvents: disabled ? "none" : "auto" }}
      onPointerDown={disabled ? undefined : onPointerDown}
      onPointerMove={disabled ? undefined : onPointerMove}
      onPointerUp={disabled ? undefined : onPointerUp}
      onPointerEnter={disabled ? undefined : onPointerEnter}
      onPointerLeave={disabled ? undefined : onPointerLeave}
    />
  );
}

export { Orb };
export type { OrbProps };

// #endregion 🎄Orb

// #region 🧫Ring
// SVG ring container with draggable Orbs. Fires onOrbChange(orbId, oldT, newT) on drag.

interface RingOrbData {
  id: string;
  t: number;
  disabled?: boolean;
  selected?: boolean;
  hovered?: boolean;
}

interface RingProps extends ElementProps {
  orbs: RingOrbData[];
  radius?: number;
  size?: number;
  onOrbChange?: (orbId: string, oldT: number, newT: number) => void;
  onOrbSelect?: (orbId: string) => void;
  onOrbHoverChange?: (orbId: string, hovered: boolean) => void;
  showLabel?: boolean;
  className?: string;
}

function Ring({ id, orbs, radius = 40, size = 100, onOrbChange, onOrbSelect, onOrbHoverChange, showLabel, className }: RingProps) {
  const transaction = useTransaction();
  const svgRef = reactHostPort.useRef<SVGSVGElement>(null);
  const [draggingOrbId, setDraggingOrbId] = reactHostPort.useState<string | null>(null);
  const [localT, setLocalT] = reactHostPort.useState<number | null>(null);
  const dragStartT = reactHostPort.useRef<number>(0);
  const rafId = reactHostPort.useRef<number>(0);
  const pendingT = reactHostPort.useRef<number | null>(null);
  const center = size / 2;
  const angleFromEvent = reactHostPort.useCallback(
    (e: React.PointerEvent | PointerEvent): number => {
      if (!svgRef.current) return 0;
      const rect = svgRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left - center;
      const y = e.clientY - rect.top - center;
      let angle = Math.atan2(y, x) + Math.PI / 2;
      if (angle < 0) angle += 2 * Math.PI;
      return (angle / (2 * Math.PI)) % 1;
    },
    [center],
  );
  const handleOrbPointerDown = reactHostPort.useCallback(
    (orbId: string, t: number) => (e: React.PointerEvent<SVGCircleElement>) => {
      e.preventDefault();
      setDraggingOrbId(orbId);
      setLocalT(t);
      dragStartT.current = t;
      pendingT.current = null;
      transaction?.start?.();
      onOrbSelect?.(orbId);
    },
    [transaction, onOrbSelect],
  );
  const flushPendingChange = reactHostPort.useCallback(
    (orbId: string) => {
      if (pendingT.current !== null) {
        onOrbChange?.(orbId, dragStartT.current, pendingT.current);
        pendingT.current = null;
      }
    },
    [onOrbChange],
  );
  reactHostPort.useEffect(() => {
    if (!draggingOrbId) return;
    const onMove = (e: PointerEvent) => {
      const newT = angleFromEvent(e);
      setLocalT(newT);
      pendingT.current = newT;
      if (!rafId.current) {
        const orbId = draggingOrbId;
        rafId.current = requestAnimationFrame(() => {
          rafId.current = 0;
          flushPendingChange(orbId);
        });
      }
    };
    const onUp = (e: PointerEvent) => {
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      const newT = angleFromEvent(e);
      setLocalT(null);
      onOrbChange?.(draggingOrbId, dragStartT.current, newT);
      setDraggingOrbId(null);
      transaction?.finalize?.();
    };
    const onCancel = () => {
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      setLocalT(null);
      setDraggingOrbId(null);
      transaction?.abort?.();
    };
    const bindings = createDOMEventBinding();
    bindings.listen(window, "pointermove", onMove);
    bindings.listen(window, "pointerup", onUp);
    bindings.listen(window, "pointercancel", onCancel);
    return () => bindings.dispose();
  }, [draggingOrbId, angleFromEvent, flushPendingChange, onOrbChange, transaction]);
  reactHostPort.useEffect(() => {
    return () => {
      if (rafId.current) cancelAnimationFrame(rafId.current);
    };
  }, []);
  const ringElement = (
    <svg
      ref={svgRef}
      data-slot="ring"
      data-detail-panel-control="fit"
      id={id}
      width={size}
      height={size}
      viewBox={`${-center} ${-center} ${size} ${size}`}
      className={cn("w-fit shrink-0 touch-none select-none overflow-visible", className)}
      style={{ overflow: "visible" }}
    >
      <circle data-slot="ring-track" cx={0} cy={0} r={radius} className="fill-none stroke-muted-foreground/30 stroke-[2px]" />
      {orbs.map((orb) => (
        <Orb
          key={orb.id}
          id={orb.id}
          t={draggingOrbId === orb.id && localT !== null ? localT : orb.t}
          disabled={orb.disabled}
          selected={orb.selected}
          hovered={orb.hovered}
          dragging={draggingOrbId === orb.id}
          radius={radius}
          onPointerDown={handleOrbPointerDown(orb.id, orb.t)}
          onPointerEnter={onOrbHoverChange ? () => onOrbHoverChange(orb.id, true) : undefined}
          onPointerLeave={onOrbHoverChange ? () => onOrbHoverChange(orb.id, false) : undefined}
        />
      ))}
    </svg>
  );
  if (showLabel) {
    return (
      <Label id={id} labelElementId={`${id}-label`} className={className}>
        {ringElement}
      </Label>
    );
  }
  return ringElement;
}

export { Ring };
export type { RingOrbData, RingProps };

// #endregion 🧫Ring

// #endregion 🛒Input Components

// #region 🗼Aggregation Components

// #region 🛒Accordion
// Collapsible accordion built on Radix primitives.
// Consumers MUST use AccordionItem children.

/**
 * Accordion holds the data fields for a Accordion record.
 **/
function Accordion({ ...props }: React.ComponentProps<typeof AccordionPrimitive.Root>) {
  return <AccordionPrimitive.Root data-slot="accordion" {...props} />;
}

/**
 * AccordionItem holds the data fields for a AccordionItem record.
 **/
function AccordionItem({ className, ...props }: React.ComponentProps<typeof AccordionPrimitive.Item>) {
  return <AccordionPrimitive.Item data-slot="accordion-item" className={cn("border-b border-element last:border-b-0", className)} {...props} />;
}

/**
 * AccordionTrigger holds the data fields for a AccordionTrigger record.
 **/
function AccordionTrigger({ className, children, ...props }: React.ComponentProps<typeof AccordionPrimitive.Trigger>) {
  return (
    <AccordionPrimitive.Header className="flex">
      <AccordionPrimitive.Trigger data-slot="accordion-trigger" className={cn(className)} {...props}>
        {children as React.ReactNode}
        <ChevronDownIconAlt className="text-muted-foreground pointer-events-none size-small shrink-0 translate-y-0.5 transition-transform duration-200" />
      </AccordionPrimitive.Trigger>
    </AccordionPrimitive.Header>
  );
}

/**
 * AccordionContent wraps collapsible accordion body content.
 **/
function AccordionContent({ className, children, ...props }: React.ComponentProps<typeof AccordionPrimitive.Content>) {
  return (
    <AccordionPrimitive.Content data-slot="accordion-content" className={cn("data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm", className)} {...props}>
      <div className="pb-4 pt-0">{children}</div>
    </AccordionPrimitive.Content>
  );
}

export { Accordion, AccordionContent, AccordionItem, AccordionTrigger };

// #endregion 🛒Accordion

// #region 🖥️Collapsible
// Collapsible section built on Radix primitives.
// Consumers MUST use CollapsibleTrigger.

/**
 * Collapsible holds the data fields for a Collapsible record.
 **/
function Collapsible({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.Root>) {
  return <CollapsiblePrimitive.Root data-slot="collapsible" {...props} />;
}

/**
 * CollapsibleTrigger holds the data fields for a CollapsibleTrigger record.
 **/
function CollapsibleTrigger({ className, ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleTrigger>) {
  return <CollapsiblePrimitive.CollapsibleTrigger data-slot="collapsible-trigger" className={cn(className)} {...props} />;
}

/**
 **/
function CollapsibleContent({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleContent>) {
  return <CollapsiblePrimitive.CollapsibleContent data-slot="collapsible-content" {...props} />;
}

export { Collapsible, CollapsibleContent, CollapsibleTrigger };

// #endregion 🖥️Collapsible

// #region 🧸Dialog
// Modal dialog built on Radix primitives.
// Consumers MUST use DialogTrigger to open.

/**
 * Dialog holds the data fields for a Dialog record.
 **/
function Dialog({ ...props }: React.ComponentProps<typeof DialogPrimitive.Root>) {
  return <DialogPrimitive.Root data-slot="dialog" {...props} />;
}

/**
 * DialogTrigger holds the data fields for a DialogTrigger record.
 **/
function DialogTrigger({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Trigger>) {
  return <DialogPrimitive.Trigger data-slot="dialog-trigger" className={cn(className)} {...props} />;
}

/**
 * DialogPortal holds the data fields for a DialogPortal record.
 **/
function DialogPortal({ ...props }: React.ComponentProps<typeof DialogPrimitive.Portal>) {
  return <DialogPrimitive.Portal data-slot="dialog-portal" {...props} />;
}

/**
 * DialogClose holds the data fields for a DialogClose record.
 **/
function DialogClose({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Close>) {
  return <DialogPrimitive.Close data-slot="dialog-close" className={cn(className)} {...props} />;
}

/**
 * DialogOverlay holds the data fields for a DialogOverlay record.
 **/
function DialogOverlay({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Overlay>) {
  return (
    <DialogPrimitive.Overlay
      data-slot="dialog-overlay"
      className={cn("data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-overlay bg-black/50", className)}
      {...props}
    />
  );
}

/**
 * DialogContent holds the data fields for a DialogContent record.
 **/
function DialogContent({
  className,
  showCloseButton = true,
  children,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Content> & {
  showCloseButton?: boolean;
}) {
  return (
    <DialogPortal data-slot="dialog-portal">
      <DialogPrimitive.Content
        data-slot="dialog-content"
        className={cn(
          "bg-transparent backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-temporary grid w-full max-w-[calc(100%-2*var(--spacing)*var(--medium))] translate-x-[-50%] translate-y-[-50%] gap-medium border p-medium duration-200 sm:max-w-lg",
          className,
        )}
        {...props}
      >
        {children}
        {showCloseButton && (
          <DialogPrimitive.Close
            data-slot="dialog-close"
            className="ring-offset-background focus:ring-ring data-[state=open]:bg-accent data-[state=open]:text-muted-foreground absolute top-medium right-4 rounded-xs opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-small"
          >
            <CloseIconAlt />
            <span className="sr-only">Close</span>
          </DialogPrimitive.Close>
        )}
      </DialogPrimitive.Content>
    </DialogPortal>
  );
}

/**
 * DialogHeader holds the data fields for a DialogHeader record.
 **/
function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-header" className={cn("flex flex-col gap-single text-center sm:text-left", className)} {...props} />;
}

/**
 * DialogFooter holds the data fields for a DialogFooter record.
 **/
function DialogFooter({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-footer" className={cn("flex flex-col-reverse gap-single sm:flex-row sm:justify-end", className)} {...props} />;
}

/**
 * DialogTitle holds the data fields for a DialogTitle record.
 **/
function DialogTitle({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Title>) {
  return <DialogPrimitive.Title data-slot="dialog-title" className={cn("text-lg font-semibold leading-none tracking-tight", className)} {...props} />;
}

/**
 * DialogDescription holds the data fields for a DialogDescription record.
 **/
function DialogDescription({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Description>) {
  return <DialogPrimitive.Description data-slot="dialog-description" className={cn("text-muted-foreground text-sm", className)} {...props} />;
}

export { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger };

// #endregion 🧸Dialog

// #region 🪬Resizable

function ResizablePanelGroup({
  className,
  orientation = "horizontal",
  ...props
}: React.ComponentProps<typeof ResizablePrimitive.Group>) {
  return (
    <ResizablePrimitive.Group
      data-slot="resizable-panel-group"
      data-panel-group-direction={orientation}
      className={cn("flex h-full w-full", orientation === "vertical" ? "flex-col" : "flex-row", className)}
      orientation={orientation}
      {...props}
    />
  );
}

/**
 * ResizablePanel holds the data fields for a ResizablePanel record.
 **/
function ResizablePanel({ ...props }: React.ComponentProps<typeof ResizablePrimitive.Panel>) {
  return <ResizablePrimitive.Panel data-slot="resizable-panel" {...props} />;
}

function ResizableHandle({
  className,
  onMouseDown: externalOnMouseDown,
  onMouseEnter: externalOnMouseEnter,
  onMouseLeave: externalOnMouseLeave,
  ...props
}: React.ComponentProps<typeof ResizablePrimitive.Separator> & { onMouseDown?: React.MouseEventHandler<HTMLDivElement>; onMouseEnter?: React.MouseEventHandler<HTMLDivElement>; onMouseLeave?: React.MouseEventHandler<HTMLDivElement> }) {
  const [isHovered, setIsHovered] = reactHostPort.useState(false);
  const [isDragging, setIsDragging] = reactHostPort.useState(false);

  const handleMouseDown: React.MouseEventHandler<HTMLDivElement> = (e) => {
    setIsDragging(true);
    externalOnMouseDown?.(e as any);

    const bindings = createDOMEventBinding();
    const handleMouseUp = () => {
      setIsDragging(false);
      bindings.dispose();
    };

    bindings.listen(document, "mouseup", handleMouseUp, true);
  };

  const handleMouseEnter: React.MouseEventHandler<HTMLDivElement> = (e) => {
    setIsHovered(true);
    externalOnMouseEnter?.(e as any);
  };

  const handleMouseLeave: React.MouseEventHandler<HTMLDivElement> = (e) => {
    if (!isDragging) {
      setIsHovered(false);
    }
    externalOnMouseLeave?.(e as any);
  };

  return (
    <ResizablePrimitive.Separator
      data-slot="resizable-handle"
      className={cn(
        "relative flex w-px items-center justify-center",
        "border-r",
        isDragging || isHovered ? "bg-accent border-accent" : "hover:border-accent",
        "before:absolute before:inset-y-0 before:-left-2 before:w-tiny before:cursor-ew-resize",
        "focus-visible:ring-ring focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none",
        "after:absolute after:inset-y-0 after:left-1/2 after:w-single after:-translate-x-1/2",
        className,
      )}
      onMouseDown={handleMouseDown as any}
      onMouseEnter={handleMouseEnter as any}
      onMouseLeave={handleMouseLeave as any}
      {...(props as any)}
    />
  );
}

export { ResizableHandle, ResizablePanel, ResizablePanelGroup };

// #endregion 🪬Resizable

// #region 🎮Scrollable
// Custom scrollable area built on Radix ScrollArea.
// 🔷Consumers MUST wrap content in Scrollable.
const Scrollable = reactHostPort.forwardRef<React.ElementRef<typeof ScrollAreaPrimitive.Viewport>, React.ComponentPropsWithoutRef<typeof ScrollAreaPrimitive.Root> & { orientation?: "vertical" | "horizontal" | "both" }>(
  ({ className, children, orientation = "vertical", ...props }, ref) => {
    return (
      <ScrollAreaPrimitive.Root data-slot="scroll-area" className={cn("relative", className)} {...props}>
        <ScrollAreaPrimitive.Viewport
          ref={ref}
          data-slot="scroll-area-viewport"
          className={cn(
            "focus-visible:ring-ring/50 size-full transition-[color,box-shadow] outline-none focus-visible:ring-[3px] focus-visible:outline-1 min-w-0",
            orientation === "horizontal" ? "overflow-x-auto overflow-y-hidden" : orientation === "vertical" ? "overflow-y-auto overflow-x-hidden" : "overflow-auto",
          )}
        >
          {children}
        </ScrollAreaPrimitive.Viewport>
        {(orientation === "vertical" || orientation === "both") && <ScrollBar />}
        {(orientation === "horizontal" || orientation === "both") && <ScrollBar orientation="horizontal" />}
        <ScrollAreaPrimitive.Corner />
      </ScrollAreaPrimitive.Root>
    );
  },
);
Scrollable.displayName = "Scrollable";

/**
 * ScrollBar holds the data fields for a ScrollBar record.
 **/
function ScrollBar({ className, orientation = "vertical", ...props }: React.ComponentProps<typeof ScrollAreaPrimitive.ScrollAreaScrollbar>) {
  return (
    <ScrollAreaPrimitive.ScrollAreaScrollbar
      data-slot="scroll-area-scrollbar"
      orientation={orientation}
      className={cn(
        "flex touch-none select-none transition-colors",
        orientation === "vertical" && "h-full w-2.5 border-l border-l-transparent p-[1px]",
        orientation === "horizontal" && "h-2.5 flex-col border-t border-t-transparent p-[1px]",
        className,
      )}
      {...props}
    >
      <ScrollAreaPrimitive.ScrollAreaThumb data-slot="scroll-area-thumb" className="bg-border relative flex-1 rounded-full" />
    </ScrollAreaPrimitive.ScrollAreaScrollbar>
  );
}

export { Scrollable, ScrollBar };

// #endregion 🎮Scrollable

// #region 🥁Band
// Horizontal band of navigation items with labels and icons.
// Consumers MUST provide BandItem entries.

/**
 * Configuration interface for a single band item.
 **/
export interface BandItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Band component.
 **/
export interface BandProps {
  id?: string;
  items: BandItem[];
  scrollable?: boolean;
  className?: string;
}

/**
 * Band holds the data fields for a Band record.
 **/
function Band({ items, scrollable = true, className, id }: BandProps) {
  const level = useLevel();
  const bgClass = getLevelBgClass(level);
  const borderClass = getLevelBorderElementClass(level);
  const itemsElement = (
    <div id={id} data-slot="band" className={cn("p-single flex gap-single items-center min-w-0", scrollable ? "w-fit" : "w-full")}>
      {items.map((item, index) => (
        <div key={item.key ?? index} className={cn("h-medium flex items-center min-w-0", item.className)}>
          {item.content}
        </div>
      ))}
    </div>
  );

  if (scrollable)
    return (
      <Scrollable orientation="horizontal" className={cn("border-b h-large", borderClass, bgClass, className)}>
        {itemsElement}
      </Scrollable>
    );
  return <div className={cn("border-b h-large", borderClass, bgClass, className)}>{itemsElement}</div>;
}

export { Band as Band };

// #endregion 🥁Band

// #region 📢Strip
// Vertical strip of icon items for compact navigation.
// Consumers MUST provide StripItem entries.

/**
 * Configuration interface for a single strip item.
 **/
export interface StripItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Strip component.
 **/
export interface StripProps {
  id?: string;
  items: StripItem[];
  scrollable?: boolean;
  className?: string;
}

/**
 * Strip holds the data fields for a Strip record.
 **/
function Strip({ items, scrollable = true, className, id }: StripProps) {
  const level = useLevel();
  const bgClass = getLevelBgClass(level);
  const borderClass = getLevelBorderElementClass(level);
  const itemsElement = (
    <div id={id} data-slot="strip" className={cn("p-single flex gap-single items-center min-w-0", scrollable ? "w-fit" : "w-full")}>
      {items.map((item, index) => (
        <div key={item.key ?? index} className={cn("h-small flex items-center min-w-0", item.className)}>
          {item.content}
        </div>
      ))}
    </div>
  );

  if (scrollable)
    return (
      <Scrollable orientation="horizontal" className={cn("border-b h-medium", borderClass, bgClass, className)}>
        {itemsElement}
      </Scrollable>
    );
  return <div className={cn("border-b h-medium", borderClass, bgClass, className)}>{itemsElement}</div>;
}

export { Strip };

// #endregion 📢Strip

// #region 🩺Navbar
// Top navigation bar with icon items.
// Consumers MUST provide NavbarItem entries.

/**
 * Configuration interface for a single navbar item.
 **/
export interface NavbarItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Navbar component.
 **/
export interface NavbarProps {
  items: NavbarItem[];
  className?: string;
}

/**
 * Navbar holds the data fields for a Navbar record.
 **/
function Navbar({ items, className }: NavbarProps) {
  const level = useLevel();
  const bgClass = getLevelBgClass(level);
  return (
    <nav id="semio.sketchpad.navbar" data-slot="navbar" className={cn("border-b h-large z-navbar", bgClass, className)}>
      <div className="p-single flex gap-single items-center min-w-0">
        {items.map((item, index) => (
          <div key={item.key ?? index} className={cn("h-medium flex items-center min-w-0", item.className)}>
            {item.content}
          </div>
        ))}
      </div>
    </nav>
  );
}

export { Navbar };

// #endregion 🩺Navbar

// #region 🏷️Tabs
// Tab container built on Radix primitives.
// Consumers MUST use TabsTrigger and TabsContent.

/**
 * Tabs holds the data fields for a Tabs record.
 **/
function Tabs({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Root>) {
  return <TabsPrimitive.Root data-slot="tabs" className={cn("flex flex-col gap-single", className)} {...props} />;
}

/**
 * TabsList holds the data fields for a TabsList record.
 **/
function TabsList({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.List>) {
  const level = useLevel();
  const bgClass = getLevelBgClass(level);
  return <TabsPrimitive.List data-slot="tabs-list" className={cn("text-muted-foreground inline-flex h-large w-fit items-center justify-center p-single", bgClass, className)} {...props} />;
}

/** TabsTrigger holds the data fields for a TabsTrigger record.
 **/
/**
 **/
function TabsTrigger({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Trigger>) {
  const level = useLevel();
  const activeHoverClass = getLevelActiveHoverClass(level);
  const hoverClass = getLevelHoverClass(level);
  return (
    <TabsPrimitive.Trigger
      data-slot="tabs-trigger"
      className={cn(
        "focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring text-foreground inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-single border border-transparent p-single text-sm font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:ring-[3px] focus-visible:outline-1 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:shadow-sm [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        activeHoverClass,
        hoverClass,
        className,
      )}
      {...props}
    />
  );
}

/**
 **/
function TabsContent({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Content>) {
  return <TabsPrimitive.Content data-slot="tabs-content" className={cn("flex-1 outline-none", className)} {...props} />;
}

export { Tabs, TabsContent, TabsList, TabsTrigger };

// #endregion 🏷️Tabs

// #region 🖼️IconSelector

/** @emoji 🎛️ Tab buckets for {@link IconSelector} (board WASM `iconKind`: `typst:` / `$…`, `data:` payloads, `emoji:`, catalog or inline SVG). */
export type ElementsBoardIconSelectorMode = "data" | "emoji" | "math" | "vector";

function stripLegacyImageDataPrefixForIconSelectorUi(raw: string): string {
	const t = raw.trim();
	return t.startsWith("image:") ? t.slice("image:".length).trim() : t;
}

function isRasterDataUrlPayloadForIconSelectorUi(s: string): boolean {
	const u = s.trim().toLowerCase();
	return (
		u.startsWith("data:image/png;base64,") ||
		u.startsWith("data:image/jpeg;base64,") ||
		u.startsWith("data:image/jpg;base64,")
	);
}

function looksLikeAsciiCatalogishVectorStemForIconSelectorUi(s: string): boolean {
	const t = s.trim();
	if (t === "") {
		return false;
	}
	if (!/^[\w.-]+$/.test(t)) {
		return false;
	}
	return /[.-_]/.test(t) || t.length > 48;
}

/** @emoji 🧭 Derives {@link IconSelector} tab; keep aligned with `classifyElementsBoardIconSelectorMode` in `@puzzle/2d/react`. */
function defaultClassifyElementsBoardIconSelectorMode(raw: string): ElementsBoardIconSelectorMode {
	const t = raw.trim();
	if (t === "") {
		return "math";
	}
	if (t.startsWith("typst:") || t.startsWith("$")) {
		return "math";
	}
	if (t.startsWith("emoji:")) {
		return "emoji";
	}
	const lower = t.toLowerCase();
	if (lower.startsWith("data:") || isRasterDataUrlPayloadForIconSelectorUi(stripLegacyImageDataPrefixForIconSelectorUi(t))) {
		return "data";
	}
	if (lower.startsWith("<?xml") || lower.includes("<svg")) {
		return "vector";
	}
	if (looksLikeAsciiCatalogishVectorStemForIconSelectorUi(t)) {
		return "vector";
	}
	return "emoji";
}

function stripTypstEmojiPrefixesForIconSelector(raw: string): string {
	const t = raw.trim();
	if (t.startsWith("typst:")) {
		return t.slice("typst:".length).trim();
	}
	if (t.startsWith("emoji:")) {
		return t.slice("emoji:".length).trim();
	}
	return t;
}

function mathInnerFromIconKindStored(stored: string): string {
	return stripTypstEmojiPrefixesForIconSelector(stored);
}

function emitMathIconKindFromInner(inner: string): string {
	const i = inner.trim();
	if (i === "") {
		return "";
	}
	return `typst:${i}`;
}

function emojiInnerFromIconKindStored(stored: string): string {
	const t = stored.trim();
	return t.startsWith("emoji:") ? t.slice("emoji:".length).trim() : stripTypstEmojiPrefixesForIconSelector(t);
}

function emitEmojiIconKindFromInner(inner: string): string {
	const i = inner.trim();
	return i === "" ? "" : `emoji:${i}`;
}

function migrateIconKindToIconSelectorMode(
	prev: string,
	mode: ElementsBoardIconSelectorMode,
	classify: (raw: string) => ElementsBoardIconSelectorMode,
): string {
	const cur = classify(prev);
	if (cur === mode) {
		return prev;
	}
	if (mode === "data") {
		return cur === "data" ? prev : "";
	}
	if (mode === "vector") {
		if (cur === "vector") {
			return prev;
		}
		return "";
	}
	const neutral = stripTypstEmojiPrefixesForIconSelector(prev).trim();
	if (mode === "math") {
		return neutral === "" ? "" : emitMathIconKindFromInner(neutral);
	}
	if (mode === "emoji") {
		return neutral === "" ? "" : emitEmojiIconKindFromInner(neutral);
	}
	return "";
}

export interface IconSelectorProps {
	id: string;
	value: string;
	onChange: (next: string) => void;
	disabled?: boolean;
	uniform?: boolean;
	classifyElementsBoardIconSelectorMode?: (raw: string) => ElementsBoardIconSelectorMode;
}

/** @emoji 🖼️ Board `iconKind` editor: mode dropdown (math / data URL / emoji / catalog or SVG), one editor, preview strip, import and clear. */
export function IconSelector({
	id,
	value,
	onChange,
	disabled = false,
	uniform = true,
	classifyElementsBoardIconSelectorMode: classifyModeProp,
}: IconSelectorProps): React.ReactElement {
	const classifyMode = classifyModeProp ?? defaultClassifyElementsBoardIconSelectorMode;
	const activeMode = classifyMode(value);
	const fileInputRef = reactHostPort.useRef<HTMLInputElement>(null);
	const locked = disabled || !uniform;
	const mathFieldValue = uniform && activeMode === "math" ? mathInnerFromIconKindStored(value) : "";
	const dataFieldValue = uniform && activeMode === "data" ? value : "";
	const emojiFieldValue = uniform && activeMode === "emoji" ? emojiInnerFromIconKindStored(value) : "";
	const vectorFieldValue = uniform && activeMode === "vector" ? value : "";

	const onModeSelect = (next: string) => {
		if (locked) {
			return;
		}
		const mode = next as ElementsBoardIconSelectorMode;
		onChange(migrateIconKindToIconSelectorMode(value, mode, classifyMode));
	};

	const editorValue = uniform
		? activeMode === "math"
			? mathFieldValue
			: activeMode === "data"
				? dataFieldValue
				: activeMode === "emoji"
					? emojiFieldValue
					: vectorFieldValue
		: "";

	const onEditorChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
		if (locked) {
			return;
		}
		const raw = e.target.value;
		if (activeMode === "math") {
			onChange(emitMathIconKindFromInner(raw));
			return;
		}
		if (activeMode === "emoji") {
			onChange(emitEmojiIconKindFromInner(raw));
			return;
		}
		onChange(raw);
	};

	const editorPlaceholder =
		activeMode === "math"
			? "Typst markup (e.g. $x^2$)"
			: activeMode === "data"
				? "data:image/png;base64,… or other data:… URL"
				: activeMode === "emoji"
					? "Typst body after emoji: (e.g. 😀)"
					: "Catalog id or inline <svg …>";

	const modeSelectTitle =
		activeMode === "math"
			? "Typst math (stored as typst:… or leading $)"
			: activeMode === "data"
				? "Raster or other data: URL"
				: activeMode === "emoji"
					? "Typst emoji cell (stored as emoji:…)"
					: "Metabolism catalog id or inline SVG";

	const onPickFiles = (e: React.ChangeEvent<HTMLInputElement>) => {
		const list = e.target.files;
		const f = list?.[0];
		e.target.value = "";
		if (!f || locked) {
			return;
		}
		const isRasterMime = f.type === "image/png" || f.type === "image/jpeg" || /\.png$/i.test(f.name) || /\.jpe?g$/i.test(f.name);
		const isSvgMime = f.type === "image/svg+xml" || /\.svg$/i.test(f.name);
		if (isSvgMime) {
			const reader = new FileReader();
			reader.onload = () => {
				const text = typeof reader.result === "string" ? reader.result.trim() : "";
				onChange(text);
			};
			reader.readAsText(f);
			return;
		}
		if (isRasterMime) {
			const reader = new FileReader();
			reader.onload = () => {
				const url = typeof reader.result === "string" ? reader.result : "";
				onChange(url.trim());
			};
			reader.readAsDataURL(f);
			return;
		}
		const reader = new FileReader();
		reader.onload = () => {
			const url = typeof reader.result === "string" ? reader.result : "";
			onChange(url.trim());
		};
		reader.readAsDataURL(f);
	};

	const vectorPreviewSrc = reactHostPort.useMemo(() => {
		const t = value.trim();
		const lower = t.toLowerCase();
		if (!lower.includes("<svg")) {
			return null;
		}
		try {
			return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(t)}`;
		} catch {
			return null;
		}
	}, [value]);

	const rasterPreviewSrc = reactHostPort.useMemo(() => {
		const t = value.trim();
		if (!/^data:image\/(png|jpeg|jpg);/i.test(t)) {
			return null;
		}
		return t;
	}, [value]);

	const preview = (() => {
		const t = value.trim();
		if (t === "") {
			return <span className="text-muted-foreground text-xs">—</span>;
		}
		const mode = classifyMode(t);
		if (mode === "data") {
			const src = rasterPreviewSrc ?? (t.toLowerCase().startsWith("data:") ? t : null);
			if (src && /^data:image\/(png|jpeg|svg\+xml|jpg)/i.test(src)) {
				return <img alt="" className="max-h-14 max-w-full object-contain" src={src} />;
			}
			return (
				<span className="text-muted-foreground max-h-14 overflow-hidden font-mono text-[10px] leading-tight break-all">
					{t.length > 180 ? `${t.slice(0, 180)}…` : t}
				</span>
			);
		}
		if (mode === "vector") {
			if (vectorPreviewSrc) {
				return <img alt="" className="max-h-14 max-w-full object-contain" src={vectorPreviewSrc} />;
			}
			return (
				<span className="text-muted-foreground max-h-14 overflow-hidden font-mono text-xs leading-snug break-all">
					{t.length > 120 ? `${t.slice(0, 120)}…` : t}
				</span>
			);
		}
		if (mode === "emoji") {
			const inner = emojiInnerFromIconKindStored(t);
			return (
				<span className="text-2xl leading-none" style={{ fontFamily: "'Noto Color Emoji','Segoe UI Emoji',sans-serif" }}>
					{inner || "—"}
				</span>
			);
		}
		const inner = mathInnerFromIconKindStored(t);
		return <span className="text-muted-foreground max-h-14 overflow-hidden font-mono text-xs leading-snug break-all">{inner || "—"}</span>;
	})();

	return (
		<div className={cn("border-element/50 flex min-w-0 flex-col gap-2 rounded-md border p-2", locked && "pointer-events-none opacity-60")} data-slot="icon-selector">
			<Select disabled={locked} onValueChange={onModeSelect} value={activeMode}>
				<SelectTrigger className="h-8 w-full min-w-0 px-2 text-xs whitespace-normal" id={`${id}.mode`} title={modeSelectTitle}>
					<SelectValue />
				</SelectTrigger>
				<SelectContent position="popper">
					<SelectItem id={`${id}.mode.math`} value="math">
						<span className="inline-flex items-center gap-2">
							<BoardIconMathGlyphIcon aria-hidden className="size-3.5 shrink-0" />
							Math
						</span>
					</SelectItem>
					<SelectItem id={`${id}.mode.data`} value="data">
						<span className="inline-flex items-center gap-2">
							<BoardIconRasterGlyphIcon aria-hidden className="size-3.5 shrink-0" />
							Data URL
						</span>
					</SelectItem>
					<SelectItem id={`${id}.mode.emoji`} value="emoji">
						<span className="inline-flex items-center gap-2">
							<BoardIconEmojiGlyphIcon aria-hidden className="size-3.5 shrink-0" />
							Emoji
						</span>
					</SelectItem>
					<SelectItem id={`${id}.mode.vector`} value="vector">
						<span className="inline-flex items-center gap-2">
							<BoardIconCatalogGlyphIcon aria-hidden className="size-3.5 shrink-0" />
							Catalog / SVG
						</span>
					</SelectItem>
				</SelectContent>
			</Select>
			<Textarea
				className={cn("min-h-[72px] font-mono text-xs", (activeMode === "data" || activeMode === "vector") && "min-h-[88px]")}
				id={`${id}.field`}
				key={activeMode}
				mixed={!uniform}
				onChange={onEditorChange}
				placeholder={editorPlaceholder}
				readOnly={locked}
				rows={activeMode === "data" || activeMode === "vector" ? 5 : 4}
				value={editorValue}
			/>
			<div className="bg-muted/30 flex min-h-[56px] items-center justify-center overflow-hidden rounded-sm border px-1 py-2">{preview}</div>
			<div className="flex min-w-0 flex-wrap items-center justify-between gap-2">
				<Button className="h-7 shrink-0 gap-1 px-2 text-xs" disabled={locked} onClick={() => fileInputRef.current?.click()} type="button" variant="outline">
					<BoardIconFileImportIcon className="size-3.5" />
					Import file…
				</Button>
				<Button className="h-7 shrink-0 px-2 text-xs whitespace-nowrap" disabled={locked} onClick={() => onChange("")} type="button" variant="ghost">
					Clear
				</Button>
			</div>
			<input accept="image/png,image/jpeg,image/svg+xml,.svg,.png,.jpg,.jpeg" className="hidden" onChange={onPickFiles} ref={fileInputRef} type="file" />
		</div>
	);
}

// #endregion 🖼️IconSelector

// #region 📜Tree
// Hierarchical tree view with sections, items, and file trees.
// Consumers MUST wrap components in TreeStateProvider.

/**
 * TreeStateContextValue holds the data fields for a TreeStateContextValue record.
 **/
interface TreeStateContextValue {
  openStates: Record<string, boolean>;
  setOpenState: (id: string, open: boolean) => void;
  getOpenState: (id: string, defaultOpen: boolean) => boolean;
}

/**
 * TreeStateContext holds the data fields for a TreeStateContext record.
 **/
const TreeStateContext = reactHostPort.createContext<TreeStateContextValue | null>(null);

/**
 * Context provider managing tree expansion state.
 **/
export const TreeStateProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [openStates, setOpenStates] = reactHostPort.useState<Record<string, boolean>>({});

  const setOpenState = (id: string, open: boolean) => {
    setOpenStates((prev) => ({ ...prev, [id]: open }));
  };

  const getOpenState = (id: string, defaultOpen: boolean) => {
    return openStates[id] !== undefined ? openStates[id] : defaultOpen;
  };

  return <TreeStateContext.Provider value={{ openStates, setOpenState, getOpenState }}>{children}</TreeStateContext.Provider>;
};

/**
 * Hook returning tree expansion state and toggle functions.
 **/
export const useTreeState = () => {
  const context = reactHostPort.useContext(TreeStateContext);
  if (!context) throw new Error("useTreeState must be used within TreeStateProvider");
  return context;
};

const useTreeOpenState = (itemId: string, defaultOpen: boolean) => {
  const treeState = reactHostPort.useContext(TreeStateContext);
  const [fallbackOpen, setFallbackOpen] = reactHostPort.useState(defaultOpen);
  const open = treeState ? treeState.getOpenState(itemId, defaultOpen) : fallbackOpen;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      if (treeState) {
        treeState.setOpenState(itemId, value);
        return;
      }
      setFallbackOpen(value);
    },
    [itemId, treeState],
  );
  return { open, setOpen };
};

const treeSectionElementMarker = Symbol.for("semio.tree.section");

type TreeComponentMarker = {
  [treeSectionElementMarker]?: boolean;
  displayName?: string;
};

const isTreeSectionElementType = (value: unknown): boolean => {
  if ((typeof value !== "function" && typeof value !== "object") || value === null) {
    return false;
  }
  return Boolean((value as TreeComponentMarker)[treeSectionElementMarker]);
};

const hasNonEmptyChildren = (children: React.ReactNode): boolean => {
  if (!children) return false;
  const childArray = React.Children.toArray(children);
  return (
    childArray.length > 0 &&
    childArray.some((child) => {
      if (React.isValidElement(child)) return true;
      if (typeof child === "string" && child.trim().length > 0) return true;
      if (typeof child === "number") return true;
      return false;
    })
  );
};

const isIgnorableTreeChild = (child: React.ReactNode): boolean => child === null || child === undefined || typeof child === "boolean" || (typeof child === "string" && child.trim().length === 0);

const assertNoNestedTreeSections = (children: React.ReactNode, ownerName: "TreeSection" | "TreeItem") => {
  const visitNestedChildren = (value: React.ReactNode) => {
    React.Children.forEach(value, (child) => {
      if (isIgnorableTreeChild(child)) {
        return;
      }
      if (!React.isValidElement(child)) {
        return;
      }
      const childProps = child.props as { children?: React.ReactNode };
      if (child.type === React.Fragment) {
        visitNestedChildren(childProps.children);
        return;
      }
      if (isTreeSectionElementType(child.type)) {
        throw new Error(`${ownerName} cannot contain a TreeSection. Only TreeItem elements can be nested.`);
      }
      visitNestedChildren(childProps.children);
    });
  };

  visitNestedChildren(children);
};

const TreeContext = reactHostPort.createContext<{ level: number; isLastAtLevel: boolean[]; showLines: boolean; isTree: boolean; indentMultiplier: number }>({ level: 0, isLastAtLevel: [], showLines: true, isTree: false, indentMultiplier: 1 });
const TreeRowAlignmentContext = reactHostPort.createContext(false);
// True when children are rendered inside the value column of a Label property row.
const PropertyValueColumnContext = reactHostPort.createContext(false);
const detailPanelIndentPx = (level: number, multiplier = 1): number => level * 10 * multiplier;
const treeRowHeightPx = 24;
const detailPanelHeaderLineCenterPx = treeRowHeightPx / 2;
const detailPanelPropertyLabelColumnWidthPx = 96;
const detailPanelPropertyInlineGapPx = 8;
const detailPanelPropertyStackedRowGapPx = 4;
const detailPanelPropertyStackedToInlineHysteresisPx = 24;
const detailPanelPropertyRowClassName = "group grid min-w-0 items-start gap-x-[8px] min-h-[24px]";
const detailPanelPropertyControlClassName =
  "min-w-0 w-full self-start flex items-stretch justify-end [&_[data-detail-panel-control='fill']]:min-w-0 [&_[data-detail-panel-control='fill']]:w-full [&_[data-detail-panel-control='fit']]:ml-auto [&_[data-detail-panel-control='fit']]:max-w-full [&_[data-detail-panel-control='fit']]:shrink-0";
const treeInspectorInnerRowClassName = "min-w-0 w-full";
const treeHeaderRowClassName = "flex min-w-0 w-full items-center gap-[6px]";
const treeHeaderMainClassName = "flex min-w-0 flex-1 items-center gap-[6px]";
const treeHeaderActionsClassName = "flex flex-shrink-0 items-center gap-single";
const indentationLinePx = (i: number, multiplier = 1): number => detailPanelIndentPx(i, multiplier) + 7;
const treeRowInlineGapPx = 6;
const treeToggleSlotWidthPx = 14;
const treeRowVerticalPaddingPx = 0;
const treeBranchRowGapPx = 0;
const treeSectionContentPaddingTopPx = 0;
const treeItemContentPaddingTopPx = 0;
const treeCompactSiblingGapPx = 0;
const treeSubtreeGapPx = 0;
const treeSectionBoundaryGapPx = 10;
const treeGutterToContentGapPx = treeRowInlineGapPx;
const treeItemLabelStyle: React.CSSProperties = {};
const treeGutterSlotLeftPx = (level: number, extraLeftPx = 0, multiplier = 1): number => detailPanelIndentPx(level, multiplier) + extraLeftPx;
const treeGutterAnchorTop = (anchorOffsetPx?: number): string => (anchorOffsetPx === undefined ? "50%" : `${anchorOffsetPx}px`);
const treeGutterSlotStyle = (level: number, extraLeftPx = 0, multiplier = 1, anchorOffsetPx?: number): React.CSSProperties => ({
  top: treeGutterAnchorTop(anchorOffsetPx),
  left: `${treeGutterSlotLeftPx(level, extraLeftPx, multiplier)}px`,
});
const treeGutterWidthPx = (level: number, multiplier = 1): number => detailPanelIndentPx(level, multiplier) + treeToggleSlotWidthPx;
const treeBranchContentStyle = (topPaddingPx = 0): React.CSSProperties => ({
  rowGap: `${treeBranchRowGapPx}px`,
  ...(topPaddingPx > 0 ? { paddingTop: `${topPaddingPx}px` } : {}),
});
const getTreeSiblingGapPx = (_previousKind: string, _currentKind: string): number => treeCompactSiblingGapPx;
const treeAlignedRowStyle = (level: number, multiplier = 1): React.CSSProperties => ({
  gridTemplateColumns: `${treeGutterWidthPx(level, multiplier)}px minmax(0, 1fr)`,
  columnGap: `${treeGutterToContentGapPx}px`,
});

/** IndentationLines holds the data fields for a IndentationLines record.
 **/
/**
 **/
const IndentationLines: React.FC<{ level: number; showLines: boolean }> = ({ level, showLines }) => {
  const { indentMultiplier, isLastAtLevel } = reactHostPort.useContext(TreeContext);
  if (!showLines || level === 0) return null;

  const guideIndices = Array.from({ length: level }, (_, index) => index).filter((index) => !isLastAtLevel[index]);
  return (
    <div data-slot="tree-guide" className="absolute left-0 top-0 bottom-0 pointer-events-none">
      {guideIndices.map((guideIndex) => (
        <div key={guideIndex} className="absolute top-0 bottom-0" style={{ left: `${indentationLinePx(guideIndex, indentMultiplier) - 0.5}px` }}>
          <div data-tree-guide-line="" className="w-px h-full bg-muted-foreground/40 transition-[width,background-color] duration-150" />
        </div>
      ))}
    </div>
  );
};

interface TreeHierarchyGutterProps {
  level: number;
  showLines: boolean;
  slot?: React.ReactNode;
  connectCurrentLevel?: boolean;
  extendCurrentLevelToBottom?: boolean;
  slotOffsetPx?: number;
  anchorOffsetPx?: number;
}

const TreeHierarchyGutter: React.FC<TreeHierarchyGutterProps> = ({ level, showLines, slot, connectCurrentLevel = false, extendCurrentLevelToBottom = false, slotOffsetPx = 0, anchorOffsetPx }) => {
  const { indentMultiplier } = reactHostPort.useContext(TreeContext);
  const currentGuidePx = indentationLinePx(level, indentMultiplier);
  const parentGuidePx = level > 0 ? indentationLinePx(level - 1, indentMultiplier) : 0;
  const hasSlot = slot !== null && slot !== undefined && slot !== false;
  const slotLeftPx = treeGutterSlotLeftPx(level, slotOffsetPx, indentMultiplier);
  const elbowEndPx = hasSlot ? slotLeftPx : currentGuidePx;
  const elbowWidthPx = Math.max(elbowEndPx - parentGuidePx, 0);
  const gutterWidthPx = treeGutterWidthPx(level, indentMultiplier);
  const positionedSlot =
    hasSlot && React.isValidElement(slot) ? (
      React.cloneElement(slot as React.ReactElement<any>, {
        ...(slot as React.ReactElement<any>).props,
        "data-slot": (slot as React.ReactElement<any>).props["data-slot"] ?? "tree-gutter-slot",
        className: cn("absolute -translate-y-1/2", (slot as React.ReactElement<any>).props.className),
        style: { ...treeGutterSlotStyle(level, slotOffsetPx, indentMultiplier, anchorOffsetPx), ...(slot as React.ReactElement<any>).props.style },
      })
    ) : hasSlot ? (
      <span data-slot="tree-gutter-slot" className="pointer-events-none absolute -translate-y-1/2" style={treeGutterSlotStyle(level, slotOffsetPx, indentMultiplier, anchorOffsetPx)}>
        {slot}
      </span>
    ) : null;

  return (
    <div data-slot="tree-gutter" className="relative min-h-full" style={{ width: `${gutterWidthPx}px`, minWidth: `${gutterWidthPx}px` }}>
      {showLines && level > 0 && connectCurrentLevel && (
        <div
          data-slot="tree-branch-elbow"
          className="pointer-events-none absolute h-px bg-muted-foreground/40 -translate-y-1/2 transition-[height,background-color] duration-150"
          style={{ top: treeGutterAnchorTop(anchorOffsetPx), left: `${parentGuidePx}px`, width: `${elbowWidthPx}px` }}
        />
      )}
      {showLines && level > 0 && extendCurrentLevelToBottom && (
        <div
          data-slot="tree-branch-stem"
          className="pointer-events-none absolute w-px bg-muted-foreground/40 transition-[height,background-color] duration-150"
          style={{ top: treeGutterAnchorTop(anchorOffsetPx), left: `${currentGuidePx - 0.5}px`, bottom: "0px" }}
        />
      )}
      {positionedSlot}
    </div>
  );
};

interface TreeAlignedRowProps {
  level: number;
  isLastAtLevel: boolean[];
  showLines: boolean;
  slot?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
  contentClassName?: string;
  align?: "start" | "center";
  connectCurrentLevel?: boolean;
  extendCurrentLevelToBottom?: boolean;
  slotOffsetPx?: number;
  anchorOffsetPx?: number;
}

const TreeAlignedRow: React.FC<TreeAlignedRowProps> = ({
  level,
  isLastAtLevel,
  showLines,
  slot,
  children,
  className,
  contentClassName,
  align = "center",
  connectCurrentLevel = false,
  extendCurrentLevelToBottom = false,
  slotOffsetPx = 0,
  anchorOffsetPx,
}) => {
  const { indentMultiplier } = reactHostPort.useContext(TreeContext);
  return (
    <div data-slot="tree-row-layout" className={cn("grid min-w-0", align === "start" ? "items-start" : "items-center", className)} style={treeAlignedRowStyle(level, indentMultiplier)}>
      <TreeHierarchyGutter level={level} showLines={showLines} slot={slot} connectCurrentLevel={connectCurrentLevel} extendCurrentLevelToBottom={extendCurrentLevelToBottom} slotOffsetPx={slotOffsetPx} anchorOffsetPx={anchorOffsetPx} />
      <div data-slot="tree-row-content" className={cn("min-w-0", contentClassName)}>
        {children}
      </div>
    </div>
  );
};

/**
 * Wrapper rendering tree children with connecting lines.
 **/
export const TreeContent: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { level, isLastAtLevel, showLines } = reactHostPort.useContext(TreeContext);
  return (
    <div data-slot="tree-content" data-tree-row-kind="content" className="relative" style={{ paddingTop: `${treeRowVerticalPaddingPx}px`, paddingBottom: `${treeRowVerticalPaddingPx}px` }}>
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0}>
        {children}
      </TreeAlignedRow>
    </div>
  );
};

interface TreeBranchContentProps {
  slot: string;
  children: React.ReactNode;
  className?: string;
  topPaddingPx?: number;
  ownerRowKind?: string;
  ownerExpanded?: boolean;
}

const TreeBranchContent: React.FC<TreeBranchContentProps> = ({ slot, children, className, topPaddingPx = 0, ownerRowKind, ownerExpanded = false }) => {
  const { level, showLines, isTree } = reactHostPort.useContext(TreeContext);
  const branchRef = reactHostPort.useRef<HTMLDivElement>(null);
  reactHostPort.useLayoutEffect(() => {
    const branchElement = branchRef.current;
    if (!branchElement || !isTree) {
      return;
    }

    const branchSlots = new Set(["tree-section-content", "tree-item-content", "tree-property-content", "control-tree-folder-content"]);
    const rowSlots = new Set(["tree-item-row", "tree-section-row", "tree-property-item", "tree-row", "tree-content", "control-tree-row"]);
    const directChildren = Array.from(branchElement.children) as HTMLElement[];
    const isRowElement = (el: HTMLElement): boolean => rowSlots.has(el.dataset.slot ?? "");
    const isBranchElement = (el: HTMLElement): boolean => branchSlots.has(el.dataset.slot ?? "");
    const getRowKind = (el: HTMLElement): string => el.dataset.treeRowKind ?? "leaf";
    const setMarginTop = (el: HTMLElement, marginTopPx: number) => {
      el.style.marginTop = marginTopPx > 0 ? `${marginTopPx}px` : "0px";
    };

    for (const child of directChildren) {
      setMarginTop(child, 0);
    }

    let previousDirect: HTMLElement | null = null;
    for (const child of directChildren) {
      if (!previousDirect) {
        previousDirect = child;
        continue;
      }

      if (isBranchElement(child)) {
        setMarginTop(child, treeSubtreeGapPx);
        previousDirect = child;
        continue;
      }

      if (!isRowElement(child)) {
        previousDirect = child;
        continue;
      }

      if (isBranchElement(previousDirect)) {
        setMarginTop(child, treeSubtreeGapPx);
        previousDirect = child;
        continue;
      }

      if (isRowElement(previousDirect)) {
        const currentKind = getRowKind(child);
        const previousKind = getRowKind(previousDirect);
        setMarginTop(child, getTreeSiblingGapPx(previousKind, currentKind));
      }

      previousDirect = child;
    }
  }, [children, isTree]);

  return (
    <div ref={branchRef} data-slot={slot} data-tree-owner-kind={ownerRowKind} data-tree-owner-expanded={ownerExpanded ? "true" : "false"} className={cn("relative flex min-w-0 flex-col", className)} style={treeBranchContentStyle(topPaddingPx)}>
      {isTree ? <IndentationLines level={level} showLines={showLines} /> : null}
      {children}
    </div>
  );
};

/**
 * Configuration interface for an action button on a tree section.
 **/
export interface TreeSectionAction {
  kind?: "button";
  icon: React.ReactNode;
  onClick: () => void;
  title?: string;
  id?: string;
}

/**
 * Configuration interface for a checkbox action on a tree header row.
 **/
export interface TreeCheckboxAction {
  kind: "checkbox";
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  title?: string;
  id?: string;
  disabled?: boolean;
  ariaLabel?: string;
}

export type TreeHeaderAction = TreeSectionAction | TreeCheckboxAction;

const renderTreeHeaderActions = (actions: TreeHeaderAction[]) => (
  <div data-slot="tree-header-actions" className={treeHeaderActionsClassName}>
    {actions.map((action, index) =>
      action.kind === "checkbox" ? (
        <label
          key={action.id ?? index}
          data-slot="tree-action-checkbox-wrapper"
          className="inline-flex h-[22px] min-w-[14px] flex-shrink-0 cursor-pointer items-center justify-center"
          title={action.title}
          onClick={(event) => {
            event.preventDefault();
            event.stopPropagation();
          }}
        >
          <input
            data-slot="tree-action-checkbox"
            id={action.id}
            type="checkbox"
            className="m-0 size-[12px] cursor-pointer accent-foreground"
            aria-label={action.ariaLabel ?? action.title ?? action.id ?? "Toggle tree item"}
            checked={action.checked}
            disabled={action.disabled}
            onChange={(event) => {
              event.stopPropagation();
              action.onCheckedChange(event.currentTarget.checked);
            }}
          />
        </label>
      ) : (
        <Action
          key={action.id ?? index}
          onClick={(event) => {
            event.preventDefault();
            event.stopPropagation();
            action.onClick();
          }}
          id={action.id}
          icon={action.icon}
        />
      ),
    )}
  </div>
);

const TreeDragHandle: React.FC<{
  attributes?: Record<string, unknown> | object;
  listeners?: Record<string, unknown>;
  onClick?: React.MouseEventHandler<HTMLButtonElement>;
}> = ({ attributes, listeners, onClick }) => (
  <button
    type="button"
    data-slot="tree-drag-handle"
    className="text-muted-foreground inline-flex h-[22px] min-w-[14px] flex-shrink-0 cursor-grab items-center justify-center border-0 bg-transparent p-0 outline-none active:cursor-grabbing"
    onClick={onClick}
    {...(attributes as React.ComponentProps<"button">)}
    {...(listeners as React.ComponentProps<"button">)}
  >
    <GripVerticalIcon size={12} className="text-muted-foreground" />
  </button>
);

export enum TreeItemCollapsibleState {
  None = 0,
  Collapsed = 1,
  Expanded = 2,
}

export type TreeSelectionMode = "single" | "multiple";

export interface TreeDataActivationContext {
  path: string[];
  selectedIds: string[];
  sectionId: string;
}

export interface TreeDataItem {
  id: string;
  label: React.ReactNode;
  icon?: React.ReactNode;
  description?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  /** Alternative branches for this item. Each branch is an array of child items. Navigation < > switches between branches. */
  alternatives?: TreeDataItem[][];
  actions?: TreeHeaderAction[];
  className?: string;
  isHighlighted?: boolean;
  isSelected?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  collapsibleState?: TreeItemCollapsibleState;
  emptyState?: React.ReactNode;
  draggable?: boolean;
  onClick?: (event: React.MouseEvent, context: TreeDataActivationContext) => void;
  onDoubleClick?: (event: React.MouseEvent, context: TreeDataActivationContext) => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
}

export interface TreeDataSection {
  id: string;
  label?: React.ReactNode;
  icon?: React.ReactNode;
  content?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  actions?: TreeHeaderAction[];
  className?: string;
  defaultOpen?: boolean;
  emptyState?: React.ReactNode;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
}

export interface TreeDragAndDropController {
  getDragData?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => Record<string, string> | undefined;
  onDragStart?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => void;
  onDragEnd?: (context: { items: TreeDataItem[]; sourceItem: TreeDataItem; section: TreeDataSection }) => void;
  handleDrop?: (context: { target: TreeDataItem | TreeDataSection; targetKind: "item" | "section"; data: Record<string, string>; sourceItems: TreeDataItem[]; section: TreeDataSection }) => void | Promise<void>;
}

interface TreeSelectionComputationArgs {
  selectionMode: TreeSelectionMode;
  selectedIds: string[];
  orderedIds: string[];
  targetId: string;
  anchorId?: string;
  additiveKey: boolean;
  rangeKey: boolean;
}

interface TreeSelectionComputationResult {
  selectedIds: string[];
  anchorId?: string;
}

const normalizeTreeSelectedIds = (selectedIds: string[], selectionMode: TreeSelectionMode): string[] => {
  const uniqueIds = Array.from(new Set(selectedIds.filter(Boolean)));
  return selectionMode === "single" ? uniqueIds.slice(0, 1) : uniqueIds;
};

const getTreeItemDefaultOpen = (item: TreeDataItem): boolean => item.defaultOpen ?? item.collapsibleState === TreeItemCollapsibleState.Expanded;

const getTreeNextSelectionState = ({ selectionMode, selectedIds, orderedIds, targetId, anchorId, additiveKey, rangeKey }: TreeSelectionComputationArgs): TreeSelectionComputationResult => {
  if (selectionMode === "single") {
    return { selectedIds: [targetId], anchorId: targetId };
  }

  if (rangeKey) {
    const fallbackAnchorId = anchorId ?? selectedIds[selectedIds.length - 1] ?? targetId;
    const anchorIndex = orderedIds.indexOf(fallbackAnchorId);
    const targetIndex = orderedIds.indexOf(targetId);
    if (anchorIndex !== -1 && targetIndex !== -1) {
      const startIndex = Math.min(anchorIndex, targetIndex);
      const endIndex = Math.max(anchorIndex, targetIndex);
      return { selectedIds: orderedIds.slice(startIndex, endIndex + 1), anchorId: fallbackAnchorId };
    }
  }

  if (additiveKey) {
    const nextSelectedIds = selectedIds.includes(targetId) ? selectedIds.filter((id) => id !== targetId) : [...selectedIds, targetId];
    return { selectedIds: nextSelectedIds, anchorId: targetId };
  }

  return { selectedIds: [targetId], anchorId: targetId };
};

const collectTreeItemMap = (items: TreeDataItem[], map: Record<string, TreeDataItem> = {}): Record<string, TreeDataItem> => {
  items.forEach((item) => {
    map[item.id] = item;
    if (item.items?.length) {
      collectTreeItemMap(item.items, map);
    }
  });
  return map;
};

/**
 * TreeSectionProps holds the data fields for a TreeSectionProps record.
 **/
interface TreeSectionProps {
  label?: React.ReactNode;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  expandable?: boolean;
  loading?: boolean;
  className?: string;
  actions?: TreeHeaderAction[];
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
}

/**
 * SortableTreeItemProps holds the data fields for a SortableTreeItemProps record.
 **/
interface SortableTreeItemProps {
  id: string;
  label?: React.ReactNode;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  isLastItem?: boolean;
  actions?: TreeHeaderAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  layoutKind?: "default" | "property";
}

/**
 * TreeItemProps holds the data fields for a TreeItemProps record.
 **/
interface TreeItemProps {
  label?: React.ReactNode;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  sortable?: boolean;
  sortableId?: string;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  expandable?: boolean;
  loading?: boolean;
  isLastItem?: boolean;
  actions?: TreeHeaderAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
  /** Total number of alternative branches. When > 0, shows branch navigation. */
  branchCount?: number;
  /** Currently active branch index (0-based). */
  activeBranchIndex?: number;
  /** Callback when the user navigates to a different branch. */
  onBranchChange?: (index: number) => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  layoutKind?: "default" | "property";
}

/**
 * SortableTreeItemsProps holds the data fields for a SortableTreeItemsProps record.
 **/
interface SortableTreeItemsProps {
  items: { id: string; [key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => React.ReactNode;
}

/**
 * TreeRootProps holds the data fields for a TreeRootProps record.
 **/
interface TreeRootProps {
  className?: string;
  showLines?: boolean;
  sections?: TreeDataSection[];
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  dragAndDropController?: TreeDragAndDropController;
  emptyState?: React.ReactNode;
  indentMultiplier?: number;
}

/**
 * TreeDataRenderingContextValue holds the data fields for a TreeDataRenderingContextValue record.
 **/
interface TreeDataRenderingContextValue {
  sectionItemsById: Record<string, TreeDataItem[]>;
  itemItemsById: Record<string, TreeDataItem[]>;
  loadingById: Record<string, boolean>;
  selectedIds: string[];
  draggedIds: string[];
  loadSectionItems: (section: TreeDataSection) => Promise<void>;
  loadItemItems: (item: TreeDataItem) => Promise<void>;
  handleSelectItem: (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => void;
  handleDoubleClickItem: (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => void;
  handleDragStart: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  handleDragEnd: (item: TreeDataItem, section: TreeDataSection) => void;
  handleDropOnItem: (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => void;
  handleDropOnSection: (event: React.DragEvent<HTMLDivElement>, section: TreeDataSection) => void;
  handleDragOver: (event: React.DragEvent<HTMLDivElement>) => void;
}

const TreeDataRenderingContext = reactHostPort.createContext<TreeDataRenderingContextValue | null>(null);

const treeDefaultDragMimeKind = "application/vnd.code.tree.item";

const getTreeSectionStateId = (sectionId: string): string => `tree-section-${sectionId}`;

const getTreeItemStateId = (itemId: string): string => `tree-item-${itemId}`;

const getTreeSectionLoadingId = (sectionId: string): string => `tree-section-loading-${sectionId}`;

const getTreeItemLoadingId = (itemId: string): string => `tree-item-loading-${itemId}`;

const getTreeSectionItems = (section: TreeDataSection, sectionItemsById: Record<string, TreeDataItem[]>): TreeDataItem[] => sectionItemsById[section.id] ?? section.items ?? [];

const getTreeItemItems = (item: TreeDataItem, itemItemsById: Record<string, TreeDataItem[]>): TreeDataItem[] => itemItemsById[item.id] ?? item.items ?? [];

const getTreeItemOrderedIds = (sections: TreeDataSection[], sectionItemsById: Record<string, TreeDataItem[]>, itemItemsById: Record<string, TreeDataItem[]>): string[] => {
  const orderedIds: string[] = [];

  const visitItems = (items: TreeDataItem[]) => {
    items.forEach((item) => {
      orderedIds.push(item.id);
      const childItems = getTreeItemItems(item, itemItemsById);
      if (childItems.length > 0) {
        visitItems(childItems);
      }
    });
  };

  sections.forEach((section) => {
    visitItems(getTreeSectionItems(section, sectionItemsById));
  });

  return orderedIds;
};

/**
 * Collapsible tree section header with optional action buttons.
 **/
export const TreeSection: React.FC<TreeSectionProps> = ({
  label,
  id,
  icon,
  children,
  defaultOpen = true,
  open: controlledOpen,
  onOpenChange,
  expandable,
  loading = false,
  className = "",
  actions = [],
  onPointerEnter: onSectionPointerEnter,
  onPointerLeave: onSectionPointerLeave,
  onDoubleClick,
  draggable = false,
  onDragStart,
  onDragOver,
  onDragLeave,
  onDrop,
}) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const suppressLocalizedLabel = label === "";
  const resolvedLabel = label === "" ? undefined : label;
  const localizedLabel = !suppressLocalizedLabel && resolvedLabel === undefined && id ? useLabel(id) : undefined;
  const displayLabel = resolvedLabel ?? localizedLabel;
  assertNoNestedTreeSections(children, "TreeSection");
  const sectionStateId = getTreeSectionStateId(id ?? String(displayLabel ?? "section"));
  const treeOpenState = useTreeOpenState(sectionStateId, defaultOpen);
  const open = controlledOpen ?? treeOpenState.open;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      treeOpenState.setOpen(value);
      onOpenChange?.(value);
    },
    [onOpenChange, treeOpenState],
  );
  const [isHovered, setIsHovered] = reactHostPort.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const isExpandable = expandable ?? hasChildren;
  const isHeaderlessSection = displayLabel === undefined && !icon && actions.length === 0 && !loading && !draggable && !onDoubleClick && !onSectionPointerEnter && !onSectionPointerLeave && !onDragStart && !onDragOver && !onDragLeave && !onDrop;
  const rowClassName = cn("relative h-[24px] min-h-[24px] hover:bg-hover-panel select-none overflow-hidden group min-w-0", isExpandable ? "cursor-foldable" : "cursor-selectable", className);

  if (isHeaderlessSection) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree, indentMultiplier }}>{children}</TreeContext.Provider>;
  }

  if (!isExpandable) {
    return (
      <div
        data-slot="tree-section-row"
        data-tree-row-kind="section"
        id={id}
        className={rowClassName}
        draggable={draggable}
        onPointerEnter={() => {
          setIsHovered(true);
          onSectionPointerEnter?.();
        }}
        onPointerLeave={() => {
          setIsHovered(false);
          onSectionPointerLeave?.();
        }}
        onDragStart={onDragStart}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
      >
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slot={loading ? <Spinner size="small" className="text-muted-foreground" /> : null} contentClassName="min-w-0">
          <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
            <div className={treeHeaderMainClassName}>
              {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
              {id ? (
                <Tooltip>
                  <TooltipTrigger asChild>
                    <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate select-text" style={treeItemLabelStyle}>
                      {displayLabel}
                    </span>
                  </TooltipTrigger>
                  <TooltipContent>
                    <DescriptionTooltipContent id={id} />
                  </TooltipContent>
                </Tooltip>
              ) : (
                <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate select-text" style={treeItemLabelStyle}>
                  {displayLabel}
                </span>
              )}
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
          </div>
        </TreeAlignedRow>
      </div>
    );
  }

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      <CollapsibleTrigger asChild>
        <div
          data-slot="tree-section-row"
          data-tree-row-kind="section"
          id={id}
          className={rowClassName}
          role="button"
          draggable={draggable}
          onPointerEnter={() => {
            setIsHovered(true);
            onSectionPointerEnter?.();
          }}
          onPointerLeave={() => {
            setIsHovered(false);
            onSectionPointerLeave?.();
          }}
          onDragStart={onDragStart}
          onDragOver={onDragOver}
          onDragLeave={onDragLeave}
          onDrop={onDrop}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
        >
          <TreeAlignedRow
            level={level}
            isLastAtLevel={isLastAtLevel}
            showLines={showLines}
            connectCurrentLevel={level > 0}
            extendCurrentLevelToBottom={open && hasChildren}
            slot={loading ? <Spinner size="small" className="text-muted-foreground" /> : open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
            contentClassName="min-w-0"
          >
            <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
              <div className={treeHeaderMainClassName}>
                {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                {id ? (
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate select-text" style={treeItemLabelStyle}>
                        {displayLabel}
                      </span>
                    </TooltipTrigger>
                    <TooltipContent>
                      <DescriptionTooltipContent id={id} />
                    </TooltipContent>
                  </Tooltip>
                ) : (
                  <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate select-text" style={treeItemLabelStyle}>
                    {displayLabel}
                  </span>
                )}
              </div>
              {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
            </div>
          </TreeAlignedRow>
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent className="min-w-0">
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree, indentMultiplier }}>
          <TreeBranchContent slot="tree-section-content" ownerRowKind="section" ownerExpanded={open && hasChildren} topPaddingPx={treeSectionContentPaddingTopPx}>
            {children}
          </TreeBranchContent>
        </TreeContext.Provider>
      </CollapsibleContent>
    </Collapsible>
  );
};

(TreeSection as TreeComponentMarker)[treeSectionElementMarker] = true;
TreeSection.displayName = "TreeSection";

/**
 * SortableTreeItem holds the data fields for a SortableTreeItem record.
 **/
const SortableTreeItem: React.FC<SortableTreeItemProps> = ({
  id,
  label,
  icon,
  children,
  onClick,
  className = "",
  isSelected = false,
  isHighlighted = false,
  isDragHandle = false,
  defaultOpen = true,
  isLastItem = false,
  actions = [],
  onDoubleClick,
  layoutKind = "default",
}) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const localizedLabel = id ? useLabel(id) : undefined;
  const displayLabel = label ?? localizedLabel;
  const itemKey = id ?? displayLabel ?? id;
  const itemId = `item-${id}-${itemKey}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const [isHovered, setIsHovered] = reactHostPort.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  const baseClasses = `relative w-full min-h-[24px] hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${hasChildren ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-active-base text-active-foreground" : ""} ${isHighlighted ? "bg-active-base text-active-foreground" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren && displayLabel) {
    if (layoutKind === "property") {
      return (
        <>
          <div
            data-slot="tree-item-row"
            data-tree-row-kind="group"
            data-tree-group
            role="treeitem"
            id={id}
            ref={setNodeRef}
            style={style}
            className={itemClasses}
            onDoubleClick={(event) => {
              if (!onDoubleClick) return;
              event.preventDefault();
              event.stopPropagation();
              onDoubleClick(event);
            }}
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
          >
            <TreeAlignedRow
              level={level}
              isLastAtLevel={isLastAtLevel}
              showLines={showLines}
              connectCurrentLevel={level > 0}
              extendCurrentLevelToBottom={open && hasChildren}
              slot={
                <button
                  className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    setOpen(!open);
                  }}
                >
                  {open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
                </button>
              }
              contentClassName="min-w-0"
            >
              <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
                <div className={treeHeaderMainClassName}>
                  {isDragHandle && <TreeDragHandle attributes={attributes} listeners={listeners} onClick={(e) => e.stopPropagation()} />}
                  {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                  <span
                    data-slot="tree-label"
                    className="flex-1 text-xs font-normal truncate text-foreground cursor-selectable select-text"
                    style={treeItemLabelStyle}
                    onClick={(e) => {
                      if (e.detail > 1) return;
                      e.preventDefault();
                      e.stopPropagation();
                      onClick?.(e);
                    }}
                  >
                    {displayLabel as React.ReactNode}
                  </span>
                </div>
                {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
              </div>
            </TreeAlignedRow>
          </div>
          {open && (
            <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier }}>
              <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} className="min-w-0" topPaddingPx={treeItemContentPaddingTopPx}>
                {children}
              </TreeBranchContent>
            </TreeContext.Provider>
          )}
        </>
      );
    }

    return (
      <>
        <div
          data-slot="tree-item-row"
          data-tree-row-kind="group"
          data-tree-group
          role="treeitem"
          id={id}
          ref={setNodeRef}
          style={style}
          className={itemClasses}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
          onMouseEnter={() => setIsHovered(true)}
          onMouseLeave={() => setIsHovered(false)}
        >
          <TreeAlignedRow
            level={level}
            isLastAtLevel={isLastAtLevel}
            showLines={showLines}
            connectCurrentLevel={level > 0}
            extendCurrentLevelToBottom={open && hasChildren}
            slot={
              <button
                className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  setOpen(!open);
                }}
              >
                {open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
              </button>
            }
            contentClassName="min-w-0"
          >
            <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
              <div className={treeHeaderMainClassName}>
                {isDragHandle && <TreeDragHandle attributes={attributes} listeners={listeners} onClick={(e) => e.stopPropagation()} />}
                {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                <span
                  data-slot="tree-label"
                  className="flex-1 text-xs font-normal truncate text-foreground cursor-selectable select-text"
                  style={treeItemLabelStyle}
                  onClick={(e) => {
                    if (e.detail > 1) return;
                    e.preventDefault();
                    e.stopPropagation();
                    onClick?.(e);
                  }}
                >
                  {displayLabel as React.ReactNode}
                </span>
              </div>
              {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
            </div>
          </TreeAlignedRow>
        </div>
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier }}>
            <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} topPaddingPx={treeItemContentPaddingTopPx}>
              {children}
            </TreeBranchContent>
          </TreeContext.Provider>
        )}
      </>
    );
  }

  if (!displayLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree, indentMultiplier }}>{children}</TreeContext.Provider>;
  }

  if (layoutKind === "property") {
    return (
      <div
        data-slot="tree-item-row"
        data-tree-row-kind="property"
        role="treeitem"
        id={id}
        ref={setNodeRef}
        style={style}
        className={itemClasses}
        onClick={(event) => {
          if (event.detail > 1) return;
          onClick?.(event);
        }}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
        onMouseEnter={() => setIsHovered(true)}
        onMouseLeave={() => setIsHovered(false)}
      >
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0">
          <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
            <div className={treeHeaderMainClassName}>
              {isDragHandle && <TreeDragHandle attributes={attributes} listeners={listeners} />}
              {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
              <span data-slot="tree-label" className="flex-1 text-xs font-normal truncate text-foreground select-text" style={treeItemLabelStyle}>
                {displayLabel as React.ReactNode}
              </span>
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
          </div>
        </TreeAlignedRow>
      </div>
    );
  }

  return (
    <div
      data-slot="tree-item-row"
      data-tree-row-kind="leaf"
      role="treeitem"
      id={id}
      ref={setNodeRef}
      style={style}
      className={itemClasses}
      onClick={(event) => {
        if (event.detail > 1) return;
        onClick?.(event);
      }}
      onDoubleClick={(event) => {
        if (!onDoubleClick) return;
        event.preventDefault();
        event.stopPropagation();
        onDoubleClick(event);
      }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0">
        <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
          <div className={treeHeaderMainClassName}>
            {isDragHandle && <TreeDragHandle attributes={attributes} listeners={listeners} />}
            {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span data-slot="tree-label" className="flex-1 text-xs font-normal truncate text-foreground select-text" style={treeItemLabelStyle}>
              {displayLabel as React.ReactNode}
            </span>
          </div>
          {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
        </div>
      </TreeAlignedRow>
    </div>
  );
};

/**
 * Drag-and-drop sortable container for tree items.
 **/
export const SortableTreeItems: React.FC<SortableTreeItemsProps> = ({ items, onReorder, children }) => {
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (over && active.id !== over.id) {
      const oldIndex = items.findIndex((item) => item.id === active.id);
      const newIndex = items.findIndex((item) => item.id === over.id);
      if (oldIndex !== -1 && newIndex !== -1) {
        onReorder(oldIndex, newIndex);
      }
    }
  };

  return (
    <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <SortableContext items={items.map((item) => item.id)} strategy={verticalListSortingStrategy}>
        {items.map((item, index) => (
          <React.Fragment key={item.id}>{children(item, index)}</React.Fragment>
        ))}
      </SortableContext>
    </DndContext>
  );
};

/**
 * Single tree item row with icon, label, and interaction handlers.
 **/
export const TreeItem: React.FC<TreeItemProps> = ({
  label,
  id,
  icon,
  children,
  onClick,
  className = "",
  isSelected = false,
  isHighlighted = false,
  sortable = false,
  sortableId,
  isDragHandle = false,
  defaultOpen = true,
  isLastItem = false,
  actions = [],
  onDoubleClick,
  open: controlledOpen,
  onOpenChange,
  expandable,
  loading = false,
  draggable = false,
  onDragStart,
  onDragOver,
  onDragLeave,
  onDrop,
  branchCount = 0,
  activeBranchIndex = 0,
  onBranchChange,
  onPointerEnter,
  onPointerLeave,
  layoutKind = "default",
}) => {
  const localizedLabel = id ? useLabel(id) : undefined;
  const resolvedLabel = label !== undefined ? label : localizedLabel;
  assertNoNestedTreeSections(children, "TreeItem");
  if (sortable && sortableId) {
    return (
      <SortableTreeItem
        id={sortableId}
        label={resolvedLabel}
        icon={icon}
        className={className}
        isSelected={isSelected}
        isHighlighted={isHighlighted}
        isDragHandle={isDragHandle}
        defaultOpen={defaultOpen}
        isLastItem={isLastItem}
        actions={actions}
        onDoubleClick={onDoubleClick}
        onPointerEnter={onPointerEnter}
        onPointerLeave={onPointerLeave}
      >
        {children}
      </SortableTreeItem>
    );
  }

  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const itemKey = id ?? resolvedLabel ?? sortableId ?? "tree-item";
  const itemId = getTreeItemStateId(String(itemKey));
  const treeOpenState = useTreeOpenState(itemId, defaultOpen);
  const open = controlledOpen ?? treeOpenState.open;
  const setOpen = reactHostPort.useCallback(
    (value: boolean) => {
      treeOpenState.setOpen(value);
      onOpenChange?.(value);
    },
    [onOpenChange, treeOpenState],
  );
  const [isHovered, setIsHovered] = reactHostPort.useState(false);
  const handlePointerEnter = reactHostPort.useCallback(() => {
    setIsHovered(true);
    onPointerEnter?.();
  }, [onPointerEnter]);
  const handlePointerLeave = reactHostPort.useCallback(() => {
    setIsHovered(false);
    onPointerLeave?.();
  }, [onPointerLeave]);
  const hasChildren = hasNonEmptyChildren(children);
  const isExpandable = expandable ?? hasChildren;
  const baseClasses = `relative w-full min-h-[24px] hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${isExpandable ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-active-base text-active-foreground" : ""} ${isHighlighted ? "bg-active-base text-active-foreground" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (layoutKind === "property" && resolvedLabel) {
    return (
      <div
        data-slot="tree-property-item"
        data-tree-row-kind={isExpandable ? "group" : "property"}
        role="treeitem"
        id={id}
        data-state={open ? "open" : "closed"}
        className={cn("group min-w-0 w-full", className)}
        draggable={draggable}
        onDragStart={onDragStart}
        onDragOver={onDragOver}
        onDragLeave={onDragLeave}
        onDrop={onDrop}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
        onMouseEnter={handlePointerEnter}
        onMouseLeave={handlePointerLeave}
      >
        <TreeAlignedRow
          level={level}
          isLastAtLevel={isLastAtLevel}
          showLines={showLines}
          connectCurrentLevel={level > 0}
          extendCurrentLevelToBottom={isExpandable && open && hasChildren}
          slot={
            isExpandable ? (
              <button
                type="button"
                className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                onClick={(event) => {
                  event.preventDefault();
                  event.stopPropagation();
                  setOpen(!open);
                }}
              >
                {loading ? <Spinner size="small" className="text-muted-foreground" /> : open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
              </button>
            ) : undefined
          }
          contentClassName="min-w-0"
        >
          <div className={cn(treeHeaderRowClassName, "h-[22px]", treeInspectorInnerRowClassName)}>
            <div className={cn(treeHeaderMainClassName, "h-[22px]")}>
              {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
              {id ? (
                <Tooltip>
                  <TooltipTrigger asChild>
                    <span
                      data-slot="tree-label"
                      className={cn("flex min-w-0 flex-1 items-center text-xs font-medium text-left truncate text-foreground transition-colors hover:bg-hover-panel h-[22px] select-text", isExpandable ? "cursor-foldable" : "cursor-selectable")}
                      style={treeItemLabelStyle}
                      onClick={(event) => {
                        if (event.detail > 1) return;
                        event.preventDefault();
                        event.stopPropagation();
                        if (isExpandable) {
                          setOpen(!open);
                          return;
                        }
                        onClick?.(event);
                      }}
                    >
                      {resolvedLabel as React.ReactNode}
                    </span>
                  </TooltipTrigger>
                  <TooltipContent>
                    <DescriptionTooltipContent id={id} />
                  </TooltipContent>
                </Tooltip>
              ) : (
                <span
                  data-slot="tree-label"
                  className={cn("flex min-w-0 flex-1 items-center text-xs font-medium text-left truncate text-foreground transition-colors hover:bg-hover-panel h-[22px] select-text", isExpandable ? "cursor-foldable" : "cursor-selectable")}
                  style={treeItemLabelStyle}
                  onClick={(event) => {
                    if (event.detail > 1) return;
                    event.preventDefault();
                    event.stopPropagation();
                    if (isExpandable) {
                      setOpen(!open);
                      return;
                    }
                    onClick?.(event);
                  }}
                >
                  {resolvedLabel as React.ReactNode}
                </span>
              )}
            </div>
            {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
          </div>
        </TreeAlignedRow>
        {open ? (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier }}>
            <TreeBranchContent slot="tree-property-content" ownerRowKind={isExpandable ? "group" : "property"} ownerExpanded={open && hasChildren} className="min-w-0" topPaddingPx={treeItemContentPaddingTopPx}>
              {children}
            </TreeBranchContent>
          </TreeContext.Provider>
        ) : (
          <div data-slot="tree-property-content" className="min-w-0" />
        )}
      </div>
    );
  }

  if (isExpandable && resolvedLabel) {
    return (
      <>
        <div
          data-slot="tree-item-row"
          data-tree-row-kind="group"
          data-tree-group
          role="treeitem"
          id={id}
          className={itemClasses}
          draggable={draggable}
          onDragStart={onDragStart}
          onDragOver={onDragOver}
          onDragLeave={onDragLeave}
          onDrop={onDrop}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
          onMouseEnter={handlePointerEnter}
          onMouseLeave={handlePointerLeave}
        >
          <TreeAlignedRow
            level={level}
            isLastAtLevel={isLastAtLevel}
            showLines={showLines}
            connectCurrentLevel={level > 0}
            extendCurrentLevelToBottom={open && hasChildren}
            slot={
              <button
                className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  setOpen(!open);
                }}
              >
                {loading ? <Spinner size="small" className="text-muted-foreground" /> : open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
              </button>
            }
            contentClassName="min-w-0"
          >
            <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
              <div className={treeHeaderMainClassName}>
                {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                <span
                  data-slot="tree-label"
                  className="flex-1 text-xs font-normal truncate text-foreground cursor-selectable select-text"
                  style={treeItemLabelStyle}
                  onClick={(e) => {
                    if (e.detail > 1) return;
                    e.preventDefault();
                    e.stopPropagation();
                    onClick?.(e);
                  }}
                >
                  {resolvedLabel as React.ReactNode}
                </span>
              </div>
              {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
              {branchCount > 0 && (
                <div data-slot="tree-branch-nav" className="flex items-center gap-[2px] flex-shrink-0">
                  <button
                    data-slot="tree-branch-prev"
                    className="p-0 border-0 bg-transparent cursor-selectable disabled:opacity-30 disabled:cursor-default"
                    disabled={activeBranchIndex <= 0}
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      onBranchChange?.(activeBranchIndex - 1);
                    }}
                  >
                    <ChevronLeftIcon className="size-[12px] text-muted-foreground" />
                  </button>
                  <span data-slot="tree-branch-indicator" className="text-[10px] text-muted-foreground tabular-nums select-none">
                    {activeBranchIndex + 1}/{branchCount}
                  </span>
                  <button
                    data-slot="tree-branch-next"
                    className="p-0 border-0 bg-transparent cursor-selectable disabled:opacity-30 disabled:cursor-default"
                    disabled={activeBranchIndex >= branchCount - 1}
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      onBranchChange?.(activeBranchIndex + 1);
                    }}
                  >
                    <ChevronRightIcon className="size-[12px] text-muted-foreground" />
                  </button>
                </div>
              )}
            </div>
          </TreeAlignedRow>
        </div>
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree, indentMultiplier }}>
            <TreeBranchContent slot="tree-item-content" ownerRowKind="group" ownerExpanded={open && hasChildren} topPaddingPx={treeItemContentPaddingTopPx}>
              {children}
            </TreeBranchContent>
          </TreeContext.Provider>
        )}
      </>
    );
  }

  if (!resolvedLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree, indentMultiplier }}>{children}</TreeContext.Provider>;
  }

  return (
    <div
      data-slot="tree-item-row"
      data-tree-row-kind={layoutKind === "property" ? "property" : "leaf"}
      role="treeitem"
      id={id}
      className={itemClasses}
      draggable={draggable}
      onDragStart={onDragStart}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
      onClick={onClick}
      onMouseEnter={handlePointerEnter}
      onMouseLeave={handlePointerLeave}
    >
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0">
        <div className={cn(treeHeaderRowClassName, treeInspectorInnerRowClassName)}>
          <div className={treeHeaderMainClassName}>
            {loading && <Spinner size="small" className="text-muted-foreground" />}
            {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span data-slot="tree-label" className="flex-1 text-xs font-normal truncate text-foreground cursor-selectable select-text" style={treeItemLabelStyle}>
              {resolvedLabel as React.ReactNode}
            </span>
          </div>
          {actions.length > 0 ? renderTreeHeaderActions(actions) : null}
          {branchCount > 0 && (
            <div data-slot="tree-branch-nav" className="flex items-center gap-[2px] flex-shrink-0">
              <button
                data-slot="tree-branch-prev"
                className="p-0 border-0 bg-transparent cursor-selectable disabled:opacity-30 disabled:cursor-default"
                disabled={activeBranchIndex <= 0}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  onBranchChange?.(activeBranchIndex - 1);
                }}
              >
                <ChevronLeftIcon className="size-[12px] text-muted-foreground" />
              </button>
              <span data-slot="tree-branch-indicator" className="text-[10px] text-muted-foreground tabular-nums select-none">
                {activeBranchIndex + 1}/{branchCount}
              </span>
              <button
                data-slot="tree-branch-next"
                className="p-0 border-0 bg-transparent cursor-selectable disabled:opacity-30 disabled:cursor-default"
                disabled={activeBranchIndex >= branchCount - 1}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  onBranchChange?.(activeBranchIndex + 1);
                }}
              >
                <ChevronRightIcon className="size-[12px] text-muted-foreground" />
              </button>
            </div>
          )}
        </div>
      </TreeAlignedRow>
    </div>
  );
};

/**
 * Iterator rendering a list of tree item children.
 **/
export const TreeItems: React.FC<{ children: React.ReactNode[]; renderItem: (child: React.ReactNode, index: number, isLast: boolean) => React.ReactNode }> = ({ children, renderItem }) => {
  return <>{children.map((child, index) => renderItem(child, index, index === children.length - 1))}</>;
};

/**
 * Leaf form row combining TreeItem and TreeContent into [Indent][Label][Control].
 * When a label resolves (via id or explicit label prop), delegates to TreeItem for the standard header row.
 * When no label resolves, wraps children in TreeAlignedRow so controls always get proper gutter alignment
 * and tree guide lines regardless of whether the child control uses showLabel.
 **/
const treeRowUsesPropertyHeaderAnchor = (children: React.ReactNode): boolean => {
  const childArray = React.Children.toArray(children);
  return childArray.some((child) => {
    if (!React.isValidElement(child)) {
      return false;
    }
    if (child.type === React.Fragment) {
      return treeRowUsesPropertyHeaderAnchor((child.props as { children?: React.ReactNode }).children);
    }
    const childProps = child.props as { children?: React.ReactNode; showLabel?: boolean };
    return child.type === Label || childProps.showLabel === true;
  });
};

export const TreeRow: React.FC<{
  children: React.ReactNode;
  className?: string;
  id?: string;
  /** When set (including explicit `null`), overrides useLabel(id) for the row title. Use `null` for content-only rows. */
  label?: React.ReactNode;
  onClick?: (event: React.MouseEvent) => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
  actions?: TreeHeaderAction[];
}> = ({ children, className, id, label, onClick, onDoubleClick, actions }) => {
  const localizedLabel = id ? useLabel(id) : undefined;
  const resolvedLabel = label !== undefined ? label : localizedLabel;
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const rowKind = treeRowUsesPropertyHeaderAnchor(children) ? "property" : "content";

  if (resolvedLabel) {
    return (
      <TreeItem className={className} id={id} label={label} onClick={onClick} onDoubleClick={onDoubleClick} actions={actions}>
        {children}
      </TreeItem>
    );
  }

  if (!isTree) {
    return (
      <TreeRowAlignmentContext.Provider value={true}>
        <div data-slot="tree-row" data-tree-row-kind={rowKind} className={cn("min-w-0 w-full min-h-[24px]", className)}>
          {children}
        </div>
      </TreeRowAlignmentContext.Provider>
    );
  }

  return (
    <TreeRowAlignmentContext.Provider value={true}>
      <div data-slot="tree-row" data-tree-row-kind={rowKind} className={cn("relative min-w-0 w-full", className)}>
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} contentClassName="min-w-0" anchorOffsetPx={rowKind === "property" ? detailPanelHeaderLineCenterPx : undefined}>
          {children}
        </TreeAlignedRow>
      </div>
    </TreeRowAlignmentContext.Provider>
  );
};

/**
 * Informational text row spanning the full control column width.
 * When `propertyAligned` is true and inside a tree, renders content in the
 * value-column of the shared property-row grid (same layout as Label).
 **/
export const HelperRow: React.FC<{ children: React.ReactNode; className?: string; propertyAligned?: boolean }> = ({ children, className, propertyAligned = false }) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const helperContent = (
    <div data-slot="helper-row" data-detail-panel-control="fill" className={cn("text-xs text-muted-foreground leading-tight py-[2px]", className)}>
      {children}
    </div>
  );
  if (propertyAligned && isTree) {
    const treePropertyRowOffsetPx = detailPanelIndentPx(level, indentMultiplier);
    return (
      <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} align="start" connectCurrentLevel={level > 0} anchorOffsetPx={detailPanelHeaderLineCenterPx}>
        <div
          data-slot="property-row"
          style={{ marginLeft: `${-treePropertyRowOffsetPx}px`, width: treePropertyRowOffsetPx > 0 ? `calc(100% + ${treePropertyRowOffsetPx}px)` : "100%" }}
          className={cn(detailPanelPropertyRowClassName, "grid-cols-[96px_minmax(0,1fr)]")}
        >
          <div />
          <div data-slot="property-control" className={detailPanelPropertyControlClassName}>
            {helperContent}
          </div>
        </div>
      </TreeAlignedRow>
    );
  }
  return (
    <TreeItem className={className}>
      <TreeContent>{helperContent}</TreeContent>
    </TreeItem>
  );
};

const getTreeItemLabel = (item: TreeDataItem): React.ReactNode => {
  if (!item.description) {
    return item.label;
  }

  return (
    <div className="flex min-w-0 flex-col">
      <span className="truncate">{item.label}</span>
      <span className="truncate text-[10px] text-muted-foreground">{item.description}</span>
    </div>
  );
};

const getTreeDropData = (event: React.DragEvent<HTMLDivElement>): Record<string, string> => {
  return Array.from(event.dataTransfer.types).reduce<Record<string, string>>((result, kind) => {
    try {
      result[kind] = event.dataTransfer.getData(kind);
    } catch {
      result[kind] = "";
    }
    return result;
  }, {});
};

/**
 * Data interface for a node in a file tree.
 **/
export interface FileTreeNode {
  title: string;
  path: string;
  icon?: string;
  isFolder: boolean;
  children?: FileTreeNode[];
}

//#region 🎃TreeHoverPath
// 🌳Branch containers that hold child rows and render IndentationLines.
const treeBranchSlots = new Set(["tree-section-content", "tree-item-content", "tree-property-content", "control-tree-folder-content"]);
// 🔷Row-level elements that own an elbow connector.
const treeRowSlots = new Set(["tree-item-row", "tree-section-row", "tree-property-item", "tree-row", "control-tree-row"]);
const treeHoverPathRowSelector = '[data-slot="tree-item-row"], [data-slot="tree-section-row"], [data-slot="tree-property-item"], [data-slot="tree-row"], [data-slot="control-tree-row"], [data-slot="tree-content"]';
const treeHoverPathBranchSelector = '[data-slot="tree-section-content"], [data-slot="tree-item-content"], [data-slot="tree-property-content"], [data-slot="control-tree-folder-content"]';
const treeHoverPathAttr = "data-tree-hover-path";

const clearTreeHoverPath = (root: HTMLElement) => {
  root.querySelectorAll(`[${treeHoverPathAttr}]`).forEach((el) => el.removeAttribute(treeHoverPathAttr));
};

/**
 * 📦Derive the row element that owns a branch container.
 * Handles all DOM shapes: tree-item-row/control-tree-row siblings,
 * tree-section-row behind collapsible-content, tree-property-item parent.
 */
const rowForBranch = (branch: Element): Element | null => {
  const prev = branch.previousElementSibling;
  if (prev) {
    const prevSlot = prev.getAttribute("data-slot");
    if (prevSlot && treeRowSlots.has(prevSlot)) return prev;
  }
  const parent = branch.parentElement;
  const parentSlot = parent?.getAttribute("data-slot");
  if (parentSlot === "tree-property-item") return parent!;
  if (parentSlot === "collapsible-content") {
    const sectionRow = parent!.previousElementSibling;
    if (sectionRow?.getAttribute("data-slot") === "tree-section-row") return sectionRow;
  }
  return null;
};

/**
 * 🎛️Resolve the conceptual tree row from a pointer target.
 * First tries matching a known row slot via closest(). When no row wrapper
 * exists (pass-through TreeRow, raw controls), falls back to the nearest
 * branch container and returns its owner row.
 */
const resolveHoverRow = (target: HTMLElement, root: HTMLElement): Element | null => {
  const direct = target.closest(treeHoverPathRowSelector);
  if (direct && root.contains(direct)) return direct;
  const branch = target.closest(treeHoverPathBranchSelector);
  if (branch && root.contains(branch)) return rowForBranch(branch);
  return null;
};

const markTerminalBranch = (row: Element) => {
  const slot = row.getAttribute("data-slot");
  if (slot === "tree-item-row" || slot === "control-tree-row") {
    const next = row.nextElementSibling;
    if (next) {
      const nextSlot = next.getAttribute("data-slot");
      if (nextSlot && treeBranchSlots.has(nextSlot)) {
        next.setAttribute(treeHoverPathAttr, "branch");
      }
    }
  } else if (slot === "tree-section-row") {
    const next = row.nextElementSibling;
    if (next?.getAttribute("data-slot") === "collapsible-content") {
      for (const child of Array.from(next.children)) {
        if (child.getAttribute("data-slot") === "tree-section-content") {
          child.setAttribute(treeHoverPathAttr, "branch");
          break;
        }
      }
    }
  } else if (slot === "tree-property-item") {
    for (const child of Array.from(row.children)) {
      if (child.getAttribute("data-slot") === "tree-property-content") {
        child.setAttribute(treeHoverPathAttr, "branch");
        break;
      }
    }
  }
};

const applyTreeHoverPath = (row: Element, root: HTMLElement) => {
  clearTreeHoverPath(root);
  row.setAttribute(treeHoverPathAttr, "row");
  markTerminalBranch(row);
  let el: Element | null = row.parentElement;
  while (el && el !== root) {
    const slot = el.getAttribute("data-slot");
    if (slot && treeBranchSlots.has(slot)) {
      el.setAttribute(treeHoverPathAttr, "branch");
      const ownerRow = rowForBranch(el);
      if (ownerRow) {
        ownerRow.setAttribute(treeHoverPathAttr, "row");
        markTerminalBranch(ownerRow);
      }
    }
    el = el.parentElement;
  }
};
//#endregion 🎃TreeHoverPath

/**
 * Hierarchical tree view component with optional file tree rendering.
 **/
type TreeComponent = ((props: TreeRootProps) => React.ReactElement) & {
  Files: React.FC<TreeFilesProps>;
  Section: React.FC<TreeFilesProps>;
};

export const Tree = (({
  className = "",
  showLines = true,
  sections,
  selectionMode = "single",
  selectedIds: controlledSelectedIds,
  defaultSelectedIds = [],
  onSelectionChange,
  dragAndDropController,
  emptyState,
  indentMultiplier = 1,
  children,
}: TreeRootProps & { children?: React.ReactNode }) => {
  if (hasNonEmptyChildren(children)) {
    throw new Error("Tree only accepts section data through the sections prop.");
  }
  const [sectionItemsById, setSectionItemsById] = reactHostPort.useState<Record<string, TreeDataItem[]>>(() =>
    (sections ?? []).reduce<Record<string, TreeDataItem[]>>((result, section) => {
      if (section.items) {
        result[section.id] = section.items;
      }
      return result;
    }, {}),
  );
  const [itemItemsById, setItemItemsById] = reactHostPort.useState<Record<string, TreeDataItem[]>>({});
  const [loadingById, setLoadingById] = reactHostPort.useState<Record<string, boolean>>({});
  const [uncontrolledSelectedIds, setUncontrolledSelectedIds] = reactHostPort.useState<string[]>(() => normalizeTreeSelectedIds(defaultSelectedIds, selectionMode));
  const [draggedIds, setDraggedIds] = reactHostPort.useState<string[]>([]);
  const resolvedSections = sections ?? [];
  const anchorIdRef = reactHostPort.useRef<string | undefined>(normalizeTreeSelectedIds(defaultSelectedIds, selectionMode)[0]);
  const resolvedSelectedIds = reactHostPort.useMemo(() => normalizeTreeSelectedIds(controlledSelectedIds ?? uncontrolledSelectedIds, selectionMode), [controlledSelectedIds, uncontrolledSelectedIds, selectionMode]);

  reactHostPort.useEffect(() => {
    setSectionItemsById((previousItems) => {
      let hasChanges = false;
      const nextItems = { ...previousItems };
      resolvedSections.forEach((section) => {
        if (section.items && previousItems[section.id] !== section.items) {
          nextItems[section.id] = section.items;
          hasChanges = true;
        }
      });
      return hasChanges ? nextItems : previousItems;
    });
  }, [resolvedSections]);

  const itemMap = reactHostPort.useMemo(() => {
    const map: Record<string, TreeDataItem> = {};
    resolvedSections.forEach((section) => {
      collectTreeItemMap(getTreeSectionItems(section, sectionItemsById), map);
    });
    Object.values(itemItemsById).forEach((items) => {
      collectTreeItemMap(items, map);
    });
    return map;
  }, [itemItemsById, resolvedSections, sectionItemsById]);

  const updateSelection = reactHostPort.useCallback(
    (nextSelectedIds: string[]) => {
      const normalizedIds = normalizeTreeSelectedIds(nextSelectedIds, selectionMode);
      if (controlledSelectedIds === undefined) {
        setUncontrolledSelectedIds(normalizedIds);
      }
      onSelectionChange?.(normalizedIds, normalizedIds.map((id) => itemMap[id]).filter(Boolean));
    },
    [controlledSelectedIds, itemMap, onSelectionChange, selectionMode],
  );

  const loadSectionItems = reactHostPort.useCallback(
    async (section: TreeDataSection) => {
      if (!section.getItems || sectionItemsById[section.id] !== undefined || loadingById[getTreeSectionLoadingId(section.id)]) {
        return;
      }
      setLoadingById((previousItems) => ({ ...previousItems, [getTreeSectionLoadingId(section.id)]: true }));
      try {
        const nextItems = await section.getItems();
        setSectionItemsById((previousItems) => ({ ...previousItems, [section.id]: nextItems }));
      } finally {
        setLoadingById((previousItems) => ({ ...previousItems, [getTreeSectionLoadingId(section.id)]: false }));
      }
    },
    [loadingById, sectionItemsById],
  );

  const loadItemItems = reactHostPort.useCallback(
    async (item: TreeDataItem) => {
      if (!item.getItems || itemItemsById[item.id] !== undefined || loadingById[getTreeItemLoadingId(item.id)]) {
        return;
      }
      setLoadingById((previousItems) => ({ ...previousItems, [getTreeItemLoadingId(item.id)]: true }));
      try {
        const nextItems = await item.getItems();
        setItemItemsById((previousItems) => ({ ...previousItems, [item.id]: nextItems }));
      } finally {
        setLoadingById((previousItems) => ({ ...previousItems, [getTreeItemLoadingId(item.id)]: false }));
      }
    },
    [itemItemsById, loadingById],
  );

  const handleDragOver = reactHostPort.useCallback((event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
  }, []);

  const handleSelectItem = reactHostPort.useCallback(
    (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
      const orderedIds = getTreeItemOrderedIds(resolvedSections, sectionItemsById, itemItemsById);
      const nextSelection = getTreeNextSelectionState({
        selectionMode,
        selectedIds: resolvedSelectedIds,
        orderedIds,
        targetId: item.id,
        anchorId: anchorIdRef.current,
        additiveKey: event.metaKey || event.ctrlKey,
        rangeKey: event.shiftKey,
      });
      anchorIdRef.current = nextSelection.anchorId;
      updateSelection(nextSelection.selectedIds);
      item.onClick?.(event, { path, selectedIds: nextSelection.selectedIds, sectionId: section.id });
    },
    [itemItemsById, resolvedSections, resolvedSelectedIds, sectionItemsById, selectionMode, updateSelection],
  );

  const handleDoubleClickItem = reactHostPort.useCallback(
    (event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
      item.onDoubleClick?.(event, { path, selectedIds: resolvedSelectedIds, sectionId: section.id });
    },
    [resolvedSelectedIds],
  );

  const handleDragStart = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
      const nextDraggedIds = resolvedSelectedIds.includes(item.id) ? resolvedSelectedIds : [item.id];
      const sourceItems = nextDraggedIds.map((id) => itemMap[id]).filter(Boolean);
      setDraggedIds(nextDraggedIds);
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData(treeDefaultDragMimeKind, JSON.stringify(nextDraggedIds));
      const customData = dragAndDropController?.getDragData?.({ items: sourceItems, sourceItem: item, section });
      Object.entries(customData ?? {}).forEach(([kind, value]) => {
        event.dataTransfer.setData(kind, value);
      });
      dragAndDropController?.onDragStart?.({ items: sourceItems, sourceItem: item, section });
    },
    [dragAndDropController, itemMap, resolvedSelectedIds],
  );

  const handleDrop = reactHostPort.useCallback(
    (event: React.DragEvent<HTMLDivElement>, target: TreeDataItem | TreeDataSection, targetKind: "item" | "section", section: TreeDataSection) => {
      event.preventDefault();
      const sourceIds = draggedIds.length > 0 ? draggedIds : JSON.parse(event.dataTransfer.getData(treeDefaultDragMimeKind) || "[]");
      dragAndDropController?.handleDrop?.({
        target,
        targetKind,
        data: getTreeDropData(event),
        sourceItems: sourceIds.map((id: string) => itemMap[id]).filter(Boolean),
        section,
      });
      setDraggedIds([]);
    },
    [dragAndDropController, draggedIds, itemMap],
  );

  const DataItemView: React.FC<{ item: TreeDataItem; section: TreeDataSection; path: string[]; isLastItem: boolean }> = ({ item, section, path, isLastItem }) => {
    const baseChildItems = getTreeItemItems(item, itemItemsById);
    const alternatives = item.alternatives ?? [];
    const branchCount = alternatives.length;
    const [activeBranchIndex, setActiveBranchIndex] = reactHostPort.useState(0);
    const clampedBranchIndex = branchCount > 0 ? Math.min(activeBranchIndex, branchCount - 1) : 0;
    const childItems = branchCount > 0 ? (alternatives[clampedBranchIndex] ?? []) : baseChildItems;
    const treeOpenState = useTreeOpenState(getTreeItemStateId(item.id), getTreeItemDefaultOpen(item));
    const isLoading = loadingById[getTreeItemLoadingId(item.id)] ?? false;
    const hasDynamicChildren = Boolean(item.getItems);
    const hasExpandableChildren = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState) || branchCount > 0;
    const isExpandable = item.collapsibleState === TreeItemCollapsibleState.None ? false : hasExpandableChildren;

    reactHostPort.useEffect(() => {
      if (treeOpenState.open && hasDynamicChildren) {
        void loadItemItems(item);
      }
    }, [hasDynamicChildren, item, treeOpenState.open]);

    return (
      <TreeItem
        id={item.id}
        label={getTreeItemLabel(item)}
        icon={item.icon}
        className={item.className}
        isSelected={item.isSelected ?? resolvedSelectedIds.includes(item.id)}
        isHighlighted={item.isHighlighted}
        isDragHandle={item.isDragHandle}
        defaultOpen={getTreeItemDefaultOpen(item)}
        open={treeOpenState.open}
        onOpenChange={treeOpenState.setOpen}
        expandable={isExpandable}
        loading={isLoading}
        isLastItem={isLastItem}
        actions={item.actions}
        draggable={item.draggable ?? Boolean(dragAndDropController)}
        onClick={(event) => handleSelectItem(event, item, section, path)}
        onDoubleClick={(event) => handleDoubleClickItem(event, item, section, path)}
        onDragStart={(event) => handleDragStart(event, item, section)}
        onDragOver={handleDragOver}
        onDrop={(event) => handleDrop(event, item, "item", section)}
        onPointerEnter={item.onPointerEnter}
        onPointerLeave={item.onPointerLeave}
        branchCount={branchCount}
        activeBranchIndex={clampedBranchIndex}
        onBranchChange={setActiveBranchIndex}
      >
        {childItems.map((childItem, index) => (
          <DataItemView key={childItem.id} item={childItem} section={section} path={[...path, childItem.id]} isLastItem={index === childItems.length - 1} />
        ))}
        {!isLoading && childItems.length === 0 && item.emptyState && (
          <TreeItem>
            <TreeContent>{item.emptyState}</TreeContent>
          </TreeItem>
        )}
      </TreeItem>
    );
  };

  const DataSectionView: React.FC<{ section: TreeDataSection }> = ({ section }) => {
    const treeOpenState = useTreeOpenState(getTreeSectionStateId(section.id), section.defaultOpen ?? true);
    const items = getTreeSectionItems(section, sectionItemsById);
    const isLoading = loadingById[getTreeSectionLoadingId(section.id)] ?? false;
    const hasDynamicChildren = Boolean(section.getItems);
    const isExpandable = items.length > 0 || hasDynamicChildren || Boolean(section.emptyState) || hasNonEmptyChildren(section.content);

    reactHostPort.useEffect(() => {
      if (treeOpenState.open && hasDynamicChildren) {
        void loadSectionItems(section);
      }
    }, [hasDynamicChildren, section, treeOpenState.open]);

    return (
      <TreeSection
        id={section.id}
        label={section.label}
        icon={section.icon}
        className={section.className}
        defaultOpen={section.defaultOpen}
        open={treeOpenState.open}
        onOpenChange={treeOpenState.setOpen}
        expandable={isExpandable}
        loading={isLoading}
        actions={section.actions}
        onPointerEnter={section.onPointerEnter}
        onPointerLeave={section.onPointerLeave}
        onDoubleClick={section.onDoubleClick}
        onDragOver={handleDragOver}
        onDrop={(event) => handleDrop(event, section, "section", section)}
      >
        {section.content}
        {items.map((item, index) => (
          <DataItemView key={item.id} item={item} section={section} path={[section.id, item.id]} isLastItem={index === items.length - 1} />
        ))}
        {!isLoading && items.length === 0 && section.emptyState && <HelperRow>{section.emptyState}</HelperRow>}
      </TreeSection>
    );
  };

  const treeRootRef = reactHostPort.useRef<HTMLDivElement>(null);

  const handleTreePointerOver = reactHostPort.useCallback((e: React.PointerEvent<HTMLDivElement>) => {
    const root = treeRootRef.current;
    if (!root) return;
    const row = resolveHoverRow(e.target as HTMLElement, root);
    if (row) applyTreeHoverPath(row, root);
    else clearTreeHoverPath(root);
  }, []);

  const handleTreePointerLeave = reactHostPort.useCallback(() => {
    const root = treeRootRef.current;
    if (root) clearTreeHoverPath(root);
  }, []);

  return (
    <TreeStateProvider>
      <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines, isTree: true, indentMultiplier }}>
        <div ref={treeRootRef} className={`w-full min-w-0 overflow-hidden ${className}`} onPointerOver={handleTreePointerOver} onPointerLeave={handleTreePointerLeave}>
          {resolvedSections.map((section, index) => (
            <div key={section.id} data-slot="tree-section-wrapper" style={{ marginTop: index === 0 ? "0px" : `${treeSectionBoundaryGapPx}px` }}>
              <DataSectionView section={section} />
            </div>
          ))}
          {resolvedSections.length === 0 && emptyState}
        </div>
      </TreeContext.Provider>
    </TreeStateProvider>
  );
}) as TreeComponent;

// #region 🎇Basic Chat Panel
// Shared side-panel chat UI with local-only message storage.
// Consumers MUST provide a stable id and title per app tab.

interface BasicChatPanelProps extends ElementProps {
  title: string;
}

type BasicChatMessageRole = "assistant" | "user";

interface BasicChatMessage {
  id: string;
  role: BasicChatMessageRole;
  body: string;
}

const createBasicChatMessages = (id: string, title: string): BasicChatMessage[] => [
  {
    id: `${id}.assistant.0`,
    role: "assistant",
    body: `Chat is ready for ${title}.`,
  },
  {
    id: `${id}.assistant.1`,
    role: "assistant",
    body: "Messages stay local in this panel until a connected assistant is added.",
  },
];

export const BasicChatPanel: React.FC<BasicChatPanelProps> = ({ id, title }) => {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const [messages, setMessages] = reactHostPort.useState<BasicChatMessage[]>(() => createBasicChatMessages(id, title));
  const [draft, setDraft] = reactHostPort.useState("");
  const nextMessageIndexRef = reactHostPort.useRef(2);
  const appendMessage = (role: BasicChatMessageRole, body: string) => {
    const nextMessageId = `${id}.${role}.${nextMessageIndexRef.current}`;
    nextMessageIndexRef.current += 1;
    setMessages((previousMessages) => [
      ...previousMessages,
      {
        id: nextMessageId,
        role,
        body,
      },
    ]);
  };
  const clearMessages = () => {
    nextMessageIndexRef.current = 2;
    setMessages(createBasicChatMessages(id, title));
    setDraft("");
  };
  const sendDraft = () => {
    const trimmedDraft = draft.trim();
    if (!trimmedDraft) {
      return;
    }
    const responsePreview = trimmedDraft.length > 72 ? `${trimmedDraft.slice(0, 69)}...` : trimmedDraft;
    setDraft("");
    appendMessage("user", trimmedDraft);
    appendMessage("assistant", `Saved locally: "${responsePreview}"`);
  };

  reactHostPort.useEffect(() => {
    nextMessageIndexRef.current = 2;
    setMessages(createBasicChatMessages(id, title));
    setDraft("");
  }, [id, title]);

  return (
    <div data-testid="basic-chat-panel" className="flex h-full min-h-0 flex-col gap-single">
      <HelperRow>{`Local chat for ${title}. Use Enter to send and Shift+Enter for a new line.`}</HelperRow>
      <div data-testid="basic-chat-feed" className={cn("min-h-0 flex-1 overflow-y-auto rounded-[3px] border", borderClass)}>
        <Tree
          className="min-w-0 p-single"
          sections={[
            {
              id: `${id}.messages`,
              label: null,
              content: messages.map((message) => (
                <TreeRow key={message.id}>
                  <div data-testid="basic-chat-message" data-chat-role={message.role} className="flex min-w-0 flex-col gap-[2px]">
                    <span className="text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">{message.role}</span>
                    <p className="text-xs text-foreground whitespace-pre-wrap break-words">{message.body}</p>
                  </div>
                </TreeRow>
              )),
            },
          ]}
        />
      </div>
      <div className="flex shrink-0 flex-col gap-single">
        <Textarea
          id={`${id}.draft`}
          data-testid="basic-chat-draft"
          value={draft}
          onChange={(event) => setDraft(event.target.value)}
          onKeyDown={(event) => {
            if (event.key !== "Enter" || event.shiftKey) {
              return;
            }
            event.preventDefault();
            sendDraft();
          }}
          rows={3}
          placeholder={`Write a message for ${title.toLowerCase()}...`}
        />
        <div className="flex items-center justify-end gap-single">
          <Button type="button" id={`${id}.clear`} data-testid="basic-chat-clear" text="Clear" onClick={clearMessages} />
          <Button type="button" id={`${id}.send`} data-testid="basic-chat-send" text="Send" onClick={sendDraft} disabled={!draft.trim()} />
        </div>
      </div>
    </div>
  );
};

// #endregion 🎇Basic Chat Panel

interface FileTreeItemProps {
  node: FileTreeNode;
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
}

/**
 * FileTreeItem holds the data fields for a FileTreeItem record.
 **/
const FileTreeItem: React.FC<FileTreeItemProps> = ({ node, currentPath, onNavigate, as = "a" }) => {
  const { level, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const [isHovered, setIsHovered] = reactHostPort.useState(false);
  const itemId = `file-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, true);

  const isActive = currentPath === node.path;
  const hasChildren = node.children && node.children.length > 0;
  const Icon = node.isFolder ? FolderIcon : DocumentIcon;

  const baseClasses = "flex items-center gap-single text-sm rounded-small cursor-selectable select-none";
  const stateClasses = isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:text-foreground";
  const itemClasses = `${baseClasses} ${stateClasses}`;
  const handleClick = (e: React.MouseEvent) => {
    if (hasChildren) {
      e.preventDefault();
      setOpen(!open);
    }
    if (onNavigate) {
      onNavigate(node.path);
    }
  };

  const content = (
    <>
      {node.icon ? <span className="text-sm shrink-0">{node.icon}</span> : <Icon className="size-tiny shrink-0" />}
      <span className="text-sm">{node.title}</span>
    </>
  );

  const sharedProps = {
    className: itemClasses,
    style: { paddingLeft: `${detailPanelIndentPx(level, indentMultiplier) + 12}px` },
    onClick: handleClick,
    onMouseEnter: () => setIsHovered(true),
    onMouseLeave: () => setIsHovered(false),
  };

  const itemElement =
    as === "a" ? (
      <a href={`/${node.path}`} {...sharedProps}>
        {content}
      </a>
    ) : (
      <div {...sharedProps}>{content}</div>
    );

  if (hasChildren && node.isFolder) {
    return (
      <>
        {itemElement}
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [], showLines: false, isTree, indentMultiplier }}>
            {node.children!.map((child, idx) => (
              <FileTreeItem key={idx} node={child} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </TreeContext.Provider>
        )}
      </>
    );
  }

  return itemElement;
};

/**
 * TreeFilesProps holds the data fields for a TreeFilesProps record.
 **/
interface TreeFilesProps {
  title?: string;
  nodes: FileTreeNode[];
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
  className?: string;
}

const TreeFiles: React.FC<TreeFilesProps> = ({ title, nodes, currentPath, onNavigate, as = "a", className = "" }) => {
  return (
    <TreeStateProvider>
      <div className={`not-prose my-medium p-medium rounded-lg border border-element bg-card ${className}`}>
        {title && <h3 className="text-lg font-semibold mb-4">{title}</h3>}
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false, isTree: true, indentMultiplier: 1 }}>
          <div className="flex flex-col gap-single">
            {nodes.map((node, idx) => (
              <FileTreeItem key={idx} node={node} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </div>
        </TreeContext.Provider>
      </div>
    </TreeStateProvider>
  );
};

Tree.Files = TreeFiles;
Tree.Section = Tree.Files;

/** Alias for Tree.Files rendering a file tree from FileTreeNode data.
 **/
export const FileTree = TreeFiles;

// #region 🔬ControlTree
// Leva-like nested folder+controls tree UI using existing design system components.
// Consumers MUST provide ControlDef[] and optional ControlTreeFolderSettings.

/**
 * Leaf control definition for the ControlTree.
 **/
export interface ControlDef {
  path: string;
  key?: string;
  order?: number;
  controlKind: string;
  value: any;
  onChange: (next: any) => void;
  meta?: Record<string, any>;
}

/**
 * Folder settings for the ControlTree.
 **/
export interface ControlTreeFolderSettings {
  path: string;
  order?: number;
  collapsed?: boolean;
  color?: string;
}

/**
 * Styling classname overrides for ControlTree visual slots.
 **/
export interface ControlTreeClassNames {
  panel?: string;
  folderRow?: string;
  folderTitle?: string;
  folderChevron?: string;
  folderChildren?: string;
  controlRow?: string;
  controlLabel?: string;
  controlBody?: string;
}

interface ControlTreeNode {
  kind: "folder" | "control";
  key: string;
  path: string;
  order: number;
  control?: ControlDef;
  children?: Record<string, ControlTreeNode>;
}

/**
 * Pure function converting flat ControlDef[] paths into a nested tree. Filtering matches leaf keys case-insensitively.
 **/
export function buildControlTree(controls: ControlDef[], filterText: string, folderSettings?: Record<string, ControlTreeFolderSettings>): Record<string, ControlTreeNode> {
  const root: Record<string, ControlTreeNode> = {};
  const lowerFilter = filterText.toLowerCase();
  for (const control of controls) {
    const leafKey = control.key ?? control.path.split("/").pop() ?? control.path;
    if (lowerFilter && !leafKey.toLowerCase().includes(lowerFilter)) continue;
    const segments = control.path.split("/");
    let current = root;
    let pathAccum = "";
    for (let i = 0; i < segments.length - 1; i++) {
      const seg = segments[i];
      pathAccum = pathAccum ? `${pathAccum}/${seg}` : seg;
      if (!current[seg]) {
        current[seg] = {
          kind: "folder",
          key: seg,
          path: pathAccum,
          order: folderSettings?.[pathAccum]?.order ?? 0,
          children: {},
        };
      }
      current = current[seg].children!;
    }
    const lastSeg = segments[segments.length - 1];
    current[lastSeg] = {
      kind: "control",
      key: leafKey,
      path: control.path,
      order: control.order ?? 0,
      control,
    };
  }
  return root;
}

function sortControlTreeNodes(nodes: Record<string, ControlTreeNode>): ControlTreeNode[] {
  return Object.values(nodes).sort((a, b) => {
    if (a.order !== b.order) return a.order - b.order;
    return a.key.localeCompare(b.key);
  });
}

/**
 * Default control renderer mapping controlKind to built-in components.
 **/
export const defaultControlRenderer = (def: ControlDef): React.ReactNode => {
  const controlId = def.path.replace(/\//g, ".");
  switch (def.controlKind) {
    case "number":
      return <Stepper id={controlId} value={def.value} onChange={def.onChange} min={def.meta?.min} max={def.meta?.max} step={def.meta?.step ?? 1} />;
    case "slider":
      return <Slider id={controlId} value={[def.value]} onValueChange={(v) => def.onChange(v[0])} min={def.meta?.min ?? 0} max={def.meta?.max ?? 100} />;
    case "boolean":
      return <Toggle id={controlId} pressed={def.value} onPressedChange={def.onChange} icon={def.value ? <CheckIcon className="size-small" /> : <CloseIcon className="size-small" />} />;
    case "string":
      return <Input id={controlId} lazy value={def.value} onLazyChange={def.onChange} />;
    case "color":
      return <Input id={controlId} type="color" value={def.value} onChange={(e) => def.onChange(e.target.value)} />;
    case "select":
      return (
        <Select id={controlId} value={def.value} onValueChange={def.onChange}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {(def.meta?.options ?? []).map((opt: string | { value: string; label: string }) => {
              const v = typeof opt === "string" ? opt : opt.value;
              const l = typeof opt === "string" ? opt : opt.label;
              return (
                <SelectItem key={v} value={v}>
                  {l}
                </SelectItem>
              );
            })}
          </SelectContent>
        </Select>
      );
    case "text":
      return <Textarea id={controlId} lazy value={def.value} onLazyChange={def.onChange} />;
    default:
      return <Input id={controlId} lazy value={String(def.value)} onLazyChange={def.onChange} />;
  }
};

interface ControlTreeFolderProps {
  node: ControlTreeNode;
  folderSettings?: Record<string, ControlTreeFolderSettings>;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
  renderControl: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
}
const controlTreeValueColumnWidthPx = 160;
interface ControlTreeRowProps {
  className?: string;
  left: React.ReactNode;
  right?: React.ReactNode;
}
const ControlTreeRow: React.FC<ControlTreeRowProps> = ({ className, left, right }) => (
  <div data-slot="control-tree-row" className={cn("grid min-w-0 w-full items-center gap-x-[8px] min-h-[20px]", className)} style={{ gridTemplateColumns: `minmax(0, 1fr) ${controlTreeValueColumnWidthPx}px` }}>
    <div data-slot="control-tree-row-left" className="relative min-w-0">
      {left}
    </div>
    <div data-slot="control-tree-row-right" className="min-w-0">
      {right}
    </div>
  </div>
);
interface ControlTreeFolderRowProps {
  node: ControlTreeNode;
  classNames?: ControlTreeClassNames;
  children?: React.ReactNode;
  defaultOpen: boolean;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
}
const ControlTreeFolderRow: React.FC<ControlTreeFolderRowProps> = ({ node, classNames, children, defaultOpen, onToggleFolder }) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier } = reactHostPort.useContext(TreeContext);
  const itemId = `control-tree-folder-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const hasChildren = hasNonEmptyChildren(children);
  return (
    <>
      <ControlTreeRow
        className={cn("hover:bg-hover-panel select-none overflow-hidden group", classNames?.folderRow)}
        left={
          <TreeAlignedRow
            level={level}
            isLastAtLevel={isLastAtLevel}
            showLines={showLines}
            connectCurrentLevel={level > 0}
            extendCurrentLevelToBottom={open && hasChildren}
            slotOffsetPx={2}
            slot={
              hasChildren ? (
                <button
                  className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                  onClick={(event) => {
                    event.preventDefault();
                    event.stopPropagation();
                    const nextOpen = !open;
                    setOpen(nextOpen);
                    onToggleFolder?.(node.path, !nextOpen);
                  }}
                >
                  {open ? <ChevronDownIcon className={cn("size-[14px] flex-shrink-0", classNames?.folderChevron)} /> : <ChevronRightIcon className={cn("size-[14px] flex-shrink-0", classNames?.folderChevron)} />}
                </button>
              ) : undefined
            }
            contentClassName="flex min-w-0 items-center gap-[6px]"
          >
            <span data-slot="control-tree-folder-label" className={cn("text-xs font-semibold uppercase tracking-wide truncate text-muted-foreground", classNames?.folderTitle)} style={treeItemLabelStyle}>
              {node.key}
            </span>
          </TreeAlignedRow>
        }
      />
      {open && hasChildren && (
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree, indentMultiplier }}>
          <TreeBranchContent slot="control-tree-folder-content" className={classNames?.folderChildren}>
            {children}
          </TreeBranchContent>
        </TreeContext.Provider>
      )}
    </>
  );
};
interface ControlTreeLeafRowProps {
  node: ControlTreeNode;
  renderControl: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
}
const ControlTreeLeafRow: React.FC<ControlTreeLeafRowProps> = ({ node, renderControl, classNames }) => {
  const { level, isLastAtLevel, showLines } = reactHostPort.useContext(TreeContext);
  return (
    <ControlTreeRow
      className={cn("hover:bg-hover-panel select-none overflow-hidden group", classNames?.controlRow)}
      left={
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slotOffsetPx={2} contentClassName="flex min-w-0 items-center gap-[6px]">
          <span data-slot="control-tree-control-label" className={cn("text-xs font-normal truncate text-foreground", classNames?.controlLabel)} style={treeItemLabelStyle}>
            {node.key}
          </span>
        </TreeAlignedRow>
      }
      right={
        <div data-slot="control-tree-control-body" className={cn("min-w-0", classNames?.controlBody)}>
          {renderControl(node.control!)}
        </div>
      }
    />
  );
};

const ControlTreeFolder: React.FC<ControlTreeFolderProps> = ({ node, folderSettings, onToggleFolder, renderControl, classNames }) => {
  const settings = folderSettings?.[node.path];
  const defaultOpen = !(settings?.collapsed ?? false);
  const sorted = sortControlTreeNodes(node.children ?? {});
  return (
    <ControlTreeFolderRow node={node} classNames={classNames} defaultOpen={defaultOpen} onToggleFolder={onToggleFolder}>
      {sorted.map((child) =>
        child.kind === "folder" ? (
          <ControlTreeFolder key={child.path} node={child} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
        ) : (
          <ControlTreeLeafRow key={child.path} node={child} renderControl={renderControl} classNames={classNames} />
        ),
      )}
    </ControlTreeFolderRow>
  );
};

/**
 * Props interface for the ControlTree component.
 **/
export interface ControlTreeProps {
  controls: ControlDef[];
  filterText?: string;
  folderSettings?: Record<string, ControlTreeFolderSettings>;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
  renderControl?: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
  className?: string;
}

/**
 * Leva-like nested folder+controls tree panel using existing design system components.
 **/
export const ControlTree: React.FC<ControlTreeProps> = ({ controls, filterText = "", folderSettings, onToggleFolder, renderControl = defaultControlRenderer, classNames, className }) => {
  const tree = reactHostPort.useMemo(() => buildControlTree(controls, filterText, folderSettings), [controls, filterText, folderSettings]);
  const sorted = reactHostPort.useMemo(() => sortControlTreeNodes(tree), [tree]);
  return (
    <div data-slot="control-tree" className={cn("w-full min-w-0", classNames?.panel, className)}>
      <Tree
        sections={[
          {
            id: "control-tree-root",
            label: null,
            content: sorted.map((node) =>
              node.kind === "folder" ? (
                <ControlTreeFolder key={node.path} node={node} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
              ) : (
                <ControlTreeLeafRow key={node.path} node={node} renderControl={renderControl} classNames={classNames} />
              ),
            ),
          },
        ]}
      />
    </div>
  );
};

// #endregion 🔬ControlTree

// #endregion 📜Tree

// #endregion 🗼Aggregation Components

// #region 🔷Navigation Components

// #region 💡Breadcrumb
// Breadcrumb trail for hierarchical page navigation.
// Consumers MUST provide BreadcrumbItemData entries.

/**
 * Data interface for a single breadcrumb entry.
 **/
export interface BreadcrumbItemData {
  id?: string;
  content: React.ReactNode;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
}

/**
 * BreadcrumbProps holds the data fields for a BreadcrumbProps record.
 **/
interface BreadcrumbProps extends Omit<React.ComponentProps<"nav">, "children"> {
  items: BreadcrumbItemData[];
}

/** Breadcrumb holds the data fields for a Breadcrumb record.
 **/
/**
 **/
function Breadcrumb({ className, items, ...props }: BreadcrumbProps) {
  const [openIndex, setOpenIndex] = reactHostPort.useState<number | null>(null);
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);

  return (
    <nav aria-label="breadcrumb" data-slot="breadcrumb" className={cn("flex h-medium items-stretch border", borderClass, className)} {...props}>
      <ol data-slot="breadcrumb-list" className="flex flex-nowrap items-stretch text-xs break-words overflow-hidden h-full min-w-0">
        {items.map((item, index) => {
          const hasOptions = !!(item.options && item.options.length > 0);
          const isOpen = openIndex === index;

          return (
            <React.Fragment key={index}>
              <BreadcrumbItem {...item} />
              <BreadcrumbSeparatorItem hasOptions={hasOptions} isOpen={isOpen} onOpenChange={(open) => setOpenIndex(open ? index : null)} id={item.id} options={item.options} onNavigate={item.onNavigate} />
            </React.Fragment>
          );
        })}
      </ol>
    </nav>
  );
}

/**
 * BreadcrumbItemProps holds the data fields for a BreadcrumbItemProps record.
 **/
interface BreadcrumbItemProps extends Omit<React.ComponentProps<"li">, "content"> {
  id?: string;
  content?: React.ReactNode;
  onNavigate?: (href: string) => void;
  options?: { label: React.ReactNode; href: string; id?: string }[];
}

/**
 * BreadcrumbItem holds the data fields for a BreadcrumbItem record.
 **/
function BreadcrumbItem({ className, id, content, children, onNavigate, options, ...props }: BreadcrumbItemProps) {
  const itemContent = content ?? children;
  const interactiveContent = reactHostPort.useMemo(() => {
    if (itemContent == null || typeof itemContent === "boolean") return null;
    if (React.isValidElement(itemContent)) {
      if (itemContent.type === React.Fragment) {
        return (
          <span data-slot="breadcrumb-link" className="cursor-selectable flex h-full min-w-0 items-center">
            {itemContent}
          </span>
        );
      }
      const elementProps = itemContent.props as { className?: string; ["data-slot"]?: string };
      return React.cloneElement(itemContent as React.ReactElement<any>, {
        className: cn("cursor-selectable h-full min-w-0", elementProps?.className),
        "data-slot": elementProps?.["data-slot"] ?? "breadcrumb-link",
      });
    }
    return (
      <span data-slot="breadcrumb-link" className="cursor-selectable flex h-full min-w-0 items-center">
        {itemContent}
      </span>
    );
  }, [itemContent]);

  const itemElement = (
    <li data-slot="breadcrumb-item" id={id} className={cn("flex h-full min-w-0 items-stretch cursor-selectable overflow-hidden", className)} {...props}>
      {interactiveContent}
    </li>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{itemElement}</TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return itemElement;
}

/**
 * BreadcrumbSeparatorItemProps holds the data fields for a BreadcrumbSeparatorItemProps record.
 **/
interface BreadcrumbSeparatorItemProps {
  hasOptions: boolean;
  isOpen: boolean;
  onOpenChange?: (open: boolean) => void;
  id?: string;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
}

/** BreadcrumbSeparatorItem holds the data fields for a BreadcrumbSeparatorItem record.
 **/
/**
 **/
function BreadcrumbSeparatorItem({ hasOptions, isOpen, onOpenChange, id, options, onNavigate }: BreadcrumbSeparatorItemProps) {
  const icon = isOpen ? <ChevronDownIcon className="cursor-foldable" /> : <ChevronRightIcon className="cursor-foldable" />;

  const handleSelect = (href: string) => {
    onOpenChange?.(false);
    onNavigate?.(href);
  };

  const separatorControlClassName =
    "text-foreground inline-flex h-full aspect-square items-center justify-center shrink-0 p-single transition-colors cursor-selectable overflow-hidden outline-none hover:bg-hover-base focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive rounded-none [&_svg]:pointer-events-none [&_svg]:size-tiny [&_svg]:shrink-0";

  if (!hasOptions || !options?.length) {
    return (
      <li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" className="flex h-full items-stretch">
        <div data-slot="breadcrumb-separator-control" className={cn(separatorControlClassName, "pointer-events-none")}>
          {icon}
        </div>
      </li>
    );
  }
  return (
    <li data-slot="breadcrumb-separator" role="presentation" className="flex h-full items-stretch">
      <DropdownMenuPrimitive.Root open={isOpen} onOpenChange={onOpenChange}>
        <DropdownMenuPrimitive.Trigger asChild>
          <button type="button" id={id && !isOpen ? id : undefined} data-slot="breadcrumb-separator-control" className={separatorControlClassName}>
            {icon}
          </button>
        </DropdownMenuPrimitive.Trigger>
        <DropdownMenuPrimitive.Portal>
          <DropdownMenuPrimitive.Content align="center" sideOffset={8} className="bg-transparent backdrop-blur-sm w-auto overflow-hidden border p-single z-temporary">
            {options.map((item, index) => {
              const menuItem = (
                <DropdownMenuPrimitive.Item
                  key={index}
                  className="text-foreground hover:bg-hover-temporary focus:bg-hover-temporary relative flex items-center p-single text-sm outline-none whitespace-nowrap"
                  onClick={() => handleSelect(item.href)}
                  role="button"
                >
                  {item.label}
                </DropdownMenuPrimitive.Item>
              );

              const wrappedItem = item.id ? (
                <Tooltip key={index}>
                  <TooltipTrigger asChild>{menuItem}</TooltipTrigger>
                  <TooltipContent side="right">
                    <DescriptionTooltipContent id={item.id} />
                  </TooltipContent>
                </Tooltip>
              ) : (
                menuItem
              );

              return (
                <React.Fragment key={index}>
                  {wrappedItem}
                  {index < options.length - 1 && <DropdownMenuPrimitive.Separator className="h-px bg-border my-single" />}
                </React.Fragment>
              );
            })}
          </DropdownMenuPrimitive.Content>
        </DropdownMenuPrimitive.Portal>
      </DropdownMenuPrimitive.Root>
    </li>
  );
}

export { Breadcrumb, BreadcrumbItem };

// #endregion 💡Breadcrumb

// #region 🪩PageNavigation

/**
 * Configuration interface for a previous/next page link.
 **/
export interface PageNavigationLink {
  path: string;
  title: string;
  section?: string;
}
/**
 **/
export interface PageNavigationProps {
  prev?: PageNavigationLink;
  next?: PageNavigationLink;
}

/**
 * PageNavigation holds the data fields for a PageNavigation record.
 **/
const PageNavigation: React.FC<PageNavigationProps> = ({ prev, next }) => {
  const navigate = useNavigate();
  const { t } = useTranslation();

  if (!prev && !next) return null;

  return (
    <div className="flex items-center justify-between border-t border-element pt-4 mt-8">
      {prev ? (
        <Button id="semio.sketchpad.docs.navigation.previous" onClick={() => navigate(`/${prev.path}`)} className="flex items-center gap-single">
          <div className="text-left">
            <div className="text-xs text-muted-foreground">{t("pageNavigation.previous")}</div>
            <div className="font-medium">{prev.title}</div>
          </div>
        </Button>
      ) : (
        <div />
      )}
      {next ? (
        <Button id="semio.sketchpad.docs.navigation.next" onClick={() => navigate(`/${next.path}`)} className="flex items-center gap-single">
          <div className="text-right">
            <div className="text-xs text-muted-foreground">{t("pageNavigation.next")}</div>
            <div className="font-medium">{next.title}</div>
          </div>
        </Button>
      ) : (
        <div />
      )}
    </div>
  );
};

export { PageNavigation };

// #endregion 🪩PageNavigation

// #endregion 🔷Navigation Components

// #region 📷Panel Components

// #region 🦉Panel
// Resizable dockable panel with sections and collapse support.
// Consumers MUST set resizeSide for the handle.

/**
 * Union type for panel resize handle positions.
 **/
export type ResizeSide = "left" | "right" | "top" | "bottom";

/**
 * Configuration interface for a collapsible section within a panel.
 **/
export interface PanelSection {
  id: string;
  content: React.ReactNode | (() => React.ReactNode);
  specificity?: number;
  defaultOpen?: boolean;
  order?: number;
  actions?: Array<{
    id: string;
    icon: React.ReactNode;
    onClick: () => void;
  }>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: () => void;
}

/**
 * Props interface for the Panel component.
 **/
export interface PanelProps {
  visible?: boolean;
  onSizeChange?: (size: number) => void;
  size?: number;
  resizeSide?: ResizeSide;
  zIndex?: 10 | 20 | 30 | 40;
  showBackground?: boolean;
  minSize?: number;
  maxSize?: number;
  sections?: PanelSection[];
  emptyMessage?: string;
  additionalContent?: React.ReactNode;
  footer?: React.ReactNode;
  className?: string;
  opacity?: number;
  panelKey?: string;
}

/**
 * Panel holds the data fields for a Panel record.
 **/
const Panel: React.FC<PanelProps> = ({
  visible = true,
  onSizeChange,
  size = 250,
  resizeSide = "right",
  zIndex = 20,
  showBackground = true,
  minSize = 150,
  maxSize = 500,
  sections = [],
  emptyMessage,
  additionalContent,
  footer,
  className = "",
  opacity = 1,
  panelKey,
}) => {
  const mode = useTooltipMode();
  const [isResizeHovered, setIsResizeHovered] = reactHostPort.useState(false);
  const [isResizing, setIsResizing] = reactHostPort.useState(false);
  if (!visible) return null;
  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    const startPos = resizeSide === "top" || resizeSide === "bottom" ? e.clientY : e.clientX;
    const startSize = size;
    const bindings = createDOMEventBinding();
    const handleMouseMove = (e: MouseEvent) => {
      const currentPos = resizeSide === "top" || resizeSide === "bottom" ? e.clientY : e.clientX;
      const delta = currentPos - startPos;
      let newSize: number;
      if (resizeSide === "right" || resizeSide === "bottom") {
        newSize = startSize + delta;
      } else {
        newSize = startSize - delta;
      }
      if (newSize >= minSize && newSize <= maxSize) {
        onSizeChange?.(newSize);
      }
    };
    const handleMouseUp = () => {
      setIsResizing(false);
      bindings.dispose();
    };
    bindings.listen(document, "mousemove", handleMouseMove);
    bindings.listen(document, "mouseup", handleMouseUp);
  };
  const sortedSections = [...sections].sort((a, b) => (a.order || 0) - (b.order || 0));
  const borderClass =
    resizeSide === "left"
      ? isResizing || isResizeHovered
        ? "border-l-accent"
        : "border-l"
      : resizeSide === "right"
        ? isResizing || isResizeHovered
          ? "border-r-accent"
          : "border-r"
        : resizeSide === "top"
          ? isResizing || isResizeHovered
            ? "border-t-accent"
            : "border-t"
          : isResizing || isResizeHovered
            ? "border-b-accent"
            : "border-b";
  const containerClass = `absolute text-foreground border min-w-0 overflow-hidden ${borderClass} ${className}`;
  const hasContent = sortedSections.length > 0 || additionalContent;
  const isHorizontal = resizeSide === "left" || resizeSide === "right";
  const positionStyle = isHorizontal
    ? resizeSide === "right"
      ? { left: "var(--spacing-double)", top: "var(--spacing-double)", bottom: "var(--spacing-double)", width: `${size}px`, zIndex }
      : { right: "var(--spacing-double)", top: "var(--spacing-double)", bottom: "var(--spacing-double)", width: `${size}px`, zIndex }
    : resizeSide === "top"
      ? { top: "var(--spacing-double)", left: "var(--spacing-double)", right: "var(--spacing-double)", height: `${size}px`, zIndex }
      : { bottom: "var(--spacing-double)", left: "var(--spacing-double)", right: "var(--spacing-double)", height: `${size}px`, zIndex };
  const resizeHandleClass = isHorizontal ? `absolute top-0 bottom-0 ${resizeSide === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize` : `absolute left-0 right-0 ${resizeSide === "top" ? "top-0" : "bottom-0"} h-single cursor-ns-resize`;
  const treeSections = reactHostPort.useMemo<TreeDataSection[]>(() => {
    const nextSections: TreeDataSection[] = [];
    if (additionalContent) {
      nextSections.push({ id: `${panelKey}-additional`, label: null, content: additionalContent });
    }
    sortedSections.forEach((section, index) => {
      nextSections.push({
        id: section.id,
        defaultOpen: section.defaultOpen ?? index === 0,
        actions: section.actions,
        onPointerEnter: section.onPointerEnter,
        onPointerLeave: section.onPointerLeave,
        onDoubleClick: section.onDoubleClick,
        content: typeof section.content === "function" ? section.content() : section.content,
      });
    });
    if (!hasContent && emptyMessage) {
      nextSections.push({
        id: `${panelKey}-empty`,
        label: null,
        content: <div className="p-small text-center text-muted-foreground">{emptyMessage}</div>,
      });
    }
    return nextSections;
  }, [additionalContent, emptyMessage, hasContent, panelKey, sortedSections]);
  return (
    <LevelProvider level="panel">
      <div data-panel={panelKey} className={cn(containerClass, showBackground ? "bg-panel" : undefined)} style={{ ...positionStyle, opacity, transition: "opacity 150ms" }}>
        <Scrollable className="h-full">
          <div className={`${className || "p-single"} overflow-hidden min-w-0`}>
            <TreeStateProvider>
              <Tree className="min-w-0 overflow-hidden" sections={treeSections} />
            </TreeStateProvider>
          </div>
          {footer}
        </Scrollable>
        {onSizeChange && <div className={resizeHandleClass} onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />}
      </div>
    </LevelProvider>
  );
};

export { Panel };

// #endregion 🦉Panel

// #region 🎙️PanelGroup
// Flex container grouping multiple panels together.
// Consumers MUST provide panel children.

/**
 * Props interface for the PanelGroup component.
 **/
export interface PanelGroupProps {
  className?: string;
  position?: "left" | "right" | "middle" | "bottom";
  children?: React.ReactNode;
}

/**
 * PanelGroup holds the data fields for a PanelGroup record.
 **/
const PanelGroup: React.FC<PanelGroupProps> = ({ children, className = "", position = "middle" }) => {
  const baseClass = "flex";
  const positionClass = position === "left" || position === "right" || position === "middle" ? "flex-col" : "flex-row";
  return <div className={`${baseClass} ${positionClass} ${className}`}>{children}</div>;
};

export { PanelGroup };

// #endregion 🎙️PanelGroup

// #region 💊LeftPanel
// Left-docked panel variant with right resize handle.

/**
 * Props type for LeftPanel omitting resizeSide.
 *
 **/
export type LeftPanelProps = Omit<PanelProps, "resizeSide">;

/** LeftPanel holds the data fields for a LeftPanel record.
 **/
/**
 **/
const LeftPanel: React.FC<LeftPanelProps> = (props) => <Panel {...props} resizeSide="right" />;

export { LeftPanel };

// #endregion 💊LeftPanel

// 🔷#region 🎽RightPanel
export type RightPanelProps = Omit<PanelProps, "resizeSide">;

/** RightPanel holds the data fields for a RightPanel record.
 **/
/**
 **/
const RightPanel: React.FC<RightPanelProps> = (props) => <Panel {...props} resizeSide="left" />;

export { RightPanel };

// #endregion 🎽RightPanel

// #region 🌙MiddlePanel
// Center panel variant without resize handles.

/**
 * Props type for MiddlePanel omitting resizeSide.
 **/
export interface MiddlePanelProps extends Omit<PanelProps, "resizeSide"> {
  resizeSide?: "left" | "right";
}

/**
 * MiddlePanel holds the data fields for a MiddlePanel record.
 **/
const MiddlePanel: React.FC<MiddlePanelProps> = ({ resizeSide = "right", ...props }) => <Panel {...props} resizeSide={resizeSide} />;

export { MiddlePanel };

// #endregion 🌙MiddlePanel

// #region 🏪BottomPanel

// Bottom-docked panel variant with top resize handle.
// Consumers MUST provide visible and children props.

/**
 * Props type for BottomPanel omitting resizeSide.
 *
 **/
export type BottomPanelProps = Omit<PanelProps, "resizeSide">;

/** BottomPanel holds the data fields for a BottomPanel record.
 **/
/**
 **/
const BottomPanel: React.FC<BottomPanelProps> = (props) => <Panel {...props} resizeSide="top" />;

export { BottomPanel };

// #endregion 🏪BottomPanel

// #region 📌SidePanel
// Collapsible side panel with tabbed content.
// Consumers MUST provide SidePanelTabConfig entries.

/**
 * Configuration interface for a side panel tab.
 **/
export interface SidePanelTabConfig {
  id: string;
  icon: React.ComponentType<{ size?: number }>;
  order?: number;
  tree: TreePanelSource;
}

export interface TreePanelConfig {
  sections: TreeDataSection[];
  dragAndDropController?: TreeDragAndDropController;
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  emptyState?: React.ReactNode;
  indentMultiplier?: number;
  className?: string;
}

export interface TreePanelDefinition {
  resolveTree(): TreePanelConfig;
}

export type TreePanelSource = TreePanelConfig | TreePanelDefinition;

export interface SidePanelTabDefinition {
  resolveTab(): SidePanelTabConfig;
}

/** @emoji 🌲 Factory for a static {@link TreePanelDefinition}. */
export function staticTreePanelDefinition(config: TreePanelConfig): TreePanelDefinition {
  return { resolveTree: () => config };
}

/** @emoji 📑 Factory for a static {@link SidePanelTabDefinition}. */
export function staticSidePanelTabDefinition(config: SidePanelTabConfig): SidePanelTabDefinition {
  return { resolveTab: () => config };
}

export type SidePanelTabSource = SidePanelTabConfig | SidePanelTabDefinition;

function resolveTreePanelSource(tree: TreePanelSource): TreePanelConfig {
  if (typeof (tree as TreePanelDefinition).resolveTree === "function") {
    return (tree as TreePanelDefinition).resolveTree();
  }
  return tree;
}

function resolveSidePanelTabSource(tab: SidePanelTabSource): SidePanelTabConfig {
  if (typeof (tab as SidePanelTabDefinition).resolveTab === "function") {
    return (tab as SidePanelTabDefinition).resolveTab();
  }
  return tab;
}

function resolveSidePanelTabs(tabs: readonly SidePanelTabSource[] | undefined): SidePanelTabConfig[] | undefined {
  return tabs?.map(resolveSidePanelTabSource);
}

/** @emoji 🖱️ Pointer-drag props for a host element (replaces imperative drag controllers). */
export function usePointerDrag<TElement extends HTMLElement = HTMLDivElement>(handlers: {
  onStart?: (event: React.PointerEvent<TElement>) => void;
  onMove?: (event: React.PointerEvent<TElement>) => void;
  onEnd?: (event: React.PointerEvent<TElement>) => void;
  onCancel?: (event: React.PointerEvent<TElement>) => void;
}): Pick<React.HTMLAttributes<TElement>, "onPointerCancel" | "onPointerDown" | "onPointerMove" | "onPointerUp"> {
  const activePointerIdRef = reactHostPort.useRef<number | null>(null);
  return reactHostPort.useMemo(
    () => ({
      onPointerDown: (event: React.PointerEvent<TElement>) => {
        activePointerIdRef.current = event.pointerId;
        event.currentTarget.setPointerCapture(event.pointerId);
        handlers.onStart?.(event);
      },
      onPointerMove: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        handlers.onMove?.(event);
      },
      onPointerUp: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        activePointerIdRef.current = null;
        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
          event.currentTarget.releasePointerCapture(event.pointerId);
        }
        handlers.onEnd?.(event);
      },
      onPointerCancel: (event: React.PointerEvent<TElement>) => {
        if (activePointerIdRef.current !== event.pointerId) return;
        activePointerIdRef.current = null;
        if (event.currentTarget.hasPointerCapture(event.pointerId)) {
          event.currentTarget.releasePointerCapture(event.pointerId);
        }
        handlers.onCancel?.(event);
      },
    }),
    [handlers],
  );
}

/** @emoji 📦 Native HTML drag-and-drop event props for a host element. */
export function useNativeDragAndDrop<TElement extends HTMLElement = HTMLDivElement>(
  handlers: {
    onDragStart?: React.DragEventHandler<TElement>;
    onDragEnd?: React.DragEventHandler<TElement>;
    onDragOver?: React.DragEventHandler<TElement>;
    onDrop?: React.DragEventHandler<TElement>;
  },
  draggable = true,
): Pick<React.HTMLAttributes<TElement>, "draggable" | "onDragEnd" | "onDragOver" | "onDragStart" | "onDrop"> {
  return reactHostPort.useMemo(
    () => ({
      draggable,
      onDragStart: handlers.onDragStart,
      onDragEnd: handlers.onDragEnd,
      onDragOver: handlers.onDragOver,
      onDrop: handlers.onDrop,
    }),
    [draggable, handlers.onDragEnd, handlers.onDragOver, handlers.onDragStart, handlers.onDrop],
  );
}

/**
 * Props interface for the SidePanel component.
 **/
export interface SidePanelProps {
  position: "left" | "right";
  visible?: boolean;
  size?: number;
  onSizeChange?: (size: number) => void;
  tabs: SidePanelTabConfig[];
  activeTabId?: string;
  onActiveTabChange?: (tabId: string) => void;
  minSize?: number;
  maxSize?: number;
  zIndex?: 10 | 20 | 30 | 40;
  className?: string;
}

const SidePanel: React.FC<SidePanelProps> = ({ position, visible = true, size = 300, onSizeChange, tabs, activeTabId, onActiveTabChange, minSize = 200, maxSize = 600, zIndex = 20, className = "" }) => {
  const [isResizeHovered, setIsResizeHovered] = reactHostPort.useState(false);
  const [isResizing, setIsResizing] = reactHostPort.useState(false);
  const [internalActiveTab, setInternalActiveTab] = reactHostPort.useState<string | undefined>(tabs[0]?.id);
  const sizeRef = reactHostPort.useRef(size);
  const resizeStartRef = reactHostPort.useRef<{ pointerX: number; size: number } | null>(null);

  reactHostPort.useEffect(() => {
    sizeRef.current = size;
  }, [size]);

  const currentActiveTab = activeTabId ?? internalActiveTab;
  const sortedTabs = [...tabs].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
  const showTabBar = sortedTabs.length > 1;
  const activeTab = sortedTabs.find((tab) => tab.id === currentActiveTab) ?? sortedTabs[0];
  const activeTabTree = activeTab ? resolveTreePanelSource(activeTab.tree) : null;

  const handleTabChange = (tabId: string) => {
    if (onActiveTabChange) {
      onActiveTabChange(tabId);
    } else {
      setInternalActiveTab(tabId);
    }
  };
  const resizeSide = position === "left" ? "right" : "left";

  const resizePointerProps = usePointerDrag<HTMLDivElement>({
    onStart: (event) => {
      event.preventDefault();
      resizeStartRef.current = { pointerX: event.clientX, size: sizeRef.current };
      setIsResizing(true);
    },
    onMove: (event) => {
      const start = resizeStartRef.current;
      if (!start) return;
      const delta = event.clientX - start.pointerX;
      const nextSize = position === "left" ? start.size + delta : start.size - delta;
      if (nextSize >= minSize && nextSize <= maxSize) {
        onSizeChange?.(nextSize);
      }
    },
    onEnd: () => {
      resizeStartRef.current = null;
      setIsResizing(false);
    },
    onCancel: () => {
      resizeStartRef.current = null;
      setIsResizing(false);
    },
  });

  const borderClass = resizeSide === "left" ? (isResizing || isResizeHovered ? "border-l-accent" : "border-l") : isResizing || isResizeHovered ? "border-r-accent" : "border-r";

  const positionStyle =
    position === "left"
      ? { left: "var(--spacing-double)", top: "var(--spacing-double)", bottom: "var(--spacing-double)", width: `${size}px`, zIndex }
      : { right: "var(--spacing-double)", top: "var(--spacing-double)", bottom: "var(--spacing-double)", width: `${size}px`, zIndex };

  const resizeHandleClass = `absolute top-0 bottom-0 ${resizeSide === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize`;

  return (
    <LevelProvider level="panel">
      <div data-panel={position === "left" ? "leftSidePanel" : "rightSidePanel"} className={cn("absolute text-foreground border bg-panel min-w-0 overflow-hidden flex flex-col", borderClass, className)} style={positionStyle}>
        {showTabBar && (
          <div data-slot="side-panel-tabs" className="flex items-center h-medium border-b shrink-0 overflow-x-auto">
            {sortedTabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = tab.id === activeTab?.id;
              return (
                <Tooltip key={tab.id}>
                  <TooltipTrigger asChild>
                    <button
                      data-slot="side-panel-tab-button"
                      id={tab.id}
                      onClick={() => handleTabChange(tab.id)}
                      className={cn("flex items-center justify-center h-full px-small border-r cursor-pointer transition-colors", isActive ? "bg-hover-panel" : "hover:bg-hover-panel")}
                    >
                      <Icon size={16} />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent>
                    <DescriptionTooltipContent id={tab.id} />
                  </TooltipContent>
                </Tooltip>
              );
            })}
          </div>
        )}
        <Scrollable className="flex-1 min-h-0">
          <div data-slot="side-panel-content" className="flex min-h-0 flex-1 flex-col">
            {activeTabTree ? (
              <TreeStateProvider>
                <Tree
                  className={cn("min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden", activeTabTree.className)}
                  defaultSelectedIds={activeTabTree.defaultSelectedIds}
                  dragAndDropController={activeTabTree.dragAndDropController}
                  emptyState={activeTabTree.emptyState}
                  indentMultiplier={activeTabTree.indentMultiplier}
                  onSelectionChange={activeTabTree.onSelectionChange}
                  sections={activeTabTree.sections}
                  selectedIds={activeTabTree.selectedIds}
                  selectionMode={activeTabTree.selectionMode}
                />
              </TreeStateProvider>
            ) : null}
          </div>
        </Scrollable>
        {onSizeChange && <div className={resizeHandleClass} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} {...resizePointerProps} />}
      </div>
    </LevelProvider>
  );
};
export { SidePanel };

// #endregion 📌SidePanel

// #region 💧MobilePanel
// Full-width tabbed panel for mobile layouts. Not resizable. All tabs in one panel.

/**
 * Props interface for the MobilePanel component.
 **/
export interface MobilePanelProps {
  visible?: boolean;
  tabs: SidePanelTabConfig[];
  activeTabId?: string;
  onActiveTabChange?: (tabId: string) => void;
  className?: string;
  height?: number;
}

/**
 * MobilePanel is a full-width tabbed panel for mobile layouts.
 * It merges all tabs into a single non-resizable panel.
 **/
const MobilePanel: React.FC<MobilePanelProps> = ({ visible = true, tabs, activeTabId, onActiveTabChange, className = "", height = 260 }) => {
  const [internalActiveTab, setInternalActiveTab] = reactHostPort.useState<string | undefined>(tabs[0]?.id);

  if (!visible || tabs.length === 0) return null;

  const currentActiveTab = activeTabId ?? internalActiveTab;
  const sortedTabs = [...tabs].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
  const showTabBar = sortedTabs.length > 1;
  const activeTab = sortedTabs.find((tab) => tab.id === currentActiveTab) ?? sortedTabs[0];
  const activeTabTree = activeTab ? resolveTreePanelSource(activeTab.tree) : null;

  const handleTabChange = (tabId: string) => {
    if (onActiveTabChange) {
      onActiveTabChange(tabId);
    } else {
      setInternalActiveTab(tabId);
    }
  };

  return (
    <LevelProvider level="panel">
      <div data-panel="mobilePanel" className={cn("w-full text-foreground border-b bg-panel flex flex-col", className)} style={{ height: `${height}px` }}>
        {showTabBar && (
          <div data-slot="mobile-panel-tabs" className="flex items-center h-large border-b shrink-0 overflow-x-auto">
            {sortedTabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = tab.id === activeTab?.id;
              return (
                <Tooltip key={tab.id}>
                  <TooltipTrigger asChild>
                    <button
                      data-slot="mobile-panel-tab-button"
                      id={tab.id}
                      onClick={() => handleTabChange(tab.id)}
                      className={cn("flex items-center justify-center h-full px-medium border-r cursor-pointer transition-colors", isActive ? "bg-hover-panel" : "hover:bg-hover-panel")}
                    >
                      <Icon size={20} />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent>
                    <DescriptionTooltipContent id={tab.id} />
                  </TooltipContent>
                </Tooltip>
              );
            })}
          </div>
        )}
        <Scrollable className="flex-1 min-h-0">
          <div data-slot="mobile-panel-content" className="flex min-h-0 flex-1 flex-col">
            {activeTabTree ? (
              <TreeStateProvider>
                <Tree
                  className={cn("min-h-0 min-w-0 flex-1 overflow-y-auto overflow-x-hidden", activeTabTree.className)}
                  defaultSelectedIds={activeTabTree.defaultSelectedIds}
                  dragAndDropController={activeTabTree.dragAndDropController}
                  emptyState={activeTabTree.emptyState}
                  indentMultiplier={activeTabTree.indentMultiplier}
                  onSelectionChange={activeTabTree.onSelectionChange}
                  sections={activeTabTree.sections}
                  selectedIds={activeTabTree.selectedIds}
                  selectionMode={activeTabTree.selectionMode}
                />
              </TreeStateProvider>
            ) : null}
          </div>
        </Scrollable>
      </div>
    </LevelProvider>
  );
};
export { MobilePanel };

// #endregion 💧MobilePanel

// #endregion 📷Panel Components

// #region 🩻Toolbar Components

interface ToolbarZoneProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
}

function ToolbarZone({ className, children, ...props }: ToolbarZoneProps) {
  return (
    <div data-slot="toolbar-zone" className={cn("bg-panel flex h-[var(--toolbar-item-height)] shrink-0 items-center gap-[var(--toolbar-gap)] rounded-md shadow-sm overflow-hidden", className)} {...props}>
      {children}
    </div>
  );
}

interface ToolbarGroupProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
}

function ToolbarGroup({ className, children, ...props }: ToolbarGroupProps) {
  return (
    <div data-slot="toolbar-group" role="group" className={cn("flex shrink-0 items-center gap-[var(--toolbar-gap)] h-full", className)} {...props}>
      {children}
    </div>
  );
}

function ToolbarDivider({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="toolbar-divider" className={cn("w-px h-[var(--toolbar-divider-height)] bg-border my-auto shrink-0", className)} {...props} />;
}

interface ToolbarItemProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
}

function ToolbarItem({ className, children, ...props }: ToolbarItemProps) {
  return (
    <div data-slot="toolbar-item" className={cn("shrink-0 flex items-center h-full min-w-0", className)} {...props}>
      {children}
    </div>
  );
}

export { ToolbarDivider, ToolbarGroup, ToolbarItem, ToolbarZone };

// #endregion 🩻Toolbar Components

// #region 🧭Shell

export interface EngagementOption {
  id: string;
  label?: string;
  icon?: React.ReactNode;
  pressed?: boolean;
  disabled?: boolean;
  onPress?: () => void;
}

export interface EngagementInput {
  id?: string;
  value?: string;
  placeholder?: string;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
  disabled?: boolean;
}

export interface EngagementStatus {
  id: string;
  content: React.ReactNode;
}

/** @emoji 🔎 One autocomplete row for {@link EngagementSpec.possibleEngagements} (interaction, transition, …). */
export interface EngagementPossible {
  id: string;
  label: string;
  detail?: string;
  onSelect?: () => void;
}

/** @emoji 🔎 Filters {@link EngagementPossible} rows by label, detail, and id for the engagement command line. */
export function filterEngagementPossibles(query: string, items: readonly EngagementPossible[]): EngagementPossible[] {
  const trimmed = query.trim().toLowerCase();
  if (!trimmed) return [...items];
  return items.filter((item) => {
    const haystack = `${item.label} ${item.detail ?? ""} ${item.id}`.toLowerCase();
    return haystack.includes(trimmed) || item.id.toLowerCase().startsWith(trimmed);
  });
}

/** @emoji 💬 Floating window engagement payload with options, input, and status lines. */
export interface EngagementSpec {
  options?: EngagementOption[];
  input?: EngagementInput;
  status?: EngagementStatus[];
  possibleEngagements?: EngagementPossible[];
}

export interface WindowLayoutWindowNode {
  kind: "window";
  id: string;
  title?: string;
  size?: number;
}

export interface WindowLayoutStackNode {
  kind: "stack";
  size?: number;
  activeId?: string;
  children: readonly WindowLayoutWindowNode[];
}

export interface WindowLayoutAxisNode {
  kind: "row" | "column";
  size?: number;
  children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
}

/** @emoji 🪟 Recursive resizable window layout tree for {@link Mode}. */
export type WindowLayoutNode = WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode;

/** @emoji 🪟 Builds an even horizontal split layout for the given window ids. */
export function createEvenWindowLayout(windowIds: readonly string[]): WindowLayoutNode {
  if (windowIds.length === 0) return { kind: "stack", children: [] };
  if (windowIds.length === 1) return { kind: "stack", children: [{ kind: "window", id: windowIds[0]! }] };
  return {
    kind: "row",
    children: windowIds.map((id) => ({
      kind: "stack" as const,
      children: [{ kind: "window" as const, id }],
    })),
  };
}

export interface EngagementProps extends EngagementSpec {
  className?: string;
  /** @emoji 🎯 When true, focuses the command input whenever this engagement belongs to the globally active window. */
  active?: boolean;
}

/** @emoji 💬 Top-aligned engagement: command input with optional right chevron for possibles; status and option buttons below. */
const Engagement: React.FC<EngagementProps> = ({ options, input, status, possibleEngagements, className = "", active = false }) => {
  const [draft, setDraft] = reactHostPort.useState(input?.value ?? "");
  const [possiblesExpanded, setPossiblesExpanded] = reactHostPort.useState(false);
  const [activePossibleIndex, setActivePossibleIndex] = reactHostPort.useState(0);
  const engagementRef = reactHostPort.useRef<HTMLDivElement>(null);
  const filteredPossibles = reactHostPort.useMemo(
    () => filterEngagementPossibles(draft, possibleEngagements ?? []),
    [draft, possibleEngagements],
  );

  reactHostPort.useEffect(() => {
    setDraft(input?.value ?? "");
  }, [input?.value]);

  reactHostPort.useEffect(() => {
    setActivePossibleIndex((index) => (filteredPossibles.length ? Math.min(index, filteredPossibles.length - 1) : 0));
  }, [filteredPossibles.length, draft]);

  reactHostPort.useEffect(() => {
    setPossiblesExpanded(false);
  }, [possibleEngagements]);

  const hasOptions = !!options?.length;
  const hasInput = !!input;
  const hasStatus = !!status?.length;
  const hasPossibles = !!possibleEngagements?.length;
  const showAutocomplete = hasPossibles && possiblesExpanded && filteredPossibles.length > 0;

  const applyDraft = reactHostPort.useCallback(
    (value: string) => {
      setDraft(value);
      input?.onChange?.(value);
    },
    [input],
  );

  const selectPossible = reactHostPort.useCallback(
    (item: EngagementPossible) => {
      item.onSelect?.();
      applyDraft("");
      setPossiblesExpanded(false);
      setActivePossibleIndex(0);
    },
    [applyDraft],
  );

  const activatePossible = reactHostPort.useCallback((): boolean => {
    if (!filteredPossibles.length) return false;
    selectPossible(filteredPossibles[activePossibleIndex] ?? filteredPossibles[0]!);
    return true;
  }, [activePossibleIndex, filteredPossibles, selectPossible]);

  reactHostPort.useEffect(() => {
    if (!active || !hasInput || input?.disabled) return;
    const field = engagementRef.current?.querySelector<HTMLInputElement>('[data-slot="input"]');
    field?.focus({ preventScroll: true });
  }, [active, hasInput, input?.disabled, input?.id]);

  if (!hasOptions && !hasInput && !hasStatus) return null;

  return (
    <LevelProvider level="overlay">
      <div ref={engagementRef} data-slot="engagement" data-active={active ? "true" : undefined} className={cn("pointer-events-auto flex w-[min(100%,28rem)] flex-col gap-half", className)}>
      {hasInput ? (
        <Popover
          open={showAutocomplete}
          onOpenChange={(open) => {
            if (!open) setPossiblesExpanded(false);
          }}
        >
          <PopoverAnchor asChild>
            <div data-slot="engagement-command-row" className="flex w-full min-w-0 items-stretch gap-half">
              <Input
                id={input!.id ?? "engagement-input"}
                className="min-w-0 flex-1"
                value={draft}
                tabIndex={active ? 0 : -1}
                onChange={(event) => applyDraft(event.target.value)}
                onKeyDown={(event) => {
                  if (event.key === "Escape") {
                    if (possiblesExpanded) {
                      setPossiblesExpanded(false);
                      return;
                    }
                    return;
                  }
                  if (event.key === " " && !event.ctrlKey && !event.metaKey && !event.altKey && possiblesExpanded && hasPossibles) {
                    event.preventDefault();
                    activatePossible();
                    return;
                  }
                  if (event.key === "ArrowDown" && possiblesExpanded && filteredPossibles.length) {
                    event.preventDefault();
                    setActivePossibleIndex((index) => (index + 1) % filteredPossibles.length);
                    return;
                  }
                  if (event.key === "ArrowUp" && possiblesExpanded && filteredPossibles.length) {
                    event.preventDefault();
                    setActivePossibleIndex((index) => (index - 1 + filteredPossibles.length) % filteredPossibles.length);
                    return;
                  }
                  if (event.key === "Enter") {
                    event.preventDefault();
                    if (possiblesExpanded && activatePossible()) return;
                    input!.onSubmit?.(draft);
                  }
                }}
                placeholder={input!.placeholder}
                disabled={input!.disabled}
              />
              {hasPossibles ? (
                <Action
                  id="engagement-possibles-toggle"
                  aria-expanded={possiblesExpanded}
                  data-slot="engagement-possibles-toggle"
                  icon={possiblesExpanded ? <ChevronDownIcon className="size-small" /> : <ChevronRightIcon className="size-small" />}
                  onClick={() => setPossiblesExpanded((open) => !open)}
                />
              ) : null}
            </div>
          </PopoverAnchor>
          {hasPossibles ? (
            <PopoverContent
              data-slot="engagement-autocomplete"
              className="w-[min(100vw-1rem,28rem)] p-0"
              align="end"
              onOpenAutoFocus={(event) => event.preventDefault()}
            >
              <Command shouldFilter={false}>
                <CommandList>
                  {filteredPossibles.length ? (
                    <CommandGroup>
                      {filteredPossibles.map((item, index) => (
                        <CommandItem
                          key={item.id}
                          value={item.id}
                          data-active={index === activePossibleIndex ? "true" : undefined}
                          className={cn(index === activePossibleIndex && "bg-active-base")}
                          onSelect={() => selectPossible(item)}
                        >
                          <span className="truncate">{item.label}</span>
                          {item.detail ? <span className="ml-auto truncate text-xs text-muted-foreground">{item.detail}</span> : null}
                        </CommandItem>
                      ))}
                    </CommandGroup>
                  ) : (
                    <CommandEmpty>No matches</CommandEmpty>
                  )}
                </CommandList>
              </Command>
            </PopoverContent>
          ) : null}
        </Popover>
      ) : null}
      {hasStatus ? (
        <div data-slot="engagement-status" className="flex flex-wrap items-center justify-center gap-single text-xs text-muted-foreground">
          {status!.map((item) => (
            <span key={item.id} data-slot="engagement-status-item">
              {item.content}
            </span>
          ))}
        </div>
      ) : null}
      {hasOptions ? (
        <div data-slot="engagement-options" className="flex flex-wrap items-center justify-center gap-half">
          <ButtonGroup id="engagement-options">
            {options!.map((option) => (
              <ButtonGroupItem
                key={option.id}
                id={option.id}
                icon={option.icon}
                text={option.label}
                className={cn(option.pressed && "bg-active-base")}
                onClick={option.onPress}
                disabled={option.disabled}
              />
            ))}
          </ButtonGroup>
        </div>
      ) : null}
      </div>
    </LevelProvider>
  );
};

export { Engagement };

// #endregion 🧭Shell

// #region 🔍Window Components

// #region 🌊Window

export interface WindowConfig {
  id: string;
  children: React.ReactNode;
  defaultSize?: number;
  onDoubleClick?: () => void;
  className?: string;
  loading?: boolean;
  error?: Error | null;
  skeleton?: React.ReactNode;
  showControls?: boolean;
  onOpenInNewWindow?: () => void;
  onMaximize?: () => void;
  onMinimize?: () => void;
  onClose?: () => void;
  controls?: React.ReactNode;
  measures?: React.ReactNode;
  engagement?: EngagementSpec;
  active?: boolean;
  onActivate?: () => void;
}

/**
 * WindowProps holds the data fields for a WindowProps record.
 **/
interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

/**
 * DefaultErrorDisplay holds the data fields for a DefaultErrorDisplay record.
 **/
const DefaultErrorDisplay: React.FC<{ error: Error }> = ({ error }) => {
  const bgClass = "bg-window";
  return (
    <div className={cn("flex flex-col items-center justify-center h-full w-full p-small", bgClass)}>
      <div className="text-center space-y-2 max-w-md">
        <div className="text-4xl mb-4">⚠️</div>
        <h3 className="text-lg font-medium">Error</h3>
        <p className="text-sm text-muted-foreground">{error.message}</p>
      </div>
    </div>
  );
};

/**
 * Window holds the data fields for a Window record.
 **/
const Window: React.FC<WindowProps> = ({ id, children, onDoubleClick, className = "", isVisible = true, loading = false, error = null, skeleton, showControls = false, onOpenInNewWindow, onMaximize, onMinimize, onClose, controls, measures, engagement, active = false, onActivate }) => {
  const bgClass = "bg-window";

  if (!isVisible) return null;

  const hasControls = showControls || controls || onOpenInNewWindow || onMaximize || onMinimize || onClose;

  const controlsContent = hasControls && (
    <div className="flex items-stretch gap-single">
      {controls}
      {(showControls || onOpenInNewWindow || onMaximize || onMinimize || onClose) && (
        <ActionGroup id={`${id}-window-controls`}>
          {onOpenInNewWindow && (
            <ActionGroupItem id={`${id}-window-controls-external`} onClick={onOpenInNewWindow}>
              <ExternalLinkIcon />
            </ActionGroupItem>
          )}
          {(onMaximize || onMinimize) && (
            <ActionGroupItem id={`${id}-window-controls-maximize`} onClick={onMaximize ?? onMinimize}>
              <Maximize2Icon />
            </ActionGroupItem>
          )}
          {onClose && (
            <ActionGroupItem id={`${id}-window-controls-close`} onClick={onClose}>
              <CloseIcon />
            </ActionGroupItem>
          )}
        </ActionGroup>
      )}
    </div>
  );

  return (
    <LevelProvider level="window">
      <div
        data-slot="window"
        data-active={active ? "true" : undefined}
        onDoubleClick={onDoubleClick}
        onPointerDownCapture={() => onActivate?.()}
        className={cn(`relative flex h-full min-h-0 w-full min-w-0 flex-col overflow-hidden ${bgClass}`, className)}
      >
        {hasControls ? <div className="absolute top-1 right-1 z-panel flex items-stretch gap-single">{controlsContent}</div> : null}
        <div className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
          {error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}
          {measures ? (
            <div
              data-slot="window-measures-overlay"
              className="pointer-events-none absolute top-0 right-0 bottom-0 left-auto z-panel flex w-max max-w-[min(11rem,calc(100%-0.5rem))] flex-col items-end justify-start gap-half overflow-hidden p-single"
            >
              <div
                data-slot="window-measures-stack"
                className="pointer-events-auto flex max-h-full max-w-[min(11rem,calc(100%-0.5rem))] flex-col items-end gap-half overflow-y-auto overscroll-contain"
              >
                {measures}
              </div>
            </div>
          ) : null}
          {engagement && active ? (
            <div
              data-slot="window-engagement-overlay"
              className="pointer-events-none absolute inset-x-0 top-0 z-window flex items-start justify-center px-single pt-single"
            >
              <Engagement {...engagement} active />
            </div>
          ) : null}
        </div>
      </div>
    </LevelProvider>
  );
};

export { Window };

// #endregion 🌊Window

// #region 🌈Page
// Full-page content wrapper with frontmatter and footer.
// Consumers MUST provide frontmatter and children.

/**
 * Frontmatter metadata interface for a documentation page.
 **/
export interface PageFrontmatter {
  title?: string;
  description?: string;
  icon?: string;
  sidebar?: boolean;
  order?: number;
  concepts?: string[];
}

/**
 * Props interface for the Page component.
 **/
export interface PageProps {
  frontmatter?: PageFrontmatter;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  footer?: React.ReactNode;
  children: React.ReactNode;
}

/**
 * Full-page wrapper with frontmatter header and footer.
 **/
export const Page: React.FC<PageProps> = ({ frontmatter, focusedItemId, onFocusComplete, footer, children }) => {
  const scrollAreaRef = reactHostPort.useRef<HTMLDivElement>(null);

  reactHostPort.useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const element = getElementById(focusedItemId);
      if (element) {
        element.scrollIntoView({ behavior: "smooth", block: "center" });
        if (onFocusComplete) {
          setTimeout(() => onFocusComplete(), 600);
        }
      }
    }
  }, [focusedItemId, onFocusComplete]);
  return (
    <Scrollable ref={scrollAreaRef} className="h-full w-full">
      <div className="prose prose-sm max-w-none dark:prose-invert p-medium">
        {frontmatter?.title && <h1>{frontmatter.title}</h1>}
        {frontmatter?.description && <p className="text-muted-foreground">{frontmatter.description}</p>}
        {children}
        {footer}
      </div>
    </Scrollable>
  );
};
// #endregion 🌈Page

// #region 🧫Diagram
// Interactive node-edge diagram built on ReactFlow and D3 force.
// Consumers MUST provide nodes and edges arrays.

export {
  applyNodeChanges,
  Background,
  BackgroundVariant,
  BaseEdge,
  forceCenter,
  forceCollide,
  forceLink,
  forceManyBody,
  forceSimulation,
  forceX,
  forceY,
  getBezierPath,
  Handle,
  Position,
  ReactFlow,
  ReactFlowProvider,
  useInternalNode,
  useReactFlow,
  useStoreApi,
  ViewportPortal,
};
export type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, ReactFlowInstance, Connection as RFConnection, Simulation, SimulationLinkDatum, SimulationNodeDatum };

/**
 * Base pixel unit for diagram node sizing.
 **/
export const DIAGRAM_UNIT = 48;

/**
 * Union type for diagram layout directions (TB/BT/LR/RL).
 **/
export type DiagramLayoutDirection = "TB" | "BT" | "LR" | "RL";

/**
 * Configuration interface for dagre-based diagram layout.
 **/
export interface DiagramLayoutOptions {
  direction?: DiagramLayoutDirection;
  nodeWidth?: number;
  nodeHeight?: number;
  rankSep?: number;
  nodeSep?: number;
}

/**
 * Computes dagre layout positions for diagram nodes and edges.
 **/
export function calculateDiagramLayout(nodes: Node[], edges: Edge[], options: DiagramLayoutOptions = {}): { nodes: Node[]; edges: Edge[] } {
  const { direction = "TB", nodeWidth = DIAGRAM_UNIT, nodeHeight = DIAGRAM_UNIT, rankSep = DIAGRAM_UNIT * 1.67, nodeSep = DIAGRAM_UNIT * 1.04 } = options;

  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({ rankdir: direction, ranksep: rankSep, nodesep: nodeSep });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  const layoutedNodes = nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    return {
      ...node,
      position: {
        x: nodeWithPosition.x - nodeWidth / 2,
        y: nodeWithPosition.y - nodeHeight / 2,
      },
    };
  });

  return { nodes: layoutedNodes, edges };
}

/**
 * Configuration interface for D3 force simulation parameters.
 **/
export interface DiagramForceConfig {
  enabled: boolean;
  chargeStrength?: number;
  linkDistance?: number;
  collideRadius?: number;
  centerStrength?: number;
  updateIntervalMs?: number;
}

/**
 * Default D3 force configuration values.
 **/
export const defaultDiagramForceConfig: DiagramForceConfig = {
  enabled: false,
  chargeStrength: -DIAGRAM_UNIT * 1.67,
  linkDistance: DIAGRAM_UNIT * 1.25,
  collideRadius: DIAGRAM_UNIT * 0.625,
  centerStrength: 0.15,
  updateIntervalMs: 50,
};

/**
 * ForceNode holds the data fields for a ForceNode record.
 **/
interface ForceNode extends SimulationNodeDatum {
  id: string;
  data: any;
}

/**
 * ForceLink holds the data fields for a ForceLink record.
 **/
interface ForceLink extends SimulationLinkDatum<ForceNode> {
  id: string;
}

/**
 * Props interface for the Diagram component.
 **/
export interface DiagramProps {
  nodeTypes: NodeTypes;
  edgeTypes?: EdgeTypes;
  initialNodes?: Node[];
  initialEdges?: Edge[];
  nodes?: Node[];
  edges?: Edge[];
  onNodesChange?: (nodes: Node[]) => void;
  onEdgesChange?: (edges: Edge[]) => void;
  onNodesChangeReactFlow?: (changes: any[]) => void;
  onEdgesChangeReactFlow?: (changes: any[]) => void;
  onConnect?: (connection: any) => void;
  onNodeClick?: (event: React.MouseEvent, node: Node) => void;
  onNodeDoubleClick?: (event: React.MouseEvent, node: Node) => void;
  onNodeMouseEnter?: (event: React.MouseEvent, node: Node) => void;
  onNodeMouseLeave?: (event: React.MouseEvent, node: Node) => void;
  onNodeDragStart?: (event: React.MouseEvent, node: Node) => void;
  onNodeDrag?: (event: React.MouseEvent, node: Node) => void;
  onNodeDragStop?: (event: React.MouseEvent, node: Node) => void;
  onEdgeClick?: (event: React.MouseEvent, edge: Edge) => void;
  onEdgeMouseEnter?: (event: React.MouseEvent, edge: Edge) => void;
  onEdgeMouseLeave?: (event: React.MouseEvent, edge: Edge) => void;
  onPaneClick?: (event: React.MouseEvent) => void;
  onPaneDoubleClick?: (event: React.MouseEvent) => void;
  onMoveStart?: () => void;
  onMoveEnd?: () => void;
  reactFlowInstanceRef?: React.RefObject<ReactFlowInstance | null>;
  onInit?: (instance: ReactFlowInstance) => void;
  wrapperRef?: React.RefObject<HTMLDivElement> | ((node: HTMLDivElement | null) => void);
  showBackground?: boolean;
  backgroundVariant?: BackgroundVariant;
  showControls?: boolean;
  showMinimap?: boolean;
  panels?: React.ReactNode;
  className?: string;
  fitView?: boolean;
  minZoom?: number;
  maxZoom?: number;
  defaultZoom?: number;
  connectionMode?: "strict" | "loose";
  connectionLineComponent?: any;
  deleteKeyCode?: string | string[];
  panOnDrag?: boolean | number[];
  selectionOnDrag?: boolean;
  zoomOnScroll?: boolean;
  zoomOnPinch?: boolean;
  zoomOnDoubleClick?: boolean;
  elementsSelectable?: boolean;
  nodesFocusable?: boolean;
  edgesFocusable?: boolean;
  nodesDraggable?: boolean;
  miniMapNodeComponent?: any;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  forceConfig?: Partial<DiagramForceConfig>;
  selectionMode?: SelectionMode;
  panOnScroll?: boolean;
  proOptions?: { hideAttribution: boolean };
  onSelectionChange?: (selection: OnSelectionChangeParams) => void;
  onSelectionStart?: (event: React.MouseEvent) => void;
  onSelectionEnd?: (event: React.MouseEvent) => void;
  defaultViewport?: { x: number; y: number; zoom: number };
  autoPanOnNodeDrag?: boolean;
  selectNodesOnDrag?: boolean;
}

/**
 * DiagramInner holds the data fields for a DiagramInner record.
 **/
const DiagramInner: React.FC<DiagramProps> = ({
  nodeTypes,
  edgeTypes,
  initialNodes = [],
  initialEdges = [],
  nodes: controlledNodes,
  edges: controlledEdges,
  onNodesChange: onNodesChangeProp,
  onEdgesChange: onEdgesChangeProp,
  onNodesChangeReactFlow,
  onEdgesChangeReactFlow,
  onConnect,
  onNodeClick,
  onNodeDoubleClick,
  onNodeMouseEnter,
  onNodeMouseLeave,
  onNodeDragStart: onNodeDragStartProp,
  onNodeDrag: onNodeDragProp,
  onNodeDragStop: onNodeDragStopProp,
  onEdgeClick,
  onEdgeMouseEnter,
  onEdgeMouseLeave,
  onPaneClick,
  onPaneDoubleClick,
  onMoveStart,
  onMoveEnd,
  reactFlowInstanceRef,
  onInit: onInitProp,
  wrapperRef,
  showMinimap = false,
  panels,
  className = "",
  fitView = true,
  minZoom = 0.1,
  maxZoom = 12,
  connectionMode = "loose",
  connectionLineComponent,
  deleteKeyCode = "Delete",
  panOnDrag = [0],
  selectionOnDrag = false,
  zoomOnScroll = true,
  zoomOnPinch = true,
  zoomOnDoubleClick = false,
  elementsSelectable = false,
  nodesFocusable = false,
  edgesFocusable = false,
  nodesDraggable = true,
  miniMapNodeComponent,
  focusedItemId,
  onFocusComplete,
  forceConfig: forceConfigProp,
  selectionMode = SelectionMode.Partial,
  panOnScroll = false,
  proOptions = { hideAttribution: true },
  onSelectionChange,
  onSelectionStart,
  onSelectionEnd,
  defaultViewport,
  autoPanOnNodeDrag,
  selectNodesOnDrag,
}) => {
  const forceConfig = reactHostPort.useMemo(() => ({ ...defaultDiagramForceConfig, ...forceConfigProp }), [forceConfigProp]);
  const simulationRef = reactHostPort.useRef<Simulation<any, any> | null>(null);
  const draggingNodeRef = reactHostPort.useRef<string | null>(null);
  const isControlled = controlledNodes !== undefined && controlledEdges !== undefined;
  const rfStoreApi = useStoreApi();
  reactHostPort.useEffect(() => {
    const original = rfStoreApi.setState;
    const api = rfStoreApi as any;
    api.__suppressTransform = false;
    api.__pendingTransform = null;
    api.__original = original;
    rfStoreApi.setState = ((partial: any, replace: any) => {
      if (typeof partial === "object" && partial !== null && !replace) {
        const state = rfStoreApi.getState();
        const keys = Object.keys(partial);
        if (keys.length > 0 && keys.every((k) => Object.is((state as any)[k], partial[k]))) return;
        if (api.__suppressTransform && keys.length === 1 && keys[0] === "transform") {
          const t = partial.transform;
          const el = queryElement<HTMLElement>(".react-flow__viewport");
          if (el) el.style.transform = `translate(${t[0]}px, ${t[1]}px) scale(${t[2]})`;
          api.__pendingTransform = t;
          return;
        }
      }
      return original(partial, replace);
    }) as typeof original;
    return () => {
      rfStoreApi.setState = original;
    };
  }, [rfStoreApi]);

  const [internalNodes, setInternalNodes] = reactHostPort.useState<Node[]>(initialNodes);
  const [internalEdges, setInternalEdges] = reactHostPort.useState<Edge[]>(initialEdges);

  const finalNodes = isControlled ? controlledNodes : internalNodes;
  const finalEdges = isControlled ? controlledEdges : internalEdges;

  const onNodesChangeReactFlowRef = reactHostPort.useRef(onNodesChangeReactFlow);
  onNodesChangeReactFlowRef.current = onNodesChangeReactFlow;
  const onNodeDragStartPropRef = reactHostPort.useRef(onNodeDragStartProp);
  onNodeDragStartPropRef.current = onNodeDragStartProp;
  const onNodeDragPropRef = reactHostPort.useRef(onNodeDragProp);
  onNodeDragPropRef.current = onNodeDragProp;
  const onNodeDragStopPropRef = reactHostPort.useRef(onNodeDragStopProp);
  onNodeDragStopPropRef.current = onNodeDragStopProp;
  const onInitPropRef = reactHostPort.useRef(onInitProp);
  onInitPropRef.current = onInitProp;
  const onConnectRef = reactHostPort.useRef(onConnect);
  onConnectRef.current = onConnect;
  const onMoveStartRef = reactHostPort.useRef(onMoveStart);
  onMoveStartRef.current = onMoveStart;
  const onMoveEndRef = reactHostPort.useRef(onMoveEnd);
  onMoveEndRef.current = onMoveEnd;
  const onSelectionChangeRef = reactHostPort.useRef(onSelectionChange);
  onSelectionChangeRef.current = onSelectionChange;
  const finalNodesRef = reactHostPort.useRef(finalNodes);
  finalNodesRef.current = finalNodes;

  const handleNodesChange = reactHostPort.useCallback(
    (changes: any[]) => {
      onNodesChangeReactFlowRef.current?.(changes);
      if (!isControlled) {
        setInternalNodes((nds) => applyNodeChanges(changes, nds));
      }
    },
    [isControlled],
  );

  const handleEdgesChange = reactHostPort.useCallback(
    (changes: any[]) => {
      if (!isControlled) {
        setInternalEdges((eds) => {
          const updated = [...eds];
          for (const change of changes) {
            if (change.type === "remove") {
              const idx = updated.findIndex((e) => e.id === change.id);
              if (idx !== -1) updated.splice(idx, 1);
            }
          }
          return updated;
        });
      }
    },
    [isControlled],
  );

  const handleInit = reactHostPort.useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        (reactFlowInstanceRef as any).current = instance;
      }
      onInitPropRef.current?.(instance);
    },
    [reactFlowInstanceRef],
  );

  const handleNodeDragStart = reactHostPort.useCallback(
    (event: React.MouseEvent, node: Node) => {
      draggingNodeRef.current = node.id;
      if (forceConfig.enabled && simulationRef.current) {
        const currentPositions = new Map(finalNodesRef.current.map((n) => [n.id, n.position]));
        const simNode = simulationRef.current.nodes().find((currentNode) => currentNode.id === node.id);
        for (const simNode of simulationRef.current.nodes()) {
          const pos = currentPositions.get(simNode.id);
          if (pos) {
            simNode.x = pos.x;
          }
        }
        if (simNode) {
          simNode.fx = node.position.x;
          simNode.fy = node.position.y;
          simulationRef.current.alphaTarget(0.3).restart();
        }
      }
      onNodeDragStartPropRef.current?.(event, node);
    },
    [forceConfig.enabled],
  );

  const handleNodeDrag = reactHostPort.useCallback(
    (event: React.MouseEvent, node: Node) => {
      if (draggingNodeRef.current !== node.id) return;
      if (forceConfig.enabled && simulationRef.current) {
        const selectedNodes = finalNodesRef.current.filter((n) => n.selected);
        if (selectedNodes.length > 1 && node.selected) {
          const currentPositions = new Map(finalNodesRef.current.map((n) => [n.id, n.position]));
          for (const simNode of simulationRef.current.nodes()) {
            const pos = currentPositions.get(simNode.id);
            if (pos && selectedNodes.find((sn) => sn.id === simNode.id)) {
              simNode.fx = pos.x;
              simNode.fy = pos.y;
            }
          }
        } else {
          const simNode = simulationRef.current.nodes().find((n) => n.id === node.id);
          if (simNode) {
            simNode.fx = node.position.x;
            simNode.fy = node.position.y;
          }
        }
      }
      onNodeDragPropRef.current?.(event, node);
    },
    [forceConfig.enabled],
  );

  const handleNodeDragStop = reactHostPort.useCallback(
    (event: React.MouseEvent, node: Node) => {
      if (forceConfig.enabled && simulationRef.current) {
        simulationRef.current.alphaTarget(0);
        for (const simNode of simulationRef.current.nodes()) {
          simNode.fx = null;
          simNode.fy = null;
        }
      }
      draggingNodeRef.current = null;
      onNodeDragStopPropRef.current?.(event, node);
    },
    [forceConfig.enabled],
  );

  const stableOnConnect = reactHostPort.useCallback((connection: any) => {
    onConnectRef.current?.(connection);
  }, []);
  const stableOnMoveStart = reactHostPort.useCallback(() => {
    onMoveStartRef.current?.();
  }, []);
  const stableOnMoveEnd = reactHostPort.useCallback(() => {
    onMoveEndRef.current?.();
  }, []);
  const stableOnSelectionChange = reactHostPort.useCallback((selection: OnSelectionChangeParams) => {
    onSelectionChangeRef.current?.(selection);
  }, []);

  reactHostPort.useEffect(() => {
    if (!forceConfig.enabled || finalNodes.length === 0) {
      simulationRef.current = null;
      return;
    }

    const nodesCopy: ForceNode[] = finalNodes.map((n) => ({
      id: n.id,
      x: n.position.x,
      y: n.position.y,
      data: n.data,
    }));

    const linksCopy: ForceLink[] = finalEdges.map((e) => ({
      id: e.id,
      source: e.source,
      target: e.target,
    }));

    const simulation = forceSimulation<ForceNode, ForceLink>(nodesCopy)
      .force("charge", forceManyBody().strength(forceConfig.chargeStrength ?? -100))
      .force(
        "link",
        forceLink<ForceNode, ForceLink>(linksCopy)
          .id((d) => d.id)
          .distance(forceConfig.linkDistance ?? 100),
      )
      .force("collide", forceCollide().radius(forceConfig.collideRadius ?? 50))
      .force("x", forceX(0).strength(forceConfig.centerStrength ?? 0.1))
      .force("y", forceY(0).strength(forceConfig.centerStrength ?? 0.1))
      .stop();

    // 🔷Run simulation synchronously to completion once
    const numTicks = Math.ceil(Math.log(simulation.alphaMin()) / Math.log(1 - simulation.alphaDecay()));
    for (let i = 0; i < numTicks; i++) {
      simulation.tick();
    }

    // 🌿Set final positions once
    const positionedNodes = finalNodes.map((node) => {
      const simNode = simulation.nodes().find((n) => n.id === node.id);
      return {
        ...node,
        position: { x: simNode?.x ?? 0, y: simNode?.y ?? 0 },
      };
    });

    if (!isControlled) {
      setInternalNodes(positionedNodes);
    } else if (onNodesChangeProp) {
      onNodesChangeProp(positionedNodes);
    }

    simulation.on("tick", () => {
      if (!isControlled) {
        setInternalNodes((nds) =>
          nds.map((node) => {
            const simNode = simulation.nodes().find((n) => n.id === node.id);
            if (simNode) {
              return {
                ...node,
                position: { x: simNode.x ?? 0, y: simNode.y ?? 0 },
              };
            }
            return node;
          }),
        );
      } else if (onNodesChangeProp) {
        onNodesChangeProp(
          simulation.nodes().map((n) => {
            const original = finalNodes.find((fn) => fn.id === n.id)!;
            return {
              ...original,
              position: { x: n.x ?? 0, y: n.y ?? 0 },
            };
          }),
        );
      }
    });

    simulationRef.current = simulation;

    return () => {
      simulation.stop();
      simulationRef.current = null;
    };
  }, [forceConfig.enabled, forceConfig.chargeStrength, forceConfig.linkDistance, forceConfig.collideRadius, forceConfig.centerStrength, finalNodes.length, finalEdges.length, isControlled, onNodesChangeProp]);

  reactHostPort.useEffect(() => {
    if (focusedItemId && reactFlowInstanceRef?.current) {
      const node = finalNodes.find((n) => n.id === focusedItemId);
      const edge = finalEdges.find((e) => e.id === focusedItemId);

      if (node) {
        reactFlowInstanceRef.current.fitView({
          padding: 0.5,
          duration: 600,
          nodes: [node],
        });
      } else if (edge) {
        const sourceNode = finalNodes.find((n) => n.id === edge.source);
        const targetNode = finalNodes.find((n) => n.id === edge.target);
        const nodesToFit = [sourceNode, targetNode].filter(Boolean) as Node[];
        if (nodesToFit.length > 0) {
          reactFlowInstanceRef.current.fitView({
            padding: 0.5,
            duration: 600,
            nodes: nodesToFit,
          });
        }
      }

      if (onFocusComplete) {
        setTimeout(() => onFocusComplete(), 600);
      }
    }
  }, [focusedItemId, finalNodes, finalEdges, reactFlowInstanceRef, onFocusComplete]);

  reactHostPort.useEffect(() => {
    if (!isControlled) {
      setInternalNodes(initialNodes);
      setInternalEdges(initialEdges);
    }
  }, [initialNodes, initialEdges, isControlled]);

  reactHostPort.useEffect(() => {
    if (!isControlled && onNodesChangeProp) {
      onNodesChangeProp(internalNodes);
    }
  }, [internalNodes, onNodesChangeProp, isControlled]);

  reactHostPort.useEffect(() => {
    if (!isControlled && onEdgesChangeProp) {
      onEdgesChangeProp(internalEdges);
    }
  }, [internalEdges, onEdgesChangeProp, isControlled]);

  return (
    <div ref={wrapperRef as any} className={`relative w-full h-full ${className}`}>
      <HostReactFlow
        nodes={finalNodes}
        edges={finalEdges}
        onNodesChange={handleNodesChange}
        onEdgesChange={handleEdgesChange}
        onConnect={stableOnConnect}
        onInit={handleInit}
        onNodeClick={onNodeClick}
        onNodeDoubleClick={onNodeDoubleClick}
        onNodeMouseEnter={onNodeMouseEnter}
        onNodeMouseLeave={onNodeMouseLeave}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onEdgeClick={onEdgeClick}
        onEdgeMouseEnter={onEdgeMouseEnter}
        onEdgeMouseLeave={onEdgeMouseLeave}
        onPaneClick={onPaneClick}
        onDoubleClick={onPaneDoubleClick}
        onMoveStart={stableOnMoveStart}
        onMoveEnd={stableOnMoveEnd}
        onSelectionChange={stableOnSelectionChange}
        onSelectionStart={onSelectionStart}
        onSelectionEnd={onSelectionEnd}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionLineComponent={connectionLineComponent}
        fitView={fitView}
        minZoom={minZoom}
        maxZoom={maxZoom}
        defaultViewport={defaultViewport}
        connectionMode={connectionMode === "loose" ? ConnectionMode.Loose : ConnectionMode.Strict}
        deleteKeyCode={deleteKeyCode}
        panOnDrag={panOnDrag}
        panOnScroll={panOnScroll}
        preventScrolling={true}
        selectionOnDrag={selectionOnDrag}
        selectionMode={selectionMode}
        zoomOnScroll={zoomOnScroll}
        zoomOnPinch={zoomOnPinch}
        zoomOnDoubleClick={zoomOnDoubleClick}
        elementsSelectable={elementsSelectable}
        nodesFocusable={nodesFocusable}
        edgesFocusable={edgesFocusable}
        nodesDraggable={nodesDraggable}
        autoPanOnNodeDrag={autoPanOnNodeDrag}
        selectNodesOnDrag={selectNodesOnDrag}
        proOptions={proOptions}
        className="bg-background"
      >
        {showMinimap && <MiniMap className="border border-element" maskColor="var(--accent)" bgColor="var(--background)" nodeStrokeWidth={3} zoomable pannable nodeComponent={miniMapNodeComponent} />}
        {panels}
      </HostReactFlow>
    </div>
  );
};

/**
 * Diagram holds the data fields for a Diagram record.
 **/
const Diagram: React.FC<DiagramProps> = (props) => {
  return (
    <HostReactFlowProvider>
      <DiagramInner {...props} />
    </HostReactFlowProvider>
  );
};

export { Diagram, SelectionMode };
export type { OnSelectionChangeParams };

/**
 * Hook computing and memoizing diagram layout from nodes and edges.
 **/
export function useDiagramLayout(initialNodes: Node[], initialEdges: Edge[], layoutOptions?: DiagramLayoutOptions): { nodes: Node[]; edges: Edge[] } {
  return reactHostPort.useMemo(() => {
    if (initialNodes.length === 0) {
      return { nodes: [], edges: [] };
    }
    return calculateDiagramLayout(initialNodes, initialEdges, layoutOptions);
  }, [initialNodes, initialEdges, layoutOptions]);
}

/**
 * DiagramSkeletonProps holds the data fields for a DiagramSkeletonProps record.
 **/
interface DiagramSkeletonProps {
  nodeCount?: number;
  edgeCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a diagram.
 **/
export const DiagramSkeleton: React.FC<DiagramSkeletonProps> = ({ nodeCount = 5, edgeCount = 4, className = "" }) => {
  const skeletonNodes: Node[] = reactHostPort.useMemo(
    () =>
      Array.from({ length: nodeCount }).map((_, i) => ({
        id: `skeleton-node-${i}`,
        type: "default",
        position: { x: (i % 3) * 150 + 50, y: Math.floor(i / 3) * 150 + 50 },
        data: { label: " " },
        draggable: false,
      })),
    [nodeCount],
  );
  const skeletonEdges: Edge[] = reactHostPort.useMemo(
    () =>
      Array.from({ length: edgeCount }).map((_, i) => ({
        id: `skeleton-edge-${i}`,
        source: `skeleton-node-${i}`,
        target: `skeleton-node-${Math.min(i + 1, nodeCount - 1)}`,
        animated: false,
      })),
    [edgeCount, nodeCount],
  );
  return (
    <div className={`relative w-full h-full ${className}`}>
      <HostReactFlow
        nodes={skeletonNodes}
        edges={skeletonEdges}
        nodeTypes={{}}
        edgeTypes={{}}
        nodesDraggable={false}
        elementsSelectable={false}
        panOnDrag={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        proOptions={{ hideAttribution: true }}
        className="bg-background animate-pulse opacity-50"
      ></HostReactFlow>
    </div>
  );
};

// #endregion 🧫Diagram

// #region 📍Scene
// 3D scene viewer built on React Three Fiber.
// Consumers MUST provide SceneGeometry data.

export const sceneFrameControlRef: { current: { pause: () => void; resume: () => void } | null } = { current: null };
const SceneFrameControl: React.FC = () => {
  const gl = useThree((s) => s.gl);
  const setFrameloop = useThree((s) => s.setFrameloop);
  const invalidate = useThree((s) => s.invalidate);
  reactHostPort.useEffect(() => {
    sceneFrameControlRef.current = {
      pause: () => setFrameloop("never"),
      resume: () => {
        setFrameloop("demand");
        invalidate();
      },
    };
    return () => {
      sceneFrameControlRef.current = null;
    };
  }, [gl, setFrameloop, invalidate]);
  return null;
};

const _elementsComputedColorCache = new Map<string, string>();
const getComputedColor = (variable: string): string => {
  const cached = _elementsComputedColorCache.get(variable);
  if (cached !== undefined) return cached;
  const value = getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  _elementsComputedColorCache.set(variable, value);
  return value;
};

/**
 * selectableCursorUsageCount holds the data fields for a selectableCursorUsageCount record.
 **/
let selectableCursorUsageCount = 0;

/**
 * Interface for a geometry entry in a 3D scene.
 **/
export interface SceneGeometry {
  guid: string;
  plane?: Plane;
  isSelected?: boolean;
  isHovered?: boolean;
  isFocusable?: boolean;
  onClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
}

/**
 * Extended SceneGeometry with transform delta support.
 **/
export interface TransformableGeometry extends SceneGeometry {
  isTransformable?: boolean;
}

/**
 * Interface for an incremental plane transformation delta.
 **/
export interface PlaneTransformDelta {
  translation?: { x: number; y: number; z: number };
  rotation?: { x: number; y: number; z: number; w: number };
  scale?: number;
}

/**
 * Callback type for a single plane update.
 **/
export type OnPlaneUpdate = (geometryGuid: string, newPlane: Plane) => void;

/**
 * Callback type for batch plane updates.
 **/
export type OnMultiPlaneUpdate = (updates: Array<{ geometryGuid: string; newPlane: Plane }>) => void;

/**
 * Constructs a Plane from a point and direction vector.
 **/
export const planeFromPointAndDirection = (point: Point, direction: Vector): Plane => {
  const dir = new THREE.Vector3(direction.x, direction.y, direction.z).normalize();

  const tempVec = Math.abs(dir.z) < 0.9 ? new THREE.Vector3(0, 0, 1) : new THREE.Vector3(1, 0, 0);

  const xAxis = new THREE.Vector3().crossVectors(tempVec, dir).normalize();
  const yAxis = new THREE.Vector3().crossVectors(dir, xAxis).normalize();

  return {
    origin: { x: point.x, y: point.y, z: point.z },
    xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
    yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
  };
};

/**
 * Extracts the THREE.Vector3 position from a Plane.
 **/
export const getPlanePosition = (plane: Plane): THREE.Vector3 => {
  return new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
};

/**
 * Checks whether a geometry has a non-null plane.
 **/
export const hasValidPlane = (geometry: SceneGeometry): boolean => {
  return geometry.plane !== undefined && geometry.plane !== null;
};

/**
 * Checks whether a geometry has a valid plane for camera focus.
 **/
export const isGeometryFocusable = (geometry: SceneGeometry): boolean => {
  return hasValidPlane(geometry) && (geometry.isFocusable === undefined || geometry.isFocusable === true);
};

/**
 * GeometryProps holds the data fields for a GeometryProps record.
 **/
interface GeometryProps {
  children?: React.ReactNode;
  selected?: boolean;
  hovered?: boolean;
  onClick?: (event: ThreeEvent<MouseEvent>) => void;
  onDoubleClick?: (event: ThreeEvent<MouseEvent>) => void;
  onPointerEnter?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerLeave?: (event: ThreeEvent<PointerEvent>) => void;
  color?: string;
  emissiveColor?: string;
  emissiveIntensity?: number;
  showEdges?: boolean;
  edgeColor?: string;
  userData?: any;
}

/**
 * 3D geometry mesh component with selection, hover, and edge rendering.
 **/
export const Geometry: React.FC<GeometryProps> = ({ children, selected = false, hovered = false, onClick, onDoubleClick, onPointerEnter, onPointerLeave, color, emissiveColor, emissiveIntensity = 0.45, showEdges = true, edgeColor, userData }) => {
  const foregroundColor = reactHostPort.useMemo(() => getComputedColor("--foreground"), []);
  const activeBaseColor = reactHostPort.useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = reactHostPort.useMemo(() => getComputedColor("--hover-base"), []);
  const [isPointerOver, setIsPointerOver] = reactHostPort.useState(false);
  const isInteractive = Boolean(onClick || onDoubleClick);

  const resolvedColor = reactHostPort.useMemo(() => {
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    if (color) return color;
    return foregroundColor;
  }, [color, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);

  const resolvedEmissiveColor = reactHostPort.useMemo(() => {
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    if (emissiveColor) return emissiveColor;
    return resolvedColor;
  }, [selected, hovered, activeBaseColor, hoverBaseColor, emissiveColor, resolvedColor]);
  const resolvedEdgeColor = reactHostPort.useMemo(() => {
    if (edgeColor) return edgeColor;
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    return foregroundColor;
  }, [edgeColor, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);
  const handlePointerEnter = reactHostPort.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isInteractive) {
        setIsPointerOver(true);
      }
      onPointerEnter?.(event);
    },
    [isInteractive, onPointerEnter],
  );

  const handlePointerLeave = reactHostPort.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isInteractive) {
        setIsPointerOver(false);
      }
      onPointerLeave?.(event);
    },
    [isInteractive, onPointerLeave],
  );

  reactHostPort.useEffect(() => {
    if (!isInteractive || !isPointerOver) return;
    selectableCursorUsageCount += 1;
    document.body.classList.add("cursor-selectable");
    return () => {
      selectableCursorUsageCount = Math.max(0, selectableCursorUsageCount - 1);
      if (selectableCursorUsageCount === 0) {
        document.body.classList.remove("cursor-selectable");
      }
    };
  }, [isInteractive, isPointerOver]);

  return (
    <group userData={userData} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave}>
      {children ? (
        children
      ) : (
        <mesh>
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial color={resolvedColor} emissive={resolvedEmissiveColor} emissiveIntensity={emissiveIntensity} />
          {showEdges && <Edges scale={1.001} color={resolvedEdgeColor} />}
        </mesh>
      )}
    </group>
  );
};

/**
 * GltfProps holds the data fields for a GltfProps record.
 **/
interface GltfProps {
  src: string;
  roughness?: number;
  metalness?: number;
}

/**
 * getComputedColorForGltf holds the data fields for a getComputedColorForGltf record.
 **/
const getComputedColorForGltf = (variable: string): string => {
  const cached = _elementsComputedColorCache.get(variable);
  if (cached !== undefined) return cached;
  const value = getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  _elementsComputedColorCache.set(variable, value);
  return value;
};

/**
 * Gltf holds the data fields for a Gltf record.
 **/
const Gltf: React.FC<GltfProps> = ({ src, roughness = 0.8, metalness = 0 }) => {
  const { scene } = useGLTF(src);
  const plasterColor = reactHostPort.useMemo(() => new THREE.Color(getComputedColorForGltf("--plaster")), []);
  const plasterEdgeColor = reactHostPort.useMemo(() => new THREE.Color(getComputedColorForGltf("--plaster-edge")), []);

  const clonedScene = reactHostPort.useMemo(() => {
    const cloned = scene.clone();
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness,
      roughness,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if ((child as any).isMesh) {
        (child as any).raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray((child as any).material)) {
          (child as any).material = (child as any).material.map(() => plasterMaterial.clone());
        } else {
          (child as any).material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [scene, plasterColor, plasterEdgeColor, roughness, metalness]);

  return <primitive object={clonedScene} />;
};

/**
 * GeometryFileProps holds the data fields for a GeometryFileProps record.
 **/
interface GeometryFileProps {
  src: string;
  environment?: string;
  roughness?: number;
  metalness?: number;
}
/** GeometryFile holds the data fields for a GeometryFile record.
 **/
/**
 **/
const GeometryFile: React.FC<GeometryFileProps> = ({ src, environment, roughness, metalness }) => {
  return (
    <div className="w-full h-full">
      <Geometry>
        <reactHostPort.Suspense fallback={null}>
          <Gltf src={src} roughness={roughness} metalness={metalness} />
        </reactHostPort.Suspense>
      </Geometry>
    </div>
  );
};

/**
 * GizmoProps holds the data fields for a GizmoProps record.
 **/
interface GizmoProps {
  show?: boolean;
  onAxisClick?: (direction: THREE.Vector3) => void;
}

type SceneProjectionKind = "camera" | "orthographic";

type SceneSnapViewKind = "front" | "back" | "side" | "opposite-side" | "top" | "bottom";

interface SceneGizmoSnapTarget {
  axis: "x" | "y" | "z";
  sign: 1 | -1;
  view: SceneSnapViewKind;
  cameraDirection: {
    x: number;
    y: number;
    z: number;
  };
  up: {
    x: number;
    y: number;
    z: number;
  };
}

interface SceneGizmoViewportPlacement {
  alignment: "top-left" | "top-right" | "bottom-left" | "bottom-right";
  margin: [number, number];
}

/**
 * resolveSceneGizmoSnapTarget holds the data fields for a resolveSceneGizmoSnapTarget record.
 **/
export const resolveSceneGizmoSnapTarget = (direction: Pick<THREE.Vector3, "x" | "y" | "z">): SceneGizmoSnapTarget => {
  const dominantAxis = [
    { axis: "x" as const, magnitude: Math.abs(direction.x), raw: direction.x },
    { axis: "y" as const, magnitude: Math.abs(direction.y), raw: direction.y },
    { axis: "z" as const, magnitude: Math.abs(direction.z), raw: direction.z },
  ].sort((a, b) => b.magnitude - a.magnitude)[0] ?? { axis: "x" as const, magnitude: 1, raw: 1 };
  const sign = dominantAxis.raw >= 0 ? 1 : -1;

  if (dominantAxis.axis === "x") {
    return {
      axis: "x",
      sign,
      view: sign > 0 ? "side" : "opposite-side",
      cameraDirection: { x: sign, y: 0, z: 0 },
      up: { x: 0, y: 1, z: 0 },
    };
  }

  if (dominantAxis.axis === "y") {
    return {
      axis: "y",
      sign,
      view: sign > 0 ? "top" : "bottom",
      cameraDirection: { x: 0, y: sign, z: 0 },
      up: sign > 0 ? { x: 0, y: 0, z: -1 } : { x: 0, y: 0, z: 1 },
    };
  }

  return {
    axis: "z",
    sign,
    view: sign > 0 ? "front" : "back",
    cameraDirection: { x: 0, y: 0, z: sign },
    up: { x: 0, y: 1, z: 0 },
  };
};

/**
 * resolveSceneGizmoViewportPlacement holds the data fields for a resolveSceneGizmoViewportPlacement record.
 **/
export const resolveSceneGizmoViewportPlacement = (viewport: { width: number; height: number }): SceneGizmoViewportPlacement => {
  const clampHorizontalMargin = (width: number): number => Math.min(56, Math.max(26, Math.floor(width / 5)));
  const clampVerticalMargin = (height: number): number => Math.min(40, Math.max(18, Math.floor(height / 7)));
  return {
    alignment: "bottom-right",
    margin: [clampHorizontalMargin(viewport.width), clampVerticalMargin(viewport.height)],
  };
};

const updateSceneCameraProjection = (camera: THREE.Camera): void => {
  if (camera instanceof THREE.OrthographicCamera || camera instanceof THREE.PerspectiveCamera) {
    camera.updateProjectionMatrix();
  }
};

/**
 * Gizmo holds the data fields for a Gizmo record.
 **/
const Gizmo: React.FC<GizmoProps> = ({ show = true, onAxisClick }) => {
  const { size } = useThree();
  const [colors, setColors] = reactHostPort.useState<[string, string, string]>(() => [getComputedColor("--accent"), getComputedColor("--accent-tertiary"), getComputedColor("--accent-secondary")]);
  const labels = reactHostPort.useMemo(() => ["X", "Z", "-Y"] as [string, string, string], []);
  const placement = reactHostPort.useMemo(() => resolveSceneGizmoViewportPlacement(size), [size]);
  // GizmoViewport axis box uses boxGeometry args [length, thickness, thickness]; uniform scale yields a chunky cube.
  const axisScale = reactHostPort.useMemo(() => [0.88, 0.036, 0.036] as [number, number, number], []);
  const labelColor = reactHostPort.useMemo(() => getComputedColor("--foreground"), []);

  reactHostPort.useEffect(() => {
    const updateColors = () => setColors([getComputedColor("--accent"), getComputedColor("--accent-tertiary"), getComputedColor("--accent-secondary")]);
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });
    return () => observer.disconnect();
  }, []);

  if (!show) return null;
  return (
    <GizmoHelper alignment={placement.alignment} margin={placement.margin}>
      <GizmoViewport
        labels={labels}
        axisColors={colors}
        axisScale={axisScale}
        axisHeadScale={0.92}
        hideNegativeAxes
        labelColor={labelColor}
        font="16px Inter var, Arial, sans-serif"
        onClick={
          onAxisClick
            ? (e: ThreeEvent<MouseEvent>) => {
                onAxisClick(e.object.position.clone());
                return null;
              }
            : undefined
        }
      />
    </GizmoHelper>
  );
};

/**
 * SceneInnerProps holds the data fields for a SceneInnerProps record.
 **/
interface SceneInnerProps {
  children?: React.ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  projection: SceneProjectionKind;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  onProjectionChange?: (projection: SceneProjectionKind) => void;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  selectionOnDrag?: boolean;
  onOrbitEnd?: () => void;
}

/**
 * SceneInner holds the data fields for a SceneInner record.
 **/
const SceneInner: React.FC<SceneInnerProps> = ({ children, showGrid = true, showGizmo = true, projection, camera: initialCamera, onCameraChange, onProjectionChange, focusedItemId, onFocusComplete, selectionOnDrag = false, onOrbitEnd }) => {
  const [gridColors, setGridColors] = reactHostPort.useState({
    sectionColor: getComputedColor("--foreground"),
    cellColor: getComputedColor("--accent-foreground"),
  });

  reactHostPort.useEffect(() => {
    const updateColors = () =>
      setGridColors({
        sectionColor: getComputedColor("--foreground"),
        cellColor: getComputedColor("--accent-foreground"),
      });
    updateColors();
    const observer = new MutationObserver(updateColors);
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });
    return () => observer.disconnect();
  }, []);

  const { camera: threeCamera, gl, size, scene: threeScene } = useThree();
  const controlsRef = reactHostPort.useRef<any>(null);
  const isUpdatingCameraRef = reactHostPort.useRef(false);
  const prevCameraStringRef = reactHostPort.useRef<string | undefined>(initialCamera ? JSON.stringify(initialCamera) : undefined);
  const cameraRestoredRef = reactHostPort.useRef(false);
  const restoredCameraStringRef = reactHostPort.useRef<string | undefined>(undefined);
  const previousProjectionRef = reactHostPort.useRef<SceneProjectionKind>(projection);
  const cameraRef = reactHostPort.useRef<THREE.Camera>(threeCamera as THREE.Camera);
  const [pendingSnapTarget, setPendingSnapTarget] = reactHostPort.useState<SceneGizmoSnapTarget | null>(null);

  reactHostPort.useEffect(() => {
    cameraRef.current = threeCamera as THREE.Camera;
    const currentCamera = cameraRef.current;
    if (projection === "orthographic" && currentCamera instanceof THREE.OrthographicCamera) {
      currentCamera.zoom = 50;
    }
    updateSceneCameraProjection(currentCamera);
  }, [projection, threeCamera]);

  const emitCameraChange = reactHostPort.useCallback(() => {
    if (!cameraRef.current || !controlsRef.current || !onCameraChange) return;
    const position = cameraRef.current.position;
    const target = controlsRef.current.target;
    const forwardVector = new THREE.Vector3().subVectors(target, position);
    if (forwardVector.lengthSq() < 0.001) return;
    const forward = forwardVector.normalize();
    const up = cameraRef.current.up.clone().normalize();
    onCameraChange({
      position: { x: position.x, y: position.y, z: position.z },
      forward: { x: forward.x, y: forward.y, z: forward.z },
      up: { x: up.x, y: up.y, z: up.z },
    });
  }, [onCameraChange]);

  reactHostPort.useEffect(() => {
    if (!cameraRef.current || !controlsRef.current) return;

    const currentCameraString = initialCamera ? JSON.stringify(initialCamera) : undefined;

    if (previousProjectionRef.current !== projection) {
      previousProjectionRef.current = projection;
      cameraRestoredRef.current = false;
      restoredCameraStringRef.current = undefined;
    }

    if (prevCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
      prevCameraStringRef.current = currentCameraString;
    }
    if (restoredCameraStringRef.current !== currentCameraString) {
      cameraRestoredRef.current = false;
    }

    if (cameraRestoredRef.current) return;

    isUpdatingCameraRef.current = true;

    if (initialCamera) {
      const forwardLength = Math.sqrt(initialCamera.forward.x * initialCamera.forward.x + initialCamera.forward.y * initialCamera.forward.y + initialCamera.forward.z * initialCamera.forward.z);

      if (forwardLength < 0.01) {
        cameraRestoredRef.current = true;
        isUpdatingCameraRef.current = false;
        return;
      }

      requestAnimationFrame(() => {
        if (!cameraRef.current || !controlsRef.current) return;

        cameraRef.current.position.set(initialCamera.position.x, initialCamera.position.y, initialCamera.position.z);
        cameraRef.current.up.set(initialCamera.up.x, initialCamera.up.y, initialCamera.up.z);
        const target = new THREE.Vector3(initialCamera.position.x + initialCamera.forward.x, initialCamera.position.y + initialCamera.forward.y, initialCamera.position.z + initialCamera.forward.z);
        controlsRef.current.target.copy(target);
        if (projection === "orthographic" && cameraRef.current instanceof THREE.OrthographicCamera) {
          cameraRef.current.zoom = 50;
        }
        updateSceneCameraProjection(cameraRef.current);
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    } else {
      requestAnimationFrame(() => {
        if (!cameraRef.current || !controlsRef.current) return;

        cameraRef.current.position.set(10, 10, 10);
        cameraRef.current.up.set(0, 1, 0);
        controlsRef.current.target.set(0, 0, 0);
        if (projection === "orthographic" && cameraRef.current instanceof THREE.OrthographicCamera) {
          cameraRef.current.zoom = 50;
        }
        updateSceneCameraProjection(cameraRef.current);
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    }
  }, [initialCamera, projection]);

  reactHostPort.useEffect(() => {
    if (!pendingSnapTarget || !cameraRef.current || !controlsRef.current) return;

    const currentCamera = cameraRef.current;
    const controls = controlsRef.current;
    const currentTarget = controls.target.clone();
    const currentPosition = currentCamera.position.clone();
    const currentUp = currentCamera.up.clone().normalize();
    const nextDirection = new THREE.Vector3(pendingSnapTarget.cameraDirection.x, pendingSnapTarget.cameraDirection.y, pendingSnapTarget.cameraDirection.z).normalize();
    const nextUp = new THREE.Vector3(pendingSnapTarget.up.x, pendingSnapTarget.up.y, pendingSnapTarget.up.z).normalize();
    const nextPosition = currentTarget.clone().add(nextDirection.multiplyScalar(Math.max(currentPosition.distanceTo(currentTarget), 1)));
    const animationDurationMs = 280;

    isUpdatingCameraRef.current = true;

    const animateSnap = (startTime: number) => {
      const frame = (now: number) => {
        if (!cameraRef.current || !controlsRef.current) {
          setPendingSnapTarget(null);
          isUpdatingCameraRef.current = false;
          return;
        }

        const progress = Math.min(1, (now - startTime) / animationDurationMs);
        const easedProgress = progress < 0.5 ? 4 * progress * progress * progress : 1 - Math.pow(-2 * progress + 2, 3) / 2;

        cameraRef.current.position.lerpVectors(currentPosition, nextPosition, easedProgress);
        cameraRef.current.up.lerpVectors(currentUp, nextUp, easedProgress).normalize();
        controlsRef.current.target.copy(currentTarget);

        if (projection === "orthographic" && cameraRef.current instanceof THREE.OrthographicCamera) {
          cameraRef.current.zoom = 50;
        }
        updateSceneCameraProjection(cameraRef.current);
        controlsRef.current.update();

        if (progress < 1) {
          requestAnimationFrame(frame);
          return;
        }

        emitCameraChange();
        onProjectionChange?.("orthographic");
        setPendingSnapTarget(null);
        isUpdatingCameraRef.current = false;
      };

      requestAnimationFrame(frame);
    };

    requestAnimationFrame(animateSnap);
  }, [emitCameraChange, onProjectionChange, pendingSnapTarget, projection]);

  const handleGizmoAxisClick = reactHostPort.useCallback((direction: THREE.Vector3) => {
    setPendingSnapTarget(resolveSceneGizmoSnapTarget(direction));
  }, []);

  const handleStart = reactHostPort.useCallback(() => {
    if (isUpdatingCameraRef.current || projection !== "orthographic") return;
    emitCameraChange();
    onProjectionChange?.("camera");
  }, [emitCameraChange, onProjectionChange, projection]);

  const handleEnd = reactHostPort.useCallback(() => {
    if (isUpdatingCameraRef.current) return;
    onOrbitEnd?.();
    emitCameraChange();
  }, [emitCameraChange, onOrbitEnd]);

  reactHostPort.useEffect(() => {
    if (!focusedItemId || !cameraRef.current || !controlsRef.current) return;

    let retryCount = 0;
    const maxRetries = 20;

    const findAndFocusObject = () => {
      if (!cameraRef.current || !controlsRef.current) return;

      let targetObject: THREE.Object3D | null = null;

      threeScene.traverse((obj: THREE.Object3D) => {
        if (obj.userData?.id === focusedItemId || obj.name === focusedItemId) {
          targetObject = obj;
        }
      });

      if (!targetObject) {
        retryCount++;
        if (retryCount < maxRetries) {
          setTimeout(findAndFocusObject, 50);
        } else {
          console.warn(`Focus: Object ${focusedItemId} not found after ${maxRetries} retries`);
          if (onFocusComplete) onFocusComplete();
        }
        return;
      }

      const box = new THREE.Box3().setFromObject(targetObject);
      const center = box.getCenter(new THREE.Vector3());
      const size = box.getSize(new THREE.Vector3());
      const maxDim = Math.max(size.x, size.y, size.z);
      const distance = maxDim * 2;

      const camera = cameraRef.current;
      const currentPos = camera.position.clone();
      const direction = new THREE.Vector3().subVectors(currentPos, controlsRef.current.target).normalize();
      const newPosition = center.clone().add(direction.multiplyScalar(distance));

      isUpdatingCameraRef.current = true;

      const animate = () => {
        if (!cameraRef.current || !controlsRef.current) return;

        const t = 0.1;
        camera.position.lerp(newPosition, t);
        controlsRef.current.target.lerp(center, t);
        updateSceneCameraProjection(camera);
        controlsRef.current.update();

        const distanceToTarget = camera.position.distanceTo(newPosition);
        const targetDistanceToCenter = controlsRef.current.target.distanceTo(center);

        if (distanceToTarget > 0.01 || targetDistanceToCenter > 0.01) {
          requestAnimationFrame(animate);
        } else {
          isUpdatingCameraRef.current = false;
          if (onFocusComplete) onFocusComplete();
        }
      };

      requestAnimationFrame(animate);
    };

    findAndFocusObject();
  }, [focusedItemId, threeScene, onFocusComplete]);

  return (
    <>
      <OrbitControls
        ref={controlsRef}
        enableDamping={false}
        onStart={handleStart}
        mouseButtons={
          selectionOnDrag
            ? {
                LEFT: undefined,
                MIDDLE: THREE.MOUSE.ROTATE,
                RIGHT: THREE.MOUSE.ROTATE,
              }
            : {
                LEFT: THREE.MOUSE.ROTATE,
                MIDDLE: undefined,
                RIGHT: undefined,
              }
        }
        onEnd={handleEnd}
      />
      <ambientLight intensity={1} />
      {children}
      {showGrid && <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />}
      {showGizmo && <Gizmo onAxisClick={handleGizmoAxisClick} />}
    </>
  );
};

/**
 * SceneProps holds the data fields for a SceneProps record.
 **/
interface SceneProps {
  children?: React.ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  onDoubleClickCapture?: (e: React.MouseEvent) => void;
  onPointerMissed?: (e: MouseEvent) => void;
  orthographic?: boolean;
  shadows?: boolean;
  className?: string;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  projection?: SceneProjectionKind;
  onProjectionChange?: (projection: SceneProjectionKind) => void;
  selectionOnDrag?: boolean;
}

/**
 * 3D scene viewer with orbit controls, grid, and geometry rendering.
 **/
export const Scene: React.FC<SceneProps> = ({
  children,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  onDoubleClickCapture,
  onPointerMissed,
  orthographic = false,
  shadows = false,
  className = "",
  focusedItemId,
  onFocusComplete,
  projection = "camera",
  onProjectionChange,
  selectionOnDrag = false,
}) => {
  const [resolvedProjection, setResolvedProjection] = reactHostPort.useState<SceneProjectionKind>(projection ?? (orthographic ? "orthographic" : "camera"));

  reactHostPort.useEffect(() => {
    setResolvedProjection(projection ?? (orthographic ? "orthographic" : "camera"));
  }, [orthographic, projection]);

  const handleProjectionChange = reactHostPort.useCallback(
    (nextProjection: SceneProjectionKind) => {
      setResolvedProjection(nextProjection);
      onProjectionChange?.(nextProjection);
    },
    [onProjectionChange],
  );

  const projectionOptions: ActionDropdownOption[] = [
    {
      value: "camera",
      icon: <CameraIcon className="size-3" />,
      label: "Perspective",
    },
    {
      value: "orthographic",
      icon: <GripVerticalIcon className="size-3" />,
      label: "Orthographic",
    },
  ];

  return (
    <div className={`relative h-full w-full ${className}`} style={{ minHeight: "100%", minWidth: "100%" }} onDoubleClick={onDoubleClickCapture}>
      <div className="absolute top-1 right-1 z-panel">
        <ActionDropdown id="scene-projection" options={projectionOptions} value={resolvedProjection} onValueChange={(value) => handleProjectionChange(value as SceneProjectionKind)} />
      </div>
      <HostThreeCanvas
        onPointerMissed={onPointerMissed}
        orthographic={resolvedProjection === "orthographic"}
        shadows={shadows}
        frameloop="demand"
        camera={resolvedProjection === "orthographic" ? { zoom: 50, position: [10, 10, 10], near: -10000, far: 10000 } : { fov: 75, position: [10, 10, 10], near: 0.1, far: 10000 }}
        style={{ width: "100%", height: "100%" }}
      >
        <SceneFrameControl />
        <SceneInner
          showGrid={showGrid}
          showGizmo={showGizmo}
          projection={resolvedProjection}
          camera={camera}
          onCameraChange={onCameraChange}
          onProjectionChange={handleProjectionChange}
          focusedItemId={focusedItemId}
          onFocusComplete={onFocusComplete}
          selectionOnDrag={selectionOnDrag}
        >
          {children}
        </SceneInner>
      </HostThreeCanvas>
    </div>
  );
};

/**
 * Skeleton loading placeholder for a 3D scene.
 *
 **/
export const SceneSkeleton: React.FC = () => (
  <div className="h-full w-full bg-background flex items-center justify-center">
    <div className="relative w-32 h-32 animate-pulse">
      <div className="absolute inset-0 border-4 border-muted-foreground/20 rounded-lg" />
      <div className="absolute inset-2 border-2 border-muted-foreground/20 rounded-lg" />
      <div className="absolute inset-4 border border-muted-foreground/20 rounded-lg" />
    </div>
  </div>
);

// #endregion 📍Scene

// #region 🛎️Table
// Sortable, hierarchical data table with drag-drop support.
// Consumers MUST provide columns and data arrays.

/**
 * Union type for ascending or descending sort order.
 **/
export type SortDirection = "asc" | "desc";

/**
 * Configuration interface for a table column definition.
 **/
export interface TableColumn<T = unknown> {
  id: string;
  header: React.ReactNode;
  accessor: (row: T) => React.ReactNode;
  width?: string;
  className?: string;
  headerClassName?: string;
  sortable?: boolean;
  visible?: boolean | ((data: T[]) => boolean);
}

/**
 * Interface for hierarchical row data with parent/child relations.
 **/
export interface HierarchicalRowData {
  id: string;
  level?: number;
  parentId?: string;
  hasChildren?: boolean;
  isExpanded?: boolean;
}

/**
 * Configuration interface for table drag-and-drop behavior.
 **/
export interface DragDropConfig {
  enabled?: boolean;
  onDragStart?: (rowId: string) => void;
  onDragEnd?: (event: { active: string; over: string | null }) => void;
  canDrag?: (rowId: string) => boolean;
  canDrop?: (draggedId: string, targetId: string) => boolean;
  renderDragOverlay?: (rowId: string) => React.ReactNode;
}

/**
 * Props interface for the Table component.
 **/
export interface TableProps<T = unknown> {
  columns: TableColumn<T>[];
  data: T[];
  onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowDoubleClick?: (row: T, index: number) => void;
  onRowMouseEnter?: (row: T, index: number) => void;
  onRowMouseLeave?: (row: T, index: number) => void;
  rowClassName?: (row: T, index: number) => string;
  rowKey?: (row: T, index: number) => string;
  emptyMessage?: string;
  className?: string;
  sortColumn?: string;
  sortDirection?: SortDirection;
  onSort?: (columnId: string, direction: SortDirection) => void;
  selectedRows?: Set<string> | string[];
  getRowId?: (row: T) => string;
  stickyHeader?: boolean;
  headerClassName?: string;
  rowHeight?: "compact" | "normal" | "comfortable";
  focusedItemId?: string;
  onFocusComplete?: () => void;
  renderMobileRow?: (row: T, index: number, isSelected: boolean, onClick: (e: React.MouseEvent) => void, onDoubleClick: () => void) => React.ReactNode;
  isMobile?: boolean;
  hierarchical?: boolean;
  onToggleRow?: (rowId: string) => void;
  renderHierarchyControls?: (row: T & HierarchicalRowData) => React.ReactNode;
  dragDrop?: DragDropConfig;
  wrapperComponent?: React.ComponentType<{ children: React.ReactNode }>;
}

/**
 * Table holds the data fields for a Table record.
 **/
const Table = <T,>({
  columns,
  data,
  onRowClick,
  onRowDoubleClick,
  onRowMouseEnter,
  onRowMouseLeave,
  rowClassName,
  rowKey,
  emptyMessage = "No data",
  className = "",
  sortColumn,
  sortDirection,
  onSort,
  selectedRows,
  getRowId,
  stickyHeader = true,
  headerClassName = "",
  rowHeight = "normal",
  focusedItemId,
  onFocusComplete,
  renderMobileRow,
  isMobile = false,
  hierarchical = false,
  onToggleRow,
  renderHierarchyControls,
  dragDrop,
  wrapperComponent: WrapperComponent,
}: TableProps<T>) => {
  const selectedSet = selectedRows instanceof Set ? selectedRows : new Set(selectedRows || []);
  const scrollAreaRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [activeId, setActiveId] = reactHostPort.useState<string | null>(null);
  const level = useLevel();
  const headerBgClass = {
    base: "bg-base",
    canvas: "bg-canvas",
    window: "bg-window",
    panel: "bg-panel",
    overlay: "bg-overlay",
    temporary: "bg-temporary",
  }[level];

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    }),
  );

  reactHostPort.useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const rowElements = scrollAreaRef.current.querySelectorAll(isMobile ? "[data-row]" : "tbody tr");
      let focusedIndex = -1;

      data.forEach((row, index) => {
        const rowId = getRowId ? getRowId(row) : rowKey ? rowKey(row, index) : index.toString();
        if (rowId === focusedItemId) {
          focusedIndex = index;
        }
      });

      if (focusedIndex >= 0 && rowElements[focusedIndex]) {
        rowElements[focusedIndex].scrollIntoView({ behavior: "smooth", block: "center" });
        if (onFocusComplete) {
          setTimeout(() => onFocusComplete(), 600);
        }
      }
    }
  }, [focusedItemId, data, getRowId, rowKey, onFocusComplete, isMobile]);

  const rowHeightClass = {
    compact: "h-medium",
    normal: "h-medium",
    comfortable: "h-medium",
  }[rowHeight];

  const visibleColumns = columns.filter((col) => {
    if (col.visible === undefined) return true;
    if (typeof col.visible === "boolean") return col.visible;
    return col.visible(data);
  });

  const handleDragStart = (event: any) => {
    const id = event.active.id;
    setActiveId(id);
    dragDrop?.onDragStart?.(id);
  };

  const handleDragEnd = (event: any) => {
    const { active, over } = event;
    setActiveId(null);
    if (dragDrop?.onDragEnd) {
      dragDrop.onDragEnd({ active: active.id, over: over?.id || null });
    }
  };

  const DraggableRow = ({ row, rowId, index, isSelected, customRowClassName }: { row: T; rowId: string; index: number; isSelected: boolean; customRowClassName: string }) => {
    const canDragRow = !dragDrop?.canDrag || dragDrop.canDrag(rowId);
    const {
      attributes,
      listeners,
      setNodeRef: setDraggableRef,
      transform,
      isDragging: isDraggingHook,
    } = useDraggable({
      id: rowId,
      disabled: !canDragRow,
      data: { row },
    });
    const { setNodeRef: setDroppableRef, isOver } = useDroppable({
      id: rowId,
      data: { row },
    });

    const style = transform ? { transform: `translate3d(${transform.x}px, ${transform.y}px, 0)` } : undefined;

    const combinedRef = (node: HTMLElement | null) => {
      setDraggableRef(node);
      setDroppableRef(node);
    };

    const baseRowClassName = `border-b border-element ${rowHeightClass} ${isSelected ? "bg-active-base text-active-foreground" : isOver ? "bg-hover-base ring-2 ring-active" : "hover:bg-hover-base"}`;
    const isDragging = activeId === rowId || isDraggingHook;

    return (
      <tr
        ref={combinedRef}
        style={style}
        className={`${baseRowClassName} ${customRowClassName} ${isDragging ? "opacity-50" : ""} ${onRowClick ? "cursor-selectable" : ""}`}
        {...(canDragRow ? { ...attributes, ...listeners } : {})}
        onClick={(e) => onRowClick?.(row, index, e)}
        onDoubleClick={() => onRowDoubleClick?.(row, index)}
        onMouseEnter={() => onRowMouseEnter?.(row, index)}
        onMouseLeave={() => onRowMouseLeave?.(row, index)}
        role={onRowClick ? "button" : undefined}
        tabIndex={onRowClick ? 0 : undefined}
        data-row-id={rowId}
      >
        {visibleColumns.map((column) => (
          <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
            <div className="flex items-center h-full min-w-0">{column.accessor(row)}</div>
          </td>
        ))}
      </tr>
    );
  };

  const renderTableContent = () => {
    if (isMobile && renderMobileRow) {
      return (
        <Scrollable ref={scrollAreaRef} className={`h-full w-full ${className}`}>
          <div className="flex flex-col">
            {data.length === 0 ? (
              <div className="p-small text-center text-muted-foreground">{emptyMessage}</div>
            ) : (
              data.map((row, index) => {
                const key = rowKey ? rowKey(row, index) : index.toString();
                const rowId = getRowId ? getRowId(row) : key;
                const isSelected = selectedSet.has(rowId);
                return (
                  <div key={key} data-row onMouseEnter={() => onRowMouseEnter?.(row, index)} onMouseLeave={() => onRowMouseLeave?.(row, index)}>
                    {renderMobileRow(
                      row,
                      index,
                      isSelected,
                      (e) => onRowClick?.(row, index, e),
                      () => onRowDoubleClick?.(row, index),
                    )}
                  </div>
                );
              })
            )}
          </div>
        </Scrollable>
      );
    }

    return (
      <Scrollable ref={scrollAreaRef} className={`h-full w-full ${className}`}>
        <table className="w-full border-collapse">
          <thead className={`${headerBgClass} border-b border-element ${stickyHeader ? "sticky top-0 z-panel" : ""} ${headerClassName}`}>
            <tr className="h-large">
              {visibleColumns.map((column) => (
                <th key={column.id} className={`text-left p-single font-medium h-large ${column.headerClassName || column.className || ""}`} style={{ width: column.width }}>
                  {column.header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {data.length === 0 ? (
              <tr>
                <td colSpan={visibleColumns.length} className="p-small text-center text-muted-foreground">
                  {emptyMessage}
                </td>
              </tr>
            ) : (
              data.map((row, index) => {
                const key = rowKey ? rowKey(row, index) : index.toString();
                const rowId = getRowId ? getRowId(row) : key;
                const isSelected = selectedSet.has(rowId);
                const customRowClassName = rowClassName ? rowClassName(row, index) : "";

                if (dragDrop?.enabled) {
                  return <DraggableRow key={key} row={row} rowId={rowId} index={index} isSelected={isSelected} customRowClassName={customRowClassName} />;
                }

                const baseRowClassName = `border-b border-element ${rowHeightClass} ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`;
                const isDragging = activeId === rowId;

                return (
                  <tr
                    key={key}
                    className={`${baseRowClassName} ${customRowClassName} ${isDragging ? "opacity-50" : ""} ${onRowClick ? "cursor-selectable" : ""}`}
                    onClick={(e) => onRowClick?.(row, index, e)}
                    onDoubleClick={() => onRowDoubleClick?.(row, index)}
                    onMouseEnter={() => onRowMouseEnter?.(row, index)}
                    onMouseLeave={() => onRowMouseLeave?.(row, index)}
                    role={onRowClick ? "button" : undefined}
                    tabIndex={onRowClick ? 0 : undefined}
                    data-row-id={rowId}
                  >
                    {visibleColumns.map((column) => (
                      <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
                        <div className="flex items-center h-full min-w-0">{column.accessor(row)}</div>
                      </td>
                    ))}
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </Scrollable>
    );
  };

  const content = renderTableContent();

  if (dragDrop?.enabled) {
    return (
      <DndContext sensors={sensors} collisionDetection={closestCenter} onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
        {WrapperComponent ? <WrapperComponent>{content}</WrapperComponent> : content}
      </DndContext>
    );
  }

  return WrapperComponent ? <WrapperComponent>{content}</WrapperComponent> : content;
};

export { Table };

/**
 * Props interface for the TableSkeleton component.
 **/
export interface TableSkeletonProps {
  columns: TableColumn[];
  rowCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a table.
 **/
export const TableSkeleton: React.FC<TableSkeletonProps> = ({ columns, rowCount = 5, className = "" }) => (
  <Scrollable className={`h-full w-full ${className}`}>
    <table className="w-full border-collapse">
      <thead className="bg-window border-b border-element sticky top-0 z-panel">
        <tr className="h-large">
          {columns.map((column) => (
            <th key={column.id} className={`text-left p-single text-sm font-medium h-large ${column.className || ""}`} style={{ width: column.width }}>
              {column.header}
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        {Array.from({ length: rowCount }).map((_, index) => (
          <tr key={index} className="border-b border-element h-medium">
            {columns.map((column) => (
              <td key={column.id} className={`h-medium px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
                <div className="flex items-center h-full min-w-0">
                  <div className="h-small bg-muted-foreground/20 rounded animate-pulse w-full" />
                </div>
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  </Scrollable>
);

// #endregion 🛎️Table

// #region ⚙️Canvas

/**
 * Container component for canvas window layout.
 **/
export const Canvas: React.FC<{ children: React.ReactNode; id?: string }> = ({ children, id }) => {
  return (
    <LevelProvider level="canvas">
      <div id={id} data-slot="canvas" className="box-border h-full w-full bg-canvas p-single">
        {children}
      </div>
    </LevelProvider>
  );
};

/**
 * Layout component arranging windows horizontally.
 **/
export const HorizontalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-row h-full w-full gap-single">{children}</div>;
};

/**
 * Layout component arranging windows vertically.
 **/
export const VerticalWindows: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="flex flex-col h-full w-full gap-single">{children}</div>;
};

// #region 🧭Mode

/** @emoji 🪟 Window descriptor rendered inside {@link Mode}. */
export interface ModeWindowDescriptor extends Omit<WindowConfig, "children" | "onOpenInNewWindow" | "onMaximize" | "onMinimize" | "onClose"> {
  title?: string;
  children: React.ReactNode;
}

export interface ModeProps {
  windows: ModeWindowDescriptor[];
  activeWindowId: string | null;
  onActiveWindowChange?: (windowId: string) => void;
  layout?: WindowLayoutNode;
  children?: React.ReactNode;
  className?: string;
}

//#region 🧭ModeCanvasSpacing

/** @emoji 📐 Canvas inset on {@link Mode} body; inter-panel splitters use the same spacing step. */
const MODE_CANVAS_INSET_CLASS = "p-double";

function modeCanvasSplitterClass(orientation: "horizontal" | "vertical"): string {
  return orientation === "horizontal" ? "h-full min-h-0 w-double shrink-0" : "w-full min-h-0 h-double shrink-0";
}

//#endregion 🧭ModeCanvasSpacing

//#region 🧭ModeLayoutUtils

type ModeLayoutPath = string;
type ModeDockSide = "left" | "right" | "top" | "bottom";

function modePathSegments(path: ModeLayoutPath): number[] {
  return path ? path.split(".").map((segment) => Number(segment)) : [];
}

function modeJoinPath(parent: ModeLayoutPath, index: number): ModeLayoutPath {
  return parent ? `${parent}.${index}` : String(index);
}

function modeCollectWindowIds(node: WindowLayoutNode): string[] {
  if (node.kind === "window") return [node.id];
  if (node.kind === "stack") return node.children.map((child) => child.id);
  return node.children.flatMap(modeCollectWindowIds);
}

/** @emoji 🪟 Ensures every window leaf sits inside a tab stack. */
function normalizeLayoutToStacks(node: WindowLayoutNode): WindowLayoutNode {
  if (node.kind === "window") return { kind: "stack", children: [node], activeId: node.id };
  if (node.kind === "stack") return { ...node, activeId: node.activeId ?? node.children[0]?.id };
  return { ...node, children: node.children.map((child) => normalizeLayoutToStacks(child) as WindowLayoutAxisNode | WindowLayoutStackNode) };
}

/** @emoji 🪟 Collapses empty axes and hoists single-child axes. */
function collapseLayout(node: WindowLayoutNode | null): WindowLayoutNode | null {
  if (!node) return null;
  if (node.kind === "window") return node;
  if (node.kind === "stack") return node.children.length === 0 ? null : node;
  const children = node.children.map((child) => collapseLayout(child)).filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => child !== null);
  if (children.length === 0) return null;
  if (children.length === 1) {
    const only = children[0]!;
    return { ...only, size: only.size ?? node.size };
  }
  return { ...node, children };
}

function updateLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath, updater: (node: WindowLayoutNode) => WindowLayoutNode): WindowLayoutNode {
  if (!path) return updater(layout);
  const [head, ...rest] = modePathSegments(path);
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return updater(layout);
  const child = layout.children[head!];
  if (!child) return layout;
  if (rest.length === 0) {
    const nextChildren = [...layout.children];
    nextChildren[head!] = updater(child as WindowLayoutNode) as WindowLayoutAxisNode | WindowLayoutStackNode;
    return { ...layout, children: nextChildren };
  }
  const nextChildren = [...layout.children];
  nextChildren[head!] = updateLayoutAtPath(child as WindowLayoutNode, rest.join("."), updater) as WindowLayoutAxisNode | WindowLayoutStackNode;
  return { ...layout, children: nextChildren };
}

function readLayoutAtPath(layout: WindowLayoutNode, path: ModeLayoutPath): WindowLayoutNode | null {
  if (!path) return layout;
  const [head, ...rest] = modePathSegments(path);
  if (layout.kind === "window") return null;
  if (layout.kind === "stack") return layout;
  const child = layout.children[head!];
  if (!child) return null;
  if (rest.length === 0) return child as WindowLayoutNode;
  return readLayoutAtPath(child as WindowLayoutNode, rest.join("."));
}

function mapLayoutStacks(layout: WindowLayoutNode, mapper: (stack: WindowLayoutStackNode, path: ModeLayoutPath) => WindowLayoutStackNode, path = ""): WindowLayoutNode {
  if (layout.kind === "window") return layout;
  if (layout.kind === "stack") return mapper(layout, path);
  return {
    ...layout,
    children: layout.children.map((child, index) => mapLayoutStacks(child as WindowLayoutNode, mapper, modeJoinPath(path, index)) as WindowLayoutAxisNode | WindowLayoutStackNode),
  };
}

/** @emoji 🪟 Adds missing windows and removes stale ones from the layout tree. */
function reconcileWindows(layout: WindowLayoutNode, windowIds: readonly string[]): WindowLayoutNode {
  const normalized = normalizeLayoutToStacks(layout);
  const allowed = new Set(windowIds);
  let result = windowIds.length === 0 ? normalized : removeAbsentWindowsFromLayout(normalized, allowed);
  const present = new Set(modeCollectWindowIds(result));
  const missing = windowIds.filter((id) => !present.has(id));
  if (missing.length === 0) return collapseLayout(result) ?? { kind: "stack", children: [] };
  const newStacks: WindowLayoutStackNode[] = missing.map((id) => ({ kind: "stack", children: [{ kind: "window", id }], activeId: id }));
  if (result.kind === "row" || result.kind === "column") {
    result = { ...result, children: [...result.children, ...newStacks] };
  } else if (result.kind === "stack") {
    if (modeCollectWindowIds(result).length === 0) result = newStacks[0]!;
    else result = { kind: "row", children: [result, ...newStacks] };
  } else {
    result = newStacks[0]!;
  }
  return collapseLayout(result) ?? { kind: "stack", children: [] };
}

function removeAbsentWindowsFromLayout(layout: WindowLayoutNode, allowed: ReadonlySet<string>): WindowLayoutNode {
  if (layout.kind === "window") return allowed.has(layout.id) ? layout : { kind: "stack", children: [] };
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => allowed.has(child.id));
    const activeId = layout.activeId && allowed.has(layout.activeId) ? layout.activeId : children[0]?.id;
    return { ...layout, children, activeId };
  }
  return {
    ...layout,
    children: layout.children
      .map((child) => removeAbsentWindowsFromLayout(child as WindowLayoutNode, allowed))
      .filter((child) => child.kind !== "stack" || child.children.length > 0) as (WindowLayoutAxisNode | WindowLayoutStackNode)[],
  };
}

/** @emoji 🪟 Removes a window from the layout tree and collapses empty nodes. */
function removeWindowFromLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode | null {
  if (layout.kind === "window") return layout.id === windowId ? null : layout;
  if (layout.kind === "stack") {
    const children = layout.children.filter((child) => child.id !== windowId);
    if (children.length === 0) return null;
    const activeId = layout.activeId === windowId ? children[0]?.id : layout.activeId;
    return { ...layout, children, activeId };
  }
  const children = layout.children
    .map((child) => removeWindowFromLayout(child as WindowLayoutNode, windowId))
    .filter((child): child is WindowLayoutAxisNode | WindowLayoutStackNode => child !== null);
  if (children.length === 0) return null;
  return collapseLayout({ ...layout, children });
}

function insertWindowAsTab(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, index?: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const insertAt = index === undefined || index < 0 ? children.length : index;
    children.splice(insertAt, 0, { kind: "window", id: windowId });
    return { ...node, children, activeId: windowId };
  });
}

function reorderTabInStack(layout: WindowLayoutNode, stackPath: ModeLayoutPath, fromIndex: number, toIndex: number): WindowLayoutNode {
  return updateLayoutAtPath(layout, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const children = [...node.children];
    const [item] = children.splice(fromIndex, 1);
    if (!item) return node;
    children.splice(toIndex, 0, item);
    return { ...node, children };
  });
}

/** @emoji 🪟 Splits a stack with a dragged window on the given side. */
function splitWithWindow(layout: WindowLayoutNode, stackPath: ModeLayoutPath, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  return updateLayoutAtPath(without, stackPath, (node) => {
    if (node.kind !== "stack") return node;
    const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
    const horizontal = side === "left" || side === "right";
    const children = side === "left" || side === "top" ? [incoming, node] : [node, incoming];
    return { kind: horizontal ? "row" : "column", children, size: node.size } as WindowLayoutAxisNode;
  });
}

function splitRootWithWindow(layout: WindowLayoutNode, windowId: string, side: ModeDockSide): WindowLayoutNode {
  const without = removeWindowFromLayout(layout, windowId) ?? { kind: "stack", children: [] };
  const incoming: WindowLayoutStackNode = { kind: "stack", children: [{ kind: "window", id: windowId }], activeId: windowId };
  const horizontal = side === "left" || side === "right";
  const children = side === "left" || side === "top" ? [incoming, without] : [without, incoming];
  return { kind: horizontal ? "row" : "column", children: children as (WindowLayoutAxisNode | WindowLayoutStackNode)[] };
}

/** @emoji 🪟 Writes resizable panel percentages back onto axis children. */
function applyAxisSizes(layout: WindowLayoutNode, axisPath: ModeLayoutPath, sizes: Record<string, number>): WindowLayoutNode {
  return updateLayoutAtPath(layout, axisPath, (node) => {
    if (node.kind !== "row" && node.kind !== "column") return node;
    const children = node.children.map((child, index) => {
      const panelKey = modeJoinPath(axisPath, index);
      const size = sizes[panelKey] ?? sizes[String(index)] ?? child.size;
      return { ...child, size };
    });
    return { ...node, children };
  });
}

function setActiveWindowInLayout(layout: WindowLayoutNode, windowId: string): WindowLayoutNode {
  return mapLayoutStacks(layout, (stack) => {
    if (!stack.children.some((child) => child.id === windowId)) return stack;
    if (stack.activeId === windowId) return stack;
    return { ...stack, activeId: windowId };
  });
}

function resolveModeLayout(windows: readonly ModeWindowDescriptor[], layout?: WindowLayoutNode): WindowLayoutNode {
  const base = layout ?? createEvenWindowLayout(windows.map((window) => window.id));
  return reconcileWindows(base, windows.map((window) => window.id));
}

//#endregion 🧭ModeLayoutUtils

//#region 🧭ModeDockDrag

type ModeDropZone =
  | { kind: "tab"; stackPath: ModeLayoutPath; index: number }
  | { kind: "split"; stackPath: ModeLayoutPath; side: ModeDockSide }
  | { kind: "root-split"; side: ModeDockSide };

interface ModeDragState {
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  x: number;
  y: number;
}

interface ModePendingDrag {
  windowId: string;
  stackPath: ModeLayoutPath;
  tabIndex: number;
  pointerId: number;
  ghostLabel: string;
  startX: number;
  startY: number;
}

interface ModeStackDropTargets {
  tabBar: DOMRect | null;
  body: DOMRect | null;
  tabBarElement: HTMLElement | null;
}

function listModeDockTabElements(tabBarElement: HTMLElement | null): HTMLElement[] {
  if (!tabBarElement) return [];
  return [...tabBarElement.querySelectorAll('[data-slot="mode-dock-tab"]')].filter((tab) => tab.getAttribute("data-drag-source") !== "true");
}

function computeTabInsertIndex(pointerX: number, tabBarElement: HTMLElement | null): number {
  const tabs = listModeDockTabElements(tabBarElement);
  for (let index = 0; index < tabs.length; index++) {
    const tabRect = tabs[index]!.getBoundingClientRect();
    if (pointerX < tabRect.left + tabRect.width / 2) return index;
  }
  return tabs.length;
}

/** @emoji 📍 Resolves tab-bar insertion line and slot preview geometry for drag feedback. */
function computeTabInsertPreview(tabBarElement: HTMLElement | null, insertIndex: number): { insertX: number; top: number; height: number; slotLeft: number; slotWidth: number } | null {
  if (!tabBarElement) return null;
  const tabBarRect = tabBarElement.getBoundingClientRect();
  const tabRects = listModeDockTabElements(tabBarElement).map((tab) => tab.getBoundingClientRect());
  const top = tabBarRect.top;
  const height = tabBarRect.height;
  const defaultWidth = tabRects[0]?.width ?? 96;
  const resolvedIndex = insertIndex < 0 ? tabRects.length : insertIndex;

  if (tabRects.length === 0) {
    return { insertX: tabBarRect.left + 6, top, height, slotLeft: tabBarRect.left + 4, slotWidth: defaultWidth };
  }
  if (resolvedIndex <= 0) {
    return { insertX: tabRects[0]!.left, top, height, slotLeft: tabRects[0]!.left, slotWidth: defaultWidth };
  }
  if (resolvedIndex >= tabRects.length) {
    const last = tabRects[tabRects.length - 1]!;
    return { insertX: last.right, top, height, slotLeft: last.right, slotWidth: defaultWidth };
  }
  const prev = tabRects[resolvedIndex - 1]!;
  const next = tabRects[resolvedIndex]!;
  const insertX = (prev.right + next.left) / 2;
  return { insertX, top, height, slotLeft: insertX - defaultWidth / 2, slotWidth: defaultWidth };
}

function pointerInRect(x: number, y: number, rect: DOMRect): boolean {
  return x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom;
}

/** @emoji 🧭 Maps pointer position in a rectangle to a split side using half-panel zones (dominant axis from center). */
function resolveModeSplitSideInBody(localX: number, localY: number, bodyWidth: number, bodyHeight: number): ModeDockSide {
  const midX = bodyWidth / 2;
  const midY = bodyHeight / 2;
  const dx = Math.abs(localX - midX);
  const dy = Math.abs(localY - midY);
  if (dx >= dy) return localX < midX ? "left" : "right";
  return localY < midY ? "top" : "bottom";
}

/** @emoji 📐 Half-panel rectangle for split drop preview inside a stack body (origin top-left of body). */
function computeModeSplitPreviewInBody(
  bodyWidth: number,
  bodyHeight: number,
  side: ModeDockSide,
): { left: number; top: number; width: number; height: number } {
  const halfWidth = bodyWidth / 2;
  const halfHeight = bodyHeight / 2;
  if (side === "left") return { left: 0, top: 0, width: halfWidth, height: bodyHeight };
  if (side === "right") return { left: bodyWidth - halfWidth, top: 0, width: halfWidth, height: bodyHeight };
  if (side === "top") return { left: 0, top: 0, width: bodyWidth, height: halfHeight };
  return { left: 0, top: bodyHeight - halfHeight, width: bodyWidth, height: halfHeight };
}

function computeModeDropZone(
  pointerX: number,
  pointerY: number,
  stackTargets: ReadonlyMap<ModeLayoutPath, ModeStackDropTargets>,
  modeRect: DOMRect | null,
): ModeDropZone | null {
  for (const [stackPath, targets] of stackTargets) {
    if (targets.tabBar && pointerInRect(pointerX, pointerY, targets.tabBar)) {
      return { kind: "tab", stackPath, index: computeTabInsertIndex(pointerX, targets.tabBarElement) };
    }
  }
  for (const [stackPath, targets] of stackTargets) {
    const rect = targets.body;
    if (!rect || !pointerInRect(pointerX, pointerY, rect)) continue;
    const side = resolveModeSplitSideInBody(pointerX - rect.left, pointerY - rect.top, rect.width, rect.height);
    return { kind: "split", stackPath, side };
  }
  if (!modeRect || !pointerInRect(pointerX, pointerY, modeRect)) return null;
  const side = resolveModeSplitSideInBody(
    pointerX - modeRect.left,
    pointerY - modeRect.top,
    modeRect.width,
    modeRect.height,
  );
  return { kind: "root-split", side };
}

function applyModeDrop(layout: WindowLayoutNode, drag: ModeDragState, zone: ModeDropZone): WindowLayoutNode {
  const { windowId, stackPath: sourcePath, tabIndex } = drag;
  if (zone.kind === "root-split") return splitRootWithWindow(layout, windowId, zone.side);
  if (zone.kind === "split") return splitWithWindow(layout, zone.stackPath, windowId, zone.side);
  if (zone.stackPath === sourcePath) {
    const stackNode = readLayoutAtPath(layout, sourcePath);
    const childCount = stackNode?.kind === "stack" ? stackNode.children.length : 0;
    const withoutLength = Math.max(0, childCount - 1);
    const toIndex = zone.index < 0 ? tabIndex : Math.min(zone.index, withoutLength);
    if (toIndex === tabIndex) return layout;
    return reorderTabInStack(layout, sourcePath, tabIndex, toIndex);
  }
  const without = removeWindowFromLayout(layout, windowId);
  if (!without) return layout;
  return insertWindowAsTab(without, zone.stackPath, windowId, zone.index < 0 ? undefined : zone.index);
}

/** @emoji 🪓 Removes the dragged window from the committed layout while it floats on the cursor. */
function modeDockOutLayout(committed: WindowLayoutNode, windowId: string): WindowLayoutNode {
  return removeWindowFromLayout(committed, windowId) ?? committed;
}

//#endregion 🧭ModeDockDrag

//#region 🧭ModeDockDragPreview

const MODE_DRAG_CURSOR_TAB_WIDTH = 148;
const MODE_DRAG_CURSOR_OFFSET_X = 12;
const MODE_DRAG_CURSOR_OFFSET_Y = 14;

interface ModeDockDragPreviewProps {
  title: string;
  content?: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
  tabOnly?: boolean;
}

/** @emoji 🪟 Floating tab or window preview shown while docking. */
const ModeDockDragPreview: React.FC<ModeDockDragPreviewProps> = ({ title, content, className, style, tabOnly = false }) =>
  tabOnly ? (
    <div
      data-slot="mode-dock-drag-preview"
      className={cn(
        "pointer-events-none flex max-w-[12rem] shrink-0 items-center gap-half px-single text-xs text-foreground shadow-md select-none",
        modeDockInactiveTabClass,
        className,
      )}
      style={style}
    >
      <span className="truncate">{title}</span>
    </div>
  ) : (
    <div
      data-slot="mode-dock-drag-preview"
      className={cn("pointer-events-none flex flex-col overflow-hidden rounded shadow-lg", className)}
      style={style}
    >
      <div data-slot="mode-dock-drag-preview-cap" className={cn("relative z-[2] flex h-medium shrink-0 items-stretch px-single", windowCapFrameClass)}>
        <span className="flex min-w-0 flex-1 items-center truncate text-xs">{title}</span>
      </div>
      <div
        data-slot="mode-dock-drag-preview-body"
        className={cn("relative min-h-0 flex-1 overflow-hidden p-single opacity-95", windowBodyFrameClass)}
      >
        {content ? <div className="h-full w-full overflow-hidden bg-window [&_*]:pointer-events-none">{content}</div> : null}
      </div>
    </div>
  );

//#endregion 🧭ModeDockDragPreview

//#region 🧭ModeDockTabBar

interface ModeDockContextValue {
  dragState: ModeDragState | null;
  registerStackDropTargets: (path: ModeLayoutPath, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => void;
  startTabDrag: (windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => void;
  clearPendingDrag: (pointerId: number) => void;
  closeWindow: (windowId: string) => void;
  activateWindow: (windowId: string) => void;
  maximizedStackPath: ModeLayoutPath | null;
  toggleMaximize: (stackPath: ModeLayoutPath) => void;
}

const ModeDockContext = reactHostPort.createContext<ModeDockContextValue | null>(null);

interface ModeDockTabBarProps {
  stackPath: ModeLayoutPath;
  tabs: readonly { id: string; title: string }[];
  activeId: string | undefined;
  activeWindowId: string | null;
  onSelectTab: (windowId: string) => void;
  chromeGrid?: ModeDockChromeGrid;
  chromeBody?: React.ReactNode;
}

const ModeDockTabBar = reactHostPort.forwardRef<HTMLDivElement, ModeDockTabBarProps>(({ stackPath, tabs, activeId, activeWindowId, onSelectTab, chromeGrid, chromeBody }, ref) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const isMaximized = dock?.maximizedStackPath === stackPath;
  const stackGloballyActive = Boolean(activeId && activeWindowId === activeId);
  const perTabActiveChrome = Boolean(chromeGrid);
  const capFrameClass = stackGloballyActive ? windowCapFrameActiveClass : windowCapFrameClass;
  const gapFrameClass = stackGloballyActive ? windowGapFrameActiveClass : windowGapFrameClass;
  const frameLineClass = stackGloballyActive ? activeLineClass : secondaryLineClass;
  const baselineBottomClass = stackGloballyActive ? "border-b-active-base" : "border-b-element";

  const renderTab = (tab: (typeof tabs)[number], index: number) => (
    <div
      key={tab.id}
      data-slot="mode-dock-tab"
      data-window-id={tab.id}
      data-stack-active={activeId === tab.id ? "true" : undefined}
      data-active={activeWindowId === tab.id ? "true" : undefined}
      className={cn(
        "group flex max-w-[12rem] shrink-0 cursor-pointer items-center gap-half px-single text-xs text-muted-foreground select-none hover:bg-hover-window hover:text-foreground",
        !perTabActiveChrome && "bg-window",
        perTabActiveChrome && activeId !== tab.id && cn(modeDockInactiveTabClass, baselineBottomClass),
        perTabActiveChrome && activeId === tab.id && !stackGloballyActive && cn(modeDockInactiveTabClass, baselineBottomClass),
        perTabActiveChrome && activeId === tab.id && stackGloballyActive && modeDockActiveTabClass,
        !perTabActiveChrome && activeWindowId === tab.id && modeDockActiveTabFillClass,
        activeWindowId !== tab.id && activeId === tab.id && "text-foreground",
        perTabActiveChrome && activeId === tab.id && "text-foreground",
      )}
      onClick={() => onSelectTab(tab.id)}
      onPointerUp={(event) => {
        if (event.button !== 0) return;
        dock?.clearPendingDrag?.(event.pointerId);
      }}
      onPointerDownCapture={(event) => {
        if ((event.target as HTMLElement).closest("[data-slot='mode-dock-tab-close']")) return;
        dock?.startTabDrag(tab.id, stackPath, index, tab.title, event);
      }}
    >
      <span className="truncate">{tab.title}</span>
      <button
        type="button"
        data-slot="mode-dock-tab-close"
        className="ml-auto flex size-small shrink-0 items-center justify-center rounded opacity-60 hover:bg-hover-window hover:opacity-100"
        onPointerDown={(event) => event.stopPropagation()}
        onClick={(event) => {
          event.stopPropagation();
          dock?.closeWindow(tab.id);
        }}
      >
        <CloseIcon className="size-tiny" />
      </button>
    </div>
  );

  const controlsCap = (
    <div
      data-slot="mode-dock-controls-cap"
      className={cn(
        perTabActiveChrome
          ? stackGloballyActive
            ? windowControlsCapActiveSplitClass
            : windowControlsCapClass
          : stackGloballyActive
            ? windowControlsCapActiveClass
            : windowControlsCapClass,
      )}
    >
      <button
        type="button"
        data-slot="mode-dock-maximize"
        className="flex size-medium items-center justify-center border-0 bg-transparent hover:bg-hover-window"
        onClick={() => dock?.toggleMaximize(stackPath)}
      >
        {isMaximized ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />}
      </button>
      <div data-slot="mode-dock-controls-cap-corner" className={windowCapCornerClass(frameLineClass, "left")} aria-hidden />
    </div>
  );

  const tabGap = (
    <div
      data-slot="mode-dock-tab-gap"
      className={cn("relative min-h-medium min-w-0 flex-1 bg-canvas", perTabActiveChrome ? "z-0" : "z-[1]", gapFrameClass)}
      aria-hidden
    />
  );

  if (perTabActiveChrome && chromeGrid && chromeBody) {
    return (
      <div
        data-slot="mode-dock-chrome-column"
        className="relative z-[2] grid h-full min-h-0 min-w-0 flex-1 grid-rows-[auto_minmax(0,1fr)]"
        style={{ gridTemplateColumns: chromeGrid.templateColumns }}
      >
        <div
          ref={ref}
          data-slot="mode-dock-tabbar"
          className="grid min-h-medium min-w-0 items-stretch"
          style={{ gridColumn: "1 / -1", gridRow: 1, gridTemplateColumns: chromeGrid.templateColumns }}
        >
          {tabs.map((tab, index) => (
            <div
              key={tab.id}
              data-slot={activeId === tab.id && stackGloballyActive ? "mode-dock-tab-active-cell" : "mode-dock-tab-cell"}
              className={cn(
                "relative flex min-h-medium items-stretch justify-self-start overflow-visible",
                activeId === tab.id && stackGloballyActive ? "z-10" : "z-20",
              )}
              style={{ gridColumn: chromeGrid.tabCol(index) }}
            >
              {tabInsertIndex === index ? renderInsertSlot(`insert-${index}`) : null}
              {renderTab(tab, index)}
              {activeId === tab.id && stackGloballyActive ? (
                <div data-slot="mode-dock-tab-cap-corner" className={windowCapCornerClass(frameLineClass, "right", true)} aria-hidden />
              ) : null}
            </div>
          ))}
          {tabInsertIndex === tabs.length ? (
            <div className="relative z-20 flex min-h-medium items-stretch justify-self-start" style={{ gridColumn: chromeGrid.gapCol }}>
              {renderInsertSlot("insert-end")}
            </div>
          ) : null}
          <div className="relative z-0 flex min-h-medium min-w-0 items-stretch" style={{ gridColumn: chromeGrid.gapCol }}>
            {tabGap}
          </div>
          <div className="relative z-10 flex min-h-medium items-stretch justify-self-end" style={{ gridColumn: chromeGrid.controlsCol }}>
            {controlsCap}
          </div>
        </div>
        <div
          className="flex min-h-0 min-w-0 flex-col overflow-hidden"
          style={{ gridColumn: chromeGrid.bodyColumnSpan, gridRow: 2 }}
        >
          {chromeBody}
        </div>
      </div>
    );
  }

  return (
    <div ref={ref} data-slot="mode-dock-tabbar" className="relative z-[2] flex w-full min-w-0 shrink-0 items-stretch bg-transparent">
      <div
        data-slot="mode-dock-tab-cap"
        className={cn(
          "relative flex min-h-medium min-w-0 max-w-[calc(100%-var(--size-medium))] shrink-0 items-stretch",
          capFrameClass,
        )}
      >
        <div data-slot="mode-dock-tabs" className="flex min-w-0 items-stretch justify-start overflow-x-auto overflow-y-hidden">
          {renderTabsWithInserts(tabs, 0)}
          {tabInsertIndex === tabs.length ? renderInsertSlot("insert-end") : null}
        </div>
        <div data-slot="mode-dock-tab-cap-corner" className={windowCapCornerClass(frameLineClass, "right")} aria-hidden />
      </div>
      {tabGap}
      {controlsCap}
    </div>
  );
});

ModeDockTabBar.displayName = "ModeDockTabBar";

//#region 🧭ModeDockStack

interface ModeDockStackProps {
  stackPath: ModeLayoutPath;
  node: WindowLayoutStackNode;
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
}

const ModeDockStack: React.FC<ModeDockStackProps> = ({ stackPath, node, windowsById, activeWindowId }) => {
  const dock = reactHostPort.useContext(ModeDockContext);
  const tabBarRef = reactHostPort.useRef<HTMLDivElement>(null);
  const bodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const activeId = node.activeId ?? node.children[0]?.id;
  const tabs = node.children.map((child) => ({
    id: child.id,
    title: child.title ?? windowsById.get(child.id)?.title ?? child.id,
  }));

  reactHostPort.useLayoutEffect(() => {
    dock?.registerStackDropTargets(stackPath, tabBarRef.current, bodyRef.current);
    return () => dock?.registerStackDropTargets(stackPath, null, null);
  }, [dock, stackPath, node.children.length]);

  const activeDescriptor = activeId ? windowsById.get(activeId) : undefined;
  const stackGloballyActive = Boolean(activeId && activeWindowId === activeId);
  const chromeGrid = tabs.length > 1 ? modeDockChromeGridPlacement(tabs, activeId) : undefined;

  const stackBody = (
    <div
      ref={bodyRef}
      data-slot="mode-dock-stack-body"
      className={cn(
        "relative z-0 flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden p-single",
        stackGloballyActive ? windowBodyFrameActiveClass : windowBodyFrameClass,
      )}
    >
      {activeDescriptor ? (
        (() => {
          const { children, engagement, ...windowProps } = activeDescriptor;
          return (
            <Window {...windowProps} engagement={engagement} active={activeWindowId === activeId} onActivate={() => dock?.activateWindow(activeId!)}>
              {children}
            </Window>
          );
        })()
      ) : null}
    </div>
  );

  return (
    <div
      data-slot="mode-dock-stack"
      data-stack-path={stackPath}
      data-active={stackGloballyActive ? "true" : undefined}
      className="flex h-full min-h-0 w-full min-w-0 flex-col overflow-hidden bg-transparent"
    >
      {chromeGrid ? (
        <ModeDockTabBar
          ref={tabBarRef}
          stackPath={stackPath}
          tabs={tabs}
          activeId={activeId}
          activeWindowId={activeWindowId}
          chromeGrid={chromeGrid}
          chromeBody={stackBody}
          onSelectTab={(windowId) => dock?.activateWindow(windowId)}
        />
      ) : (
        <>
          <ModeDockTabBar
            ref={tabBarRef}
            stackPath={stackPath}
            tabs={tabs}
            activeId={activeId}
            activeWindowId={activeWindowId}
            onSelectTab={(windowId) => dock?.activateWindow(windowId)}
          />
          {stackBody}
        </>
      )}
    </div>
  );
};

//#endregion 🧭ModeDockStack

//#region 🧭ModeRender

interface ModeRenderContext {
  windowsById: ReadonlyMap<string, ModeWindowDescriptor>;
  activeWindowId: string | null;
  onAxisLayoutChanged: (axisPath: ModeLayoutPath, sizes: Record<string, number>) => void;
}

function renderModeDockNode(node: WindowLayoutNode, path: ModeLayoutPath, ctx: ModeRenderContext): React.ReactNode {
  if (node.kind === "stack") {
    return <ModeDockStack key={path || "root-stack"} stackPath={path} node={node} windowsById={ctx.windowsById} activeWindowId={ctx.activeWindowId} />;
  }
  const orientation = node.kind === "row" ? "horizontal" : "vertical";
  const panels: React.ReactNode[] = [];
  node.children.forEach((child, index) => {
    const childPath = modeJoinPath(path, index);
    if (index > 0)
      panels.push(
        <ResizableHandle
          key={`sep-${childPath}`}
          className={cn(
            "relative border-0 bg-transparent after:hidden hover:bg-accent/25",
            modeCanvasSplitterClass(orientation),
          )}
        />,
      );
    panels.push(
      <ResizablePanel key={childPath} id={childPath} defaultSize={child.size ?? 100 / node.children.length} minSize={8} className="box-border min-h-0 min-w-0">
        {renderModeDockNode(child as WindowLayoutNode, childPath, ctx)}
      </ResizablePanel>,
    );
  });
  return (
    <ResizablePanelGroup
      key={path || "root-axis"}
      id={`mode-axis-${path || "root"}`}
      orientation={orientation}
      onLayoutChanged={(sizes) => ctx.onAxisLayoutChanged(path, sizes)}
      className="h-full min-h-0 w-full min-w-0"
    >
      {panels}
    </ResizablePanelGroup>
  );
}

//#endregion 🧭ModeRender

/** @emoji 🪟 Golden-Layout-style docking mode shell with tab stacks, drag-dock, resize, maximize, and close. */
const Mode: React.FC<ModeProps> = ({ windows, activeWindowId, onActiveWindowChange, layout, children, className = "" }) => {
  const windowsById = reactHostPort.useMemo(() => new Map(windows.map((window) => [window.id, window])), [windows]);
  const windowsKey = reactHostPort.useMemo(() => windows.map((window) => window.id).join("|"), [windows]);
  const layoutKey = reactHostPort.useMemo(() => JSON.stringify(layout ?? null), [layout]);
  const initialLayout = reactHostPort.useMemo(() => resolveModeLayout(windows, layout), [layout, windows]);
  const [layoutState, setLayoutState] = reactHostPort.useState<WindowLayoutNode>(() => initialLayout);
  const [maximizedStackPath, setMaximizedStackPath] = reactHostPort.useState<ModeLayoutPath | null>(null);
  const [dragState, setDragState] = reactHostPort.useState<ModeDragState | null>(null);
  const [pendingDrag, setPendingDrag] = reactHostPort.useState<ModePendingDrag | null>(null);
  const [dropZone, setDropZone] = reactHostPort.useState<ModeDropZone | null>(null);
  const dropZoneRef = reactHostPort.useRef<ModeDropZone | null>(null);
  const modeBodyRef = reactHostPort.useRef<HTMLDivElement>(null);
  const stackDropElementsRef = reactHostPort.useRef(new Map<ModeLayoutPath, { tabBar: HTMLElement | null; body: HTMLElement | null }>());
  const layoutStateRef = reactHostPort.useRef(layoutState);
  const dragLayoutSnapshotRef = reactHostPort.useRef<WindowLayoutNode | null>(null);
  const layoutKeyRef = reactHostPort.useRef(layoutKey);
  const windowsKeyRef = reactHostPort.useRef(windowsKey);
  layoutStateRef.current = layoutState;

  reactHostPort.useEffect(() => {
    const layoutChanged = layoutKeyRef.current !== layoutKey;
    if (!layoutChanged && windowsKeyRef.current === windowsKey) return;
    layoutKeyRef.current = layoutKey;
    windowsKeyRef.current = windowsKey;
    setLayoutState(resolveModeLayout(windows, layout));
    setMaximizedStackPath(null);
  }, [layout, layoutKey, windows, windowsKey]);

  reactHostPort.useEffect(() => {
    if (!activeWindowId) return;
    setLayoutState((prev) => setActiveWindowInLayout(prev, activeWindowId));
  }, [activeWindowId]);

  const registerStackDropTargets = reactHostPort.useCallback((path: ModeLayoutPath, tabBarElement: HTMLElement | null, bodyElement: HTMLElement | null) => {
    if (!tabBarElement && !bodyElement) {
      stackDropElementsRef.current.delete(path);
      return;
    }
    const prev = stackDropElementsRef.current.get(path) ?? { tabBar: null, body: null };
    stackDropElementsRef.current.set(path, {
      tabBar: tabBarElement ?? prev.tabBar,
      body: bodyElement ?? prev.body,
    });
  }, []);

  const activateWindow = reactHostPort.useCallback(
    (windowId: string) => {
      if (activeWindowId === windowId) return;
      setLayoutState((prev) => setActiveWindowInLayout(prev, windowId));
      onActiveWindowChange?.(windowId);
    },
    [activeWindowId, onActiveWindowChange],
  );

  const closeWindow = reactHostPort.useCallback(
    (windowId: string) => {
      setLayoutState((prev) => {
        const next = collapseLayout(removeWindowFromLayout(prev, windowId)) ?? { kind: "stack", children: [] };
        const remaining = modeCollectWindowIds(next);
        if (activeWindowId === windowId) {
          const fallback = remaining[0] ?? null;
          if (fallback) onActiveWindowChange?.(fallback);
        }
        return next;
      });
    },
    [activeWindowId, onActiveWindowChange],
  );

  const toggleMaximize = reactHostPort.useCallback((stackPath: ModeLayoutPath) => {
    setMaximizedStackPath((prev) => (prev === stackPath ? null : stackPath));
  }, []);

  const refreshDropZone = reactHostPort.useCallback((clientX: number, clientY: number) => {
    const targets = new Map<ModeLayoutPath, ModeStackDropTargets>();
    stackDropElementsRef.current.forEach((elements, path) => {
      targets.set(path, {
        tabBar: elements.tabBar?.getBoundingClientRect() ?? null,
        body: elements.body?.getBoundingClientRect() ?? null,
        tabBarElement: elements.tabBar,
      });
    });
    const modeRect = modeBodyRef.current?.getBoundingClientRect() ?? null;
    const zone = computeModeDropZone(clientX, clientY, targets, modeRect);
    dropZoneRef.current = zone;
    setDropZone(zone);
  }, []);

  const finishDrag = reactHostPort.useCallback(
    (drag: ModeDragState, zone: ModeDropZone | null) => {
      if (!zone) return;
      setLayoutState((prev) => applyModeDrop(prev, drag, zone));
      activateWindow(drag.windowId);
    },
    [activateWindow],
  );

  const clearPendingDrag = reactHostPort.useCallback((pointerId: number) => {
    setPendingDrag((prev) => (prev?.pointerId === pointerId ? null : prev));
  }, []);

  const startTabDrag = reactHostPort.useCallback(
    (windowId: string, stackPath: ModeLayoutPath, tabIndex: number, label: string, event: React.PointerEvent<HTMLElement>) => {
      if (event.button !== 0) return;
      setPendingDrag({
        windowId,
        stackPath,
        tabIndex,
        pointerId: event.pointerId,
        ghostLabel: label,
        startX: event.clientX,
        startY: event.clientY,
      });
    },
    [],
  );

  reactHostPort.useEffect(() => {
    if (!pendingDrag && !dragState) return;
    const handleMove = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (pendingDrag && !dragState) {
        const distance = Math.hypot(event.clientX - pendingDrag.startX, event.clientY - pendingDrag.startY);
        if (distance < 6) return;
        dragLayoutSnapshotRef.current = layoutStateRef.current;
        setDragState({
          windowId: pendingDrag.windowId,
          stackPath: pendingDrag.stackPath,
          tabIndex: pendingDrag.tabIndex,
          pointerId: pendingDrag.pointerId,
          ghostLabel: pendingDrag.ghostLabel,
          x: event.clientX,
          y: event.clientY,
        });
        setPendingDrag(null);
        refreshDropZone(event.clientX, event.clientY);
        return;
      }
      if (!dragState) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropZone(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (dragState) finishDrag(dragState, dropZoneRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [pendingDrag, dragState, finishDrag, refreshDropZone]);

  reactHostPort.useEffect(() => {
    if (!dragState) return;
    const cancelDrag = () => {
      if (dragLayoutSnapshotRef.current) setLayoutState(dragLayoutSnapshotRef.current);
      dragLayoutSnapshotRef.current = null;
      setDragState(null);
      setPendingDrag(null);
      dropZoneRef.current = null;
      setDropZone(null);
    };
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      cancelDrag();
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [dragState]);

  const onAxisLayoutChanged = reactHostPort.useCallback((axisPath: ModeLayoutPath, sizes: Record<string, number>) => {
    setLayoutState((prev) => applyAxisSizes(prev, axisPath, sizes));
  }, []);

  const dockContext = reactHostPort.useMemo<ModeDockContextValue>(
    () => ({
      dragState,
      dropZone,
      registerStackDropTargets,
      startTabDrag,
      clearPendingDrag,
      closeWindow,
      activateWindow,
      maximizedStackPath,
      toggleMaximize,
    }),
    [dragState, dropZone, registerStackDropTargets, startTabDrag, clearPendingDrag, closeWindow, activateWindow, maximizedStackPath, toggleMaximize],
  );

  const renderContext = reactHostPort.useMemo<ModeRenderContext>(() => ({ windowsById, activeWindowId, onAxisLayoutChanged }), [windowsById, activeWindowId, onAxisLayoutChanged]);

  const dockOutLayout = reactHostPort.useMemo(
    () => (dragState ? modeDockOutLayout(layoutState, dragState.windowId) : layoutState),
    [layoutState, dragState],
  );

  const maximizedStack =
    maximizedStackPath !== null
      ? (() => {
          let found: WindowLayoutStackNode | null = null;
          mapLayoutStacks(dockOutLayout, (stack, path) => {
            if (path === maximizedStackPath) found = stack;
            return stack;
          });
          return found;
        })()
      : null;

  const body =
    children ??
    (maximizedStack ? (
      <ModeDockContext.Provider value={dockContext}>
        <ModeDockStack stackPath={maximizedStackPath!} node={maximizedStack} windowsById={windowsById} activeWindowId={activeWindowId} />
      </ModeDockContext.Provider>
    ) : (
      <ModeDockContext.Provider value={dockContext}>{renderModeDockNode(dockOutLayout, "", renderContext)}</ModeDockContext.Provider>
    ));

  const draggedWindow = dragState ? windowsById.get(dragState.windowId) : undefined;
  const draggedPreviewTitle = draggedWindow?.title ?? dragState?.ghostLabel ?? "";

  return (
    <div
      data-slot="mode"
      data-dragging={dragState ? "true" : undefined}
      data-maximized-path={maximizedStackPath ?? undefined}
      className={cn("relative flex h-full min-h-0 w-full flex-col", className)}
    >
      <LevelProvider level="canvas">
        <div ref={modeBodyRef} data-slot="mode-body" className={cn("relative box-border flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden bg-canvas", MODE_CANVAS_INSET_CLASS)}>
          {body}
        {dragState ? (
          <>
            <ModeDockDragPreview
              title={draggedPreviewTitle}
              tabOnly
              compact
              style={{
                position: "fixed",
                left: dragState.x + MODE_DRAG_CURSOR_OFFSET_X,
                top: dragState.y - MODE_DRAG_CURSOR_OFFSET_Y,
                width: MODE_DRAG_CURSOR_TAB_WIDTH,
                zIndex: 70,
              }}
            />
            {dropZone ? (
              <div data-slot="mode-dock-drop-indicator" className="pointer-events-none absolute inset-0 z-panel">
                {dropZone.kind === "split" || dropZone.kind === "root-split" ? (
                  <div
                    className="absolute rounded-sm border-2 border-accent bg-accent/20"
                    style={(() => {
                      if (dropZone.kind === "root-split") {
                        const side = dropZone.side;
                        if (side === "left") return { left: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "right") return { right: 0, top: 0, width: "50%", height: "100%" };
                        if (side === "top") return { left: 0, top: 0, width: "100%", height: "50%" };
                        return { left: 0, bottom: 0, width: "100%", height: "50%" };
                      }
                      const elements = stackDropElementsRef.current.get(dropZone.stackPath);
                      const rect = elements?.body?.getBoundingClientRect();
                      const modeRect = modeBodyRef.current?.getBoundingClientRect();
                      if (!rect || !modeRect) return { display: "none" };
                      const bodyOriginLeft = rect.left - modeRect.left;
                      const bodyOriginTop = rect.top - modeRect.top;
                      const preview = computeModeSplitPreviewInBody(rect.width, rect.height, dropZone.side);
                      return {
                        left: bodyOriginLeft + preview.left,
                        top: bodyOriginTop + preview.top,
                        width: preview.width,
                        height: preview.height,
                      };
                    })()}
                  />
                ) : (
                  (() => {
                    const elements = stackDropElementsRef.current.get(dropZone.stackPath);
                    const preview = computeTabInsertPreview(elements?.tabBar ?? null, dropZone.index);
                    const bodyRect = elements?.body?.getBoundingClientRect();
                    const modeRect = modeBodyRef.current?.getBoundingClientRect();
                    if (!preview || !modeRect) return null;
                    const lineLeft = preview.insertX - modeRect.left - 1;
                    const slotTop = preview.top - modeRect.top;
                    const bodyLeft = bodyRect ? bodyRect.left - modeRect.left : 0;
                    const bodyTop = bodyRect ? bodyRect.top - modeRect.top : slotTop + preview.height;
                    const bodyWidth = bodyRect?.width ?? 0;
                    const bodyHeight = bodyRect?.height ?? 0;
                    return (
                      <>
                        <div
                          data-slot="mode-dock-tab-insert-line"
                          className="absolute w-[2px] rounded-full bg-accent shadow-[0_0_0_1px_var(--accent)]"
                          style={{ left: lineLeft, top: slotTop + 2, height: preview.height - 4 }}
                        />
                        {bodyRect ? (
                          <div
                            data-slot="mode-dock-tab-target-body"
                            className="absolute rounded-sm border-2 border-accent/80 bg-accent/10"
                            style={{ left: bodyLeft, top: bodyTop, width: bodyWidth, height: bodyHeight }}
                          />
                        ) : null}
                      </>
                    );
                  })()
                )}
              </div>
            ) : null}
          </>
        ) : null}
        </div>
      </LevelProvider>
    </div>
  );
};

export {
  Mode,
  removeWindowFromLayout,
  splitWithWindow,
  applyModeDrop,
  reconcileWindows,
  normalizeLayoutToStacks,
  collapseLayout,
  computeModeDropZone,
  computeModeSplitPreviewInBody,
  resolveModeSplitSideInBody,
  computeTabInsertPreview,
  modeDockOutLayout,
};

// #endregion 🧭Mode

// #region 🧭App

/** @emoji 📱 Mode descriptor rendered inside {@link App}. */
export interface AppModeDescriptor {
  id: string;
  label?: string;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export interface AppProps {
  modes: AppModeDescriptor[];
  activeModeId: string;
  onActiveModeChange?: (modeId: string) => void;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** @emoji 📱 App shell with optional mode switcher and one active mode body. */
const App: React.FC<AppProps> = ({ modes, activeModeId, onActiveModeChange, children, className = "", chrome = true }) => {
  const activeMode = modes.find((mode) => mode.id === activeModeId) ?? modes[0];
  const body = children ?? activeMode?.children;
  const showModeNav = chrome && modes.length > 1 && !!onActiveModeChange;

  return (
    <div data-slot="app" className={cn("flex h-full min-h-0 w-full flex-col", className)}>
      {showModeNav ? (
        <div data-slot="app-mode-nav" className="flex shrink-0 items-center gap-single border-b border-element p-single">
          <Select id="app.mode.select" value={activeModeId} onValueChange={onActiveModeChange}>
            <SelectTrigger className="w-[min(100%,16rem)]">
              <SelectValue placeholder="Mode" />
            </SelectTrigger>
            <SelectContent>
              {modes.map((mode) => (
                <SelectItem key={mode.id} value={mode.id}>
                  {mode.label ?? mode.id}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      ) : null}
      <div data-slot="app-body" className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
      </div>
    </div>
  );
};

export { App };

// #endregion 🧭App

// #region 🧭Ui

/** @emoji 🖥️ App descriptor rendered inside {@link Ui}. */
export interface UiAppDescriptor {
  id: string;
  label?: string;
  icon?: React.ReactNode;
  children: React.ReactNode;
}

export interface UiProps {
  apps: UiAppDescriptor[];
  activeAppId: string;
  onActiveAppChange?: (appId: string) => void;
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  toolbar?: React.ReactNode;
  children?: React.ReactNode;
  className?: string;
  chrome?: boolean;
}

/** @emoji 🖥️ Top-level UI shell with optional app switcher and one active app body. */
const Ui: React.FC<UiProps> = ({ apps, activeAppId, onActiveAppChange, navbar, footer, toolbar, children, className = "", chrome = true }) => {
  const activeApp = apps.find((app) => app.id === activeAppId) ?? apps[0];
  const body = children ?? activeApp?.children;
  const showAppNav = chrome && apps.length > 1 && !!onActiveAppChange;

  const navbarItems: NavbarItem[] = [];
  if (showAppNav) {
    navbarItems.push({
      key: "appNav",
      content: (
        <ButtonGroup id="ui.appNav">
          {apps.map((app) => (
            <ButtonGroupItem
              key={app.id}
              id={`ui.appNav.${app.id}`}
              className={cn(activeAppId === app.id && "bg-active-base")}
              onClick={() => onActiveAppChange?.(app.id)}
              text={app.label}
            >
              {app.icon ?? app.label ?? app.id}
            </ButtonGroupItem>
          ))}
        </ButtonGroup>
      ),
    });
  }
  if (navbar) {
    navbarItems.push({ key: "navbar", className: "flex-1 min-w-0", content: navbar });
  }

  return (
    <div data-slot="ui" className={cn("relative flex h-full min-h-0 w-full flex-col", className)}>
      {navbarItems.length > 0 ? <Navbar items={navbarItems} /> : null}
      <div data-slot="ui-body" className="relative flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {body}
        {toolbar ? <div className="pointer-events-none absolute inset-x-0 bottom-large z-toolbar flex justify-center">{toolbar}</div> : null}
      </div>
      {footer ? <div data-slot="ui-footer">{footer}</div> : null}
    </div>
  );
};

export { Ui };

// #endregion 🧭Ui

// #endregion ⚙️Canvas

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { render, screen, fireEvent, waitFor } = await import("@testing-library/react");

  describe("Shell components", () => {
    it("Ui renders the active app body", () => {
      render(
        <Ui
          apps={[
            { id: "editor", label: "Editor", children: <div>Editor Body</div> },
            { id: "dashboard", label: "Dashboard", children: <div>Dashboard Body</div> },
          ]}
          activeAppId="dashboard"
          onActiveAppChange={() => {}}
        />,
      );
      expect(screen.getByText("Dashboard Body")).toBeTruthy();
    });

    it("App renders the active mode body", () => {
      render(
        <App
          modes={[
            { id: "design", label: "Design", children: <div>Design Mode</div> },
            { id: "review", label: "Review", children: <div>Review Mode</div> },
          ]}
          activeModeId="review"
          onActiveModeChange={() => {}}
        />,
      );
      expect(screen.getByText("Review Mode")).toBeTruthy();
    });

    it("modeDockChromeGridPlacement keeps tabs left and controls right", () => {
      const grid = modeDockChromeGridPlacement(
        [
          { id: "a", title: "A" },
          { id: "b", title: "B" },
          { id: "c", title: "C" },
        ],
        "b",
      );
      expect(grid.templateColumns).toBe("max-content max-content max-content minmax(0, 1fr) max-content");
      expect(grid.tabCol(0)).toBe(1);
      expect(grid.tabCol(1)).toBe(2);
      expect(grid.tabCol(2)).toBe(3);
      expect(grid.activeCol).toBe(2);
      expect(grid.gapCol).toBe(4);
      expect(grid.controlsCol).toBe(5);
      expect(grid.bodyColumnSpan).toBe("2 / 5");
    });

    it("modeDockChromeGridPlacement keeps every tab left of the flex gap", () => {
      const grid = modeDockChromeGridPlacement(
        [
          { id: "a", title: "A" },
          { id: "b", title: "B" },
          { id: "c", title: "C" },
        ],
        "b",
      );
      expect(grid.tabCol(0)).toBeLessThan(grid.gapCol);
      expect(grid.tabCol(1)).toBeLessThan(grid.gapCol);
      expect(grid.tabCol(2)).toBeLessThan(grid.gapCol);
      expect(grid.gapCol).toBeLessThan(grid.controlsCol);
    });

    it("Mode lays out all windows and marks the active one", () => {
      const { container } = render(
        <div className="h-[400px] w-[600px]">
          <Mode
            windows={[
              { id: "left", title: "Left", children: <div>Left Pane</div> },
              { id: "right", title: "Right", children: <div>Right Pane</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "left" }], activeId: "left" },
                { kind: "stack", children: [{ kind: "window", id: "right" }], activeId: "right" },
              ],
            }}
            activeWindowId="right"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(screen.getByText("Left Pane")).toBeTruthy();
      expect(screen.getByText("Right Pane")).toBeTruthy();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="window"][data-active="true"]')?.className).not.toContain("border-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="right"][data-active="true"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="left"][data-active="true"]')).toBeNull();
      expect(screen.getByText("Left")).toBeTruthy();
      expect(screen.getByText("Right")).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-body"]')?.className).toContain("bg-canvas");
      expect(container.querySelector('[data-slot="mode-dock-canvas-label"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("flex-1");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("bg-canvas");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("ml-auto");
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).not.toContain("justify-end");
      const tabbar = container.querySelector('[data-slot="mode-dock-tabbar"]');
      expect(tabbar?.querySelector('[data-slot="mode-dock-tab-cap"]')).toBeTruthy();
      expect(tabbar?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeTruthy();
      expect(
        [...(tabbar?.children ?? [])].map((child) => child.getAttribute("data-slot")).filter(Boolean),
      ).toEqual(["mode-dock-tab-cap", "mode-dock-tab-gap", "mode-dock-controls-cap"]);
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("bg-window");
      expect(container.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("bg-window");
      const activeStack = container.querySelector('[data-slot="mode-dock-stack"][data-active="true"]');
      const inactiveStack = container.querySelector('[data-slot="mode-dock-stack"]:not([data-active="true"])');
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("border-active-base");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("border-b-0");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-active-base");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-b-0");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-x");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).not.toContain("border-l-0");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-element");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-x");
      expect(activeStack?.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("border-active-base");
      expect(activeStack?.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("-mt-px");
      expect(activeStack?.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("border-t-0");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-b");
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-cap-corner"]')).toBeTruthy();
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap-corner"]')).toBeTruthy();
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-cap"]')?.className).toContain("border-element");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("border-element");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-element");
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).not.toContain("border-element");
    });

    it("Mode clears multi-tab active chrome on inactive stacks", () => {
      const { container } = render(
        <div className="h-[400px] w-[800px]">
          <Mode
            windows={[
              { id: "a1", title: "A1", children: <div>A1 Body</div> },
              { id: "a2", title: "A2", children: <div>A2 Body</div> },
              { id: "b1", title: "B1", children: <div>B1 Body</div> },
              { id: "b2", title: "B2", children: <div>B2 Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                {
                  kind: "stack",
                  children: [{ kind: "window", id: "a1" }, { kind: "window", id: "a2" }],
                  activeId: "a1",
                },
                {
                  kind: "stack",
                  children: [{ kind: "window", id: "b1" }, { kind: "window", id: "b2" }],
                  activeId: "b2",
                },
              ],
            }}
            activeWindowId="b2"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const inactiveStack = container.querySelector('[data-slot="mode-dock-stack"]:not([data-active="true"])');
      const activeStack = container.querySelector('[data-slot="mode-dock-stack"][data-active="true"]');
      const inactiveStackTab = inactiveStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]');
      const activeStackTab = activeStack?.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]');
      expect(inactiveStackTab?.className).toContain("border-element");
      expect(inactiveStackTab?.className).not.toContain("border-active-base");
      expect(inactiveStackTab?.className).not.toContain("border-b-0");
      expect(activeStackTab?.className).toContain("border-active-base");
      expect(activeStackTab?.className).toContain("border-b-0");
      expect(activeStackTab?.className).toContain("bg-active-base");
      expect(activeStackTab?.className).toContain("text-active-foreground");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-element");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).not.toContain("border-active-base");
      expect(activeStack?.querySelector('[data-slot="mode-dock-controls-cap"]')?.className).toContain("border-active-base");
      expect(inactiveStack?.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeNull();
      expect(activeStack?.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeTruthy();
    });

    it("Mode keeps one canvas inset and one gutter between adjacent stacks", () => {
      const { container } = render(
        <div className="h-[400px] w-[600px]">
          <Mode
            windows={[
              { id: "left", title: "Left", children: <div>Left Pane</div> },
              { id: "right", title: "Right", children: <div>Right Pane</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "left" }], activeId: "left" },
                { kind: "stack", children: [{ kind: "window", id: "right" }], activeId: "right" },
              ],
            }}
            activeWindowId="left"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const modeBody = container.querySelector('[data-slot="mode-body"]');
      expect(modeBody?.className).toContain(MODE_CANVAS_INSET_CLASS);
      const panelGroup = container.querySelector('[data-slot="resizable-panel-group"]');
      expect(panelGroup?.getAttribute("data-panel-group-direction")).toBe("horizontal");
      const panels = [...container.querySelectorAll('[data-slot="resizable-panel"]')];
      expect(panels.length).toBeGreaterThanOrEqual(2);
      for (const panel of panels) {
        expect(panel.className).not.toContain("p-single");
        expect(panel.className).not.toContain("p-double");
      }
      const horizontalHandle = container.querySelector('[data-slot="resizable-handle"]');
      expect(horizontalHandle).toBeTruthy();
      expect(horizontalHandle!.className).toContain("w-double");
      expect(horizontalHandle!.className).not.toContain("data-[panel-group-direction=horizontal]:w-double");
    });

    it("Mode tab stack shows only the active window body", () => {
      const { container } = render(
        <div className="h-[400px] w-[600px]">
          <Mode
            windows={[
              { id: "a", title: "Alpha", children: <div>Alpha Body</div> },
              { id: "b", title: "Beta", children: <div>Beta Body</div> },
            ]}
            layout={{ kind: "stack", children: [{ kind: "window", id: "a" }, { kind: "window", id: "b" }], activeId: "a" }}
            activeWindowId="a"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("bg-canvas");
      expect(container.querySelector('[data-slot="mode-dock-stack-body"]')?.className).toContain("p-single");
      expect(screen.getByText("Alpha Body")).toBeTruthy();
      expect(screen.queryByText("Beta Body")).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"]')?.className).toContain("z-[2]");
      expect(container.querySelector('[data-slot="mode-dock-tab-active-cell"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-tab-cap"]')).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("border-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("bg-active-base");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("text-active-foreground");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("border-r");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("border-b-0");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).not.toContain("border-r-0");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.className).toContain("z-20");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).toContain("z-30");
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-window-id="b"]')?.className).toContain("border-element");
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"]')).toBeTruthy();
      expect(container.querySelector('[data-slot="mode-dock-chrome-column"] [data-slot="mode-dock-stack-body"]')).toBeTruthy();
      const multiTabBar = container.querySelector('[data-slot="mode-dock-chrome-column"] [data-slot="mode-dock-tabbar"]');
      expect(multiTabBar?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeTruthy();
      expect(multiTabBar?.querySelectorAll('[data-slot="mode-dock-maximize"]')).toHaveLength(1);
      expect(container.querySelector('[data-slot="mode-dock-stack"]')?.className).not.toContain("grid");
      expect(container.querySelector('[data-slot="mode-dock-tab-gap"]')?.className).toContain("border-active-base");
      const tabOrder = () =>
        [...container.querySelectorAll('[data-slot="mode-dock-tab"]')].map((tab) => tab.getAttribute("data-window-id"));
      expect(tabOrder()).toEqual(["a", "b"]);
      fireEvent.click(screen.getByText("Beta"));
      expect(screen.getByText("Beta Body")).toBeTruthy();
      expect(screen.queryByText("Alpha Body")).toBeNull();
      expect(container.querySelector('[data-slot="mode-dock-tab"][data-stack-active="true"]')?.getAttribute("data-window-id")).toBe("b");
      expect(tabOrder()).toEqual(["a", "b"]);
    });

    it("Mode tab stack places body under active tab and gap only", () => {
      const { container } = render(
        <div className="h-[400px] w-[600px]">
          <Mode
            windows={[
              { id: "shape", title: "Shape", children: <div>Shape Body</div> },
              { id: "energy", title: "Energy", children: <div>Energy Body</div> },
            ]}
            layout={{ kind: "stack", children: [{ kind: "window", id: "shape" }, { kind: "window", id: "energy" }], activeId: "energy" }}
            activeWindowId="energy"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const grid = modeDockChromeGridPlacement(
        [
          { id: "shape", title: "Shape" },
          { id: "energy", title: "Energy" },
        ],
        "energy",
      );
      expect(grid.bodyColumnSpan).toBe("2 / 4");
      const chromeColumn = container.querySelector('[data-slot="mode-dock-chrome-column"]');
      const stackBody = chromeColumn?.querySelector('[data-slot="mode-dock-stack-body"]');
      expect(stackBody).toBeTruthy();
      expect(chromeColumn?.querySelectorAll('[data-slot="mode-dock-tab-cell"], [data-slot="mode-dock-tab-active-cell"]').length).toBe(2);
      expect(chromeColumn?.querySelector('[data-slot="mode-dock-tabs-before"]')).toBeNull();
      expect(chromeColumn?.querySelector('[data-slot="mode-dock-tabs-after"]')).toBeNull();
      expect(chromeColumn?.querySelector('[data-slot="mode-dock-tab-gap"]')).toBeTruthy();
      expect(chromeColumn?.querySelector('[data-slot="mode-dock-controls-cap"]')).toBeTruthy();
      expect(screen.getByText("Energy Body")).toBeTruthy();
      const bodyRow = chromeColumn?.querySelector('[data-slot="mode-dock-stack-body"]')?.parentElement;
      expect(bodyRow?.className).toContain("flex");
      expect(bodyRow?.className).toContain("flex-col");
      expect(bodyRow?.className).toContain("min-h-0");
      const gapCell = chromeColumn?.querySelector('[data-slot="mode-dock-tab-gap"]')?.parentElement;
      expect(gapCell?.className).toContain("flex");
      expect(gapCell?.className).toContain("items-stretch");
      const inactiveTab = chromeColumn?.querySelector('[data-slot="mode-dock-tab"][data-window-id="shape"]');
      expect(inactiveTab?.className).toContain("border-b-active-base");
      expect(inactiveTab?.className).not.toContain("border-b-0");
      const activeTab = chromeColumn?.querySelector('[data-slot="mode-dock-tab"][data-window-id="energy"]');
      expect(activeTab?.className).toContain("border-active-base");
      expect(activeTab?.className).toContain("border-b-0");
      expect(activeTab?.className).toContain("bg-active-base");
      expect(activeTab?.className).toContain("text-active-foreground");
    });

    it("Mode close removes a tab and collapses an emptied stack", () => {
      const { container } = render(
        <div className="h-[400px] w-[600px]">
          <Mode
            windows={[
              { id: "solo", title: "Solo", children: <div>Solo Body</div> },
              { id: "peer", title: "Peer", children: <div>Peer Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "solo" }], activeId: "solo" },
                { kind: "stack", children: [{ kind: "window", id: "peer" }], activeId: "peer" },
              ],
            }}
            activeWindowId="solo"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      const soloTab = container.querySelector("[data-stack-path='0'] [data-slot='mode-dock-tab-close']");
      expect(soloTab).toBeTruthy();
      fireEvent.click(soloTab!);
      expect(screen.queryByText("Solo Body")).toBeNull();
      expect(screen.getByText("Peer Body")).toBeTruthy();
    });

    it("computeTabInsertPreview resolves slot geometry at tab boundaries", () => {
      const tabBar = document.createElement("div");
      tabBar.setAttribute("data-slot", "mode-dock-tabbar");
      const tabA = document.createElement("div");
      tabA.setAttribute("data-slot", "mode-dock-tab");
      tabA.getBoundingClientRect = () => ({ left: 0, right: 80, top: 0, bottom: 24, width: 80, height: 24 }) as DOMRect;
      const tabB = document.createElement("div");
      tabB.setAttribute("data-slot", "mode-dock-tab");
      tabB.getBoundingClientRect = () => ({ left: 80, right: 160, top: 0, bottom: 24, width: 80, height: 24 }) as DOMRect;
      tabBar.appendChild(tabA);
      tabBar.appendChild(tabB);
      tabBar.getBoundingClientRect = () => ({ left: 0, right: 160, top: 0, bottom: 24, width: 160, height: 24 }) as DOMRect;
      const between = computeTabInsertPreview(tabBar, 1);
      expect(between?.insertX).toBe(80);
      const end = computeTabInsertPreview(tabBar, 2);
      expect(end?.insertX).toBe(160);
    });

    it("computeModeSplitPreviewInBody covers half the stack body on each side", () => {
      expect(computeModeSplitPreviewInBody(400, 300, "left")).toEqual({ left: 0, top: 0, width: 200, height: 300 });
      expect(computeModeSplitPreviewInBody(400, 300, "right")).toEqual({ left: 200, top: 0, width: 200, height: 300 });
      expect(computeModeSplitPreviewInBody(400, 300, "top")).toEqual({ left: 0, top: 0, width: 400, height: 150 });
      expect(computeModeSplitPreviewInBody(400, 300, "bottom")).toEqual({ left: 0, top: 150, width: 400, height: 150 });
    });

    it("resolveModeSplitSideInBody uses half-panel zones with dominant axis at corners", () => {
      expect(resolveModeSplitSideInBody(50, 100, 200, 200)).toBe("left");
      expect(resolveModeSplitSideInBody(150, 100, 200, 200)).toBe("right");
      expect(resolveModeSplitSideInBody(100, 50, 200, 200)).toBe("top");
      expect(resolveModeSplitSideInBody(100, 150, 200, 200)).toBe("bottom");
      expect(resolveModeSplitSideInBody(40, 40, 200, 200)).toBe("left");
      expect(resolveModeSplitSideInBody(160, 40, 200, 200)).toBe("right");
    });

    it("computeModeDropZone treats tab bar hits as tab drops not body splits", () => {
      const tabBar = { left: 0, top: 0, right: 200, bottom: 24, width: 200, height: 24 } as DOMRect;
      const body = { left: 0, top: 24, right: 200, bottom: 224, width: 200, height: 200 } as DOMRect;
      const targets = new Map([["1", { tabBar, body, tabBarElement: null }]]);
      expect(computeModeDropZone(100, 12, targets, null)).toEqual({ kind: "tab", stackPath: "1", index: 0 });
      expect(computeModeDropZone(100, 30, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "top" });
      expect(computeModeDropZone(100, 200, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "bottom" });
      expect(computeModeDropZone(50, 120, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "left" });
      expect(computeModeDropZone(150, 120, targets, null)).toEqual({ kind: "split", stackPath: "1", side: "right" });
    });

    it("computeModeDropZone root-split uses half of the mode when pointer is outside stack bodies", () => {
      const modeRect = { left: 0, top: 0, right: 400, bottom: 300, width: 400, height: 300 } as DOMRect;
      expect(computeModeDropZone(80, 150, new Map(), modeRect)).toEqual({ kind: "root-split", side: "left" });
      expect(computeModeDropZone(320, 150, new Map(), modeRect)).toEqual({ kind: "root-split", side: "right" });
      expect(computeModeDropZone(200, 40, new Map(), modeRect)).toEqual({ kind: "root-split", side: "top" });
      expect(computeModeDropZone(200, 260, new Map(), modeRect)).toEqual({ kind: "root-split", side: "bottom" });
    });

    it("modeDockOutLayout removes the dragged window without mutating drop targets", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          { kind: "stack", children: [{ kind: "window", id: "a" }, { kind: "window", id: "b" }], activeId: "a" },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const dockedOut = modeDockOutLayout(layout, "b");
      expect(modeCollectWindowIds(dockedOut)).toEqual(["a", "c"]);
      expect(modeCollectWindowIds(layout)).toEqual(["a", "b", "c"]);
    });

    it("removeWindowFromLayout and splitWithWindow mutate the layout tree", () => {
      const layout: WindowLayoutNode = {
        kind: "row",
        children: [
          { kind: "stack", children: [{ kind: "window", id: "a" }, { kind: "window", id: "b" }], activeId: "a" },
          { kind: "stack", children: [{ kind: "window", id: "c" }], activeId: "c" },
        ],
      };
      const removed = removeWindowFromLayout(layout, "b");
      expect(removed?.kind).toBe("row");
      const split = splitWithWindow(layout, "1", "b", "left");
      expect(split.kind).toBe("row");
      if (split.kind === "row") {
        const target = split.children[1];
        expect(target?.kind === "row" || target?.kind === "column").toBe(true);
      }
    });

    it("applyModeDrop splits within the same stack when dropping on a body edge zone", () => {
      const layout: WindowLayoutNode = {
        kind: "stack",
        children: [{ kind: "window", id: "a" }, { kind: "window", id: "b" }],
        activeId: "a",
      };
      const drag = {
        windowId: "a",
        stackPath: "",
        tabIndex: 0,
        pointerId: 1,
        ghostLabel: "A",
        x: 0,
        y: 0,
      };
      const zone = { kind: "split" as const, stackPath: "", side: "right" as const };
      const next = applyModeDrop(layout, drag, zone);
      expect(next.kind).toBe("row");
      if (next.kind !== "row") return;
      expect(next.children).toHaveLength(2);
      const leftStack = next.children[0];
      const rightStack = next.children[1];
      expect(leftStack?.kind).toBe("stack");
      expect(rightStack?.kind).toBe("stack");
      if (leftStack?.kind === "stack") expect(leftStack.children.map((c) => c.id)).toEqual(["b"]);
      if (rightStack?.kind === "stack") expect(rightStack.children.map((c) => c.id)).toEqual(["a"]);
    });

    it("Mode maximize shows only one stack", () => {
      const { container } = render(
        <div className="h-[400px] w-[600px]">
          <Mode
            windows={[
              { id: "a", title: "A", children: <div>A Body</div> },
              { id: "b", title: "B", children: <div>B Body</div> },
            ]}
            layout={{
              kind: "row",
              children: [
                { kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" },
                { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" },
              ],
            }}
            activeWindowId="a"
            onActiveWindowChange={() => {}}
          />
        </div>,
      );
      fireEvent.click(container.querySelector("[data-stack-path='0'] [data-slot='mode-dock-maximize']")!);
      expect(container.querySelector('[data-slot="mode"]')?.getAttribute("data-maximized-path")).toBe("0");
      expect(screen.getByText("A Body")).toBeTruthy();
      expect(screen.queryByText("B Body")).toBeNull();
    });

    it("Engagement renders options, input, and status lines", () => {
      const { container } = render(
        <Engagement
          options={[{ id: "opt-a", label: "Option A", onPress: () => {} }]}
          input={{ placeholder: "Type here" }}
          status={[{ id: "status-a", content: "Ready" }]}
        />,
      );
      expect(screen.getByRole("button", { name: "Option A" })).toBeTruthy();
      expect(screen.getByPlaceholderText("Type here")).toBeTruthy();
      expect(screen.getByText("Ready")).toBeTruthy();
      expect(container.querySelector('[data-slot="engagement"]')).toBeTruthy();
    });

    it("Engagement option buttons size to label text without clipping", () => {
      const longLabel = "C Confirm selection";
      const { container } = render(<Engagement options={[{ id: "engagement-transition-confirm-c", label: longLabel, onPress: () => {} }]} />);
      const item = container.querySelector('[data-slot="button-group-item"]') as HTMLElement;
      expect(item?.textContent).toContain(longLabel);
      expect(item?.className).toContain("aspect-auto");
      expect(item?.className).not.toContain("aspect-square");
    });

    it("Engagement focuses its input when active", async () => {
      const { rerender } = render(<Engagement active={false} input={{ id: "engagement-input", placeholder: "Command" }} />);
      const field = () => screen.getByPlaceholderText("Command") as HTMLInputElement;
      expect(document.activeElement).not.toBe(field());
      rerender(<Engagement active input={{ id: "engagement-input", placeholder: "Command" }} />);
      await waitFor(() => expect(document.activeElement).toBe(field()));
      expect(field().tabIndex).toBe(0);
    });

    it("Engagement input is removed from tab order when inactive", () => {
      render(<Engagement active={false} input={{ placeholder: "Command" }} />);
      expect((screen.getByPlaceholderText("Command") as HTMLInputElement).tabIndex).toBe(-1);
    });

    it("filterEngagementPossibles matches label, detail, and id", () => {
      const items = [
        { id: "primitive.box", label: "Box", detail: "b" },
        { id: "primitive.sphere", label: "Sphere", detail: "s" },
      ];
      expect(filterEngagementPossibles("", items)).toHaveLength(2);
      expect(filterEngagementPossibles("sph", items).map((row) => row.id)).toEqual(["primitive.sphere"]);
    });

    it("Engagement hides possibles until chevron opens and activates on Space or Enter", async () => {
      const scrollIntoView = Element.prototype.scrollIntoView;
      Element.prototype.scrollIntoView = () => undefined;
      const selected: string[] = [];
      render(
        <Engagement
          active
          input={{ placeholder: "Type an interaction" }}
          possibleEngagements={[
            { id: "primitive.box", label: "Box", detail: "b", onSelect: () => selected.push("primitive.box") },
            { id: "primitive.sphere", label: "Sphere", detail: "s", onSelect: () => selected.push("primitive.sphere") },
          ]}
        />,
      );
      const field = screen.getByPlaceholderText("Type an interaction");
      expect(document.querySelector('[data-slot="engagement-autocomplete"]')).toBeNull();
      fireEvent.click(document.querySelector('[data-slot="engagement-possibles-toggle"]')!);
      fireEvent.change(field, { target: { value: "sph" } });
      await waitFor(() => expect(document.querySelector('[data-slot="engagement-autocomplete"]')).toBeTruthy());
      expect(screen.getByText("Sphere")).toBeTruthy();
      fireEvent.keyDown(field, { key: "Enter" });
      await waitFor(() => expect(selected).toEqual(["primitive.sphere"]));
      fireEvent.change(field, { target: { value: "" } });
      fireEvent.click(document.querySelector('[data-slot="engagement-possibles-toggle"]')!);
      fireEvent.keyDown(field, { key: " " });
      await waitFor(() => expect(selected).toEqual(["primitive.sphere", "primitive.box"]));
      Element.prototype.scrollIntoView = scrollIntoView;
    });

    it("Window anchors engagement in a top overlay when active", () => {
      const { container } = render(
        <Window id="engagement-window" active engagement={{ status: [{ id: "s", content: "Idle" }] }}>
          <div>Body</div>
        </Window>,
      );
      const overlay = container.querySelector('[data-slot="window-engagement-overlay"]');
      expect(overlay).toBeTruthy();
      expect(overlay?.className).toContain("top-0");
      expect(overlay?.className).toContain("items-start");
      expect(overlay?.className).not.toContain("items-center");
      expect(overlay?.className).toContain("z-window");
      expect(overlay?.className).not.toContain("z-overlay");
      expect(screen.getByText("Idle")).toBeTruthy();
    });

    it("Window hides engagement overlay when inactive", () => {
      const { container } = render(
        <Window id="engagement-window" engagement={{ status: [{ id: "s", content: "Idle" }] }}>
          <div>Body</div>
        </Window>,
      );
      expect(container.querySelector('[data-slot="window-engagement-overlay"]')).toBeNull();
      expect(screen.queryByText("Idle")).toBeNull();
    });

    it("createEvenWindowLayout builds a row of stacks", () => {
      const layout = createEvenWindowLayout(["a", "b"]);
      expect(layout.kind).toBe("row");
      if (layout.kind === "row") {
        expect(layout.children).toHaveLength(2);
      }
    });
  });
}

// #endregion 🔍Window Components


// #region 🗿Framework Re-exports

// Re-exports of common libraries used alongside UI primitives.
// Workbench shell types and chrome live in `@framework/platform/core` / `@framework/platform/renderer/react`.

// #region 🌩️DnD Kit
export { closestCenter, DndContext, DragOverlay, PointerSensor, pointerWithin, rectIntersection, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
export type { DragEndEvent, DragOverEvent, DragStartEvent } from "@dnd-kit/core";
export { arrayMove, SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
export { CSS as DndCSS } from "@dnd-kit/utilities";
// #endregion 🌩️DnD Kit

// #region 📰Three.js
export { Select as DreiSelect, Edges, GizmoHelper, GizmoViewport, Grid, Line, OrbitControls, Sphere, useFBX, useGLTF } from "@react-three/drei";
export { Canvas as ThreeCanvas, useFrame, useLoader, useThree } from "@react-three/fiber";
export type { ThreeEvent } from "@react-three/fiber";
export * as THREE from "three";
export { OBJLoader } from "three/addons/loaders/OBJLoader.js";
// #endregion 📰Three.js

// #region 🎽XY Flow (additions not already exported inline)
export { ConnectionMode, MiniMap } from "@xyflow/react";
// #endregion 🎽XY Flow

// #region ⚗️Dagre
export * as dagre from "dagre";
// #endregion ⚗️Dagre

// #region 🖋️State Management
export { useSelector as useXStateSelector } from "@xstate/react";
export { assign, createActor, fromCallback, setup, type ActorRefFrom, type AnyActorRef, type SnapshotFrom } from "xstate";
// #endregion 🖋️State Management

// #region 🌈Routing
export { BrowserRouter, Link, MemoryRouter, Outlet, Route, Routes, useLocation, useNavigate, useParams, useSearchParams } from "react-router";
// #endregion 🌈Routing

// #region 🗿I18n
export { i18next, initReactI18next, LanguageDetector, useTranslation };
// #endregion 🗿I18n

// #region 🌙Hotkeys
export { useHotkeys } from "react-hotkeys-hook";
// #endregion 🌙Hotkeys

// #region ⛅Date
export { formatDistanceToNow } from "date-fns";
export { de as dateFnsDe, enUS as dateFnsEnUS } from "date-fns/locale";
// #endregion ⛅Date

// #region 🔔Search
export { default as Fuse } from "fuse.js";
export type { FuseResult } from "fuse.js";
// #endregion 🔔Search

// #region 🧵MDX
export { MDXProvider } from "@mdx-js/react";
// #endregion 🧵MDX

// #region 🌨️Styling
export { cva } from "class-variance-authority";
export type { VariantProps } from "class-variance-authority";
export { clsx } from "clsx";
// #endregion 🌨️Styling

// #region 📮Resizable Panels
export * as ResizablePrimitive from "react-resizable-panels";
// #endregion 📮Resizable Panels

// #endregion 🗿Framework Re-exports

const treeVitest = (
  import.meta as ImportMeta & {
    vitest?: {
      describe: typeof import("vitest").describe;
      expect: typeof import("vitest").expect;
      it: typeof import("vitest").it;
      vi: typeof import("vitest").vi;
    };
  }
).vitest;

if (treeVitest) {
  const { describe, expect, it, vi } = treeVitest;

  describe("tree helpers", () => {
    it("uses a single compact sibling gap for every row-kind transition", () => {
      expect(getTreeSiblingGapPx("leaf", "group")).toBe(treeCompactSiblingGapPx);
      expect(getTreeSiblingGapPx("property", "group")).toBe(treeCompactSiblingGapPx);
      expect(getTreeSiblingGapPx("property", "property")).toBe(treeCompactSiblingGapPx);
      expect(getTreeSiblingGapPx("group", "group")).toBe(treeCompactSiblingGapPx);
      expect(getTreeSiblingGapPx("content", "group")).toBe(treeCompactSiblingGapPx);
    });

    it("normalizes selected ids for single and multiple selection", () => {
      expect(normalizeTreeSelectedIds(["a", "a", "b"], "single")).toEqual(["a"]);
      expect(normalizeTreeSelectedIds(["a", "a", "b"], "multiple")).toEqual(["a", "b"]);
    });

    it("resolves hotkey values from strings and translation objects", () => {
      expect(resolveHotkeyValue("ctrl+p")).toBe("ctrl+p");
      expect(resolveHotkeyValue({ hotkey: "ctrl+f" })).toBe("ctrl+f");
      expect(resolveHotkeyValue({ label: "Search" })).toBeUndefined();
    });

    it("computes additive and range multi selection", () => {
      expect(
        getTreeNextSelectionState({
          selectionMode: "multiple",
          selectedIds: ["a"],
          orderedIds: ["a", "b", "c", "d"],
          targetId: "c",
          anchorId: "a",
          additiveKey: false,
          rangeKey: true,
        }),
      ).toEqual({ selectedIds: ["a", "b", "c"], anchorId: "a" });

      expect(
        getTreeNextSelectionState({
          selectionMode: "multiple",
          selectedIds: ["a"],
          orderedIds: ["a", "b", "c", "d"],
          targetId: "c",
          anchorId: "a",
          additiveKey: true,
          rangeKey: false,
        }),
      ).toEqual({ selectedIds: ["a", "c"], anchorId: "c" });
    });

    it("orders nested tree items across sections", () => {
      const sections: TreeDataSection[] = [
        {
          id: "section-a",
          label: "Section A",
          items: [
            { id: "item-a", label: "Item A", items: [{ id: "item-a-1", label: "Item A1" }] },
            { id: "item-b", label: "Item B" },
          ],
        },
        {
          id: "section-b",
          label: "Section B",
          items: [{ id: "item-c", label: "Item C" }],
        },
      ];

      expect(getTreeItemOrderedIds(sections, {}, {})).toEqual(["item-a", "item-a-1", "item-b", "item-c"]);
    });

    it("does not render an extra placeholder gap for tree property labels", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <Label id="tooltip.manual">
            <span>Control</span>
          </Label>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="property-label-tree"');
      expect(markup).toContain('data-slot="tree-row-layout"');
      expect(markup).toContain('data-slot="tree-gutter"');
      expect(markup).toContain("grid-template-columns:24px minmax(0, 1fr)");
      expect(markup).toContain('data-slot="property-label-tree" class="min-w-0"');
      expect(markup).toContain('data-slot="property-row"');
      expect(markup).toContain("margin-left:-10px");
      expect(markup).toContain("width:calc(100% + 10px)");
      expect(markup).toContain("grid-template-columns:96px minmax(0, 1fr)");
      expect(markup).toContain('data-slot="property-control"');
      expect(markup).toContain("justify-end");
      expect(markup).toContain("self-start");
      expect(markup).toContain("data-detail-panel-control");
      expect(markup).toContain("padding-left:10px");
      expect(markup).not.toContain("margin-left:13px");
      expect(markup).not.toContain("gap-[6px]");
      expect(markup).toContain('data-slot="tree-branch-elbow"');
      expect(markup).toContain('data-slot="tree-branch-elbow" class="pointer-events-none absolute h-px bg-muted-foreground/40 -translate-y-1/2 transition-[height,background-color] duration-150" style="top:12px;left:7px;width:10px"');
      expect(markup).not.toContain('style="top:50%;left:7px;width:10px"');
    });

    it("renders explicit property labels on the shared property-row wrapper", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <Label id="tooltip.manual" rowId="custom-row" label="piece">
            <span>Control</span>
          </Label>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('id="custom-row"');
      expect(markup).toContain('data-slot="property-control"');
      expect(markup).toContain('data-slot="tree-row-layout"');
      expect(markup).toContain('data-slot="property-control"');
      expect(markup).toContain("justify-end");
      expect(markup).toContain("self-start");
      expect(markup).toContain("data-detail-panel-control");
      expect(markup).toContain(">piece<");
    });

    it("anchors TreeRow property-control children to the fixed header line", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeRow>
            <Textarea id="tooltip.manual" value="Long value" showLabel />
          </TreeRow>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-row"');
      expect(markup).toContain('data-tree-row-kind="property"');
      expect(markup).toContain('data-slot="property-row"');
      expect(markup).toContain('data-slot="tree-branch-elbow" class="pointer-events-none absolute h-px bg-muted-foreground/40 -translate-y-1/2 transition-[height,background-color] duration-150" style="top:12px;left:7px;width:10px"');
      expect(markup).not.toContain('style="top:50%;left:7px;width:10px"');
    });

    it("marks unlabeled non-property TreeRow wrappers as content rows", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeRow>
            <span>Note</span>
          </TreeRow>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-row"');
      expect(markup).toContain('data-tree-row-kind="content"');
    });

    it("renders property-layout tree items with a dedicated control column", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeItem id="tooltip.manual" layoutKind="property" defaultOpen={true}>
            <Label id="tooltip.manual">
              <span>Control</span>
            </Label>
          </TreeItem>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-property-item"');
      expect(markup).toContain('data-slot="tree-row-content"');
      expect(markup).toContain('class="flex items-center gap-[6px] h-[22px] min-w-0 w-full"');
      expect(markup).toContain('data-slot="tree-row-layout"');
      expect(markup).toContain('data-slot="tree-gutter"');
      expect(markup).toContain("grid-template-columns:14px minmax(0, 1fr)");
      expect(markup).toContain("column-gap:6px");
      expect(markup).toContain('data-slot="tree-property-content"');
      expect(markup).not.toContain('data-slot="tree-header-actions"');
      expect(markup).toContain('data-slot="property-row"');
    });

    it("keeps leaf and expandable sibling rows on the same gutter rhythm", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeSection id="tooltip.manual" defaultOpen={true}>
            <TreeItem id="tooltip.tutorial" defaultOpen={true}>
              <TreeContent>
                <span>Nested content</span>
              </TreeContent>
            </TreeItem>
            <TreeItem id="tooltip.docs" />
          </TreeSection>
        </TreeContext.Provider>,
      );

      expect(markup.match(/grid-template-columns:24px minmax\(0, 1fr\)/g)?.length ?? 0).toBe(2);
      expect(markup).not.toContain("margin-left:-10px");
      expect(markup).not.toContain("padding-left:10px");
    });

    it("renders steppers at full control width with the current numeric value visible", () => {
      const markup = renderToStaticMarkup(<Stepper id="semio.sketchpad.app.design.panel.details.section.connection.x" value={12.5} />);

      expect(markup).toContain('data-slot="stepper-group"');
      expect(markup).toContain('data-detail-panel-control="fill"');
      expect(markup).toContain('data-stepper-input="true"');
      expect(markup).toContain("w-full");
      expect(markup).toContain("min-w-0");
      expect(markup).toContain('value="12.5"');
    });

    it("renders shared field roots that stretch within the property value column", () => {
      const inputMarkup = renderToStaticMarkup(<Input id="tooltip.manual" value="value" />);
      const textareaMarkup = renderToStaticMarkup(<Textarea id="tooltip.manual" value="value" />);

      expect(inputMarkup).toContain('data-slot="input-root"');
      expect(inputMarkup).toContain('data-detail-panel-control="fill"');
      expect(inputMarkup).toContain("flex min-w-0 w-full flex-1 items-stretch");
      expect(textareaMarkup).toContain('data-slot="textarea-root"');
      expect(textareaMarkup).toContain('data-detail-panel-control="fill"');
      expect(textareaMarkup).toContain("flex min-w-0 w-full flex-1 items-stretch");
    });

    it("anchors fit-content button and toggle controls to the shared property edge", () => {
      const buttonMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Button text="Apply" />
        </Label>,
      );
      const toggleMarkup = renderToStaticMarkup(<Toggle id="tooltip.manual" icon={<CheckIcon />} showLabel />);

      expect(buttonMarkup).toContain('data-slot="property-control"');
      expect(buttonMarkup).toContain("justify-end");
      expect(buttonMarkup).toContain('data-slot="button-group"');
      expect(buttonMarkup).toContain('data-detail-panel-control="fit"');
      expect(buttonMarkup).toContain("w-fit shrink-0");
      expect(toggleMarkup).toContain('data-slot="property-control"');
      expect(toggleMarkup).toContain("justify-end");
      expect(toggleMarkup).toContain('data-slot="toggle-group"');
      expect(toggleMarkup).toContain('data-detail-panel-control="fit"');
      expect(toggleMarkup).toContain("w-fit shrink-0");
    });

    it("renders ring inside tree-aligned property row with label and fit control", () => {
      const ringMarkup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 1, isLastAtLevel: [true], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeRowAlignmentContext.Provider value={true}>
            <div data-slot="tree-row">
              <TreeAlignedRow level={1} isLastAtLevel={[true]} showLines={true} connectCurrentLevel={true} contentClassName="min-w-0">
                <Ring id="semio.sketchpad.app.type.panel.details.section.connectors.ring" orbs={[{ id: "connector-1", t: 0.25, selected: true }]} showLabel />
              </TreeAlignedRow>
            </div>
          </TreeRowAlignmentContext.Provider>
        </TreeContext.Provider>,
      );

      expect(ringMarkup).toContain('data-slot="tree-row-layout"');
      expect(ringMarkup).toContain('data-slot="tree-gutter"');
      expect(ringMarkup).toContain('data-slot="property-row"');
      expect(ringMarkup).toContain('data-slot="property-label"');
      expect(ringMarkup).toContain('data-slot="property-control"');
      expect(ringMarkup).toContain('data-slot="ring"');
      expect(ringMarkup).toContain('data-detail-panel-control="fit"');
      expect(ringMarkup).toContain("w-fit shrink-0");
      expect(ringMarkup).toContain('id="semio.sketchpad.app.type.panel.details.section.connectors.ring-label"');
      expect(ringMarkup).toContain(">Ring<");
    });

    it("marks combobox and select triggers as fill-width detail controls", () => {
      const comboboxMarkup = renderToStaticMarkup(
        <Combobox
          id="tooltip.manual"
          showLabel
          value="alpha"
          onValueChange={() => undefined}
          options={[
            { label: "Alpha", value: "alpha" },
            { label: "Beta", value: "beta" },
          ]}
        />,
      );
      const selectMarkup = renderToStaticMarkup(
        <Select id="tooltip.manual" showLabel defaultValue="alpha">
          <SelectTrigger>
            <SelectValue placeholder="Select" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="alpha">Alpha</SelectItem>
          </SelectContent>
        </Select>,
      );

      expect(comboboxMarkup).toContain("group/button-group");
      expect(comboboxMarkup).toContain('data-detail-panel-control="fill"');
      expect(comboboxMarkup).toContain('role="combobox"');
      expect(selectMarkup).toContain('data-slot="select-trigger"');
      expect(selectMarkup).toContain('data-detail-panel-control="fill"');
    });

    it("renders section and item content slots flush under their headers", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeSection id="tooltip.manual" defaultOpen={true}>
            <TreeItem id="tooltip.tutorial" defaultOpen={true}>
              <Label id="tooltip.manual">
                <span>Control</span>
              </Label>
            </TreeItem>
          </TreeSection>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-section-content"');
      expect(markup).toContain('data-slot="tree-item-content"');
      expect(markup).toContain('data-slot="tree-section-content" data-tree-owner-kind="section" data-tree-owner-expanded="true" class="relative flex min-w-0 flex-col" style="row-gap:0px"');
      expect(markup).toContain('data-slot="tree-item-content" data-tree-owner-kind="group" data-tree-owner-expanded="true" class="relative flex min-w-0 flex-col" style="row-gap:0px"');
      expect(markup).not.toContain("padding-top:6px");
      expect(markup).not.toContain("padding-top:2px");
      expect(markup).not.toContain("margin-bottom:12px");
    });

    it("keeps guide wrappers continuous and pushes labels farther from the guide stroke", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeSection id="tooltip.manual" defaultOpen={true}>
            <TreeItem id="tooltip.tutorial" defaultOpen={true}>
              <TreeContent>
                <span>Nested content</span>
              </TreeContent>
              <TreeItem id="tooltip.docs">
                <TreeContent>
                  <span>Leaf content</span>
                </TreeContent>
              </TreeItem>
            </TreeItem>
          </TreeSection>
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-content" data-tree-row-kind="content" class="relative"');
      expect(markup).toContain('data-slot="tree-gutter"');
      expect(markup).toContain('data-slot="tree-branch-elbow"');
      expect(markup).toContain('data-slot="tree-gutter-slot"');
      expect(markup).toContain("grid-template-columns:24px minmax(0, 1fr)");
      expect(markup).toContain("grid-template-columns:34px minmax(0, 1fr)");
      expect(markup).toContain("grid-template-columns:44px minmax(0, 1fr)");
      expect(markup).toContain("column-gap:6px");
      expect(markup).not.toMatch(/data-slot="tree-gutter"[^>]*><div class="absolute left-0 top-0 bottom-0 pointer-events-none"/);
      expect(markup).not.toContain('data-slot="tree-gutter-slot" class="absolute inset-y-0 left-0 flex items-center justify-center"');
      expect(markup).toContain('data-slot="tree-gutter-slot"');
      expect(markup).toContain('class="absolute -translate-y-1/2');
      expect(markup).toContain('style="top:50%;left:0px"');
      expect(markup).toContain('data-slot="tree-branch-elbow" class="pointer-events-none absolute h-px bg-muted-foreground/40 -translate-y-1/2 transition-[height,background-color] duration-150" style="top:50%;left:7px;width:3px"');
      expect(markup).toContain('data-slot="tree-branch-stem"');
      expect(markup.match(/data-tree-guide-line="" class="w-px h-full bg-muted-foreground\/40/g)?.length ?? 0).toBeGreaterThanOrEqual(3);
      expect(markup).not.toContain('data-slot="tree-content" class="relative" style="padding-top:3px;padding-bottom:3px;padding-left:');
      expect(markup).not.toContain('data-slot="tree-property-label" class="relative min-w-0" style="padding-left:');
      expect(markup).toContain('data-slot="tree-section-content"');
      expect(markup).toContain('data-slot="tree-item-content" data-tree-owner-kind="group" data-tree-owner-expanded="true" class="relative flex min-w-0 flex-col"');
    });

    it("renders sortable drag handles without bordered action chrome", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeItem id="tooltip.manual" sortable={true} sortableId="sortable-manual" isDragHandle={true} />
        </TreeContext.Provider>,
      );

      const handleClassName = markup.match(/data-slot="tree-drag-handle"[^>]*class="([^"]+)"/)?.[1] ?? "";
      expect(markup).toContain('data-slot="tree-drag-handle"');
      expect(handleClassName).toContain("cursor-grab");
      expect(handleClassName).toContain("border-0");
      expect(handleClassName).not.toContain("hover:bg-hover");
      expect(markup).not.toContain('data-slot="tree-drag-handle" class="text-foreground');
    });

    it("renders control-tree folder branches inside the same continuous guide wrapper", () => {
      const markup = renderToStaticMarkup(
        <ControlTree
          controls={[
            {
              path: "Folder/Value",
              controlKind: "number",
              value: 3,
              onChange: () => undefined,
            },
          ]}
        />,
      );

      expect(markup).toContain('data-slot="control-tree-folder-content" data-tree-owner-expanded="false" class="relative flex min-w-0 flex-col"');
      expect(markup).toContain('data-slot="control-tree-folder-label"');
      expect(markup).toContain('data-slot="control-tree-control-label"');
      expect(markup).toContain('data-slot="tree-row-layout"');
      expect(markup).toContain('data-slot="tree-gutter"');
      expect(markup).toContain("grid-template-columns:14px minmax(0, 1fr)");
      expect(markup).toContain("grid-template-columns:24px minmax(0, 1fr)");
      expect(markup).toContain("column-gap:6px");
      expect(markup).not.toContain("margin-left:13px");
    });

    it("truncates collapsed field text on word boundaries before falling back to characters", () => {
      const measureText = (value: string) => value.length * 8;

      expect(
        fitCollapsedFieldText({
          value: "Alpha beta gamma delta",
          maxWidth: measureText("Alpha beta..."),
          measureText,
        }),
      ).toBe("Alpha beta...");

      expect(
        fitCollapsedFieldText({
          value: "Supercalifragilisticexpialidocious",
          maxWidth: measureText("Supercali..."),
          measureText,
        }),
      ).toBe("Supercali...");
    });

    it("uses stacked overflow when enabled and inline ellipsis when disabled", () => {
      const measureText = (value: string) => value.length * 8;
      const stackedState = resolveCollapsedFieldDisplayState({
        allowStackedOverflow: true,
        value: "Alpha beta gamma delta",
        maxWidth: measureText("Alpha beta gamma"),
        measureText,
      });
      const inlineState = resolveCollapsedFieldDisplayState({
        value: "Alpha beta gamma delta",
        maxWidth: measureText("Alpha beta gamma"),
        measureText,
      });

      expect(stackedState.value).toBe("Alpha beta gamma");
      expect(stackedState.isOverflowing).toBe(true);
      expect(stackedState.layoutKind).toBe("stacked-overflow");
      expect(stackedState.value.endsWith(COLLAPSED_FIELD_ELLIPSIS)).toBe(false);

      expect(inlineState.value).toBe("Alpha beta...");
      expect(inlineState.isOverflowing).toBe(true);
      expect(inlineState.layoutKind).toBe("single-line");
      expect(inlineState.value.endsWith(COLLAPSED_FIELD_ELLIPSIS)).toBe(true);
    });

    it("keeps single-line text fields in the normal state when the text still fits", () => {
      const measureText = (value: string) => value.length * 8;
      const fittingState = resolveCollapsedFieldDisplayState({
        allowStackedOverflow: true,
        value: "Nakagin Capsule Tower",
        maxWidth: measureText("Nakagin Capsule Tower"),
        measureText,
      });

      expect(fittingState.isOverflowing).toBe(false);
      expect(fittingState.layoutKind).toBe("single-line");
      expect(fittingState.value).toBe("Nakagin Capsule Tower");
    });

    it("enables stacked overflow only after the rendered value exceeds the inner field width", () => {
      const measureText = (value: string) => value.length * 8;
      const exactFitState = resolveCollapsedFieldDisplayState({
        allowStackedOverflow: true,
        value: "Nakagin Capsule Tower",
        maxWidth: measureText("Nakagin Capsule Tower"),
        measureText,
      });
      const overflowingState = resolveCollapsedFieldDisplayState({
        allowStackedOverflow: true,
        value: "Nakagin Capsule Tower",
        maxWidth: measureText("Nakagin Capsule Towe"),
        measureText,
      });

      expect(exactFitState.isOverflowing).toBe(false);
      expect(exactFitState.layoutKind).toBe("single-line");
      expect(overflowingState.isOverflowing).toBe(true);
      expect(overflowingState.layoutKind).toBe("stacked-overflow");
      expect(overflowingState.value).toBe("Nakagin Capsule");
    });

    it("keeps tree section actions inline with the header row when isTree is true", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeSection id="tooltip.manual" defaultOpen={false} actions={[{ icon: <span data-testid="add-icon" />, onClick: () => undefined }]} />
        </TreeContext.Provider>,
      );

      expect(markup).toContain('class="flex items-center gap-[6px] min-w-0 w-full"');
      expect(markup).toContain('data-slot="tree-header-actions"');
      expect(markup).not.toContain('data-slot="property-control"');
      const rowContentIdx = markup.indexOf('data-slot="tree-row-content"');
      const actionsIdx = markup.indexOf('data-testid="add-icon"');
      expect(rowContentIdx).toBeGreaterThan(-1);
      expect(actionsIdx).toBeGreaterThan(-1);
      expect(actionsIdx).toBeGreaterThan(rowContentIdx);
    });

    it("keeps tree item actions inline with the header row when isTree is true", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeItem id="tooltip.manual" actions={[{ icon: <span data-testid="remove-icon" />, onClick: () => undefined }]} />
        </TreeContext.Provider>,
      );

      expect(markup).toContain('class="flex items-center gap-[6px] min-w-0 w-full"');
      expect(markup).toContain('data-slot="tree-header-actions"');
      expect(markup).not.toContain('data-slot="property-control"');
      expect(markup).toContain('data-testid="remove-icon"');
    });

    it("uses the same inline tree header actions when isTree is false", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false, isTree: false, indentMultiplier: 1 }}>
          <TreeItem id="tooltip.manual" actions={[{ icon: <span data-testid="add-icon" />, onClick: () => undefined }]} />
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-header-actions"');
      expect(markup).not.toContain('data-slot="property-control"');
      expect(markup).toContain('data-testid="add-icon"');
    });

    it("renders checkbox actions inline with tree headers", () => {
      const markup = renderToStaticMarkup(
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
          <TreeItem
            id="tooltip.manual"
            actions={[
              {
                kind: "checkbox",
                id: "tree-checkbox-action",
                checked: true,
                title: "Toggle item",
                onCheckedChange: () => undefined,
              },
            ]}
          />
        </TreeContext.Provider>,
      );

      expect(markup).toContain('data-slot="tree-header-actions"');
      expect(markup).toContain('data-slot="tree-action-checkbox-wrapper"');
      expect(markup).toContain('data-slot="tree-action-checkbox"');
      expect(markup).toContain('id="tree-checkbox-action"');
      expect(markup).toContain('type="checkbox"');
      expect(markup).toContain('checked=""');
      expect(markup).toContain('aria-label="Toggle item"');
    });

    it("renders empty Input inside a Label property row with muted opacity and full opacity when value is present", () => {
      const emptyMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Input id="tooltip.manual" value="" />
        </Label>,
      );
      const filledMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Input id="tooltip.manual" value="hello" />
        </Label>,
      );
      const standaloneMarkup = renderToStaticMarkup(<Input id="tooltip.manual" value="" />);

      expect(emptyMarkup).toContain('data-slot="input-root"');
      expect(emptyMarkup).toContain("opacity:0.6");
      expect(filledMarkup).toContain("opacity:1");
      // outside Label (not in property value column) — no muted opacity
      expect(standaloneMarkup).not.toContain("opacity:0.6");
    });

    it("renders empty Textarea inside a Label property row with muted opacity and full opacity when value is present", () => {
      const emptyMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Textarea id="tooltip.manual" value="" />
        </Label>,
      );
      const filledMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Textarea id="tooltip.manual" value="some text" />
        </Label>,
      );
      const standaloneMarkup = renderToStaticMarkup(<Textarea id="tooltip.manual" value="" />);

      expect(emptyMarkup).toContain('data-slot="textarea-root"');
      expect(emptyMarkup).toContain("opacity:0.6");
      expect(filledMarkup).toContain("opacity:1");
      expect(standaloneMarkup).not.toContain("opacity:0.6");
    });

    it("renders empty Combobox inside a Label property row with muted opacity and full opacity when value is selected", () => {
      const options = [
        { label: "Alpha", value: "alpha" },
        { label: "Beta", value: "beta" },
      ];
      const emptyMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Combobox id="tooltip.manual" value="" options={options} onValueChange={() => undefined} />
        </Label>,
      );
      const filledMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Combobox id="tooltip.manual" value="alpha" options={options} onValueChange={() => undefined} />
        </Label>,
      );
      const standaloneMarkup = renderToStaticMarkup(<Combobox id="tooltip.manual" value="" options={options} onValueChange={() => undefined} />);

      // PopoverTrigger asChild merges ButtonGroup — check class presence instead of data-slot
      expect(emptyMarkup).toContain("group/button-group");
      expect(emptyMarkup).toContain("opacity:0.6");
      expect(filledMarkup).toContain("opacity:1");
      expect(standaloneMarkup).not.toContain("opacity:0.6");
    });

    it("renders Stepper with undefined value inside a Label property row with muted opacity and full opacity when value is defined", () => {
      const emptyMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.x" value={undefined} />
        </Label>,
      );
      const filledMarkup = renderToStaticMarkup(
        <Label id="tooltip.manual">
          <Stepper id="semio.sketchpad.app.design.panel.details.section.connection.x" value={5} />
        </Label>,
      );
      const standaloneMarkup = renderToStaticMarkup(<Stepper id="semio.sketchpad.app.design.panel.details.section.connection.x" value={undefined} />);

      expect(emptyMarkup).toContain('data-slot="stepper-group"');
      expect(emptyMarkup).toContain("opacity:0.6");
      expect(filledMarkup).toContain("opacity:1");
      expect(standaloneMarkup).not.toContain("opacity:0.6");
    });
  });

  describe("scene helpers", () => {
    it("maps dominant gizmo axes to blender-style orthographic snap targets", () => {
      expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(1, 0.2, 0.1))).toEqual({
        axis: "x",
        sign: 1,
        view: "side",
        cameraDirection: { x: 1, y: 0, z: 0 },
        up: { x: 0, y: 1, z: 0 },
      });

      expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0.1, 1, 0.2))).toEqual({
        axis: "y",
        sign: 1,
        view: "top",
        cameraDirection: { x: 0, y: 1, z: 0 },
        up: { x: 0, y: 0, z: -1 },
      });

      expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0.1, 0.2, -1))).toEqual({
        axis: "z",
        sign: -1,
        view: "back",
        cameraDirection: { x: 0, y: 0, z: -1 },
        up: { x: 0, y: 1, z: 0 },
      });
    });

    it("preserves the complementary blender views for negative axis clicks", () => {
      expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(-1, 0, 0))).toEqual({
        axis: "x",
        sign: -1,
        view: "opposite-side",
        cameraDirection: { x: -1, y: 0, z: 0 },
        up: { x: 0, y: 1, z: 0 },
      });

      expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0, -1, 0))).toEqual({
        axis: "y",
        sign: -1,
        view: "bottom",
        cameraDirection: { x: 0, y: -1, z: 0 },
        up: { x: 0, y: 0, z: 1 },
      });

      expect(resolveSceneGizmoSnapTarget(new THREE.Vector3(0, 0, 1))).toEqual({
        axis: "z",
        sign: 1,
        view: "front",
        cameraDirection: { x: 0, y: 0, z: 1 },
        up: { x: 0, y: 1, z: 0 },
      });
    });

    it("keeps the gizmo in the bottom-right corner with a larger inset so it stays visible", () => {
      expect(resolveSceneGizmoViewportPlacement({ width: 1280, height: 720 })).toEqual({
        alignment: "bottom-right",
        margin: [56, 40],
      });

      expect(resolveSceneGizmoViewportPlacement({ width: 120, height: 160 })).toEqual({
        alignment: "bottom-right",
        margin: [26, 22],
      });

      expect(resolveSceneGizmoViewportPlacement({ width: 40, height: 48 })).toEqual({
        alignment: "bottom-right",
        margin: [26, 18],
      });
    });
  });

  describe("sketchpad kit i18n", () => {
    function resourceAt(path: string): unknown {
      const tr = elementUiTranslationBundles.en.translation as Record<string, unknown>;
      return path.split(".").reduce<unknown>((acc, k) => (acc && typeof acc === "object" ? (acc as Record<string, unknown>)[k] : undefined), tr);
    }

    it("defines kit-level tag, tags, concept, and concepts strings used by sketchpad", () => {
      expect(resourceAt("semio.sketchpad.app.kit.tags.multipleTitle")).toBeDefined();
      expect(resourceAt("semio.sketchpad.app.kit.tag.descriptionPlaceholder.label")).toBeDefined();
      expect(resourceAt("semio.sketchpad.app.kit.concept.descriptionPlaceholder.label")).toBeDefined();
      expect(resourceAt("semio.sketchpad.app.kit.concepts.multipleSelected")).toBeDefined();
    });
  });
}
