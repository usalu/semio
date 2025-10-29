// #region Header

// App.tsx

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

import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import { ArrowDown, ArrowUp, Award, Box, FileText, Layout, Plus, User } from "lucide-react";
import React, { FC, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ScrollArea } from "../../../elements/aggregation/ScrollArea";
import { Action } from "../../../elements/input/Action";
import { Input } from "../../../elements/input/Input";
import { Toggle } from "../../../elements/input/Toggle";
import i18n from "../../../i18n";
import { Author, Design, generateUniqueName, guid, Kit, Quality, File as SemioFile, Type } from "../../../semio";
import { Canvas, Window } from "../../Canvas";
import { useAddPanelSection, useFocus, useRemovePanelSection } from "../../Navbar";
import { useAppType, useIsMobile, useKit, useKitCommands, useKitScope, useNavigation, useSketchpadCommands, useSketchpadStore } from "../../store";
import { DesignSection, KitSection, MultipleArtifactsSection, TypeSection } from "./panels/Details";
import { KitAppState, useKitApp, useKitAppCommands } from "./store";

type ArtifactKind = "designs" | "types" | "qualities" | "files" | "authors";

type TableRow = {
  id: string;
  kind: ArtifactKind;
  artifact: string;
  authors: string;
  updatedAt: string;
  createdAt: string;
  level: number;
  parentId?: string;
  hasChildren: boolean;
  isExpanded: boolean;
  data: Design | Type | Quality | SemioFile | Author;
};

const ChevronRight: FC<{ className?: string }> = ({ className }) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={className}>
    <path d="m9 18 6-6-6-6" />
  </svg>
);

const ChevronDown: FC<{ className?: string }> = ({ className }) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={className}>
    <path d="m6 9 6 6 6-6" />
  </svg>
);

const AppContent: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const params = useParams();
  const [searchParams, setSearchParams] = useSearchParams();

  const kitScope = useKitScope();
  const sketchpadStore = useSketchpadStore();
  const hasKit = kitScope?.guid ? sketchpadStore.hasKit(kitScope.guid) : false;

  const kit = useKit() as Kit;
  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitAppCommands = useKitAppCommands();
  const kitApp = useKitApp() as KitAppState;
  const isMobile = useIsMobile();
  
  const [isDragOver, setIsDragOver] = React.useState(false);

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const appType = useAppType();

  // Early return if no kit is loaded
  if (!hasKit) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">{t("semio.sketchpad.app.kit.noKitLoaded")}</p>
      </div>
    );
  }

  // Get filters from search params (?kind=&name=&variant=&view=)
  const selectedKind = searchParams.get("kind") as ArtifactKind | null;
  const selectedName = searchParams.get("name");
  const selectedVariant = searchParams.get("variant");
  const selectedView = searchParams.get("view");

  // Get concepts and search from search params
  const selectedConcepts = searchParams.getAll("c");
  const searchQuery = searchParams.get("q") || "";

  // Get selection parameter for auto-selecting designs/types
  const selectParam = searchParams.get("select");
  const expandedRowsArray = kitApp?.expandedRows || [];
  const expandedRows = new Set(expandedRowsArray);

  const selection = {
    types: kitApp?.selection?.types || [],
    designs: kitApp?.selection?.designs || [],
    qualities: kitApp?.selection?.qualities || [],
    files: kitApp?.selection?.files || [],
    authors: kitApp?.selection?.authors || [],
  };
  const sortColumn = kitApp?.sortColumn;
  const sortDirection = kitApp?.sortDirection || "asc";

  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kit?.designs?.forEach((d: Design) => d.concepts?.forEach((c: string) => conceptSet.add(c)));
    return Array.from(conceptSet).sort();
  }, [kit?.designs]);

  // Collect unique names for the selected kind (or unified when no kind selected)
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();
    if (!selectedKind || selectedKind === "designs") {
      kit?.designs?.forEach((d: Design) => nameSet.add(d.name));
    }
    if (!selectedKind || selectedKind === "types") {
      kit?.types?.forEach((t: Type) => nameSet.add(t.name));
    }
    return Array.from(nameSet).sort();
  }, [kit?.designs, kit?.types, selectedKind]);

  // Collect unique variants for the selected name
  const uniqueVariants = useMemo(() => {
    if (!selectedName) return [];
    const variantSet = new Set<string>();
    if (!selectedKind || selectedKind === "designs") {
      kit?.designs?.forEach((d: Design) => {
        if (d.name === selectedName) {
          variantSet.add(d.variant || "");
        }
      });
    }
    if (!selectedKind || selectedKind === "types") {
      kit?.types?.forEach((t: Type) => {
        if (t.name === selectedName) {
          variantSet.add(t.variant || "");
        }
      });
    }
    return Array.from(variantSet).sort();
  }, [kit?.designs, kit?.types, selectedKind, selectedName]);

  // Collect unique views for the selected name and variant (only for designs)
  const uniqueViews = useMemo(() => {
    if (!selectedName || selectedVariant === null || selectedKind !== "designs") return [];
    const viewSet = new Set<string>();
    kit?.designs?.forEach((d: Design) => {
      if (d.name === selectedName && (d.variant || "") === selectedVariant) {
        viewSet.add(d.view || "");
      }
    });
    return Array.from(viewSet).sort();
  }, [kit?.designs, selectedKind, selectedName, selectedVariant]);

  useEffect(() => {
    if (appType !== "kit") {
      return;
    }

    const selection = kitApp?.selection;
    const typesCount = selection?.types?.length || 0;
    const designsCount = selection?.designs?.length || 0;
    const qualitiesCount = selection?.qualities?.length || 0;
    const filesCount = selection?.files?.length || 0;
    const authorsCount = selection?.authors?.length || 0;
    const totalSelectedKinds = [typesCount > 0, designsCount > 0, qualitiesCount > 0, filesCount > 0, authorsCount > 0].filter(Boolean).length;

    removeSection("details", "kit-multiple-artifacts");
    removeSection("details", "kit-design");
    removeSection("details", "kit-type");
    removeSection("details", "kit-details");

    if (totalSelectedKinds > 1) {
      addSection("details", {
        id: "kit-multiple-artifacts",
        label: t("artifacts.multiple"),
        order: 0,
        defaultOpen: true,
        content: () => <MultipleArtifactsSection />,
      });
    }

    if (designsCount > 0 && totalSelectedKinds === 1) {
      addSection("details", {
        id: "kit-design",
        label: designsCount === 1 ? t("semio.sketchpad.app.design.title") : t("semio.sketchpad.app.kit.designs.multipleTitle"),
        order: 10,
        defaultOpen: true,
        content: () => <DesignSection />,
      });
    }

    if (typesCount > 0 && totalSelectedKinds === 1) {
      addSection("details", {
        id: "kit-type",
        label: typesCount === 1 ? t("semio.sketchpad.app.type.title") : t("semio.sketchpad.app.kit.types.multipleTitle"),
        order: 20,
        defaultOpen: true,
        content: () => <TypeSection />,
      });
    }

    addSection("details", {
      id: "kit-details",
      label: t("semio.sketchpad.app.kit.title"),
      order: 100,
      defaultOpen: true,
      content: () => <KitSection />,
    });

    return () => {
      removeSection("details", "kit-multiple-artifacts");
      removeSection("details", "kit-design");
      removeSection("details", "kit-type");
      removeSection("details", "kit-details");
    };
  }, [addSection, removeSection, appType, t, kitApp?.selection]);

  // Auto-select design/type when select parameter is present
  useEffect(() => {
    if (!selectParam) return;

    if (selectedKind === "designs") {
      const design = kit.designs?.find((d: Design) => d.guid === selectParam);
      if (design) {
        kitAppCommands.selectDesign(selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    } else if (selectedKind === "types") {
      const type = kit.types?.find((t: Type) => t.guid === selectParam);
      if (type) {
        kitAppCommands.selectType(selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectParam, selectedKind]);

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date) => {
      if (!date) return "";
      const parsedDate = date instanceof Date ? date : new Date(date);
      if (isNaN(parsedDate.getTime())) return "";
      return formatDistanceToNow(parsedDate, { addSuffix: true, locale });
    };

    if (!selectedKind || selectedKind === "designs") {
      const designGroups = new Map<string, Design[]>();
      kit.designs?.forEach((design: Design) => {
        const key = design.name;
        if (!designGroups.has(key)) designGroups.set(key, []);
        designGroups.get(key)!.push(design);
      });

      designGroups.forEach((designs, name) => {
        // Apply name filter
        if (selectedName && name !== selectedName) return;

        const filteredDesigns = designs.filter((d) => {
          if (selectedConcepts.length > 0 && !d.concepts?.some((c) => selectedConcepts.includes(c))) return false;
          if (searchQuery && !name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
          // Apply variant filter
          if (selectedVariant && (d.variant || "") !== selectedVariant) return false;
          // Apply view filter
          if (selectedView && (d.view || "") !== selectedView) return false;
          return true;
        });

        if (filteredDesigns.length === 0) return;

        // Find the default design (default variant and default view)
        const defaultDesign = filteredDesigns.find((d) => (!d.variant || d.variant === "") && (!d.view || d.view === ""));

        // Group by variant
        const variantGroups = new Map<string, Design[]>();
        filteredDesigns.forEach((design) => {
          const variantKey = design.variant || "";
          if (!variantGroups.has(variantKey)) variantGroups.set(variantKey, []);
          variantGroups.get(variantKey)!.push(design);
        });

        const hasMultipleVariants = variantGroups.size > 1 || (variantGroups.size === 1 && Array.from(variantGroups.keys())[0] !== "");
        const defaultVariantDesigns = variantGroups.get("") || [];
        const defaultVariantNonDefaultViews = defaultVariantDesigns.filter((d) => d.view && d.view !== "");
        const hasDefaultVariantViews = defaultVariantNonDefaultViews.length > 0;

        const parentId = `design-${name}`;

        // Parent row shows the default variant and default view
        result.push({
          id: parentId,
          kind: "designs",
          artifact: name,
          authors: defaultDesign?.authors?.join(", ") || "",
          updatedAt: defaultDesign ? formatDate(defaultDesign.updatedAt) : "",
          createdAt: defaultDesign ? formatDate(defaultDesign.createdAt) : "",
          level: 0,
          hasChildren: hasMultipleVariants || hasDefaultVariantViews,
          isExpanded: expandedRows.has(parentId),
          data: defaultDesign || filteredDesigns[0],
        });

        if (expandedRows.has(parentId)) {
          // Show views for default variant if they exist
          if (hasDefaultVariantViews) {
            defaultVariantNonDefaultViews.forEach((design) => {
              const viewId = `${parentId}-default-${design.view}`;
              result.push({
                id: viewId,
                kind: "designs",
                artifact: `View: ${design.view}`,
                authors: design.authors?.join(", ") || "",
                updatedAt: formatDate(design.updatedAt),
                createdAt: formatDate(design.createdAt),
                level: 1,
                parentId,
                hasChildren: false,
                isExpanded: false,
                data: design,
              });
            });
          }

          // Only show variant rows if there are non-default variants
          if (hasMultipleVariants) {
            variantGroups.forEach((variantDesigns, variant) => {
              // Skip default variant as it's shown in parent row
              if (variant === "") return;

              const variantId = `${parentId}-${variant}`;
              const viewGroups = variantDesigns.filter((d) => d.view && d.view !== "");
              const hasMultipleViews = viewGroups.length > 0;
              const defaultViewDesign = variantDesigns.find((d) => !d.view || d.view === "");

              result.push({
                id: variantId,
                kind: "designs",
                artifact: `Variant: ${variant}`,
                authors: defaultViewDesign?.authors?.join(", ") || "",
                updatedAt: defaultViewDesign ? formatDate(defaultViewDesign.updatedAt) : "",
                createdAt: defaultViewDesign ? formatDate(defaultViewDesign.createdAt) : "",
                level: 1,
                parentId,
                hasChildren: hasMultipleViews,
                isExpanded: expandedRows.has(variantId),
                data: defaultViewDesign || variantDesigns[0],
              });

              // Only show view rows for non-default views
              if (expandedRows.has(variantId) && hasMultipleViews) {
                viewGroups.forEach((design) => {
                  const viewId = `${variantId}-${design.view}`;
                  result.push({
                    id: viewId,
                    kind: "designs",
                    artifact: `View: ${design.view}`,
                    authors: design.authors?.join(", ") || "",
                    updatedAt: formatDate(design.updatedAt),
                    createdAt: formatDate(design.createdAt),
                    level: 2,
                    parentId: variantId,
                    hasChildren: false,
                    isExpanded: false,
                    data: design,
                  });
                });
              }
            });
          }
        }
      });
    }

    if (!selectedKind || selectedKind === "types") {
      const typeGroups = new Map<string, Type[]>();
      kit.types?.forEach((type: Type) => {
        const key = type.name;
        if (!typeGroups.has(key)) typeGroups.set(key, []);
        typeGroups.get(key)!.push(type);
      });

      typeGroups.forEach((types, name) => {
        // Apply name filter
        if (selectedName && name !== selectedName) return;

        const filteredTypes = types.filter((t) => {
          if (searchQuery && !name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
          // Apply variant filter
          if (selectedVariant && (t.variant || "") !== selectedVariant) return false;
          return true;
        });

        if (filteredTypes.length === 0) return;

        // Find the default type (default variant)
        const defaultType = filteredTypes.find((t) => !t.variant || t.variant === "");

        // Check if there are non-default variants
        const hasMultipleVariants = filteredTypes.some((t) => t.variant && t.variant !== "");

        const parentId = `type-${name}`;

        // Parent row shows the default variant
        result.push({
          id: parentId,
          kind: "types",
          artifact: name,
          authors: defaultType?.authors?.join(", ") || "",
          updatedAt: defaultType ? formatDate(defaultType.updatedAt) : "",
          createdAt: defaultType ? formatDate(defaultType.createdAt) : "",
          level: 0,
          hasChildren: hasMultipleVariants,
          isExpanded: expandedRows.has(parentId),
          data: defaultType || filteredTypes[0],
        });

        // Only show variant rows if there are non-default variants
        if (expandedRows.has(parentId) && hasMultipleVariants) {
          filteredTypes.forEach((type) => {
            // Skip default variant as it's shown in parent row
            if (!type.variant || type.variant === "") return;

            const variantId = `${parentId}-${type.variant}`;
            result.push({
              id: variantId,
              kind: "types",
              artifact: `Variant: ${type.variant}`,
              authors: type.authors?.join(", ") || "",
              updatedAt: formatDate(type.updatedAt),
              createdAt: formatDate(type.createdAt),
              level: 1,
              parentId,
              hasChildren: false,
              isExpanded: false,
              data: type,
            });
          });
        }
      });
    }

    if (!selectedKind || selectedKind === "authors") {
      kit.authors?.forEach((author: Author) => {
        if (searchQuery && !author.name.toLowerCase().includes(searchQuery.toLowerCase()) && !author.email.toLowerCase().includes(searchQuery.toLowerCase())) return;
        result.push({
          id: `author-${author.guid}`,
          kind: "authors",
          artifact: author.name,
          authors: author.email,
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: author,
        });
      });
    }

    if (sortColumn) {
      const level0Rows = result.filter((r) => r.level === 0);
      const level1Rows = result.filter((r) => r.level === 1);
      const level2Rows = result.filter((r) => r.level === 2);
      level0Rows.sort((a, b) => {
        let comparison = 0;
        switch (sortColumn) {
          case "artifact":
            comparison = a.artifact.localeCompare(b.artifact);
            break;
          case "kind":
            comparison = a.kind.localeCompare(b.kind);
            break;
          case "authors":
            comparison = a.authors.localeCompare(b.authors);
            break;
          case "updatedAt":
            comparison = a.updatedAt.localeCompare(b.updatedAt);
            break;
          case "createdAt":
            comparison = a.createdAt.localeCompare(b.createdAt);
            break;
        }
        return sortDirection === "asc" ? comparison : -comparison;
      });
      const sortedResult: TableRow[] = [];
      level0Rows.forEach((parent) => {
        sortedResult.push(parent);
        const children = level1Rows.filter((c) => c.parentId === parent.id);
        children.sort((a, b) => {
          let comparison = 0;
          switch (sortColumn) {
            case "artifact":
              comparison = a.artifact.localeCompare(b.artifact);
              break;
            case "kind":
              comparison = a.kind.localeCompare(b.kind);
              break;
            case "authors":
              comparison = a.authors.localeCompare(b.authors);
              break;
            case "updatedAt":
              comparison = a.updatedAt.localeCompare(b.updatedAt);
              break;
            case "createdAt":
              comparison = a.createdAt.localeCompare(b.createdAt);
              break;
          }
          return sortDirection === "asc" ? comparison : -comparison;
        });
        children.forEach((child) => {
          sortedResult.push(child);
          const grandchildren = level2Rows.filter((gc) => gc.parentId === child.id);
          grandchildren.sort((a, b) => {
            let comparison = 0;
            switch (sortColumn) {
              case "artifact":
                comparison = a.artifact.localeCompare(b.artifact);
                break;
              case "kind":
                comparison = a.kind.localeCompare(b.kind);
                break;
              case "authors":
                comparison = a.authors.localeCompare(b.authors);
                break;
              case "updatedAt":
                comparison = a.updatedAt.localeCompare(b.updatedAt);
                break;
              case "createdAt":
                comparison = a.createdAt.localeCompare(b.createdAt);
                break;
            }
            return sortDirection === "asc" ? comparison : -comparison;
          });
          sortedResult.push(...grandchildren);
        });
      });
      return sortedResult;
    }

    return result;
  }, [kit, selectedKind, selectedName, selectedVariant, selectedView, selectedConcepts, searchQuery, expandedRows, sortColumn, sortDirection]);

  const { setFocusItems, setOnFocusItem } = useFocus();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const prevRowsRef = useRef<string>("");

  useEffect(() => {
    const items = rows.map((row) => ({
      id: row.id,
      label: row.artifact,
      category: row.kind.charAt(0).toUpperCase() + row.kind.slice(1),
    }));
    // Only update if the items have actually changed
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevRowsRef.current !== itemsKey) {
      prevRowsRef.current = itemsKey;
      setFocusItems(items);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rows]);

  useEffect(() => {
    const handleFocus = (itemId: string) => {
      setFocusedItemId(itemId);
    };
    setOnFocusItem(handleFocus);
    return () => setOnFocusItem(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const tbody = scrollAreaRef.current.querySelector("tbody");
      if (tbody) {
        const rowElements = tbody.querySelectorAll("tr");
        const focusedIndex = rows.findIndex((row) => row.id === focusedItemId);
        if (focusedIndex >= 0 && rowElements[focusedIndex]) {
          rowElements[focusedIndex].scrollIntoView({ behavior: "smooth", block: "center" });
          setTimeout(() => setFocusedItemId(undefined), 600);
        }
      }
    }
  }, [focusedItemId, rows]);

  const toggleRow = (rowId: string) => {
    kitAppCommands.toggleExpandedRow(rowId);
  };

  const handleCreateArtifact = (kind: ArtifactKind) => {
    switch (kind) {
      case "designs": {
        const existingNames = (kit.designs || []).map((d: Design) => d.name);
        const uniqueName = generateUniqueName(t("semio.sketchpad.app.design.defaultName"), existingNames);
        const newDesign: Design = {
          guid: guid(),
          name: uniqueName,
          variant: "",
          view: "",
          pieces: [],
          connections: [],
        };
        kitCommands.createDesign(newDesign);
        sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
        break;
      }
      case "types": {
        const existingNames = (kit.types || []).map((t: Type) => t.name);
        const uniqueName = generateUniqueName(t("semio.sketchpad.app.type.defaultName"), existingNames);
        const newType: Type = {
          guid: guid(),
          name: uniqueName,
          variant: "",
          ports: [],
        };
        kitCommands.createType(newType);
        sketchpadCommands.navigateToType(kit.guid, newType.guid);
        break;
      }
      case "qualities": {
        const existingNames = (kit.qualities || []).map((q: Quality) => q.name || "");
        const uniqueName = generateUniqueName(t("semio.sketchpad.app.quality.defaultName"), existingNames);
        const existingKeys = (kit.qualities || []).map((q: Quality) => q.key);
        const uniqueKey = generateUniqueName("new.quality", existingKeys, ".");
        const newQuality: Quality = {
          guid: guid(),
          key: uniqueKey,
          name: uniqueName,
        };
        kitCommands.createQuality(newQuality);
        sketchpadCommands.navigateToQuality(kit.guid, newQuality.guid);
        break;
      }
      case "files": {
        // TODO: Implement file creationbreak;
      }
      case "authors": {
        // TODO: Implement author creationbreak;
      }
    }
  };

  const handleCreateVariantForRow = (row: TableRow) => {
    if (row.kind === "designs") {
      const design = row.data as Design;
      const existingVariants = (kit.designs || []).filter((d: Design) => d.name === design.name).map((d: Design) => d.variant || "");
      const uniqueVariant = generateUniqueName(t("semio.sketchpad.app.design.newVariant"), existingVariants);
      const newDesign: Design = {
        guid: guid(),
        name: design.name,
        variant: uniqueVariant,
        view: "",
        pieces: [],
        connections: [],
      };
      kitCommands.createDesign(newDesign);
      sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
    } else if (row.kind === "types") {
      const type = row.data as Type;
      const existingVariants = (kit.types || []).filter((t: Type) => t.name === type.name).map((t: Type) => t.variant || "");
      const uniqueVariant = generateUniqueName(t("semio.sketchpad.app.type.newVariant"), existingVariants);
      const newType: Type = {
        guid: guid(),
        name: type.name,
        variant: uniqueVariant,
        ports: [],
      };
      kitCommands.createType(newType);
      sketchpadCommands.navigateToType(kit.guid, newType.guid);
    }
  };

  const handleCreateViewForRow = (row: TableRow) => {
    if (row.kind !== "designs") return;
    const design = row.data as Design;
    const existingViews = (kit.designs || []).filter((d: Design) => d.name === design.name && d.variant === design.variant).map((d: Design) => d.view || "");
    const uniqueView = generateUniqueName(t("semio.sketchpad.app.design.newView"), existingViews);
    const newDesign: Design = {
      guid: guid(),
      name: design.name,
      variant: design.variant,
      view: uniqueView,
      pieces: [],
      connections: [],
    };
    kitCommands.createDesign(newDesign);
    sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
  };

  const toggleKind = (kind: ArtifactKind) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedKind === kind) {
      newParams.delete("kind");
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.set("kind", kind);
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    }
    setSearchParams(newParams);
  };

  const toggleConcept = (concept: string) => {
    const newParams = new URLSearchParams(searchParams);
    const currentConcepts = newParams.getAll("c");

    if (currentConcepts.includes(concept)) {
      newParams.delete("c");
      currentConcepts.filter((c) => c !== concept).forEach((c) => newParams.append("c", c));
    } else {
      newParams.append("c", concept);
    }

    setSearchParams(newParams);
  };

  const toggleName = (name: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedName === name) {
      newParams.delete("name");
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.set("name", name);
      newParams.delete("variant");
      newParams.delete("view");
    }
    setSearchParams(newParams);
  };

  const toggleVariant = (variant: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedVariant === variant) {
      newParams.delete("variant");
      newParams.delete("view");
    } else {
      newParams.set("variant", variant);
      newParams.delete("view");
    }
    setSearchParams(newParams);
  };

  const toggleView = (view: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedView === view) {
      newParams.delete("view");
    } else {
      newParams.set("view", view);
    }
    setSearchParams(newParams);
  };

  const handleRowClick = (row: TableRow, e: React.MouseEvent) => {
    if (row.kind === "designs") {
      const designId = (row.data as Design).guid;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "designs" && (r.data as Design).guid === designId);
        if (selection.designs.length > 0) {
          const lastSelectedId = selection.designs[selection.designs.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "designs" && (r.data as Design).guid === lastSelectedId);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangeIds = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "designs")
              .map((r) => (r.data as Design).guid);
            kitAppCommands.selectDesigns(rangeIds);
          }
        } else {
          kitAppCommands.selectDesign(designId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.designs.includes(designId)) {
          kitAppCommands.removeDesignFromSelection(designId);
        } else {
          kitAppCommands.addDesignToSelection(designId);
        }
      } else {
        kitAppCommands.selectDesign(designId);
      }
    } else if (row.kind === "types") {
      const typeId = (row.data as Type).guid;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "types" && (r.data as Type).guid === typeId);
        if (selection.types.length > 0) {
          const lastSelectedId = selection.types[selection.types.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "types" && (r.data as Type).guid === lastSelectedId);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangeIds = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "types")
              .map((r) => (r.data as Type).guid);
            kitAppCommands.selectTypes(rangeIds);
          }
        } else {
          kitAppCommands.selectType(typeId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.types.includes(typeId)) {
          kitAppCommands.removeTypeFromSelection(typeId);
        } else {
          kitAppCommands.addTypeToSelection(typeId);
        }
      } else {
        kitAppCommands.selectType(typeId);
      }
    } else if (row.kind === "qualities") {
      const qualityKey = (row.data as Quality).key;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "qualities" && (r.data as Quality).key === qualityKey);
        if (selection.qualities.length > 0) {
          const lastSelectedKey = selection.qualities[selection.qualities.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "qualities" && (r.data as Quality).key === lastSelectedKey);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangeKeys = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "qualities")
              .map((r) => (r.data as Quality).key);
            kitAppCommands.selectQualities(rangeKeys);
          }
        } else {
          kitAppCommands.selectQuality(qualityKey);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.qualities.includes(qualityKey)) {
          kitAppCommands.removeQualityFromSelection(qualityKey);
        } else {
          kitAppCommands.addQualityToSelection(qualityKey);
        }
      } else {
        kitAppCommands.selectQuality(qualityKey);
      }
    } else if (row.kind === "files") {
      const filePath = (row.data as SemioFile).path;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "files" && (r.data as SemioFile).path === filePath);
        if (selection.files.length > 0) {
          const lastSelectedPath = selection.files[selection.files.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "files" && (r.data as SemioFile).path === lastSelectedPath);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangePaths = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "files")
              .map((r) => (r.data as SemioFile).path);
            kitAppCommands.selectFiles(rangePaths);
          }
        } else {
          kitAppCommands.selectFile(filePath);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.files.includes(filePath)) {
          kitAppCommands.removeFileFromSelection(filePath);
        } else {
          kitAppCommands.addFileToSelection(filePath);
        }
      } else {
        kitAppCommands.selectFile(filePath);
      }
    } else if (row.kind === "authors") {
      const authorName = (row.data as Author).name;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "authors" && (r.data as Author).name === authorName);
        if (selection.authors.length > 0) {
          const lastSelectedName = selection.authors[selection.authors.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "authors" && (r.data as Author).name === lastSelectedName);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangeNames = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "authors")
              .map((r) => (r.data as Author).name);
            kitAppCommands.selectAuthors(rangeNames);
          }
        } else {
          kitAppCommands.selectAuthor(authorName);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.authors.includes(authorName)) {
          kitAppCommands.removeAuthorFromSelection(authorName);
        } else {
          kitAppCommands.addAuthorToSelection(authorName);
        }
      } else {
        kitAppCommands.selectAuthor(authorName);
      }
    }
  };

  const handleRowDoubleClick = (row: TableRow) => {
    if (row.kind === "designs") {
      sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
    } else if (row.kind === "types") {
      sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
    } else if (row.kind === "qualities") {
      sketchpadCommands.navigateToQuality(kit.guid, (row.data as Quality).key);
    }
  };

  const handleSortClick = (column: "artifact" | "kind" | "authors" | "updatedAt" | "createdAt") => {
    kitAppCommands.toggleSort(column);
  };
  
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer.types.includes("Files")) {
      setIsDragOver(true);
    }
  };
  
  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  };
  
  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
    
    const files = Array.from(e.dataTransfer.files);
    if (files.length === 0) return;
    
    for (const file of files) {
      const newFile: SemioFile = {
        guid: guid(),
        path: file.name,
        size: file.size,
        hash: undefined,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
      
      try {
        await kitCommands?.addFile(newFile, file);
      } catch (error) {
        console.error(`Failed to add file ${file.name}:`, error);
      }
    }
  };

  if (isMobile) {
    return (
      <div
        className="flex flex-col h-full"
        onClick={(e: React.MouseEvent) => {
          if (e.target === e.currentTarget) {
            kitAppCommands.deselectAll();
          }
        }}
      >
        {/* Flexible filter layout with automatic wrapping for mobile */}
        <div className="flex flex-wrap items-center gap-1 p-1 border-b">
          {selectedKind && (
            <Toggle
              type="withAction"
              pressed={true}
              onPressedChange={() => toggleKind(selectedKind)}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact(selectedKind)}
              tooltip={tooltip("kitApp.hideKind")}
              actionTooltip={tooltip("kitApp.createArtifact")}
            >
              {selectedKind === "designs" && <Layout className="size-4" />}
              {selectedKind === "types" && <Box className="size-4" />}
              {selectedKind === "qualities" && <Award className="size-4" />}
              {selectedKind === "files" && <FileText className="size-4" />}
              {selectedKind === "authors" && <User className="size-4" />}
            </Toggle>
          )}
          {selectedName && (
            <Toggle pressed={true} onPressedChange={() => toggleName(selectedName)}>
              {selectedName}
            </Toggle>
          )}
          {selectedVariant !== null && (
            <Toggle pressed={true} onPressedChange={() => toggleVariant(selectedVariant)}>
              {selectedVariant || <span className="italic opacity-50">{selectedKind === "designs" ? t("semio.sketchpad.app.design.defaultVariant") : t("semio.sketchpad.app.type.defaultVariant")}</span>}
            </Toggle>
          )}
          {selectedView !== null && (
            <Toggle pressed={true} onPressedChange={() => toggleView(selectedView)}>
              {selectedView || <span className="italic opacity-50">{t("semio.sketchpad.app.design.defaultView")}</span>}
            </Toggle>
          )}
          {selectedConcepts.length > 0 &&
            selectedConcepts.map((concept) => (
              <Toggle key={concept} pressed={true} onPressedChange={() => toggleConcept(concept)} i18n="semio.sketchpad.app.kit.filter.concept.hide">
                {concept}
              </Toggle>
            ))}
          {!selectedKind && (
            <>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("designs")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("designs")}
                tooltip={tooltip("kitApp.showDesigns")}
                actionTooltip={tooltip("kitApp.createDesign")}
              >
                <Layout className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("types")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("types")}
                tooltip={tooltip("kitApp.showTypes")}
                actionTooltip={tooltip("kitApp.createType")}
              >
                <Box className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("qualities")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("qualities")}
                tooltip={tooltip("kitApp.showQualities")}
                actionTooltip={tooltip("kitApp.createQuality")}
              >
                <Award className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("files")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("files")}
                tooltip={tooltip("kitApp.showFiles")}
                actionTooltip={tooltip("kitApp.createFile")}
              >
                <FileText className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("authors")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("authors")}
                tooltip={tooltip("kitApp.showAuthors")}
                actionTooltip={tooltip("kitApp.createAuthor")}
              >
                <User className="size-4" />
              </Toggle>
            </>
          )}
          {allConcepts.length > 0 &&
            allConcepts
              .filter((c) => !selectedConcepts.includes(c))
              .map((concept) => (
                <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} i18n="semio.sketchpad.app.kit.filter.concept.show">
                  {concept}
                </Toggle>
              ))}
          {selectedKind &&
            !selectedName &&
            uniqueNames.length > 0 &&
            uniqueNames.map((name) => (
              <Toggle key={name} pressed={false} onPressedChange={() => toggleName(name)} i18n="semio.sketchpad.app.kit.filter.name">
                {name}
              </Toggle>
            ))}
          {selectedKind &&
            selectedName &&
            selectedVariant === null &&
            uniqueVariants.length > 0 &&
            uniqueVariants.map((variant) => (
              <Toggle key={variant} pressed={false} onPressedChange={() => toggleVariant(variant)} i18n="semio.sketchpad.app.kit.filter.variant">
                {variant || <span className="italic opacity-50">{selectedKind === "designs" ? t("semio.sketchpad.app.design.defaultVariant") : t("semio.sketchpad.app.type.defaultVariant")}</span>}
              </Toggle>
            ))}
          {selectedKind === "designs" &&
            selectedName &&
            selectedVariant !== null &&
            uniqueViews.length > 0 &&
            uniqueViews
              .filter((view) => view !== selectedView)
              .map((view) => (
                <Toggle key={view} pressed={false} onPressedChange={() => toggleView(view)} i18n="semio.sketchpad.app.kit.filter.view">
                  {view || <span className="italic opacity-50">{t("semio.sketchpad.app.design.defaultView")}</span>}
                </Toggle>
              ))}
          <div className="flex items-center gap-1 flex-1 min-w-[160px]">
            <Input className="flex-1 min-w-0" placeholder={t("semio.sketchpad.common.search")} value={searchQuery} onChange={(e) => kitAppCommands.setFilterSearch(e.target.value)} />
            <Toggle
              type="dropdown"
              pressed={sortColumn === "artifact"}
              value={sortColumn === "artifact" ? sortDirection : "asc"}
              onValueChange={(value) => {
                kitAppCommands.setSortColumn("artifact");
                kitAppCommands.setSortDirection(value as "asc" | "desc");
              }}
              items={[
                { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: tooltip("sort.ascending") },
                { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: tooltip("sort.descending") },
              ]}
              tooltip={tooltip("kitApp.sortByName")}
            />
          </div>
        </div>

        {/* Simplified table - only name column, no headers */}
        <ScrollArea className="flex-1">
          <div className="flex flex-col">
            {rows.map((row) => {
              const isSelected =
                (row.kind === "designs" && selection.designs.includes((row.data as Design).guid)) ||
                (row.kind === "types" && selection.types.includes((row.data as Type).guid)) ||
                (row.kind === "qualities" && selection.qualities.includes((row.data as Quality).key)) ||
                (row.kind === "files" && selection.files.includes((row.data as SemioFile).path)) ||
                (row.kind === "authors" && selection.authors.includes((row.data as Author).name));
              return (
                <div
                  key={row.id}
                  className={`border-b p-2 cursor-selectable ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`}
                  onClick={(e) => handleRowClick(row, e)}
                  onDoubleClick={() => handleRowDoubleClick(row)}
                  role="button"
                  tabIndex={0}
                >
                  <div className="flex items-center gap-2 justify-between" style={{ paddingLeft: `${row.level * 16}px` }} onClick={(e) => e.stopPropagation()}>
                    <div className="flex items-center gap-2 flex-1 min-w-0">
                      {row.hasChildren ? (
                        <Action
                          level="base"
                          onClick={(e) => {
                            e.stopPropagation();
                            toggleRow(row.id);
                          }}
                        >
                          {row.isExpanded ? <ChevronDown /> : <ChevronRight />}
                        </Action>
                      ) : (
                        <span className="w-5 h-5 shrink-0" />
                      )}
                      <div className="shrink-0">
                        {row.kind === "designs" && <Layout className="size-4" />}
                        {row.kind === "types" && <Box className="size-4" />}
                        {row.kind === "qualities" && <Award className="size-4" />}
                        {row.kind === "files" && <FileText className="size-4" />}
                        {row.kind === "authors" && <User className="size-4" />}
                      </div>
                      <span className="text-left flex-1 min-w-0 truncate">{row.artifact}</span>
                    </div>
                    <div className="flex items-center gap-0.5 shrink-0">
                      {row.kind === "designs" && row.level === 0 && (
                        <>
                          <Action
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateVariantForRow(row);
                            }}
                            tooltip={tooltip("kitApp.createVariant")}
                            level="base"
                          >
                            <Plus />
                          </Action>
                          <Action
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateViewForRow(row);
                            }}
                            tooltip={tooltip("kitApp.createView")}
                            level="base"
                          >
                            <Plus />
                          </Action>
                        </>
                      )}
                      {row.kind === "types" && row.level === 0 && (
                        <Action
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCreateVariantForRow(row);
                          }}
                          tooltip={tooltip("kitApp.createVariant")}
                          level="base"
                        >
                          <Plus />
                        </Action>
                      )}
                      {row.kind === "designs" && row.level === 1 && (
                        <Action
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCreateViewForRow(row);
                          }}
                          tooltip={tooltip("kitApp.createView")}
                          level="base"
                        >
                          <Plus />
                        </Action>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </ScrollArea>
      </div>
    );
  }

  return (
    <div
      className="flex flex-col h-full"
      onClick={(e: React.MouseEvent) => {
        if (e.target === e.currentTarget) {
          kitAppCommands.deselectAll();
        }
      }}
    >
      {/* Flexible filter layout with automatic wrapping */}
      <div className="flex flex-wrap items-center gap-1 p-1 border-b">
        {selectedKind && (
          <Toggle
            type="withAction"
            pressed={true}
            onPressedChange={() => toggleKind(selectedKind)}
            actionIcon={<Plus className="size-3.5" />}
            onActionClick={() => handleCreateArtifact(selectedKind)}
            tooltip={tooltip("kitApp.hideKind")}
            actionTooltip={tooltip("kitApp.createArtifact")}
          >
            {selectedKind === "designs" && <Layout className="size-4" />}
            {selectedKind === "types" && <Box className="size-4" />}
            {selectedKind === "qualities" && <Award className="size-4" />}
            {selectedKind === "files" && <FileText className="size-4" />}
            {selectedKind === "authors" && <User className="size-4" />}
          </Toggle>
        )}
        {selectedName && (
          <Toggle pressed={true} onPressedChange={() => toggleName(selectedName)}>
            {selectedName}
          </Toggle>
        )}
        {selectedVariant !== null && (
          <Toggle pressed={true} onPressedChange={() => toggleVariant(selectedVariant)}>
            {selectedVariant || <span className="italic opacity-50">{selectedKind === "designs" ? t("semio.sketchpad.app.design.defaultVariant") : t("semio.sketchpad.app.type.defaultVariant")}</span>}
          </Toggle>
        )}
        {selectedView !== null && (
          <Toggle pressed={true} onPressedChange={() => toggleView(selectedView)}>
            {selectedView || <span className="italic opacity-50">{t("semio.sketchpad.app.design.defaultView")}</span>}
          </Toggle>
        )}
        {selectedConcepts.length > 0 &&
          selectedConcepts.map((concept) => (
            <Toggle key={concept} pressed={true} onPressedChange={() => toggleConcept(concept)} i18n="semio.sketchpad.app.kit.filter.concept.hide">
              {concept}
            </Toggle>
          ))}
        {!selectedKind && (
          <>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("designs")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("designs")}
              tooltip={tooltip("kitApp.showDesigns")}
              actionTooltip={tooltip("kitApp.createDesign")}
            >
              <Layout className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("types")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("types")}
              tooltip={tooltip("kitApp.showTypes")}
              actionTooltip={tooltip("kitApp.createType")}
            >
              <Box className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("qualities")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("qualities")}
              tooltip={tooltip("kitApp.showQualities")}
              actionTooltip={tooltip("kitApp.createQuality")}
            >
              <Award className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("files")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("files")}
              tooltip={tooltip("kitApp.showFiles")}
              actionTooltip={tooltip("kitApp.createFile")}
            >
              <FileText className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("authors")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("authors")}
              tooltip={tooltip("kitApp.showAuthors")}
              actionTooltip={tooltip("kitApp.createAuthor")}
            >
              <User className="size-4" />
            </Toggle>
          </>
        )}
        {allConcepts.length > 0 &&
          allConcepts
            .filter((c) => !selectedConcepts.includes(c))
            .map((concept) => (
              <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} i18n="semio.sketchpad.app.kit.filter.concept.show">
                {concept}
              </Toggle>
            ))}
        {selectedKind &&
          !selectedName &&
          uniqueNames.length > 0 &&
          uniqueNames.map((name) => (
            <Toggle key={name} pressed={false} onPressedChange={() => toggleName(name)}>
              {name}
            </Toggle>
          ))}
        {selectedKind &&
          selectedName &&
          selectedVariant === null &&
          uniqueVariants.length > 0 &&
          uniqueVariants.map((variant) => (
            <Toggle key={variant} pressed={false} onPressedChange={() => toggleVariant(variant)}>
              {variant || <span className="italic opacity-50">{selectedKind === "designs" ? t("semio.sketchpad.app.design.defaultVariant") : t("semio.sketchpad.app.type.defaultVariant")}</span>}
            </Toggle>
          ))}
        {selectedKind === "designs" &&
          selectedName &&
          selectedVariant !== null &&
          uniqueViews.length > 0 &&
          uniqueViews
            .filter((view) => view !== selectedView)
            .map((view) => (
              <Toggle key={view} pressed={false} onPressedChange={() => toggleView(view)}>
                {view || <span className="italic opacity-50">{t("semio.sketchpad.app.design.defaultView")}</span>}
              </Toggle>
            ))}
        <Input className="flex-1 min-w-[200px]" placeholder={t("semio.sketchpad.common.search")} value={searchQuery} onChange={(e) => kitAppCommands.setFilterSearch(e.target.value)} />
      </div>
      <ScrollArea 
        ref={scrollAreaRef} 
        className="flex-1"
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        {isDragOver && (
          <div className="absolute inset-0 bg-active-base/50 border-2 border-dashed border-active-foreground flex items-center justify-center z-10">
            <div className="text-active-foreground text-lg font-medium">Drop files to add to kit</div>
          </div>
        )}
        <table className="w-full border-collapse">
          <thead className="sticky top-0 border-b">
            <tr className="h-9">
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("semio.sketchpad.app.kit.name")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "artifact"}
                    value={sortColumn === "artifact" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("artifact");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.descending") },
                    ]}
                    className="px-1 min-w-0"
                  />
                </div>
                <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
              </th>
              {!selectedKind && (
                <th className="text-left p-1 font-medium relative group">
                  <div className="flex items-center justify-between w-full">
                    <span>{t("semio.sketchpad.app.kit.kind")}</span>
                    <Toggle
                      type="dropdown"
                      pressed={sortColumn === "kind"}
                      value={sortColumn === "kind" ? sortDirection : "asc"}
                      onValueChange={(value) => {
                        kitAppCommands.setSortColumn("kind");
                        kitAppCommands.setSortDirection(value as "asc" | "desc");
                      }}
                      items={[
                        { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.ascending") },
                        { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.descending") },
                      ]}
                      className="px-1 min-w-0"
                    />
                  </div>
                  <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                </th>
              )}
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("semio.sketchpad.app.kit.lastUpdated")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "updatedAt"}
                    value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("updatedAt");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.descending") },
                    ]}
                    className="px-1 min-w-0"
                  />
                </div>
                <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
              </th>
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("semio.sketchpad.app.kit.created")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "createdAt"}
                    value={sortColumn === "createdAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitAppCommands.setSortColumn("createdAt");
                      kitAppCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("semio.sketchpad.common.sort.descending") },
                    ]}
                    className="px-1 min-w-0"
                  />
                </div>
              </th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => {
              const isSelected =
                (row.kind === "designs" && selection.designs.includes((row.data as Design).guid)) ||
                (row.kind === "types" && selection.types.includes((row.data as Type).guid)) ||
                (row.kind === "qualities" && selection.qualities.includes((row.data as Quality).key)) ||
                (row.kind === "files" && selection.files.includes((row.data as SemioFile).path)) ||
                (row.kind === "authors" && selection.authors.includes((row.data as Author).name));
              return (
                <tr
                  key={row.id}
                  className={`border-b cursor-selectable ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`}
                  onClick={(e) => handleRowClick(row, e)}
                  onDoubleClick={() => handleRowDoubleClick(row)}
                  role="button"
                  tabIndex={0}
                >
                  <td className="p-1" onClick={(e) => e.stopPropagation()}>
                    <div className="flex items-center gap-1 justify-between" style={{ paddingLeft: `${row.level * 24}px` }}>
                      <div className="flex items-center gap-1 flex-1 min-w-0">
                        {row.hasChildren ? (
                          <Action
                            level="base"
                            onClick={(e) => {
                              e.stopPropagation();
                              toggleRow(row.id);
                            }}
                          >
                            {row.isExpanded ? <ChevronDown /> : <ChevronRight />}
                          </Action>
                        ) : (
                          <span className="w-4 h-4 shrink-0" />
                        )}
                        <span className="text-left flex-1 min-w-0 truncate">{row.artifact}</span>
                      </div>
                      <div className="flex items-center gap-0.5 shrink-0">
                        {row.kind === "designs" && row.level === 0 && (
                          <>
                            <Action
                              onClick={(e) => {
                                e.stopPropagation();
                                handleCreateVariantForRow(row);
                              }}
                              tooltip={tooltip("kitApp.createVariant")}
                              level="base"
                            >
                              <Plus />
                            </Action>
                            <Action
                              onClick={(e) => {
                                e.stopPropagation();
                                handleCreateViewForRow(row);
                              }}
                              tooltip={tooltip("kitApp.createView")}
                              level="base"
                            >
                              <Plus />
                            </Action>
                          </>
                        )}
                        {row.kind === "types" && row.level === 0 && (
                          <Action
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateVariantForRow(row);
                            }}
                            tooltip={tooltip("kitApp.createVariant")}
                            level="base"
                          >
                            <Plus />
                          </Action>
                        )}
                        {row.kind === "designs" && row.level === 1 && (
                          <Action
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateViewForRow(row);
                            }}
                            tooltip={tooltip("kitApp.createView")}
                            level="base"
                          >
                            <Plus />
                          </Action>
                        )}
                      </div>
                    </div>
                  </td>
                  {!selectedKind && (
                    <td className="p-1">
                      {row.kind === "designs" && <Layout className="size-4" />}
                      {row.kind === "types" && <Box className="size-4" />}
                      {row.kind === "qualities" && <Award className="size-4" />}
                      {row.kind === "files" && <FileText className="size-4" />}
                      {row.kind === "authors" && <User className="size-4" />}
                    </td>
                  )}
                  <td className="p-1">{row.updatedAt}</td>
                  <td className="p-1">{row.createdAt}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </ScrollArea>
    </div>
  );
};

class ErrorBoundary extends React.Component<{ children: React.ReactNode; fallback: React.ReactNode }, { hasError: boolean; error: Error | null }> {
  constructor(props: { children: React.ReactNode; fallback: React.ReactNode }) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {}

  componentDidUpdate(prevProps: { children: React.ReactNode; fallback: React.ReactNode }) {
    if (prevProps.children !== this.props.children && this.state.hasError) {
      this.setState({ hasError: false, error: null });
    }
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback;
    }
    return this.props.children;
  }
}

const App: FC = () => {
  return (
    <ErrorBoundary
      fallback={
        <div className="flex items-center justify-center h-full">
          <p className="text-sm text-muted-foreground">Failed to load kit app</p>
        </div>
      }
    >
      <Canvas>
        <Window id="kit-table">
          <AppContent />
        </Window>
      </Canvas>
    </ErrorBoundary>
  );
};

export default App;
