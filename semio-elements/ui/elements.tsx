// #region Header

// semio-elements/ui/elements.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { closestCenter, DndContext, DragEndEvent, PointerSensor, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
import { SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
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
import { Slot } from "@radix-ui/react-slot";
import * as TabsPrimitive from "@radix-ui/react-tabs";
import * as TogglePrimitive from "@radix-ui/react-toggle";
import * as ToggleGroupPrimitive from "@radix-ui/react-toggle-group";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import { Edges, GizmoHelper, GizmoViewport, Grid, OrbitControls, useGLTF } from "@react-three/drei";
import { Canvas as ThreeCanvas, ThreeEvent, useFrame, useThree } from "@react-three/fiber";
import {
  AddIcon,
  AlertCircleIcon,
  BookIcon,
  CameraIcon,
  CheckIcon,
  CheckIconAlt,
  ChevronDownIcon,
  ChevronDownIconAlt,
  ChevronLeftIcon,
  ChevronRightIcon,
  ChevronsUpDownIcon,
  CloseIcon,
  CloseIconAlt,
  DocumentIcon,
  ExternalLinkIcon,
  FolderIcon,
  GripVerticalIcon,
  InfoIcon,
  LightbulbIcon,
  LucideIcon,
  Maximize2Icon,
  Minimize2Icon,
  RemoveIcon,
  SearchIcon,
  TriangleAlertIcon,
  TutorialIcon,
} from "@semio/assets";
import type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance } from "@xyflow/react";
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
import "@xyflow/react/dist/style.css";
import { cva, type VariantProps } from "class-variance-authority";
import { Command as CommandPrimitive } from "cmdk";
import Fuse, { type FuseResult } from "fuse.js";
import { forceCenter, forceCollide, forceLink, forceManyBody, forceSimulation, forceX, forceY, Simulation, SimulationLinkDatum, SimulationNodeDatum } from "d3-force";
import * as dagre from "dagre";
import * as React from "react";
import { createPortal } from "react-dom";
import { createRoot } from "react-dom/client";
import { useTranslation } from "react-i18next";
import * as ResizablePrimitive from "react-resizable-panels";
import { Link, useNavigate } from "react-router";
import * as THREE from "three";
import { ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

// #endregion Imports

// #region Utilities

// Generic utility and type definitions that make semio-elements/ui self-contained.
// These MUST NOT depend on any external semio package.

/**
 * Merges CSS class names using Tailwind merge.
 **/
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Expertise levels for label resolution.
 **/
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

// #endregion Utilities

// #region Section Specificity

// [👤semio📚js🗃️sketchpad💻elementstsx🔖sectionspecificity](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/SECTION-SPECIFICITY)
// Enum defining priority levels for section content ownership.
// Consumers MUST use these constants for section precedence.

/**
 * Priority enum for section content ownership across apps.
 * [👤semio📚js🗃️sketchpad💻elements🔖sectionspecificity🛠️sectionspecificity](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Section%20Specificity/d/i/SectionSpecificity)
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

// #endregion Section Specificity

// #region Interaction Context

// [👤semio📚js🗃️sketchpad💻elementstsx🔖interactioncontext](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/INTERACTION-CONTEXT)
// React context for tracking active UI interactions.
// Consumers MUST wrap interactive elements with InteractionProvider.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext✂️interactioncommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/InteractionCommands)
 * InteractionCommands holds the data fields for a InteractionCommands record.
 **/
interface InteractionCommands {
  setActiveInteraction: (elementId?: string, interactionId?: string) => void;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext🪨interactioncontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/InteractionContext)
 * InteractionContext holds the data fields for a InteractionContext record.
 **/
const InteractionContext = React.createContext<InteractionCommands | undefined>(undefined);
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext🪨activeinteractioncontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/ActiveInteractionContext)
 * ActiveInteractionContext holds the data fields for a ActiveInteractionContext record.
 **/
const ActiveInteractionContext = React.createContext<string | undefined>(undefined);

/**
 * Context provider for UI interaction commands and active state.
 * [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext🪨interactionprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/InteractionProvider)
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

// [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext🪨useinteractioncommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/useInteractionCommands)
/**
 * useInteractionCommands holds the data fields for a useInteractionCommands record.
 **/
const useInteractionCommands = () => React.useContext(InteractionContext);
/** useActiveInteraction holds the data fields for a useActiveInteraction record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext🪨useactiveinteraction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/useActiveInteraction)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖interactioncontext🪨useactiveinteraction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Interaction%20Context/d/i/useActiveInteraction)
 **/
const useActiveInteraction = () => React.useContext(ActiveInteractionContext);

// #endregion Interaction Context

// #region Level Context

// [👤semio📚js🗃️sketchpad💻elementstsx🔖levelcontext](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/LEVEL-CONTEXT)
// React context for UI depth level tracking.
// Consumers MUST wrap components with LevelProvider.

/**
 * Union type for UI depth levels.
 * [👤semio📚js🗃️sketchpad💻elements🔖levelcontext🛠️level](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Level%20Context/d/i/Level)
 **/
export type Level = "base" | "window" | "panel" | "overlay" | "temporary";

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖levelcontext🪨levelcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Level%20Context/d/i/LevelContext)
 * LevelContext holds the data fields for a LevelContext record.
 **/
const LevelContext = React.createContext<Level>("base");

/**
 * Context provider that sets the current UI level.
 * [👤semio📚js🗃️sketchpad💻elements🔖levelcontext🪨levelprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Level%20Context/d/i/LevelProvider)
 **/
export const LevelProvider: React.FC<{
  level: Level;
  children: React.ReactNode;
}> = ({ level, children }) => {
  return <LevelContext.Provider value={level}>{children}</LevelContext.Provider>;
};

/**
 * Hook returning the current UI depth level.
 * [👤semio📚js🗃️sketchpad💻elements🔖levelcontext🪨uselevel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Level%20Context/d/i/useLevel)
 **/
export const useLevel = () => React.useContext(LevelContext);

// #endregion Level Context

// #region Element

// [👤semio📚js🗃️sketchpad💻elementstsx🔖element](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/ELEMENT)
// Core element types, transaction context, and level-based CSS class helpers.
// Consumers MUST use level functions for consistent styling.

/**
 * Interface for start/finalize/abort lifecycle of a UI transaction.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🛠️transaction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/Transaction)
 **/
export interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

/**
 * TransactionContext holds the data fields for a TransactionContext record.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨transactioncontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/TransactionContext)
 **/
const TransactionContext = React.createContext<Transaction | undefined>(undefined);

/**
 * Context provider that supplies a Transaction to descendants.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨transactionprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/TransactionProvider)
 **/
export const TransactionProvider: React.FC<{
  transaction?: Transaction;
  children: React.ReactNode;
}> = ({ transaction, children }) => {
  return <TransactionContext.Provider value={transaction}>{children}</TransactionContext.Provider>;
};

/**
 * Hook returning the current Transaction context.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨usetransaction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/useTransaction)
 **/
export const useTransaction = (): Transaction | undefined => React.useContext(TransactionContext);

/**
 * Base props interface requiring an id string.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🛠️elementbaseprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/ElementBaseProps)
 **/
export interface ElementBaseProps {
  id: string;
}

export interface ElementProps extends ElementBaseProps { }

/**
 * Returns the Tailwind background class for a given level.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨getlevelbgclass](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/getLevelBgClass)
 **/
export const getLevelBgClass = (level: Level): string => {
  switch (level) {
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
};

/**
 * Returns the Tailwind hover background class for a given level.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨getlevelhoverclass](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/getLevelHoverClass)
 **/
export const getLevelHoverClass = (level: Level): string => {
  switch (level) {
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
};

/**
 * Returns the Tailwind active-state hover class for a given level.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨getlevelactivehoverclass](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/getLevelActiveHoverClass)
 **/
export const getLevelActiveHoverClass = (level: Level): string => {
  switch (level) {
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
};

/**
 * Returns the Tailwind z-index class for a given level.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨getlevelzclass](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/getLevelZClass)
 **/
export const getLevelZClass = (level: Level): string => {
  switch (level) {
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
};

/**
 * Returns the Tailwind border class for a given level.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨getlevelborderelementclass](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/getLevelBorderElementClass)
 **/
export const getLevelBorderElementClass = (level: Level): string => {
  switch (level) {
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
};

/**
 * Returns the Tailwind divide class for a given level.
 * [👤semio📚js🗃️sketchpad💻elements🔖element🪨getleveldivideelementclass](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Element/d/i/getLevelDivideElementClass)
 **/
export const getLevelDivideElementClass = (level: Level): string => {
  switch (level) {
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
};

// #endregion Element

// #region Command

// [👤semio📚js🗃️sketchpad💻elementstsx🔖command](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/COMMAND)
// Command palette UI built on cmdk primitives.
// Consumers MUST use CommandInput for search functionality.

/**
 * Command holds the data fields for a Command record.
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️command](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/Command)
 **/
function Command({ className, ...props }: React.ComponentProps<typeof CommandPrimitive>) {
  return <CommandPrimitive data-slot="command" className={cn("bg-popover text-popover-foreground flex h-full w-full flex-col overflow-hidden", className)} {...props} />;
}

/**
 * CommandDialog holds the data fields for a CommandDialog record.
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commanddialog](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandDialog)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commandinput](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandInput)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commandlist](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandList)
 * CommandList holds the data fields for a CommandList record.
 **/
function CommandList({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.List>) {
  return <CommandPrimitive.List data-slot="command-list" className={cn("max-h-[300px] scroll-py-single overflow-x-hidden overflow-y-auto", className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commandempty](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandEmpty)
 * CommandEmpty holds the data fields for a CommandEmpty record.
 **/
function CommandEmpty({ ...props }: React.ComponentProps<typeof CommandPrimitive.Empty>) {
  return <CommandPrimitive.Empty data-slot="command-empty" className="py-medium text-center text-sm" {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commandgroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandGroup)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commanditem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandItem)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖command🛠️commandshortcut](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Command/d/i/CommandShortcut)
 * CommandShortcut holds the data fields for a CommandShortcut record.
 **/
function CommandShortcut({ className, ...props }: React.ComponentProps<"span">) {
  return <span data-slot="command-shortcut" className={cn("text-muted-foreground ml-auto text-xs tracking-widest", className)} {...props} />;
}

// #endregion Command

export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };

// #region Footer

// [👤semio📚js🗃️sketchpad💻elementstsx🔖footer](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/FOOTER)
// Status bar component at the bottom of the layout.
// Consumers MUST provide FooterItem entries for each action.

/**
 * Configuration interface for a single footer action item.
 * [👤semio📚js🗃️sketchpad💻elements🔖footer🛠️footeritem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Footer/d/i/FooterItem)
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
 *[👤semio📚js🗃️sketchpad💻elements🔖footer🛠️footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Footer/d/i/Footer)
 **/
export interface FooterProps {
  items?: FooterItem[];
  className?: string;
  isVisible?: boolean;
}

// [👤semio📚js🗃️sketchpad💻elements🔖footer🪨footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Footer/d/i/Footer)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖footer🪨footer](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Footer/d/i/Footer)
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

// #endregion Footer

// #region Layout

// [👤semio📚js🗃️sketchpad💻elementstsx🔖layout](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/LAYOUT)
// Top-level layout orchestrating navbar, panels, canvas, and footer.
// Consumers MUST provide a canvas element.

/**
 * Props interface for the top-level Layout component.
 * [👤semio📚js🗃️sketchpad💻elements🔖layout🛠️layoutprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Layout/d/i/LayoutProps)
 **/
export interface LayoutProps {
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  bottomPanel?: BottomPanelProps;
  leftSidePanel?: SidePanelProps;
  rightSidePanel?: SidePanelProps;
  canvas: React.ReactNode;
  toolbar?: React.ReactNode;
  className?: string;
}

const Layout: React.FC<LayoutProps> = ({ navbar, footer, bottomPanel, leftSidePanel, rightSidePanel, canvas, toolbar, className = "" }) => (
  <div className={`flex flex-col h-screen w-screen overflow-hidden ${className}`}>
    {navbar && <div className="flex-shrink-0">{navbar}</div>}
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
    {(footer || toolbar) && (
      <div className="flex-shrink-0 relative">
        {toolbar && <div className="absolute bottom-[calc(100%+var(--spacing-double))] left-1/2 -translate-x-1/2 z-panel pointer-events-none">{toolbar}</div>}
        {footer}
      </div>
    )}
  </div>
);

export { Layout };

// #endregion Layout

// #region Popover

// [👤semio📚js🗃️sketchpad💻elementstsx🔖popover](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/POPOVER)
// Floating popover component built on Radix primitives.

/**
 * Popover holds the data fields for a Popover record.
 * [👤semio📚js🗃️sketchpad💻elements🔖popover🛠️popover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Popover/d/i/Popover)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖popover🛠️popover](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Popover/d/i/Popover)
 * Popover holds the data fields for a Popover record.
 **/
function Popover({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root data-slot="popover" {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖popover🛠️popovertrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Popover/d/i/PopoverTrigger)
 * PopoverTrigger holds the data fields for a PopoverTrigger record.
 **/
function PopoverTrigger({ className, ...props }: React.ComponentProps<typeof PopoverPrimitive.Trigger>) {
  return <PopoverPrimitive.Trigger data-slot="popover-trigger" className={cn(className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖popover🛠️popovercontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Popover/d/i/PopoverContent)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖popover🛠️popoveranchor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Popover/d/i/PopoverAnchor)
 * PopoverAnchor holds the data fields for a PopoverAnchor record.
 **/
function PopoverAnchor({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Anchor>) {
  return <PopoverPrimitive.Anchor data-slot="popover-anchor" {...props} />;
}

export { Popover, PopoverAnchor, PopoverContent, PopoverTrigger };

// #endregion Popover

// #region Tooltip

// [👤semio📚js🗃️sketchpad💻elements🔖tooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip)
// Tooltip components with expertise-level adaptive content.
// Consumers MUST configure the expertise mode provider.

/**
 * Configuration for enhanced tooltip with label, paths, and hotkey.
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️tooltipconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/TooltipConfig)
 **/
export interface TooltipConfig {
  labelKey: string;
  manualPath?: string;
  tutorialPath?: string;
  hotkey?: string;
}

/**
 * Data interface for description-based tooltip content.
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️descriptiontooltipdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/DescriptionTooltipData)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️settooltipmodeprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/setTooltipModeProvider)
 **/
export function setTooltipModeProvider(fn: () => Expertise) {
  setExpertiseProvider(fn);
}

/**
 * Hook returning the current expertise level for tooltips.
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️usetooltipmode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/useTooltipMode)
 **/
export function useTooltipMode(): Expertise {
  if (!_expertiseProvider) return Expertise.BEGINNER;
  return _expertiseProvider();
}

/**
 * TooltipProvider holds the data fields for a TooltipProvider record.
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️tooltipprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/TooltipProvider)
 **/
function TooltipProvider({ delayDuration = 400, ...props }: React.ComponentProps<typeof TooltipPrimitive.Provider>) {
  return <TooltipPrimitive.Provider data-slot="tooltip-provider" delayDuration={delayDuration} {...props} />;
}

/**
 * Tooltip holds the data fields for a Tooltip record.
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️tooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/Tooltip)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️tooltiptrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/TooltipTrigger)
 **/
function TooltipTrigger({ className, asChild, ...props }: React.ComponentProps<typeof TooltipPrimitive.Trigger>) {
  return <TooltipPrimitive.Trigger data-slot="tooltip-trigger" asChild={asChild} className={cn(className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️tooltipcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/TooltipContent)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip✂️enhancedtooltipcontentprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/EnhancedTooltipContentProps)
 * EnhancedTooltipContentProps holds the data fields for a EnhancedTooltipContentProps record.
 **/
interface EnhancedTooltipContentProps {
  config: TooltipConfig;
}

/** EnhancedTooltipContent holds the data fields for a EnhancedTooltipContent record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️enhancedtooltipcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/EnhancedTooltipContent)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🪨enhancedtooltipcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/EnhancedTooltipContent)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip✂️descriptiontooltipcontentprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/DescriptionTooltipContentProps)
 * DescriptionTooltipContentProps holds the data fields for a DescriptionTooltipContentProps record.
 **/
interface DescriptionTooltipContentProps {
  id: string;
}

// [👤semio📚js🗃️sketchpad💻elements🔖tooltip🛠️descriptiontooltipcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/DescriptionTooltipContent)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tooltip🪨descriptiontooltipcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tooltip/d/i/DescriptionTooltipContent)
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
  const label = typeof value === "string" ? value : typeof value === "object" && value?.label ? value.label : undefined;

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

// #endregion Tooltip

// #region Base Components

// [👤semio📚js🗃️sketchpad💻elements🔖basecomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Base%20Components)
// Foundational internal components like Label.
// Consumers MUST use these as building blocks for inputs.

/**
 * LabelProps holds the data fields for a LabelProps record.
 * [👤semio📚js🗃️sketchpad💻elements🔖basecomponents✂️labelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Base%20Components/d/i/LabelProps)
 **/
interface LabelProps {
  id: string;
  labelElementId?: string;
  className?: string;
  children: React.ReactNode;
}

// [👤semio📚js🗃️sketchpad💻elements🔖basecomponents🪨label](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Base%20Components/d/i/Label)
function Label({ id, labelElementId, className, children }: LabelProps) {
  const label = useLabel(id);
  const { level, isLastAtLevel, showLines, isTree } = React.useContext(TreeContext);
  const treeLabelCellPaddingLeft = `${detailPanelIndentPx(level) + 20}px`;
  return (
    <div data-slot="property-row" className={cn("group grid min-w-0 w-full items-center gap-x-[8px] min-h-[24px]", isTree ? "grid-cols-[minmax(0,1fr)_160px]" : "grid-cols-[96px_1fr]", className)}>
      <Tooltip>
        <TooltipTrigger asChild>
          {isTree ? (
            <div data-slot="property-label-tree" className="relative min-w-0" style={{ paddingLeft: treeLabelCellPaddingLeft }}>
              <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
              <div className="inline-flex items-center gap-[6px] min-w-0 h-[22px]">
                <div className="w-[14px] flex-shrink-0" />
                <span data-slot="property-label" id={labelElementId} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-left truncate cursor-pointer transition-colors hover:bg-hover-panel h-[22px]">
                  {label}
                </span>
              </div>
            </div>
          ) : (
            <span data-slot="property-label" id={labelElementId} className="inline-flex items-center text-xs font-medium flex-shrink-0 text-left truncate cursor-pointer transition-colors hover:bg-hover-panel h-[22px]">
              {label}
            </span>
          )}
        </TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
      <div data-slot="property-control" className="min-w-0">
        {children}
      </div>
    </div>
  );
}

// #endregion Base Components

// #region Display Components

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components)
// Read-only display wrappers for tooltips and callouts.
// Consumers MUST pass valid config objects.

/**
 * SemioTooltipProps holds the data fields for a SemioTooltipProps record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents✂️semiotooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/d/i/SemioTooltip)
 **/
interface SemioTooltipProps {
  children: React.ReactElement;
  config: TooltipConfig;
}

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🛠️semiotooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/d/i/SemioTooltip)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🪨semiotooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/d/i/SemioTooltip)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents✂️idsemiotooltipprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/d/i/IdSemioTooltipProps)
 * IdSemioTooltipProps holds the data fields for a IdSemioTooltipProps record.
 **/
interface IdSemioTooltipProps {
  id: string;
  children: React.ReactNode;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🪨idsemiotooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/d/i/IdSemioTooltip)
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

// #region Aside

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖aside](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Aside)
// Callout boxes for notes, tips, cautions, and dangers.
// Consumers MUST specify a valid kind prop.

/**
 * Props interface for the Aside callout component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖aside🛠️asideprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Aside/d/i/AsideProps)
 **/
export interface AsideProps {
  kind?: "note" | "tip" | "caution" | "danger";
  title?: string;
  children: React.ReactNode;
}

/**
 * iconMap holds the data fields for a iconMap record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖aside🪨iconmap](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Aside/d/i/iconMap)
 **/
const iconMap = {
  note: InfoIcon,
  tip: LightbulbIcon,
  caution: TriangleAlertIcon,
  danger: AlertCircleIcon,
};

/**
 * colorMap holds the data fields for a colorMap record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖aside🪨colormap](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Aside/d/i/colorMap)
 **/
const colorMap = {
  note: "border-info-border bg-info-bg text-info-foreground",
  tip: "border-success-border bg-success-bg text-success-foreground",
  caution: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

/**
 * Callout component rendering note, tip, caution, or danger boxes.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖aside🪨aside](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Aside/d/i/Aside)
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

// #endregion Aside

// #region Avatar

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar)
// User avatar components with image, fallback, drag, and table variants.
// Consumers MUST provide content for the fallback.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🪨avatar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/Avatar)
 * Avatar holds the data fields for a Avatar record.
 **/
const Avatar = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Root>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>>(({ className, style, ...props }, ref) => {
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
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🪨avatarimage](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/AvatarImage)
 **/
const AvatarImage = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Image>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Image ref={ref} data-slot="avatar-image" className={cn("aspect-square size-full", className)} {...props} />
));
AvatarImage.displayName = "AvatarImage";

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🪨avatarfallback](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/AvatarFallback)
 * AvatarFallback holds the data fields for a AvatarFallback record.
 **/
const AvatarFallback = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Fallback>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Fallback ref={ref} data-slot="avatar-fallback" className={cn("bg-muted flex size-full items-center justify-center rounded-full", className)} {...props} />
));
AvatarFallback.displayName = "AvatarFallback";

/**
 * Props interface for the DraggableAvatar component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🛠️draggableavatarprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/DraggableAvatarProps)
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
  dataDragKind?: "type" | "design";
  dataDragGuid?: string;
}

/**
 * Avatar component with drag-and-drop support and selection styling.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🪨draggableavatar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/DraggableAvatar)
 **/
export const DraggableAvatar = React.forwardRef<HTMLDivElement, DraggableAvatarProps>(
  ({ content, isSelected, isHovered, shouldFade, title, dragRef, dragListeners, dragAttributes, onClick, onPointerDown, onMouseDown, onDoubleClick, onPointerEnter, onPointerLeave, className, dataDragKind, dataDragGuid }, ref) => {
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
          className={cn("cursor-grab active:cursor-grabbing select-none", isSelected && "ring-1 ring-[color:var(--active-base)]", isHovered && !isSelected && "ring-1 ring-[color:var(--hover-base)]")}
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
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🛠️tableavatarprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/TableAvatarProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖avatar🪨tableavatar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Avatar/d/i/TableAvatar)
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

// #endregion Avatar

// #region Card

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖card](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Card)
// Card container and grid layout for content blocks.
// Consumers MUST provide a title string.
/**
 * Props interface for the Card component.
 *
 *[👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖card🛠️cardprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Card/d/i/CardProps)
 **/
export interface CardProps {
  title: string;
  icon?: string | LucideIcon;
  children: React.ReactNode;
  className?: string;
}

/**
 * Content card with title, icon, and children.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖card🪨card](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Card/d/i/Card)
 **/
export const Card: React.FC<CardProps> = ({ title, icon, children, className = "" }) => {
  const IconComponent = typeof icon === "string" ? null : icon;
  return (
    <div className={`border p-single ${className}`}>
      <div className="flex items-start gap-tiny mb-single">
        {IconComponent && <IconComponent className="size-small flex-shrink-0 mt-0.5" />}
        {typeof icon === "string" && <span className="text-xl flex-shrink-0">{icon}</span>}
        <h3 className="font-semibold text-base">{title}</h3>
      </div>
      <div className="text-sm">{children}</div>
    </div>
  );
};

/**
 * Props interface for the CardGrid component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖card🛠️cardgridprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Card/d/i/CardGridProps)
 **/
export interface CardGridProps {
  stagger?: boolean;
  className?: string;
  children: React.ReactNode;
}

/** [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖card🪨cardgrid](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Card/d/i/CardGrid)
 **/
export const CardGrid: React.FC<CardGridProps> = ({ stagger = false, children, className = "" }) => {
  return <div className={`grid grid-cols-1 md:grid-cols-2 gap-medium my-medium ${className}`}>{children}</div>;
};

// #endregion Card

// #region Spinner

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖spinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Spinner)
// Animated loading spinner in small, medium, or large sizes.
// Consumers MUST choose an appropriate size for the context.

/**
 * Props interface for the Spinner component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖spinner🛠️spinnerprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Spinner/d/i/SpinnerProps)
 **/
export interface SpinnerProps {
  size?: "small" | "medium" | "large";
  className?: string;
}

/**
 * Animated SVG loading spinner.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖spinner🪨spinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Spinner/d/i/Spinner)
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

// #endregion Spinner

// #region NotFound

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖notfound](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/NotFound)
// 404-style placeholder with icon, title, and back navigation.
// Consumers MUST provide a title for the error.

/**
 * Props interface for the NotFound component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖notfound🛠️notfoundprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/NotFound/d/i/NotFoundProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖notfound🪨notfound](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/NotFound/d/i/NotFound)
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

// #endregion NotFound

// #region LoadingRow

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖loadingrow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/LoadingRow)
// Skeleton loading row with pulsing icon and name.
// Consumers MUST provide a name for the placeholder.

/**
 * Props interface for the LoadingRow component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖loadingrow🛠️loadingrowprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/LoadingRow/d/i/LoadingRowProps)
/**
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖loadingrow🛠️loadingrowprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/LoadingRow/d/i/LoadingRowProps)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖loadingrow🛠️loadingrowprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/LoadingRow/d/i/LoadingRowProps)
 **/
export interface LoadingRowProps {
  name: string;
  icon?: React.ReactNode;
  className?: string;
}

/** LoadingRow holds the data fields for a LoadingRow record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖loadingrow🪨loadingrow](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/LoadingRow/d/i/LoadingRow)
 **/
export const LoadingRow: React.FC<LoadingRowProps> = ({ name, icon, className = "" }) => {
  return (
    <div className={`flex items-center gap-single p-single opacity-50 pointer-events-none ${className}`}>
      {icon && <span className="shrink-0">{icon}</span>}
      <span className="flex-1 truncate">{name}</span>
    </div>
  );
};

// #endregion LoadingRow

// #region DiagramNode

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖diagramnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/DiagramNode)
// Individual diagram node element with selection and hover states.
// Consumers MUST provide content for the node.

/**
 * Props interface for the DiagramNode component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖diagramnode🛠️diagramnodeprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/DiagramNode/d/i/DiagramNodeProps)
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
}

/**
 * Individual node element within a diagram graph.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖diagramnode🪨diagramnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/DiagramNode/d/i/DiagramNode)
 **/
export const DiagramNode: React.FC<DiagramNodeProps> = ({ content, selected = false, hovered = false, isPlaceholder = false, showTopHandle = false, showBottomHandle = false, className = "", onMouseEnter, onMouseLeave, onClick }) => {
  return (
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
  );
};
/**
 * PlaceholderDiagramNode holds the data fields for a PlaceholderDiagramNode record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖diagramnode🪨placeholderdiagramnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/DiagramNode/d/i/PlaceholderDiagramNode)
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖diagramnode🪨placeholderdiagramnode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/DiagramNode/d/i/PlaceholderDiagramNode)
export const PlaceholderDiagramNode: React.FC<{ id?: string; onClick?: () => void }> = ({ id = "diagram.placeholder", onClick }) => {
  return <DiagramNode content={useLabel(id)} isPlaceholder showTopHandle onClick={onClick} className="hover:border-[color:var(--hover-base)] hover:bg-[color:var(--hover-panel)]" />;
};

// #endregion DiagramNode

// #region HoverCard

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖hovercard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/HoverCard)
// Hover-triggered card built on Radix primitives.
// Consumers MUST use HoverCardTrigger to activate.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖hovercard🛠️hovercard](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/HoverCard/d/i/HoverCard)
 * HoverCard holds the data fields for a HoverCard record.
 **/
function HoverCard({ ...props }: React.ComponentProps<typeof HoverCardPrimitive.Root>) {
  return <HoverCardPrimitive.Root data-slot="hover-card" {...props} />;
}

/**
 * HoverCardTrigger holds the data fields for a HoverCardTrigger record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖hovercard🛠️hovercardtrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/HoverCard/d/i/HoverCardTrigger)
 **/
function HoverCardTrigger({ className, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Trigger>) {
  return <HoverCardPrimitive.Trigger data-slot="hover-card-trigger" className={cn(className)} {...props} />;
}

/**
 * HoverCardContent holds the data fields for a HoverCardContent record.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖hovercard🛠️hovercardcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/HoverCard/d/i/HoverCardContent)
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

// #endregion HoverCard

// #region Icons

// [👤semio📚js🗃️sketchpad💻elementstsx🔖displaycomponents🔖icons](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/DISPLAY-COMPONENTS/ICONS)
// Cursor icon component for collaborative pointer display.
// Consumers MUST provide position data for rendering.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖hovercard🛠️cursor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/HoverCard/d/i/Cursor)
 * CursorProps holds the data fields for a CursorProps record.
 **/
interface CursorProps {
  color: string;
  x?: number;
  y?: number;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖icons🪨cursor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Icons/d/i/Cursor)
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

// #endregion Icons

// #region Section

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖section](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Section)
// Collapsible section container with heading and specificity.
// Consumers MUST provide a heading string.

/**
 * Props interface for the Section component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖section🛠️sectionprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Section/d/i/SectionProps)
 **/
export interface SectionProps {
  id?: string;
  title?: string;
  children: React.ReactNode;
  className?: string;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖section🪨section](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Section/d/i/Section)
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

// #endregion Section

// #region Steps

// [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖steps](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Steps)
// Ordered step list container for tutorial or wizard flows.
// Consumers MUST provide step children in order.

/**
 * Props interface for the Steps component.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖steps🛠️stepsprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Steps/d/i/StepsProps)
 **/
export interface StepsProps {
  children: React.ReactNode;
  className?: string;
}

/**
 * Ordered step list container rendering numbered children.
 * [👤semio📚js🗃️sketchpad💻elements🔖displaycomponents🔖steps🪨steps](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Display%20Components/s/Steps/d/i/Steps)
 **/
export const Steps: React.FC<StepsProps> = ({ children, className = "" }) => {
  return <ol className={`flex flex-col gap-medium ${className}`}>{children}</ol>;
};

// #endregion Steps

// #endregion Display Components

// #region Input Components

// #region ActionGroup

// [👤semio📚js🗃️sketchpad💻elementstsx🔖inputcomponents](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/INPUT-COMPONENTS)
// Compact action button group with dropdown support.
// Consumers MUST provide action items for the group.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🪨actiongroupitemvariants](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/actionGroupItemVariants)
 * actionGroupItemVariants holds the data fields for a actionGroupItemVariants record.
 **/
const actionGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center shrink-0 transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg]:size-tiny [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive overflow-hidden aspect-square p-single",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🪨actiongroupcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionGroupContext)
 * ActionGroupContext holds the data fields for a ActionGroupContext record.
 **/
const ActionGroupContext = React.createContext<{ level: Level }>({
  level: "base",
});

/**
 * ActionGroupProps holds the data fields for a ActionGroupProps record.
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup✂️actiongroupprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionGroupProps)
 **/
interface ActionGroupProps extends Omit<React.ComponentProps<"div">, "children"> {
  children: React.ReactNode;
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🛠️actiongroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionGroup)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🪨actiongroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionGroup)
 * ActionGroup holds the data fields for a ActionGroup record.
 **/
function ActionGroup({ className, children, ...props }: ActionGroupProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const divideClass = getLevelDivideElementClass(level);
  return (
    <div data-slot="action-group" data-level={level} className={cn("group/action-group flex h-small items-center border divide-x overflow-hidden", borderClass, divideClass, className)} {...props}>
      <ActionGroupContext.Provider value={{ level }}>{children}</ActionGroupContext.Provider>
    </div>
  );
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🛠️actiongroupitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionGroupItem)
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
  const context = React.useContext(ActionGroupContext);
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
        <TooltipTrigger asChild>
          {actionGroupItemElement}
        </TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return actionGroupItemElement;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup✂️actiondropdownoption](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionDropdownOption)
 * ActionDropdownOption holds the data fields for a ActionDropdownOption record.
 **/
interface ActionDropdownOption {
  value: string;
  icon: React.ReactNode;
  label?: string;
}

/**
 * ActionDropdownProps holds the data fields for a ActionDropdownProps record.
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup✂️actiondropdownprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionDropdownProps)
 **/
interface ActionDropdownProps extends Omit<React.ComponentProps<"button">, "children" | "id"> {
  id: string;
  options: ActionDropdownOption[];
  value: string;
  onValueChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🛠️actiondropdown](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionDropdown)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🪨actiondropdown](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionDropdown)
 * ActionDropdown holds the data fields for a ActionDropdown record.
 **/
function ActionDropdown({ className, id, options, value, onValueChange, startTransaction, finalizeTransaction, ...props }: ActionDropdownProps) {
  const transaction = useTransaction();
  const [open, setOpen] = React.useState(false);
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup✂️actionprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/ActionProps)
 * ActionProps holds the data fields for a ActionProps record.
 **/
interface ActionProps extends Omit<React.ComponentProps<"button">, "children"> {
  as?: "button" | "div";
  loading?: boolean;
  icon?: React.ReactNode;
  text?: string;
  id?: string;
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🛠️action](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/Action)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖actiongroup🪨action](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ActionGroup/d/i/Action)
 * Action holds the data fields for a Action record.
 **/
function Action({ className, id, icon, text, as = "button", ...props }: ActionProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const Comp = as;
  const hasText = Boolean(text);

  const actionElement = (
    <Comp
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
        <TooltipTrigger asChild>
          {actionElement}
        </TooltipTrigger>
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

// #endregion ActionGroup

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🪨buttongroupitemvariants](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/buttonGroupItemVariants)
 * buttonGroupItemVariants holds the data fields for a buttonGroupItemVariants record.
 **/
const buttonGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap h-medium aspect-square p-single overflow-hidden",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🪨buttongroupcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonGroupContext)
 * ButtonGroupContext holds the data fields for a ButtonGroupContext record.
 **/
const ButtonGroupContext = React.createContext<{ level: Level }>({
  level: "base",
});

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents✂️buttongroupprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonGroupProps)
 * ButtonGroupProps holds the data fields for a ButtonGroupProps record.
 **/
interface ButtonGroupProps extends Omit<React.ComponentProps<"div">, "id"> {
  id?: string;
  showLabel?: boolean;
  children: React.ReactNode;
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🛠️buttongroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonGroup)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🪨buttongroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonGroup)
 * ButtonGroup holds the data fields for a ButtonGroup record.
 **/
function ButtonGroup({ className, id, showLabel, children, ...props }: ButtonGroupProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const divideClass = getLevelDivideElementClass(level);
  const buttonGroupElement = (
    <div data-slot="button-group" id={id} data-level={level} className={cn("group/button-group flex w-fit shrink-0 items-center border divide-x overflow-hidden h-medium", borderClass, divideClass, className)} {...props}>
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🛠️buttongroupitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonGroupItem)
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
  const context = React.useContext(ButtonGroupContext);
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
        <TooltipTrigger asChild>
          {buttonGroupItemElement}
        </TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return buttonGroupItemElement;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents✂️buttonprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents✂️buttoncycleitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonCycleItem)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents✂️buttoncycleprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonCycleProps)
 * ButtonCycleProps holds the data fields for a ButtonCycleProps record.
 **/
interface ButtonCycleProps<T extends string> extends Omit<React.ComponentProps<"button">, "children" | "id">, ElementProps {
  value?: T;
  onValueChange?: (value: T) => void;
  items: ButtonCycleItem<T>[];
  showLabel?: boolean;
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🛠️button](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/Button)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🪨button](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/Button)
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

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🛠️buttoncycle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonCycle)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🪨buttoncycle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/d/i/ButtonCycle)
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

// #region Combobox

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖combobox](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Combobox)
// Searchable dropdown with popover options list.
// Consumers MUST provide options and onValueChange handler.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖combobox✂️comboboxoption](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Combobox/d/i/ComboboxOption)
 * ComboboxOption holds the data fields for a ComboboxOption record.
 **/
interface ComboboxOption {
  value: string;
  label: string;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖combobox✂️comboboxprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Combobox/d/i/ComboboxProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖combobox🪨combobox](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Combobox/d/i/Combobox)
 **/
export const Combobox: React.FC<ComboboxProps> = ({ options, value = "", placeholder = "Select option...", placeholderId, emptyMessage = "No options found.", onValueChange, className, allowClear = false, showLabel, id }) => {
  const transaction = useTransaction();
  const [open, setOpen] = React.useState(false);
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

  const comboboxElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <Button id={id} role="combobox" aria-expanded={open} className="w-full justify-between flex-1 min-w-0">
          {selectedOption ? selectedOption.label : computedPlaceholder}
          <ChevronsUpDownIcon className="ml-2 size-tiny shrink-0 opacity-50" />
        </Button>
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

// #endregion Combobox

// #region Input

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖input](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Input)
// Text input field with label, validation, and clear support.
// Consumers MUST provide an id for accessibility.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select✂️select](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/Select)
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
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖input🛠️input](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Input/d/i/Input)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖input🪨input](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Input/d/i/Input)
 * Input holds the data fields for a Input record.
 **/
function Input({ className, type, lazy, value: externalValue, onChange, onLazyChange, interactionId, id, placeholderId, placeholder, showLabel, ...props }: InputProps) {
  const transaction = useTransaction();
  const [localValue, setLocalValue] = React.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = React.useState(false);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const placeholderLabel = useLabel(placeholderId || "");
  const computedPlaceholder = placeholderId ? placeholderLabel : placeholder;

  React.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLInputElement>) => {
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (lazy) {
      setIsEditing(true);
      transaction?.start?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLInputElement>) => {
    if (interactionId && setActiveInteraction) setActiveInteraction(id, interactionId);
    if (lazy) {
      setIsEditing(false);
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

  const inputElement = (
    <div style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }}>
      <input
        type={type}
        data-slot="input"
        id={id}
        className={cn(
          "file:text-foreground placeholder:text-muted-foreground text-foreground flex h-medium w-full min-w-0 border bg-transparent p-single text-base transition-[color,border-color] outline-none file:inline-flex file:h-medium file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
          "focus-visible:border-accent",
          "aria-invalid:ring-destructive/20 aria-invalid:border-destructive flex-1",
          type === "number" && "[&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]",
          className,
        )}
        value={inputValue}
        onChange={handleChange}
        onFocus={handleFocus}
        onBlur={handleBlur}
        onKeyDown={handleKeyDown}
        placeholder={computedPlaceholder}
        {...props}
      />
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

// #endregion Input

// #region Select

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select)
// Dropdown select built on Radix primitives.
// Consumers MUST use SelectItem children for options.

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️select](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/Select)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🪨select](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/Select)
 * Select holds the data fields for a Select record.
 **/
function Select({ id, showLabel, children, value, defaultValue, onOpenChange, ...props }: React.ComponentProps<typeof SelectPrimitive.Root> & ElementProps & { showLabel?: boolean }) {
  const transaction = useTransaction();
  const fallbackValue = React.useMemo(() => {
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectgroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectGroup)
 * SelectGroup holds the data fields for a SelectGroup record.
 **/
function SelectGroup({ ...props }: React.ComponentProps<typeof SelectPrimitive.Group>) {
  return <SelectPrimitive.Group data-slot="select-group" {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectValue)
 * SelectValue holds the data fields for a SelectValue record.
 **/
function SelectValue({ ...props }: React.ComponentProps<typeof SelectPrimitive.Value>) {
  return <SelectPrimitive.Value data-slot="select-value" {...props} />;
}

/**
 * SelectTrigger holds the data fields for a SelectTrigger record.
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selecttrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectTrigger)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectContent)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectlabel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectLabel)
 * SelectLabel holds the data fields for a SelectLabel record.
 **/
function SelectLabel({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Label>) {
  return <SelectPrimitive.Label data-slot="select-label" className={cn("text-muted-foreground p-single text-xs", className)} {...props} />;
}

/**
 * SelectItem holds the data fields for a SelectItem record.
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectItem)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectseparator](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectSeparator)
 **/
function SelectSeparator({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Separator>) {
  return <SelectPrimitive.Separator data-slot="select-separator" className={cn("bg-border pointer-events-none -mx-single my-single h-px", className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectscrollupbutton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectScrollUpButton)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🛠️selectscrolldownbutton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/SelectScrollDownButton)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖select🪨chevronupicon](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Select/d/i/ChevronUpIcon)
 **/
const ChevronUpIcon = ChevronDownIconAlt;

export { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton, SelectSeparator, SelectTrigger, SelectValue };

// #endregion Select

// #region Slider

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖slider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Slider)
// Range slider built on Radix primitives.
// Consumers MUST provide min and max values.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖slider🛠️slider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Slider/d/i/Slider)
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
  const [isEditing, setIsEditing] = React.useState(false);
  const [isSliding, setIsSliding] = React.useState(false);
  const [editValue, setEditValue] = React.useState("");
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const activeInteraction = useActiveInteraction();
  const isInteracting = interactionId && activeInteraction === interactionId;
  const shouldFade = activeInteraction && !isInteracting;

  const _values = React.useMemo(() => (Array.isArray(value) ? value : Array.isArray(defaultValue) ? defaultValue : [min, max]), [value, defaultValue, min, max]);

  const displayValue = _values[0] ?? min;

  const findNearestSnapValue = React.useCallback(
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

  const handleValueChange = React.useCallback(
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
    <div data-slot="slider-content" style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }} className="flex-1 min-w-0">
      <div data-slot="slider-row" className="grid h-[22px] grid-cols-[minmax(0,1fr)_28px] items-center gap-x-[8px]">
        <div data-slot="slider-track-cell" className="min-w-0">{wrappedSlider}</div>
        {isEditing ? (
          <Input type="number" value={editValue} onChange={(e) => setEditValue(e.target.value)} onKeyDown={handleEditKeyDown} onBlur={handleEditBlur} className="w-[28px] min-w-[28px] border-0 px-0 text-right text-xs" min={min} max={max} autoFocus id={id} />
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

// #endregion Slider

// #region Stepper

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖stepper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Stepper)
// Numeric stepper with increment/decrement and drag adjustment.
// Consumers MUST provide min and max bounds.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖stepper✂️stepperprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Stepper/d/i/StepperProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖stepper🪨stepper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Stepper/d/i/Stepper)
 **/
export const Stepper: React.FC<StepperProps> = ({ value, defaultValue = 0, min, max, step = 1, onChange, onPointerDown, onPointerUp, onPointerCancel, interactionId, id, showLabel }) => {
  const transaction = useTransaction();
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const [internalValue, setInternalValue] = React.useState(value ?? defaultValue);
  const [isEditing, setIsEditing] = React.useState(false);
  const intervalRef = React.useRef<NodeJS.Timeout | null>(null);
  const timeoutRef = React.useRef<NodeJS.Timeout | null>(null);
  const commands = useInteractionCommands();
  const setActiveInteraction = commands?.setActiveInteraction;
  const activeInteraction = useActiveInteraction();

  React.useEffect(() => {
    if (value !== undefined) {
      setInternalValue(value);
    }
  }, [value]);

  const clampValue = React.useCallback(
    (val: number): number => {
      let clampedValue = val;
      if (min !== undefined) clampedValue = Math.max(clampedValue, min);
      if (max !== undefined) clampedValue = Math.min(clampedValue, max);
      return clampedValue;
    },
    [min, max],
  );

  const updateValue = React.useCallback(
    (newValue: number) => {
      const clampedValue = clampValue(newValue);
      setInternalValue(clampedValue);
      onChange?.(clampedValue);
    },
    [clampValue, onChange],
  );

  const startContinuousChange = React.useCallback(
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

  const stopContinuousChange = React.useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  React.useEffect(() => {
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

  const labelElementId = `${id.split(".").join("-")}-label`;

  const stepperElement = (
    <div data-slot="stepper-group" className={cn("flex h-[22px] w-[100px] min-w-[100px] items-stretch overflow-hidden rounded-[3px] border transition-[border-color] focus-within:border-accent", borderClass)}>
      <button
        data-slot="stepper-minus"
        type="button"
        onMouseDown={handleMouseDown(-step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(-step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepDown}
        className={cn("flex h-[22px] w-[22px] cursor-pointer items-center justify-center border-r hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted", borderClass)}
      >
        <RemoveIcon className="size-tiny" />
      </button>
      <Input
        type="number"
        onChange={handleInputChange}
        onFocus={() => {
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
        className="h-[22px] w-[56px] min-w-[56px] border-0 px-0 text-center focus-visible:border-0"
        step={step}
        min={min}
        max={max}
        aria-labelledby={labelElementId}
        id={id}
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
        className={cn("flex h-[22px] w-[22px] cursor-pointer items-center justify-center border-l hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted", borderClass)}
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

// #endregion Stepper

// #region Textarea

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖textarea](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Textarea)
// Multi-line text input with label and validation.
// Consumers MUST provide an id for the field.

/**
 * TextareaProps holds the data fields for a TextareaProps record.
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖textarea✂️textareaprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Textarea/d/i/TextareaProps)
 **/
interface TextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onLazyChange?: (value: string) => void;
  showLabel?: boolean;
  placeholderId?: string;
  readOnly?: boolean;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖textarea🪨textarea](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Textarea/d/i/Textarea)
 **/
function Textarea({ className, lazy, value: externalValue, onChange, onLazyChange, id, showLabel, placeholderId, placeholder, ...props }: TextareaProps) {
  const transaction = useTransaction();
  const [localValue, setLocalValue] = React.useState(externalValue?.toString() || "");
  const [isEditing, setIsEditing] = React.useState(false);
  const computedPlaceholder = placeholderId ? useLabel(placeholderId) : placeholder;

  React.useEffect(() => {
    if (!isEditing) setLocalValue(externalValue?.toString() || "");
  }, [externalValue, isEditing]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setLocalValue(e.target.value);
    } else if (onChange) {
      onChange(e);
    }
  };

  const handleFocus = (e: React.FocusEvent<HTMLTextAreaElement>) => {
    if (lazy) {
      setIsEditing(true);
      transaction?.start?.();
    }
    props.onFocus?.(e);
  };

  const handleBlur = (e: React.FocusEvent<HTMLTextAreaElement>) => {
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

  const textareaElement = (
    <textarea
      data-slot="textarea"
      id={id}
      className={cn(
        "placeholder:text-muted-foreground text-foreground flex field-sizing-content min-h-huge w-full border bg-transparent px-tiny py-single text-base transition-[color,border-color] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-accent",
        "aria-invalid:border-destructive flex-1",
        className,
      )}
      value={textareaValue}
      onChange={handleChange}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      placeholder={computedPlaceholder}
      {...props}
    />
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

// #endregion Textarea

// #region Toggle

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle)
// Toggle button with pressed/unpressed states.
// Consumers MUST handle onPressedChange events.

/**
 * toggleVariants holds the data fields for a toggleVariants record.
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle🪨togglevariants](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle/d/i/toggleVariants)
 **/
const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap data-[state=on]:bg-active-base data-[state=on]:text-active-foreground data-[state=on]:hover:bg-active-base/90 data-[state=on]:hover:text-active-foreground h-medium aspect-square p-single leading-none overflow-hidden",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle🛠️toggleitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle/d/i/ToggleItem)
 **/
export interface ToggleItem<T extends string> {
  value: T;
  label: React.ReactNode;
  text?: string;
  dropdownText?: string;
  id?: string;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle✂️togglestandardprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle/d/i/ToggleStandardProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle✂️togglewithactionprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle/d/i/ToggleWithActionProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle✂️toggledropdownprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle/d/i/ToggleDropdownProps)
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

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖toggle✂️toggleprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/Toggle/d/i/ToggleProps)
type ToggleProps<T extends string = string> = ToggleStandardProps | ToggleWithActionProps | ToggleDropdownProps<T>;

export type { ToggleProps };

// #endregion Toggle

// #region ToggleGroup

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup)
// Group of mutually exclusive or multi-select toggles.
// Consumers MUST provide items with distinct values.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🪨togglegroupcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/ToggleGroupContext)
 * ToggleGroupContext holds the data fields for a ToggleGroupContext record.
 **/
const ToggleGroupContext = React.createContext<{ level: Level }>({
  level: "base",
});

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup✂️togglegroupitemprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/ToggleGroupItemProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup✂️addiconsize](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/addIconSize)
 **/
interface ToggleGroupProps extends Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Root>, "children" | "type" | "id"> {
  id?: string;
  showLabel?: boolean;
  kind?: "single" | "multiple";
  items: ToggleGroupItemProps[];
}

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🛠️togglegroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/ToggleGroup)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🪨togglegroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/ToggleGroup)
 * ToggleGroup holds the data fields for a ToggleGroup record.
 **/
function ToggleGroup({ className, id, showLabel, items, kind = "single", ...restProps }: ToggleGroupProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  const divideClass = getLevelDivideElementClass(level);

  const controlledValue = (restProps as any).value;
  const rootDataState = kind === "single" && controlledValue !== undefined ? (controlledValue ? "on" : "off") : undefined;

  const toggleGroupElement = (
    <ToggleGroupPrimitive.Root data-slot="toggle-group" data-state={rootDataState} id={id} type={kind} className={cn("group/toggle-group flex w-fit shrink-0 items-center border overflow-hidden h-medium divide-x", borderClass, divideClass, className)} {...(restProps as any)}>
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

// [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🛠️togglegroupitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/ToggleGroupItem)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🪨togglegroupitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/ToggleGroupItem)
 * ToggleGroupItem holds the data fields for a ToggleGroupItem record.
 **/
function ToggleGroupItem({ className, id, icon, text, action, ...props }: ToggleGroupItemProps) {
  const context = React.useContext(ToggleGroupContext);
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
        <TooltipTrigger asChild>
          {toggleGroupItemElement}
        </TooltipTrigger>
        <TooltipContent>
          <DescriptionTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }
  return toggleGroupItemElement;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🪨addiconsize](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/addIconSize)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖inputcomponents🔖togglegroup🛠️toggle](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Input%20Components/s/ToggleGroup/d/i/Toggle)
 * Toggle holds the data fields for a Toggle record.
 **/
function Toggle<T extends string = string>(props: ToggleProps<T>) {
  if ("kind" in props && props.kind === "withAction") {
    const { actionIcon, onActionClick, icon, text, pressed, defaultPressed, onPressedChange, id, showLabel, className, actionId } = props as ToggleWithActionProps;
    const value = pressed !== undefined ? (pressed ? "on" : "") : undefined;
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
    const [internalValue, setInternalValue] = React.useState<T | undefined>(defaultValue);
    const [internalOpen, setInternalOpen] = React.useState(false);

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

// #endregion ToggleGroup

// #region Orb

// [👤semio📚js🗃️sketchpad💻elementstsx🔖inputcomponents🔖orb](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/INPUT-COMPONENTS/ORB)
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

// #endregion Orb

// #region Ring

// [👤semio📚js🗃️sketchpad💻elementstsx🔖inputcomponents🔖ring](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/INPUT-COMPONENTS/RING)
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
  const svgRef = React.useRef<SVGSVGElement>(null);
  const [draggingOrbId, setDraggingOrbId] = React.useState<string | null>(null);
  const [localT, setLocalT] = React.useState<number | null>(null);
  const dragStartT = React.useRef<number>(0);
  const rafId = React.useRef<number>(0);
  const pendingT = React.useRef<number | null>(null);
  const center = size / 2;
  const angleFromEvent = React.useCallback(
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
  const handleOrbPointerDown = React.useCallback(
    (orbId: string, t: number) => (e: React.PointerEvent<SVGCircleElement>) => {
      e.preventDefault();
      (e.target as SVGCircleElement).setPointerCapture(e.pointerId);
      setDraggingOrbId(orbId);
      setLocalT(t);
      dragStartT.current = t;
      pendingT.current = null;
      transaction?.start?.();
      onOrbSelect?.(orbId);
    },
    [transaction, onOrbSelect],
  );
  const flushPendingChange = React.useCallback(
    (orbId: string) => {
      if (pendingT.current !== null) {
        onOrbChange?.(orbId, dragStartT.current, pendingT.current);
        pendingT.current = null;
      }
    },
    [onOrbChange],
  );
  const handlePointerMove = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      if (!draggingOrbId) return;
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
    },
    [draggingOrbId, angleFromEvent, flushPendingChange],
  );
  const handlePointerUp = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      if (!draggingOrbId) return;
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      const newT = angleFromEvent(e);
      setLocalT(null);
      onOrbChange?.(draggingOrbId, dragStartT.current, newT);
      setDraggingOrbId(null);
      transaction?.finalize?.();
    },
    [draggingOrbId, angleFromEvent, onOrbChange, transaction],
  );
  const handlePointerCancel = React.useCallback(() => {
    if (!draggingOrbId) return;
    if (rafId.current) {
      cancelAnimationFrame(rafId.current);
      rafId.current = 0;
    }
    setLocalT(null);
    setDraggingOrbId(null);
    transaction?.abort?.();
  }, [draggingOrbId, transaction]);
  React.useEffect(() => {
    return () => {
      if (rafId.current) cancelAnimationFrame(rafId.current);
    };
  }, []);
  const ringElement = (
    <svg
      ref={svgRef}
      data-slot="ring"
      id={id}
      width={size}
      height={size}
      viewBox={`${-center} ${-center} ${size} ${size}`}
      className={cn("touch-none select-none", className)}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerCancel}
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
export type { RingProps, RingOrbData };

// #endregion Ring

// #endregion Input Components

// #region Aggregation Components

// #region Accordion

// [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖accordion](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/ACCORDION)
// Collapsible accordion built on Radix primitives.
// Consumers MUST use AccordionItem children.

/**
 * Accordion holds the data fields for a Accordion record.
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖accordion🛠️accordion](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Accordion/d/i/Accordion)
 **/
function Accordion({ ...props }: React.ComponentProps<typeof AccordionPrimitive.Root>) {
  return <AccordionPrimitive.Root data-slot="accordion" {...props} />;
}

/**
 * AccordionItem holds the data fields for a AccordionItem record.
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖accordion🛠️accordionitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Accordion/d/i/AccordionItem)
 **/
function AccordionItem({ className, ...props }: React.ComponentProps<typeof AccordionPrimitive.Item>) {
  return <AccordionPrimitive.Item data-slot="accordion-item" className={cn("border-b border-element last:border-b-0", className)} {...props} />;
}

/**
 * AccordionTrigger holds the data fields for a AccordionTrigger record.
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖accordion🛠️accordiontrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Accordion/d/i/AccordionTrigger)
 **/
function AccordionTrigger({ className, children, ...props }: React.ComponentProps<typeof AccordionPrimitive.Trigger>) {
  return (
    <AccordionPrimitive.Header className="flex">
      <AccordionPrimitive.Trigger
        data-slot="accordion-trigger"
        className={cn(
          className,
        )}
        {...props}
      >
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

// #endregion Accordion

// #region Collapsible

// [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖collapsible](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/COLLAPSIBLE)
// Collapsible section built on Radix primitives.
// Consumers MUST use CollapsibleTrigger.

/**
 * Collapsible holds the data fields for a Collapsible record.
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🛠️collapsible](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/d/i/Collapsible)
 **/
function Collapsible({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.Root>) {
  return <CollapsiblePrimitive.Root data-slot="collapsible" {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🛠️collapsibletrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/d/i/CollapsibleTrigger)
 * CollapsibleTrigger holds the data fields for a CollapsibleTrigger record.
 **/
function CollapsibleTrigger({ className, ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleTrigger>) {
  return <CollapsiblePrimitive.CollapsibleTrigger data-slot="collapsible-trigger" className={cn(className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖collapsible🛠️collapsiblecontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Collapsible/d/i/CollapsibleContent)
 **/
function CollapsibleContent({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleContent>) {
  return <CollapsiblePrimitive.CollapsibleContent data-slot="collapsible-content" {...props} />;
}

export { Collapsible, CollapsibleContent, CollapsibleTrigger };

// #endregion Collapsible

// #region Dialog

// [👤semio📚js🗃️sketchpad💻elements🔖dialog](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog)
// Modal dialog built on Radix primitives.
// Consumers MUST use DialogTrigger to open.

/**
 * Dialog holds the data fields for a Dialog record.
 *[👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialog](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/Dialog)
 **/
function Dialog({ ...props }: React.ComponentProps<typeof DialogPrimitive.Root>) {
  return <DialogPrimitive.Root data-slot="dialog" {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogtrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogTrigger)
 * DialogTrigger holds the data fields for a DialogTrigger record.
 **/
function DialogTrigger({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Trigger>) {
  return <DialogPrimitive.Trigger data-slot="dialog-trigger" className={cn(className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogportal](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogPortal)
 * DialogPortal holds the data fields for a DialogPortal record.
 **/
function DialogPortal({ ...props }: React.ComponentProps<typeof DialogPrimitive.Portal>) {
  return <DialogPrimitive.Portal data-slot="dialog-portal" {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogclose](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogClose)
 * DialogClose holds the data fields for a DialogClose record.
 **/
function DialogClose({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Close>) {
  return <DialogPrimitive.Close data-slot="dialog-close" className={cn(className)} {...props} />;
}

/**
 * DialogOverlay holds the data fields for a DialogOverlay record.
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogOverlay)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogcontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogContent)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogheader](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogHeader)
 * DialogHeader holds the data fields for a DialogHeader record.
 **/
function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-header" className={cn("flex flex-col gap-single text-center sm:text-left", className)} {...props} />;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogfooter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogFooter)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖dialog🛠️dialogdescription](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Dialog/d/i/DialogDescription)
 * DialogDescription holds the data fields for a DialogDescription record.
 **/
function DialogDescription({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Description>) {
  return <DialogPrimitive.Description data-slot="dialog-description" className={cn("text-muted-foreground text-sm", className)} {...props} />;
}

export { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger };

// #endregion Dialog

// #region Resizable

function ResizablePanelGroup({ className, ...props }: React.ComponentProps<typeof ResizablePrimitive.Group>) {
  return <ResizablePrimitive.Group data-slot="resizable-panel-group" className={cn("flex h-full w-full", className)} {...props} />;
}

/**
 * ResizablePanel holds the data fields for a ResizablePanel record.
 * [👤semio📚js🗃️sketchpad💻elements🛠️resizablepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/d/i/ResizablePanel)
 **/
function ResizablePanel({ ...props }: React.ComponentProps<typeof ResizablePrimitive.Panel>) {
  return <ResizablePrimitive.Panel data-slot="resizable-panel" {...props} />;
}

function ResizableHandle({ className, onMouseDown: externalOnMouseDown, onMouseEnter: externalOnMouseEnter, onMouseLeave: externalOnMouseLeave, ...props }: React.ComponentProps<typeof ResizablePrimitive.Separator> & { onMouseDown?: React.MouseEventHandler<HTMLDivElement>; onMouseEnter?: React.MouseEventHandler<HTMLDivElement>; onMouseLeave?: React.MouseEventHandler<HTMLDivElement> }) {
  const [isHovered, setIsHovered] = React.useState(false);
  const [isDragging, setIsDragging] = React.useState(false);

  const handleMouseDown: React.MouseEventHandler<HTMLDivElement> = (e) => {
    setIsDragging(true);
    externalOnMouseDown?.(e as any);

    const handleMouseUp = () => {
      setIsDragging(false);
      document.removeEventListener("mouseup", handleMouseUp, true);
    };

    document.addEventListener("mouseup", handleMouseUp, true);
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

// #endregion Resizable

// #region Scrollable

// [👤semio📚js🗃️sketchpad💻elements🔖scrollable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Scrollable)
// Custom scrollable area built on Radix ScrollArea.
// Consumers MUST wrap content in Scrollable.

// [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖scrollable🪨scrollable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Scrollable/d/i/Scrollable)
const Scrollable = React.forwardRef<
  React.ElementRef<typeof ScrollAreaPrimitive.Viewport>,
  React.ComponentPropsWithoutRef<typeof ScrollAreaPrimitive.Root> & { orientation?: "vertical" | "horizontal" | "both" }
>(({ className, children, orientation = "vertical", ...props }, ref) => {
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
});
Scrollable.displayName = "Scrollable";

/**
 * ScrollBar holds the data fields for a ScrollBar record.
 * [👤semio📚js🗃️sketchpad💻elements🔖scrollable🛠️scrollbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Scrollable/d/i/ScrollBar)
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

// #endregion Scrollable

// #region Band

// [👤semio📚js🗃️sketchpad💻elements🔖band](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Band)
// Horizontal band of navigation items with labels and icons.
// Consumers MUST provide BandItem entries.

/**
 * Configuration interface for a single band item.
 * [👤semio📚js🗃️sketchpad💻elements🔖band🛠️banditem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Band/d/i/BandItem)
 **/
export interface BandItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Band component.
 * [👤semio📚js🗃️sketchpad💻elements🔖band🛠️bandprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Band/d/i/BandProps)
 **/
export interface BandProps {
  id?: string;
  items: BandItem[];
  scrollable?: boolean;
  className?: string;
}

// [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖band🛠️band](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Band/d/i/Band)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖band🪨band](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Band/d/i/Band)
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

// #endregion Band

// #region Strip

// [👤semio📚js🗃️sketchpad💻elements🔖strip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Strip)
// Vertical strip of icon items for compact navigation.
// Consumers MUST provide StripItem entries.

/**
 * Configuration interface for a single strip item.
 * [👤semio📚js🗃️sketchpad💻elements🔖strip🛠️stripitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Strip/d/i/StripItem)
 **/
export interface StripItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Strip component.
 * [👤semio📚js🗃️sketchpad💻elements🔖strip🛠️strip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Strip/d/i/Strip)
 **/
export interface StripProps {
  id?: string;
  items: StripItem[];
  scrollable?: boolean;
  className?: string;
}

// [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖strip🛠️strip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Strip/d/i/Strip)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖strip🪨strip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Strip/d/i/Strip)
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

// #endregion Strip

// #region Navbar

// [👤semio📚js🗃️sketchpad💻elements🔖navbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navbar)
// Top navigation bar with icon items.
// Consumers MUST provide NavbarItem entries.

/**
 * Configuration interface for a single navbar item.
 * [👤semio📚js🗃️sketchpad💻elements🔖navbar🛠️navbaritem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navbar/d/i/NavbarItem)
 **/
export interface NavbarItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
}

/**
 * Props interface for the Navbar component.
 * [👤semio📚js🗃️sketchpad💻elements🔖navbar🛠️navbarprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navbar/d/i/NavbarProps)
 **/
export interface NavbarProps {
  items: NavbarItem[];
  className?: string;
}

// [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖navbar🛠️navbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Navbar/d/i/Navbar)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖navbar🪨navbar](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Navbar/d/i/Navbar)
 * Navbar holds the data fields for a Navbar record.
 **/
function Navbar({ items, className }: NavbarProps) {
  const level = useLevel();
  const bgClass = getLevelBgClass(level);
  return (
    <nav id="semio.sketchpad.navbar" data-slot="navbar" className={cn("border-b h-large z-navbar flex items-center", bgClass, className)}>
      {items.map((item, index) => (
        <div key={item.key ?? index} className={cn("h-medium flex items-center min-w-0", item.className)}>
          {item.content}
        </div>
      ))}
    </nav>
  );
}

export { Navbar };

// #endregion Navbar

// #region Tabs

// [👤semio📚js🗃️sketchpad💻elements🔖tabs](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tabs)
// Tab container built on Radix primitives.
// Consumers MUST use TabsTrigger and TabsContent.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tabs🛠️tabs](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tabs/d/i/Tabs)
 * Tabs holds the data fields for a Tabs record.
 **/
function Tabs({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Root>) {
  return <TabsPrimitive.Root data-slot="tabs" className={cn("flex flex-col gap-single", className)} {...props} />;
}

// [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖tabs🛠️tabslist](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Tabs/d/i/TabsList)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖tabs🪨tabslist](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Tabs/d/i/TabsList)
 * TabsList holds the data fields for a TabsList record.
 **/
function TabsList({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.List>) {
  const level = useLevel();
  const bgClass = getLevelBgClass(level);
  return <TabsPrimitive.List data-slot="tabs-list" className={cn("text-muted-foreground inline-flex h-large w-fit items-center justify-center p-single", bgClass, className)} {...props} />;
}

/** TabsTrigger holds the data fields for a TabsTrigger record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖tabs🛠️tabscontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tabs/d/i/TabsContent)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖tabs🪨tabstrigger](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Tabs/d/i/TabsTrigger)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tabs🛠️tabscontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tabs/d/i/TabsContent)
 **/
function TabsContent({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Content>) {
  return <TabsPrimitive.Content data-slot="tabs-content" className={cn("flex-1 outline-none", className)} {...props} />;
}

export { Tabs, TabsContent, TabsList, TabsTrigger };

// #endregion Tabs

// #region Tree

// [👤semio📚js🗃️sketchpad💻elements🔖tree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree)
// Hierarchical tree view with sections, items, and file trees.
// Consumers MUST wrap components in TreeStateProvider.

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️treestatecontextvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeStateContextValue)
 * TreeStateContextValue holds the data fields for a TreeStateContextValue record.
 **/
interface TreeStateContextValue {
  openStates: Record<string, boolean>;
  setOpenState: (id: string, open: boolean) => void;
  getOpenState: (id: string, defaultOpen: boolean) => boolean;
}

/**
 * TreeStateContext holds the data fields for a TreeStateContext record.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨treestatecontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeStateContext)
 **/
const TreeStateContext = React.createContext<TreeStateContextValue | null>(null);

/**
 * Context provider managing tree expansion state.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨treestateprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeStateProvider)
 **/
export const TreeStateProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [openStates, setOpenStates] = React.useState<Record<string, boolean>>({});

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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨usetreestate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/useTreeState)
 **/
export const useTreeState = () => {
  const context = React.useContext(TreeStateContext);
  if (!context) throw new Error("useTreeState must be used within TreeStateProvider");
  return context;
};

const useTreeOpenState = (itemId: string, defaultOpen: boolean) => {
  const treeState = React.useContext(TreeStateContext);
  const [fallbackOpen, setFallbackOpen] = React.useState(defaultOpen);
  const open = treeState ? treeState.getOpenState(itemId, defaultOpen) : fallbackOpen;
  const setOpen = React.useCallback((value: boolean) => {
    if (treeState) {
      treeState.setOpenState(itemId, value);
      return;
    }
    setFallbackOpen(value);
  }, [itemId, treeState]);
  return { open, setOpen };
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

const TreeContext = React.createContext<{ level: number; isLastAtLevel: boolean[]; showLines: boolean; isTree: boolean }>({ level: 0, isLastAtLevel: [], showLines: true, isTree: false });
const detailPanelIndentPx = (level: number): number => level * 10;
const indentationLinePx = (i: number): number => detailPanelIndentPx(i) + 7;

/** IndentationLines holds the data fields for a IndentationLines record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖tree🪨indentationlines](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/IndentationLines)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖tree🪨indentationlines](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Tree/d/i/IndentationLines)
 **/
const IndentationLines: React.FC<{ level: number; isLastAtLevel: boolean[]; showLines: boolean }> = ({ level, isLastAtLevel, showLines }) => {
  if (!showLines || level === 0) return null;

  return (
    <div className="absolute left-0 top-0 bottom-0 pointer-events-none">
      {Array.from({ length: level }, (_, i) => (
        <div key={i} className="absolute top-0 bottom-0" style={{ left: `${indentationLinePx(i) - 0.5}px` }}>
          {!isLastAtLevel[i] && <div className="w-px h-full bg-muted-foreground/40" />}
        </div>
      ))}
    </div>
  );
};

/**
 * Wrapper rendering tree children with connecting lines.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨treecontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeContent)
 **/
export const TreeContent: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { level, isLastAtLevel, showLines } = React.useContext(TreeContext);
  return (
    <div data-slot="tree-content" className="relative" style={{ paddingTop: "3px", paddingBottom: "3px", paddingLeft: `${detailPanelIndentPx(level) + 20}px` }}>
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      {children}
    </div>
  );
};

/**
 * Configuration interface for an action button on a tree section.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🛠️treesectionaction](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeSectionAction)
 **/
export interface TreeSectionAction {
  icon: React.ReactNode;
  onClick: () => void;
  title?: string;
  id?: string;
}

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
  actions?: TreeSectionAction[];
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
}

export interface TreeDataSection {
  id: string;
  label: React.ReactNode;
  icon?: React.ReactNode;
  items?: TreeDataItem[];
  getItems?: () => Promise<TreeDataItem[]>;
  actions?: TreeSectionAction[];
  className?: string;
  defaultOpen?: boolean;
  emptyState?: React.ReactNode;
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️treesectionprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeSectionProps)
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
  actions?: TreeSectionAction[];
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️sortabletreeitemprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/SortableTreeItemProps)
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
  actions?: TreeSectionAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️treeitemprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeItemProps)
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
  actions?: TreeSectionAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
  draggable?: boolean;
  onDragStart?: React.DragEventHandler<HTMLDivElement>;
  onDragOver?: React.DragEventHandler<HTMLDivElement>;
  onDragLeave?: React.DragEventHandler<HTMLDivElement>;
  onDrop?: React.DragEventHandler<HTMLDivElement>;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️sortabletreeitemsprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/SortableTreeItemsProps)
 * SortableTreeItemsProps holds the data fields for a SortableTreeItemsProps record.
 **/
interface SortableTreeItemsProps {
  items: { id: string;[key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => React.ReactNode;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️treeRootProps](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeRootProps)
 * TreeRootProps holds the data fields for a TreeRootProps record.
 **/
interface TreeRootProps {
  children?: React.ReactNode;
  className?: string;
  showLines?: boolean;
  sections?: TreeDataSection[];
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  dragAndDropController?: TreeDragAndDropController;
  emptyState?: React.ReactNode;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️treeDataRenderingContextValue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeDataRenderingContextValue)
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

const TreeDataRenderingContext = React.createContext<TreeDataRenderingContextValue | null>(null);

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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨treesection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeSection)
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
  const { level, isLastAtLevel, showLines, isTree } = React.useContext(TreeContext);
  const displayLabel = id ? useLabel(id) : label;
  const sectionStateId = getTreeSectionStateId(id ?? String(displayLabel ?? "section"));
  const treeOpenState = useTreeOpenState(sectionStateId, defaultOpen);
  const open = controlledOpen ?? treeOpenState.open;
  const setOpen = React.useCallback((value: boolean) => {
    treeOpenState.setOpen(value);
    onOpenChange?.(value);
  }, [onOpenChange, treeOpenState]);
  const [isHovered, setIsHovered] = React.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const isExpandable = expandable ?? hasChildren;
  const rowClassName = cn(
    "relative flex items-center gap-[6px] hover:bg-hover-panel select-none overflow-hidden group min-w-0",
    isExpandable ? "cursor-foldable" : "cursor-selectable",
    className,
  );

  if (!isExpandable) {
    return (
      <div
        data-slot="tree-section-row"
        id={id}
        className={rowClassName}
        style={{ paddingLeft: `${detailPanelIndentPx(level)}px`, height: "20px", marginBottom: "6px" }}
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
        <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
        <div className="w-[14px] flex-shrink-0" />
        {loading && <Spinner size="small" className="text-muted-foreground" />}
        {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
        {id ? (
          <Tooltip>
            <TooltipTrigger asChild>
              <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate">{displayLabel}</span>
            </TooltipTrigger>
            <TooltipContent>
              <DescriptionTooltipContent id={id} />
            </TooltipContent>
          </Tooltip>
        ) : (
          <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate">{displayLabel}</span>
        )}
        {actions.length > 0 && (
          <div className="flex items-center gap-single">
            {actions.map((action, index) => (
              <Action
                key={index}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  action.onClick();
                }}
                id={action.id}
                icon={action.icon}
              />
            ))}
          </div>
        )}
      </div>
    );
  }

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      <CollapsibleTrigger asChild>
        <div
          data-slot="tree-section-row"
          id={id}
          className={rowClassName}
          style={{ paddingLeft: `${detailPanelIndentPx(level)}px`, height: "20px", marginBottom: "6px" }}
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
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
          {loading ? <Spinner size="small" className="text-muted-foreground" /> : open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          {id ? (
            <Tooltip>
              <TooltipTrigger asChild>
                <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate">{displayLabel}</span>
              </TooltipTrigger>
              <TooltipContent>
                <DescriptionTooltipContent id={id} />
              </TooltipContent>
            </Tooltip>
          ) : (
            <span data-slot="tree-label" className="flex-1 text-xs text-muted-foreground font-semibold uppercase tracking-wide truncate">{displayLabel}</span>
          )}
          {actions.length > 0 && (
            <div className="flex items-center gap-single">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  id={action.id}
                  icon={action.icon}
                />
              ))}
            </div>
          )}
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent className="min-w-0">
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree }}>
          <div className="flex flex-col gap-y-[2px]">{children}</div>
        </TreeContext.Provider>
      </CollapsibleContent>
    </Collapsible>
  );
};

/**
 * SortableTreeItem holds the data fields for a SortableTreeItem record.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨sortabletreeitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/SortableTreeItem)
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
}) => {
  const { level, isLastAtLevel, showLines, isTree } = React.useContext(TreeContext);
  const displayLabel = id ? useLabel(id) : label;
  const itemKey = id ?? displayLabel ?? id;
  const itemId = `item-${id}-${itemKey}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const [isHovered, setIsHovered] = React.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
    paddingLeft: `${detailPanelIndentPx(level)}px`,
  };

  const baseClasses = `relative flex items-center gap-[6px] hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${hasChildren ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-active-base text-active-foreground" : ""} ${isHighlighted ? "bg-active-base text-active-foreground" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren && displayLabel) {
    return (
      <>
        <div
          data-slot="tree-item-row"
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
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
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
          {isDragHandle && <Action className="cursor-grab active:cursor-grabbing" {...attributes} {...listeners} onClick={(e) => e.stopPropagation()} icon={<GripVerticalIcon size={12} className="text-muted-foreground" />} />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span
            data-slot="tree-label"
            className="flex-1 text-xs font-normal truncate text-foreground cursor-selectable"
            onClick={(e) => {
              if (e.detail > 1) return;
              e.preventDefault();
              e.stopPropagation();
              onClick?.(e);
            }}
          >
            {displayLabel as React.ReactNode}
          </span>
          {actions.length > 0 && (
            <div className="flex items-center gap-single">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  id={action.id}
                  icon={action.icon}
                />
              ))}
            </div>
          )}
        </div>
        {open && <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree }}><div className="flex flex-col gap-y-[2px]">{children}</div></TreeContext.Provider>}
      </>
    );
  }

  if (!displayLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree }}>{children}</TreeContext.Provider>;
  }

  return (
    <div
      data-slot="tree-item-row"
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
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      <div className="w-[14px] flex-shrink-0" />
      {isDragHandle && <Action className="cursor-grab active:cursor-grabbing" {...attributes} {...listeners} icon={<GripVerticalIcon size={12} className="text-muted-foreground" />} />}
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span data-slot="tree-label" className="flex-1 text-xs font-normal truncate text-foreground">{displayLabel as React.ReactNode}</span>
      {actions.length > 0 && (
        <div className="flex items-center gap-single">
          {actions.map((action, index) => (
            <Action
              key={index}
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                action.onClick();
              }}
              id={action.id}
              icon={action.icon}
            />
          ))}
        </div>
      )}
    </div>
  );
};

/**
 * Drag-and-drop sortable container for tree items.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨sortabletreeitems](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/SortableTreeItems)
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
        {items.map((item, index) => children(item, index))}
      </SortableContext>
    </DndContext>
  );
};

/**
 * Single tree item row with icon, label, and interaction handlers.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨treeitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeItem)
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
}) => {
  const resolvedLabel = id ? useLabel(id) : label;
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
      >
        {children}
      </SortableTreeItem>
    );
  }

  const { level, isLastAtLevel, showLines, isTree } = React.useContext(TreeContext);
  const itemKey = id ?? resolvedLabel ?? sortableId ?? "tree-item";
  const itemId = getTreeItemStateId(String(itemKey));
  const treeOpenState = useTreeOpenState(itemId, defaultOpen);
  const open = controlledOpen ?? treeOpenState.open;
  const setOpen = React.useCallback((value: boolean) => {
    treeOpenState.setOpen(value);
    onOpenChange?.(value);
  }, [onOpenChange, treeOpenState]);
  const [isHovered, setIsHovered] = React.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const isExpandable = expandable ?? hasChildren;
  const baseClasses = `relative flex items-center gap-[6px] hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${isExpandable ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-active-base text-active-foreground" : ""} ${isHighlighted ? "bg-active-base text-active-foreground" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (isExpandable && resolvedLabel) {
    return (
      <>
        <div
          data-slot="tree-item-row"
          role="treeitem"
          id={id}
          className={itemClasses}
          style={{ paddingLeft: `${detailPanelIndentPx(level)}px` }}
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
          onMouseEnter={() => setIsHovered(true)}
          onMouseLeave={() => setIsHovered(false)}
        >
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
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
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span
            data-slot="tree-label"
            className="flex-1 text-xs font-normal truncate text-foreground cursor-selectable"
            onClick={(e) => {
              if (e.detail > 1) return;
              e.preventDefault();
              e.stopPropagation();
              onClick?.(e);
            }}
          >
            {resolvedLabel as React.ReactNode}
          </span>
          {actions.length > 0 && (
            <div className="flex items-center gap-single">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  id={action.id}
                  icon={action.icon}
                />
              ))}
            </div>
          )}
        </div>
        {open && <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines, isTree }}><div className="flex flex-col gap-y-[2px]">{children}</div></TreeContext.Provider>}
      </>
    );
  }

  if (!resolvedLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines, isTree }}>{children}</TreeContext.Provider>;
  }

  return (
    <div data-slot="tree-item-row" role="treeitem" id={id} className={itemClasses} style={{ paddingLeft: `${detailPanelIndentPx(level)}px` }} draggable={draggable} onDragStart={onDragStart} onDragOver={onDragOver} onDragLeave={onDragLeave} onDrop={onDrop} onClick={onClick} onMouseEnter={() => setIsHovered(true)} onMouseLeave={() => setIsHovered(false)}>
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      <div className="w-[14px] flex-shrink-0" />
      {loading && <Spinner size="small" className="text-muted-foreground" />}
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span data-slot="tree-label" className="flex-1 text-xs font-normal truncate text-foreground">{resolvedLabel as React.ReactNode}</span>
      {actions.length > 0 && (
        <div className="flex items-center gap-single">
          {actions.map((action, index) => (
            <Action
              key={index}
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                action.onClick();
              }}
              id={action.id}
              icon={action.icon}
            />
          ))}
        </div>
      )}
    </div>
  );
};

/**
 * Iterator rendering a list of tree item children.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨treeitems](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeItems)
 **/
export const TreeItems: React.FC<{ children: React.ReactNode[]; renderItem: (child: React.ReactNode, index: number, isLast: boolean) => React.ReactNode }> = ({ children, renderItem }) => {
  return <>{children.map((child, index) => renderItem(child, index, index === children.length - 1))}</>;
};

/**
 * Leaf form row combining TreeItem and TreeContent into [Indent][Label][Control].
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🪨treerow](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/TREE-ROW)
 **/
export const TreeRow: React.FC<{ children: React.ReactNode; className?: string; id?: string; onClick?: (event: React.MouseEvent) => void; onDoubleClick?: (event: React.MouseEvent) => void; actions?: TreeSectionAction[] }> = ({ children, className, id, onClick, onDoubleClick, actions }) => (
  <TreeItem className={className} id={id} onClick={onClick} onDoubleClick={onDoubleClick} actions={actions}>
    {children}
  </TreeItem>
);

/**
 * Informational text row spanning the full control column width.
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🪨helperrow](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/HELPER-ROW)
 **/
export const HelperRow: React.FC<{ children: React.ReactNode; className?: string }> = ({ children, className }) => (
  <TreeItem className={className}>
    <TreeContent>
      <div data-slot="helper-row" className="text-xs text-muted-foreground leading-tight py-[2px]">{children}</div>
    </TreeContent>
  </TreeItem>
);

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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🛠️filetreenode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/FileTreeNode)
 **/
export interface FileTreeNode {
  title: string;
  path: string;
  icon?: string;
  isFolder: boolean;
  children?: FileTreeNode[];
}

/**
 * Hierarchical tree view component with optional file tree rendering.
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨tree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/Tree)
 **/
export const Tree = (({
  children,
  className = "",
  showLines = true,
  sections,
  selectionMode = "single",
  selectedIds: controlledSelectedIds,
  defaultSelectedIds = [],
  onSelectionChange,
  dragAndDropController,
  emptyState,
}: TreeRootProps) => {
  const [sectionItemsById, setSectionItemsById] = React.useState<Record<string, TreeDataItem[]>>(() => (sections ?? []).reduce<Record<string, TreeDataItem[]>>((result, section) => {
    if (section.items) {
      result[section.id] = section.items;
    }
    return result;
  }, {}));
  const [itemItemsById, setItemItemsById] = React.useState<Record<string, TreeDataItem[]>>({});
  const [loadingById, setLoadingById] = React.useState<Record<string, boolean>>({});
  const [uncontrolledSelectedIds, setUncontrolledSelectedIds] = React.useState<string[]>(() => normalizeTreeSelectedIds(defaultSelectedIds, selectionMode));
  const [draggedIds, setDraggedIds] = React.useState<string[]>([]);
  const resolvedSections = sections ?? [];
  const anchorIdRef = React.useRef<string | undefined>(normalizeTreeSelectedIds(defaultSelectedIds, selectionMode)[0]);
  const resolvedSelectedIds = React.useMemo(() => normalizeTreeSelectedIds(controlledSelectedIds ?? uncontrolledSelectedIds, selectionMode), [controlledSelectedIds, uncontrolledSelectedIds, selectionMode]);

  React.useEffect(() => {
    setSectionItemsById((previousItems) => {
      const nextItems = { ...previousItems };
      resolvedSections.forEach((section) => {
        if (section.items) {
          nextItems[section.id] = section.items;
        }
      });
      return nextItems;
    });
  }, [resolvedSections]);

  const itemMap = React.useMemo(() => {
    const map: Record<string, TreeDataItem> = {};
    resolvedSections.forEach((section) => {
      collectTreeItemMap(getTreeSectionItems(section, sectionItemsById), map);
    });
    Object.values(itemItemsById).forEach((items) => {
      collectTreeItemMap(items, map);
    });
    return map;
  }, [itemItemsById, resolvedSections, sectionItemsById]);

  const updateSelection = React.useCallback((nextSelectedIds: string[]) => {
    const normalizedIds = normalizeTreeSelectedIds(nextSelectedIds, selectionMode);
    if (controlledSelectedIds === undefined) {
      setUncontrolledSelectedIds(normalizedIds);
    }
    onSelectionChange?.(normalizedIds, normalizedIds.map((id) => itemMap[id]).filter(Boolean));
  }, [controlledSelectedIds, itemMap, onSelectionChange, selectionMode]);

  const loadSectionItems = React.useCallback(async (section: TreeDataSection) => {
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
  }, [loadingById, sectionItemsById]);

  const loadItemItems = React.useCallback(async (item: TreeDataItem) => {
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
  }, [itemItemsById, loadingById]);

  const handleDragOver = React.useCallback((event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
  }, []);

  const handleSelectItem = React.useCallback((event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
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
  }, [itemItemsById, resolvedSections, resolvedSelectedIds, sectionItemsById, selectionMode, updateSelection]);

  const handleDoubleClickItem = React.useCallback((event: React.MouseEvent, item: TreeDataItem, section: TreeDataSection, path: string[]) => {
    item.onDoubleClick?.(event, { path, selectedIds: resolvedSelectedIds, sectionId: section.id });
  }, [resolvedSelectedIds]);

  const handleDragStart = React.useCallback((event: React.DragEvent<HTMLDivElement>, item: TreeDataItem, section: TreeDataSection) => {
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
  }, [dragAndDropController, itemMap, resolvedSelectedIds]);

  const handleDrop = React.useCallback((event: React.DragEvent<HTMLDivElement>, target: TreeDataItem | TreeDataSection, targetKind: "item" | "section", section: TreeDataSection) => {
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
  }, [dragAndDropController, draggedIds, itemMap]);

  const DataItemView: React.FC<{ item: TreeDataItem; section: TreeDataSection; path: string[]; isLastItem: boolean }> = ({ item, section, path, isLastItem }) => {
    const childItems = getTreeItemItems(item, itemItemsById);
    const treeOpenState = useTreeOpenState(getTreeItemStateId(item.id), getTreeItemDefaultOpen(item));
    const isLoading = loadingById[getTreeItemLoadingId(item.id)] ?? false;
    const hasDynamicChildren = Boolean(item.getItems);
    const hasExpandableChildren = childItems.length > 0 || hasDynamicChildren || Boolean(item.emptyState);
    const isExpandable = item.collapsibleState === TreeItemCollapsibleState.None ? false : hasExpandableChildren;

    React.useEffect(() => {
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
    const isExpandable = items.length > 0 || hasDynamicChildren || Boolean(section.emptyState);

    React.useEffect(() => {
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
        onDoubleClick={section.onDoubleClick}
        onDragOver={handleDragOver}
        onDrop={(event) => handleDrop(event, section, "section", section)}
      >
        {items.map((item, index) => (
          <DataItemView key={item.id} item={item} section={section} path={[section.id, item.id]} isLastItem={index === items.length - 1} />
        ))}
        {!isLoading && items.length === 0 && section.emptyState && <HelperRow>{section.emptyState}</HelperRow>}
      </TreeSection>
    );
  };

  return (
    <TreeStateProvider>
      <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines, isTree: true }}>
        <div className={`w-full min-w-0 overflow-hidden ${className}`}>
          {resolvedSections.length > 0 ? resolvedSections.map((section) => (
            <DataSectionView key={section.id} section={section} />
          )) : children}
          {resolvedSections.length === 0 && !children && emptyState}
        </div>
      </TreeContext.Provider>
    </TreeStateProvider>
  );
}) as React.FC<TreeRootProps> & {
  Files: React.FC<TreeFilesProps>;
  Section: React.FC<TreeFilesProps>;
};

// #region Basic Chat Panel

// [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖basicchatpanel](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/BASIC-CHAT-PANEL)
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
  const [messages, setMessages] = React.useState<BasicChatMessage[]>(() => createBasicChatMessages(id, title));
  const [draft, setDraft] = React.useState("");
  const nextMessageIndexRef = React.useRef(2);
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

  React.useEffect(() => {
    nextMessageIndexRef.current = 2;
    setMessages(createBasicChatMessages(id, title));
    setDraft("");
  }, [id, title]);

  return (
    <div data-testid="basic-chat-panel" className="flex h-full min-h-0 flex-col gap-single">
      <HelperRow>{`Local chat for ${title}. Use Enter to send and Shift+Enter for a new line.`}</HelperRow>
      <div data-testid="basic-chat-feed" className={cn("min-h-0 flex-1 overflow-y-auto rounded-[3px] border", borderClass)}>
        <Tree className="min-w-0 p-single">
          {messages.map((message) => (
            <TreeRow key={message.id}>
              <div data-testid="basic-chat-message" data-chat-role={message.role} className="flex min-w-0 flex-col gap-[2px]">
                <span className="text-[10px] font-semibold uppercase tracking-wide text-muted-foreground">{message.role}</span>
                <p className="text-xs text-foreground whitespace-pre-wrap break-words">{message.body}</p>
              </div>
            </TreeRow>
          ))}
        </Tree>
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

// #endregion Basic Chat Panel

interface FileTreeItemProps {
  node: FileTreeNode;
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
}

// [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖tree🪨filetreeitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Tree/d/i/FileTreeItem)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖aggregationcomponents🔖tree🪨filetreeitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Aggregation%20Components/s/Tree/d/i/FileTreeItem)
 * FileTreeItem holds the data fields for a FileTreeItem record.
 **/
const FileTreeItem: React.FC<FileTreeItemProps> = ({ node, currentPath, onNavigate, as = "a" }) => {
  const { level, isTree } = React.useContext(TreeContext);
  const [isHovered, setIsHovered] = React.useState(false);
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
    style: { paddingLeft: `${detailPanelIndentPx(level) + 12}px` },
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
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [], showLines: false, isTree }}>
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree✂️treefilesprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/TreeFilesProps)
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
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false, isTree: true }}>
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
 * [👤semio📚js🗃️sketchpad💻elements🔖tree🪨filetree](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Tree/d/i/FileTree)
 **/
export const FileTree = TreeFiles;

// #region ControlTree

// [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE)
// Leva-like nested folder+controls tree UI using existing design system components.
// Consumers MUST provide ControlDef[] and optional ControlTreeFolderSettings.

/**
 * Leaf control definition for the ControlTree.
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🛠️controldef](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/CONTROL-DEF)
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
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🛠️controltreefoldersettings](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/CONTROL-TREE-FOLDER-SETTINGS)
 **/
export interface ControlTreeFolderSettings {
  path: string;
  order?: number;
  collapsed?: boolean;
  color?: string;
}

/**
 * Styling classname overrides for ControlTree visual slots.
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🛠️controltreeclassnames](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/CONTROL-TREE-CLASS-NAMES)
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
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🪨buildcontroltree](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/BUILD-CONTROL-TREE)
 **/
export function buildControlTree(
  controls: ControlDef[],
  filterText: string,
  folderSettings?: Record<string, ControlTreeFolderSettings>,
): Record<string, ControlTreeNode> {
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
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🪨defaultcontrolrenderer](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/DEFAULT-CONTROL-RENDERER)
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
              return <SelectItem key={v} value={v}>{l}</SelectItem>;
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
    <div data-slot="control-tree-row-left" className="relative min-w-0">{left}</div>
    <div data-slot="control-tree-row-right" className="min-w-0">{right}</div>
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
  const { level, isLastAtLevel, showLines, isTree } = React.useContext(TreeContext);
  const itemId = `control-tree-folder-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const hasChildren = hasNonEmptyChildren(children);
  return (
    <>
      <ControlTreeRow
        className={cn("hover:bg-hover-panel select-none overflow-hidden group", classNames?.folderRow)}
        left={
          <div
            className="flex min-w-0 items-center gap-[6px]"
            style={{ paddingTop: "3px", paddingBottom: "3px", paddingLeft: `${detailPanelIndentPx(level) + 2}px` }}
          >
            <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
            {hasChildren ? (
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
            ) : (
              <div className="w-[14px] flex-shrink-0" />
            )}
            <span data-slot="control-tree-folder-label" className={cn("text-xs font-semibold uppercase tracking-wide truncate text-muted-foreground", classNames?.folderTitle)}>
              {node.key}
            </span>
          </div>
        }
      />
      {open && hasChildren && (
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree }}>
          <div className={cn("flex flex-col gap-y-[2px]", classNames?.folderChildren)}>{children}</div>
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
  const { level, isLastAtLevel, showLines } = React.useContext(TreeContext);
  return (
    <ControlTreeRow
      className={cn("hover:bg-hover-panel select-none overflow-hidden group", classNames?.controlRow)}
      left={
        <div
          className="flex min-w-0 items-center gap-[6px]"
          style={{ paddingTop: "3px", paddingBottom: "3px", paddingLeft: `${detailPanelIndentPx(level) + 2}px` }}
        >
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
          <div className="w-[14px] flex-shrink-0" />
          <span data-slot="control-tree-control-label" className={cn("text-xs font-normal truncate text-foreground", classNames?.controlLabel)}>
            {node.key}
          </span>
        </div>
      }
      right={<div data-slot="control-tree-control-body" className={cn("min-w-0", classNames?.controlBody)}>{renderControl(node.control!)}</div>}
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
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🛠️controltreeprops](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/CONTROL-TREE-PROPS)
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
 * [👤semio📚js🗃️sketchpad💻elementstsx🔖aggregationcomponents🔖tree🔖controltree🪨controltree](semiorepo://definition/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/AGGREGATION-COMPONENTS/TREE/CONTROL-TREE/CONTROL-TREE)
 **/
export const ControlTree: React.FC<ControlTreeProps> = ({
  controls,
  filterText = "",
  folderSettings,
  onToggleFolder,
  renderControl = defaultControlRenderer,
  classNames,
  className,
}) => {
  const tree = React.useMemo(() => buildControlTree(controls, filterText, folderSettings), [controls, filterText, folderSettings]);
  const sorted = React.useMemo(() => sortControlTreeNodes(tree), [tree]);
  return (
    <div data-slot="control-tree" className={cn("w-full min-w-0", classNames?.panel, className)}>
      <Tree>
        {sorted.map((node) =>
          node.kind === "folder" ? (
            <ControlTreeFolder key={node.path} node={node} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
          ) : (
            <ControlTreeLeafRow key={node.path} node={node} renderControl={renderControl} classNames={classNames} />
          ),
        )}
      </Tree>
    </div>
  );
};

// #endregion ControlTree

// #endregion Tree

// #endregion Aggregation Components

// #region Navigation Components

// #region Breadcrumb

// [👤semio📚js🗃️sketchpad💻elementstsx🔖navigationcomponents🔖breadcrumb](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/NAVIGATION-COMPONENTS/BREADCRUMB)
// Breadcrumb trail for hierarchical page navigation.
// Consumers MUST provide BreadcrumbItemData entries.

/**
 * Data interface for a single breadcrumb entry.
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🛠️breadcrumbitemdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbItemData)
 **/
export interface BreadcrumbItemData {
  id?: string;
  content: React.ReactNode;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb✂️breadcrumbprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbProps)
 * BreadcrumbProps holds the data fields for a BreadcrumbProps record.
 **/
interface BreadcrumbProps extends Omit<React.ComponentProps<"nav">, "children"> {
  items: BreadcrumbItemData[];
}

/** Breadcrumb holds the data fields for a Breadcrumb record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🛠️breadcrumb](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/Breadcrumb)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🪨breadcrumb](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/Breadcrumb)
 **/
function Breadcrumb({ className, items, ...props }: BreadcrumbProps) {
  const [openIndex, setOpenIndex] = React.useState<number | null>(null);
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);

  return (
    <nav aria-label="breadcrumb" data-slot="breadcrumb" className={cn("flex h-medium items-stretch border", borderClass, className)} {...props}>
      <ol data-slot="breadcrumb-list" className="flex flex-wrap items-stretch text-xs break-words overflow-hidden h-full">
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
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb✂️breadcrumbitemprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbItemProps)
 * BreadcrumbItemProps holds the data fields for a BreadcrumbItemProps record.
 **/
interface BreadcrumbItemProps extends Omit<React.ComponentProps<"li">, "content"> {
  id?: string;
  content?: React.ReactNode;
  onNavigate?: (href: string) => void;
  options?: { label: React.ReactNode; href: string; id?: string }[];
}

// [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🛠️breadcrumbitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbItem)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🪨breadcrumbitem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbItem)
 * BreadcrumbItem holds the data fields for a BreadcrumbItem record.
 **/
function BreadcrumbItem({ className, id, content, children, onNavigate, options, ...props }: BreadcrumbItemProps) {
  const itemContent = content ?? children;
  const interactiveContent = React.useMemo(() => {
    if (itemContent == null || typeof itemContent === "boolean") return null;
    if (React.isValidElement(itemContent)) {
      if (itemContent.type === React.Fragment) {
        return (
          <span data-slot="breadcrumb-link" className="cursor-selectable">
            {itemContent}
          </span>
        );
      }
      const elementProps = itemContent.props as { className?: string;["data-slot"]?: string };
      return React.cloneElement(itemContent as React.ReactElement<any>, {
        className: cn("cursor-selectable", elementProps?.className),
        "data-slot": elementProps?.["data-slot"] ?? "breadcrumb-link",
      });
    }
    return (
      <span data-slot="breadcrumb-link" className="cursor-selectable">
        {itemContent}
      </span>
    );
  }, [itemContent]);

  const itemElement = (
    <li data-slot="breadcrumb-item" id={id} className={cn("flex items-stretch cursor-selectable", className)} {...props}>
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
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb✂️breadcrumbseparatoritemprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbSeparatorItemProps)
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
// [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🛠️breadcrumbseparatoritem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbSeparatorItem)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖breadcrumb🪨breadcrumbseparatoritem](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/Breadcrumb/d/i/BreadcrumbSeparatorItem)
 **/
function BreadcrumbSeparatorItem({ hasOptions, isOpen, onOpenChange, id, options, onNavigate }: BreadcrumbSeparatorItemProps) {
  const icon = isOpen ? <ChevronDownIcon className="cursor-foldable" /> : <ChevronRightIcon className="cursor-foldable" />;

  const handleSelect = (href: string) => {
    onOpenChange?.(false);
    onNavigate?.(href);
  };

  if (!hasOptions || !options?.length) {
    return (
      <li data-slot="breadcrumb-separator" role="presentation" aria-hidden="true" className="flex items-center p-single">
        <Action icon={icon} className="cursor-foldable pointer-events-none" as="div" />
      </li>
    );
  }
  return (
    <li data-slot="breadcrumb-separator" role="presentation" className="flex items-center p-single">
      <DropdownMenuPrimitive.Root open={isOpen} onOpenChange={onOpenChange}>
        <DropdownMenuPrimitive.Trigger asChild>
          <div>
            <Action id={id && !isOpen ? id : undefined} icon={icon} className="cursor-foldable" />
          </div>
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

// #endregion Breadcrumb

// #region PageNavigation

/**
 * Configuration interface for a previous/next page link.
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖pagenavigation🛠️pagenavigationlink](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/PageNavigation/d/i/PageNavigationLink)
 **/
export interface PageNavigationLink {
  path: string;
  title: string;
  section?: string;
}
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖pagenavigation🛠️pagenavigationprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/PageNavigation/d/i/PageNavigationProps)
 **/
export interface PageNavigationProps {
  prev?: PageNavigationLink;
  next?: PageNavigationLink;
}

// [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖pagenavigation🪨pagenavigation](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/PageNavigation/d/i/PageNavigation)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖navigationcomponents🔖pagenavigation🪨pagenavigation](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Navigation%20Components/s/PageNavigation/d/i/PageNavigation)
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

// #endregion PageNavigation

// #endregion Navigation Components

// #region Panel Components

// #region Panel

// [👤semio📚js🗃️sketchpad💻elementstsx🔖panelcomponents](semiorepo://section/SEMIO/JS/SKETCHPAD/ELEMENTS.TSX/PANEL-COMPONENTS)
// Resizable dockable panel with sections and collapse support.
// Consumers MUST set resizeSide for the handle.

/**
 * Union type for panel resize handle positions.
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🛠️resizeside](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/ResizeSide)
 **/
export type ResizeSide = "left" | "right" | "top" | "bottom";

/**
 * Configuration interface for a collapsible section within a panel.
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🛠️panelsection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/PanelSection)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🛠️panelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/PanelProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🪨panel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/Panel)
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
  const [isResizeHovered, setIsResizeHovered] = React.useState(false);
  const [isResizing, setIsResizing] = React.useState(false);
  if (!visible) return null;
  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    const startPos = resizeSide === "top" || resizeSide === "bottom" ? e.clientY : e.clientX;
    const startSize = size;
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
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
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
  return (
    <LevelProvider level="panel">
      <div data-panel={panelKey} className={cn(containerClass, showBackground ? "bg-panel" : undefined)} style={{ ...positionStyle, opacity, transition: "opacity 150ms" }}>
        <Scrollable className="h-full">
          <div className={`${className || "p-single"} overflow-hidden min-w-0`}>
            <TreeStateProvider>
              <Tree className="min-w-0 overflow-hidden">
                {additionalContent}
                {sortedSections.map((section, index) => {
                  const content = typeof section.content === "function" ? section.content() : section.content;
                  return (
                    <PanelSectionWrapper key={section.id} section={section} defaultOpen={section.defaultOpen ?? index === 0}>
                      {content}
                    </PanelSectionWrapper>
                  );
                })}
                {!hasContent && emptyMessage && <div className="p-small text-center text-muted-foreground">{emptyMessage}</div>}
              </Tree>
            </TreeStateProvider>
          </div>
          {footer}
        </Scrollable>
        {onSizeChange && <div className={resizeHandleClass} onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />}
      </div>
    </LevelProvider>
  );
};

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🪨panelsectionwrapper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/PanelSectionWrapper)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🪨panelsectionwrapper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/PanelSectionWrapper)
 *PanelSectionWrapper holds the data fields for a PanelSectionWrapper record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panel🪨panelsectionwrapper](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/Panel/d/i/PanelSectionWrapper)
const PanelSectionWrapper: React.FC<{ section: PanelSection; defaultOpen: boolean; children: React.ReactNode }> = ({ section, defaultOpen, children }) => {
  const sectionLabel = useLabel(section.id) ?? section.id;
  return (
    <TreeSection label={sectionLabel} id={section.id} defaultOpen={defaultOpen} actions={section.actions} onPointerEnter={section.onPointerEnter} onPointerLeave={section.onPointerLeave} onDoubleClick={section.onDoubleClick}>
      {children}
    </TreeSection>
  );
};

export { Panel };

// #endregion Panel

// #region PanelGroup

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panelgroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/PanelGroup)
// Flex container grouping multiple panels together.
// Consumers MUST provide panel children.

/**
 * Props interface for the PanelGroup component.
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panelgroup🛠️panelgroupprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/PanelGroup/d/i/PanelGroupProps)
 **/
export interface PanelGroupProps {
  className?: string;
  position?: "left" | "right" | "middle" | "bottom";
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖panelgroup🪨panelgroup](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/PanelGroup/d/i/PanelGroup)
 * PanelGroup holds the data fields for a PanelGroup record.
 **/
const PanelGroup: React.FC<PanelGroupProps> = ({ children, className = "", position = "middle" }) => {
  const baseClass = "flex";
  const positionClass = position === "left" || position === "right" || position === "middle" ? "flex-col" : "flex-row";
  return <div className={`${baseClass} ${positionClass} ${className}`}>{children}</div>;
};

export { PanelGroup };

// #endregion PanelGroup

// #region LeftPanel

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖leftpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/LeftPanel)
// Left-docked panel variant with right resize handle.

/**
 * Props type for LeftPanel omitting resizeSide.
 *
 *[👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖leftpanel🛠️leftpanelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/LeftPanel/d/i/LeftPanelProps)
 **/
export type LeftPanelProps = Omit<PanelProps, "resizeSide">;

/** LeftPanel holds the data fields for a LeftPanel record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖leftpanel🪨leftpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/LeftPanel/d/i/LeftPanel)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖leftpanel🪨leftpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/LeftPanel/d/i/LeftPanel)
 **/
const LeftPanel: React.FC<LeftPanelProps> = (props) => <Panel {...props} resizeSide="right" />;

export { LeftPanel };

// #endregion LeftPanel

// #region RightPanel

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖rightpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/RightPanel)
/** RightPanel holds the data fields for a RightPanel record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖rightpanel🪨rightpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/RightPanel/d/i/RightPanel)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖rightpanel🪨rightpanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/RightPanel/d/i/RightPanel)
 **/
const RightPanel: React.FC<RightPanelProps> = (props) => <Panel {...props} resizeSide="left" />;

export { RightPanel };

// #endregion RightPanel

// #region MiddlePanel

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖middlepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/MiddlePanel)
// Center panel variant without resize handles.

/**
 * Props type for MiddlePanel omitting resizeSide.
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖middlepanel🛠️middlepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/MiddlePanel/d/i/MiddlePanel)
 **/
export interface MiddlePanelProps extends Omit<PanelProps, "resizeSide"> {
  resizeSide?: "left" | "right";
};

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖middlepanel🪨middlepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/MiddlePanel/d/i/MiddlePanel)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖middlepanel🪨middlepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/MiddlePanel/d/i/MiddlePanel)
 * MiddlePanel holds the data fields for a MiddlePanel record.
 **/
const MiddlePanel: React.FC<MiddlePanelProps> = ({ resizeSide = "right", ...props }) => <Panel {...props} resizeSide={resizeSide} />;

export { MiddlePanel };

// #endregion MiddlePanel

// #region BottomPanel

// Bottom-docked panel variant with top resize handle.
// Consumers MUST provide visible and children props.

/**
 * Props type for BottomPanel omitting resizeSide.
 *
 *[👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖bottompanel🛠️bottompanelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/BottomPanel/d/i/BottomPanelProps)
 **/
export type BottomPanelProps = Omit<PanelProps, "resizeSide">;

/** BottomPanel holds the data fields for a BottomPanel record.
 **/
// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖bottompanel🪨bottompanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/BottomPanel/d/i/BottomPanel)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖bottompanel🪨bottompanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/BottomPanel/d/i/BottomPanel)
 **/
const BottomPanel: React.FC<BottomPanelProps> = (props) => <Panel {...props} resizeSide="top" />;

export { BottomPanel };

// #endregion BottomPanel

// #region SidePanel

// [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖sidepanel](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/SidePanel)
// Collapsible side panel with tabbed content.
// Consumers MUST provide SidePanelTabConfig entries.

/**
 * Configuration interface for a side panel tab.
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖sidepanel🛠️sidepaneltabconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/SidePanel/d/i/SidePanelTabConfig)
 **/
export interface SidePanelTabConfig {
  id: string;
  icon: React.ComponentType<{ size?: number }>;
  order?: number;
  content: React.ReactNode | (() => React.ReactNode);
}

/**
 * Props interface for the SidePanel component.
 * [👤semio📚js🗃️sketchpad💻elements🔖panelcomponents🔖sidepanel🛠️sidepanelprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Panel%20Components/s/SidePanel/d/i/SidePanelProps)
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

const SidePanel: React.FC<SidePanelProps> = ({
  position,
  visible = true,
  size = 300,
  onSizeChange,
  tabs,
  activeTabId,
  onActiveTabChange,
  minSize = 200,
  maxSize = 600,
  zIndex = 20,
  className = "",
}) => {
  const [isResizeHovered, setIsResizeHovered] = React.useState(false);
  const [isResizing, setIsResizing] = React.useState(false);
  const [internalActiveTab, setInternalActiveTab] = React.useState<string | undefined>(tabs[0]?.id);

  const currentActiveTab = activeTabId ?? internalActiveTab;
  const sortedTabs = [...tabs].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));
  const activeTab = sortedTabs.find((tab) => tab.id === currentActiveTab) ?? sortedTabs[0];
  const ActiveTabContent = typeof activeTab?.content === "function" ? activeTab.content : null;

  const handleTabChange = (tabId: string) => {
    if (onActiveTabChange) {
      onActiveTabChange(tabId);
    } else {
      setInternalActiveTab(tabId);
    }
  };
  const resizeSide = position === "left" ? "right" : "left";

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    const startX = e.clientX;
    const startSize = size;
    const handleMouseMove = (e: MouseEvent) => {
      const delta = e.clientX - startX;
      let newSize: number;
      if (position === "left") {
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
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  const borderClass = resizeSide === "left" ? (isResizing || isResizeHovered ? "border-l-accent" : "border-l") : isResizing || isResizeHovered ? "border-r-accent" : "border-r";

  const positionStyle =
    position === "left"
      ? { left: "var(--spacing-double)", top: "var(--spacing-double)", bottom: "var(--spacing-double)", width: `${size}px`, zIndex }
      : { right: "var(--spacing-double)", top: "var(--spacing-double)", bottom: "var(--spacing-double)", width: `${size}px`, zIndex };

  const resizeHandleClass = `absolute top-0 bottom-0 ${resizeSide === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize`;

  return (
    <LevelProvider level="panel">
      <div data-panel={position === "left" ? "leftSidePanel" : "rightSidePanel"} className={cn("absolute text-foreground border bg-panel min-w-0 overflow-hidden flex flex-col", borderClass, className)} style={positionStyle}>
        <div data-slot="side-panel-tabs" className="flex items-center h-medium border-b shrink-0 overflow-x-auto">
          {sortedTabs.map((tab) => {
            const Icon = tab.icon;
            const isActive = tab.id === activeTab?.id;
            return (
              <Tooltip key={tab.id}>
                <TooltipTrigger asChild>
                  <button data-slot="side-panel-tab-button" id={tab.id} onClick={() => handleTabChange(tab.id)} className={cn("flex items-center justify-center h-full px-small border-r cursor-pointer transition-colors", isActive ? "bg-hover-panel" : "hover:bg-hover-panel")}>
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
        <Scrollable className="flex-1 min-h-0">
          <div data-slot="side-panel-content" className="p-[10px]">{activeTab && (ActiveTabContent ? <ActiveTabContent /> : (activeTab.content as React.ReactNode))}</div>
        </Scrollable>
        {onSizeChange && <div className={resizeHandleClass} onMouseDown={handleMouseDown} onMouseEnter={() => setIsResizeHovered(true)} onMouseLeave={() => !isResizing && setIsResizeHovered(false)} />}
      </div>
    </LevelProvider>
  );
};
export { SidePanel };

// #endregion SidePanel



// #endregion Panel Components

// #region Toolbar Components

interface ToolbarZoneProps extends React.ComponentProps<"div"> {
  children: React.ReactNode;
}

function ToolbarZone({ className, children, ...props }: ToolbarZoneProps) {
  const level = useLevel();
  const borderClass = getLevelBorderElementClass(level);
  return (
    <div
      data-slot="toolbar-zone"
      className={cn(
        "bg-panel flex h-[var(--toolbar-item-height)] shrink-0 items-center gap-[var(--toolbar-gap)] border rounded-md px-[var(--toolbar-padding-inline)] shadow-sm overflow-hidden",
        borderClass,
        className,
      )}
      {...props}
    >
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

// #endregion Toolbar Components

// #region Window Components

// #region Window

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
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖window✂️window](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Window/d/i/Window)
 * WindowProps holds the data fields for a WindowProps record.
 **/
interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖window🪨defaulterrordisplay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Window/d/i/DefaultErrorDisplay)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖window🪨defaulterrordisplay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Window/d/i/DefaultErrorDisplay)
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

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖window🪨window](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Window/d/i/Window)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖window🪨window](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Window/d/i/Window)
 * Window holds the data fields for a Window record.
 **/
const Window: React.FC<WindowProps> = ({ id, children, onDoubleClick, className = "", isVisible = true, loading = false, error = null, skeleton, showControls = false, onOpenInNewWindow, onMaximize, onMinimize, onClose, controls }) => {
  const [isMaximized, setIsMaximized] = React.useState(false);
  const [headerElement, setHeaderElement] = React.useState<HTMLElement | null>(null);
  const windowRef = React.useRef<HTMLDivElement>(null);
  const bgClass = "bg-window";

  const handleMaximize = () => {
    setIsMaximized(!isMaximized);
    if (isMaximized && onMinimize) onMinimize();
    else if (!isMaximized && onMaximize) onMaximize();
  };

  React.useEffect(() => {
    if (windowRef.current) {
      const stack = windowRef.current.closest(".lm_item.lm_stack");
      const header = stack?.querySelector(".lm_header") as HTMLElement | null;
      setHeaderElement(header);
    }
  }, []);

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
            <ActionGroupItem id={`${id}-window-controls-maximize`} onClick={handleMaximize}>
              {isMaximized ? <Minimize2Icon /> : <Maximize2Icon />}
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
      <div ref={windowRef} onDoubleClick={onDoubleClick} className={cn(`relative w-full h-full overflow-hidden ${bgClass}`, className)}>
        {headerElement
          ? createPortal(<div className="absolute right-1 top-0 -bottom-px flex items-center z-panel bg-window border-t border-l border-element">{controlsContent}</div>, headerElement)
          : hasControls && <div className="absolute top-1 right-1 z-panel flex items-stretch gap-single">{controlsContent}</div>}
        {error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}
      </div>
    </LevelProvider>
  );
};

export { Window };

// #endregion Window

// #region Page

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖page](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Page)
// Full-page content wrapper with frontmatter and footer.
// Consumers MUST provide frontmatter and children.

/**
 * Frontmatter metadata interface for a documentation page.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖page🛠️pagefrontmatter](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Page/d/i/PageFrontmatter)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖page🛠️pageprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Page/d/i/PageProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖page🪨page](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Page/d/i/Page)
 **/
export const Page: React.FC<PageProps> = ({ frontmatter, focusedItemId, onFocusComplete, footer, children }) => {
  const scrollAreaRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
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
// #endregion Page

// #region Diagram

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram)
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
  ViewportPortal
};
export type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, ReactFlowInstance, Simulation, SimulationLinkDatum, SimulationNodeDatum };

/**
 * Base pixel unit for diagram node sizing.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🪨diagramunit](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DIAGRAM_UNIT)
 **/
export const DIAGRAM_UNIT = 48;

/**
 * Union type for diagram layout directions (TB/BT/LR/RL).
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🛠️diagramlayoutdirection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramLayoutDirection)
 **/
export type DiagramLayoutDirection = "TB" | "BT" | "LR" | "RL";

/**
 * Configuration interface for dagre-based diagram layout.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🛠️diagramlayoutoptions](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramLayoutOptions)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🛠️calculatediagramlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/calculateDiagramLayout)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🛠️diagramforceconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramForceConfig)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🪨defaultdiagramforceconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/defaultDiagramForceConfig)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram✂️forcenode](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/ForceNode)
 * ForceNode holds the data fields for a ForceNode record.
 **/
interface ForceNode extends SimulationNodeDatum {
  id: string;
  data: any;
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram✂️forcelink](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/ForceLink)
 * ForceLink holds the data fields for a ForceLink record.
 **/
interface ForceLink extends SimulationLinkDatum<ForceNode> {
  id: string;
}

/**
 * Props interface for the Diagram component.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🛠️diagramprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🪨diagraminner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramInner)
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
  const forceConfig = React.useMemo(() => ({ ...defaultDiagramForceConfig, ...forceConfigProp }), [forceConfigProp]);
  const simulationRef = React.useRef<Simulation<any, any> | null>(null);
  const draggingNodeRef = React.useRef<string | null>(null);
  const isControlled = controlledNodes !== undefined && controlledEdges !== undefined;
  const rfStoreApi = useStoreApi();
  React.useEffect(() => {
    const original = rfStoreApi.setState;
    const api = rfStoreApi as any;
    api.__suppressTransform = false;
    api.__pendingTransform = null;
    api.__original = original;
    rfStoreApi.setState = ((partial: any, replace: any) => {
      if (typeof partial === 'object' && partial !== null && !replace) {
        const state = rfStoreApi.getState();
        const keys = Object.keys(partial);
        if (keys.length > 0 && keys.every((k) => Object.is((state as any)[k], partial[k]))) return;
        if (api.__suppressTransform && keys.length === 1 && keys[0] === 'transform') {
          const t = partial.transform;
          const el = document.querySelector('.react-flow__viewport') as HTMLElement;
          if (el) el.style.transform = `translate(${t[0]}px, ${t[1]}px) scale(${t[2]})`;
          api.__pendingTransform = t;
          return;
        }
      }
      return original(partial, replace);
    }) as typeof original;
    return () => { rfStoreApi.setState = original; };
  }, [rfStoreApi]);

  const [internalNodes, setInternalNodes] = React.useState<Node[]>(initialNodes);
  const [internalEdges, setInternalEdges] = React.useState<Edge[]>(initialEdges);

  const finalNodes = isControlled ? controlledNodes : internalNodes;
  const finalEdges = isControlled ? controlledEdges : internalEdges;

  const onNodesChangeReactFlowRef = React.useRef(onNodesChangeReactFlow);
  onNodesChangeReactFlowRef.current = onNodesChangeReactFlow;
  const onNodeDragStartPropRef = React.useRef(onNodeDragStartProp);
  onNodeDragStartPropRef.current = onNodeDragStartProp;
  const onNodeDragPropRef = React.useRef(onNodeDragProp);
  onNodeDragPropRef.current = onNodeDragProp;
  const onNodeDragStopPropRef = React.useRef(onNodeDragStopProp);
  onNodeDragStopPropRef.current = onNodeDragStopProp;
  const onInitPropRef = React.useRef(onInitProp);
  onInitPropRef.current = onInitProp;
  const onConnectRef = React.useRef(onConnect);
  onConnectRef.current = onConnect;
  const onMoveStartRef = React.useRef(onMoveStart);
  onMoveStartRef.current = onMoveStart;
  const onMoveEndRef = React.useRef(onMoveEnd);
  onMoveEndRef.current = onMoveEnd;
  const onSelectionChangeRef = React.useRef(onSelectionChange);
  onSelectionChangeRef.current = onSelectionChange;
  const finalNodesRef = React.useRef(finalNodes);
  finalNodesRef.current = finalNodes;

  const handleNodesChange = React.useCallback(
    (changes: any[]) => {
      onNodesChangeReactFlowRef.current?.(changes);
      if (!isControlled) {
        setInternalNodes((nds) => applyNodeChanges(changes, nds));
      }
    },
    [isControlled],
  );

  const handleEdgesChange = React.useCallback(
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

  const handleInit = React.useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        (reactFlowInstanceRef as any).current = instance;
      }
      onInitPropRef.current?.(instance);
    },
    [reactFlowInstanceRef],
  );

  const handleNodeDragStart = React.useCallback(
    (event: React.MouseEvent, node: Node) => {
      draggingNodeRef.current = node.id;
      if (forceConfig.enabled && simulationRef.current) {
        const currentPositions = new Map(finalNodesRef.current.map((n) => [n.id, n.position]));
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

  const handleNodeDrag = React.useCallback(
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

  const handleNodeDragStop = React.useCallback(
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

  const stableOnConnect = React.useCallback((connection: any) => { onConnectRef.current?.(connection); }, []);
  const stableOnMoveStart = React.useCallback(() => { onMoveStartRef.current?.(); }, []);
  const stableOnMoveEnd = React.useCallback(() => { onMoveEndRef.current?.(); }, []);
  const stableOnSelectionChange = React.useCallback((selection: OnSelectionChangeParams) => { onSelectionChangeRef.current?.(selection); }, []);

  React.useEffect(() => {
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

    // Run simulation synchronously to completion once
    const numTicks = Math.ceil(Math.log(simulation.alphaMin()) / Math.log(1 - simulation.alphaDecay()));
    for (let i = 0; i < numTicks; i++) {
      simulation.tick();
    }

    // Set final positions once
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

  React.useEffect(() => {
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

  React.useEffect(() => {
    if (!isControlled) {
      setInternalNodes(initialNodes);
      setInternalEdges(initialEdges);
    }
  }, [initialNodes, initialEdges, isControlled]);

  React.useEffect(() => {
    if (!isControlled && onNodesChangeProp) {
      onNodesChangeProp(internalNodes);
    }
  }, [internalNodes, onNodesChangeProp, isControlled]);

  React.useEffect(() => {
    if (!isControlled && onEdgesChangeProp) {
      onEdgesChangeProp(internalEdges);
    }
  }, [internalEdges, onEdgesChangeProp, isControlled]);

  return (
    <div ref={wrapperRef as any} className={`relative w-full h-full ${className}`}>
      <ReactFlow
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
      </ReactFlow>
    </div>
  );
};

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🪨diagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/Diagram)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🪨diagram](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/Diagram)
 * Diagram holds the data fields for a Diagram record.
 **/
const Diagram: React.FC<DiagramProps> = (props) => {
  return (
    <ReactFlowProvider>
      <DiagramInner {...props} />
    </ReactFlowProvider>
  );
};

export { Diagram, SelectionMode };
export type { OnSelectionChangeParams };

/**
 * Hook computing and memoizing diagram layout from nodes and edges.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🛠️usediagramlayout](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/useDiagramLayout)
 **/
export function useDiagramLayout(initialNodes: Node[], initialEdges: Edge[], layoutOptions?: DiagramLayoutOptions): { nodes: Node[]; edges: Edge[] } {
  return React.useMemo(() => {
    if (initialNodes.length === 0) {
      return { nodes: [], edges: [] };
    }
    return calculateDiagramLayout(initialNodes, initialEdges, layoutOptions);
  }, [initialNodes, initialEdges, layoutOptions]);
}

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram✂️diagramskeletonprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramSkeletonProps)
 * DiagramSkeletonProps holds the data fields for a DiagramSkeletonProps record.
 **/
interface DiagramSkeletonProps {
  nodeCount?: number;
  edgeCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a diagram.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖diagram🪨diagramskeleton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Diagram/d/i/DiagramSkeleton)
 **/
export const DiagramSkeleton: React.FC<DiagramSkeletonProps> = ({ nodeCount = 5, edgeCount = 4, className = "" }) => {
  const skeletonNodes: Node[] = React.useMemo(
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
  const skeletonEdges: Edge[] = React.useMemo(
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
      <ReactFlow
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
      ></ReactFlow>
    </div>
  );
};

// #endregion Diagram

// #region Scene

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene)
// 3D scene viewer built on React Three Fiber.
// Consumers MUST provide SceneGeometry data.

export const sceneFrameControlRef: { current: { pause: () => void; resume: () => void } | null } = { current: null };
const SceneFrameControl: React.FC = () => {
  const gl = useThree((s) => s.gl);
  const setFrameloop = useThree((s) => s.setFrameloop);
  const invalidate = useThree((s) => s.invalidate);
  React.useEffect(() => {
    sceneFrameControlRef.current = {
      pause: () => setFrameloop("never"),
      resume: () => { setFrameloop("demand"); invalidate(); },
    };
    return () => { sceneFrameControlRef.current = null; };
  }, [gl, setFrameloop, invalidate]);
  return null;
};

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨selectablecursorusagecount](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/selectableCursorUsageCount)
 * selectableCursorUsageCount holds the data fields for a selectableCursorUsageCount record.
 **/
let selectableCursorUsageCount = 0;

/**
 * Interface for a geometry entry in a 3D scene.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🛠️scenegeometry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/SceneGeometry)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🛠️transformablegeometry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/TransformableGeometry)
 **/
export interface TransformableGeometry extends SceneGeometry {
  isTransformable?: boolean;
}

/**
 * Interface for an incremental plane transformation delta.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🛠️planetransformdelta](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/PlaneTransformDelta)
 **/
export interface PlaneTransformDelta {
  translation?: { x: number; y: number; z: number };
  rotation?: { x: number; y: number; z: number; w: number };
  scale?: number;
}

/**
 * Callback type for a single plane update.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🛠️onplaneupdate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/OnPlaneUpdate)
 **/
export type OnPlaneUpdate = (geometryGuid: string, newPlane: Plane) => void;

/**
 * Callback type for batch plane updates.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🛠️onmultiplaneupdate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/OnMultiPlaneUpdate)
 **/
export type OnMultiPlaneUpdate = (updates: Array<{ geometryGuid: string; newPlane: Plane }>) => void;

/**
 * Constructs a Plane from a point and direction vector.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨planefrompointanddirection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/planeFromPointAndDirection)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨getplaneposition](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/getPlanePosition)
 **/
export const getPlanePosition = (plane: Plane): THREE.Vector3 => {
  return new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
};

/**
 * Checks whether a geometry has a non-null plane.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨hasvalidplane](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/hasValidPlane)
 **/
export const hasValidPlane = (geometry: SceneGeometry): boolean => {
  return geometry.plane !== undefined && geometry.plane !== null;
};

/**
 * Checks whether a geometry has a valid plane for camera focus.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨isgeometryfocusable](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/isGeometryFocusable)
 **/
export const isGeometryFocusable = (geometry: SceneGeometry): boolean => {
  return hasValidPlane(geometry) && (geometry.isFocusable === undefined || geometry.isFocusable === true);
};

/**
 * GeometryProps holds the data fields for a GeometryProps record.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene✂️geometryprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/GeometryProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨geometry](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/Geometry)
 **/
export const Geometry: React.FC<GeometryProps> = ({ children, selected = false, hovered = false, onClick, onDoubleClick, onPointerEnter, onPointerLeave, color, emissiveColor, emissiveIntensity = 0.45, showEdges = true, edgeColor, userData }) => {
  const foregroundColor = React.useMemo(() => getComputedColor("--foreground"), []);
  const activeBaseColor = React.useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = React.useMemo(() => getComputedColor("--hover-base"), []);
  const [isPointerOver, setIsPointerOver] = React.useState(false);
  const isInteractive = Boolean(onClick || onDoubleClick);

  const resolvedColor = React.useMemo(() => {
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    if (color) return color;
    return foregroundColor;
  }, [color, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);

  const resolvedEmissiveColor = React.useMemo(() => {
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    if (emissiveColor) return emissiveColor;
    return resolvedColor;
  }, [selected, hovered, activeBaseColor, hoverBaseColor, emissiveColor, resolvedColor]);
  const resolvedEdgeColor = React.useMemo(() => {
    if (edgeColor) return edgeColor;
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    return foregroundColor;
  }, [edgeColor, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);
  const handlePointerEnter = React.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isInteractive) {
        setIsPointerOver(true);
      }
      onPointerEnter?.(event);
    },
    [isInteractive, onPointerEnter],
  );

  const handlePointerLeave = React.useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (isInteractive) {
        setIsPointerOver(false);
      }
      onPointerLeave?.(event);
    },
    [isInteractive, onPointerLeave],
  );

  React.useEffect(() => {
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene✂️gltfprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/GltfProps)
 * GltfProps holds the data fields for a GltfProps record.
 **/
interface GltfProps {
  src: string;
  roughness?: number;
  metalness?: number;
}

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨getcomputedcolorforgltf](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/getComputedColorForGltf)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨getcomputedcolorforgltf](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/getComputedColorForGltf)
 * getComputedColorForGltf holds the data fields for a getComputedColorForGltf record.
 **/
const getComputedColorForGltf = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨gltf](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/Gltf)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨gltf](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/Gltf)
 * Gltf holds the data fields for a Gltf record.
 **/
const Gltf: React.FC<GltfProps> = ({ src, roughness = 0.8, metalness = 0 }) => {
  const { scene } = useGLTF(src);
  const plasterColor = React.useMemo(() => new THREE.Color(getComputedColorForGltf("--plaster")), []);
  const plasterEdgeColor = React.useMemo(() => new THREE.Color(getComputedColorForGltf("--plaster-edge")), []);

  const clonedScene = React.useMemo(() => {
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene✂️geometryfileprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/GeometryFileProps)
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
// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨geometryfile](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/GeometryFile)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨geometryfile](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/GeometryFile)
 **/
const GeometryFile: React.FC<GeometryFileProps> = ({ src, environment, roughness, metalness }) => {
  return (
    <div className="w-full h-full">
      <Geometry>
        <React.Suspense fallback={null}>
          <Gltf src={src} roughness={roughness} metalness={metalness} />
        </React.Suspense>
      </Geometry>
    </div>
  );
};

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene✂️gizmoprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/GizmoProps)
 * GizmoProps holds the data fields for a GizmoProps record.
 **/
interface GizmoProps {
  show?: boolean;
}

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨gizmo](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/Gizmo)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨gizmo](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/Gizmo)
 * Gizmo holds the data fields for a Gizmo record.
 **/
const Gizmo: React.FC<GizmoProps> = ({ show = true }) => {
  const [colors, setColors] = React.useState<[string, string, string]>(() => [getComputedColor("--accent"), getComputedColor("--accent-tertiary"), getComputedColor("--accent-secondary")]);
  const labels = React.useMemo(() => ["X", "Z", "-Y"] as [string, string, string], []);
  const margin = React.useMemo(() => [80, 80] as [number, number], []);

  React.useEffect(() => {
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
    <GizmoHelper alignment="bottom-right" margin={margin}>
      <GizmoViewport labels={labels} axisColors={colors} />
    </GizmoHelper>
  );
};

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene✂️sceneinnerprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/SceneInnerProps)
 * SceneInnerProps holds the data fields for a SceneInnerProps record.
 **/
interface SceneInnerProps {
  children?: React.ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  focusedItemId?: string;
  onFocusComplete?: () => void;
}

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨sceneinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/SceneInner)
/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨sceneinner](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/SceneInner)
 * SceneInner holds the data fields for a SceneInner record.
 **/
const SceneInner: React.FC<SceneInnerProps> = ({ children, showGrid = true, showGizmo = true, camera: initialCamera, onCameraChange, focusedItemId, onFocusComplete }) => {
  const [gridColors, setGridColors] = React.useState({
    sectionColor: getComputedColor("--foreground"),
    cellColor: getComputedColor("--accent-foreground"),
  });

  React.useEffect(() => {
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
  const controlsRef = React.useRef<any>(null);
  const isUpdatingCameraRef = React.useRef(false);
  const prevCameraStringRef = React.useRef<string | undefined>(initialCamera ? JSON.stringify(initialCamera) : undefined);
  const cameraRestoredRef = React.useRef(false);
  const restoredCameraStringRef = React.useRef<string | undefined>(undefined);

  const cameraRef = React.useRef<THREE.OrthographicCamera>(threeCamera as THREE.OrthographicCamera);

  React.useEffect(() => {
    const cam = cameraRef.current;
    if (cam && cam instanceof THREE.OrthographicCamera) {
      cam.zoom = 50;
      cam.updateProjectionMatrix();
    }
  }, []);

  React.useEffect(() => {
    if (!cameraRef.current || !controlsRef.current) return;

    const currentCameraString = initialCamera ? JSON.stringify(initialCamera) : undefined;

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
        cameraRef.current.updateProjectionMatrix();
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
        cameraRef.current.updateProjectionMatrix();
        controlsRef.current.update();

        setTimeout(() => {
          isUpdatingCameraRef.current = false;
        }, 300);
      });

      cameraRestoredRef.current = true;
      restoredCameraStringRef.current = currentCameraString;
    }
  }, [initialCamera]);

  const handleEnd = React.useCallback(() => {
    if (isUpdatingCameraRef.current) return;
    if (cameraRef.current && controlsRef.current && onCameraChange) {
      const position = cameraRef.current.position;
      const target = controlsRef.current.target;
      const forwardVec = new THREE.Vector3().subVectors(target, position);

      if (forwardVec.lengthSq() < 0.001) return;

      const forward = forwardVec.normalize();
      const up = cameraRef.current.up;
      const newCamera = {
        position: { x: position.x, y: position.y, z: position.z },
        forward: { x: forward.x, y: forward.y, z: forward.z },
        up: { x: up.x, y: up.y, z: up.z },
      };
      onCameraChange(newCamera);
    }
  }, [onCameraChange]);

  React.useEffect(() => {
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
        camera.updateProjectionMatrix();
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
        mouseButtons={{
          LEFT: THREE.MOUSE.ROTATE,
          MIDDLE: undefined,
          RIGHT: undefined,
        }}
        onEnd={handleEnd}
      />
      <ambientLight intensity={1} />
      {children}
      {showGrid && <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />}
      {showGizmo && <Gizmo />}
    </>
  );
};

/**
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene✂️sceneprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/SceneProps)
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
  projection?: "camera" | "orthographic";
  onProjectionChange?: (projection: "camera" | "orthographic") => void;
}

/**
 * 3D scene viewer with orbit controls, grid, and geometry rendering.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨scene](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/Scene)
 **/
export const Scene: React.FC<SceneProps> = ({
  children,
  showGrid = true,
  showGizmo = true,
  camera,
  onCameraChange,
  onDoubleClickCapture,
  onPointerMissed,
  orthographic = true,
  shadows = false,
  className = "",
  focusedItemId,
  onFocusComplete,
  projection = "orthographic",
  onProjectionChange,
}) => {
  const projectionOptions: ActionDropdownOption[] = [
    {
      value: "camera",
      icon: <CameraIcon className="size-3" />,
      label: "Camera",
    },
    {
      value: "orthographic",
      icon: <GripVerticalIcon className="size-3" />,
      label: "Orthographic",
    },
  ];

  return (
    <div className={`relative h-full w-full ${className}`} style={{ minHeight: "100%", minWidth: "100%" }} onDoubleClick={onDoubleClickCapture}>
      {onProjectionChange && (
        <div className="absolute top-1 right-1 z-panel">
          <ActionDropdown id="scene-projection" options={projectionOptions} value={projection} onValueChange={(value) => onProjectionChange(value as "camera" | "orthographic")} />
        </div>
      )}
      <ThreeCanvas onPointerMissed={onPointerMissed} orthographic={orthographic} shadows={shadows} frameloop="demand" camera={orthographic ? { zoom: 50, position: [10, 10, 10], near: -10000, far: 10000 } : undefined} style={{ width: "100%", height: "100%" }}>
        <SceneFrameControl />
        <SceneInner showGrid={showGrid} showGizmo={showGizmo} camera={camera} onCameraChange={onCameraChange} focusedItemId={focusedItemId} onFocusComplete={onFocusComplete}>
          {children}
        </SceneInner>
      </ThreeCanvas>
    </div>
  );
};

/**
 * Skeleton loading placeholder for a 3D scene.
 *
 *[👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖scene🪨sceneskeleton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Scene/d/i/SceneSkeleton)
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

// #endregion Scene

// #region Table

// [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table)
// Sortable, hierarchical data table with drag-drop support.
// Consumers MUST provide columns and data arrays.

/**
 * Union type for ascending or descending sort order.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🛠️sortdirection](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/SortDirection)
 **/
export type SortDirection = "asc" | "desc";

/**
 * Configuration interface for a table column definition.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🛠️tablecolumn](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/TableColumn)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🛠️hierarchicalrowdata](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/HierarchicalRowData)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🛠️dragdropconfig](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/DragDropConfig)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🛠️tableprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/TableProps)
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🪨table](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/Table)
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
  const scrollAreaRef = React.useRef<HTMLDivElement>(null);
  const [activeId, setActiveId] = React.useState<string | null>(null);
  const level = useLevel();
  const headerBgClass = {
    base: "bg-base",
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

  React.useEffect(() => {
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
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🛠️tableskeletonprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/TableSkeletonProps)
 **/
export interface TableSkeletonProps {
  columns: TableColumn[];
  rowCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a table.
 * [👤semio📚js🗃️sketchpad💻elements🔖windowcomponents🔖table🪨tableskeleton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/elements.tsx/s/Window%20Components/s/Table/d/i/TableSkeleton)
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

// #endregion Table

// #region Canvas

/**
 * Container component for canvas window layout.
 **/
export const Canvas: React.FC<{ children: React.ReactNode; id?: string }> = ({ children, id }) => {
  return (
    <div id={id} className="h-full w-full box-border p-single">
      {children}
    </div>
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

// #endregion Canvas

// #endregion Window Components

// #region UI

// Domain-neutral composite component providing a full application shell.
// An app has window kinds (rendered with golden-layout) and registers
// left/right side panel tabs and footer items.
// Every UI has a toolbar, a search (Ctrl+P command palette), panel toggles, and breadcrumb.
// Every app has a find (Ctrl+F scoped command palette).
// Every panel has a tree.

/**
 * Window kind classification for app windows.
 **/
export enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
  SETTINGS = "settings",
  CHAT = "chat",
  WORKBENCH = "workbench",
}

/**
 * UI theme classification.
 **/
export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

/**
 * UI interaction mode.
 **/
export enum Mode {
  VIEW = "view",
  EDIT = "edit",
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
 * Definition of a window kind with label, icon, component, and controls.
 * Each window kind can be registered with golden-layout.
 **/
export interface UIWindowKindDefinition {
  id: string;
  label?: string;
  icon?: React.ReactNode;
  component: React.ComponentType<any>;
  controls?: UIWindowControl[];
  variants?: {
    id: string;
    icon?: React.ReactNode;
    componentProps?: Record<string, any>;
  }[];
}

/**
 * App-level window configuration with window kinds and default golden-layout config.
 **/
export interface UIWindowConfig {
  windowKinds: UIWindowKindDefinition[];
  defaultLayout?: any;
}

/**
 * Creates a default golden-layout configuration from window IDs and direction.
 * MUST generate a golden-layout config with one stack per window ID.
 **/
export function createDefaultLayout(windowIds: string[], direction: "row" | "column" = "row", sizes?: number[], titles?: string[]): any {
  return {
    root: {
      type: direction === "row" ? "row" : "column",
      content: windowIds.map((id, index) => ({
        type: "stack",
        content: [
          {
            type: "component",
            componentName: id,
            title: titles?.[index] ?? id,
            componentState: {},
          },
        ],
        ...(sizes?.[index] !== undefined ? { size: `${sizes[index]}%` } : {}),
      })),
    },
  };
}

/**
 * Parses a window layout from a string, object, or undefined input.
 * MUST return undefined for null, empty, or unparseable inputs.
 **/
export function parseWindowLayout(layout: unknown): any | undefined {
  if (layout === undefined || layout === null) return undefined;
  if (typeof layout === "string") {
    const trimmed = layout.trim();
    if (!trimmed) return undefined;
    try {
      return JSON.parse(trimmed);
    } catch {
      return undefined;
    }
  }
  if (typeof layout === "object") return layout;
  return undefined;
}

/**
 * Serializes a window layout to a JSON string.
 * MUST return undefined when serialization fails.
 **/
export function stringifyWindowLayout(layout: unknown): string | undefined {
  if (layout === undefined || layout === null) return undefined;
  try {
    return JSON.stringify(layout);
  } catch {
    return undefined;
  }
}

/**
 * Removes duplicate and disallowed window components from a layout.
 **/
export function deduplicateWindowLayout(layout: any, allowedWindowIds: string[]): any | undefined {
  if (!layout || typeof layout !== "object") return layout;
  const seenComponents = new Set<string>();
  const deduplicateContent = (content: any[]): any[] => {
    if (!Array.isArray(content)) return content;
    return content
      .map((item) => {
        if (!item || typeof item !== "object") return item;
        if (item.type === "component") {
          const componentName = item.componentName;
          if (seenComponents.has(componentName) || !allowedWindowIds.includes(componentName)) return null;
          seenComponents.add(componentName);
          return item;
        }
        if (item.content && Array.isArray(item.content)) {
          const deduped = deduplicateContent(item.content);
          if (deduped.length === 0) return null;
          return { ...item, content: deduped };
        }
        return item;
      })
      .filter((item) => item !== null);
  };
  const root = layout.root;
  if (!root || typeof root !== "object") return layout;
  if (root.content && Array.isArray(root.content)) {
    const dedupedContent = deduplicateContent(root.content);
    return { ...layout, root: { ...root, content: dedupedContent } };
  }
  return layout;
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

/**
 * Golden-layout canvas that renders window kinds.
 * Dynamically imports golden-layout and registers each window kind as a component.
 **/
const UICanvas: React.FC<{
  windowConfig: UIWindowConfig;
  layoutState?: any;
  onLayoutChange?: (config: any) => void;
  onActiveWindowChange?: (windowId: string) => void;
}> = ({ windowConfig, layoutState, onLayoutChange, onActiveWindowChange }) => {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const layoutRef = React.useRef<any>(null);

  React.useEffect(() => {
    if (!containerRef.current || layoutRef.current) return;

    const loadGoldenLayout = async () => {
      try {
        const goldenLayoutModule = await import("golden-layout");
        const GoldenLayout = (goldenLayoutModule as any).GoldenLayout;
        if (!GoldenLayout || typeof GoldenLayout !== "function") {
          console.error("[UICanvas] GoldenLayout is not a constructor");
          return;
        }

        const normalizeLayoutConfig = (config: any): any => {
          if (Array.isArray(config)) return config.map(normalizeLayoutConfig);
          const normalized: any = {};
          for (const [key, value] of Object.entries(config)) {
            const unitKey = `${key}Unit`;
            if (unitKey in config) {
              const unit = (config as any)[unitKey];
              normalized[key] = typeof value === "number" ? `${value}${unit}` : typeof value === "string" ? `${parseFloat(value) || 1}${unit}` : `1${unit}`;
            } else if ((key === "size" || key === "width" || key === "height") && typeof value === "number") {
              normalized[key] = `${value}%`;
            } else if (key === "content" && Array.isArray(value)) {
              normalized[key] = value.map(normalizeLayoutConfig);
            } else if (typeof value === "object" && value !== null && !Array.isArray(value)) {
              normalized[key] = normalizeLayoutConfig(value);
            } else {
              normalized[key] = value;
            }
          }
          return normalized;
        };

        const rawConfig = parseWindowLayout(layoutState) || parseWindowLayout(windowConfig.defaultLayout);
        if (!rawConfig) {
          console.error("[UICanvas] No layout config");
          return;
        }
        const config = normalizeLayoutConfig(rawConfig);

        const layout = new GoldenLayout(config, containerRef.current!);
        let isInitialized = false;

        windowConfig.windowKinds.forEach((windowKind) => {
          layout.registerComponent(windowKind.id, (container: any) => {
            const element = container.getElement();
            let domElement: HTMLElement;
            if (element instanceof HTMLElement) {
              domElement = element;
            } else if (Array.isArray(element) && element[0] instanceof HTMLElement) {
              domElement = element[0];
            } else if (element?.[0] instanceof HTMLElement) {
              domElement = element[0];
            } else if (element?.nodeType === 1) {
              domElement = element as HTMLElement;
            } else {
              console.error("[UICanvas] Could not extract DOM element from container");
              return;
            }

            const root = createRoot(domElement);
            const WindowComponent = windowKind.component;

            const clickGoldenLayoutControl = (selector: string) => {
              const stackElement = domElement.closest(".lm_item.lm_stack") as HTMLElement | null;
              const controlElement = stackElement?.querySelector(selector) as HTMLElement | null;
              controlElement?.click();
            };

            root.render(
              <Window
                id={windowKind.id}
                isVisible={true}
                showControls={true}
                onOpenInNewWindow={() => clickGoldenLayoutControl(".lm_popout")}
                onMaximize={() => clickGoldenLayoutControl(".lm_maximise")}
                onMinimize={() => clickGoldenLayoutControl(".lm_maximise")}
                onClose={() => clickGoldenLayoutControl(".lm_close")}
                controls={windowKind.controls ? <UIWindowControlsGroup controls={windowKind.controls} /> : undefined}
              >
                <WindowComponent />
              </Window>,
            );

            container.on("destroy", () => {
              setTimeout(() => root.unmount(), 0);
            });
          });
        });

        layout.on("stateChanged", () => {
          if (!onLayoutChange || !isInitialized) return;
          try {
            onLayoutChange(layout.toConfig());
          } catch (error: any) {
            if (!error?.message?.includes("not yet initialised")) {
              console.warn("[UICanvas] Failed to get layout config:", error);
            }
          }
        });

        layout.on("tab", (tab: any) => {
          if (tab._header) {
            tab._header.on("click", () => {
              const componentName = tab._contentItem?.config?.componentName;
              if (componentName && onActiveWindowChange) onActiveWindowChange(componentName);
            });
          }
        });

        layout.init();
        isInitialized = true;
        layoutRef.current = layout;

        const handleResize = () => layout.updateSize();
        window.addEventListener("resize", handleResize);

        return () => {
          window.removeEventListener("resize", handleResize);
          try {
            layout.destroy();
          } catch { }
          layoutRef.current = null;
        };
      } catch (error) {
        console.error("[UICanvas] Failed to load GoldenLayout:", error);
      }
    };

    loadGoldenLayout();
  }, [windowConfig, layoutState, onLayoutChange, onActiveWindowChange]);

  return <div ref={containerRef} className="w-full h-full" />;
};

// #region UISearch

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
  const [query, setQuery] = React.useState("");

  const fuse = React.useMemo(
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

  const results = React.useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return items.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UISearchItem>);
  }, [fuse, query, items]);

  const grouped = React.useMemo(() => {
    const groups: Record<string, FuseResult<UISearchItem>[]> = {};
    results.forEach((result) => {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [results]);

  const handleSelect = React.useCallback(
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

// #endregion UISearch

// #region UIFind

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

const UIFindContext = React.createContext<UIFindContextValue | null>(null);

/**
 * Provider for per-app find functionality.
 * Wraps children and exposes find items + trigger via context.
 **/
export const UIFindProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [findItems, setFindItems] = React.useState<UIFindItem[]>([]);
  const onFindItemCallbackRef = React.useRef<((itemId: string) => void) | undefined>(undefined);

  const setFindItemsStable = React.useCallback((items: UIFindItem[]) => {
    setFindItems(items);
  }, []);

  const setOnFindItem = React.useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);

  const triggerFindItem = React.useCallback((itemId: string) => {
    if (onFindItemCallbackRef.current) {
      onFindItemCallbackRef.current(itemId);
    }
  }, []);

  const contextValue = React.useMemo(
    () => ({ findItems, setFindItems: setFindItemsStable, setOnFindItem, triggerFindItem }),
    [findItems, setFindItemsStable, setOnFindItem, triggerFindItem],
  );
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
};

/**
 * Hook to access the find context. Throws if used outside UIFindProvider.
 **/
export function useUIFind(): UIFindContextValue {
  const context = React.useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

/**
 * Hook to access the find context. Returns null if outside UIFindProvider.
 **/
export function useUIFindSafe(): UIFindContextValue | null {
  return React.useContext(UIFindContext);
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
  const [query, setQuery] = React.useState("");
  const findContext = React.useContext(UIFindContext);
  const findItems = findContext?.findItems || [];
  const triggerFindItem = findContext?.triggerFindItem;

  const fuse = React.useMemo(
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

  const results = React.useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return findItems.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UIFindItem>);
  }, [fuse, query, findItems]);

  const grouped = React.useMemo(() => {
    const groups: Record<string, FuseResult<UIFindItem>[]> = {};
    results.forEach((result) => {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [results]);

  const handleSelect = React.useCallback(
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

// #endregion UIFind

// #region UIToolbar

/**
 * A toolbar action item registered by an app or the UI.
 **/
export interface UIToolbarItem {
  id: string;
  icon?: React.ReactNode;
  label?: string;
  onClick?: () => void;
  kind?: "button" | "toggle" | "separator";
  pressed?: boolean;
  onPressedChange?: (pressed: boolean) => void;
  order?: number;
}

/**
 * Renders a floating toolbar zone with structured items.
 * Each app can provide toolbar items; the UI merges them with global items.
 **/
const UIToolbar: React.FC<{
  items: UIToolbarItem[];
  className?: string;
}> = ({ items, className }) => {
  const sorted = React.useMemo(() => [...items].sort((a, b) => (a.order ?? 0) - (b.order ?? 0)), [items]);

  if (sorted.length === 0) return null;

  return (
    <div className={cn("flex items-center justify-center pointer-events-none", className)}>
      <ToolbarZone className="pointer-events-auto">
        {sorted.map((item) => {
          if (item.kind === "separator") {
            return <ToolbarDivider key={item.id} />;
          }
          if (item.kind === "toggle") {
            return (
              <ToolbarItem key={item.id}>
                <Toggle
                  kind="icon"
                  id={item.id}
                  pressed={item.pressed ?? false}
                  onPressedChange={(p) => item.onPressedChange?.(p)}
                  icon={item.icon}
                />
              </ToolbarItem>
            );
          }
          return (
            <ToolbarItem key={item.id}>
              <button
                onClick={item.onClick}
                className="flex items-center gap-single px-single py-tiny hover:bg-hover-panel rounded text-sm cursor-pointer"
              >
                {item.icon}
                {item.label && <span>{item.label}</span>}
              </button>
            </ToolbarItem>
          );
        })}
      </ToolbarZone>
    </div>
  );
};

// #endregion UIToolbar

/**
 * Configuration for a single app registered with the UI.
 * An app has window kinds (with golden-layout) and registers
 * left/right side panel tabs, footer items, toolbar items, and find items.
 **/
export interface UIAppConfig {
  id: string;
  label: string;
  icon?: React.ReactNode;
  windowConfig: UIWindowConfig;
  leftPanelTabs?: SidePanelTabConfig[];
  rightPanelTabs?: SidePanelTabConfig[];
  toolbarContent?: React.ReactNode;
  toolbarItems?: UIToolbarItem[];
  footerItems?: FooterItem[];
  findItems?: UIFindItem[];
  onFindSelect?: (itemId: string) => void;
}

/**
 * Props for the UI composite component.
 * Navbar is fixed: [breadcrumb (flex-1)] [search] [find] [left panel toggle] [right panel toggle].
 **/
export interface UIProps {
  apps: UIAppConfig[];
  defaultAppId?: string;
  breadcrumbItems?: BreadcrumbItemData[];
  footerItems?: FooterItem[];
  searchItems?: UISearchItem[];
  toolbarItems?: UIToolbarItem[];
  className?: string;
}

/**
 * Panel visibility state for the UI.
 **/
export interface UIPanelVisibility {
  leftSidePanel: boolean;
  rightSidePanel: boolean;
}

/**
 * Context for the active UI app state.
 **/
interface UIContextValue {
  activeAppId: string;
  setActiveAppId: (id: string) => void;
  apps: UIAppConfig[];
  panelVisibility: UIPanelVisibility;
  togglePanel: (panel: keyof UIPanelVisibility) => void;
}

const UIContext = React.createContext<UIContextValue | undefined>(undefined);

/**
 * Hook to access the UI context.
 **/
export function useUI(): UIContextValue {
  const ctx = React.useContext(UIContext);
  if (!ctx) throw new Error("useUI must be used within a UI component");
  return ctx;
}

/**
 * Left panel toggle for the navbar.
 * Uses the first tab icon as the toggle icon.
 * Styled to match sketchpad: border border-element, h-medium.
 **/
const UILeftPanelToggle: React.FC<{
  tabs?: SidePanelTabConfig[];
  visible: boolean;
  onToggle: () => void;
}> = ({ tabs, visible, onToggle }) => {
  if (!tabs || tabs.length === 0) return null;
  const Icon = tabs[0]?.icon;
  return (
    <div className="flex items-stretch border border-element overflow-hidden h-medium">
      <Toggle
        kind="icon"
        id="ui.panelToggle.left"
        pressed={visible}
        onPressedChange={onToggle}
        className="border-0"
        icon={Icon ? <Icon size={16} /> : <ChevronLeftIcon className="size-small" />}
      />
    </div>
  );
};

/**
 * Right panel toggle for the navbar.
 * Uses the first tab icon as the toggle icon.
 * Styled to match sketchpad: border border-element, h-medium.
 **/
const UIRightPanelToggle: React.FC<{
  tabs?: SidePanelTabConfig[];
  visible: boolean;
  onToggle: () => void;
}> = ({ tabs, visible, onToggle }) => {
  if (!tabs || tabs.length === 0) return null;
  const Icon = tabs[0]?.icon;
  return (
    <div className="flex items-stretch border border-element overflow-hidden h-medium">
      <Toggle
        kind="icon"
        id="ui.panelToggle.right"
        pressed={visible}
        onPressedChange={onToggle}
        className="border-0"
        icon={Icon ? <Icon size={16} /> : <ChevronRightIcon className="size-small" />}
      />
    </div>
  );
};

/**
 * Domain-neutral composite component providing a full application shell.
 * The UI only has apps. An app has window kinds (rendered with golden-layout)
 * and registers left/right side panel tabs, footer items, toolbar items, and find items.
 * Every UI has: toolbar, search (Ctrl+P), breadcrumb, panel toggles.
 * Every app has: find (Ctrl+F).
 * Every panel has: tree.
 * Fixed navbar layout: [breadcrumb (flex-1)] [search] [find] [left panel toggle] [right panel toggle].
 **/
export const UI: React.FC<UIProps> = ({
  apps,
  defaultAppId,
  breadcrumbItems = [],
  footerItems: globalFooterItems = [],
  searchItems = [],
  toolbarItems: globalToolbarItems = [],
  className,
}) => {
  const [activeAppId, setActiveAppId] = React.useState(defaultAppId ?? apps[0]?.id ?? "");
  const [leftPanelSize, setLeftPanelSize] = React.useState(280);
  const [rightPanelSize, setRightPanelSize] = React.useState(300);
  const [panelVisibility, setPanelVisibility] = React.useState<UIPanelVisibility>({ leftSidePanel: false, rightSidePanel: true });
  const [searchOpen, setSearchOpen] = React.useState(false);
  const [findOpen, setFindOpen] = React.useState(false);

  // Keyboard shortcuts: Ctrl+P for search, Ctrl+F for find
  React.useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      if (event.ctrlKey || event.metaKey) {
        if (event.key.toLowerCase() === "p") {
          const activeEl = document.activeElement as HTMLElement | null;
          if (!searchOpen && activeEl && (activeEl.tagName === "INPUT" || activeEl.tagName === "TEXTAREA" || activeEl.isContentEditable)) return;
          event.preventDefault();
          event.stopPropagation();
          setSearchOpen((prev) => !prev);
        } else if (event.key === "f") {
          event.preventDefault();
          setFindOpen((prev) => !prev);
        }
      }
    };
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [searchOpen]);

  const togglePanel = React.useCallback((panel: keyof UIPanelVisibility) => {
    setPanelVisibility((prev) => ({ ...prev, [panel]: !prev[panel] }));
  }, []);

  const activeApp = apps.find((a) => a.id === activeAppId) ?? apps[0];
  if (!activeApp) return null;

  const hasLeftPanel = activeApp.leftPanelTabs && activeApp.leftPanelTabs.length > 0;
  const hasRightPanel = activeApp.rightPanelTabs && activeApp.rightPanelTabs.length > 0;

  // Merge toolbar items: global + app-specific
  const mergedToolbarItems = [...globalToolbarItems, ...(activeApp.toolbarItems ?? [])].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

  // Fixed navbar: [breadcrumb (flex-1)] [search] [find] [left panel toggle] [right panel toggle]
  const navbarItems: NavbarItem[] = [];

  // Breadcrumb (fills remaining space, always first)
  navbarItems.push({
    key: "breadcrumb",
    className: "flex-1 min-w-0",
    content: breadcrumbItems.length > 0 ? <Breadcrumb items={breadcrumbItems} /> : null,
  });

  // Search toggle
  navbarItems.push({
    key: "search",
    content: (
      <Toggle
        kind="icon"
        id="ui.search.toggle"
        pressed={searchOpen}
        onPressedChange={setSearchOpen}
        icon={<SearchIcon size={16} />}
      />
    ),
  });

  // Find toggle
  navbarItems.push({
    key: "find",
    content: (
      <Toggle
        kind="icon"
        id="ui.find.toggle"
        pressed={findOpen}
        onPressedChange={setFindOpen}
        icon={<SearchIcon size={16} />}
      />
    ),
  });

  // Left panel toggle
  navbarItems.push({
    key: "leftPanel",
    content: (
      <UILeftPanelToggle
        tabs={activeApp.leftPanelTabs}
        visible={panelVisibility.leftSidePanel}
        onToggle={() => togglePanel("leftSidePanel")}
      />
    ),
  });

  // Right panel toggle
  navbarItems.push({
    key: "rightPanel",
    content: (
      <UIRightPanelToggle
        tabs={activeApp.rightPanelTabs}
        visible={panelVisibility.rightSidePanel}
        onToggle={() => togglePanel("rightSidePanel")}
      />
    ),
  });

  const mergedFooterItems = [...globalFooterItems, ...(activeApp.footerItems ?? [])].sort((a, b) => (a.order ?? 0) - (b.order ?? 0));

  // Determine toolbar: structured items take precedence, then toolbarContent fallback
  const toolbarElement = mergedToolbarItems.length > 0 ? <UIToolbar items={mergedToolbarItems} /> : activeApp.toolbarContent;

  return (
    <UIContext.Provider value={{ activeAppId, setActiveAppId, apps, panelVisibility, togglePanel }}>
      <UIFindProvider>
        <UIFindItemsSync findItems={activeApp.findItems} onFindSelect={activeApp.onFindSelect} />
        <Layout
          className={className}
          navbar={<Navbar items={navbarItems} />}
          footer={mergedFooterItems.length > 0 ? <Footer items={mergedFooterItems} /> : undefined}
          toolbar={toolbarElement}
          leftSidePanel={
            hasLeftPanel
              ? {
                position: "left" as const,
                visible: panelVisibility.leftSidePanel,
                size: leftPanelSize,
                onSizeChange: setLeftPanelSize,
                tabs: activeApp.leftPanelTabs!,
              }
              : undefined
          }
          rightSidePanel={
            hasRightPanel
              ? {
                position: "right" as const,
                visible: panelVisibility.rightSidePanel,
                size: rightPanelSize,
                onSizeChange: setRightPanelSize,
                tabs: activeApp.rightPanelTabs!,
              }
              : undefined
          }
          canvas={<UICanvas windowConfig={activeApp.windowConfig} />}
        />
        {searchItems.length > 0 && <UISearch items={searchItems} open={searchOpen} onOpenChange={setSearchOpen} />}
        <UIFind open={findOpen} onOpenChange={setFindOpen} />
      </UIFindProvider>
    </UIContext.Provider>
  );
};

/**
 * Internal component that syncs app-level find items into the UIFindContext.
 * Automatically updates find items and callback when the active app changes.
 **/
const UIFindItemsSync: React.FC<{
  findItems?: UIFindItem[];
  onFindSelect?: (itemId: string) => void;
}> = ({ findItems, onFindSelect }) => {
  const findCtx = React.useContext(UIFindContext);
  React.useEffect(() => {
    if (findCtx) {
      findCtx.setFindItems(findItems ?? []);
      findCtx.setOnFindItem(onFindSelect);
    }
  }, [findItems, onFindSelect, findCtx]);
  return null;
};

// #endregion UI

const treeVitest = (import.meta as ImportMeta & {
  vitest?: {
    describe: typeof import("vitest").describe;
    expect: typeof import("vitest").expect;
    it: typeof import("vitest").it;
  };
}).vitest;

if (treeVitest) {
  const { describe, expect, it } = treeVitest;

  describe("tree helpers", () => {
    it("normalizes selected ids for single and multiple selection", () => {
      expect(normalizeTreeSelectedIds(["a", "a", "b"], "single")).toEqual(["a"]);
      expect(normalizeTreeSelectedIds(["a", "a", "b"], "multiple")).toEqual(["a", "b"]);
    });

    it("computes additive and range multi selection", () => {
      expect(getTreeNextSelectionState({
        selectionMode: "multiple",
        selectedIds: ["a"],
        orderedIds: ["a", "b", "c", "d"],
        targetId: "c",
        anchorId: "a",
        additiveKey: false,
        rangeKey: true,
      })).toEqual({ selectedIds: ["a", "b", "c"], anchorId: "a" });

      expect(getTreeNextSelectionState({
        selectionMode: "multiple",
        selectedIds: ["a"],
        orderedIds: ["a", "b", "c", "d"],
        targetId: "c",
        anchorId: "a",
        additiveKey: true,
        rangeKey: false,
      })).toEqual({ selectedIds: ["a", "c"], anchorId: "c" });
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
  });
}
