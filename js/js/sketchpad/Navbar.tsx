// #region Header

// Navbar.tsx

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

import Fuse, { FuseResult } from "fuse.js";
import { ArrowLeft, ArrowRight, ArrowUp, Award, Box, ChevronDown, ChevronUp, Clock, Cloud, FileText, Focus as FocusIcon, Fullscreen, HardDrive, Home, Layout, Minus, Search as SearchIcon, Square, User, X } from "lucide-react";
import { createContext, FC, Fragment, ReactNode, useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useLocation, useNavigate, useParams, useSearchParams } from "react-router";
import { CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "../elements/Command";
import { ButtonGroup, ButtonGroupItem } from "../elements/input/ButtonGroup";
import { Toggle } from "../elements/input/Toggle";
import { ToggleGroup, ToggleGroupItem } from "../elements/input/ToggleGroup";
import { Breadcrumb, BreadcrumbBreak, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "../elements/navigation/Breadcrumb";
import { Author, AuthorDiff, Connection, Design, DesignDiff, DesignShallow, FileDiff, generateUniqueName, Guid, KitShallow, Piece, Quality, File as SemioFile, Type, TypeDiff, TypeShallow } from "../semio";
import "./apps";
import { appRegistry } from "./apps";
import { useDesignAppCommands } from "./apps/design/store";
import { docsRegistry } from "./apps/docs/registry";
import { useHomeCommands } from "./apps/home/store";
import { useKitAppCommands } from "./apps/kit/store";
import { useQualityAppCommands } from "./apps/quality/store";
import { useTypeAppCommands } from "./apps/type/store";
import {
  PanelVisibility,
  SketchpadScope,
  useAppCommands,
  useAppPanelVisibility,
  useAppType,
  useIsFullscreen,
  useIsMobile,
  useIsNavbarExpanded,
  useKits,
  useMode,
  useNavigation,
  useNavigationHistory,
  useSketchpad,
  useSketchpadCommands,
  useSketchpadScope,
  useSketchpadStore,
  useTooltip,
  WindowEvents,
} from "./store";

export interface PanelSection {
  id: string;
  label: string;
  content: ReactNode | (() => ReactNode);
  defaultOpen?: boolean;
  order?: number;
  actions?: Array<{
    icon: ReactNode;
    onClick: () => void;
    title: string;
  }>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: () => void;
}

export type PanelKey = "details" | "workbench" | "tools" | "hud" | "stats" | "console" | "chat" | "settings" | "toolbar";

export interface PanelSections {
  details: PanelSection[];
  workbench: PanelSection[];
  tools: PanelSection[];
  hud: PanelSection[];
  stats: PanelSection[];
  console: PanelSection[];
  chat: PanelSection[];
  settings: PanelSection[];
  toolbar: PanelSection[];
}

export interface FocusItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
}

interface FocusContextValue {
  focusItems: FocusItem[];
  setFocusItems: (items: FocusItem[]) => void;
  setOnFocusItem: (callback: ((itemId: string) => void) | undefined) => void;
  triggerFocusItem: (itemId: string) => void;
}

const FocusContext = createContext<FocusContextValue | null>(null);

export const FocusProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [focusItems, setFocusItems] = useState<FocusItem[]>([]);
  const onFocusItemCallbackRef = useRef<((itemId: string) => void) | undefined>(undefined);

  const setFocusItemsStable = useCallback((items: FocusItem[]) => {
    setFocusItems(items);
  }, []);

  const setOnFocusItem = useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFocusItemCallbackRef.current = callback;
  }, []);

  const triggerFocusItem = useCallback((itemId: string) => {
    if (onFocusItemCallbackRef.current) {
      onFocusItemCallbackRef.current(itemId);
    }
  }, []);

  // Separate the stable functions from the changing state
  // This prevents unnecessary re-renders of components that only use the functions
  const contextValue = useMemo(
    () => ({ focusItems, setFocusItems: setFocusItemsStable, setOnFocusItem, triggerFocusItem }),
    // Only include focusItems, as the functions are already stable
    [focusItems],
  );

  return <FocusContext.Provider value={contextValue}>{children}</FocusContext.Provider>;
};

export const useFocus = () => {
  const context = useContext(FocusContext);
  if (!context) throw new Error("useFocus must be used within FocusProvider");
  return context;
};

export const useFocusSafe = () => {
  const context = useContext(FocusContext);
  return context;
};

interface PanelSectionContextValue {
  sections: PanelSections;
  addSection: (panelKey: PanelKey, section: PanelSection) => void;
  removeSection: (panelKey: PanelKey, sectionId: string) => void;
}

const PanelSectionContext = createContext<PanelSectionContextValue | null>(null);

export const PanelSectionProvider: FC<{ children: ReactNode }> = ({ children }) => {
  const [sections, setSections] = useState<PanelSections>({
    details: [],
    workbench: [],
    tools: [],
    hud: [],
    stats: [],
    console: [],
    chat: [],
    settings: [],
    toolbar: [],
  });

  const addSection = useCallback((panelKey: PanelKey, section: PanelSection) => {
    setSections((prev) => {
      const updated = {
        ...prev,
        [panelKey]: [...prev[panelKey].filter((s) => s.id !== section.id), section].sort((a, b) => (a.order || 0) - (b.order || 0)),
      };
      return updated;
    });
  }, []);

  const removeSection = useCallback((panelKey: PanelKey, sectionId: string) => {
    setSections((prev) => ({ ...prev, [panelKey]: prev[panelKey].filter((s) => s.id !== sectionId) }));
  }, []);

  return <PanelSectionContext.Provider value={{ sections, addSection, removeSection }}>{children}</PanelSectionContext.Provider>;
};

export const usePanelSections = (panelKey: PanelKey): PanelSection[] => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("usePanelSections must be used within PanelSectionProvider");
  return context.sections[panelKey];
};

export const useAddPanelSection = () => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("useAddPanelSection must be used within PanelSectionProvider");
  return context.addSection;
};

export const useRemovePanelSection = () => {
  const context = useContext(PanelSectionContext);
  if (!context) throw new Error("useRemovePanelSection must be used within PanelSectionProvider");
  return context.removeSection;
};

export interface PanelDefinition {
  key: string;
  icon: React.ComponentType<{ size?: number }>;
  i18n: import("../elements/display/Tooltip").TooltipConfig;
  hotkey: string;
}

export const getPanelConfigs = (t: (key: string) => string): Record<string, PanelDefinition[]> => appRegistry.getPanelConfigs(t);

interface NavigationProps {
  mobile?: boolean;
}

const Navigation: FC<NavigationProps> = ({ mobile = false }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams] = useSearchParams();
  const kits = useKits();

  const mode = useMode();
  const isMobile = useIsMobile();
  const isNavbarExpanded = useIsNavbarExpanded();

  const pathParts = navigation.split("/").filter((p) => p);
  const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
  const isKitsPath = pathParts[0] === "kits";
  const isDocsPath = pathParts[0] === "docs";

  const homeKind = !isKitsPath || pathParts.length === 1 ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !isKitsPath || pathParts.length === 1 ? searchParams.get("name") : null;
  const homeVersion = !isKitsPath || pathParts.length === 1 ? searchParams.get("version") : null;

  const docsSection = isDocsPath && pathParts[1] ? pathParts[1] : null;
  const docsPagePath = isDocsPath && pathParts.length > 1 ? pathParts.slice(1).join("/") : null;
  const docsSectionsList = docsRegistry.getAllSections();

  const kitGuid = isKitsPath && pathParts[1] ? pathParts[1] : null;

  const secondPart = pathParts[2];
  const thirdPart = pathParts[3];
  const isDesignApp = isKitsPath && secondPart === "designs" && thirdPart && isUuidPattern(thirdPart);
  const isTypeApp = isKitsPath && secondPart === "types" && thirdPart && isUuidPattern(thirdPart);
  const isQualityApp = isKitsPath && secondPart === "qualities" && thirdPart && isUuidPattern(thirdPart);
  const itemGuid = isDesignApp || isTypeApp || isQualityApp ? thirdPart : null;

  const filteredKind = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? (searchParams.get("kind") as "designs" | "types" | "qualities" | "files" | "authors" | null) : null;
  const filteredName = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? searchParams.get("name") : null;
  const filteredVariant = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? searchParams.get("variant") : null;
  const filteredView = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp ? searchParams.get("view") : null;

  const isKitApp = kitGuid && !isDesignApp && !isTypeApp && !isQualityApp;

  const kit = kits.find((k) => k.guid === kitGuid);
  const store = useSketchpadStore();

  const kitKind = useMemo(() => {
    if (!kitGuid || !store.hasKit(kitGuid)) return undefined;
    const kitStore = store.kit(kitGuid);
    if (!kitStore) return undefined;
    if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) return "remote";
    if (kitStore.isLocallyPersisted) return "local";
    return "temporary";
  }, [kitGuid, store]);

  const kitKindItems = [
    { label: <Clock size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.temporary", href: "/?kind=temporary" },
    { label: <HardDrive size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.local", href: "/?kind=local" },
    { label: <Cloud size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.remote", href: "/?kind=remote" },
  ];

  const kitItemsWithCreate = useMemo(() => {
    const items = kits
      .filter((k) => {
        if (!kitKind) return true;
        const ks = store.kit(k.guid);
        const kKind = ks.isLocallyPersisted && ks.isRemotelySynced ? "remote" : ks.isLocallyPersisted ? "local" : "temporary";
        return kKind === kitKind;
      })
      .map((k) => ({ label: k.name, href: `/kits/${k.guid}` }));

    items.push({ label: "+ " + t("semio.sketchpad.navbar.createKit"), href: "#create-kit" });
    return items;
  }, [kits, kitKind, store, t]);

  const sketchpadCommands = useSketchpadCommands();

  const kitCommands = useMemo(() => {
    if (!kitGuid || !store.hasKit(kitGuid)) return null;
    const kitStore = store.kit(kitGuid);
    return {
      importKit: (url: string) => kitStore.execute("semio.kit.import", url),
      exportKit: () => kitStore.execute("semio.kit.export"),
      createAuthor: (author: Author) => kitStore.execute("semio.kit.createAuthor", author),
      updateAuthor: (authorId: string, authorDiff: AuthorDiff) => kitStore.execute("semio.kit.updateAuthor", authorId, authorDiff),
      deleteAuthor: (authorId: string) => kitStore.execute("semio.kit.deleteAuthor", authorId),
      createType: (type: Type) => kitStore.execute("semio.kit.createType", type),
      updateType: (guid: Guid, diff: TypeDiff) => kitStore.execute("semio.kit.updateType", guid, diff),
      deleteType: (guid: Guid) => kitStore.execute("semio.kit.deleteType", guid),
      createDesign: (design: Design) => kitStore.execute("semio.kit.createDesign", design),
      updateDesign: (guid: Guid, diff: DesignDiff) => kitStore.execute("semio.kit.updateDesign", guid, diff),
      deleteDesign: (guid: Guid) => kitStore.execute("semio.kit.deleteDesign", guid),
      addFile: (file: SemioFile, blob?: Blob) => kitStore.execute("semio.kit.addFile", file, blob),
      updateFile: (url: string, fileDiff: FileDiff, blob?: Blob) => kitStore.execute("semio.kit.updateFile", url, fileDiff, blob),
      removeFile: (url: string) => kitStore.execute("semio.kit.removeFile", url),
      addPiece: (design: Guid, piece: Piece) => kitStore.execute("semio.kit.addPiece", design, piece),
      addPieces: (design: Guid, pieces: Piece[]) => kitStore.execute("semio.kit.addPieces", design, pieces),
      removePiece: (design: Guid, piece: Guid) => kitStore.execute("semio.kit.removePiece", design, piece),
      removePieces: (design: Guid, pieces: Guid[]) => kitStore.execute("semio.kit.removePieces", design, pieces),
      addConnection: (design: Guid, connection: Connection) => kitStore.execute("semio.kit.addConnection", design, connection),
      addConnections: (design: Guid, connections: Connection[]) => kitStore.execute("semio.kit.addConnections", design, connections),
      removeConnection: (design: Guid, connection: Guid) => kitStore.execute("semio.kit.removeConnection", design, connection),
      removeConnections: (design: Guid, connections: Guid[]) => kitStore.execute("semio.kit.removeConnections", design, connections),
      deleteSelected: (design: Guid, selectedPieces: Guid[], selectedConnections: Guid[]) => kitStore.execute("semio.kit.deleteSelected", design, selectedPieces, selectedConnections),
    };
  }, [kitGuid, store]);

  const artifactKinds = [
    { label: <Layout size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.designs", kind: "designs", href: kitGuid ? `/kits/${kitGuid}?kind=designs` : "/kits?kind=designs" },
    { label: <Box size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.types", kind: "types", href: kitGuid ? `/kits/${kitGuid}?kind=types` : "/kits?kind=types" },
    { label: <Award size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.qualities", kind: "qualities", href: kitGuid ? `/kits/${kitGuid}?kind=qualities` : "/kits?kind=qualities" },
    { label: <FileText size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.files", kind: "files", href: kitGuid ? `/kits/${kitGuid}?kind=files` : "/kits?kind=files" },
    { label: <User size={16} />, i18n: "semio.sketchpad.navbar.breadcrumb.authors", kind: "authors", href: kitGuid ? `/kits/${kitGuid}?kind=authors` : "/kits?kind=authors" },
  ];

  const allDesigns: Design[] = useMemo(() => {
    if (!kit?.designs) return [];
    return (kit.designs as any[]).filter((d): d is Design => typeof d === "object" && d.guid !== undefined);
  }, [kit?.designs]);

  const allTypes: Type[] = useMemo(() => {
    if (!kit?.types) return [];
    return (kit.types as any[]).filter((t): t is Type => typeof t === "object" && t.guid !== undefined);
  }, [kit?.types]);

  const allQualities: Quality[] = useMemo(() => {
    if (!kit?.qualities) return [];
    return (kit.qualities as any[]).filter((q): q is Quality => typeof q === "object" && q.guid !== undefined);
  }, [kit?.qualities]);

  const handleCreateKit = useCallback(() => {
    const guid = crypto.randomUUID();
    const now = new Date();
    const existingNames = kits.map((k) => k.name);
    const uniqueName = generateUniqueName(t("semio.sketchpad.app.kit.defaultName"), existingNames);
    sketchpadCommands.createKit({
      guid,
      name: uniqueName,
      version: "",
      createdAt: now,
      updatedAt: now,
    });
    navigate(`/kits/${guid}`);
  }, [navigate, sketchpadCommands, kits, t]);

  const handleCreateVersion = useCallback(() => {
    if (!kit) return;
    const newGuid = crypto.randomUUID();
    const now = new Date();
    const existingVersions = kits.filter((k) => k.name === kit.name).map((k) => k.version || "");
    const uniqueVersion = generateUniqueName(t("semio.sketchpad.app.kit.newVersion"), existingVersions);
    sketchpadCommands.createKit({
      guid: newGuid,
      name: kit.name,
      version: uniqueVersion,
      createdAt: now,
      updatedAt: now,
    });
    navigate(`/kits/${newGuid}`);
  }, [kit, kits, navigate, sketchpadCommands]);

  const handleCreateDesign = useCallback(
    (name?: string) => {
      if (!kitCommands) return;
      const guid = crypto.randomUUID();
      const existingNames = allDesigns.map((d) => d.name);
      const uniqueName = name || generateUniqueName(t("semio.sketchpad.app.design.defaultName"), existingNames);
      kitCommands.createDesign({ guid, name: uniqueName, variant: "", view: "", pieces: [], connections: [] });
      navigate(`/kits/${kitGuid}/designs/${guid}`);
    },
    [kitCommands, kitGuid, navigate, allDesigns, t],
  );

  const handleCreateType = useCallback(
    (name?: string) => {
      if (!kitCommands) return;
      const guid = crypto.randomUUID();
      const existingNames = allTypes.map((t) => t.name);
      const uniqueName = name || generateUniqueName(t("semio.sketchpad.app.type.defaultName"), existingNames);
      kitCommands.createType({ guid, name: uniqueName, variant: "", ports: [] });
      navigate(`/kits/${kitGuid}/types/${guid}`);
    },
    [kitCommands, kitGuid, navigate, allTypes, t],
  );

  const handleCreateVariant = useCallback(
    (designOrType: Design | Type, isType: boolean) => {
      if (!kitCommands) return;
      const guid = crypto.randomUUID();
      if (!isType) {
        const d = designOrType as Design;
        const existingVariants = allDesigns.filter((design) => design.name === d.name).map((design) => design.variant || "");
        const uniqueVariant = generateUniqueName(t("semio.sketchpad.app.design.newVariant"), existingVariants);
        kitCommands.createDesign({
          guid,
          name: d.name,
          variant: uniqueVariant,
          view: "",
          pieces: [],
          connections: [],
        });
        navigate(`/kits/${kitGuid}/designs/${guid}`);
      } else {
        const typeObj = designOrType as Type;
        const existingVariants = allTypes.filter((type) => type.name === typeObj.name).map((type) => type.variant || "");
        const uniqueVariant = generateUniqueName(t("semio.sketchpad.app.type.newVariant"), existingVariants);
        kitCommands.createType({
          guid,
          name: typeObj.name,
          variant: uniqueVariant,
          ports: [],
        });
        navigate(`/kits/${kitGuid}/types/${guid}`);
      }
    },
    [kitCommands, kitGuid, navigate, allDesigns, allTypes],
  );

  const handleCreateView = useCallback(
    (design: Design) => {
      if (!kitCommands) return;
      const guid = crypto.randomUUID();
      const existingViews = allDesigns.filter((d) => d.name === design.name && (d.variant || "") === (design.variant || "")).map((d) => d.view || "");
      const uniqueView = generateUniqueName(t("semio.sketchpad.app.design.newView"), existingViews);
      kitCommands.createDesign({
        guid,
        name: design.name,
        variant: design.variant,
        view: uniqueView,
        pieces: [],
        connections: [],
      });
      navigate(`/kits/${kitGuid}/designs/${guid}`);
    },
    [kitCommands, kitGuid, navigate, allDesigns],
  );

  const handleCreate = useCallback(() => {
    if (!kit || !filteredKind || !kitCommands) return;

    switch (filteredKind) {
      case "designs":
        handleCreateDesign();
        break;
      case "types":
        handleCreateType();
        break;
      case "authors":
        const guid = crypto.randomUUID();
        kitCommands.createAuthor({ guid, name: "New Author", email: "" });
        break;
      case "qualities":
        // TODO: Add createQuality command
        break;
      case "files":
        // TODO: Add createFile command
        break;
    }
  }, [kit, filteredKind, kitCommands, handleCreateDesign, handleCreateType]);

  // Find current design or type or quality
  const design = isDesignApp ? allDesigns.find((d) => d.guid === itemGuid) : undefined;
  const type = isTypeApp ? allTypes.find((t) => t.guid === itemGuid) : undefined;
  const quality = isQualityApp ? allQualities.find((q) => q.guid === itemGuid) : undefined;

  // Build breadcrumb items for designs
  const designNameItems = useMemo(() => {
    const nameMap = new Map<string, Design>();
    allDesigns.forEach((d) => {
      if (!nameMap.has(d.name)) nameMap.set(d.name, d);
    });
    const items = Array.from(nameMap.entries()).map(([name, d]) => ({
      label: name,
      href: `/kits/${kitGuid}/designs/${d.guid}`,
    }));
    items.push({ label: "+ " + t("semio.sketchpad.navbar.createDesign"), href: "#create-design" });
    return items;
  }, [allDesigns, kitGuid, t]);

  const designVariantItems = useMemo(() => {
    if (!design) return [];
    const variants = new Map<string, Design>();
    allDesigns.forEach((d) => {
      if (d.name === design.name) {
        const key = d.variant || "";
        if (!variants.has(key)) variants.set(key, d);
      }
    });
    const items = Array.from(variants.entries()).map(([variant, d]) => ({
      label: variant || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultVariant")}</span>,
      href: `/kits/${kitGuid}/designs/${d.guid}`,
    }));
    items.push({ label: "+ " + t("semio.sketchpad.navbar.createVariant"), href: "#create-variant" });
    return items;
  }, [design, allDesigns, kitGuid, t]);

  const designViewItems = useMemo(() => {
    if (!design) return [];
    const items = allDesigns
      .filter((d) => d.name === design.name && (d.variant || "") === (design.variant || ""))
      .map((d) => ({
        label: d.view || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultView")}</span>,
        href: `/kits/${kitGuid}/designs/${d.guid}`,
      }));
    items.push({ label: "+ " + t("semio.sketchpad.navbar.createView"), href: "#create-view" });
    return items;
  }, [design, allDesigns, kitGuid, t]);

  // Build breadcrumb items for types
  const typeNameItems = useMemo(() => {
    const nameMap = new Map<string, Type>();
    allTypes.forEach((t) => {
      if (!nameMap.has(t.name)) nameMap.set(t.name, t);
    });
    const items = Array.from(nameMap.entries()).map(([name, t]) => ({
      label: name,
      href: `/kits/${kitGuid}/types/${t.guid}`,
    }));
    items.push({ label: "+ " + t("semio.sketchpad.navbar.createType"), href: "#create-type" });
    return items;
  }, [allTypes, kitGuid, t]);

  const typeVariantItems = useMemo(() => {
    if (!type) return [];
    const variants = new Map<string, Type>();
    allTypes.forEach((t) => {
      if (t.name === type.name) {
        const key = t.variant || "";
        if (!variants.has(key)) variants.set(key, t);
      }
    });
    const items = Array.from(variants.entries()).map(([variant, typeObj]) => ({
      label: variant || <span className="italic opacity-70">{t("semio.sketchpad.app.type.defaultVariant")}</span>,
      href: `/kits/${kitGuid}/types/${typeObj.guid}`,
    }));
    items.push({ label: "+ " + t("semio.sketchpad.navbar.createVariant"), href: "#create-variant" });
    return items;
  }, [type, allTypes, kitGuid, t]);

  // Build breadcrumb items for kit versions
  const kitVersionItems = useMemo(() => {
    if (!kit?.name) return [];
    const sameNameKits = kits.filter((k) => k.name === kit.name);
    const items = sameNameKits.map((k) => ({
      label: k.version || <span className="italic opacity-70">{t("semio.sketchpad.app.kit.defaultVersion")}</span>,
      href: `/kits/${k.guid}`,
    }));
    items.push({ label: "+ " + t("semio.sketchpad.navbar.createVersion"), href: "#create-version" });
    return items;
  }, [kit, kits, t]);

  // Build breadcrumb items for home page kits filtered by kind
  const homeKitsForKind = useMemo(() => {
    if (!homeKind) return [];
    return kits
      .filter((k) => {
        const ks = store.kit(k.guid);
        const kKind = ks.isLocallyPersisted && ks.isRemotelySynced ? "remote" : ks.isLocallyPersisted ? "local" : "temporary";
        return kKind === homeKind;
      })
      .map((k) => ({
        label: k.name,
        href: `/?kind=${homeKind}&name=${encodeURIComponent(k.name)}`,
      }));
  }, [homeKind, kits, store]);

  // Build breadcrumb items for home page versions filtered by name
  const homeVersionsForName = useMemo(() => {
    if (!homeName || !homeKind) return [];
    return kits
      .filter((k) => {
        if (k.name !== homeName) return false;
        const ks = store.kit(k.guid);
        const kKind = ks.isLocallyPersisted && ks.isRemotelySynced ? "remote" : ks.isLocallyPersisted ? "local" : "temporary";
        return kKind === homeKind;
      })
      .map((k) => ({
        label: k.version || <span className="italic opacity-70">{t("semio.sketchpad.app.kit.defaultVersion")}</span>,
        href: `/kits/${k.guid}`,
      }));
  }, [homeName, homeKind, kits, store, t]);

  // Build breadcrumb items for filtered names in kit app
  const filteredNameItems = useMemo(() => {
    if (!kit || !filteredKind) return [];
    const nameSet = new Set<string>();

    if (filteredKind === "designs") {
      allDesigns.forEach((d) => nameSet.add(d.name));
    } else if (filteredKind === "types") {
      allTypes.forEach((t) => nameSet.add(t.name));
    }

    return Array.from(nameSet).map((name) => ({
      label: name,
      href: `/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(name)}`,
    }));
  }, [kit, filteredKind, allDesigns, allTypes, kitGuid]);

  // Build breadcrumb items for filtered variants in kit app
  const filteredVariantItems = useMemo(() => {
    if (!kit || !filteredKind || !filteredName) return [];
    const variantSet = new Set<string>();

    if (filteredKind === "designs") {
      allDesigns.forEach((d) => {
        if (d.name === filteredName) {
          variantSet.add(d.variant || "");
        }
      });
    } else if (filteredKind === "types") {
      allTypes.forEach((t) => {
        if (t.name === filteredName) {
          variantSet.add(t.variant || "");
        }
      });
    }

    return Array.from(variantSet).map((variant) => ({
      label: variant || <span className="italic opacity-70">{filteredKind === "designs" ? t("semio.sketchpad.app.design.defaultVariant") : t("semio.sketchpad.app.type.defaultVariant")}</span>,
      href: `/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(variant)}`,
    }));
  }, [kit, filteredKind, filteredName, allDesigns, allTypes, kitGuid, t]);

  // Build breadcrumb items for filtered views in kit app
  const filteredViewItems = useMemo(() => {
    if (!kit || filteredKind !== "designs" || !filteredName || filteredVariant === null) return [];
    const viewSet = new Set<string>();

    allDesigns.forEach((d) => {
      if (d.name === filteredName && (d.variant || "") === filteredVariant) {
        viewSet.add(d.view || "");
      }
    });

    return Array.from(viewSet).map((view) => ({
      label: view || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultView")}</span>,
      href: `/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(filteredVariant)}&view=${encodeURIComponent(view)}`,
    }));
  }, [kit, filteredKind, filteredName, filteredVariant, allDesigns, kitGuid, t]);

  // Determine if we're at root or if a kind filter is active
  const isAtRoot = navigation === "/";
  const hasKindFilter = filteredKind || (kitGuid && kitKind);

  return (
    <Breadcrumb className="flex-1 min-w-0">
      <BreadcrumbList>
        {/* Always show Home icon with dropdown to select kinds */}
        <BreadcrumbItem i18n="semio.sketchpad.navbar.home">
          <BreadcrumbLink onClick={() => navigate("/")} style={{ cursor: "pointer" }}>
            <Home size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>

        {/* Show separator with kind selector dropdown */}
        <BreadcrumbSeparator items={kitKindItems} i18n="semio.sketchpad.navbar.kitKinds" onNavigate={(href) => navigate(href)} />

        {/* If viewing a kit (or we have a selected home kind), show the kind breadcrumb */}
        {(kitGuid && kitKind) || homeKind ? (
          <>
            <BreadcrumbItem i18n={"semio.sketchpad.navbar.breadcrumb.${kitKind || homeKind}"}>
              <BreadcrumbLink onClick={() => navigate(`/?kind=${kitKind || homeKind}`)} style={{ cursor: "pointer" }}>
                {(kitKind === "temporary" || homeKind === "temporary") && <Clock size={16} />}
                {(kitKind === "local" || homeKind === "local") && <HardDrive size={16} />}
                {(kitKind === "remote" || homeKind === "remote") && <Cloud size={16} />}
              </BreadcrumbLink>
            </BreadcrumbItem>

            {/* Show kits dropdown when on home page with kind selected */}
            {!kitGuid && <BreadcrumbSeparator items={homeKitsForKind} i18n="semio.sketchpad.navbar.kits" onNavigate={(href) => navigate(href)} />}

            {homeName && (
              <>
                <BreadcrumbItem i18n="semio.sketchpad.navbar.kitName">
                  <BreadcrumbLink onClick={() => navigate(`/?kind=${homeKind}&name=${encodeURIComponent(homeName)}`)} style={{ cursor: "pointer" }}>
                    {homeName}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator items={homeVersionsForName} i18n="semio.sketchpad.navbar.versions" onNavigate={(href) => navigate(href)} />
                {homeVersion !== null && (
                  <BreadcrumbItem i18n="semio.sketchpad.navbar.kitVersion">
                    <BreadcrumbLink style={{ cursor: "default" }}>{homeVersion || <span className="italic opacity-70">{t("semio.sketchpad.app.kit.defaultVersion")}</span>}</BreadcrumbLink>
                  </BreadcrumbItem>
                )}
              </>
            )}
            {kitGuid && (
              <>
                <BreadcrumbSeparator
                  items={kitItemsWithCreate}
                  i18n="semio.sketchpad.navbar.kits"
                  onNavigate={(href) => {
                    if (href === "#create-kit") handleCreateKit();
                    else navigate(href);
                  }}
                />
                <BreadcrumbItem>
                  <BreadcrumbLink
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      navigate(`/?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}`);
                    }}
                    style={{ cursor: "pointer" }}
                    i18n={"semio.sketchpad.navbar.kit"}
                  >
                    {kit?.name || kitGuid}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator
                  items={kitVersionItems}
                  i18n="semio.sketchpad.navbar.versions"
                  onNavigate={(href) => {
                    if (href === "#create-version") handleCreateVersion();
                    else navigate(href);
                  }}
                />
                <BreadcrumbItem>
                  <BreadcrumbLink
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      const versionParam = kit?.version !== undefined ? `&version=${encodeURIComponent(kit.version)}` : "";
                      navigate(`/?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}${versionParam}`);
                    }}
                    style={{ cursor: "pointer" }}
                    i18n={"semio.sketchpad.navbar.kitVersion"}
                  >
                    {kit?.version || <span className="italic opacity-70">{t("semio.sketchpad.app.kit.defaultVersion")}</span>}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </>
            )}
          </>
        ) : null}
        {isKitApp && (
          <>
            <BreadcrumbBreak />
            <BreadcrumbSeparator items={artifactKinds} i18n="semio.sketchpad.navbar.artifacts" onNavigate={(href) => navigate(href)} />
            {filteredKind && (
              <>
                <BreadcrumbItem i18n={"semio.sketchpad.navbar.breadcrumb.${filteredKind}"}>
                  <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}`)} style={{ cursor: "pointer" }}>
                    {filteredKind === "designs" && <Layout size={16} />}
                    {filteredKind === "types" && <Box size={16} />}
                    {filteredKind === "qualities" && <Award size={16} />}
                    {filteredKind === "files" && <FileText size={16} />}
                    {filteredKind === "authors" && <User size={16} />}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbBreak />
                <BreadcrumbSeparator items={filteredNameItems} i18n="semio.sketchpad.navbar.selectName" onNavigate={(href) => navigate(href)} />
                {filteredName !== null && (
                  <>
                    <BreadcrumbItem>
                      <BreadcrumbLink
                        onClick={() => {
                          const firstMatchingDesign = (kit?.designs as any[])?.find((d: any) => d.name === filteredName);
                          if (firstMatchingDesign) {
                            navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&select=${firstMatchingDesign.guid}`);
                          }
                        }}
                        style={{ cursor: "pointer" }}
                        i18n={"semio.sketchpad.navbar.name"}
                      >
                        {filteredName}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator items={filteredVariantItems} i18n="semio.sketchpad.navbar.selectVariant" onNavigate={(href) => navigate(href)} />
                  </>
                )}
                {filteredName !== null && filteredVariant !== null && (
                  <>
                    <BreadcrumbItem>
                      <BreadcrumbLink
                        onClick={() => {
                          const firstMatchingDesign = (kit?.designs as any[])?.find((d: any) => d.name === filteredName && (d.variant || "") === filteredVariant);
                          if (firstMatchingDesign) {
                            navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(filteredVariant || "")}&select=${firstMatchingDesign.guid}`);
                          }
                        }}
                        style={{ cursor: "pointer" }}
                        i18n={"semio.sketchpad.navbar.variant"}
                      >
                        {filteredVariant || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultVariant")}</span>}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator items={filteredViewItems} i18n="semio.sketchpad.navbar.selectView" onNavigate={(href) => navigate(href)} />
                  </>
                )}
                {filteredName !== null && filteredVariant !== null && filteredView !== null && (
                  <>
                    <BreadcrumbItem>
                      <BreadcrumbLink
                        onClick={() => {
                          const firstMatchingDesign = (kit?.designs as any[])?.find((d: any) => d.name === filteredName && (d.variant || "") === filteredVariant && (d.view || "") === filteredView);
                          if (firstMatchingDesign) {
                            navigate(
                              `/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(filteredVariant || "")}&view=${encodeURIComponent(filteredView || "")}&select=${firstMatchingDesign.guid}`,
                            );
                          }
                        }}
                        style={{ cursor: "pointer" }}
                        i18n={"semio.sketchpad.navbar.view"}
                      >
                        {filteredView || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultView")}</span>}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                  </>
                )}
              </>
            )}
          </>
        )}
        {isDesignApp && design && (
          <>
            <BreadcrumbBreak />
            <BreadcrumbSeparator items={artifactKinds} i18n="semio.sketchpad.navbar.artifacts" onNavigate={(href) => navigate(href)} />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.breadcrumb.designs">
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=designs`)} style={{ cursor: "pointer" }}>
                <Layout size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbBreak />
            <BreadcrumbSeparator
              items={designNameItems}
              i18n="semio.sketchpad.navbar.selectDesign"
              onNavigate={(href) => {
                if (href === "#create-design") handleCreateDesign();
                else navigate(href);
              }}
            />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.design">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(design.name)}&select=${design.guid}`);
                }}
              >
                <button type="button">{design.name}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={designVariantItems}
              i18n="semio.sketchpad.navbar.selectVariant"
              onNavigate={(href) => {
                if (href === "#create-variant") handleCreateVariant(design, false);
                else navigate(href);
              }}
            />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.variant">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(design.name)}&variant=${encodeURIComponent(design.variant || "")}&select=${design.guid}`);
                }}
              >
                <button type="button">{design.variant || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultVariant")}</span>}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={designViewItems}
              i18n="semio.sketchpad.navbar.selectView"
              onNavigate={(href) => {
                if (href === "#create-view") handleCreateView(design);
                else navigate(href);
              }}
            />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.view">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(design.name)}&variant=${encodeURIComponent(design.variant || "")}&view=${encodeURIComponent(design.view || "")}&select=${design.guid}`);
                }}
              >
                <button type="button">{design.view || <span className="italic opacity-70">{t("semio.sketchpad.app.design.defaultView")}</span>}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {isTypeApp && type && (
          <>
            <BreadcrumbBreak />
            <BreadcrumbSeparator items={artifactKinds} i18n="semio.sketchpad.navbar.artifacts" onNavigate={(href) => navigate(href)} />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.breadcrumb.types">
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=types`)} style={{ cursor: "pointer" }}>
                <Box size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbBreak />
            <BreadcrumbSeparator
              items={typeNameItems}
              i18n="semio.sketchpad.navbar.selectType"
              onNavigate={(href) => {
                if (href === "#create-type") handleCreateType();
                else navigate(href);
              }}
            />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.type">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=types&name=${encodeURIComponent(type.name)}&select=${type.guid}`);
                }}
              >
                <button type="button">{type.name}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={typeVariantItems}
              i18n="semio.sketchpad.navbar.selectVariant"
              onNavigate={(href) => {
                if (href === "#create-variant") handleCreateVariant(type, true);
                else navigate(href);
              }}
            />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.variant">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=types&name=${encodeURIComponent(type.name)}&variant=${encodeURIComponent(type.variant || "")}&select=${type.guid}`);
                }}
              >
                <button type="button">{type.variant || <span className="italic opacity-70">{t("semio.sketchpad.app.type.defaultVariant")}</span>}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {isQualityApp && quality && (
          <>
            <BreadcrumbBreak />
            <BreadcrumbSeparator items={artifactKinds} i18n="semio.sketchpad.navbar.artifacts" onNavigate={(href) => navigate(href)} />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.breadcrumb.qualities">
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=qualities`)} style={{ cursor: "pointer" }}>
                <Award size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbBreak />
            <BreadcrumbItem i18n="semio.sketchpad.navbar.quality">
              <BreadcrumbLink
                asChild
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  navigate(`/kits/${kitGuid}?kind=qualities&key=${encodeURIComponent(quality.key)}&select=${quality.guid}`);
                }}
              >
                <button type="button">{quality.name}</button>
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
        {isDocsPath && (
          <>
            <BreadcrumbItem i18n="semio.sketchpad.navbar.docs">
              <BreadcrumbLink onClick={() => navigate("/docs")} style={{ cursor: "pointer" }}>
                <FileText size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            {docsSection && (
              <>
                <BreadcrumbSeparator
                  items={docsSectionsList.map((s) => ({
                    label: (
                      <span className="flex items-center gap-1">
                        {s.icon && <span aria-hidden="true">{s.icon}</span>}
                        <span>{s.label}</span>
                      </span>
                    ),
                    href: `/docs/${s.id}`,
                  }))}
                  i18n="semio.sketchpad.navbar.sections"
                  onNavigate={(href) => navigate(href)}
                />
                <BreadcrumbItem>
                  <BreadcrumbLink onClick={() => navigate(`/docs/${docsSection}`)} style={{ cursor: "pointer" }}>
                    {(() => {
                      const sectionInfo = docsSectionsList.find((s) => s.id === docsSection);
                      if (!sectionInfo) return docsSection;
                      return (
                        <span className="flex items-center gap-1">
                          {sectionInfo.icon && <span aria-hidden="true">{sectionInfo.icon}</span>}
                          <span>{sectionInfo.label}</span>
                        </span>
                      );
                    })()}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </>
            )}
            {docsPagePath &&
              docsSection &&
              (() => {
                const pathAfterSection = docsPagePath.split("/").slice(1);
                const sectionPages = docsRegistry.getAllPages().filter((page) => page.section === docsSection);
                const breadcrumbItems: React.ReactElement[] = [];

                pathAfterSection.forEach((part, index) => {
                  const isLast = index === pathAfterSection.length - 1;
                  const partialParts = pathAfterSection.slice(0, index + 1);
                  const partialPath = `docs/${docsSection}/${partialParts.join("/")}`;
                  const parentParts = pathAfterSection.slice(0, index);
                  const siblings = sectionPages
                    .filter((page) => {
                      const segments = page.path.replace(/^docs\//, "").split("/");
                      const trimmedSegments = segments[segments.length - 1] === "index" ? segments.slice(0, -1) : segments;
                      if (trimmedSegments[0] !== docsSection) return false;
                      const relative = trimmedSegments.slice(1);
                      if (relative.length !== parentParts.length + 1) return false;
                      for (let i = 0; i < parentParts.length; i++) {
                        if (relative[i] !== parentParts[i]) return false;
                      }
                      return true;
                    })
                    .sort((a, b) => {
                      const orderDiff = (a.order ?? 999) - (b.order ?? 999);
                      if (orderDiff !== 0) return orderDiff;
                      return a.title.localeCompare(b.title);
                    });
                  const separatorItems = siblings.map((page) => ({
                    label: page.title,
                    href: `/${page.path.replace(/\/index$/, "")}`,
                  }));
                  const normalizedPartial = `${docsSection}/${partialParts.join("/")}`;
                  const match = siblings.find((page) => page.path.replace(/^docs\//, "").replace(/\/index$/, "") === normalizedPartial) || sectionPages.find((page) => page.path.replace(/^docs\//, "").replace(/\/index$/, "") === normalizedPartial);
                  const label = match?.title
                    ? match.title
                    : part
                        .split("-")
                        .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
                        .join(" ");

                  breadcrumbItems.push(
                    <Fragment key={partialPath}>
                      <BreadcrumbSeparator items={separatorItems} onNavigate={(href) => navigate(href)} />
                      <BreadcrumbItem>
                        <BreadcrumbLink onClick={() => !isLast && navigate(`/${partialPath}`)} style={{ cursor: isLast ? "default" : "pointer" }}>
                          {label}
                        </BreadcrumbLink>
                      </BreadcrumbItem>
                    </Fragment>,
                  );
                });

                return <>{breadcrumbItems}</>;
              })()}
          </>
        )}
      </BreadcrumbList>
    </Breadcrumb>
  );
};

type SearchResult = {
  type: "kit" | "design" | "type" | "quality" | "docs";
  item: KitShallow | DesignShallow | TypeShallow | Quality | { title: string; description?: string; path: string };
  kitGuid?: string;
};

const buildSearchResultPath = (result: SearchResult): string => {
  if (result.type === "kit") return `/kits/${(result.item as KitShallow).guid}`;
  if (result.type === "design") return `/kits/${result.kitGuid}/designs/${(result.item as DesignShallow).guid}`;
  if (result.type === "type") return `/kits/${result.kitGuid}/types/${(result.item as TypeShallow).guid}`;
  if (result.type === "quality") return `/kits/${result.kitGuid}?kind=qualities&select=${(result.item as Quality).guid}`;
  if (result.type === "docs") return `/${(result.item as { path: string }).path}`;
  return "";
};

const Search: FC = ({}) => {
  const { t } = useTranslation();

  const navigate = useNavigate();
  const store = useSketchpadStore();
  const recentSearches = (useSketchpad((s) => s.recentSearches) as string[]) || [];
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const kits = useKits();
  useEffect(() => {
    const handler = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "p") {
        const activeElement = document.activeElement as HTMLElement | null;
        if (!open && activeElement && (activeElement.tagName === "INPUT" || activeElement.tagName === "TEXTAREA" || activeElement.isContentEditable)) return;
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        setOpen((prev) => !prev);
      }
    };
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [open, setOpen]);

  const searchData = useMemo(() => {
    const results: SearchResult[] = [];
    kits.forEach((kit) => {
      results.push({ type: "kit", item: kit as KitShallow, kitGuid: kit.guid });
      (kit.designs || []).forEach((design) => {
        if (typeof design === "object") results.push({ type: "design", item: design as DesignShallow, kitGuid: kit.guid });
      });
      (kit.types || []).forEach((type) => {
        if (typeof type === "object") results.push({ type: "type", item: type as TypeShallow, kitGuid: kit.guid });
      });
      (kit.qualities || []).forEach((quality) => {
        if (typeof quality === "object") results.push({ type: "quality", item: quality as Quality, kitGuid: kit.guid });
      });
    });
    const docsPages = docsRegistry.getAllPages();
    docsPages.forEach((page) => {
      results.push({ type: "docs", item: page });
    });
    return results;
  }, [kits]);

  const searchIndex = useMemo(() => {
    const map = new Map<string, SearchResult>();
    searchData.forEach((result) => {
      const path = buildSearchResultPath(result);
      if (path) map.set(path, result);
    });
    return map;
  }, [searchData]);

  const fuse = useMemo(
    () =>
      new Fuse(searchData, {
        keys: [
          { name: "item.name", weight: 2 },
          { name: "item.title", weight: 2 },
          { name: "item.variant", weight: 1.5 },
          { name: "item.view", weight: 1 },
          { name: "item.description", weight: 0.5 },
          { name: "item.key", weight: 1.5 },
          { name: "item.path", weight: 1 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [searchData],
  );

  const recentResults = useMemo(() => {
    return recentSearches.map((path) => searchIndex.get(path)).filter((result): result is SearchResult => !!result);
  }, [recentSearches, searchIndex]);

  const searchResults = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    // Show all recent results without limit, or fallback to first 20 if no recent results
    const base = recentResults.length > 0 ? recentResults : searchData.slice(0, 20);
    return base.map((item, idx) => ({ item, refIndex: idx }) as FuseResult<SearchResult>);
  }, [fuse, query, recentResults, searchData]);

  const groupedSearchResults = useMemo(() => {
    return {
      kits: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "kit"),
      designs: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "design"),
      types: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "type"),
      qualities: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "quality"),
      docs: searchResults.filter((r: FuseResult<SearchResult>) => r.item.type === "docs"),
    };
  }, [searchResults]);

  const handleSelect = useCallback(
    (result: SearchResult) => {
      const path = buildSearchResultPath(result);
      if (path) {
        const next = [path, ...recentSearches.filter((entry) => entry !== path)].slice(0, 20);
        const changed = next.length !== recentSearches.length || next.some((entry, index) => entry !== recentSearches[index]);
        if (changed) store.change({ recentSearches: next });
      }
      setOpen(false);
      setQuery("");
      if (path) navigate(path);
      else {
        const { type, item, kitGuid } = result;
        if (type === "kit") navigate(`/kits/${(item as KitShallow).guid}`);
        else if (type === "design") navigate(`/kits/${kitGuid}/designs/${(item as DesignShallow).guid}`);
        else if (type === "type") navigate(`/kits/${kitGuid}/types/${(item as TypeShallow).guid}`);
        else if (type === "quality") navigate(`/kits/${kitGuid}?kind=qualities&select=${(item as Quality).guid}`);
        else if (type === "docs") navigate(`/${(item as { path: string }).path}`);
      }
    },
    [navigate, recentSearches, store],
  );

  const getIcon = (type: SearchResult["type"]) => {
    if (type === "kit") return <HardDrive size={16} />;
    if (type === "design") return <Layout size={16} />;
    if (type === "type") return <Box size={16} />;
    if (type === "quality") return <Award size={16} />;
    if (type === "docs") return <FileText size={16} />;
    return null;
  };

  const getDisplayName = (result: SearchResult) => {
    const { type, item } = result;
    if (type === "quality") return (item as Quality).name;
    if (type === "docs") return (item as { title: string }).title;
    const name = (item as any).name || "";
    const variant = (item as any).variant || "";
    const view = (item as any).view || "";
    return [name, variant, view].filter(Boolean).join(" - ");
  };

  return (
    <>
      <Toggle i18n="semio.sketchpad.navbar.search.open" i18nPressed="semio.sketchpad.navbar.search.close" pressed={open} onPressedChange={setOpen}>
        <SearchIcon size={16} />
      </Toggle>
      <CommandDialog i18n={t("semio.sketchpad.navbar.searchTitle")} description={t("semio.sketchpad.navbar.searchDescription")} open={open} onOpenChange={setOpen}>
        <CommandInput placeholder={t("semio.sketchpad.navbar.searchPlaceholder")} value={query} onValueChange={setQuery} />
        <CommandList>
          <CommandEmpty>{t("semio.sketchpad.navbar.noResults")}</CommandEmpty>
          {searchResults.length > 0 && (
            <>
              {groupedSearchResults.kits.length > 0 && (
                <CommandGroup heading={t("semio.sketchpad.navbar.kits")}>
                  {groupedSearchResults.kits.map((r: FuseResult<SearchResult>, idx: number) => (
                    <CommandItem key={`kit-${(r.item.item as KitShallow).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                      <div className="flex items-center gap-2">
                        {getIcon(r.item.type)}
                        <span>{getDisplayName(r.item)}</span>
                      </div>
                    </CommandItem>
                  ))}
                </CommandGroup>
              )}
              {groupedSearchResults.designs.length > 0 && (
                <CommandGroup heading={t("semio.sketchpad.navbar.breadcrumb.designs")}>
                  {groupedSearchResults.designs.map((r: FuseResult<SearchResult>, idx: number) => (
                    <CommandItem key={`design-${(r.item.item as DesignShallow).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                      <div className="flex items-center gap-2">
                        {getIcon(r.item.type)}
                        <span>{getDisplayName(r.item)}</span>
                      </div>
                    </CommandItem>
                  ))}
                </CommandGroup>
              )}
              {groupedSearchResults.types.length > 0 && (
                <CommandGroup heading={t("semio.sketchpad.navbar.breadcrumb.types")}>
                  {groupedSearchResults.types.map((r: FuseResult<SearchResult>, idx: number) => (
                    <CommandItem key={`type-${(r.item.item as TypeShallow).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                      <div className="flex items-center gap-2">
                        {getIcon(r.item.type)}
                        <span>{getDisplayName(r.item)}</span>
                      </div>
                    </CommandItem>
                  ))}
                </CommandGroup>
              )}
              {groupedSearchResults.qualities.length > 0 && (
                <CommandGroup heading={t("semio.sketchpad.navbar.breadcrumb.qualities")}>
                  {groupedSearchResults.qualities.map((r: FuseResult<SearchResult>, idx: number) => (
                    <CommandItem key={`quality-${(r.item.item as Quality).guid}-${idx}`} onSelect={() => handleSelect(r.item)}>
                      <div className="flex items-center gap-2">
                        {getIcon(r.item.type)}
                        <span>{getDisplayName(r.item)}</span>
                      </div>
                    </CommandItem>
                  ))}
                </CommandGroup>
              )}
              {groupedSearchResults.docs.length > 0 && (
                <CommandGroup heading={t("semio.sketchpad.navbar.docs", "Documentation")}>
                  {groupedSearchResults.docs.map((r: FuseResult<SearchResult>, idx: number) => (
                    <CommandItem key={`docs-${(r.item.item as { path: string }).path}-${idx}`} onSelect={() => handleSelect(r.item)}>
                      <div className="flex items-center gap-2">
                        {getIcon(r.item.type)}
                        <span>{getDisplayName(r.item)}</span>
                      </div>
                    </CommandItem>
                  ))}
                </CommandGroup>
              )}
            </>
          )}
        </CommandList>
      </CommandDialog>
    </>
  );
};

const Focus: FC = ({}) => {
  const { t } = useTranslation();

  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const focusContext = useFocusSafe();
  const store = useSketchpadStore();
  const appType = useAppType();
  const recentFocusMap = (useSketchpad((s) => s.recentFocusItems) as Record<string, string[]>) || {};
  const recentFocusIds = recentFocusMap[appType] || [];

  const focusItems = focusContext?.focusItems || [];
  const triggerFocusItem = focusContext?.triggerFocusItem;
  const focusItemIndex = useMemo(() => {
    const map = new Map<string, FocusItem>();
    focusItems.forEach((item) => map.set(item.id, item));
    return map;
  }, [focusItems]);
  const recentFocusItems = useMemo(() => {
    return recentFocusIds.map((id) => focusItemIndex.get(id)).filter((item): item is FocusItem => !!item);
  }, [recentFocusIds, focusItemIndex]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "f") {
        e.preventDefault();
        setOpen((prev) => !prev);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const fuse = useMemo(
    () =>
      new Fuse(focusItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [focusItems],
  );

  const focusResults = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    // Show all recent focus items without limit, or fallback to first 20 if no recent items
    const base = recentFocusItems.length > 0 ? recentFocusItems : focusItems.slice(0, 20);
    return base.map((item, idx) => ({ item, refIndex: idx }));
  }, [fuse, query, recentFocusItems, focusItems]);

  const handleSelect = useCallback(
    (item: FocusItem) => {
      const next = [item.id, ...recentFocusIds.filter((id) => id !== item.id)].slice(0, 20);
      const changed = next.length !== recentFocusIds.length || next.some((id, index) => id !== recentFocusIds[index]);
      if (changed) store.change({ recentFocusItems: { [appType]: next } });
      setOpen(false);
      setQuery("");
      if (triggerFocusItem) triggerFocusItem(item.id);
    },
    [appType, recentFocusIds, store, triggerFocusItem],
  );

  const groupedResults = useMemo(() => {
    const groups: Record<string, typeof focusResults> = {};
    focusResults.forEach((result) => {
      const category = result.item.category || t("semio.sketchpad.navbar.focus.other", "Other");
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    });
    return groups;
  }, [focusResults, t]);

  if (!focusContext) return null;

  return (
    <>
      <Toggle i18n="semio.sketchpad.navbar.focus.open" i18nPressed="semio.sketchpad.navbar.focus.close" pressed={open} onPressedChange={setOpen}>
        <FocusIcon size={16} />
      </Toggle>
      <CommandDialog i18n={t("semio.sketchpad.navbar.focus.title", "Focus")} description={t("semio.sketchpad.navbar.focus.description", "Focus on an element in the current view")} open={open} onOpenChange={setOpen}>
        <CommandInput placeholder={t("semio.sketchpad.navbar.focus.placeholder", "Search for an element...")} value={query} onValueChange={setQuery} />
        <CommandList>
          <CommandEmpty>{t("semio.sketchpad.navbar.noResults")}</CommandEmpty>
          {Object.entries(groupedResults).map(([category, items]) => (
            <CommandGroup key={category} heading={category}>
              {items.map((result, idx) => (
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
    </>
  );
};

const PanelToggles: FC = ({}) => {
  const { t } = useTranslation();

  const { kit, design, type, quality } = useParams();
  const appType = useAppType();
  const panelConfig = getPanelConfigs(t)[appType];
  const visiblePanels = useAppPanelVisibility();
  const appCommands = useAppCommands();
  const homeCommands = useHomeCommands();
  const isValidKit = kit && !["temporary", "local", "remote"].includes(kit);
  const kitAppCommands = useKitAppCommands(isValidKit ? { kit } : undefined);
  const designAppCommands = useDesignAppCommands(isValidKit && design ? { kit, design } : undefined);
  const typeAppCommands = useTypeAppCommands(isValidKit && type ? { kit, type } : undefined);
  const qualityAppCommands = useQualityAppCommands(isValidKit && quality ? { kit, quality } : undefined);
  const commands: Record<string, any> = {
    home: homeCommands,
    kit: kitAppCommands,
    design: designAppCommands,
    type: typeAppCommands,
    quality: qualityAppCommands,
    docs: appCommands,
  };
  const isMobile = useIsMobile();

  const workbenchPanels = ["workbench", "tools"];
  const workbenchConfigs = panelConfig.filter((p) => workbenchPanels.includes(p.key));
  const workbenchDefaultKey = workbenchConfigs[0]?.key || "";
  const workbenchSelectionRef = useRef<string>(workbenchDefaultKey);
  if (!workbenchConfigs.some((config) => config.key === workbenchSelectionRef.current)) {
    workbenchSelectionRef.current = workbenchDefaultKey;
  }
  const openWorkbenchPanelKey = workbenchConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key;
  const isAnyWorkbenchPanelOpen = Boolean(openWorkbenchPanelKey);
  if (openWorkbenchPanelKey && workbenchSelectionRef.current !== openWorkbenchPanelKey) {
    workbenchSelectionRef.current = openWorkbenchPanelKey;
  }
  const activeWorkbenchPanel = workbenchSelectionRef.current || workbenchDefaultKey;

  const hudPanels = ["hud", "stats"];
  const hudConfigs = panelConfig.filter((p) => hudPanels.includes(p.key));
  const hudDefaultKey = hudConfigs[0]?.key || "";
  const hudSelectionRef = useRef<string>(hudDefaultKey);
  if (!hudConfigs.some((config) => config.key === hudSelectionRef.current)) {
    hudSelectionRef.current = hudDefaultKey;
  }
  const openHudPanelKey = hudConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key;
  const isAnyHudPanelOpen = Boolean(openHudPanelKey);
  if (openHudPanelKey && hudSelectionRef.current !== openHudPanelKey) {
    hudSelectionRef.current = openHudPanelKey;
  }
  const activeHudPanel = hudSelectionRef.current || hudDefaultKey;

  const rightPanels = ["details", "chat", "settings"];
  const rightConfigs = panelConfig.filter((p) => rightPanels.includes(p.key));
  const rightDefaultKey = rightConfigs[0]?.key || "";
  const rightSelectionRef = useRef<string>(rightDefaultKey);
  if (!rightConfigs.some((config) => config.key === rightSelectionRef.current)) {
    rightSelectionRef.current = rightDefaultKey;
  }
  const openRightPanelKey = rightConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key;
  const isAnyRightPanelOpen = Boolean(openRightPanelKey);
  if (openRightPanelKey && rightSelectionRef.current !== openRightPanelKey) {
    rightSelectionRef.current = openRightPanelKey;
  }
  const activeRightPanel = rightSelectionRef.current || rightDefaultKey;

  const otherConfigs = panelConfig.filter((p) => !workbenchPanels.includes(p.key) && !hudPanels.includes(p.key) && !rightPanels.includes(p.key) && p.key !== "toolbar");

  const panelToggleTooltip = (panelKey: string, open: boolean) => (panelKey ? `semio.sketchpad.navbar.panelToggle.${panelKey}.${open ? "hide" : "show"}` : undefined);
  const rightDropdownAriaLabel = `semio.sketchpad.navbar.panelToggle.right.label` || undefined;

  const handleToggle = (panelKey: keyof PanelVisibility) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    const current = visiblePanels[panelKey];

    if (isMobile) {
      if (!current) {
        (Object.keys(visiblePanels) as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
    } else {
      if (!current && rightPanels.includes(panelKey)) {
        (rightPanels as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
      if (!current && workbenchPanels.includes(panelKey)) {
        (workbenchPanels as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
      if (!current && hudPanels.includes(panelKey)) {
        (hudPanels as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
    }
    togglePanel(panelKey);
  };

  const handleWorkbenchPressedChange = (pressed: boolean) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (pressed) {
      if (activeWorkbenchPanel && !visiblePanels[activeWorkbenchPanel as keyof PanelVisibility]) {
        handleToggle(activeWorkbenchPanel as keyof PanelVisibility);
      }
    } else {
      const openPanel = workbenchConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
      if (openPanel) {
        togglePanel(openPanel.key as keyof PanelVisibility);
      }
    }
  };

  const handleWorkbenchValueChange = (value: string | undefined) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (!value) return;
    workbenchSelectionRef.current = value;

    (workbenchPanels as Array<keyof PanelVisibility>).forEach((p) => {
      const isOpen = visiblePanels[p];
      const shouldOpen = p === value;

      if (isOpen && !shouldOpen) {
        togglePanel(p);
      } else if (!isOpen && shouldOpen) {
        togglePanel(p);
      }
    });
  };

  const handleHudPressedChange = (pressed: boolean) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (pressed) {
      if (activeHudPanel && !visiblePanels[activeHudPanel as keyof PanelVisibility]) {
        handleToggle(activeHudPanel as keyof PanelVisibility);
      }
    } else {
      const openPanel = hudConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
      if (openPanel) {
        togglePanel(openPanel.key as keyof PanelVisibility);
      }
    }
  };

  const handleHudValueChange = (value: string | undefined) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (!value) return;
    hudSelectionRef.current = value;

    (hudPanels as Array<keyof PanelVisibility>).forEach((p) => {
      const isOpen = visiblePanels[p];
      const shouldOpen = p === value;

      if (isOpen && !shouldOpen) {
        togglePanel(p);
      } else if (!isOpen && shouldOpen) {
        togglePanel(p);
      }
    });
  };

  const handleRightPressedChange = (pressed: boolean) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (pressed) {
      if (activeRightPanel && !visiblePanels[activeRightPanel as keyof PanelVisibility]) {
        handleToggle(activeRightPanel as keyof PanelVisibility);
      }
    } else {
      const openPanel = rightConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
      if (openPanel) {
        togglePanel(openPanel.key as keyof PanelVisibility);
      }
    }
  };

  const handleRightValueChange = (value: string | undefined) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (!value) return;
    rightSelectionRef.current = value;

    (rightPanels as Array<keyof PanelVisibility>).forEach((p) => {
      const isOpen = visiblePanels[p];
      const shouldOpen = p === value;

      if (isOpen && !shouldOpen) {
        togglePanel(p);
      } else if (!isOpen && shouldOpen) {
        togglePanel(p);
      }
    });
  };

  if (!panelConfig || panelConfig.length === 0) return null;

  return (
    <div className="flex items-stretch border overflow-hidden h-9">
      {workbenchConfigs.length > 0 && (
        <Toggle
          type="dropdown"
          pressed={isAnyWorkbenchPanelOpen}
          onPressedChange={handleWorkbenchPressedChange}
          value={activeWorkbenchPanel}
          onValueChange={handleWorkbenchValueChange}
          i18n={panelToggleTooltip(activeWorkbenchPanel, isAnyWorkbenchPanelOpen)}
          dropdownI18n={"semio.sketchpad.navbar.changePanelType"}
          className="border-0 border-l first:border-l-0 -ml-px first:ml-0"
          items={workbenchConfigs.map(({ key, icon: Icon, hotkey }) => ({
            value: key,
            label: <Icon />,
            i18n: panelToggleTooltip(key, key === activeWorkbenchPanel ? isAnyWorkbenchPanelOpen : false),
            hotkey,
          }))}
        />
      )}

      {hudConfigs.length > 0 && (
        <Toggle
          type="dropdown"
          pressed={isAnyHudPanelOpen}
          onPressedChange={handleHudPressedChange}
          value={activeHudPanel}
          onValueChange={handleHudValueChange}
          i18n={panelToggleTooltip(activeHudPanel, isAnyHudPanelOpen)}
          dropdownI18n={"semio.sketchpad.navbar.changePanelType"}
          className="border-0 border-l first:border-l-0 -ml-px first:ml-0"
          items={hudConfigs.map(({ key, icon: Icon, hotkey }) => ({
            value: key,
            label: <Icon />,
            i18n: panelToggleTooltip(key, key === activeHudPanel ? isAnyHudPanelOpen : false),
            hotkey,
          }))}
        />
      )}

      <ToggleGroup
        type="multiple"
        value={[
          ...(isAnyWorkbenchPanelOpen ? [activeWorkbenchPanel] : []),
          ...(isAnyHudPanelOpen ? [activeHudPanel] : []),
          ...otherConfigs.filter((p) => visiblePanels[p.key as keyof PanelVisibility]).map((p) => p.key),
          ...(isAnyRightPanelOpen ? [activeRightPanel] : []),
        ]}
        className="border-0 border-l first:border-l-0 -ml-px first:ml-0"
      >
        {otherConfigs.map(({ key, icon: Icon }) => (
          <ToggleGroupItem
            key={key}
            value={key}
            i18n={panelToggleTooltip(key, Boolean(visiblePanels[key as keyof PanelVisibility]))}
            onClick={() => {
              handleToggle(key as keyof PanelVisibility);
            }}
          >
            <Icon />
          </ToggleGroupItem>
        ))}
      </ToggleGroup>

      {rightConfigs.length > 0 && (
        <Toggle
          type="dropdown"
          pressed={isAnyRightPanelOpen}
          onPressedChange={handleRightPressedChange}
          value={activeRightPanel}
          onValueChange={handleRightValueChange}
          i18n={panelToggleTooltip(activeRightPanel, isAnyRightPanelOpen)}
          dropdownI18n={"semio.sketchpad.navbar.changePanelType"}
          className="border-0 border-l first:border-l-0 -ml-px first:ml-0"
          aria-label={rightDropdownAriaLabel}
          items={rightConfigs.map(({ key, icon: Icon, hotkey }) => ({
            value: key,
            label: <Icon />,
            i18n: panelToggleTooltip(key, key === activeRightPanel ? isAnyRightPanelOpen : false),
            hotkey,
          }))}
        />
      )}
    </div>
  );
};

interface NavbarBaseProps {
  isFullscreen: boolean;
  isNavbarExpanded: boolean;
  i18n: (key: string) => string | undefined;
  onWindowEvents?: WindowEvents;
}

function NavbarMobile({ isFullscreen, isNavbarExpanded, i18n, onWindowEvents }: NavbarBaseProps) {
  const { t } = useTranslation();
  const { toggleFullscreen, toggleNavbarExpanded, navigateBack, navigateForward } = useSketchpadCommands();
  const [isVisible, setIsVisible] = useState(true);
  const [searchOpen, setSearchOpen] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();
  const currentPathname = useNavigation();
  const currentPath = `${currentPathname}${location.search}`;
  const { canGoBack, canGoForward } = useNavigationHistory();
  const [searchParams] = useSearchParams();
  const kits = useKits();
  const store = useSketchpadStore();
  const pathParts = currentPathname.split("/").filter((p) => p);
  const isKitsPath = pathParts[0] === "kits";
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
  const isDesignAppPath = isKitsPath && pathParts[2] === "designs" && uuidRegex.test(pathParts[3] || "");
  const isTypeAppPath = isKitsPath && pathParts[2] === "types" && uuidRegex.test(pathParts[3] || "");
  const kitGuid = isKitsPath && pathParts[1] ? pathParts[1] : null;
  const itemGuid = isDesignAppPath || isTypeAppPath ? pathParts[3] : null;
  const homeKind = !isKitsPath || pathParts.length === 1 ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !isKitsPath || pathParts.length === 1 ? searchParams.get("name") : null;
  const homeVersion = !isKitsPath || pathParts.length === 1 ? searchParams.get("version") : null;
  const kitKind = useMemo(() => {
    if (!kitGuid || !store.hasKit(kitGuid)) return undefined;
    const kitStore = store.kit(kitGuid);
    if (!kitStore) return undefined;
    if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) return "remote";
    if (kitStore.isLocallyPersisted) return "local";
    return "temporary";
  }, [kitGuid, store]);
  const currentKit = useMemo(() => {
    if (!kitGuid) return undefined;
    return kits.find((k) => k.guid === kitGuid);
  }, [kits, kitGuid]);
  const allDesigns = useMemo(() => {
    if (!currentKit?.designs) return [];
    return (currentKit.designs as any[]).filter((d): d is Design => typeof d === "object" && d.guid !== undefined);
  }, [currentKit?.designs]);
  const allTypes = useMemo(() => {
    if (!currentKit?.types) return [];
    return (currentKit.types as any[]).filter((t): t is Type => typeof t === "object" && t.guid !== undefined);
  }, [currentKit?.types]);

  const allQualities = useMemo(() => {
    if (!currentKit?.qualities) return [];
    return (currentKit.qualities as any[]).filter((q): q is Quality => typeof q === "object" && q.guid !== undefined);
  }, [currentKit?.qualities]);
  const filteredKind = kitGuid && !isDesignAppPath && !isTypeAppPath ? (searchParams.get("kind") as "designs" | "types" | "qualities" | "files" | "authors" | null) : null;
  const filteredName = kitGuid && !isDesignAppPath && !isTypeAppPath ? searchParams.get("name") : null;
  const filteredVariant = kitGuid && !isDesignAppPath && !isTypeAppPath ? searchParams.get("variant") : null;
  const filteredView = kitGuid && !isDesignAppPath && !isTypeAppPath ? searchParams.get("view") : null;
  const selectedGuid = kitGuid && !isDesignAppPath && !isTypeAppPath ? searchParams.get("select") : null;
  const currentDesign = useMemo(() => {
    if (!isDesignAppPath || !itemGuid) return undefined;
    return allDesigns.find((d) => d.guid === itemGuid);
  }, [isDesignAppPath, allDesigns, itemGuid]);
  const currentType = useMemo(() => {
    if (!isTypeAppPath || !itemGuid) return undefined;
    return allTypes.find((t) => t.guid === itemGuid);
  }, [isTypeAppPath, allTypes, itemGuid]);
  const isKitAppPath = Boolean(kitGuid && !isDesignAppPath && !isTypeAppPath);
  const breadcrumbTrail = useMemo(() => {
    const items: string[] = [];
    const add = (value: string | null | undefined) => {
      if (!value) return;
      if (items[items.length - 1] !== value) items.push(value);
    };
    add("/");
    const kindValue = kitKind || homeKind;
    if (kindValue) add(`/?kind=${kindValue}`);
    if (kitGuid && kitKind && currentKit) {
      const kitNameValue = currentKit.name || "";
      add(`/?kind=${kitKind}&name=${encodeURIComponent(kitNameValue)}`);
      if (currentKit.version !== undefined) add(`/?kind=${kitKind}&name=${encodeURIComponent(kitNameValue)}&version=${encodeURIComponent(currentKit.version || "")}`);
    } else if (homeKind && homeName) {
      add(`/?kind=${homeKind}&name=${encodeURIComponent(homeName)}`);
      if (homeVersion !== null) add(`/?kind=${homeKind}&name=${encodeURIComponent(homeName)}&version=${encodeURIComponent(homeVersion)}`);
    }
    if (isKitAppPath && kitGuid && filteredKind) {
      const base = `/kits/${kitGuid}`;
      const kindParams = new URLSearchParams();
      kindParams.set("kind", filteredKind);
      add(`${base}?${kindParams.toString()}`);
      if (filteredName !== null) {
        const nameParams = new URLSearchParams(kindParams);
        nameParams.set("name", filteredName);
        if (selectedGuid) nameParams.set("select", selectedGuid);
        add(`${base}?${nameParams.toString()}`);
        if (filteredVariant !== null) {
          const variantParams = new URLSearchParams(nameParams);
          variantParams.set("variant", filteredVariant);
          add(`${base}?${variantParams.toString()}`);
          if (filteredView !== null && filteredKind === "designs") {
            const viewParams = new URLSearchParams(variantParams);
            viewParams.set("view", filteredView);
            add(`${base}?${viewParams.toString()}`);
          }
        }
      }
    }
    if (isDesignAppPath && kitGuid && currentDesign) {
      add(`/kits/${kitGuid}?kind=designs`);
      add(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(currentDesign.name)}&select=${currentDesign.guid}`);
      add(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(currentDesign.name)}&variant=${encodeURIComponent(currentDesign.variant || "")}&select=${currentDesign.guid}`);
    }
    if (isTypeAppPath && kitGuid && currentType) {
      add(`/kits/${kitGuid}?kind=types`);
      add(`/kits/${kitGuid}?kind=types&name=${encodeURIComponent(currentType.name)}&select=${currentType.guid}`);
      add(`/kits/${kitGuid}?kind=types&name=${encodeURIComponent(currentType.name)}&variant=${encodeURIComponent(currentType.variant || "")}&select=${currentType.guid}`);
    }
    add(currentPath);
    return items;
  }, [
    kitKind,
    homeKind,
    kitGuid,
    currentKit,
    homeName,
    homeVersion,
    isKitAppPath,
    filteredKind,
    filteredName,
    filteredVariant,
    filteredView,
    selectedGuid,
    isDesignAppPath,
    currentDesign,
    isTypeAppPath,
    currentType,
    currentPath,
    allDesigns,
    allTypes,
  ]);
  const upTarget = useMemo(() => {
    if (breadcrumbTrail.length < 2) return undefined;
    const current = breadcrumbTrail[breadcrumbTrail.length - 1];
    for (let i = breadcrumbTrail.length - 2; i >= 0; i--) {
      if (breadcrumbTrail[i] !== current) return breadcrumbTrail[i];
    }
    return undefined;
  }, [breadcrumbTrail]);
  const isAtRoot = !upTarget;

  // Always call hooks unconditionally
  const appType = useAppType();
  const panelConfig = getPanelConfigs(t)[appType];
  const visiblePanels = useAppPanelVisibility();
  const toolbarConfig = panelConfig.find((p) => p.key === "toolbar");
  const { kit, design, type, quality } = useParams();
  const homeCommands = useHomeCommands();
  const isValidKit = kit && !["temporary", "local", "remote"].includes(kit);
  const kitAppCommands = useKitAppCommands(isValidKit ? { kit } : undefined);
  const designAppCommands = useDesignAppCommands(isValidKit && design ? { kit, design } : undefined);
  const typeAppCommands = useTypeAppCommands(isValidKit && type ? { kit, type } : undefined);
  const qualityAppCommands = useQualityAppCommands(isValidKit && quality ? { kit, quality } : undefined);
  const appCommands = useAppCommands();
  const commands: Record<string, any> = {
    home: homeCommands,
    kit: kitAppCommands,
    design: designAppCommands,
    type: typeAppCommands,
    quality: qualityAppCommands,
    docs: appCommands,
  };
  if (!commands[appType]) commands[appType] = appCommands;

  const workbenchPanels = useMemo(() => ["workbench", "tools"].filter((panelKey) => panelConfig.some((panel) => panel.key === panelKey)) as PanelKey[], [panelConfig]);
  const hudPanels = useMemo(() => ["hud", "stats"].filter((panelKey) => panelConfig.some((panel) => panel.key === panelKey)) as PanelKey[], [panelConfig]);
  const rightPanels = useMemo(() => ["details", "chat", "settings"].filter((panelKey) => panelConfig.some((panel) => panel.key === panelKey)) as PanelKey[], [panelConfig]);

  const activeWorkbenchPanel = useMemo(() => workbenchPanels.find((key) => visiblePanels[key as keyof PanelVisibility]) ?? workbenchPanels[0], [visiblePanels, workbenchPanels]);
  const activeHudPanel = useMemo(() => hudPanels.find((key) => visiblePanels[key as keyof PanelVisibility]) ?? hudPanels[0], [visiblePanels, hudPanels]);
  const activeRightPanel = useMemo(() => rightPanels.find((key) => visiblePanels[key as keyof PanelVisibility]) ?? rightPanels[0], [visiblePanels, rightPanels]);

  const toggleGroupPanel = useCallback(
    (targetKey: PanelKey | undefined, groupKeys: PanelKey[]) => {
      const togglePanelFn = commands[appType]?.togglePanel;
      if (!targetKey || !togglePanelFn) return;
      const typedTarget = targetKey as keyof PanelVisibility;
      const isOpen = visiblePanels[typedTarget];
      if (isOpen) {
        togglePanelFn(typedTarget);
        return;
      }
      groupKeys.forEach((key) => {
        if (key !== targetKey && visiblePanels[key as keyof PanelVisibility]) {
          togglePanelFn(key as keyof PanelVisibility);
        }
      });
      togglePanelFn(typedTarget);
    },
    [appCommands, appType, commands, visiblePanels],
  );

  useEffect(() => {
    const resolvePanelKey = (preferred: PanelKey | undefined, groupKeys: PanelKey[]) => {
      if (groupKeys.length === 0) return undefined;
      if (preferred && groupKeys.includes(preferred)) return preferred;
      return groupKeys[0];
    };

    const handler = (event: KeyboardEvent) => {
      if (event.ctrlKey || event.metaKey) {
        const key = event.key.toLowerCase();
        if (key === "j") {
          const targetKey = resolvePanelKey("workbench", workbenchPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, workbenchPanels);
          return;
        }
        if (key === "k") {
          const targetKey = resolvePanelKey("hud", hudPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, hudPanels);
          return;
        }
        if (key === "l") {
          const targetKey = resolvePanelKey("details", rightPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, rightPanels);
          return;
        }
        if (key === "ö" || key === "ø") {
          const targetKey = resolvePanelKey("tools", workbenchPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, workbenchPanels);
          return;
        }
        if (key === "p") {
          const activeElement = document.activeElement as HTMLElement | null;
          if (!searchOpen && activeElement && (activeElement.tagName === "INPUT" || activeElement.tagName === "TEXTAREA" || activeElement.isContentEditable)) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          setSearchOpen((prev) => !prev);
          return;
        }
      }
      if (event.key === "F11") {
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        toggleFullscreen();
        return;
      }
      if (!event.altKey || event.ctrlKey || event.metaKey) return;
      if (event.key !== "ArrowLeft" && event.key !== "ArrowRight" && event.key !== "ArrowUp") return;
      const activeElement = document.activeElement as HTMLElement | null;
      if (activeElement && (activeElement.tagName === "INPUT" || activeElement.tagName === "TEXTAREA" || activeElement.isContentEditable)) return;
      if (event.key === "ArrowLeft") {
        if (!canGoBack) return;
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        navigateBack();
        return;
      }
      if (event.key === "ArrowRight") {
        if (!canGoForward) return;
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        navigateForward();
        return;
      }
      if (!upTarget) return;
      event.preventDefault();
      event.stopPropagation();
      event.stopImmediatePropagation();
      navigate(upTarget);
    };
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [activeHudPanel, activeRightPanel, activeWorkbenchPanel, canGoBack, canGoForward, hudPanels, navigate, navigateBack, navigateForward, rightPanels, searchOpen, setSearchOpen, toggleFullscreen, toggleGroupPanel, upTarget, workbenchPanels]);

  // Find the currently active panel (used by mobile)
  // Default to first panel if none is open
  const activePanel = panelConfig.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key || panelConfig[0]?.key || "";
  const isAnyPanelOpen = panelConfig.some((p) => visiblePanels[p.key as keyof PanelVisibility]);
  const mobilePanelTooltip = activePanel ? i18n(`semio.sketchpad.navbar.panelToggle.${activePanel}.${isAnyPanelOpen ? "hide" : "show"}`) : undefined;

  const handleMobilePanelToggle = (pressed: boolean) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (pressed) {
      // Open the active panel if none is open
      if (activePanel && !visiblePanels[activePanel as keyof PanelVisibility]) {
        togglePanel(activePanel as keyof PanelVisibility);
      } else if (!activePanel && panelConfig.length > 0) {
        // Default to first panel
        togglePanel(panelConfig[0].key as keyof PanelVisibility);
      }
    } else {
      // Close the currently open panel
      const openPanel = panelConfig.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
      if (openPanel) {
        togglePanel(openPanel.key as keyof PanelVisibility);
      }
    }
  };

  const handleMobilePanelChange = (value: string | undefined) => {
    const togglePanel = commands[appType]?.togglePanel || (() => {});
    if (!value) return;

    // Close all other panels and open the selected one
    (panelConfig.map((p) => p.key) as Array<keyof PanelVisibility>).forEach((p) => {
      const isOpen = visiblePanels[p];
      const shouldOpen = p === value;

      if (isOpen && !shouldOpen) {
        togglePanel(p);
      } else if (!isOpen && shouldOpen) {
        togglePanel(p);
      }
    });
  };

  useEffect(() => {
    if (!isFullscreen) {
      setIsVisible(true);
      return;
    }

    const handleMouseMove = (e: MouseEvent) => {
      setIsVisible(e.clientY < 50);
    };

    window.addEventListener("mousemove", handleMouseMove);
    return () => window.removeEventListener("mousemove", handleMouseMove);
  }, [isFullscreen]);

  return (
    <div id="navbar" className={`w-full border-b flex flex-col [-webkit-app-region: drag] transition-transform duration-200 ${isFullscreen && !isVisible ? "-translate-y-full" : "translate-y-0"}`} style={{ WebkitAppRegion: "drag" } as any}>
      {/* Unexpanded navbar */}
      <div className="h-12 flex items-center justify-between px-1 gap-1">
        <ButtonGroup>
          <ButtonGroupItem value="back" i18n="semio.sketchpad.navbar.back" onClick={navigateBack} disabled={!canGoBack}>
            <ArrowLeft size={16} />
          </ButtonGroupItem>
          <ButtonGroupItem value="forward" i18n="semio.sketchpad.navbar.forward" onClick={navigateForward} disabled={!canGoForward}>
            <ArrowRight size={16} />
          </ButtonGroupItem>
          <ButtonGroupItem
            value="up"
            i18n="semio.sketchpad.navbar.up"
            onClick={() => {
              if (upTarget) navigate(upTarget);
            }}
            disabled={isAtRoot}
          >
            <ArrowUp size={16} />
          </ButtonGroupItem>
        </ButtonGroup>

        {/* Single dropdown toggle for all panels on mobile */}
        {panelConfig.filter((p) => p.key !== "toolbar").length > 0 && (
          <Toggle
            type="dropdown"
            pressed={isAnyPanelOpen}
            onPressedChange={handleMobilePanelToggle}
            value={activePanel}
            onValueChange={handleMobilePanelChange}
            i18n={mobilePanelTooltip}
            dropdownI18n="semio.sketchpad.navbar.changePanelType"
            items={panelConfig
              .filter((p) => p.key !== "toolbar")
              .map(({ key, icon: Icon, hotkey }) => ({
                value: key,
                label: <Icon />,
                i18n: i18n(`navbar.panelToggle.${key}.${key === activePanel && isAnyPanelOpen ? "hide" : "show"}`),
                hotkey,
              }))}
          />
        )}

        <div className="flex gap-1">
          {toolbarConfig && (
            <Toggle
              i18n="semio.sketchpad.navbar.panelToggle.toolbar.show"
              i18nPressed="semio.sketchpad.navbar.panelToggle.toolbar.hide"
              pressed={!!visiblePanels.toolbar}
              onPressedChange={() => {
                commands[appType]?.togglePanel("toolbar");
              }}
            >
              <toolbarConfig.icon size={16} />
            </Toggle>
          )}
          <Toggle i18n="semio.sketchpad.navbar.search.open" i18nPressed="semio.sketchpad.navbar.search.close" pressed={searchOpen} onPressedChange={setSearchOpen}>
            <SearchIcon size={16} />
          </Toggle>
          <Focus />
          <Toggle i18n="semio.sketchpad.navbar.fullscreen" i18nPressed="semio.sketchpad.navbar.exitFullscreen" pressed={isFullscreen} onPressedChange={toggleFullscreen}>
            <Fullscreen size={16} />
          </Toggle>
          <Toggle i18n={"semio.sketchpad.navbar.expand"} i18nPressed={"semio.sketchpad.navbar.collapse"} pressed={isNavbarExpanded} onPressedChange={toggleNavbarExpanded}>
            {isNavbarExpanded ? <ChevronUp /> : <ChevronDown />}
          </Toggle>
        </div>
      </div>

      {/* Expanded navbar content */}
      {isNavbarExpanded && (
        <div className="flex flex-col gap-1 px-1 pb-1">
          <Navigation />

          {onWindowEvents && (
            <ButtonGroup>
              <ButtonGroupItem value="minimize" i18n="semio.sketchpad.navbar.minimize" onClick={onWindowEvents.minimize}>
                <Minus size={16} />
              </ButtonGroupItem>
              <ButtonGroupItem value="maximize" i18n="semio.sketchpad.navbar.maximize" onClick={onWindowEvents.maximize}>
                <Square size={16} />
              </ButtonGroupItem>
              <ButtonGroupItem value="close" i18n="semio.sketchpad.navbar.close" onClick={onWindowEvents.close}>
                <X size={16} />
              </ButtonGroupItem>
            </ButtonGroup>
          )}
        </div>
      )}

      {/* Search dialog */}
      <CommandDialog i18n={t("semio.sketchpad.navbar.searchTitle")} description={t("semio.sketchpad.navbar.searchDescription")} open={searchOpen} onOpenChange={setSearchOpen}>
        <CommandInput placeholder={t("semio.sketchpad.navbar.searchPlaceholder")} />
        <CommandList>
          <CommandEmpty>{t("semio.sketchpad.navbar.noResults")}</CommandEmpty>
          <CommandGroup heading={t("semio.sketchpad.navbar.suggestions")}>{/* TODO: Add command items here */}</CommandGroup>
        </CommandList>
      </CommandDialog>
    </div>
  );
}

function NavbarDesktop({ isFullscreen, isNavbarExpanded, i18n, onWindowEvents }: NavbarBaseProps) {
  const { t } = useTranslation();
  const { toggleFullscreen, navigateBack, navigateForward } = useSketchpadCommands();
  const [isVisible, setIsVisible] = useState(true);
  const navigate = useNavigate();
  const location = useLocation();
  const currentPathname = useNavigation();
  const currentPath = `${currentPathname}${location.search}`;
  const { canGoBack, canGoForward } = useNavigationHistory();
  const [searchParams] = useSearchParams();
  const kits = useKits();

  const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);

  const pathParts = currentPathname.split("/").filter(Boolean);
  const isKitsPath = pathParts[0] === "kits";
  const kitGuid = isKitsPath && pathParts[1] && isUuidPattern(pathParts[1]) ? pathParts[1] : null;
  const homeKind = !kitGuid ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !kitGuid ? searchParams.get("name") : null;

  const upTarget = currentPath === "/" ? undefined : currentPath === "/kits" ? "/" : currentPath.split("/").slice(0, -1).join("/") || "/";
  const isAtRoot = currentPath === "/" || currentPath === "/kits";

  const secondPart = pathParts[2];
  const thirdPart = pathParts[3];
  const isDesignApp = isKitsPath && secondPart === "designs" && thirdPart && isUuidPattern(thirdPart);
  const isTypeApp = isKitsPath && secondPart === "types" && thirdPart && isUuidPattern(thirdPart);
  const isQualityApp = isKitsPath && secondPart === "qualities" && thirdPart && isUuidPattern(thirdPart);

  const appType = useAppType();
  const visiblePanels = useAppPanelVisibility();
  const panelConfig = getPanelConfigs(t)[appType];
  const toolbarConfig = panelConfig.find((p) => p.key === "toolbar");
  const homeCommands = useHomeCommands();
  const isValidKit = kitGuid && !["temporary", "local", "remote"].includes(kitGuid);
  const kitAppCommands = useKitAppCommands(isValidKit ? { kit: kitGuid } : undefined);
  const design = thirdPart && isDesignApp ? thirdPart : null;
  const type = thirdPart && isTypeApp ? thirdPart : null;
  const quality = thirdPart && secondPart === "qualities" && thirdPart ? thirdPart : null;
  const designAppCommands = useDesignAppCommands(isValidKit && design ? { kit: kitGuid, design } : undefined);
  const typeAppCommands = useTypeAppCommands(isValidKit && type ? { kit: kitGuid, type } : undefined);
  const qualityAppCommands = useQualityAppCommands(isValidKit && quality ? { kit: kitGuid, quality } : undefined);
  const appCommands = useAppCommands();
  const commands: Record<string, any> = {
    home: homeCommands,
    kit: kitAppCommands,
    design: designAppCommands,
    type: typeAppCommands,
    quality: qualityAppCommands,
    docs: appCommands,
  };
  if (!commands[appType]) commands[appType] = appCommands;

  const workbenchPanels = useMemo(() => ["workbench", "tools"].filter((panelKey) => panelConfig.some((panel) => panel.key === panelKey)) as PanelKey[], [panelConfig]);
  const hudPanels = useMemo(() => ["hud", "stats"].filter((panelKey) => panelConfig.some((panel) => panel.key === panelKey)) as PanelKey[], [panelConfig]);
  const rightPanels = useMemo(() => ["details", "chat", "settings"].filter((panelKey) => panelConfig.some((panel) => panel.key === panelKey)) as PanelKey[], [panelConfig]);

  const toggleGroupPanel = useCallback(
    (targetKey: PanelKey | undefined, groupKeys: PanelKey[]) => {
      const togglePanelFn = commands[appType]?.togglePanel;
      if (!targetKey || !togglePanelFn) return;
      const typedTarget = targetKey as keyof PanelVisibility;
      const isOpen = visiblePanels[typedTarget];
      if (isOpen) {
        togglePanelFn(typedTarget);
        return;
      }
      groupKeys.forEach((key) => {
        if (key !== targetKey && visiblePanels[key as keyof PanelVisibility]) {
          togglePanelFn(key as keyof PanelVisibility);
        }
      });
      togglePanelFn(typedTarget);
    },
    [appCommands, appType, commands, visiblePanels],
  );

  useEffect(() => {
    const resolvePanelKey = (preferred: PanelKey | undefined, groupKeys: PanelKey[]) => {
      if (groupKeys.length === 0) return undefined;
      if (preferred && groupKeys.includes(preferred)) return preferred;
      return groupKeys[0];
    };

    const handler = (event: KeyboardEvent) => {
      if (event.ctrlKey || event.metaKey) {
        const key = event.key.toLowerCase();
        if (key === "j") {
          const targetKey = resolvePanelKey("workbench", workbenchPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, workbenchPanels);
          return;
        }
        if (key === "k") {
          const targetKey = resolvePanelKey("hud", hudPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, hudPanels);
          return;
        }
        if (key === "l") {
          const targetKey = resolvePanelKey("details", rightPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, rightPanels);
          return;
        }
        if (key === "ö" || key === "ø") {
          const targetKey = resolvePanelKey("tools", workbenchPanels);
          if (!targetKey) return;
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();
          toggleGroupPanel(targetKey, workbenchPanels);
          return;
        }
      }
      if (event.key === "F11") {
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        toggleFullscreen();
        return;
      }
      if (!event.altKey || event.ctrlKey || event.metaKey) return;
      if (event.key !== "ArrowLeft" && event.key !== "ArrowRight" && event.key !== "ArrowUp") return;
      const activeElement = document.activeElement as HTMLElement | null;
      if (activeElement && (activeElement.tagName === "INPUT" || activeElement.tagName === "TEXTAREA" || activeElement.isContentEditable)) return;
      if (event.key === "ArrowLeft") {
        if (!canGoBack) return;
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        navigateBack();
        return;
      }
      if (event.key === "ArrowRight") {
        if (!canGoForward) return;
        event.preventDefault();
        event.stopPropagation();
        event.stopImmediatePropagation();
        navigateForward();
        return;
      }
      if (!upTarget) return;
      event.preventDefault();
      event.stopPropagation();
      event.stopImmediatePropagation();
      navigate(upTarget);
    };
    window.addEventListener("keydown", handler, true);
    return () => window.removeEventListener("keydown", handler, true);
  }, [canGoBack, canGoForward, hudPanels, navigate, navigateBack, navigateForward, rightPanels, toggleFullscreen, toggleGroupPanel, upTarget, workbenchPanels]);

  useEffect(() => {
    if (!isFullscreen) {
      setIsVisible(true);
      return;
    }

    const handleMouseMove = (e: MouseEvent) => {
      setIsVisible(e.clientY < 50);
    };

    window.addEventListener("mousemove", handleMouseMove);
    return () => window.removeEventListener("mousemove", handleMouseMove);
  }, [isFullscreen]);

  return (
    <div
      id="navbar"
      className={`w-full h-12 border-b flex items-center gap-1 px-1 [-webkit-app-region: drag] transition-transform duration-200 ${isFullscreen && !isVisible ? "-translate-y-full" : "translate-y-0"}`}
      style={{ WebkitAppRegion: "drag" } as any}
    >
      <ButtonGroup>
        <ButtonGroupItem value="back" i18n="semio.sketchpad.navbar.back" onClick={navigateBack} disabled={!canGoBack}>
          <ArrowLeft size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="forward" i18n="semio.sketchpad.navbar.forward" onClick={navigateForward} disabled={!canGoForward}>
          <ArrowRight size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem
          value="up"
          i18n="semio.sketchpad.navbar.up"
          onClick={() => {
            if (upTarget) navigate(upTarget);
          }}
          disabled={isAtRoot}
        >
          <ArrowUp size={16} />
        </ButtonGroupItem>
      </ButtonGroup>

      <Navigation />

      <div className="flex items-center gap-1 ml-auto">
        <Search />
        <Focus />
        <PanelToggles />
        {toolbarConfig && (
          <Toggle
            i18n="semio.sketchpad.navbar.panelToggle.toolbar.show"
            i18nPressed="semio.sketchpad.navbar.panelToggle.toolbar.hide"
            pressed={!!visiblePanels.toolbar}
            onPressedChange={() => {
              commands[appType]?.togglePanel("toolbar");
            }}
          >
            <toolbarConfig.icon />
          </Toggle>
        )}
        <Toggle i18n="semio.sketchpad.navbar.fullscreen" i18nPressed="semio.sketchpad.navbar.exitFullscreen" pressed={isFullscreen} onPressedChange={toggleFullscreen}>
          <Fullscreen />
        </Toggle>
        {onWindowEvents && (
          <ToggleGroup type="single">
            <ToggleGroupItem value="minimize" i18n="semio.sketchpad.navbar.minimize" onClick={onWindowEvents.minimize}>
              <Minus size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="maximize" i18n="semio.sketchpad.navbar.maximize" onClick={onWindowEvents.maximize}>
              <Square size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="close" i18n="semio.sketchpad.navbar.close" onClick={onWindowEvents.close} className="hover:bg-danger">
              <X size={16} />
            </ToggleGroupItem>
          </ToggleGroup>
        )}
      </div>
    </div>
  );
}

function Navbar() {
  const isMobile = useIsMobile();
  const isFullscreen = useIsFullscreen();
  const isNavbarExpanded = useIsNavbarExpanded();
  const tooltip = useTooltip();

  const { onWindowEvents } = useSketchpadScope() as SketchpadScope;

  if (isMobile) {
    return <NavbarMobile isFullscreen={isFullscreen} isNavbarExpanded={isNavbarExpanded} i18n={tooltip} onWindowEvents={onWindowEvents} />;
  }

  return <NavbarDesktop isFullscreen={isFullscreen} isNavbarExpanded={isNavbarExpanded} i18n={tooltip} onWindowEvents={onWindowEvents} />;
}

export default Navbar;
