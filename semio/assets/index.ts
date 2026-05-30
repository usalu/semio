// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, representations and images.

// #endregion 🧲Header

//#region 🗃️Exports
// Builtin Metabolism kit and UI icons only — test JSON lives in `@semio/fixtures`.

import MetabolismKitData from "../fixtures/kit/dev/metabolism/wip/initialKit/kit.semio.json";

export {
  Plus as AddIcon, AlertCircle as AlertCircleIcon,
  ArrowLeft as ArrowLeftIcon,
  Award as AwardIcon,
  BookOpen as BookIcon,
  Camera as CameraIcon,
  MessageCircle as ChatIcon,
  Check as CheckIcon,
  CheckIcon as CheckIconAlt,
  ChevronDown as ChevronDownIcon,
  ChevronDownIcon as ChevronDownIconAlt,
  ChevronLeft as ChevronLeftIcon,
  ChevronRight as ChevronRightIcon,
  ChevronsUpDown as ChevronsUpDownIcon,
  X as CloseIcon,
  XIcon as CloseIconAlt,
  Code as CodeIcon,
  Network as ConnectionIcon,
  Crosshair as ConnectorIcon, Copy as CopyIcon, Info as DetailsIcon,
  Grid3x3 as DiagramIcon,
  Link2Off as DisconnectIcon,
  FileText as DocumentIcon,
  ExternalLink as ExternalLinkIcon,
  FileArchive as FileArchiveIcon,
  FileCode as FileCodeIcon,
  FileImage as FileImageIcon,
  FileJson as FileJsonIcon,
  FileSpreadsheet as FileSpreadsheetIcon,
  FileType as FileTypeIcon,
  FileVideo as FileVideoIcon,
  Focus as FocusIcon,
  Folder as FolderIcon,
  Globe as GlobeIcon,
  GripVertical as GripVerticalIcon,
  Hand as HandIcon,
  Hash as HashIcon,
  Home as HomeIcon,
  Layers as HudIcon,
  PanelTop as HudPanelIcon,
  Info as InfoIcon,
  Combine as IntersectIcon,
  Layout as LayoutIcon,
  PanelLeft as LeftSidePanelIcon,
  Lightbulb as LightbulbIcon,
  Loader2 as LoaderIcon,
  HardDrive as LocalKitIcon,
  Maximize2 as Maximize2Icon,
  MessageCircle,
  Minimize2 as Minimize2Icon,
  Monitor as MonitorIcon,
  Moon as MoonIcon,
  MoreHorizontal as MoreHorizontalIcon,
  MousePointer as MousePointerIcon,
  ArrowLeft as NavigateBackIcon,
  ArrowRight as NavigateForwardIcon,
  ArrowUp as NavigateUpIcon,
  Pause as PauseIcon,
  Puzzle as PieceIcon,
  Play as PlayIcon,
  Plug as PortIcon,
  Circle as RecordIcon,
  Cloud as RemoteKitIcon,
  Minus as RemoveIcon, RotateCcw as ResetIcon, PanelRight as RightSidePanelIcon,
  Eye as SceneIcon,
  SearchIcon as SearchIcon,
  MousePointer2 as SelectToolIcon,
  Settings as SettingsIcon, SkipBack as SkipBackIcon,
  SkipForward as SkipForwardIcon,
  Smartphone as SmartphoneIcon,
  ArrowUp as SortAscendingIcon,
  ArrowDown as SortDescendingIcon,
  BarChart3 as StatsIcon,
  Square as StopIcon,
  Sun as SunIcon,
  Tablet as TabletIcon,
  Table2 as TableViewIcon,
  Clock as TemporaryKitIcon,
  Hammer as ToolbarIcon,
  Wrench as ToolsIcon,
  TriangleAlert as TriangleAlertIcon,
  GraduationCap as TutorialIcon,
  Box as TypeIcon,
  User as UserIcon,
  Box as WorkbenchIcon
} from "lucide-react";

export type { LucideIcon } from "lucide-react";

//#region 🔖KitBootstrapHelpers
/** @emoji 🧾 Normalizes list-or-{items} shapes found on `wip.initialKit` DTOs. */
function __itemsOf<T>(node: unknown): readonly T[] {
  if (Array.isArray(node)) return node as readonly T[];
  if (node && typeof node === "object" && "items" in node && Array.isArray((node as { items: unknown }).items)) return (node as { items: T[] }).items;
  return [];
}

/** @emoji 🧾 Resolves the materialized kit payload (legacy root vs `wip.initialKit`). */
function __metabolismKitInner(): Record<string, unknown> {
  const root = MetabolismKitData as { wip?: { initialKit?: Record<string, unknown> } };
  const inner = root.wip?.initialKit;
  return (inner && typeof inner === "object" ? inner : (MetabolismKitData as unknown as Record<string, unknown>)) ?? {};
}

/** @emoji 🏛️ Flattens kinds from root `types` or nested `typologies[].types`. */
function __kitTypesFromInner(inner: Record<string, unknown>): readonly unknown[] {
  const rootTypes = __itemsOf(inner["types"]);
  if (rootTypes.length > 0) return rootTypes;
  return __itemsOf(inner["typologies"]).flatMap((topo) => __itemsOf((topo as { types?: unknown }).types));
}

/** @emoji 🏛️ Flattens designs from root `designs` or nested `typologies[].designs`. */
function __kitDesignsFromInner(inner: Record<string, unknown>): readonly unknown[] {
  const rootDesigns = __itemsOf(inner["designs"]);
  if (rootDesigns.length > 0) return rootDesigns;
  return __itemsOf(inner["typologies"]).flatMap((topo) => __itemsOf((topo as { designs?: unknown }).designs));
}
//#endregion 🔖KitBootstrapHelpers
export { MetabolismKitData as MetabolismKit };

/**
 * Metabolism kit types array
 **/
export const MetabolismKitTypes = __kitTypesFromInner(__metabolismKitInner());
/**
 * Metabolism kit designs array
 **/
export const MetabolismKitDesigns = __kitDesignsFromInner(__metabolismKitInner());
/**
 * Metabolism kit typologies array
 **/
export const MetabolismKitTypologies = __itemsOf(__metabolismKitInner()["typologies"]);
/**
 * Metabolism kit families array
 **/
export const MetabolismKitFamilies = __itemsOf(__metabolismKitInner()["families"]);
/**
 * Metabolism kit qualities array
 **/
export const MetabolismKitQualities = __itemsOf(__metabolismKitInner()["qualities"]);
/**
 * Metabolism kit files array
 **/
export const MetabolismKitFiles = __itemsOf(__metabolismKitInner()["files"]);
/**
 * Metabolism kit folders array
 **/
export const MetabolismKitFolders = __itemsOf(__metabolismKitInner()["folders"]);
/**
 * Metabolism kit authors array
 **/
export const MetabolismKitAuthors = __itemsOf(__metabolismKitInner()["authors"]);
/**
 * Metabolism kit tags array
 **/
export const MetabolismKitTags = __itemsOf(__metabolismKitInner()["tags"]);
/**
 * Metabolism kit concepts array
 **/
export const MetabolismKitConcepts = __itemsOf(__metabolismKitInner()["concepts"]);
/**
 * Metabolism kit attributes array
 **/
export const MetabolismKitAttributes = __itemsOf(__metabolismKitInner()["attributes"]);
/**
 * Metabolism kit Nakagin Capsule Tower designs subset
 **/
export const MetabolismKitNakaginCapsuleTowerDesigns = MetabolismKitDesigns.filter((design) => String((design as { name?: string }).name ?? "") === "Nakagin Capsule Tower") ?? [];

/**
 * Builds id and name lookup maps from an item array
 *
 * Callers MUST provide an array of objects with optional id and name fields
 * buildLookup holds the data fields for a buildLookup record.
 **/
const buildLookup = (items: readonly any[] = []) => {
  const byId: Record<string, any> = {};
  const byName: Record<string, any> = {};
  items.forEach((item) => {
    if (!item) return;
    if (item.id) byId[item.id] = item;
    if (item.name) byName[item.name] = item;
  });
  return { byId, byName };
};

/**
 * typeLookup holds the data fields for a typeLookup record.
 **/
const typeLookup = buildLookup(MetabolismKitTypes);
/**
 * Design lookup maps by id and name
 **/
const designLookup = buildLookup(MetabolismKitDesigns);
/**
 * Typology lookup maps by id and name
 **/
const typologyLookup = buildLookup(MetabolismKitTypologies);
/**
 * Family lookup maps by id and name
 **/
const familyLookup = buildLookup(MetabolismKitFamilies);

/**
 * Metabolism kit types indexed by id
 **/
export const MetabolismKitTypesById = typeLookup.byId;
/**
 * Metabolism kit types indexed by name
 **/
export const MetabolismKitTypesByName = typeLookup.byName;
/**
 * Metabolism kit designs indexed by id
 **/
export const MetabolismKitDesignsById = designLookup.byId;
/**
 * Metabolism kit designs indexed by name
 **/
export const MetabolismKitDesignsByName = designLookup.byName;
/**
 * Metabolism kit typologies indexed by id
 **/
export const MetabolismKitTypologiesById = typologyLookup.byId;
/**
 * Metabolism kit typologies indexed by name
 **/
export const MetabolismKitTypologiesByName = typologyLookup.byName;
/**
 * Metabolism kit families indexed by id
 **/
export const MetabolismKitFamiliesById = familyLookup.byId;
/**
 * Metabolism kit families indexed by name
 **/
export const MetabolismKitFamiliesByName = familyLookup.byName;
/**
 * nakaginCapsuleTowerDesign holds the data fields for a nakaginCapsuleTowerDesign record.
 **/
const nakaginCapsuleTowerDesign = MetabolismKitDesigns.find((d) => String((d as { name?: string }).name ?? "") === "Nakagin Capsule Tower");
/**
 * nakaginCapsuleTowerFlatDesign holds the data fields for a nakaginCapsuleTowerFlatDesign record.
 **/
const nakaginCapsuleTowerFlatDesign = MetabolismKitDesigns.find(
  (d) => String((d as { name?: string }).name ?? "") === "Flat" && String((d as { parent?: { id?: string } }).parent?.id ?? "") === String((nakaginCapsuleTowerDesign as { id?: string } | undefined)?.id ?? ""),
);
/**
 * Nakagin Capsule Tower Flat variant piece data with plane and center
 **/
export const MetabolismKitNakaginCapsuleTowerFlatPieces =
  ((nakaginCapsuleTowerFlatDesign as { pieces?: { name?: string; plane?: unknown; center?: unknown }[] } | undefined)?.pieces ?? []).map((p) => ({
    name: p.name,
    plane: p.plane,
    center: p.center,
  })) ?? [];
//#endregion 🗃️Exports
