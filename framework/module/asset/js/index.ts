// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Barrel export for all asset modules including icons, fonts, representations and images.

// #endregion 🧲Header

//#region 🗃️Exports
// Builtin Metabolism kit and UI icons only — test JSON lives in `@semio-tech/compose-fixture`.

import metabolismShallowKit from "../../../../compose/fixture/metabolism.shallow.kit.compose.json";
import { createIconComponent } from "@semio-tech/ui-react";

const MetabolismKitData = { wip: { initialKit: metabolismShallowKit } };

//#region 🔖Icons
export const AddIcon = createIconComponent("plus");
export const AlertCircleIcon = createIconComponent("alert-circle");
export const ArrowLeftIcon = createIconComponent("arrow-left");
export const AwardIcon = createIconComponent("award");
export const BookIcon = createIconComponent("book-open");
export const CameraIcon = createIconComponent("camera");
export const ChatIcon = createIconComponent("message-circle");
export const CheckIcon = createIconComponent("check");
export const CheckIconAlt = CheckIcon;
export const ChevronDownIcon = createIconComponent("chevron-down");
export const ChevronDownIconAlt = ChevronDownIcon;
export const ChevronLeftIcon = createIconComponent("chevron-left");
export const ChevronRightIcon = createIconComponent("chevron-right");
export const ChevronsUpDownIcon = createIconComponent("chevrons-up-down");
export const CloseIcon = createIconComponent("x");
export const CloseIconAlt = CloseIcon;
export const CodeIcon = createIconComponent("code");
export const ConnectionIcon = createIconComponent("network");
export const ConnectorIcon = createIconComponent("crosshair");
export const CopyIcon = createIconComponent("copy");
export const DetailsIcon = createIconComponent("info");
export const DiagramIcon = createIconComponent("grid-3x3");
export const DisconnectIcon = createIconComponent("link-2-off");
export const DocumentIcon = createIconComponent("file-text");
export const ExternalLinkIcon = createIconComponent("external-link");
export const FileArchiveIcon = createIconComponent("file-archive");
export const FileCodeIcon = createIconComponent("file-code");
export const FileImageIcon = createIconComponent("file-image");
export const FileJsonIcon = createIconComponent("file-json");
export const FileSpreadsheetIcon = createIconComponent("file-spreadsheet");
export const FileTypeIcon = createIconComponent("file-type");
export const FileVideoIcon = createIconComponent("file-video");
export const FocusIcon = createIconComponent("focus");
export const FolderIcon = createIconComponent("folder");
export const GlobeIcon = createIconComponent("globe");
export const GripVerticalIcon = createIconComponent("grip-vertical");
export const HandIcon = createIconComponent("hand");
export const HashIcon = createIconComponent("hash");
export const HomeIcon = createIconComponent("home");
export const HudIcon = createIconComponent("hud-overlay");
export const HudPanelIcon = createIconComponent("panel-top");
export const InfoIcon = createIconComponent("info");
export const IntersectIcon = createIconComponent("combine");
export const LayoutIcon = createIconComponent("layout");
export const LeftSidePanelIcon = createIconComponent("panel-left");
export const LightbulbIcon = createIconComponent("lightbulb");
export const LoaderIcon = createIconComponent("loader-2");
export const LocalKitIcon = createIconComponent("hard-drive");
export const Maximize2Icon = createIconComponent("maximize-2");
export const MessageCircle = createIconComponent("message-circle");
export const Minimize2Icon = createIconComponent("minimize-2");
export const MonitorIcon = createIconComponent("monitor");
export const MoonIcon = createIconComponent("moon");
export const MoreHorizontalIcon = createIconComponent("more-horizontal");
export const MousePointerIcon = createIconComponent("mouse-pointer");
export const NavigateBackIcon = createIconComponent("arrow-left");
export const NavigateForwardIcon = createIconComponent("arrow-right");
export const NavigateUpIcon = createIconComponent("arrow-up");
export const PauseIcon = createIconComponent("pause");
export const PieceIcon = createIconComponent("puzzle");
export const PlayIcon = createIconComponent("play");
export const PortIcon = createIconComponent("plug");
export const RecordIcon = createIconComponent("circle");
export const RemoteKitIcon = createIconComponent("cloud");
export const RemoveIcon = createIconComponent("minus");
export const ResetIcon = createIconComponent("rotate-ccw");
export const RightSidePanelIcon = createIconComponent("panel-right");
export const SceneIcon = createIconComponent("scene-3d");
export const SearchIcon = createIconComponent("search");
export const SelectUtilityIcon = createIconComponent("mouse-pointer-2");
export const SettingsIcon = createIconComponent("settings");
export const SkipBackIcon = createIconComponent("skip-back");
export const SkipForwardIcon = createIconComponent("skip-forward");
export const SmartphoneIcon = createIconComponent("smartphone");
export const SortAscendingIcon = createIconComponent("arrow-up");
export const SortDescendingIcon = createIconComponent("arrow-down");
export const StatsIcon = createIconComponent("bar-chart-3");
export const StopIcon = createIconComponent("square");
export const SunIcon = createIconComponent("sun");
export const TabletIcon = createIconComponent("tablet");
export const TableViewIcon = createIconComponent("table-2");
export const TemporaryKitIcon = createIconComponent("clock");
export const TriangleAlertIcon = createIconComponent("triangle-alert");
export const TutorialIcon = createIconComponent("graduation-cap");
export const TypeIcon = createIconComponent("typography");
export const UserIcon = createIconComponent("user");
export const UtilitiesIcon = createIconComponent("wrench");
export const UtilityBarIcon = createIconComponent("hammer");
export const WorkbenchIcon = createIconComponent("workbench");
export type { IconName } from "@semio-tech/ui-asset";
export { isMetabolismIconName, METABOLISM_ICONS, METABOLISM_ICON_NAMES, type MetabolismIconName } from "../metabolism/icon/generated/metabolism_icons.ts";
export { resolveMetabolismIconNameFromTheme, resolveMetabolismIconSvgFromTheme } from "./icon_resolver.ts";
//#endregion 🔖Icons

//#region 🔖KitBootstrapHelpers
/** @emoji 📎 Reads a materialized `{ hash, items }` collection without importing Node-only repository tooling into browser bundles. */
function __fixtureItemsOf<T = Record<string, unknown>>(node: unknown): readonly T[] {
  return node && typeof node === "object" && Array.isArray((node as { items?: unknown }).items) ? (node as { items: T[] }).items : [];
}

/** @emoji 🧾 Resolves the materialized kit payload from `wip.initialKit`. */
function __metabolismKitInner(): Record<string, unknown> {
  const root = MetabolismKitData as { wip?: { initialKit?: Record<string, unknown> } };
  const inner = root.wip?.initialKit;
  return (inner && typeof inner === "object" ? inner : {}) ?? {};
}

/** @emoji 🏛️ Flattens kinds from root `types` or nested `typologies[].types`. */
function __kitTypesFromInner(inner: Record<string, unknown>): readonly unknown[] {
  const rootTypes = __fixtureItemsOf(inner["types"]);
  if (rootTypes.length > 0) return rootTypes;
  return __fixtureItemsOf(inner["typologies"]).flatMap((topo) => __fixtureItemsOf((topo as { types?: unknown }).types));
}

/** @emoji 🏛️ Flattens designs from root `designs` or nested `typologies[].designs`. */
function __kitDesignsFromInner(inner: Record<string, unknown>): readonly unknown[] {
  const rootDesigns = __fixtureItemsOf(inner["designs"]);
  if (rootDesigns.length > 0) return rootDesigns;
  return __fixtureItemsOf(inner["typologies"]).flatMap((topo) => __fixtureItemsOf((topo as { designs?: unknown }).designs));
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
export const MetabolismKitTypologies = __fixtureItemsOf(__metabolismKitInner()["typologies"]);
/**
 * Metabolism kit families array
 **/
export const MetabolismKitFamilies = __fixtureItemsOf(__metabolismKitInner()["families"]);
/**
 * Metabolism kit qualities array
 **/
export const MetabolismKitQualities = __fixtureItemsOf(__metabolismKitInner()["qualities"]);
/**
 * Metabolism kit files array
 **/
export const MetabolismKitFiles = __fixtureItemsOf(__metabolismKitInner()["files"]);
/**
 * Metabolism kit folders array
 **/
export const MetabolismKitFolders = __fixtureItemsOf(__metabolismKitInner()["folders"]);
/**
 * Metabolism kit authors array
 **/
export const MetabolismKitAuthors = __fixtureItemsOf(__metabolismKitInner()["authors"]);
/**
 * Metabolism kit tags array
 **/
export const MetabolismKitTags = __fixtureItemsOf(__metabolismKitInner()["tags"]);
/**
 * Metabolism kit concepts array
 **/
export const MetabolismKitConcepts = __fixtureItemsOf(__metabolismKitInner()["concepts"]);
/**
 * Metabolism kit attributes array
 **/
export const MetabolismKitAttributes = __fixtureItemsOf(__metabolismKitInner()["attributes"]);
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
