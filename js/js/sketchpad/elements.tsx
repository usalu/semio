// #region Header

// elements.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

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
import { Canvas as ThreeCanvas, ThreeEvent, useThree } from "@react-three/fiber";
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
  MoreHorizontalIcon,
  RemoveIcon,
  SearchIcon,
  TriangleAlertIcon,
  TutorialIcon,
} from "@semio/assets";
import { BackgroundVariant, Edge, EdgeTypes, Handle, MiniMap, Node, NodeTypes, Position, ReactFlow, ReactFlowInstance, ReactFlowProvider, useEdgesState, useNodesState } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { cva, type VariantProps } from "class-variance-authority";
import { Command as CommandPrimitive } from "cmdk";
import * as dagre from "dagre";
import * as React from "react";
import { useTranslation } from "react-i18next";
import * as ResizablePrimitive from "react-resizable-panels";
import { Link, useLocation, useNavigate } from "react-router";
import * as THREE from "three";
import { Expertise, setExpertiseProvider, useLabel } from "../i18n";
import { Camera, cn, Plane, Point, Vector } from "../semio";

// #endregion Imports

// #region Interaction Context
// Generic interaction tracking system for UI elements
// This allows elements to track focus/active states without coupling to specific app logic

interface InteractionCommands {
  setActiveInteraction: (elementId?: string, interactionId?: string) => void;
}

const InteractionContext = React.createContext<InteractionCommands | undefined>(undefined);
const ActiveInteractionContext = React.createContext<string | undefined>(undefined);

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

const useInteractionCommands = () => React.useContext(InteractionContext);
const useActiveInteraction = () => React.useContext(ActiveInteractionContext);

// #endregion Interaction Context

// #region Level Context

export type Level = "base" | "panel" | "temporary";

const LevelContext = React.createContext<Level>("base");

export const LevelProvider: React.FC<{
  level: Level;
  children: React.ReactNode;
}> = ({ level, children }) => {
  return <LevelContext.Provider value={level}>{children}</LevelContext.Provider>;
};

export const useLevel = () => React.useContext(LevelContext);

// #endregion Level Context

// #region Element

export interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

export interface ElementBaseProps {
  id: string;
  level?: Level;
}

export interface ElementProps extends ElementBaseProps {
  transaction?: Transaction;
}

export const useElementLevel = (propLevel?: Level): Level => {
  const contextLevel = useLevel();
  return propLevel ?? contextLevel;
};

// #endregion Element

// #region Root Components

// #endregion Canvas

// #region Command

function Command({ className, ...props }: React.ComponentProps<typeof CommandPrimitive>) {
  return <CommandPrimitive data-slot="command" className={cn("bg-popover text-popover-foreground flex h-full w-full flex-col overflow-hidden", className)} {...props} />;
}

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

function CommandInput({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.Input>) {
  return (
    <div data-slot="command-input-wrapper" className="flex h-medium items-center gap-single border-b px-tiny">
      <SearchIcon className="size-small shrink-0 opacity-50" />
      <CommandPrimitive.Input data-slot="command-input" className={cn("placeholder:text-muted-foreground flex h-medium w-full bg-transparent text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50", className)} {...props} />
    </div>
  );
}

function CommandList({ className, ...props }: React.ComponentProps<typeof CommandPrimitive.List>) {
  return <CommandPrimitive.List data-slot="command-list" className={cn("max-h-[300px] scroll-py-single overflow-x-hidden overflow-y-auto", className)} {...props} />;
}

function CommandEmpty({ ...props }: React.ComponentProps<typeof CommandPrimitive.Empty>) {
  return <CommandPrimitive.Empty data-slot="command-empty" className="py-medium text-center text-sm" {...props} />;
}

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

function CommandShortcut({ className, ...props }: React.ComponentProps<"span">) {
  return <span data-slot="command-shortcut" className={cn("text-muted-foreground ml-auto text-xs tracking-widest", className)} {...props} />;
}

// #endregion Command

export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };

// #region Footer

// #region Footer

export interface FooterItem {
  id: string;
  content: React.ReactNode;
  order?: number;
  onClick?: () => void;
  className?: string;
}

export interface FooterProps {
  items?: FooterItem[];
  className?: string;
  height?: number;
  isVisible?: boolean;
}

const Footer: React.FC<FooterProps> = ({ items = [], className = "", height = 20, isVisible = true }) => {
  const sortedItems = [...items].sort((a, b) => (a.order || 0) - (b.order || 0));
  return (
    <footer className={`bg-base border-t flex items-center transition-transform duration-200 ${isVisible ? "translate-y-0" : "translate-y-full"} ${className}`} style={{ height: `${height}px` }}>
      {sortedItems.map((item, index) => (
        <div key={item.id} className="flex items-center h-full">
          {index > 0 && <div className="h-full w-px bg-border" />}
          {item.id ? (
            <Tooltip>
              <TooltipTrigger asChild>
                <div className={`flex items-center h-full px-single text-xs cursor-pointer ${item.className || ""}`} onClick={item.onClick}>
                  {item.content}
                </div>
              </TooltipTrigger>
              <TooltipContent>{item.id}</TooltipContent>
            </Tooltip>
          ) : (
            <div className={`flex items-center h-full px-single text-xs ${item.onClick ? "cursor-pointer" : ""} ${item.className || ""}`} onClick={item.onClick}>
              {item.content}
            </div>
          )}
        </div>
      ))}
    </footer>
  );
};

export { Footer };

// #endregion Footer

// #region Layout

export interface LayoutProps {
  navbar?: React.ReactNode;
  footer?: React.ReactNode;
  leftPanel?: LeftPanelProps;
  middlePanel?: MiddlePanelProps;
  rightPanel?: RightPanelProps;
  bottomPanel?: BottomPanelProps;
  canvas: React.ReactNode;
  className?: string;
}

const Layout: React.FC<LayoutProps> = ({ navbar, footer, leftPanel, middlePanel, rightPanel, bottomPanel, canvas, className = "" }) => (
  <div className={`flex flex-col h-screen w-screen overflow-hidden ${className}`}>
    {navbar && <div className="flex-shrink-0">{navbar}</div>}
    <div className="flex flex-1 min-h-0 relative">
      {leftPanel && leftPanel.visible && <LeftPanel {...leftPanel} />}
      <div className="flex flex-col flex-1 min-w-0 relative">
        <div className="flex flex-1 min-h-0 relative">
          {middlePanel && middlePanel.visible && <MiddlePanel {...middlePanel} />}
          <div className="flex-1 min-w-0 min-h-0">{canvas}</div>
          {rightPanel && rightPanel.visible && <RightPanel {...rightPanel} />}
        </div>
        {bottomPanel && bottomPanel.visible && <BottomPanel {...bottomPanel} />}
      </div>
    </div>
    {footer && <div className="flex-shrink-0">{footer}</div>}
  </div>
);

export { Layout };

// #endregion Layout

// #region Navbar

export interface NavbarItem {
  id: string;
  content: React.ReactNode;
  onClick?: () => void;
  className?: string;
  order?: number;
}

export interface NavbarProps {
  leftItems?: NavbarItem[];
  centerItems?: NavbarItem[];
  rightItems?: NavbarItem[];
  className?: string;
  height?: number;
  isExpanded?: boolean;
}

const Navbar: React.FC<NavbarProps> = ({ leftItems = [], centerItems = [], rightItems = [], className = "", height, isExpanded = false }) => {
  const sortedLeft = [...leftItems].sort((a, b) => (a.order || 0) - (b.order || 0));
  const sortedCenter = [...centerItems].sort((a, b) => (a.order || 0) - (b.order || 0));
  const sortedRight = [...rightItems].sort((a, b) => (a.order || 0) - (b.order || 0));
  return (
    <nav id="navbar" className={`bg-base border-b flex items-center gap-single px-single h-large z-[100] ${className}`} style={height ? { height: `${height}px`, transition: "height 150ms" } : { transition: "height 150ms" }}>
      {sortedLeft.map((item) => (
        <div key={item.id} className={`flex items-center ${item.className || ""}`} onClick={item.onClick}>
          {item.content}
        </div>
      ))}
      {sortedCenter.map((item) => (
        <div key={item.id} className={`flex items-center ${item.className || ""}`} onClick={item.onClick}>
          {item.content}
        </div>
      ))}
      {sortedRight.length > 0 && (
        <div className="ml-auto flex items-center gap-single">
          {sortedRight.map((item) => (
            <div key={item.id} className={`flex items-center ${item.className || ""}`} onClick={item.onClick}>
              {item.content}
            </div>
          ))}
        </div>
      )}
    </nav>
  );
};

export { Navbar };

// #endregion Navbar

// #region Popover

function Popover({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Root>) {
  return <PopoverPrimitive.Root data-slot="popover" {...props} />;
}

function PopoverTrigger({ className, ...props }: React.ComponentProps<typeof PopoverPrimitive.Trigger>) {
  return <PopoverPrimitive.Trigger data-slot="popover-trigger" className={cn(className)} {...props} />;
}

function PopoverContent({ className, align = "center", sideOffset = 4, ...props }: React.ComponentProps<typeof PopoverPrimitive.Content>) {
  return (
    <PopoverPrimitive.Portal>
      <PopoverPrimitive.Content
        data-slot="popover-content"
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-[100] w-72 origin-(--radix-popover-content-transform-origin) border p-1 outline-hidden",
          className,
        )}
        {...props}
      />
    </PopoverPrimitive.Portal>
  );
}

function PopoverAnchor({ ...props }: React.ComponentProps<typeof PopoverPrimitive.Anchor>) {
  return <PopoverPrimitive.Anchor data-slot="popover-anchor" {...props} />;
}

export { Popover, PopoverAnchor, PopoverContent, PopoverTrigger };

// #endregion Popover

// #endregion Root Components

// #region Display Components

// #region Tooltip

export interface TooltipConfig {
  labelKey: string;
  manualPath?: string;
  tutorialPath?: string;
  hotkey?: string;
}

export interface IdTooltipData {
  label?: string;
  description?: string;
  descriptionBeginner?: string;
  manual?: string;
  tutorial?: string;
  hotkey?: string;
}

let getExpertiseFunction: (() => Expertise) | undefined;

export function setTooltipModeProvider(fn: () => Expertise) {
  getExpertiseFunction = fn;
  setExpertiseProvider(fn);
}

export function useTooltipMode(): Expertise {
  if (!getExpertiseFunction) return Expertise.BEGINNER;
  return getExpertiseFunction();
}

function TooltipProvider({ delayDuration = 400, ...props }: React.ComponentProps<typeof TooltipPrimitive.Provider>) {
  return <TooltipPrimitive.Provider data-slot="tooltip-provider" delayDuration={delayDuration} {...props} />;
}

function Tooltip({ ...props }: React.ComponentProps<typeof TooltipPrimitive.Root>) {
  return (
    <TooltipProvider>
      <TooltipPrimitive.Root data-slot="tooltip" {...props} />
    </TooltipProvider>
  );
}

function TooltipTrigger({ className, asChild, ...props }: React.ComponentProps<typeof TooltipPrimitive.Trigger>) {
  return <TooltipPrimitive.Trigger data-slot="tooltip-trigger" asChild={asChild} className={cn(className)} {...props} />;
}

function TooltipContent({ className, sideOffset = 8, children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Content>) {
  return (
    <TooltipPrimitive.Portal>
      <TooltipPrimitive.Content
        data-slot="tooltip-content"
        sideOffset={sideOffset}
        className={cn(
          "bg-temporary border border-accent-foreground text-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 max-w-[300px] origin-(--radix-tooltip-content-transform-origin) p-single text-xs text-balance",
          className,
        )}
        {...props}
      >
        {children}
      </TooltipPrimitive.Content>
    </TooltipPrimitive.Portal>
  );
}

interface EnhancedTooltipContentProps {
  config: TooltipConfig;
}

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
            <kbd onClick={handleHotkeyClick} className="bg-panel border border-accent-foreground text-muted-foreground p-single text-2xs font-mono justify-self-end cursor-pointer hover:bg-hover-panel">
              {hotkey}
            </kbd>
          ) : (
            <span className="block" />
          )}
        </div>
      ) : null}
    </div>
  );
}

interface IdTooltipContentProps {
  id: string;
}

function IdTooltipContent({ id }: IdTooltipContentProps) {
  const { t } = useTranslation();
  const mode = useTooltipMode();

  if (mode === Expertise.EXPERT) return null;

  const label = useLabel(id);
  const manualLabel = useLabel("tooltip.manual");
  const tutorialLabel = useLabel("tooltip.tutorial");
  const value = t(id);
  console.log(`[DEBUG] [BREADCRUMB-RENDER] IdTooltipContent rendering for id="${id}"`, { label, labelType: typeof label, value, valueType: typeof value });
  const manualPath = typeof value === "object" && value?.manual ? value.manual : undefined;
  const tutorialPath = typeof value === "object" && value?.tutorial ? value.tutorial : undefined;

  let hotkey: string | undefined;
  if (typeof value === "object" && value?.hotkey) {
    hotkey = typeof value.hotkey === "string" ? value.hotkey : undefined;
  } else {
    const hotkeyValue = t(`${id}.hotkey`);
    if (typeof hotkeyValue === "string") {
      hotkey = hotkeyValue;
    } else if (hotkeyValue && typeof hotkeyValue === "object" && hotkeyValue.hotkey) {
      hotkey = typeof hotkeyValue.hotkey === "string" ? hotkeyValue.hotkey : undefined;
    }
  }

  const showManual = (mode === Expertise.BEGINNER || mode === Expertise.NORMAL) && manualPath;
  const showTutorial = mode === Expertise.BEGINNER && tutorialPath;

  const fullManualPath = manualPath ? `/docs/manual/${manualPath}` : undefined;
  const fullTutorialPath = tutorialPath ? `/docs/tutorials/${tutorialPath}` : undefined;

  const displayText = label;

  const handleHotkeyClick = () => {
    window.dispatchEvent(
      new CustomEvent("navigate-to-hotkey", {
        detail: { path: id },
      }),
    );
  };

  return (
    <div className="flex flex-col gap-single">
      <span>{displayText}</span>
      {(showManual && fullManualPath) || (showTutorial && fullTutorialPath) || hotkey ? (
        <div className="grid w-full grid-cols-3 items-center border-t border-accent-foreground pt-single gap-single">
          {showManual && fullManualPath ? (
            <Link to={fullManualPath} className="flex items-center gap-single cursor-pointer text-foreground transition-colors p-single hover:bg-hover-temporary">
              <BookIcon className="size-3" />
              <span>{manualLabel}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {showTutorial && fullTutorialPath ? (
            <Link to={fullTutorialPath} className="flex items-center gap-single cursor-pointer text-foreground transition-colors p-single hover:bg-hover-temporary">
              <TutorialIcon className="size-3" />
              <span className="block text-center">{tutorialLabel}</span>
            </Link>
          ) : (
            <span className="block" />
          )}
          {hotkey ? (
            <kbd onClick={handleHotkeyClick} className="bg-panel border border-accent-foreground text-muted-foreground p-single text-2xs font-mono justify-self-end cursor-pointer hover:bg-hover-panel">
              {hotkey}
            </kbd>
          ) : (
            <span className="block" />
          )}
        </div>
      ) : null}
    </div>
  );
}

// #endregion Tooltip

// #region Base Components

interface LabelProps {
  id: string;
  children: React.ReactNode;
  className?: string;
  labelElementId?: string;
}

function Label({ id, children, className, labelElementId }: LabelProps) {
  const label = useLabel(id);
  return (
    <div className={cn("group flex items-stretch min-w-0 w-full", className)}>
      <Tooltip>
        <TooltipTrigger asChild>
          <span id={labelElementId} className="inline-flex items-center px-tiny text-xs font-medium flex-shrink-0 min-w-[80px] text-left truncate cursor-pointer transition-colors hover:bg-hover-panel">
            {label}
          </span>
        </TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
      {children}
    </div>
  );
}

// #endregion Base Components

// #region Display Components

interface SemioTooltipProps {
  children: React.ReactElement;
  config: TooltipConfig;
}

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

interface IdSemioTooltipProps {
  children: React.ReactElement;
  id: string;
}

function IdSemioTooltip({ children, id }: IdSemioTooltipProps) {
  const mode = useTooltipMode();
  if (mode === Expertise.EXPERT) return children;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{children}</TooltipTrigger>
      <TooltipContent>
        <IdTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );
}

export { EnhancedTooltipContent, IdSemioTooltip, IdTooltipContent, SemioTooltip, Tooltip, TooltipContent, TooltipProvider, TooltipTrigger };

// #endregion Tooltip

// #region Aside

export interface AsideProps {
  kind?: "note" | "tip" | "caution" | "danger";
  title?: string;
  children: React.ReactNode;
}

const iconMap = {
  note: InfoIcon,
  tip: LightbulbIcon,
  caution: TriangleAlertIcon,
  danger: AlertCircleIcon,
};

const colorMap = {
  note: "border-info-border bg-info-bg text-info-foreground",
  tip: "border-success-border bg-success-bg text-success-foreground",
  caution: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

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

const Avatar = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Root>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Root ref={ref} data-slot="avatar" className={cn("relative flex size-small shrink-0 overflow-hidden rounded-full border", className)} {...props} />
));
Avatar.displayName = "Avatar";

const AvatarImage = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Image>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Image ref={ref} data-slot="avatar-image" className={cn("aspect-square size-full", className)} {...props} />
));
AvatarImage.displayName = "AvatarImage";

const AvatarFallback = React.forwardRef<React.ElementRef<typeof AvatarPrimitive.Fallback>, React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Fallback ref={ref} data-slot="avatar-fallback" className={cn("bg-muted flex size-full items-center justify-center rounded-full", className)} {...props} />
));
AvatarFallback.displayName = "AvatarFallback";

export interface DraggableAvatarProps {
  content: string;
  isSelected?: boolean;
  isHovered?: boolean;
  shouldFade?: boolean;
  title?: string;
  dragRef?: React.Ref<HTMLDivElement>;
  dragListeners?: any;
  dragAttributes?: any;
  onClick?: () => void;
  onDoubleClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  className?: string;
}

export const DraggableAvatar = React.forwardRef<HTMLDivElement, DraggableAvatarProps>(
  ({ content, isSelected, isHovered, shouldFade, title, dragRef, dragListeners, dragAttributes, onClick, onDoubleClick, onPointerEnter, onPointerLeave, className }, ref) => {
    return (
      <div data-slot="avatar" ref={dragRef || ref} {...dragListeners} {...dragAttributes} onClick={onClick} onDoubleClick={onDoubleClick} onPointerEnter={onPointerEnter} onPointerLeave={onPointerLeave} title={title} className={className}>
        <Avatar
          className={cn(
            "cursor-grab active:cursor-grabbing select-none border-[color:var(--border-color)]",
            isSelected && "ring-1 ring-inset ring-[color:var(--active-base)]",
            isHovered && !isSelected && "ring-1 ring-inset ring-[color:var(--hover-base)]",
          )}
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

export interface TableAvatarProps {
  icon?: string | React.ReactNode;
  name?: string;
  className?: string;
}

export const TableAvatar: React.FC<TableAvatarProps> = ({ icon, name, className }) => {
  const nameStr = typeof name === "string" ? name : String(name ?? "");
  const normalizedName = nameStr.trim();
  const initials = normalizedName
    ? normalizedName
        .split(" ")
        .slice(0, 2)
        .map((word) => word.charAt(0))
        .join("")
        .toUpperCase()
        .substring(0, 2)
    : "";
  const isImageIcon = typeof icon === "string";
  const isReactIcon = icon && !isImageIcon;
  return (
    <Avatar className={cn("shrink-0", className)}>
      {isImageIcon ? <AvatarImage src={icon} alt={normalizedName} /> : null}
      <AvatarFallback className="text-xs">{isReactIcon ? icon : initials}</AvatarFallback>
    </Avatar>
  );
};
TableAvatar.displayName = "TableAvatar";

export { Avatar, AvatarFallback, AvatarImage };

// #endregion Avatar

// #region Card

export interface CardProps {
  title: string;
  icon?: string | LucideIcon;
  children: React.ReactNode;
  className?: string;
}

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

export interface CardGridProps {
  stagger?: boolean;
  children: React.ReactNode;
  className?: string;
}

export const CardGrid: React.FC<CardGridProps> = ({ stagger = false, children, className = "" }) => {
  return <div className={`grid grid-cols-1 md:grid-cols-2 gap-medium my-medium ${className}`}>{children}</div>;
};

// #endregion Card

// #region DiagramNode

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

export const PlaceholderDiagramNode: React.FC<{ id?: string; onClick?: () => void }> = ({ id = "diagram.placeholder", onClick }) => {
  return <DiagramNode content={useLabel(id)} isPlaceholder showTopHandle onClick={onClick} className="hover:border-[color:var(--hover-base)] hover:bg-[color:var(--hover-panel)]" />;
};

// #endregion DiagramNode

// #region HoverCard

function HoverCard({ ...props }: React.ComponentProps<typeof HoverCardPrimitive.Root>) {
  return <HoverCardPrimitive.Root data-slot="hover-card" {...props} />;
}

function HoverCardTrigger({ className, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Trigger>) {
  return <HoverCardPrimitive.Trigger data-slot="hover-card-trigger" className={cn(className)} {...props} />;
}

function HoverCardContent({ className, align = "center", sideOffset = 4, ...props }: React.ComponentProps<typeof HoverCardPrimitive.Content>) {
  return (
    <HoverCardPrimitive.Portal data-slot="hover-card-portal">
      <HoverCardPrimitive.Content
        data-slot="hover-card-content"
        align={align}
        sideOffset={sideOffset}
        className={cn(
          "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-64 origin-(--radix-hover-card-content-transform-origin) border p-single outline-hidden",
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

interface CursorProps {
  color: string;
  x?: number;
  y?: number;
}

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

export interface SectionProps {
  id?: string;
  title?: string;
  children: React.ReactNode;
  className?: string;
}

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

export interface StepsProps {
  children: React.ReactNode;
  className?: string;
}

export const Steps: React.FC<StepsProps> = ({ children, className = "" }) => {
  return <div className={`steps-container space-y-medium my-medium ${className}`}>{children}</div>;
};

// #endregion Steps

// #endregion Display Components

// #region Input Components

// #region ActionGroup

const actionGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center shrink-0 transition-all cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg]:!size-[tiny] [&_svg]:!max-w-tiny [&_svg]:!max-h-tiny [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive overflow-hidden aspect-square p-single",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
        panel: "hover:bg-hover-panel",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      level: "base",
    },
  },
);

const ActionGroupContext = React.createContext<{ level: Level }>({
  level: "base",
});

interface ActionGroupProps extends Omit<React.ComponentProps<"div">, "children" | "id"> {
  id: string;
  children: React.ReactNode;
  level?: Level;
}

function ActionGroup({ className, level: propLevel, id, children, ...props }: ActionGroupProps) {
  const level = useElementLevel(propLevel);
  return (
    <div data-slot="action-group" data-level={level} className={cn("group/action-group flex h-small items-center border divide-x overflow-hidden", className)} {...props}>
      <ActionGroupContext.Provider value={{ level }}>{children}</ActionGroupContext.Provider>
    </div>
  );
}

function ActionGroupItem({
  className,
  children,
  id,
  as: Component = "button",
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  as?: "button" | "div";
}) {
  const context = React.useContext(ActionGroupContext);
  const level = context.level ?? "base";

  const actionGroupItemElement = (
    <Component
      data-slot="action-group-item"
      type={Component === "button" ? "button" : undefined}
      role={Component === "div" ? "button" : undefined}
      tabIndex={Component === "div" ? 0 : undefined}
      data-level={context.level || level}
      className={cn(
        actionGroupItemVariants({
          level: context.level || level,
        }),
        "min-w-0 shrink-0 focus:z-10 focus-visible:z-10",
        !id && "flex-1",
        className,
      )}
      {...(props as any)}
    >
      {children}
    </Component>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild className="flex-1 min-w-0">
          {actionGroupItemElement}
        </TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return actionGroupItemElement;
}

interface ActionDropdownOption {
  value: string;
  icon: React.ReactNode;
  label?: string;
}

interface ActionDropdownProps extends Omit<React.ComponentProps<"button">, "children" | "id"> {
  id: string;
  options: ActionDropdownOption[];
  value: string;
  onValueChange?: (value: string) => void;
  startTransaction?: () => void;
  finalizeTransaction?: () => void;
  level?: Level;
}

function ActionDropdown({ className, level: propLevel, id, options, value, onValueChange, startTransaction, finalizeTransaction, ...props }: ActionDropdownProps) {
  const [open, setOpen] = React.useState(false);
  const level = useElementLevel(propLevel);

  const selectedOption = options.find((option) => option.value === value);

  const handleOpenChange = (isOpen: boolean) => {
    if (isOpen && startTransaction) startTransaction();
    setOpen(isOpen);
    if (!isOpen && finalizeTransaction) finalizeTransaction();
  };

  const handleSelect = (optionValue: string) => {
    if (onValueChange) onValueChange(optionValue);
    setOpen(false);
  };

  const buttonElement = (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>
        <ActionGroup id={id} level={level} className={className}>
          <ActionGroupItem {...props}>{selectedOption?.icon}</ActionGroupItem>
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

interface ActionProps extends Omit<React.ComponentProps<"button">, "children"> {
  as?: "button" | "div";
  loading?: boolean;
  icon: React.ReactNode;
  id?: string;
  level?: Level;
}

function Action({ className, level: propLevel, id, icon, as = "button", ...props }: ActionProps) {
  const level = useElementLevel(propLevel);
  return (
    <ActionGroup id={id || "action"} level={level} className={className}>
      <ActionGroupItem as={as} {...props}>
        {icon}
      </ActionGroupItem>
    </ActionGroup>
  );
}

export { Action, ActionDropdown, ActionGroup, ActionGroupItem, actionGroupItemVariants };
export type { ActionDropdownOption, ActionDropdownProps, ActionProps };

// #endregion ActionGroupup

const buttonGroupItemVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap h-medium aspect-square p-single overflow-hidden",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
        panel: "hover:bg-hover-panel",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      level: "base",
    },
  },
);

const ButtonGroupContext = React.createContext<{ level: Level }>({
  level: "base",
});

interface ButtonGroupProps extends Omit<React.ComponentProps<"div">, "id"> {
  id: string;
  level?: Level;
  showLabel?: boolean;
  children: React.ReactNode;
}

function ButtonGroup({ className, level: propLevel, id, showLabel, children, ...props }: ButtonGroupProps) {
  const level = useElementLevel(propLevel);
  const buttonGroupElement = (
    <div data-slot="button-group" data-level={level} className={cn("group/button-group flex w-fit items-center border divide-x overflow-hidden h-medium", className)} {...props}>
      <ButtonGroupContext.Provider value={{ level }}>{children as React.ReactNode}</ButtonGroupContext.Provider>
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

function ButtonGroupItem({
  className,
  children,
  id,
  icon,
  asChild = false,
  ...props
}: React.ComponentProps<"button"> & {
  id?: string;
  icon?: React.ReactNode;
  asChild?: boolean;
}) {
  const context = React.useContext(ButtonGroupContext);
  const level = context.level ?? "base";
  const Comp = asChild ? Slot : "button";

  return (
    <Comp
      data-slot="button-group-item"
      data-level={context.level || level}
      className={cn(
        buttonGroupItemVariants({
          level: context.level || level,
        }),
        "min-w-0 flex-1 shrink-0 focus:z-10 focus-visible:z-10",
        className,
      )}
      {...(props as any)}
    >
      {icon || children}
    </Comp>
  );
}

type ButtonProps = React.ComponentProps<"button"> &
  Omit<VariantProps<typeof buttonGroupItemVariants>, "level"> & {
    level?: Level;
    asChild?: boolean;
    id?: string;
    icon?: React.ReactNode;
    children?: React.ReactNode;
  };

interface ButtonCycleItem<T extends string> {
  value: T;
  label: React.ReactNode;
  id?: string;
}

interface ButtonCycleProps<T extends string> extends Omit<React.ComponentProps<"button">, "children" | "id">, ElementProps {
  value?: T;
  onValueChange?: (value: T) => void;
  items: ButtonCycleItem<T>[];
  showLabel?: boolean;
}

function Button({ className, level: propLevel, asChild = false, id, icon, children, ...props }: ButtonProps) {
  const level = useElementLevel(propLevel);
  return (
    <ButtonGroup id={id || "button"} level={level} className={className}>
      <ButtonGroupItem asChild={asChild} {...props}>
        {icon || children}
      </ButtonGroupItem>
    </ButtonGroup>
  );
}

function ButtonCycle<T extends string = string>({ className, level: propLevel, id, showLabel, value, onValueChange, items, ...props }: ButtonCycleProps<T>) {
  const level = useElementLevel(propLevel);
  const currentIndex = items.findIndex((item) => item.value === value);
  const currentItem = currentIndex >= 0 ? items[currentIndex] : items[0];

  const handleCycle = () => {
    const nextIndex = (currentIndex + 1) % items.length;
    if (onValueChange) onValueChange(items[nextIndex].value);
  };

  return (
    <ButtonGroup id={id || "button-cycle"} showLabel={showLabel} level={level} className={className}>
      <ButtonGroupItem onClick={handleCycle} icon={currentItem.label} {...props} />
    </ButtonGroup>
  );
}

export { Button, ButtonCycle, ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
export type { ButtonCycleProps, ButtonProps };

// #endregion ButtonGroup

// #region Combobox

interface ComboboxOption {
  value: string;
  label: string;
}

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

export const Combobox: React.FC<ComboboxProps> = ({ options, value = "", placeholder = "Select option...", placeholderId, emptyMessage = "No options found.", onValueChange, className, allowClear = false, showLabel, id, transaction }) => {
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
        <Button role="combobox" aria-expanded={open} className="w-full justify-between flex-1 min-w-0">
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
                  {option.label}
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

interface InputProps extends Omit<React.ComponentProps<"input">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onLazyChange?: (value: string) => void;
  interactionId?: string;
  placeholderId?: string;
  showLabel?: boolean;
}

function Input({ className, type, lazy, value: externalValue, onChange, onLazyChange, interactionId, id, placeholderId, placeholder, showLabel, transaction, ...props }: InputProps) {
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

function Select({ id, showLabel, children, value, defaultValue, onOpenChange, transaction, ...props }: React.ComponentProps<typeof SelectPrimitive.Root> & ElementProps & { showLabel?: boolean }) {
  const fallbackValue = React.useMemo(() => {
    const findValue = (nodes: React.ReactNode[]): string | undefined => {
      for (const node of nodes) {
        if (!React.isValidElement(node)) {
          continue;
        }
        const nodeProps = node.props as { "data-slot"?: string; value?: string; children?: React.ReactNode };
        if ((node.type === SelectPrimitive.Item || node.type === SelectItem || nodeProps["data-slot"] === "select-item") && nodeProps.value !== undefined) {
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
      data-slot="select"
      {...(value !== null && value !== undefined ? { value } : defaultValue !== null && defaultValue !== undefined ? { defaultValue } : fallbackValue !== undefined ? { defaultValue: fallbackValue } : {})}
      onOpenChange={handleOpenChange}
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

function SelectGroup({ ...props }: React.ComponentProps<typeof SelectPrimitive.Group>) {
  return <SelectPrimitive.Group data-slot="select-group" {...props} />;
}

function SelectValue({ ...props }: React.ComponentProps<typeof SelectPrimitive.Value>) {
  return <SelectPrimitive.Value data-slot="select-value" {...props} />;
}

function SelectTrigger({
  className,
  size = "default",
  level: propLevel,
  children,
  id,
  ...props
}: React.ComponentProps<typeof SelectPrimitive.Trigger> & {
  size?: "sm" | "default";
  level?: Level;
  id?: string;
}) {
  const level = useElementLevel(propLevel);
  const hoverClass = level === "panel" ? "hover:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary" : "hover:bg-hover-base";

  return (
    <SelectPrimitive.Trigger
      data-slot="select-trigger"
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

function SelectContent({ className, children, position = "popper", ...props }: React.ComponentProps<typeof SelectPrimitive.Content>) {
  return (
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        data-slot="select-content"
        className={cn(
          "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 relative z-50 max-h-(--radix-select-content-available-height) min-w-32 origin-(--radix-select-content-transform-origin) overflow-x-hidden overflow-y-auto border",
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

function SelectLabel({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Label>) {
  return <SelectPrimitive.Label data-slot="select-label" className={cn("text-muted-foreground p-single text-xs", className)} {...props} />;
}

function SelectItem({ className, children, ...props }: React.ComponentProps<typeof SelectPrimitive.Item>) {
  return (
    <SelectPrimitive.Item
      data-slot="select-item"
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

function SelectSeparator({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.Separator>) {
  return <SelectPrimitive.Separator data-slot="select-separator" className={cn("bg-border pointer-events-none -mx-single my-single h-px", className)} {...props} />;
}

function SelectScrollUpButton({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.ScrollUpButton>) {
  return (
    <SelectPrimitive.ScrollUpButton data-slot="select-scroll-up-button" className={cn("flex cursor-default items-center justify-center py-single", className)} {...props}>
      <ChevronUpIcon className="size-tiny" />
    </SelectPrimitive.ScrollUpButton>
  );
}

function SelectScrollDownButton({ className, ...props }: React.ComponentProps<typeof SelectPrimitive.ScrollDownButton>) {
  return (
    <SelectPrimitive.ScrollDownButton data-slot="select-scroll-down-button" className={cn("flex cursor-default items-center justify-center py-single", className)} {...props}>
      <ChevronDownIconAlt className="size-tiny" />
    </SelectPrimitive.ScrollDownButton>
  );
}

const ChevronUpIcon = ChevronDownIconAlt;

export { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton, SelectSeparator, SelectTrigger, SelectValue };

// #endregion Select

// #region Slider

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
  transaction,
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
        <IdTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );

  const sliderContent = (
    <div style={{ opacity: shouldFade ? 0 : 1, transition: "opacity 150ms" }} className="flex-1 min-w-0">
      <div className="flex items-center gap-single h-large">
        <div className="flex-1 min-w-0">{wrappedSlider}</div>
        {isEditing ? (
          <Input type="number" value={editValue} onChange={(e) => setEditValue(e.target.value)} onKeyDown={handleEditKeyDown} onBlur={handleEditBlur} className="w-20 text-sm" min={min} max={max} autoFocus id={id} />
        ) : (
          <span className="text-sm w-20 text-right px-single select-none" role="button" onDoubleClick={handleValueClick} title="Double-click to edit">
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
}

export const Stepper: React.FC<StepperProps> = ({ value, defaultValue = 0, min, max, step = 1, onChange, onPointerDown, onPointerUp, onPointerCancel, interactionId, id, transaction }) => {
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
            onChange?.(newValue);
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
    <div className="flex h-large flex-1 min-w-0 items-stretch border border-border transition-[border-color] focus-within:border-accent">
      <button
        type="button"
        onMouseDown={handleMouseDown(-step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(-step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepDown}
        className="flex h-full w-large cursor-pointer items-center justify-center border-r border-border hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted"
      >
        <RemoveIcon className="size-tiny" />
      </button>
      <Input
        type="number"
        value={internalValue.toString()}
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
        className="border-0 text-center focus-visible:border-0"
        step={step}
        min={min}
        max={max}
        aria-labelledby={labelElementId}
        id={id}
      />
      <button
        type="button"
        onMouseDown={handleMouseDown(step)}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleMouseDown(step)}
        onTouchEnd={handleMouseUp}
        disabled={!canStepUp}
        className="flex h-full w-large cursor-pointer items-center justify-center border-l border-border hover:bg-muted disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus-visible:bg-muted"
      >
        <AddIcon className="size-tiny" />
      </button>
    </div>
  );

  return <Label id={id}>{stepperElement}</Label>;
};

// #endregion Stepper

// #region Textarea

interface TextareaProps extends Omit<React.ComponentProps<"textarea">, "value" | "onChange" | "id">, ElementProps {
  lazy?: boolean;
  value?: string | number | readonly string[];
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
  onLazyChange?: (value: string) => void;
  showLabel?: boolean;
  placeholderId?: string;
}

function Textarea({ className, lazy, value: externalValue, onChange, onLazyChange, id, showLabel, placeholderId, placeholder, transaction, ...props }: TextareaProps) {
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

const toggleVariants = cva(
  "text-foreground inline-flex items-center justify-center gap-single text-sm font-medium cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap data-[state=on]:bg-active-base data-[state=on]:text-active-foreground data-[state=on]:hover:bg-active-base/90 data-[state=on]:hover:text-active-foreground h-medium aspect-square p-single leading-none overflow-hidden",
  {
    variants: {
      level: {
        base: "hover:bg-hover-base",
        panel: "hover:bg-hover-panel",
        temporary: "hover:bg-hover-temporary",
      },
    },
    defaultVariants: {
      level: "base",
    },
  },
);

export interface ToggleItem<T extends string> {
  value: T;
  label: React.ReactNode;
  id?: string;
}

interface ToggleStandardProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind?: "default";
  i18nPressed?: string;
  showLabel?: boolean;
  icon: React.ReactNode;
}

interface ToggleWithActionProps extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind: "withAction";
  actionIcon: React.ReactNode;
  onActionClick: () => void;
  showLabel?: boolean;
  actionId?: string;
  icon: React.ReactNode;
}

interface ToggleDropdownProps<T extends string> extends Omit<React.ComponentProps<typeof TogglePrimitive.Root>, "type" | "id">, ElementProps {
  kind: "dropdown";
  value?: T;
  defaultValue?: T;
  onValueChange?: (value: T) => void;
  items: ToggleItem<T>[];
  showLabel?: boolean;
  placeholder?: string;
  dropdownId?: string;
}

type ToggleProps<T extends string = string> = ToggleStandardProps | ToggleWithActionProps | ToggleDropdownProps<T>;

export type { ToggleProps };

// #endregion Toggle

// #region ToggleGroup

const ToggleGroupContext = React.createContext<{ level: Level }>({
  level: "base",
});

type ToggleGroupItemProps = Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Item>, "children"> & {
  id?: string;
  icon: React.ReactNode;
  action?: React.ReactNode;
  value: string;
};

interface ToggleGroupProps extends Omit<React.ComponentProps<typeof ToggleGroupPrimitive.Root>, "children" | "type" | "id"> {
  id?: string;
  showLabel?: boolean;
  level?: Level;
  kind?: "single" | "multiple";
  items: ToggleGroupItemProps[];
}

function ToggleGroup({ className, id, showLabel, level: propLevel, items, kind = "single", ...restProps }: ToggleGroupProps) {
  const level = useElementLevel(propLevel);
  const toggleGroupElement = (
    <ToggleGroupPrimitive.Root data-slot="toggle-group" type={kind} className={cn("group/toggle-group flex w-fit items-center border overflow-hidden h-medium divide-x", className)} {...(restProps as any)}>
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

function ToggleGroupItem({ className, id, icon, action, ...props }: ToggleGroupItemProps) {
  const context = React.useContext(ToggleGroupContext);
  const level = context.level ?? "base";

  const toggleGroupItemElement = (
    <ToggleGroupPrimitive.Item
      data-slot="toggle-group-item"
      className={cn(
        toggleVariants({
          level,
        }),
        "min-w-0 flex-1 shrink-0 focus:z-10 focus-visible:z-10 data-[state=on]:bg-active-base data-[state=on]:hover:bg-active-base/90",
        action && "flex items-center gap-0 p-single w-mega aspect-auto",
        className,
      )}
      {...props}
    >
      {icon as React.ReactNode}
      {action && (
        <div
          className={cn("flex items-center justify-center w-small h-small bg-base", level === "panel" && "bg-panel", level === "temporary" && "bg-temporary")}
          onClick={(e) => e.stopPropagation()}
          onMouseDown={(e) => e.stopPropagation()}
          onPointerDown={(e) => e.stopPropagation()}
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
          <span>{toggleGroupItemElement}</span>
        </TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return toggleGroupItemElement;
}

// Helper to add size-small class to icon elements
const addIconSize = (element: React.ReactNode): React.ReactNode => {
  if (React.isValidElement(element)) {
    const existingClassName = (element.props as any).className || "";
    // Only add size-small if no size class is already present
    if (!existingClassName.includes("size-")) {
      return React.cloneElement(element, {
        ...element.props,
        className: cn(existingClassName, "size-small"),
      } as any);
    }
  }
  return element;
};

function Toggle<T extends string = string>(props: ToggleProps<T>) {
  if ("kind" in props && props.kind === "withAction") {
    const { actionIcon, onActionClick, icon, pressed, defaultPressed, onPressedChange, id, showLabel, level, className, actionId } = props as ToggleWithActionProps;
    const value = pressed !== undefined ? (pressed ? "on" : "") : undefined;
    return (
      <ToggleGroup
        id={id}
        showLabel={showLabel}
        level={level}
        kind="single"
        value={value}
        defaultValue={pressed === undefined && defaultPressed ? "on" : undefined}
        onValueChange={(val: string) => onPressedChange?.(val === "on")}
        className={className}
        items={[
          {
            value: "on",
            icon: addIconSize(icon),
            action: <Action as="div" id={actionId} icon={addIconSize(actionIcon)} onClick={onActionClick} level={level} />,
          },
        ]}
      />
    );
  }

  if ("kind" in props && props.kind === "dropdown" && "items" in props) {
    const dropdownProps = props as ToggleDropdownProps<T>;
    const { items, value: controlledValue, defaultValue, onValueChange, pressed, defaultPressed, onPressedChange, id, showLabel, level, className, dropdownId } = dropdownProps;
    const [internalValue, setInternalValue] = React.useState<T | undefined>(defaultValue);
    const [open, setOpen] = React.useState(false);

    const isControlled = controlledValue !== undefined;
    const value = isControlled ? controlledValue : internalValue;
    const selectedItem = items.find((item) => item.value === value) || items[0];

    const handleSelect = (itemValue: string) => {
      if (!isControlled) {
        setInternalValue(itemValue as T);
      }
      if (onValueChange) onValueChange(itemValue as T);
      setOpen(false);
    };

    const handleToggleGroupValueChange = (toggleValue: string) => {
      // If the value is the selectedItem.value, it means the toggle is being pressed "on"
      // If the value is empty/undefined, it means the toggle is being pressed "off"
      const isPressed = toggleValue === selectedItem.value;
      if (onPressedChange) {
        onPressedChange(isPressed);
      }
    };

    const availableItems = items.filter((item) => item.value !== value);

    const dropdownAction = (
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Action as="div" id={dropdownId} icon={<ChevronDownIcon className="size-small" />} level={level} />
        </PopoverTrigger>
        <PopoverContent className="w-auto p-single min-w-[120px]" align="start">
          <div className="flex flex-col">
            {availableItems.map((item) => (
              <button key={item.value} onClick={() => handleSelect(item.value)} className={cn("flex items-center p-single text-xs cursor-selectable transition-colors", "hover:bg-hover-temporary outline-none focus-visible:bg-hover-temporary")}>
                <span className="flex-1 text-left">{addIconSize(item.label)}</span>
              </button>
            ))}
          </div>
        </PopoverContent>
      </Popover>
    );

    // Determine if the toggle pressed state is controlled
    const isPressedControlled = pressed !== undefined;
    const toggleGroupProps: any = {
      id,
      showLabel,
      level,
      kind: "single" as const,
      onValueChange: handleToggleGroupValueChange,
      className,
      items: [
        {
          value: selectedItem.value,
          icon: addIconSize(selectedItem.label),
          action: dropdownAction,
        },
      ],
    };

    // Only pass value OR defaultValue, never both
    if (isPressedControlled) {
      toggleGroupProps.value = pressed ? selectedItem.value : "";
    } else if (defaultPressed !== undefined) {
      toggleGroupProps.defaultValue = defaultPressed ? selectedItem.value : undefined;
    }

    return <ToggleGroup {...toggleGroupProps} />;
  }

  const { id, showLabel, level, className, icon, pressed, defaultPressed, onPressedChange } = props as ToggleStandardProps;
  const value = pressed !== undefined ? (pressed ? "on" : "") : undefined;
  return (
    <ToggleGroup
      id={id}
      showLabel={showLabel}
      level={level}
      kind="single"
      value={value}
      defaultValue={pressed === undefined && defaultPressed ? "on" : undefined}
      onValueChange={(val: string) => onPressedChange?.(val === "on")}
      className={className}
      items={[
        {
          value: "on",
          icon: addIconSize(icon),
        },
      ]}
    />
  );
}

export { Toggle, ToggleGroup, ToggleGroupItem, toggleVariants };

// #endregion ToggleGroup

// #endregion Input Components

// #region Aggregation Components

// #region Accordion

function Accordion({ ...props }: React.ComponentProps<typeof AccordionPrimitive.Root>) {
  return <AccordionPrimitive.Root data-slot="accordion" {...props} />;
}

function AccordionItem({ className, ...props }: React.ComponentProps<typeof AccordionPrimitive.Item>) {
  return <AccordionPrimitive.Item data-slot="accordion-item" className={cn("border-b last:border-b-0", className)} {...props} />;
}

function AccordionTrigger({ className, children, ...props }: React.ComponentProps<typeof AccordionPrimitive.Trigger>) {
  return (
    <AccordionPrimitive.Header className="flex">
      <AccordionPrimitive.Trigger
        data-slot="accordion-trigger"
        className={cn(
          "focus-visible:border-ring focus-visible:ring-ring/50 flex flex-1 items-start justify-between gap-medium py-small text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180",
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

function AccordionContent({ className, children, ...props }: React.ComponentProps<typeof AccordionPrimitive.Content>) {
  return (
    <AccordionPrimitive.Content data-slot="accordion-content" className="data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm" {...props}>
      <div className={cn("pt-0 pb-4", className)}>{children}</div>
    </AccordionPrimitive.Content>
  );
}

export { Accordion, AccordionContent, AccordionItem, AccordionTrigger };

// #endregion Accordion

// #region Collapsible

function Collapsible({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.Root>) {
  return <CollapsiblePrimitive.Root data-slot="collapsible" {...props} />;
}

function CollapsibleTrigger({ className, ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleTrigger>) {
  return <CollapsiblePrimitive.CollapsibleTrigger data-slot="collapsible-trigger" className={cn(className)} {...props} />;
}

function CollapsibleContent({ ...props }: React.ComponentProps<typeof CollapsiblePrimitive.CollapsibleContent>) {
  return <CollapsiblePrimitive.CollapsibleContent data-slot="collapsible-content" {...props} />;
}

export { Collapsible, CollapsibleContent, CollapsibleTrigger };

// #endregion Collapsible

// #region Dialog

function Dialog({ ...props }: React.ComponentProps<typeof DialogPrimitive.Root>) {
  return <DialogPrimitive.Root data-slot="dialog" {...props} />;
}

function DialogTrigger({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Trigger>) {
  return <DialogPrimitive.Trigger data-slot="dialog-trigger" className={cn(className)} {...props} />;
}

function DialogPortal({ ...props }: React.ComponentProps<typeof DialogPrimitive.Portal>) {
  return <DialogPrimitive.Portal data-slot="dialog-portal" {...props} />;
}

function DialogClose({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Close>) {
  return <DialogPrimitive.Close data-slot="dialog-close" className={cn(className)} {...props} />;
}

function DialogOverlay({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Overlay>) {
  return (
    <DialogPrimitive.Overlay data-slot="dialog-overlay" className={cn("data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/50", className)} {...props} />
  );
}

function DialogContent({
  className,
  children,
  showCloseButton = true,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Content> & {
  showCloseButton?: boolean;
}) {
  return (
    <DialogPortal data-slot="dialog-portal">
      <DialogOverlay />
      <DialogPrimitive.Content
        data-slot="dialog-content"
        className={cn(
          "bg-temporary data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2*var(--spacing)*var(--medium))] translate-x-[-50%] translate-y-[-50%] gap-medium border p-medium duration-200 sm:max-w-lg",
          className,
        )}
        {...props}
      >
        {children as React.ReactNode}
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

function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-header" className={cn("flex flex-col gap-single text-center sm:text-left", className)} {...props} />;
}

function DialogFooter({ className, ...props }: React.ComponentProps<"div">) {
  return <div data-slot="dialog-footer" className={cn("flex flex-col-reverse gap-single sm:flex-row sm:justify-end", className)} {...props} />;
}

function DialogTitle({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Title>) {
  return <DialogPrimitive.Title data-slot="dialog-title" className={cn("text-lg leading-none font-semibold", className)} {...props} />;
}

function DialogDescription({ className, ...props }: React.ComponentProps<typeof DialogPrimitive.Description>) {
  return <DialogPrimitive.Description data-slot="dialog-description" className={cn("text-muted-foreground text-sm", className)} {...props} />;
}

export { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger };

// #endregion Dialog

// #region Resizable

function ResizablePanelGroup({ className, ...props }: React.ComponentProps<typeof ResizablePrimitive.PanelGroup>) {
  return <ResizablePrimitive.PanelGroup data-slot="resizable-panel-group" className={cn("flex h-full w-full data-[panel-group-direction=vertical]:flex-col", className)} {...props} />;
}

function ResizablePanel({ ...props }: React.ComponentProps<typeof ResizablePrimitive.Panel>) {
  return <ResizablePrimitive.Panel data-slot="resizable-panel" {...props} />;
}

function ResizableHandle({ className, onMouseDown: externalOnMouseDown, onMouseEnter: externalOnMouseEnter, onMouseLeave: externalOnMouseLeave, ...props }: React.ComponentProps<typeof ResizablePrimitive.PanelResizeHandle>) {
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
    <ResizablePrimitive.PanelResizeHandle
      data-slot="resizable-handle"
      className={cn(
        "relative flex w-px items-center justify-center",
        "border-r",
        isDragging || isHovered ? "bg-accent border-accent" : "hover:border-accent",
        "before:absolute before:inset-y-0 before:-left-2 before:w-tiny before:cursor-ew-resize",
        "focus-visible:ring-ring focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none",
        "after:absolute after:inset-y-0 after:left-1/2 after:w-single after:-translate-x-1/2",
        "data-[panel-group-direction=vertical]:h-px data-[panel-group-direction=vertical]:w-full",
        "data-[panel-group-direction=vertical]:border-r-0 data-[panel-group-direction=vertical]:border-t",
        isDragging || isHovered ? "data-[panel-group-direction=vertical]:bg-accent data-[panel-group-direction=vertical]:border-accent" : "data-[panel-group-direction=vertical]:hover:border-accent",
        "data-[panel-group-direction=vertical]:after:left-0 data-[panel-group-direction=vertical]:after:h-single data-[panel-group-direction=vertical]:after:w-full data-[panel-group-direction=vertical]:after:-translate-y-1/2 data-[panel-group-direction=vertical]:after:translate-x-0",
        "data-[panel-group-direction=vertical]:before:inset-x-0 data-[panel-group-direction=vertical]:before:-top-2 data-[panel-group-direction=vertical]:before:h-4 data-[panel-group-direction=vertical]:before:w-full data-[panel-group-direction=vertical]:before:cursor-ns-resize",
        className,
      )}
      onMouseDown={handleMouseDown as any}
      onMouseEnter={handleMouseEnter as any}
      onMouseLeave={handleMouseLeave as any}
      {...props}
    />
  );
}

export { ResizableHandle, ResizablePanel, ResizablePanelGroup };

// #endregion Resizable

// #region Scrollable

const Scrollable = React.forwardRef<HTMLDivElement, React.ComponentProps<typeof ScrollAreaPrimitive.Root> & { orientation?: "vertical" | "horizontal" | "both" }>(({ className, children, orientation = "vertical", ...props }, ref) => {
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

function ScrollBar({ className, orientation = "vertical", ...props }: React.ComponentProps<typeof ScrollAreaPrimitive.ScrollAreaScrollbar>) {
  return (
    <ScrollAreaPrimitive.ScrollAreaScrollbar
      data-slot="scroll-area-scrollbar"
      orientation={orientation}
      className={cn("flex touch-none p-single transition-colors select-none", orientation === "vertical" && "h-full w-2.5 border-l border-l-transparent", orientation === "horizontal" && "h-2.5 flex-col border-t border-t-transparent", className)}
      {...props}
    >
      <ScrollAreaPrimitive.ScrollAreaThumb data-slot="scroll-area-thumb" className="bg-border relative flex-1" />
    </ScrollAreaPrimitive.ScrollAreaScrollbar>
  );
}

export { Scrollable, ScrollBar };

// #endregion Scrollable

// #region Strip

export interface StripProps extends ElementProps {
  direction?: "horizontal" | "vertical";
  items: React.ReactNode[];
  className?: string;
}

function Strip({ direction = "horizontal", items, className, level: propLevel, id }: StripProps) {
  const level = useElementLevel(propLevel);

  return (
    <Scrollable orientation="horizontal" className={cn("border-b", direction === "horizontal" ? "h-large" : "w-large", className)}>
      <div className={cn("p-single flex gap-single", direction === "horizontal" ? "flex-row h-full items-center w-fit" : "flex-col w-full")}>
        {items.map((item, index) => (
          <div key={index} className={cn(direction === "horizontal" ? "h-medium" : "w-medium", "shrink-0")}>
            {item}
          </div>
        ))}
      </div>
    </Scrollable>
  );
}

export { Strip };

// #endregion Strip

// #region Tabs

function Tabs({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Root>) {
  return <TabsPrimitive.Root data-slot="tabs" className={cn("flex flex-col gap-single", className)} {...props} />;
}

function TabsList({ className, level: propLevel, ...props }: React.ComponentProps<typeof TabsPrimitive.List> & { level?: Level }) {
  const level = useElementLevel(propLevel);
  const bgClass = level === "panel" ? "bg-panel" : level === "temporary" ? "bg-temporary" : "bg-background";
  return <TabsPrimitive.List data-slot="tabs-list" className={cn("text-muted-foreground inline-flex h-large w-fit items-center justify-center p-single", bgClass, className)} {...props} />;
}

function TabsTrigger({ className, level: propLevel, ...props }: React.ComponentProps<typeof TabsPrimitive.Trigger> & { level?: Level }) {
  const level = useElementLevel(propLevel);
  const activeHoverClass = level === "panel" ? "data-[state=active]:bg-hover-panel" : level === "temporary" ? "data-[state=active]:bg-hover-temporary" : "data-[state=active]:bg-hover-base";
  const hoverClass = level === "panel" ? "hover:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary" : "hover:bg-hover-base";
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

function TabsContent({ className, ...props }: React.ComponentProps<typeof TabsPrimitive.Content>) {
  return <TabsPrimitive.Content data-slot="tabs-content" className={cn("flex-1 outline-none", className)} {...props} />;
}

export { Tabs, TabsContent, TabsList, TabsTrigger };

// #endregion Tabs

// #region Tree

interface TreeStateContextValue {
  openStates: Record<string, boolean>;
  setOpenState: (id: string, open: boolean) => void;
  getOpenState: (id: string, defaultOpen: boolean) => boolean;
}

const TreeStateContext = React.createContext<TreeStateContextValue | null>(null);

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

export const useTreeState = () => {
  const context = React.useContext(TreeStateContext);
  if (!context) throw new Error("useTreeState must be used within TreeStateProvider");
  return context;
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

const TreeContext = React.createContext<{ level: number; isLastAtLevel: boolean[]; showLines: boolean }>({ level: 0, isLastAtLevel: [], showLines: true });

const IndentationLines: React.FC<{ level: number; isLastAtLevel: boolean[]; showLines: boolean }> = ({ level, isLastAtLevel, showLines }) => {
  if (!showLines || level === 0) return null;

  return (
    <div className="absolute left-0 top-0 bottom-0 pointer-events-none">
      {Array.from({ length: level }, (_, i) => (
        <div key={i} className="absolute top-0 bottom-0" style={{ left: `${i * 0.75 + 0.375}rem` }}>
          {!isLastAtLevel[i] && <div className="w-px h-full bg-muted-foreground/40" />}
        </div>
      ))}
    </div>
  );
};

export const TreeContent: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { level, isLastAtLevel, showLines } = React.useContext(TreeContext);
  return (
    <div className="relative py-single" style={{ paddingLeft: `${level * 0.75}rem` }}>
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      {children}
    </div>
  );
};

export interface TreeSectionAction {
  icon: React.ReactNode;
  onClick: () => void;
  title?: string;
  id?: string;
}

interface TreeSectionProps {
  label?: string;
  id?: string;
  icon?: React.ReactNode;
  children?: React.ReactNode;
  defaultOpen?: boolean;
  className?: string;
  actions?: TreeSectionAction[];
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: React.MouseEvent) => void;
}

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
}

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
  isLastItem?: boolean;
  actions?: TreeSectionAction[];
  onDoubleClick?: (event: React.MouseEvent) => void;
}

interface SortableTreeItemsProps {
  items: { id: string; [key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => React.ReactNode;
}

export const TreeSection: React.FC<TreeSectionProps> = ({ label, id, icon, children, defaultOpen = true, className = "", actions = [], onPointerEnter: onSectionPointerEnter, onPointerLeave: onSectionPointerLeave, onDoubleClick }) => {
  const { level, isLastAtLevel, showLines } = React.useContext(TreeContext);
  const treeState = useTreeState();
  const { t } = useTranslation();
  const mode = useTooltipMode();
  const displayLabel = id ? useLabel(id) : label;
  const sectionId = `section-${displayLabel}`;
  const open = treeState.getOpenState(sectionId, defaultOpen);
  const setOpen = (value: boolean) => treeState.setOpenState(sectionId, value);
  const [isHovered, setIsHovered] = React.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const childrenArray = React.Children.toArray(children);
  const childrenInfo = {
    length: childrenArray.length,
    types: childrenArray.map((c) => {
      if (React.isValidElement(c)) {
        return { type: c.type, props: Object.keys(c.props || {}) };
      }
      return typeof c;
    }),
  };

  if (!hasChildren) {
    return (
      <div
        className={`relative flex items-center gap-single py-single hover:bg-hover-panel select-none overflow-hidden group min-w-0 cursor-selectable ${className}`}
        style={{ paddingLeft: `${level * 0.75}rem` }}
        onPointerEnter={() => {
          setIsHovered(true);
          onSectionPointerEnter?.();
        }}
        onPointerLeave={() => {
          setIsHovered(false);
          onSectionPointerLeave?.();
        }}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
      >
        <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
        <div className="w-[14px] flex-shrink-0" />
        {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
        {id ? (
          <Tooltip>
            <TooltipTrigger asChild>
              <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
            </TooltipTrigger>
            <TooltipContent>
              <IdTooltipContent id={id} />
            </TooltipContent>
          </Tooltip>
        ) : (
          <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
        )}
        {actions.length > 0 && (
          <div className="flex items-center gap-single">
            {actions.map((action, index) => (
              <Action
                key={index}
                level="panel"
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
          className={`relative flex items-center gap-single py-single hover:bg-hover-panel select-none overflow-hidden group min-w-0 cursor-foldable ${className}`}
          style={{ paddingLeft: `${level * 0.75}rem` }}
          role="button"
          onPointerEnter={() => {
            setIsHovered(true);
            onSectionPointerEnter?.();
          }}
          onPointerLeave={() => {
            setIsHovered(false);
            onSectionPointerLeave?.();
          }}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
        >
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
          {open ? <ChevronDownIcon className="size-[14px] flex-shrink-0" /> : <ChevronRightIcon className="size-[14px] flex-shrink-0" />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          {id ? (
            <Tooltip>
              <TooltipTrigger asChild>
                <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
              </TooltipTrigger>
              <TooltipContent>
                <IdTooltipContent id={id} />
              </TooltipContent>
            </Tooltip>
          ) : (
            <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
          )}
          {actions.length > 0 && (
            <div className="flex items-center gap-single">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  level="panel"
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
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines }}>{children}</TreeContext.Provider>
      </CollapsibleContent>
    </Collapsible>
  );
};

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
  const { level, isLastAtLevel, showLines } = React.useContext(TreeContext);
  const treeState = useTreeState();
  const displayLabel = id ? useLabel(id) : label;
  const itemKey = id ?? displayLabel ?? id;
  const itemId = `item-${id}-${itemKey}`;
  const open = treeState.getOpenState(itemId, defaultOpen);
  const setOpen = (value: boolean) => treeState.setOpenState(itemId, value);
  const [isHovered, setIsHovered] = React.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
    paddingLeft: `${level * 0.75}rem`,
  };

  const baseClasses = `relative flex items-center gap-single py-single hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${hasChildren ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-active-base text-active-foreground" : ""} ${isHighlighted ? "bg-active-base text-active-foreground" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren && displayLabel) {
    return (
      <>
        <div
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
            className="flex-shrink-0 bg-transparent cursor-foldable"
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              setOpen(!open);
            }}
          >
            {open ? <ChevronDownIcon className="size-3 flex-shrink-0" /> : <ChevronRightIcon className="size-3 flex-shrink-0" />}
          </button>
          {isDragHandle && <Action level="panel" className="cursor-grab active:cursor-grabbing" {...attributes} {...listeners} onClick={(e) => e.stopPropagation()} icon={<GripVerticalIcon size={12} className="text-muted-foreground" />} />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span
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
                  level="panel"
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
        {open && <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines }}>{children}</TreeContext.Provider>}
      </>
    );
  }

  if (!displayLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines }}>{children}</TreeContext.Provider>;
  }

  return (
    <div
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
      {isDragHandle && <Action level="panel" className="cursor-grab active:cursor-grabbing" {...attributes} {...listeners} icon={<GripVerticalIcon size={12} className="text-muted-foreground" />} />}
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-xs font-normal truncate text-foreground">{displayLabel as React.ReactNode}</span>
      {actions.length > 0 && (
        <div className="flex items-center gap-single">
          {actions.map((action, index) => (
            <Action
              key={index}
              level="panel"
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
}) => {
  const { t } = useTranslation();
  const resolvedLabel = id ? useLabel(id) : label;
  if (sortable && sortableId) {
    return (
      <SortableTreeItem
        id={sortableId}
        label={resolvedLabel}
        icon={icon}
        children={children}
        onClick={onClick}
        className={className}
        isSelected={isSelected}
        isHighlighted={isHighlighted}
        isDragHandle={isDragHandle}
        defaultOpen={defaultOpen}
        isLastItem={isLastItem}
        actions={actions}
        onDoubleClick={onDoubleClick}
      />
    );
  }

  const { level, isLastAtLevel, showLines } = React.useContext(TreeContext);
  const treeState = useTreeState();
  const itemKey = id ?? resolvedLabel ?? sortableId ?? "tree-item";
  const itemId = `item-${itemKey}`;
  const open = treeState.getOpenState(itemId, defaultOpen);
  const setOpen = (value: boolean) => treeState.setOpenState(itemId, value);
  const [isHovered, setIsHovered] = React.useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const baseClasses = `relative flex items-center gap-single py-single hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${hasChildren ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-active-base text-active-foreground" : ""} ${isHighlighted ? "bg-active-base text-active-foreground" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren && resolvedLabel) {
    return (
      <>
        <div
          className={itemClasses}
          style={{ paddingLeft: `${level * 0.75}rem` }}
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
            {open ? <ChevronDownIcon className="size-3 flex-shrink-0" /> : <ChevronRightIcon className="size-3 flex-shrink-0" />}
          </button>
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span
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
                  level="panel"
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
        {open && <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines }}>{children}</TreeContext.Provider>}
      </>
    );
  }

  if (!resolvedLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines }}>{children}</TreeContext.Provider>;
  }

  return (
    <div className={itemClasses} style={{ paddingLeft: `${level * 0.75}rem` }} onClick={onClick} onMouseEnter={() => setIsHovered(true)} onMouseLeave={() => setIsHovered(false)}>
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-xs font-normal truncate text-foreground">{resolvedLabel as React.ReactNode}</span>
      {actions.length > 0 && (
        <div className="flex items-center gap-single">
          {actions.map((action, index) => (
            <Action
              key={index}
              level="panel"
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

export const TreeItems: React.FC<{ children: React.ReactNode[]; renderItem: (child: React.ReactNode, index: number, isLast: boolean) => React.ReactNode }> = ({ children, renderItem }) => {
  return <>{children.map((child, index) => renderItem(child, index, index === children.length - 1))}</>;
};

export interface FileTreeNode {
  title: string;
  path: string;
  icon?: string;
  isFolder: boolean;
  children?: FileTreeNode[];
}

export const Tree: React.FC<{ children: React.ReactNode; className?: string; showLines?: boolean }> & {
  Files: React.FC<TreeFilesProps>;
  Section: React.FC<TreeFilesProps>;
} = ({ children, className = "", showLines = true }) => {
  return (
    <TreeStateProvider>
      <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines }}>
        <div className={`w-full min-w-0 overflow-hidden ${className}`}>{children}</div>
      </TreeContext.Provider>
    </TreeStateProvider>
  );
};

interface FileTreeItemProps {
  node: FileTreeNode;
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
}

const FileTreeItem: React.FC<FileTreeItemProps> = ({ node, currentPath, onNavigate, as = "a" }) => {
  const { level } = React.useContext(TreeContext);
  const [isHovered, setIsHovered] = React.useState(false);
  const treeState = useTreeState();
  const itemId = `file-${node.path}`;
  const open = treeState.getOpenState(itemId, true);
  const setOpen = (value: boolean) => treeState.setOpenState(itemId, value);

  const isActive = currentPath === node.path;
  const hasChildren = node.children && node.children.length > 0;
  const Icon = node.isFolder ? FolderIcon : DocumentIcon;

  const baseClasses = "relative flex items-center gap-single py-single px-tiny rounded-md hover:bg-accent transition-colors cursor-selectable select-none";
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
    style: { paddingLeft: `${level * 1 + 0.75}rem` },
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
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [], showLines: false }}>
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

interface TreeFilesProps {
  title?: string;
  nodes: FileTreeNode[];
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
  className?: string;
}

Tree.Files = ({ title = "In this section", nodes, currentPath, onNavigate, as = "a", className = "" }: TreeFilesProps) => {
  if (nodes.length === 0) return null;

  return (
    <TreeStateProvider>
      <div className={`not-prose my-medium p-medium rounded-lg border border-border bg-card ${className}`}>
        {title && <h3 className="text-lg font-semibold mb-4">{title}</h3>}
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false }}>
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

Tree.Section = Tree.Files;

export const FileTree = Tree.Files;

// #endregion Tree

// #endregion Aggregation Components

// #region Navigation Components

// #region Breadcrumb

export interface BreadcrumbItemData {
  id?: string;
  content: React.ReactNode;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
}

interface BreadcrumbProps extends Omit<React.ComponentProps<"nav">, "children"> {
  items: BreadcrumbItemData[];
  level?: Level;
}

function Breadcrumb({ className, items, level: propLevel, ...props }: BreadcrumbProps) {
  const level = useElementLevel(propLevel);
  const [openIndex, setOpenIndex] = React.useState<number | null>(null);

  return (
    <nav aria-label="breadcrumb" data-slot="breadcrumb" className={cn("flex h-medium items-stretch border border-border bg-base", className)} {...props}>
      <ol data-slot="breadcrumb-list" className="flex flex-wrap items-stretch text-xs break-words overflow-hidden h-full">
        {items.map((item, index) => {
          const isLast = index === items.length - 1;
          const hasOptions = item.options && item.options.length > 0;
          const showSeparator = !isLast;
          const isOpen = openIndex === index;

          return (
            <React.Fragment key={index}>
              <BreadcrumbItem {...item} level={level} open={isOpen} onOpenChange={(open) => setOpenIndex(open ? index : null)} />
              {showSeparator && (
                <BreadcrumbSeparatorItem
                  level={level}
                  hasOptions={hasOptions}
                  isOpen={isOpen}
                  onClick={hasOptions ? () => setOpenIndex(isOpen ? null : index) : undefined}
                />
              )}
            </React.Fragment>
          );
        })}
      </ol>
    </nav>
  );
}

interface BreadcrumbItemProps extends React.ComponentProps<"li"> {
  id?: string;
  content?: React.ReactNode;
  options?: { label: React.ReactNode; href: string; id?: string }[];
  onNavigate?: (href: string) => void;
  level?: Level;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}

function BreadcrumbItem({ className, id, content, children, options, onNavigate, level: propLevel, open = false, onOpenChange, ...props }: BreadcrumbItemProps) {
  const level = useElementLevel(propLevel);
  const hoverClass = level === "panel" ? "hover:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary" : "hover:bg-hover-base";

  const handleSelect = (href: string) => {
    onOpenChange?.(false);
    onNavigate?.(href);
  };

  const itemContent = content ?? children;

  if (!options?.length) {
    const itemElement = (
      <li data-slot="breadcrumb-item" className={cn("flex items-stretch border-l first:border-l-0", className)} {...props}>
        {itemContent}
      </li>
    );

    if (id) {
      return (
        <Tooltip>
          <TooltipTrigger asChild>{itemElement}</TooltipTrigger>
          <TooltipContent>
            <IdTooltipContent id={id} />
          </TooltipContent>
        </Tooltip>
      );
    }

    return itemElement;
  }

  const itemElement = (
    <li data-slot="breadcrumb-item" className={cn("flex items-stretch border-l first:border-l-0", className)} {...props}>
      <DropdownMenuPrimitive.Root open={open} onOpenChange={onOpenChange}>
        <DropdownMenuPrimitive.Trigger asChild>
          <div className={cn("flex items-center cursor-pointer", hoverClass)}>{itemContent}</div>
        </DropdownMenuPrimitive.Trigger>
        <DropdownMenuPrimitive.Portal>
          <DropdownMenuPrimitive.Content align="start" sideOffset={8} className="bg-temporary w-auto overflow-hidden border p-single">
            {options.map((item, index) => {
              const labelKeys = typeof item.label === "object" && item.label !== null && !React.isValidElement(item.label) ? Object.keys(item.label) : undefined;
              console.log(`[DEBUG] [BREADCRUMB-RENDER] Rendering dropdown item:`, {
                item,
                label: item.label,
                labelType: typeof item.label,
                isObject: typeof item.label === "object" && item.label !== null,
                isReactElement: React.isValidElement(item.label),
                keys: labelKeys,
                labelValue: labelKeys ? item.label : "N/A",
              });
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
                  <TooltipContent>
                    <IdTooltipContent id={item.id} />
                  </TooltipContent>
                </Tooltip>
              ) : (
                menuItem
              );

              if (index < options.length - 1) {
                return (
                  <React.Fragment key={index}>
                    {wrappedItem}
                    <DropdownMenuPrimitive.Separator className="h-px bg-border my-single" />
                  </React.Fragment>
                );
              }

              return wrappedItem;
            })}
          </DropdownMenuPrimitive.Content>
        </DropdownMenuPrimitive.Portal>
      </DropdownMenuPrimitive.Root>
    </li>
  );

  if (id) {
    return (
      <Tooltip>
        <TooltipTrigger asChild>{itemElement}</TooltipTrigger>
        <TooltipContent>
          <IdTooltipContent id={id} />
        </TooltipContent>
      </Tooltip>
    );
  }

  return itemElement;
}

interface BreadcrumbSeparatorItemProps {
  level: Level;
  hasOptions: boolean;
  isOpen: boolean;
  onClick?: () => void;
}

function BreadcrumbSeparatorItem({ level, hasOptions, isOpen, onClick }: BreadcrumbSeparatorItemProps) {
  const hoverClass = level === "panel" ? "hover:bg-hover-panel" : level === "temporary" ? "hover:bg-hover-temporary" : "hover:bg-hover-base";

  return (
    <li
      data-slot="breadcrumb-separator"
      role="presentation"
      aria-hidden="true"
      className={cn("[&>svg]:size-tiny px-single flex items-center self-stretch border-l", hasOptions && "cursor-pointer", hasOptions && hoverClass)}
      onClick={onClick}
    >
      {isOpen ? <ChevronDownIcon /> : <ChevronRightIcon />}
    </li>
  );
}

export { Breadcrumb, BreadcrumbItem };
export type { BreadcrumbItemData };

// #region PageNavigation

export interface PageNavigationLink {
  path: string;
  title: string;
  section?: string;
}

export interface PageNavigationProps {
  prev?: PageNavigationLink;
  next?: PageNavigationLink;
}

const PageNavigation: React.FC<PageNavigationProps> = ({ prev, next }) => {
  const navigate = useNavigate();
  const { t } = useTranslation();

  if (!prev && !next) return null;

  return (
    <div className="flex items-center justify-between border-t border-border pt-4 mt-8">
      {prev ? (
        <Button onClick={() => navigate(`/${prev.path}`)} className="flex items-center gap-single">
          <ChevronLeftIcon className="size-tiny" />
          <div className="text-left">
            <div className="text-xs text-muted-foreground">{t("pageNavigation.previous")}</div>
            <div className="font-medium">{prev.title}</div>
          </div>
        </Button>
      ) : (
        <div />
      )}
      {next ? (
        <Button onClick={() => navigate(`/${next.path}`)} className="flex items-center gap-single">
          <div className="text-right">
            <div className="text-xs text-muted-foreground">{t("pageNavigation.next")}</div>
            <div className="font-medium">{next.title}</div>
          </div>
          <ChevronRightIcon className="size-tiny" />
        </Button>
      ) : (
        <div />
      )}
    </div>
  );
};

export { PageNavigation };

// #endregion PageNavigation

// #region SectionTree

export interface SectionTreeProps {
  title?: string;
  section?: string;
}

export const SectionTree: React.FC<SectionTreeProps> = ({ title, section }) => {
  const location = useLocation();
  const navigate = useNavigate();

  const getDocsRegistry = () => {
    const docsApp = require("../apps/docs/App");
    return docsApp.docsRegistry;
  };

  const docsRegistry = getDocsRegistry();

  const currentSection =
    section ||
    (() => {
      const path = location.pathname.replace(/^\/docs\//, "");
      const parts = path.split("/");
      return parts[0];
    })();

  const currentPath = location.pathname.replace(/^\//, "");
  const tree = docsRegistry.getSectionTree(currentSection);

  const handleNavigate = (path: string) => {
    navigate(`/${path}`);
  };

  return <Tree.Files title={title} nodes={tree} currentPath={currentPath} onNavigate={handleNavigate} as="div" />;
};

// #endregion SectionTree

// #endregion Breadcrumb

// #endregion Navigation Components

// #region Panel Components

// #region Panel

export type ResizeSide = "left" | "right" | "top" | "bottom";

export interface PanelSection {
  id: string;
  content: React.ReactNode | (() => React.ReactNode);
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
}

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
}) => {
  const { t } = useTranslation();
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
  const containerClass = `absolute top-0 bottom-0 text-foreground border min-w-0 overflow-hidden ${showBackground ? "bg-panel" : ""} ${borderClass} ${className}`;
  const hasContent = sortedSections.length > 0 || additionalContent;
  const isHorizontal = resizeSide === "left" || resizeSide === "right";
  const positionStyle = isHorizontal
    ? resizeSide === "right"
      ? { left: 0, width: `${size}px`, zIndex }
      : { right: 0, width: `${size}px`, zIndex }
    : resizeSide === "top"
      ? { top: 0, height: `${size}px`, zIndex }
      : { bottom: 0, height: `${size}px`, zIndex };
  const resizeHandleClass = isHorizontal ? `absolute top-0 bottom-0 ${resizeSide === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize` : `absolute left-0 right-0 ${resizeSide === "top" ? "top-0" : "bottom-0"} h-single cursor-ns-resize`;
  return (
    <div className={containerClass} style={{ ...positionStyle, opacity, transition: "opacity 150ms" }}>
      <Scrollable className={`h-full ${showBackground ? "bg-panel" : ""}`}>
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
  );
};

const PanelSectionWrapper: React.FC<{ section: PanelSection; defaultOpen: boolean; children: React.ReactNode }> = ({ section, defaultOpen, children }) => {
  const sectionLabel = useLabel(section.id, section.id);
  return (
    <TreeSection label={sectionLabel} id={section.id} defaultOpen={defaultOpen} actions={section.actions} onPointerEnter={section.onPointerEnter} onPointerLeave={section.onPointerLeave} onDoubleClick={section.onDoubleClick}>
      {children}
    </TreeSection>
  );
};

export { Panel };

// #endregion Panel

// #region PanelGroup

export interface PanelGroupProps {
  children: React.ReactNode;
  className?: string;
  position?: "left" | "right" | "middle" | "bottom";
}

const PanelGroup: React.FC<PanelGroupProps> = ({ children, className = "", position = "middle" }) => {
  const baseClass = "flex";
  const positionClass = position === "left" || position === "right" || position === "middle" ? "flex-col" : "flex-row";
  return <div className={`${baseClass} ${positionClass} ${className}`}>{children}</div>;
};

export { PanelGroup };

// #endregion PanelGroup

// #region LeftPanel

export type LeftPanelProps = Omit<PanelProps, "resizeSide">;

const LeftPanel: React.FC<LeftPanelProps> = (props) => <Panel {...props} resizeSide="right" />;

export { LeftPanel };

// #endregion LeftPanel

// #region RightPanel

export type RightPanelProps = Omit<PanelProps, "resizeSide">;

const RightPanel: React.FC<RightPanelProps> = (props) => <Panel {...props} resizeSide="left" />;

export { RightPanel };

// #endregion RightPanel

// #region MiddlePanel

export type MiddlePanelProps = Omit<PanelProps, "resizeSide"> & {
  resizeSide?: "left" | "right";
};

const MiddlePanel: React.FC<MiddlePanelProps> = ({ resizeSide = "right", ...props }) => <Panel {...props} resizeSide={resizeSide} />;

export { MiddlePanel };

// #endregion MiddlePanel

// #region BottomPanel

export type BottomPanelProps = Omit<PanelProps, "resizeSide">;

const BottomPanel: React.FC<BottomPanelProps> = (props) => <Panel {...props} resizeSide="top" />;

export { BottomPanel };

// #endregion BottomPanel

// #endregion Panel Components

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

interface WindowProps extends WindowConfig {
  isVisible?: boolean;
}

const DefaultErrorDisplay: React.FC<{ error: Error }> = ({ error }) => (
  <div className="flex flex-col items-center justify-center h-full w-full bg-background p-small">
    <div className="text-center space-y-2 max-w-md">
      <div className="text-4xl mb-4">⚠️</div>
      <h3 className="text-lg font-medium">Error</h3>
      <p className="text-sm text-muted-foreground">{error.message}</p>
    </div>
  </div>
);

const Window: React.FC<WindowProps> = ({ id, children, onDoubleClick, className = "", isVisible = true, loading = false, error = null, skeleton, showControls = false, onOpenInNewWindow, onMaximize, onMinimize, onClose, controls }) => {
  const [isMaximized, setIsMaximized] = React.useState(false);

  const handleMaximize = () => {
    setIsMaximized(!isMaximized);
    if (isMaximized && onMinimize) onMinimize();
    else if (!isMaximized && onMaximize) onMaximize();
  };

  if (!isVisible) return null;
  return (
    <div className={`relative h-full w-full ${className}`} onDoubleClick={onDoubleClick}>
      {(showControls || controls) && (
        <div className="absolute top-1 right-1 z-10">
          {controls || (
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
      )}
      {error ? <DefaultErrorDisplay error={error} /> : loading && skeleton ? skeleton : children}
    </div>
  );
};

export { Window };

// #endregion Window

// #region Page

export interface PageFrontmatter {
  title?: string;
  description?: string;
  icon?: string;
  sidebar?: boolean;
  order?: number;
  concepts?: string[];
}

export interface PageProps {
  frontmatter?: PageFrontmatter;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  footer?: React.ReactNode;
  children: React.ReactNode;
}

export const Page: React.FC<PageProps> = ({ frontmatter, focusedItemId, onFocusComplete, footer, children }) => {
  const scrollAreaRef = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const element = scrollAreaRef.current.querySelector(`#${focusedItemId}`);
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

export type DiagramLayoutDirection = "TB" | "BT" | "LR" | "RL";

export interface DiagramLayoutOptions {
  direction?: DiagramLayoutDirection;
  nodeWidth?: number;
  nodeHeight?: number;
  rankSep?: number;
  nodeSep?: number;
}

export function calculateDiagramLayout(nodes: Node[], edges: Edge[], options: DiagramLayoutOptions = {}): { nodes: Node[]; edges: Edge[] } {
  const { direction = "TB", nodeWidth = 48, nodeHeight = 48, rankSep = 80, nodeSep = 50 } = options;

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
  onMoveEnd?: () => void;
  reactFlowInstanceRef?: React.RefObject<ReactFlowInstance | null>;
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
}

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
  onNodeDragStart,
  onNodeDrag,
  onNodeDragStop,
  onEdgeClick,
  onEdgeMouseEnter,
  onEdgeMouseLeave,
  onPaneClick,
  onPaneDoubleClick,
  onMoveEnd,
  reactFlowInstanceRef,
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
}) => {
  const isControlled = controlledNodes !== undefined && controlledEdges !== undefined;

  const [internalNodes, setInternalNodes, onInternalNodesChange] = useNodesState(initialNodes);
  const [internalEdges, setInternalEdges, onInternalEdgesChange] = useEdgesState(initialEdges);

  const finalNodes = isControlled ? controlledNodes : internalNodes;
  const finalEdges = isControlled ? controlledEdges : internalEdges;
  const finalOnNodesChange = isControlled ? onNodesChangeReactFlow : onInternalNodesChange;
  const finalOnEdgesChange = isControlled ? onEdgesChangeReactFlow : onInternalEdgesChange;

  const handleInit = React.useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        reactFlowInstanceRef.current = instance;
      }
    },
    [reactFlowInstanceRef],
  );

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
  }, [initialNodes, initialEdges, isControlled, setInternalNodes, setInternalEdges]);

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
    <div ref={wrapperRef} className={`relative w-full h-full ${className}`}>
      <ReactFlow
        nodes={finalNodes}
        edges={finalEdges}
        onNodesChange={finalOnNodesChange}
        onEdgesChange={finalOnEdgesChange}
        onConnect={onConnect}
        onInit={handleInit}
        onNodeClick={onNodeClick}
        onNodeDoubleClick={onNodeDoubleClick}
        onNodeMouseEnter={onNodeMouseEnter}
        onNodeMouseLeave={onNodeMouseLeave}
        onNodeDragStart={onNodeDragStart}
        onNodeDrag={onNodeDrag}
        onNodeDragStop={onNodeDragStop}
        onEdgeClick={onEdgeClick}
        onEdgeMouseEnter={onEdgeMouseEnter}
        onEdgeMouseLeave={onEdgeMouseLeave}
        onPaneClick={onPaneClick}
        onDoubleClick={onPaneDoubleClick}
        onMoveEnd={onMoveEnd}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionLineComponent={connectionLineComponent}
        fitView={fitView}
        minZoom={minZoom}
        maxZoom={maxZoom}
        connectionMode={connectionMode as any}
        deleteKeyCode={deleteKeyCode}
        panOnDrag={panOnDrag}
        selectionOnDrag={selectionOnDrag}
        zoomOnScroll={zoomOnScroll}
        zoomOnPinch={zoomOnPinch}
        zoomOnDoubleClick={zoomOnDoubleClick}
        elementsSelectable={elementsSelectable}
        nodesFocusable={nodesFocusable}
        edgesFocusable={edgesFocusable}
        nodesDraggable={nodesDraggable}
        proOptions={{ hideAttribution: true }}
        className="bg-background"
      >
        {showMinimap && <MiniMap className="border border-border" maskColor="var(--accent)" bgColor="var(--background)" nodeStrokeWidth={3} zoomable pannable nodeComponent={miniMapNodeComponent} />}
        {panels}
      </ReactFlow>
    </div>
  );
};

export const Diagram: React.FC<DiagramProps> = (props) => {
  return (
    <ReactFlowProvider>
      <DiagramInner {...props} />
    </ReactFlowProvider>
  );
};

export function useDiagramLayout(initialNodes: Node[], initialEdges: Edge[], layoutOptions?: DiagramLayoutOptions): { nodes: Node[]; edges: Edge[] } {
  return React.useMemo(() => {
    if (initialNodes.length === 0) {
      return { nodes: [], edges: [] };
    }
    return calculateDiagramLayout(initialNodes, initialEdges, layoutOptions);
  }, [initialNodes, initialEdges, layoutOptions]);
}

interface DiagramSkeletonProps {
  nodeCount?: number;
  edgeCount?: number;
  className?: string;
}

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
        nodesConnectable={false}
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

const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

let selectableCursorUsageCount = 0;

export interface SceneModel {
  guid: string;
  plane?: Plane;
  isSelected?: boolean;
  isHovered?: boolean;
  isFocusable?: boolean;
  onClick?: () => void;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
}

export interface TransformableModel extends SceneModel {
  isTransformable?: boolean;
}

export interface PlaneTransformDelta {
  translation?: { x: number; y: number; z: number };
  rotation?: { x: number; y: number; z: number; w: number };
  scale?: number;
}

export type OnPlaneUpdate = (modelGuid: string, newPlane: Plane) => void;

export type OnMultiPlaneUpdate = (updates: Array<{ modelGuid: string; newPlane: Plane }>) => void;

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

export const getPlanePosition = (plane: Plane): THREE.Vector3 => {
  return new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
};

export const hasValidPlane = (model: SceneModel): boolean => {
  return model.plane !== undefined && model.plane !== null;
};

export const isModelFocusable = (model: SceneModel): boolean => {
  return hasValidPlane(model) && (model.isFocusable === undefined || model.isFocusable === true);
};

interface ModelProps {
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

export const Model: React.FC<ModelProps> = ({ children, selected = false, hovered = false, onClick, onDoubleClick, onPointerEnter, onPointerLeave, color, emissiveColor, emissiveIntensity = 0.45, showEdges = true, edgeColor, userData }) => {
  const foregroundColor = React.useMemo(() => getComputedColor("--foreground"), []);
  const activeBaseColor = React.useMemo(() => getComputedColor("--active-base"), []);
  const hoverBaseColor = React.useMemo(() => getComputedColor("--hover-base"), []);
  const [isPointerOver, setIsPointerOver] = React.useState(false);
  const isInteractive = Boolean(onClick || onDoubleClick);

  const resolvedColor = React.useMemo(() => {
    if (color) return color;
    if (selected) return activeBaseColor;
    if (hovered) return hoverBaseColor;
    return foregroundColor;
  }, [color, selected, hovered, activeBaseColor, hoverBaseColor, foregroundColor]);

  const resolvedEmissiveColor = emissiveColor || resolvedColor;
  const resolvedEdgeColor = edgeColor || foregroundColor;
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

interface GltfProps {
  src: string;
  roughness?: number;
  metalness?: number;
}
const Gltf: React.FC<GltfProps> = ({ src, roughness, metalness }) => {
  const { scene } = useGLTF(src);

  React.useEffect(() => {
    if (roughness !== undefined || metalness !== undefined) {
      scene.traverse((node) => {
        if ((node as any).isMesh && (node as any).material) {
          if ((node as any).material.roughness !== undefined && roughness !== undefined) {
            (node as any).material.roughness = roughness;
          }
          if ((node as any).material.metalness !== undefined && metalness !== undefined) {
            (node as any).material.metalness = metalness;
          }

          if (Array.isArray((node as any).material)) {
            (node as any).material.forEach((material: any) => {
              if (material.roughness !== undefined && roughness !== undefined) {
                material.roughness = roughness;
              }
              if (material.metalness !== undefined && metalness !== undefined) {
                material.metalness = metalness;
              }
            });
          }

          if ((node as any).material.needsUpdate !== undefined) {
            (node as any).material.needsUpdate = true;
          }
        }
      });
    }
  }, [scene, roughness, metalness]);

  return <primitive object={scene} />;
};

interface ModelFileProps {
  src: string;
  environment?: string;
  roughness?: number;
  metalness?: number;
}
const ModelFile: React.FC<ModelFileProps> = ({ src, environment, roughness, metalness }) => {
  return (
    <div className="w-full h-full">
      <Model>
        <React.Suspense fallback={null}>
          <Gltf src={src} roughness={roughness} metalness={metalness} />
        </React.Suspense>
      </Model>
    </div>
  );
};

interface GizmoProps {
  show?: boolean;
}

const Gizmo: React.FC<GizmoProps> = ({ show = true }) => {
  const colors = React.useMemo(() => [getComputedColor("--accent"), getComputedColor("--accent-tertiary"), getComputedColor("--accent-secondary")] as [string, string, string], []);
  const labels = React.useMemo(() => ["X", "Z", "-Y"] as [string, string, string], []);
  const margin = React.useMemo(() => [80, 80] as [number, number], []);
  if (!show) return null;
  return (
    <GizmoHelper alignment="bottom-right" margin={margin}>
      <GizmoViewport labels={labels} axisColors={colors} />
    </GizmoHelper>
  );
};

interface SceneInnerProps {
  children?: React.ReactNode;
  showGrid?: boolean;
  showGizmo?: boolean;
  camera?: Camera;
  onCameraChange?: (camera: Camera) => void;
  focusedItemId?: string;
  onFocusComplete?: () => void;
}

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

      if (forwardVec.lengthSq() < 0.0001) return;

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
          console.log(`Focus: Object ${focusedItemId} not found, retrying... (${retryCount}/${maxRetries})`);
          setTimeout(findAndFocusObject, 50);
        } else {
          console.warn(`Focus: Object ${focusedItemId} not found after ${maxRetries} retries`);
          if (onFocusComplete) onFocusComplete();
        }
        return;
      }

      console.log(`Focus: Found object ${focusedItemId}, starting zoom animation`);

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
        <div className="absolute top-1 right-1 z-10">
          <ActionDropdown id="scene-projection" options={projectionOptions} value={projection} onValueChange={(value) => onProjectionChange(value as "camera" | "orthographic")} level="base" />
        </div>
      )}
      <ThreeCanvas onPointerMissed={onPointerMissed} orthographic={orthographic} shadows={shadows} camera={orthographic ? { zoom: 50, position: [10, 10, 10] } : undefined} style={{ width: "100%", height: "100%" }}>
        <SceneInner showGrid={showGrid} showGizmo={showGizmo} camera={camera} onCameraChange={onCameraChange} focusedItemId={focusedItemId} onFocusComplete={onFocusComplete}>
          {children}
        </SceneInner>
      </ThreeCanvas>
    </div>
  );
};

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

export type SortDirection = "asc" | "desc";

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

export interface HierarchicalRowData {
  id: string;
  level?: number;
  parentId?: string;
  hasChildren?: boolean;
  isExpanded?: boolean;
}

export interface DragDropConfig {
  enabled?: boolean;
  onDragStart?: (rowId: string) => void;
  onDragEnd?: (event: { active: string; over: string | null }) => void;
  canDrag?: (rowId: string) => boolean;
  canDrop?: (draggedId: string, targetId: string) => boolean;
  renderDragOverlay?: (rowId: string) => React.ReactNode;
}

export interface TableProps<T = unknown> {
  columns: TableColumn<T>[];
  data: T[];
  onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowDoubleClick?: (row: T, index: number) => void;
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

const Table = <T,>({
  columns,
  data,
  onRowClick,
  onRowDoubleClick,
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

  // Configure sensors with activation constraint to allow clicks
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8, // Require 8px of movement before drag starts
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
    compact: "h-large",
    normal: "h-large",
    comfortable: "h-large",
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

    const baseRowClassName = `border-b ${rowHeightClass} ${isSelected ? "bg-active-base text-active-foreground" : isOver ? "bg-hover-base ring-2 ring-active" : "hover:bg-hover-base"}`;
    const isDragging = activeId === rowId || isDraggingHook;

    return (
      <tr
        ref={combinedRef}
        style={style}
        className={`${baseRowClassName} ${customRowClassName} ${isDragging ? "opacity-50" : ""} ${onRowClick ? "cursor-selectable" : ""}`}
        onClick={(e) => onRowClick?.(row, index, e)}
        onDoubleClick={() => onRowDoubleClick?.(row, index)}
        {...(canDragRow ? { ...attributes, ...listeners } : {})}
        role={onRowClick ? "button" : undefined}
        tabIndex={onRowClick ? 0 : undefined}
        data-row-id={rowId}
      >
        {visibleColumns.map((column) => (
          <td key={column.id} className={`p-single ${column.className || ""}`}>
            {column.accessor(row)}
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
                  <div key={key} data-row>
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
          <thead className={`bg-base border-b ${stickyHeader ? "sticky top-0 z-10" : ""} ${headerClassName}`}>
            <tr className="h-large">
              {visibleColumns.map((column) => (
                <th key={column.id} className={`text-left p-single font-medium ${rowHeightClass} ${column.headerClassName || column.className || ""}`} style={{ width: column.width }}>
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

                const baseRowClassName = `border-b ${rowHeightClass} ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`;
                const isDragging = activeId === rowId;

                return (
                  <tr
                    key={key}
                    className={`${baseRowClassName} ${customRowClassName} ${isDragging ? "opacity-50" : ""} ${onRowClick ? "cursor-selectable" : ""}`}
                    onClick={(e) => onRowClick?.(row, index, e)}
                    onDoubleClick={() => onRowDoubleClick?.(row, index)}
                    role={onRowClick ? "button" : undefined}
                    tabIndex={onRowClick ? 0 : undefined}
                    data-row-id={rowId}
                  >
                    {visibleColumns.map((column) => (
                      <td key={column.id} className={`p-single ${column.className || ""}`}>
                        {column.accessor(row)}
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

export interface TableSkeletonProps {
  columns: TableColumn[];
  rowCount?: number;
  className?: string;
}

export const TableSkeleton: React.FC<TableSkeletonProps> = ({ columns, rowCount = 5, className = "" }) => (
  <Scrollable className={`h-full w-full ${className}`}>
    <table className="w-full border-collapse">
      <thead className="bg-panel border-b sticky top-0 z-10">
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
          <tr key={index} className="border-b h-large">
            {columns.map((column) => (
              <td key={column.id} className={`p-single text-sm ${column.className || ""}`}>
                <div className="h-small bg-muted-foreground/20 rounded animate-pulse" />
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  </Scrollable>
);

// #endregion Table

// #endregion Window Components
