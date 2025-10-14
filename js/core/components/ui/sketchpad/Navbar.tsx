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

import {
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  Award,
  Box,
  ChevronDown,
  ChevronUp,
  Clock,
  Cloud,
  FileText,
  Fullscreen,
  HardDrive,
  Home,
  Info,
  Layout,
  MessageCircle,
  Minimize,
  Minus,
  Search as SearchIcon,
  Settings,
  Square,
  User,
  Wrench,
  X,
} from "lucide-react";
import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { generateUniqueName } from "../../../lib/utils";
import { Author, AuthorDiff, Connection, Design, DesignDiff, FileDiff, Guid, Piece, File as SemioFile, Type, TypeDiff } from "../../../semio";
import {
  EditorType,
  PanelVisibility,
  SketchpadScope,
  useDesignEditorCommands,
  useEditorPanelVisibility,
  useEditorType,
  useHomeCommands,
  useIsFullscreen,
  useIsMobile,
  useIsNavbarExpanded,
  useKitEditorCommands,
  useKits,
  useNavigation,
  useNavigationHistory,
  useSketchpadCommands,
  useSketchpadScope,
  useSketchpadStore,
  useTooltip,
  useTypeEditorCommands,
} from "../../../store";
import { Breadcrumb, BreadcrumbBreak, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from "../Breadcrumb";
import { ButtonGroup, ButtonGroupItem } from "../ButtonGroup";
import { CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandList } from "../Command";
import { Toggle } from "../Toggle";
import { ToggleGroup, ToggleGroupItem } from "../ToggleGroup";

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
}

export type PanelKey = "details" | "workbench" | "console" | "chat" | "settings";

export interface PanelSections {
  details: PanelSection[];
  workbench: PanelSection[];
  console: PanelSection[];
  chat: PanelSection[];
  settings: PanelSection[];
}

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
    console: [],
    chat: [],
    settings: [],
  });

  const addSection = useCallback((panelKey: PanelKey, section: PanelSection) => {
    setSections((prev) => ({
      ...prev,
      [panelKey]: [...prev[panelKey].filter((s) => s.id !== section.id), section].sort((a, b) => (a.order || 0) - (b.order || 0)),
    }));
  }, []);

  const removeSection = useCallback((panelKey: PanelKey, sectionId: string) => {
    setSections((prev) => ({
      ...prev,
      [panelKey]: prev[panelKey].filter((s) => s.id !== sectionId),
    }));
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
  tooltip: string;
  hotkey: string;
}

export const getPanelConfigs = (t: (key: string) => string): Record<EditorType, PanelDefinition[]> => ({
  [EditorType.HOME]: [
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  [EditorType.KIT]: [
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  [EditorType.DESIGN]: [
    { key: "workbench", icon: Wrench, tooltip: t("panels.workbench"), hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
  [EditorType.TYPE]: [
    { key: "workbench", icon: Wrench, tooltip: t("panels.workbench"), hotkey: "⌘J" },
    { key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" },
    { key: "chat", icon: MessageCircle, tooltip: t("panels.chat"), hotkey: "⌘[" },
    { key: "settings", icon: Settings, tooltip: t("panels.settings"), hotkey: "⌘," },
  ],
});

interface NavigationProps {
  mobile?: boolean;
}

const Navigation: FC<NavigationProps> = ({ mobile = false }) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams] = useSearchParams();
  const kits = useKits();
  const tooltip = useTooltip();
  const isMobile = useIsMobile();
  const isNavbarExpanded = useIsNavbarExpanded();

  // Parse URL path parts
  const pathParts = navigation.split("/").filter((p) => p);

  // Determine what kind of page we're on
  const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);

  // Check if we're in the /kits path
  const isKitsPath = pathParts[0] === "kits";

  // Home page filters (?kind=&name=&version=)
  const homeKind = !isKitsPath || pathParts.length === 1 ? (searchParams.get("kind") as "temporary" | "local" | "remote" | null) : null;
  const homeName = !isKitsPath || pathParts.length === 1 ? searchParams.get("name") : null;
  const homeVersion = !isKitsPath || pathParts.length === 1 ? searchParams.get("version") : null;

  console.log("[Navbar] Path parsing:", {
    navigation,
    pathParts,
    isKitsPath,
    homeKind,
    homeName,
    homeVersion,
    searchParamsString: searchParams.toString()
  });

  // Kit editor (/kits/:kitGuid or /kits/:kitGuid/designs/:design or /kits/:kitGuid/types/:type)
  const kitGuid = isKitsPath && pathParts[1] ? pathParts[1] : null;

  // Check if we're in design or type editor
  const secondPart = pathParts[2];
  const thirdPart = pathParts[3];
  const isDesignEditor = isKitsPath && secondPart === "designs" && thirdPart && isUuidPattern(thirdPart);
  const isTypeEditor = isKitsPath && secondPart === "types" && thirdPart && isUuidPattern(thirdPart);
  const itemGuid = isDesignEditor || isTypeEditor ? thirdPart : null;

  // Get artifact kind filters for kit editor (?kind=&name=&variant=&view=)
  const filteredKind = kitGuid && !isDesignEditor && !isTypeEditor ? (searchParams.get("kind") as "designs" | "types" | "qualities" | "files" | "authors" | null) : null;
  const filteredName = kitGuid && !isDesignEditor && !isTypeEditor ? searchParams.get("name") : null;
  const filteredVariant = kitGuid && !isDesignEditor && !isTypeEditor ? searchParams.get("variant") : null;
  const filteredView = kitGuid && !isDesignEditor && !isTypeEditor ? searchParams.get("view") : null;

  const isKitEditor = kitGuid && !isDesignEditor && !isTypeEditor;

  const kit = kits.find((k) => k.guid === kitGuid);
  const store = useSketchpadStore();

  // Determine kit storage kind
  const kitKind = useMemo(() => {
    if (!kitGuid || !store.hasKit(kitGuid)) return undefined;
    const kitStore = store.kit(kitGuid);
    if (!kitStore) return undefined;
    if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) return "remote";
    if (kitStore.isLocallyPersisted) return "local";
    return "temporary";
  }, [kitGuid, store]);

  // Kit kind items for breadcrumb
  const kitKindItems = [
    { label: <Clock size={16} />, tooltip: tooltip("breadcrumb.temporary"), href: "/kits?kind=temporary" },
    { label: <HardDrive size={16} />, tooltip: tooltip("breadcrumb.local"), href: "/kits?kind=local" },
    { label: <Cloud size={16} />, tooltip: tooltip("breadcrumb.remote"), href: "/kits?kind=remote" },
  ];

  // Filter kits by kind for the kit selector
  const kitItemsWithCreate = useMemo(() => {
    const items = kits
      .filter((k) => {
        if (!kitKind) return true;
        const ks = store.kit(k.guid);
        const kKind = ks.isLocallyPersisted && ks.isRemotelySynced ? "remote" : ks.isLocallyPersisted ? "local" : "temporary";
        return kKind === kitKind;
      })
      .map((k) => ({ label: k.name, href: `/kits/${k.guid}` }));

    // Add create option
    items.push({ label: "+ " + t("navbar.createKit"), href: "#create-kit" });
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
    { label: <Layout size={16} />, tooltip: tooltip("breadcrumb.designs"), kind: "designs", href: kitGuid ? `/kits/${kitGuid}?kind=designs` : "/kits?kind=designs" },
    { label: <Box size={16} />, tooltip: tooltip("breadcrumb.types"), kind: "types", href: kitGuid ? `/kits/${kitGuid}?kind=types` : "/kits?kind=types" },
    { label: <Award size={16} />, tooltip: tooltip("breadcrumb.qualities"), kind: "qualities", href: kitGuid ? `/kits/${kitGuid}?kind=qualities` : "/kits?kind=qualities" },
    { label: <FileText size={16} />, tooltip: tooltip("breadcrumb.files"), kind: "files", href: kitGuid ? `/kits/${kitGuid}?kind=files` : "/kits?kind=files" },
    { label: <User size={16} />, tooltip: tooltip("breadcrumb.authors"), kind: "authors", href: kitGuid ? `/kits/${kitGuid}?kind=authors` : "/kits?kind=authors" },
  ];

  // Get all designs and types as full objects (needed for create handlers)
  const allDesigns: Design[] = useMemo(() => {
    if (!kit?.designs) return [];
    return (kit.designs as any[]).filter((d): d is Design => typeof d === "object" && d.guid !== undefined);
  }, [kit?.designs]);

  const allTypes: Type[] = useMemo(() => {
    if (!kit?.types) return [];
    return (kit.types as any[]).filter((t): t is Type => typeof t === "object" && t.guid !== undefined);
  }, [kit?.types]);

  // Create handlers for various entity types
  const handleCreateKit = useCallback(() => {
    const guid = crypto.randomUUID();
    const now = new Date();
    const existingNames = kits.map((k) => k.name);
    const uniqueName = generateUniqueName(t("kit.defaultName"), existingNames);
    sketchpadCommands.createKit({
      guid,
      name: uniqueName,
      version: "",
      createdAt: now,
      updatedAt: now,
    });
    navigate(`/kits/${guid}`);
  }, [navigate, sketchpadCommands, kits, t]);

  const handleCreateDesign = useCallback(
    (name?: string) => {
      console.log("[Navbar] handleCreateDesign called", { name, kitCommands: !!kitCommands });
      if (!kitCommands) {
        console.warn("[Navbar] kitCommands is null, cannot create design");
        return;
      }
      const guid = crypto.randomUUID();
      const existingNames = allDesigns.map((d) => d.name);
      const uniqueName = name || generateUniqueName(t("design.defaultName"), existingNames);
      console.log("[Navbar] Creating design with name:", uniqueName);
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
      const uniqueName = name || generateUniqueName(t("type.defaultName"), existingNames);
      kitCommands.createType({ guid, name: uniqueName, variant: "", ports: [] });
      navigate(`/kits/${kitGuid}/types/${guid}`);
    },
    [kitCommands, kitGuid, navigate, allTypes, t],
  );

  const handleCreateVariant = useCallback(
    (designOrType: Design | Type, isType: boolean) => {
      console.log("[Navbar] handleCreateVariant called", { designOrType, isType, kitCommands: !!kitCommands });
      if (!kitCommands) {
        console.warn("[Navbar] kitCommands is null, cannot create variant");
        return;
      }
      const guid = crypto.randomUUID();
      if (!isType) {
        const d = designOrType as Design;
        const existingVariants = allDesigns.filter((design) => design.name === d.name).map((design) => design.variant || "");
        console.log("[Navbar] Creating design variant", { name: d.name, existingVariants });
        const uniqueVariant = generateUniqueName(t("design.newVariant"), existingVariants);
        console.log("[Navbar] Unique variant name:", uniqueVariant);
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
        const uniqueVariant = generateUniqueName(t("type.newVariant"), existingVariants);
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
      console.log("[Navbar] handleCreateView called", { design, kitCommands: !!kitCommands });
      if (!kitCommands) {
        console.warn("[Navbar] kitCommands is null, cannot create view");
        return;
      }
      const guid = crypto.randomUUID();
      const existingViews = allDesigns.filter((d) => d.name === design.name && (d.variant || "") === (design.variant || "")).map((d) => d.view || "");
      console.log("[Navbar] Creating design view", { name: design.name, variant: design.variant, existingViews });
      const uniqueView = generateUniqueName(t("design.newView"), existingViews);
      console.log("[Navbar] Unique view name:", uniqueView);
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

  // Create handler for filtered artifact kinds
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

  // Find current design or type
  const design = isDesignEditor ? allDesigns.find((d) => d.guid === itemGuid) : undefined;
  const type = isTypeEditor ? allTypes.find((t) => t.guid === itemGuid) : undefined;

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
    items.push({ label: "+ " + t("navbar.createDesign"), href: "#create-design" });
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
      label: variant || <span className="italic opacity-70">{t("design.defaultVariant")}</span>,
      href: `/kits/${kitGuid}/designs/${d.guid}`,
    }));
    items.push({ label: "+ " + t("navbar.createVariant"), href: "#create-variant" });
    return items;
  }, [design, allDesigns, kitGuid, t]);

  const designViewItems = useMemo(() => {
    if (!design) return [];
    const items = allDesigns
      .filter((d) => d.name === design.name && (d.variant || "") === (design.variant || ""))
      .map((d) => ({
        label: d.view || <span className="italic opacity-70">{t("design.defaultView")}</span>,
        href: `/kits/${kitGuid}/designs/${d.guid}`,
      }));
    items.push({ label: "+ " + t("navbar.createView"), href: "#create-view" });
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
    items.push({ label: "+ " + t("navbar.createType"), href: "#create-type" });
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
      label: variant || <span className="italic opacity-70">{t("type.defaultVariant")}</span>,
      href: `/kits/${kitGuid}/types/${typeObj.guid}`,
    }));
    items.push({ label: "+ " + t("navbar.createVariant"), href: "#create-variant" });
    return items;
  }, [type, allTypes, kitGuid, t]);

  // Build breadcrumb items for kit versions
  const kitVersionItems = useMemo(() => {
    if (!kit?.name) return [];
    // Get all kits with the same name
    const sameNameKits = kits.filter((k) => k.name === kit.name);
    return sameNameKits.map((k) => ({
      label: k.version || <span className="italic opacity-70">{t("kit.defaultVersion")}</span>,
      href: `/kits/${k.guid}`,
    }));
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
        href: `/kits?kind=${homeKind}&name=${encodeURIComponent(k.name)}`,
      }));
  }, [homeKind, kits, store]);

  // Build breadcrumb items for home page versions filtered by name
  const homeVersionsForName = useMemo(() => {
    if (!homeName || !homeKind) return [];
    return kits
      .filter((k) => k.name === homeName)
      .map((k) => ({
        label: k.version || <span className="italic opacity-70">{t("kit.defaultVersion")}</span>,
        href: k.version || "",
      }));
  }, [homeName, homeKind, kits, t]);

  // Build breadcrumb items for filtered names in kit editor
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

  // Build breadcrumb items for filtered variants in kit editor
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
      label: variant || <span className="italic opacity-70">{filteredKind === "designs" ? t("design.defaultVariant") : t("type.defaultVariant")}</span>,
      href: `/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(variant)}`,
    }));
  }, [kit, filteredKind, filteredName, allDesigns, allTypes, kitGuid, t]);

  // Build breadcrumb items for filtered views in kit editor
  const filteredViewItems = useMemo(() => {
    if (!kit || filteredKind !== "designs" || !filteredName || filteredVariant === null) return [];
    const viewSet = new Set<string>();

    allDesigns.forEach((d) => {
      if (d.name === filteredName && (d.variant || "") === filteredVariant) {
        viewSet.add(d.view || "");
      }
    });

    return Array.from(viewSet).map((view) => ({
      label: view || <span className="italic opacity-70">{t("design.defaultView")}</span>,
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
        <BreadcrumbItem tooltip={tooltip("navbar.home")}>
          <BreadcrumbLink onClick={() => navigate("/")} style={{ cursor: "pointer" }}>
            <Home size={16} />
          </BreadcrumbLink>
        </BreadcrumbItem>

        {/* Show separator with kind selector dropdown */}
        <BreadcrumbSeparator items={kitKindItems} tooltip={tooltip("navbar.kitKinds")} onNavigate={(href) => navigate(href)} />

        {/* If viewing a kit (or we have a selected home kind), show the kind breadcrumb */}
        {(kitGuid && kitKind) || homeKind ? (
          <>
            <BreadcrumbItem tooltip={tooltip(`breadcrumb.${kitKind || homeKind}`)}>
              <BreadcrumbLink onClick={() => navigate(`/kits?kind=${kitKind || homeKind}`)} style={{ cursor: "pointer" }}>
                {(kitKind === "temporary" || homeKind === "temporary") && <Clock size={16} />}
                {(kitKind === "local" || homeKind === "local") && <HardDrive size={16} />}
                {(kitKind === "remote" || homeKind === "remote") && <Cloud size={16} />}
              </BreadcrumbLink>
            </BreadcrumbItem>

            {/* Show kits dropdown when on home page with kind selected */}
            {!kitGuid && (
              <BreadcrumbSeparator
                items={homeKitsForKind}
                tooltip={tooltip("navbar.kits")}
                onNavigate={(href) => navigate(href)}
              />
            )}

            {homeName && (
              <>
                <BreadcrumbItem tooltip={tooltip("navbar.kitName")}>
                  <BreadcrumbLink onClick={() => navigate(`/kits?kind=${homeKind}&name=${encodeURIComponent(homeName)}`)} style={{ cursor: "pointer" }}>
                    {homeName}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator
                  items={homeVersionsForName}
                  tooltip={tooltip("navbar.versions")}
                  onNavigate={(version) => navigate(`/kits?kind=${homeKind}&name=${encodeURIComponent(homeName)}&version=${encodeURIComponent(version)}`)}
                />
              </>
            )}
            {homeVersion && (
              <>
                <BreadcrumbItem tooltip={tooltip("navbar.kitVersion")}>
                  <BreadcrumbLink style={{ cursor: "default" }}>{homeVersion || <span className="italic opacity-70">{t("kit.defaultVersion")}</span>}</BreadcrumbLink>
                </BreadcrumbItem>
              </>
            )}
            {kitGuid && (
              <>
                <BreadcrumbSeparator
                  items={kitItemsWithCreate}
                  tooltip={tooltip("navbar.kits")}
                  onNavigate={(href) => {
                    if (href === "#create-kit") handleCreateKit();
                    else navigate(href);
                  }}
                />
                <BreadcrumbItem tooltip={tooltip("navbar.kit")}>
                  <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}`)} style={{ cursor: "pointer" }}>
                    {kit?.name || kitGuid}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator
                  items={kitVersionItems}
                  tooltip={tooltip("navbar.versions")}
                  onNavigate={(href) => {
                    if (href.startsWith("/kits/")) navigate(href);
                    else navigate(`/kits?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}&version=${encodeURIComponent(href)}`);
                  }}
                />
                <BreadcrumbItem tooltip={tooltip("navbar.kitVersion")}>
                  <BreadcrumbLink onClick={() => navigate(`/kits?kind=${kitKind}&name=${encodeURIComponent(kit?.name || "")}&version=${encodeURIComponent(kit?.version || "")}`)} style={{ cursor: "pointer" }}>
                    {kit?.version || <span className="italic opacity-70">{t("kit.defaultVersion")}</span>}
                  </BreadcrumbLink>
                </BreadcrumbItem>
              </>
            )}
          </>
        ) : null}
        {isKitEditor && (
          <>
            <BreadcrumbSeparator items={artifactKinds} tooltip={tooltip("navbar.artifacts")} onNavigate={(href) => navigate(href)} />
            {filteredKind && (
              <>
                <BreadcrumbItem tooltip={tooltip(`breadcrumb.${filteredKind}`)}>
                  <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}`)} style={{ cursor: "pointer" }}>
                    {filteredKind === "designs" && <Layout size={16} />}
                    {filteredKind === "types" && <Box size={16} />}
                    {filteredKind === "qualities" && <Award size={16} />}
                    {filteredKind === "files" && <FileText size={16} />}
                    {filteredKind === "authors" && <User size={16} />}
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbBreak />
                <BreadcrumbSeparator
                  items={filteredNameItems}
                  tooltip={tooltip("navbar.selectName")}
                  onNavigate={(href) => navigate(href)}
                />
                {filteredName !== null && (
                  <>
                    <BreadcrumbItem tooltip={tooltip("navbar.name")}>
                      <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}`)} style={{ cursor: "pointer" }}>
                        {filteredName}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator
                      items={filteredVariantItems}
                      tooltip={tooltip("navbar.selectVariant")}
                      onNavigate={(href) => navigate(href)}
                    />
                  </>
                )}
                {filteredName !== null && filteredVariant !== null && (
                  <>
                    <BreadcrumbItem tooltip={tooltip("navbar.variant")}>
                      <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(filteredVariant)}`)} style={{ cursor: "pointer" }}>
                        {filteredVariant || <span className="italic opacity-70">{t("design.defaultVariant")}</span>}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator
                      items={filteredViewItems}
                      tooltip={tooltip("navbar.selectView")}
                      onNavigate={(href) => navigate(href)}
                    />
                  </>
                )}
                {filteredName !== null && filteredVariant !== null && filteredView !== null && (
                  <>
                    <BreadcrumbItem tooltip={tooltip("navbar.view")}>
                      <BreadcrumbLink
                        onClick={() => navigate(`/kits/${kitGuid}?kind=${filteredKind}&name=${encodeURIComponent(filteredName)}&variant=${encodeURIComponent(filteredVariant)}&view=${encodeURIComponent(filteredView)}`)}
                        style={{ cursor: "pointer" }}
                      >
                        {filteredView || <span className="italic opacity-70">{t("design.defaultView")}</span>}
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                  </>
                )}
              </>
            )}
          </>
        )}
        {isDesignEditor && design && (
          <>
            <BreadcrumbSeparator items={artifactKinds} tooltip={tooltip("navbar.artifacts")} onNavigate={(href) => navigate(href)} />
            <BreadcrumbItem tooltip={tooltip("breadcrumb.designs")}>
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=designs`)} style={{ cursor: "pointer" }}>
                <Layout size={16} />
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbBreak />
            <BreadcrumbSeparator
              items={designNameItems}
              tooltip={tooltip("navbar.selectDesign")}
              onNavigate={(href) => {
                if (href === "#create-design") handleCreateDesign();
                else navigate(href);
              }}
            />
            <BreadcrumbItem tooltip={tooltip("navbar.design")}>
              <BreadcrumbLink style={{ cursor: "default" }}>{design.name}</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={designVariantItems}
              tooltip={tooltip("navbar.selectVariant")}
              onNavigate={(href) => {
                console.log("[Navbar] Variant breadcrumb navigate", { href, design });
                if (href === "#create-variant") handleCreateVariant(design, false);
                else navigate(href);
              }}
            />
            <BreadcrumbItem tooltip={tooltip("navbar.variant")}>
              <BreadcrumbLink onClick={() => navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(design.name)}&variant=${encodeURIComponent(design.variant || "")}`)} style={{ cursor: "pointer" }}>
                {design.variant || <span className="italic opacity-70">{t("design.defaultVariant")}</span>}
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator
              items={designViewItems}
              tooltip={tooltip("navbar.selectView")}
              onNavigate={(href) => {
                console.log("[Navbar] View breadcrumb navigate", { href, design });
                if (href === "#create-view") handleCreateView(design);
                else navigate(href);
              }}
            />
            <BreadcrumbItem tooltip={tooltip("navbar.view")}>
              <BreadcrumbLink
                onClick={() => navigate(`/kits/${kitGuid}?kind=designs&name=${encodeURIComponent(design.name)}&variant=${encodeURIComponent(design.variant || "")}&view=${encodeURIComponent(design.view || "")}`)}
                style={{ cursor: "pointer" }}
              >
                {design.view || <span className="italic opacity-70">{t("design.defaultView")}</span>}
              </BreadcrumbLink>
            </BreadcrumbItem>
          </>
        )}
      </BreadcrumbList>
    </Breadcrumb>
  );
};

const Search: FC = ({}) => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const [open, setOpen] = useState(false);

  return (
    <>
      <Toggle tooltip={tooltip("navbar.search")} pressed={open} onPressedChange={setOpen}>
        <SearchIcon size={16} />
      </Toggle>
      <CommandDialog title={t("navbar.searchTitle")} description={t("navbar.searchDescription")} open={open} onOpenChange={setOpen}>
        <CommandInput placeholder={t("navbar.searchPlaceholder")} />
        <CommandList>
          <CommandEmpty>{t("navbar.noResults")}</CommandEmpty>
          <CommandGroup heading={t("navbar.suggestions")}>{/* TODO: Add command items here */}</CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
};

const PanelToggles: FC = ({}) => {
  const { t } = useTranslation();
  const { kit, design, type } = useParams();
  const editorType = useEditorType();
  const panelConfig = getPanelConfigs(t)[editorType];
  const visiblePanels = useEditorPanelVisibility();
  const homeCommands = useHomeCommands();
  const isValidKit = kit && !["temporary", "local", "remote"].includes(kit);
  const kitEditorCommands = useKitEditorCommands(isValidKit ? { kit } : undefined);
  const designEditorCommands = useDesignEditorCommands(isValidKit && design ? { kit, design } : undefined);
  const typeEditorCommands = useTypeEditorCommands(isValidKit && type ? { kit, type } : undefined);
  const commands = {
    [EditorType.HOME]: homeCommands,
    [EditorType.KIT]: kitEditorCommands,
    [EditorType.DESIGN]: designEditorCommands,
    [EditorType.TYPE]: typeEditorCommands,
  };
  const isMobile = useIsMobile();

  if (panelConfig.length === 0) return null;

  // Exclusive panels: only one can be open at a time (details, chat, settings)
  const exclusivePanels = ["details", "chat", "settings"];
  const exclusiveConfigs = panelConfig.filter((p) => exclusivePanels.includes(p.key));
  const regularConfigs = panelConfig.filter((p) => !exclusivePanels.includes(p.key));

  // Find currently active exclusive panel
  const activeExclusivePanel = exclusiveConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key || exclusiveConfigs[0]?.key || "";
  const isAnyExclusivePanelOpen = exclusiveConfigs.some((p) => visiblePanels[p.key as keyof PanelVisibility]);

  const handleToggle = (panelKey: keyof PanelVisibility) => {
    const togglePanel = commands[editorType]?.togglePanel || (() => {});
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
      if (!current && exclusivePanels.includes(panelKey)) {
        (exclusivePanels as Array<keyof PanelVisibility>).forEach((p) => {
          if (p !== panelKey && visiblePanels[p]) {
            togglePanel(p);
          }
        });
      }
    }
    togglePanel(panelKey);
  };

  const handleExclusivePressedChange = (pressed: boolean) => {
    const togglePanel = commands[editorType]?.togglePanel || (() => {});
    if (pressed) {
      if (activeExclusivePanel && !visiblePanels[activeExclusivePanel as keyof PanelVisibility]) {
        handleToggle(activeExclusivePanel as keyof PanelVisibility);
      }
    } else {
      const openPanel = exclusiveConfigs.find((p) => visiblePanels[p.key as keyof PanelVisibility]);
      if (openPanel) {
        togglePanel(openPanel.key as keyof PanelVisibility);
      }
    }
  };

  const handleExclusiveValueChange = (value: string | undefined) => {
    const togglePanel = commands[editorType]?.togglePanel || (() => {});
    if (!value) return;

    (exclusivePanels as Array<keyof PanelVisibility>).forEach((p) => {
      const isOpen = visiblePanels[p];
      const shouldOpen = p === value;

      if (isOpen && !shouldOpen) {
        togglePanel(p);
      } else if (!isOpen && shouldOpen) {
        togglePanel(p);
      }
    });
  };

  return (
    <ToggleGroup type="multiple" value={[...regularConfigs.filter((p) => visiblePanels[p.key as keyof PanelVisibility]).map((p) => p.key), ...(isAnyExclusivePanelOpen ? [activeExclusivePanel] : [])]}>
      {/* Regular toggles (workbench) */}
      {regularConfigs.map(({ key, icon: Icon, tooltip, hotkey }) => (
        <ToggleGroupItem
          key={key}
          value={key}
          tooltip={tooltip}
          hotkey={hotkey}
          onClick={() => {
            handleToggle(key as keyof PanelVisibility);
          }}
        >
          <Icon />
        </ToggleGroupItem>
      ))}

      {/* Dropdown toggle for exclusive panels (details, chat, settings) */}
      {exclusiveConfigs.length > 0 && (
        <Toggle
          type="dropdown"
          pressed={isAnyExclusivePanelOpen}
          onPressedChange={handleExclusivePressedChange}
          value={activeExclusivePanel}
          onValueChange={handleExclusiveValueChange}
          tooltip={exclusiveConfigs.find((p) => p.key === activeExclusivePanel)?.tooltip}
          dropdownTooltip={t("navbar.changePanelType")}
          className={regularConfigs.length > 0 ? "border-0 border-l" : "border-0"}
          items={exclusiveConfigs.map(({ key, icon: Icon, tooltip, hotkey }) => ({
            value: key,
            label: <Icon />,
            tooltip,
            hotkey,
          }))}
        />
      )}
    </ToggleGroup>
  );
};

interface NavbarProps {}

const Navbar: FC<NavbarProps> = ({}) => {
  const { t } = useTranslation();
  const { onWindowEvents } = useSketchpadScope() as SketchpadScope;
  const isFullscreen = useIsFullscreen();
  const isNavbarExpanded = useIsNavbarExpanded();
  const isMobile = useIsMobile();
  const tooltip = useTooltip();
  const { toggleFullscreen, toggleNavbarExpanded, navigateBack, navigateForward, setIsMobile } = useSketchpadCommands();
  const [isVisible, setIsVisible] = useState(true);
  const [searchOpen, setSearchOpen] = useState(false);
  const navigate = useNavigate();
  const currentPath = useNavigation();
  const { canGoBack, canGoForward } = useNavigationHistory();

  // Always call hooks unconditionally
  const editorType = useEditorType();
  const panelConfig = getPanelConfigs(t)[editorType];
  const visiblePanels = useEditorPanelVisibility();
  const { kit, design, type } = useParams();
  const homeCommands = useHomeCommands();
  const isValidKit = kit && !["temporary", "local", "remote"].includes(kit);
  const kitEditorCommands = useKitEditorCommands(isValidKit ? { kit } : undefined);
  const designEditorCommands = useDesignEditorCommands(isValidKit && design ? { kit, design } : undefined);
  const typeEditorCommands = useTypeEditorCommands(isValidKit && type ? { kit, type } : undefined);
  const commands = {
    [EditorType.HOME]: homeCommands,
    [EditorType.KIT]: kitEditorCommands,
    [EditorType.DESIGN]: designEditorCommands,
    [EditorType.TYPE]: typeEditorCommands,
  };

  const isAtRoot = currentPath === "/";

  // Find the currently active panel (used by mobile)
  // Default to first panel if none is open
  const activePanel = panelConfig.find((p) => visiblePanels[p.key as keyof PanelVisibility])?.key || panelConfig[0]?.key || "";
  const isAnyPanelOpen = panelConfig.some((p) => visiblePanels[p.key as keyof PanelVisibility]);

  const handleMobilePanelToggle = (pressed: boolean) => {
    const togglePanel = commands[editorType]?.togglePanel || (() => {});
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
    const togglePanel = commands[editorType]?.togglePanel || (() => {});
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
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };
    checkMobile();
    window.addEventListener("resize", checkMobile);
    return () => window.removeEventListener("resize", checkMobile);
  }, [setIsMobile]);

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

  if (isMobile) {
    return (
      <div id="navbar" className={`w-full border-b flex flex-col [-webkit-app-region: drag] transition-transform duration-200 ${isFullscreen && !isVisible ? "-translate-y-full" : "translate-y-0"}`} style={{ WebkitAppRegion: "drag" }}>
        {/* Unexpanded navbar */}
        <div className="h-12 flex items-center justify-between px-1 gap-1">
          <ButtonGroup>
            <ButtonGroupItem value="back" tooltip={tooltip("navbar.back")} onClick={navigateBack} disabled={!canGoBack}>
              <ArrowLeft size={16} />
            </ButtonGroupItem>
            <ButtonGroupItem value="forward" tooltip={tooltip("navbar.forward")} onClick={navigateForward} disabled={!canGoForward}>
              <ArrowRight size={16} />
            </ButtonGroupItem>
            <ButtonGroupItem value="up" tooltip={tooltip("navbar.up")} onClick={() => navigate("/")} disabled={isAtRoot}>
              <ArrowUp size={16} />
            </ButtonGroupItem>
          </ButtonGroup>

          {/* Single dropdown toggle for all panels on mobile */}
          {panelConfig.length > 0 && (
            <Toggle
              type="dropdown"
              pressed={isAnyPanelOpen}
              onPressedChange={handleMobilePanelToggle}
              value={activePanel}
              onValueChange={handleMobilePanelChange}
              tooltip={panelConfig.find((p) => p.key === activePanel)?.tooltip || t("navbar.panels")}
              dropdownTooltip={t("navbar.changePanelType")}
              items={panelConfig.map(({ key, icon: Icon, tooltip, hotkey }) => ({
                value: key,
                label: <Icon />,
                tooltip,
                hotkey,
              }))}
            />
          )}

          <div className="flex gap-1">
            <Toggle tooltip={tooltip("navbar.search")} pressed={searchOpen} onPressedChange={setSearchOpen}>
              <SearchIcon size={16} />
            </Toggle>
            <Toggle tooltip={tooltip(isFullscreen ? "navbar.exitFullscreen" : "navbar.fullscreen")} pressed={isFullscreen} onPressedChange={toggleFullscreen}>
              {isFullscreen ? <Minimize size={16} /> : <Fullscreen size={16} />}
            </Toggle>
            <Toggle tooltip={tooltip(isNavbarExpanded ? "navbar.collapse" : "navbar.expand")} pressed={isNavbarExpanded} onPressedChange={toggleNavbarExpanded}>
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
                <ButtonGroupItem value="minimize" tooltip={tooltip("navbar.minimize")} onClick={onWindowEvents.minimize}>
                  <Minus size={16} />
                </ButtonGroupItem>
                <ButtonGroupItem value="maximize" tooltip={tooltip("navbar.maximize")} onClick={onWindowEvents.maximize}>
                  <Square size={16} />
                </ButtonGroupItem>
                <ButtonGroupItem value="close" tooltip={tooltip("navbar.close")} onClick={onWindowEvents.close}>
                  <X size={16} />
                </ButtonGroupItem>
              </ButtonGroup>
            )}
          </div>
        )}

        {/* Search dialog */}
        <CommandDialog title={t("navbar.searchTitle")} description={t("navbar.searchDescription")} open={searchOpen} onOpenChange={setSearchOpen}>
          <CommandInput placeholder={t("navbar.searchPlaceholder")} />
          <CommandList>
            <CommandEmpty>{t("navbar.noResults")}</CommandEmpty>
            <CommandGroup heading={t("navbar.suggestions")}>{/* TODO: Add command items here */}</CommandGroup>
          </CommandList>
        </CommandDialog>
      </div>
    );
  }

  return (
    <div
      id="navbar"
      className={`w-full h-12 border-b flex items-center gap-1 px-1 [-webkit-app-region: drag] transition-transform duration-200 ${isFullscreen && !isVisible ? "-translate-y-full" : "translate-y-0"}`}
      style={{ WebkitAppRegion: "drag" }}
    >
      <ButtonGroup>
        <ButtonGroupItem value="back" tooltip={t("navbar.back")} onClick={navigateBack} disabled={!canGoBack}>
          <ArrowLeft size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="forward" tooltip={t("navbar.forward")} onClick={navigateForward} disabled={!canGoForward}>
          <ArrowRight size={16} />
        </ButtonGroupItem>
        <ButtonGroupItem value="up" tooltip={t("navbar.up")} onClick={() => navigate("/")} disabled={isAtRoot}>
          <ArrowUp size={16} />
        </ButtonGroupItem>
      </ButtonGroup>

      <Navigation />

      <div className="flex items-center gap-1 ml-auto">
        <Search />
        <PanelToggles />
        <Toggle tooltip={isFullscreen ? t("navbar.exitFullscreen") : t("navbar.fullscreen")} pressed={isFullscreen} onPressedChange={toggleFullscreen}>
          {isFullscreen ? <Minimize /> : <Fullscreen />}
        </Toggle>
        {onWindowEvents && (
          <ToggleGroup type="single">
            <ToggleGroupItem value="minimize" tooltip={t("navbar.minimize")} onClick={onWindowEvents.minimize}>
              <Minus size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="maximize" tooltip={t("navbar.maximize")} onClick={onWindowEvents.maximize}>
              <Square size={16} />
            </ToggleGroupItem>
            <ToggleGroupItem value="close" tooltip={t("navbar.close")} onClick={onWindowEvents.close} className="hover:bg-danger">
              <X size={16} />
            </ToggleGroupItem>
          </ToggleGroup>
        )}
      </div>
    </div>
  );
};
export default Navbar;
