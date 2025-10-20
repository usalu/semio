// #region Header

// Editor.tsx

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
import React, { FC, useEffect, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ScrollArea } from "../../../elements/aggregation/ScrollArea";
import { Action } from "../../../elements/input/Action";
import { Input } from "../../../elements/input/Input";
import { Toggle } from "../../../elements/input/Toggle";
import i18n from "../../../i18n";
import { Author, Design, generateUniqueName, guid, Kit, Quality, File as SemioFile, Type } from "../../../semio";
import { useAddPanelSection, useRemovePanelSection } from "../../Navbar";
import { EditorType, useEditorType, useIsMobile, useKit, useKitCommands, useKitScope, useNavigation, useSketchpadCommands, useSketchpadStore, useTooltip } from "../../store";
import { KitDetails } from "./panels/Details";
import { KitEditorState, useKitEditor, useKitEditorCommands } from "./store";

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

const EditorContent: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const params = useParams();
  const [searchParams, setSearchParams] = useSearchParams();
  const tooltip = useTooltip();

  const kit = useKit() as Kit;
  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitEditorCommands = useKitEditorCommands();
  const kitEditor = useKitEditor() as KitEditorState;
  const isMobile = useIsMobile();

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
  const expandedRowsArray = kitEditor?.expandedRows || [];
  const expandedRows = new Set(expandedRowsArray);

  const selection = {
    types: kitEditor?.selection?.types || [],
    designs: kitEditor?.selection?.designs || [],
    qualities: kitEditor?.selection?.qualities || [],
    files: kitEditor?.selection?.files || [],
    authors: kitEditor?.selection?.authors || [],
  };
  const sortColumn = kitEditor?.sortColumn;
  const sortDirection = kitEditor?.sortDirection || "asc";

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();

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
    if (editorType !== EditorType.KIT) {
      return;
    }

    addSection("details", {
      id: "kit-details",
      label: "Kit",
      order: 0,
      defaultOpen: true,
      content: () => <KitDetails />,
    });

    return () => {
      removeSection("details", "kit-details");
    };
  }, [addSection, removeSection, editorType]);

  // Auto-select design/type when select parameter is present
  useEffect(() => {
    if (!selectParam) return;

    if (selectedKind === "designs") {
      const design = kit.designs?.find((d: Design) => d.guid === selectParam);
      if (design) {kitEditorCommands.selectDesign(selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    } else if (selectedKind === "types") {
      const type = kit.types?.find((t: Type) => t.guid === selectParam);
      if (type) {kitEditorCommands.selectType(selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    }
  }, [selectParam, selectedKind, kit, kitEditorCommands, searchParams, setSearchParams]);

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

  const toggleRow = (rowId: string) => {
    kitEditorCommands.toggleExpandedRow(rowId);
  };

  const handleCreateArtifact = (kind: ArtifactKind) => {
    switch (kind) {
      case "designs": {
        const existingNames = (kit.designs || []).map((d: Design) => d.name);
        const uniqueName = generateUniqueName(t("design.defaultName"), existingNames);
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
        const uniqueName = generateUniqueName(t("type.defaultName"), existingNames);
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
        // TODO: Implement quality creationbreak;
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
      const uniqueVariant = generateUniqueName(t("design.newVariant"), existingVariants);
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
      const uniqueVariant = generateUniqueName(t("type.newVariant"), existingVariants);
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
    const uniqueView = generateUniqueName(t("design.newView"), existingViews);
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
            kitEditorCommands.selectDesigns(rangeIds);
          }
        } else {
          kitEditorCommands.selectDesign(designId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.designs.includes(designId)) {
          kitEditorCommands.removeDesignFromSelection(designId);
        } else {
          kitEditorCommands.addDesignToSelection(designId);
        }
      } else {
        kitEditorCommands.selectDesign(designId);
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
            kitEditorCommands.selectTypes(rangeIds);
          }
        } else {
          kitEditorCommands.selectType(typeId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.types.includes(typeId)) {
          kitEditorCommands.removeTypeFromSelection(typeId);
        } else {
          kitEditorCommands.addTypeToSelection(typeId);
        }
      } else {
        kitEditorCommands.selectType(typeId);
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
            kitEditorCommands.selectQualities(rangeKeys);
          }
        } else {
          kitEditorCommands.selectQuality(qualityKey);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.qualities.includes(qualityKey)) {
          kitEditorCommands.removeQualityFromSelection(qualityKey);
        } else {
          kitEditorCommands.addQualityToSelection(qualityKey);
        }
      } else {
        kitEditorCommands.selectQuality(qualityKey);
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
            kitEditorCommands.selectFiles(rangePaths);
          }
        } else {
          kitEditorCommands.selectFile(filePath);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.files.includes(filePath)) {
          kitEditorCommands.removeFileFromSelection(filePath);
        } else {
          kitEditorCommands.addFileToSelection(filePath);
        }
      } else {
        kitEditorCommands.selectFile(filePath);
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
            kitEditorCommands.selectAuthors(rangeNames);
          }
        } else {
          kitEditorCommands.selectAuthor(authorName);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.authors.includes(authorName)) {
          kitEditorCommands.removeAuthorFromSelection(authorName);
        } else {
          kitEditorCommands.addAuthorToSelection(authorName);
        }
      } else {
        kitEditorCommands.selectAuthor(authorName);
      }
    }
  };

  const handleSortClick = (column: "artifact" | "kind" | "authors" | "updatedAt" | "createdAt") => {
    kitEditorCommands.toggleSort(column);
  };

  if (isMobile) {
    return (
      <div className="flex flex-col h-full">
        {/* Flexible filter layout with automatic wrapping for mobile */}
        <div className="flex flex-wrap items-center gap-1 p-1 border-b">
          {selectedKind && (
            <Toggle
              type="withAction"
              pressed={true}
              onPressedChange={() => toggleKind(selectedKind)}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact(selectedKind)}
              tooltip={tooltip("kitEditor.hideKind")}
              actionTooltip={tooltip("kitEditor.createArtifact")}
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
              {selectedVariant || <span className="italic opacity-50">{selectedKind === "designs" ? t("design.defaultVariant") : t("type.defaultVariant")}</span>}
            </Toggle>
          )}
          {selectedView !== null && (
            <Toggle pressed={true} onPressedChange={() => toggleView(selectedView)}>
              {selectedView || <span className="italic opacity-50">{t("design.defaultView")}</span>}
            </Toggle>
          )}
          {selectedConcepts.length > 0 &&
            selectedConcepts.map((concept) => (
              <Toggle key={concept} pressed={true} onPressedChange={() => toggleConcept(concept)} tooltip={t("kitEditor.hideConcept", { concept })}>
                {concept}
              </Toggle>
            ))}
          {!selectedKind && (
            <>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("designs")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateArtifact("designs")}
                tooltip={tooltip("kitEditor.showDesigns")}
                actionTooltip={tooltip("kitEditor.createDesign")}
              >
                <Layout className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("types")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateArtifact("types")}
                tooltip={tooltip("kitEditor.showTypes")}
                actionTooltip={tooltip("kitEditor.createType")}
              >
                <Box className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("qualities")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateArtifact("qualities")}
                tooltip={tooltip("kitEditor.showQualities")}
                actionTooltip={tooltip("kitEditor.createQuality")}
              >
                <Award className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("files")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateArtifact("files")}
                tooltip={tooltip("kitEditor.showFiles")}
                actionTooltip={tooltip("kitEditor.createFile")}
              >
                <FileText className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("authors")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateArtifact("authors")}
                tooltip={tooltip("kitEditor.showAuthors")}
                actionTooltip={tooltip("kitEditor.createAuthor")}
              >
                <User className="size-4" />
              </Toggle>
            </>
          )}
          {allConcepts.length > 0 &&
            allConcepts
              .filter((c) => !selectedConcepts.includes(c))
              .map((concept) => (
                <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} tooltip={t("kitEditor.showConcept", { concept })}>
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
                {variant || <span className="italic opacity-50">{selectedKind === "designs" ? t("design.defaultVariant") : t("type.defaultVariant")}</span>}
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
                  {view || <span className="italic opacity-50">{t("design.defaultView")}</span>}
                </Toggle>
              ))}
          <div className="flex items-center gap-1 flex-1 min-w-[160px]">
            <Input className="flex-1 min-w-0" placeholder={t("common.search")} value={searchQuery} onChange={(e) => kitEditorCommands.setFilterSearch(e.target.value)} />
            <Toggle
              type="dropdown"
              pressed={sortColumn === "artifact"}
              value={sortColumn === "artifact" ? sortDirection : "asc"}
              onValueChange={(value) => {
                kitEditorCommands.setSortColumn("artifact");
                kitEditorCommands.setSortDirection(value as "asc" | "desc");
              }}
              items={[
                { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
              ]}
              tooltip={t("kitEditor.sortByName")}
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
                <div key={row.id} className={`border-b p-2 ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`} onClick={(e) => handleRowClick(row, e)} role="button" tabIndex={0}>
                  <div className="flex items-center gap-2 justify-between" style={{ paddingLeft: `${row.level * 16}px` }} onClick={(e) => e.stopPropagation()}>
                    <div className="flex items-center gap-2 flex-1 min-w-0">
                      {row.hasChildren ? (
                        <Action level="base" onClick={(e) => { e.stopPropagation(); toggleRow(row.id); }}>
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
                      <span
                        className="hover:underline text-left flex-1 min-w-0 truncate"
                        onClick={(e) => {
                          e.stopPropagation();
                          if (row.kind === "designs") sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
                          else if (row.kind === "types") sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
                        }}
                        role="link"
                        tabIndex={0}
                      >
                        {row.artifact}
                      </span>
                    </div>
                    <div className="flex items-center gap-0.5 shrink-0">
                      {row.kind === "designs" && row.level === 0 && (
                        <>
                          <Action
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateVariantForRow(row);
                            }}
                            tooltip={t("kitEditor.createVariant")}
                            level="base"
                          >
                            <Plus />
                          </Action>
                          <Action
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateViewForRow(row);
                            }}
                            tooltip={t("kitEditor.createView")}
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
                          tooltip={t("kitEditor.createVariant")}
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
                          tooltip={t("kitEditor.createView")}
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
    <div className="flex flex-col h-full">
      {/* Flexible filter layout with automatic wrapping */}
      <div className="flex flex-wrap items-center gap-1 p-1 border-b">
        {selectedKind && (
          <Toggle
            type="withAction"
            pressed={true}
            onPressedChange={() => toggleKind(selectedKind)}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateArtifact(selectedKind)}
            tooltip={tooltip("kitEditor.hideKind")}
            actionTooltip={tooltip("kitEditor.createArtifact")}
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
            {selectedVariant || <span className="italic opacity-50">{selectedKind === "designs" ? t("design.defaultVariant") : t("type.defaultVariant")}</span>}
          </Toggle>
        )}
        {selectedView !== null && (
          <Toggle pressed={true} onPressedChange={() => toggleView(selectedView)}>
            {selectedView || <span className="italic opacity-50">{t("design.defaultView")}</span>}
          </Toggle>
        )}
        {selectedConcepts.length > 0 &&
          selectedConcepts.map((concept) => (
            <Toggle key={concept} pressed={true} onPressedChange={() => toggleConcept(concept)} tooltip={t("kitEditor.hideConcept", { concept })}>
              {concept}
            </Toggle>
          ))}
        {!selectedKind && (
          <>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("designs")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("designs")}
              tooltip={tooltip("kitEditor.showDesigns")}
              actionTooltip={tooltip("kitEditor.createDesign")}
            >
              <Layout className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("types")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("types")}
              tooltip={tooltip("kitEditor.showTypes")}
              actionTooltip={tooltip("kitEditor.createType")}
            >
              <Box className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("qualities")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("qualities")}
              tooltip={tooltip("kitEditor.showQualities")}
              actionTooltip={tooltip("kitEditor.createQuality")}
            >
              <Award className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("files")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("files")}
              tooltip={tooltip("kitEditor.showFiles")}
              actionTooltip={tooltip("kitEditor.createFile")}
            >
              <FileText className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("authors")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("authors")}
              tooltip={tooltip("kitEditor.showAuthors")}
              actionTooltip={tooltip("kitEditor.createAuthor")}
            >
              <User className="size-4" />
            </Toggle>
          </>
        )}
        {allConcepts.length > 0 &&
          allConcepts
            .filter((c) => !selectedConcepts.includes(c))
            .map((concept) => (
              <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} tooltip={t("kitEditor.showConcept", { concept })}>
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
              {variant || <span className="italic opacity-50">{selectedKind === "designs" ? t("design.defaultVariant") : t("type.defaultVariant")}</span>}
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
                {view || <span className="italic opacity-50">{t("design.defaultView")}</span>}
              </Toggle>
            ))}
        <Input className="flex-1 min-w-[200px]" placeholder={t("common.search")} value={searchQuery} onChange={(e) => kitEditorCommands.setFilterSearch(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 border-b">
            <tr className="h-9">
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("kitEditor.name")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "artifact"}
                    value={sortColumn === "artifact" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitEditorCommands.setSortColumn("artifact");
                      kitEditorCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
                    ]}
                    className="px-1 min-w-0"
                  />
                </div>
                <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
              </th>
              {!selectedKind && (
                <th className="text-left p-1 font-medium relative group">
                  <div className="flex items-center justify-between w-full">
                    <span>{t("kitEditor.kind")}</span>
                    <Toggle
                      type="dropdown"
                      pressed={sortColumn === "kind"}
                      value={sortColumn === "kind" ? sortDirection : "asc"}
                      onValueChange={(value) => {
                        kitEditorCommands.setSortColumn("kind");
                        kitEditorCommands.setSortDirection(value as "asc" | "desc");
                      }}
                      items={[
                        { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                        { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
                      ]}
                      className="px-1 min-w-0"
                    />
                  </div>
                  <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                </th>
              )}
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("kitEditor.lastUpdated")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "updatedAt"}
                    value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitEditorCommands.setSortColumn("updatedAt");
                      kitEditorCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
                    ]}
                    className="px-1 min-w-0"
                  />
                </div>
                <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
              </th>
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("kitEditor.created")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "createdAt"}
                    value={sortColumn === "createdAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      kitEditorCommands.setSortColumn("createdAt");
                      kitEditorCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
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
                <tr key={row.id} className={`border-b ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`} onClick={(e) => handleRowClick(row, e)} role="button" tabIndex={0}>
                  <td className="p-1" onClick={(e) => e.stopPropagation()}>
                    <div className="flex items-center gap-1 justify-between" style={{ paddingLeft: `${row.level * 24}px` }}>
                      <div className="flex items-center gap-1 flex-1 min-w-0">
                        {row.hasChildren ? (
                          <Action level="base" onClick={(e) => { e.stopPropagation(); toggleRow(row.id); }}>
                            {row.isExpanded ? <ChevronDown /> : <ChevronRight />}
                          </Action>
                        ) : (
                          <span className="w-4 h-4 shrink-0" />
                        )}
                        <span
                          className="hover:underline text-left flex-1 min-w-0 truncate"
                          onClick={(e) => {
                            e.stopPropagation();
                            if (row.kind === "designs") sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
                            else if (row.kind === "types") sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
                          }}
                          role="link"
                          tabIndex={0}
                        >
                          {row.artifact}
                        </span>
                      </div>
                      <div className="flex items-center gap-0.5 shrink-0">
                        {row.kind === "designs" && row.level === 0 && (
                          <>
                            <Action
                              onClick={(e) => {
                                e.stopPropagation();
                                handleCreateVariantForRow(row);
                              }}
                              tooltip={t("kitEditor.createVariant")}
                              level="base"
                            >
                              <Plus />
                            </Action>
                            <Action
                              onClick={(e) => {
                                e.stopPropagation();
                                handleCreateViewForRow(row);
                              }}
                              tooltip={t("kitEditor.createView")}
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
                            tooltip={t("kitEditor.createVariant")}
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
                            tooltip={t("kitEditor.createView")}
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

class ErrorBoundary extends React.Component<
  { children: React.ReactNode; fallback: React.ReactNode },
  { hasError: boolean; error: Error | null }
> {
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

const Editor: FC = () => {
  const { t } = useTranslation();
  const kitScope = useKitScope();
  const sketchpadStore = useSketchpadStore();
  const hasKit = kitScope?.guid ? sketchpadStore.hasKit(kitScope.guid) : false;

  if (!hasKit) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">{t("kit.noKitLoaded")}</p>
      </div>
    );
  }

  return (
    <ErrorBoundary
      fallback={
        <div className="flex items-center justify-center h-full">
          <p className="text-sm text-muted-foreground">{t("kit.noKitLoaded")}</p>
        </div>
      }
    >
      <EditorContent />
    </ErrorBoundary>
  );
};

export default Editor;
