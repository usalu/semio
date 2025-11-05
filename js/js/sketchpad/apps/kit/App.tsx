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

import { DndContext, DragEndEvent, DragOverEvent, DragStartEvent, PointerSensor, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import { ArrowDown, ArrowUp, Award, Box, FileCode, FileImage, FileJson, FileSpreadsheet, FileText, FileType, FileVideo, Folder as FolderIcon, Layout, Plus, User } from "lucide-react";
import React, { FC, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ScrollArea } from "../../../elements/aggregation/ScrollArea";
import { TableAvatar } from "../../../elements/display/Avatar";
import { Action } from "../../../elements/input/Action";
import { Input } from "../../../elements/input/Input";
import { Toggle } from "../../../elements/input/Toggle";
import i18n from "../../../i18n";
import { Author, buildFileTree, Design, flattenFileTree, Folder, generateUniqueName, guid, Kit, Quality, File as SemioFile, Type } from "../../../semio";
import { Canvas, Window } from "../../Canvas";
import { ConceptFilter } from "../../ConceptFilter";
import { useAddPanelSection, useFocus, useRemovePanelSection } from "../../Navbar";
import { useAppType, useHasKit, useIsMobile, useKit, useKitCommands, useKitScope, useNavigation, useSketchpadCommands } from "../../store";
import { DesignSection, FileSection, FolderSection, KitSection, MultipleArtifactsSection, TypeSection } from "./panels/Details";
import { KitAppState, useKitApp, useKitAppCommands } from "./store";

type ArtifactKind = "designs" | "types" | "qualities" | "files" | "folders" | "authors";

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
  data: Design | Type | Quality | SemioFile | Author | Folder;
  folderId?: string;
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

const getFileIcon = (fileName: string) => {
  if (!fileName) return <FileText className="size-4" />;
  const extension = fileName.split(".").pop()?.toLowerCase();
  switch (extension) {
    case "jpg":
    case "jpeg":
    case "png":
    case "gif":
    case "svg":
    case "webp":
    case "bmp":
      return <FileImage className="size-4" />;
    case "mp4":
    case "avi":
    case "mov":
    case "mkv":
    case "webm":
      return <FileVideo className="size-4" />;
    case "json":
      return <FileJson className="size-4" />;
    case "js":
    case "ts":
    case "jsx":
    case "tsx":
    case "py":
    case "java":
    case "cpp":
    case "c":
    case "h":
    case "cs":
    case "rb":
    case "go":
    case "rs":
    case "php":
    case "html":
    case "css":
    case "scss":
    case "xml":
      return <FileCode className="size-4" />;
    case "csv":
    case "xlsx":
    case "xls":
      return <FileSpreadsheet className="size-4" />;
    case "txt":
    case "md":
    case "pdf":
    case "doc":
    case "docx":
      return <FileType className="size-4" />;
    default:
      return <FileText className="size-4" />;
  }
};

const getRowIcon = (row: TableRow): string | React.ReactNode | undefined => {
  switch (row.kind) {
    case "designs":
      return (row.data as Design).icon;
    case "types":
      return (row.data as Type).icon;
    case "qualities":
      return (row.data as Quality).icon;
    case "files":
      return getFileIcon((row.data as SemioFile).name);
    case "folders":
      return <FolderIcon className="size-4" />;
    case "authors":
      return <User className="size-4" />;
    default:
      return undefined;
  }
};

const DroppableTableWrapper: FC<{ children: React.ReactNode }> = ({ children }) => {
  const { setNodeRef } = useDroppable({
    id: "canvas-root",
    data: { isCanvas: true },
  });

  return (
    <div ref={setNodeRef} className="w-full min-h-full">
      {children}
    </div>
  );
};

const DraggableRow: FC<{
  row: TableRow;
  isSelected: boolean;
  isDraggedOver: boolean;
  isDragging: boolean;
  onRowClick: (row: TableRow, e: React.MouseEvent) => void;
  onRowDoubleClick: (row: TableRow) => void;
  toggleRow: (rowId: string) => void;
  handleCreateChildForRow: (row: TableRow) => void;
  isMobile: boolean;
  selectedKind?: ArtifactKind | null;
}> = ({ row, isSelected, isDraggedOver, isDragging, onRowClick, onRowDoubleClick, toggleRow, handleCreateChildForRow, isMobile, selectedKind }) => {
  // Allow dragging all items except authors
  let canDrag = row.kind !== "authors";

  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    isDragging: isDraggingHook,
  } = useDraggable({
    id: row.id,
    disabled: !canDrag,
    data: { row },
  });
  const { setNodeRef: setDroppableRef, isOver } = useDroppable({
    id: row.id,
    data: { row },
  });

  const style = transform
    ? {
      transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      opacity: isDraggingHook ? 0.5 : 1,
    }
    : undefined;

  const combinedRef = (node: HTMLElement | null) => {
    setNodeRef(node);
    setDroppableRef(node);
  };

  const isOverHighlight = isOver && (row.kind === "folders" || row.folderId);

  if (isMobile) {
    return (
      <div
        ref={combinedRef}
        style={style}
        className={`border-b p-2 cursor-selectable ${isSelected ? "bg-active-base text-active-foreground" : isOverHighlight ? "bg-hover-base ring-2 ring-active" : "hover:bg-hover-base"} ${isDraggingHook ? "opacity-50" : ""}`}
        onClick={(e) => onRowClick(row, e)}
        onDoubleClick={() => onRowDoubleClick(row)}
        {...(canDrag ? { ...attributes, ...listeners } : {})}
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
            <TableAvatar name={row.artifact} icon={getRowIcon(row)} />
            <span className="text-left flex-1 min-w-0 truncate">{row.artifact}</span>
          </div>
          <div className="flex items-center gap-0.5 shrink-0">
            {(row.kind === "designs" || row.kind === "types") && (
              <Action
                onClick={(e) => {
                  e.stopPropagation();
                  handleCreateChildForRow(row);
                }}
                id="semio.sketchpad.app.kit.kitApp.createChild"
                level="base"
              >
                <Plus />
              </Action>
            )}
          </div>
        </div>
      </div>
    );
  }

  return (
    <tr
      ref={combinedRef}
      style={style}
      className={`border-b cursor-selectable ${isSelected ? "bg-active-base text-active-foreground" : isOverHighlight ? "bg-hover-base ring-2 ring-active" : "hover:bg-hover-base"} ${isDraggingHook ? "opacity-50" : ""}`}
      onClick={(e) => onRowClick(row, e)}
      onDoubleClick={() => onRowDoubleClick(row)}
      {...(canDrag ? { ...attributes, ...listeners } : {})}
      role="button"
      tabIndex={0}
    >
      {!isMobile && !selectedKind && (
        <td className="p-1">
          {row.kind === "designs" && <Layout className="size-4" />}
          {row.kind === "types" && <Box className="size-4" />}
          {row.kind === "qualities" && <Award className="size-4" />}
          {row.kind === "files" && <FileText className="size-4" />}
          {row.kind === "folders" && <FolderIcon className="size-4" />}
          {row.kind === "authors" && <User className="size-4" />}
        </td>
      )}
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
            <TableAvatar name={row.artifact} icon={getRowIcon(row)} />
            <span className="text-left flex-1 min-w-0 truncate">{row.artifact}</span>
          </div>
          <div className="flex items-center gap-0.5 shrink-0">
            {(row.kind === "designs" || row.kind === "types") && (
              <Action
                onClick={(e) => {
                  e.stopPropagation();
                  handleCreateChildForRow(row);
                }}
                id="semio.sketchpad.app.kit.kitApp.createChild"
                level="base"
              >
                <Plus />
              </Action>
            )}
          </div>
        </div>
      </td>
      {!isMobile && <td className="p-1">{row.updatedAt}</td>}
      {!isMobile && <td className="p-1">{row.createdAt}</td>}
    </tr>
  );
};

const AppContent: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const params = useParams();
  const [searchParams, setSearchParams] = useSearchParams();

  const kitScope = useKitScope();
  const hasKit = useHasKit(kitScope?.guid || "");

  const kit = useKit(undefined, undefined, true) as Kit;
  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitAppCommands = useKitAppCommands();
  const kitApp = useKitApp() as KitAppState;
  const isMobile = useIsMobile();

  const [isDragOver, setIsDragOver] = React.useState(false);
  const [activeId, setActiveId] = React.useState<string | null>(null);
  const [overId, setOverId] = React.useState<string | null>(null);
  const sensors = useSensors(useSensor(PointerSensor, { activationConstraint: { distance: 8 } }));

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

  // Get filters from search params (?kind=&name=)
  const selectedKind = searchParams.get("kind") as ArtifactKind | null;
  const selectedName = searchParams.get("name");

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
    folders: kitApp?.selection?.folders || [],
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
  // Names are shown hierarchically based on selectedName filter
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();

    // Helper to get visible names from a hierarchy
    const collectVisibleNames = <T extends { guid: string; name: string; parent?: string }>(entities: T[] | undefined) => {
      if (!entities) return;

      if (!selectedName) {
        // No name selected - show all root entity names
        const rootEntities = entities.filter((e) => !e.parent);
        rootEntities.forEach((e) => nameSet.add(e.name));
      } else {
        // Name is selected - show children names of all entities with that name
        const matchingEntities = entities.filter((e) => e.name === selectedName);
        matchingEntities.forEach((parent) => {
          const children = entities.filter((e) => e.parent === parent.guid);
          children.forEach((child) => nameSet.add(child.name));
        });
      }
    };

    if (!selectedKind || selectedKind === "designs") {
      collectVisibleNames(kit?.designs);
    }
    if (!selectedKind || selectedKind === "types") {
      collectVisibleNames(kit?.types);
    }

    return Array.from(nameSet).sort();
  }, [kit?.designs, kit?.types, selectedKind, selectedName]);

  useEffect(() => {
    if (appType !== "kit") {
      return;
    }

    const selection = kitApp?.selection;
    const typesCount = selection?.types?.length || 0;
    const designsCount = selection?.designs?.length || 0;
    const qualitiesCount = selection?.qualities?.length || 0;
    const filesCount = selection?.files?.length || 0;
    const foldersCount = selection?.folders?.length || 0;
    const authorsCount = selection?.authors?.length || 0;
    const totalSelectedKinds = [typesCount > 0, designsCount > 0, qualitiesCount > 0, filesCount > 0, foldersCount > 0, authorsCount > 0].filter(Boolean).length;

    const artifactsMultipleId = "semio.sketchpad.app.kit.artifacts.multiple";

    removeSection("details", artifactsMultipleId);
    removeSection("details", "semio.sketchpad.app.design.title");
    removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
    removeSection("details", "semio.sketchpad.app.type.title");
    removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.file.title");
    removeSection("details", "semio.sketchpad.app.kit.files.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.folder.title");
    removeSection("details", "semio.sketchpad.app.kit.folders.multipleTitle");
    removeSection("details", "semio.sketchpad.app.kit.title");

    if (totalSelectedKinds > 1) {
      addSection("details", {
        id: artifactsMultipleId,
        translationParams: { count: totalSelectedKinds },
        order: 0,
        content: () => <MultipleArtifactsSection />,
      });
    }

    if (designsCount > 0 && totalSelectedKinds === 1) {
      const designSectionId = designsCount === 1 ? "semio.sketchpad.app.design.title" : "semio.sketchpad.app.kit.designs.multipleTitle";
      addSection("details", {
        id: designSectionId,
        translationParams: designsCount === 1 ? undefined : { count: designsCount },
        order: 10,
        content: () => <DesignSection />,
      });
    }

    if (typesCount > 0 && totalSelectedKinds === 1) {
      const typeSectionId = typesCount === 1 ? "semio.sketchpad.app.type.title" : "semio.sketchpad.app.kit.types.multipleTitle";
      addSection("details", {
        id: typeSectionId,
        translationParams: typesCount === 1 ? undefined : { count: typesCount },
        order: 20,
        content: () => <TypeSection />,
      });
    }

    if (filesCount > 0 && totalSelectedKinds === 1) {
      const fileSectionId = filesCount === 1 ? "semio.sketchpad.app.kit.file.title" : "semio.sketchpad.app.kit.files.multipleTitle";
      addSection("details", {
        id: fileSectionId,
        translationParams: filesCount === 1 ? undefined : { count: filesCount },
        order: 30,
        content: () => <FileSection />,
      });
    }

    if (foldersCount > 0 && totalSelectedKinds === 1) {
      const folderSectionId = foldersCount === 1 ? "semio.sketchpad.app.kit.folder.title" : "semio.sketchpad.app.kit.folders.multipleTitle";
      addSection("details", {
        id: folderSectionId,
        translationParams: foldersCount === 1 ? undefined : { count: foldersCount },
        order: 40,
        content: () => <FolderSection />,
      });
    }

    addSection("details", {
      id: "semio.sketchpad.app.kit.title",
      order: 100,
      content: () => <KitSection />,
    });

    return () => {
      removeSection("details", artifactsMultipleId);
      removeSection("details", "semio.sketchpad.app.design.title");
      removeSection("details", "semio.sketchpad.app.kit.designs.multipleTitle");
      removeSection("details", "semio.sketchpad.app.type.title");
      removeSection("details", "semio.sketchpad.app.kit.types.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.file.title");
      removeSection("details", "semio.sketchpad.app.kit.files.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.folder.title");
      removeSection("details", "semio.sketchpad.app.kit.folders.multipleTitle");
      removeSection("details", "semio.sketchpad.app.kit.title");
    };
  }, [addSection, removeSection, appType, kitApp?.selection]);

  // Auto-select design/type when select parameter is present
  useEffect(() => {
    if (!selectParam) return;

    if (selectedKind === "designs") {
      const design = kit.designs?.find((d: Design) => d.guid === selectParam);
      if (design) {
        kitAppCommands.selectDesign("semio.sketchpad.app.kit.autoselect.design", selectParam);
        // Remove the select parameter after selecting
        const newParams = new URLSearchParams(searchParams);
        newParams.delete("select");
        setSearchParams(newParams, { replace: true });
      }
    } else if (selectedKind === "types") {
      const type = kit.types?.find((t: Type) => t.guid === selectParam);
      if (type) {
        kitAppCommands.selectType("semio.sketchpad.app.kit.autoselect.type", selectParam);
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

      // Helper function to recursively build design hierarchy
      const buildDesignHierarchy = (designs: Design[], parentGuid: string | undefined, level: number, parentRowId?: string): void => {
        const childDesigns = designs.filter((d) => d.parent === parentGuid);

        childDesigns.forEach((design) => {
          if (selectedConcepts.length > 0 && !design.concepts?.some((c) => selectedConcepts.includes(c))) return;
          if (searchQuery && !design.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
          // Skip root designs that are in folders when not viewing the folders kind
          // Only filter at root level (parentGuid === undefined), not children
          if (!selectedKind && parentGuid === undefined && design.folder) return;

          const rowId = `design-${design.guid}`;
          const children = designs.filter((d) => d.parent === design.guid);
          const hasChildren = children.length > 0;

          result.push({
            id: rowId,
            kind: "designs",
            artifact: design.name,
            authors: design.authors?.join(", ") || "",
            updatedAt: formatDate(design.updatedAt),
            createdAt: formatDate(design.createdAt),
            level,
            parentId: parentRowId,
            hasChildren,
            isExpanded: expandedRows.has(rowId),
            data: design,
          });

          if (expandedRows.has(rowId) && hasChildren) {
            buildDesignHierarchy(designs, design.guid, level + 1, rowId);
          }
        });
      };

      // Apply name filter - if selectedName is set, only include designs with that name and their descendants
      const allDesignsArray = kit.designs || [];
      if (selectedName) {
        // Find all designs with the selected name
        const matchingDesignGuids = new Set(allDesignsArray.filter((d) => d.name === selectedName).map((d) => d.guid));

        // Collect all descendants of matching designs
        const includeGuids = new Set(matchingDesignGuids);
        const collectDescendants = (parentGuid: string) => {
          const children = allDesignsArray.filter((d) => d.parent === parentGuid);
          children.forEach((child) => {
            includeGuids.add(child.guid);
            collectDescendants(child.guid);
          });
        };
        matchingDesignGuids.forEach((guid) => collectDescendants(guid));

        // Filter to only included designs
        const filteredDesigns = allDesignsArray.filter((d) => includeGuids.has(d.guid));

        // Build hierarchy starting from matching designs (as roots)
        buildDesignHierarchy(filteredDesigns, undefined, 0);
      } else {
        // No name filter - start with root designs (no parent)
        buildDesignHierarchy(allDesignsArray, undefined, 0);
      }
    }

    if (!selectedKind || selectedKind === "types") {
      // Helper function to recursively build type hierarchy
      const buildTypeHierarchy = (types: Type[], parentGuid: string | undefined, level: number, parentRowId?: string): void => {
        const childTypes = types.filter((t) => t.parent === parentGuid);

        childTypes.forEach((type) => {
          if (searchQuery && !type.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
          // Skip root types that are in folders when not viewing the folders kind
          // Only filter at root level (parentGuid === undefined), not children
          if (!selectedKind && parentGuid === undefined && type.folder) return;

          const rowId = `type-${type.guid}`;
          const children = types.filter((t) => t.parent === type.guid);
          const hasChildren = children.length > 0;

          result.push({
            id: rowId,
            kind: "types",
            artifact: type.name,
            authors: type.authors?.join(", ") || "",
            updatedAt: formatDate(type.updatedAt),
            createdAt: formatDate(type.createdAt),
            level,
            parentId: parentRowId,
            hasChildren,
            isExpanded: expandedRows.has(rowId),
            data: type,
          });

          if (expandedRows.has(rowId) && hasChildren) {
            buildTypeHierarchy(types, type.guid, level + 1, rowId);
          }
        });
      };

      // Apply name filter - if selectedName is set, only include types with that name and their descendants
      const allTypesArray = kit.types || [];
      if (selectedName) {
        // Find all types with the selected name
        const matchingTypeGuids = new Set(allTypesArray.filter((t) => t.name === selectedName).map((t) => t.guid));

        // Collect all descendants of matching types
        const includeGuids = new Set(matchingTypeGuids);
        const collectDescendants = (parentGuid: string) => {
          const children = allTypesArray.filter((t) => t.parent === parentGuid);
          children.forEach((child) => {
            includeGuids.add(child.guid);
            collectDescendants(child.guid);
          });
        };
        matchingTypeGuids.forEach((guid) => collectDescendants(guid));

        // Filter to only included types
        const filteredTypes = allTypesArray.filter((t) => includeGuids.has(t.guid));

        // Build hierarchy starting from matching types (as roots)
        buildTypeHierarchy(filteredTypes, undefined, 0);
      } else {
        // No name filter - start with root types (no parent)
        buildTypeHierarchy(allTypesArray, undefined, 0);
      }
    }

    if (!selectedKind || selectedKind === "qualities") {
      kit.qualities?.forEach((quality: Quality) => {
        if (searchQuery && !quality.name.toLowerCase().includes(searchQuery.toLowerCase()) && !quality.key.toLowerCase().includes(searchQuery.toLowerCase())) return;
        // Skip qualities that are in folders when not viewing the folders kind
        if (!selectedKind && quality.folder) return;
        result.push({
          id: `quality-${quality.guid}`,
          kind: "qualities",
          artifact: quality.name,
          authors: quality.key,
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren: false,
          isExpanded: false,
          data: quality,
        });
      });
    }

    if (selectedKind === "files") {
      // Build file tree from files - only when specifically viewing files kind
      const fileTree = buildFileTree(kit.folders || [], kit.files || []);
      const flatTree = flattenFileTree(fileTree, 0, expandedRows);

      flatTree.forEach((node) => {
        if (searchQuery && !node.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

        result.push({
          id: `file-${node.path}`,
          kind: "files",
          artifact: node.name,
          authors: node.isDirectory ? `${node.children.length} items` : node.file?.size ? `${(node.file.size / 1024).toFixed(1)} KB` : "",
          updatedAt: node.file ? formatDate(node.file.updatedAt) : "",
          createdAt: node.file ? formatDate(node.file.createdAt) : "",
          level: node.level,
          parentId: node.parentPath ? `file-${node.parentPath}` : undefined,
          hasChildren: node.isDirectory && node.children.length > 0,
          isExpanded: node.isExpanded,
          data: node.file || ({ guid: node.path, name: node.name } as SemioFile),
        });
      });
    }

    if (!selectedKind || selectedKind === "folders") {
      // Helper function to recursively build folder hierarchy
      const buildFolderHierarchy = (parentFolder: Folder | null, level: number, parentRowId?: string): void => {
        const parentGuid = parentFolder?.guid;
        const childFolders = kit.folders?.filter((f: Folder) => f.parent === parentGuid) || [];

        childFolders.forEach((folder: Folder) => {
          if (searchQuery && !folder.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

          // Get artifacts in this folder
          const folderedDesigns = kit.designs?.filter((d: Design) => d.folder === folder.guid) || [];
          const folderedTypes = kit.types?.filter((t: Type) => t.folder === folder.guid) || [];
          const folderedQualities = kit.qualities?.filter((q: Quality) => q.folder === folder.guid) || [];
          const folderedFiles = kit.files?.filter((f: SemioFile) => f.folder === folder.guid) || [];
          const folderedSubFolders = kit.folders?.filter((f: Folder) => f.parent === folder.guid) || [];
          const folderedArtifacts = folderedDesigns.length + folderedTypes.length + folderedQualities.length + folderedFiles.length + folderedSubFolders.length;

          const folderId = `folder-${folder.guid}`;
          result.push({
            id: folderId,
            kind: "folders",
            artifact: folder.name,
            authors: `${folderedArtifacts} items`,
            updatedAt: formatDate(folder.updatedAt),
            createdAt: formatDate(folder.createdAt),
            level,
            hasChildren: folderedArtifacts > 0,
            isExpanded: expandedRows.has(folderId),
            data: folder,
            folderId: folder.parent,
            parentId: parentRowId,
          });

          // Add child artifacts if folder is expanded
          if (expandedRows.has(folderId)) {
            // Add designs in folder with their full hierarchy
            const rootFolderedDesigns = folderedDesigns.filter((d: Design) => !d.parent);
            rootFolderedDesigns.forEach((design: Design) => {
              if (!design.guid) return;
              const rowId = `design-${design.guid}`;
              const allDesigns = kit.designs || [];
              const children = allDesigns.filter((d) => d.parent === design.guid);
              const hasChildren = children.length > 0;

              result.push({
                id: rowId,
                kind: "designs",
                artifact: design.name,
                authors: (design.authors || []).join(", "),
                updatedAt: formatDate(design.updatedAt),
                createdAt: formatDate(design.createdAt),
                level: level + 1,
                hasChildren,
                isExpanded: expandedRows.has(rowId),
                data: design,
                folderId: folder.guid,
                parentId: folderId,
              });

              // Recursively add design children
              if (expandedRows.has(rowId) && hasChildren) {
                const buildDesignChildrenInFolder = (parentDesignGuid: string, childLevel: number, parentRowId: string): void => {
                  const childDesigns = allDesigns.filter((d) => d.parent === parentDesignGuid);
                  childDesigns.forEach((childDesign) => {
                    const childRowId = `design-${childDesign.guid}`;
                    const grandChildren = allDesigns.filter((d) => d.parent === childDesign.guid);
                    const hasGrandChildren = grandChildren.length > 0;

                    result.push({
                      id: childRowId,
                      kind: "designs",
                      artifact: childDesign.name,
                      authors: (childDesign.authors || []).join(", "),
                      updatedAt: formatDate(childDesign.updatedAt),
                      createdAt: formatDate(childDesign.createdAt),
                      level: childLevel,
                      hasChildren: hasGrandChildren,
                      isExpanded: expandedRows.has(childRowId),
                      data: childDesign,
                      folderId: folder.guid,
                      parentId: parentRowId,
                    });

                    if (expandedRows.has(childRowId) && hasGrandChildren) {
                      buildDesignChildrenInFolder(childDesign.guid, childLevel + 1, childRowId);
                    }
                  });
                };
                buildDesignChildrenInFolder(design.guid, level + 2, rowId);
              }
            });

            // Add types in folder with their full hierarchy
            const rootFolderedTypes = folderedTypes.filter((t: Type) => !t.parent);
            rootFolderedTypes.forEach((type: Type) => {
              if (!type.guid) return;
              const rowId = `type-${type.guid}`;
              const allTypes = kit.types || [];
              const children = allTypes.filter((t) => t.parent === type.guid);
              const hasChildren = children.length > 0;

              result.push({
                id: rowId,
                kind: "types",
                artifact: type.name,
                authors: (type.authors || []).join(", "),
                updatedAt: formatDate(type.updatedAt),
                createdAt: formatDate(type.createdAt),
                level: level + 1,
                hasChildren,
                isExpanded: expandedRows.has(rowId),
                data: type,
                folderId: folder.guid,
                parentId: folderId,
              });

              // Recursively add type children
              if (expandedRows.has(rowId) && hasChildren) {
                const buildTypeChildrenInFolder = (parentTypeGuid: string, childLevel: number, parentRowId: string): void => {
                  const childTypes = allTypes.filter((t) => t.parent === parentTypeGuid);
                  childTypes.forEach((childType) => {
                    const childRowId = `type-${childType.guid}`;
                    const grandChildren = allTypes.filter((t) => t.parent === childType.guid);
                    const hasGrandChildren = grandChildren.length > 0;

                    result.push({
                      id: childRowId,
                      kind: "types",
                      artifact: childType.name,
                      authors: (childType.authors || []).join(", "),
                      updatedAt: formatDate(childType.updatedAt),
                      createdAt: formatDate(childType.createdAt),
                      level: childLevel,
                      hasChildren: hasGrandChildren,
                      isExpanded: expandedRows.has(childRowId),
                      data: childType,
                      folderId: folder.guid,
                      parentId: parentRowId,
                    });

                    if (expandedRows.has(childRowId) && hasGrandChildren) {
                      buildTypeChildrenInFolder(childType.guid, childLevel + 1, childRowId);
                    }
                  });
                };
                buildTypeChildrenInFolder(type.guid, level + 2, rowId);
              }
            });

            // Add qualities in folder
            folderedQualities.forEach((quality: Quality) => {
              result.push({
                id: `quality-${quality.guid}`,
                kind: "qualities",
                artifact: quality.name,
                authors: "",
                updatedAt: "",
                createdAt: "",
                level: level + 1,
                hasChildren: false,
                isExpanded: false,
                data: quality,
                folderId: folder.guid,
                parentId: folderId,
              });
            });

            // Add files in folder
            folderedFiles.forEach((file: SemioFile) => {
              result.push({
                id: `file-${file.guid}`,
                kind: "files",
                artifact: file.name,
                authors: file.size ? `${(file.size / 1024).toFixed(1)} KB` : "",
                updatedAt: formatDate(file.updatedAt),
                createdAt: formatDate(file.createdAt),
                level: level + 1,
                hasChildren: false,
                isExpanded: false,
                data: file,
                folderId: folder.guid,
                parentId: folderId,
              });
            });

            // Recursively add child folders
            buildFolderHierarchy(folder, level + 1, folderId);
          }
        });
      };

      // Start with root folders (no parent)
      buildFolderHierarchy(null, 0);
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
  }, [kit, kit.files, selectedKind, selectedName, selectedConcepts, searchQuery, expandedRows, sortColumn, sortDirection]);

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
    kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.toggleRow", rowId);
  };

  const handleDragStart = (event: DragStartEvent) => {
    setActiveId(event.active.id as string);
  };

  const handleDragOver = (event: DragOverEvent) => {
    setOverId(event.over?.id as string | null);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    setActiveId(null);
    setOverId(null);

    if (!active) return;

    const draggedRow = rows.find((r) => r.id === active.id);
    if (!draggedRow) return;

    // Prevent dropping a folder onto itself
    if (draggedRow.kind === "folders" && over && over.id === active.id) {
      return;
    }

    let targetFolderId: string | undefined = undefined;
    let shouldExpandFolder = false;

    if (over) {
      // Check if dropped on canvas root (empty space in the table)
      if (over.id === "canvas-root") {
        // Dropped on canvas background - move to root (unset folder/parent)
        targetFolderId = undefined;
      } else {
        // Dropped on a row
        const targetRow = rows.find((r) => r.id === over.id);
        if (targetRow) {
          if (targetRow.kind === "folders") {
            // Dropped directly on a folder
            const folder = targetRow.data as Folder;
            targetFolderId = folder.guid;
            shouldExpandFolder = true;
          } else if (targetRow.folderId) {
            // Dropped on a non-folder child of a folder - move to parent folder
            targetFolderId = targetRow.folderId;
          } else {
            // Dropped on root-level row that's not a folder - move to root (unset folder/parent)
            targetFolderId = undefined;
          }
        } else {
          // No target row found - move to root
          targetFolderId = undefined;
        }
      }
    } else {
      // Dropped outside all droppable areas - move to root (unset folder/parent)
      targetFolderId = undefined;
    }

    // Don't move if already in the target location
    // For designs and types, check the actual folder property from the data
    let currentFolderId: string | undefined = undefined;
    let hasParent = false;

    if (draggedRow.kind === "designs") {
      const design = draggedRow.data as Design;
      currentFolderId = design.folder;
      hasParent = !!design.parent;
    } else if (draggedRow.kind === "types") {
      const type = draggedRow.data as Type;
      currentFolderId = type.folder;
      hasParent = !!type.parent;
    } else if (draggedRow.kind === "qualities") {
      currentFolderId = (draggedRow.data as Quality).folder;
    } else if (draggedRow.kind === "files") {
      currentFolderId = (draggedRow.data as SemioFile).folder;
    } else if (draggedRow.kind === "folders") {
      currentFolderId = (draggedRow.data as Folder).parent;
    }

    // If dropped on root and item has parent, allow (to unparent)
    // If dropped on root and item has no parent and no folder, skip (already at root)
    // Otherwise check if target is same as current location
    if (targetFolderId === undefined && !hasParent && !currentFolderId) return;
    if (targetFolderId !== undefined && currentFolderId === targetFolderId) return;

    if (draggedRow.kind === "designs" && kitCommands) {
      const design = draggedRow.data as Design;

      if (design.parent) {
        // Child design (variant) - only allow unparenting when dropped on root
        if (targetFolderId === undefined) {
          kitCommands.updateDesign("semio.sketchpad.app.kit.canvas.table.unparentDesign", design.guid, { parent: undefined });
        }
        // else: cannot move child designs to folders, do nothing
      } else {
        // Root design (protodesign) - can be moved to folders or root
        kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveDesignToFolder", "design", design.guid, targetFolderId);
      }
    } else if (draggedRow.kind === "types" && kitCommands) {
      const type = draggedRow.data as Type;

      if (type.parent) {
        // Child type (view) - only allow unparenting when dropped on root
        if (targetFolderId === undefined) {
          kitCommands.updateType("semio.sketchpad.app.kit.canvas.table.unparentType", type.guid, { parent: undefined });
        }
        // else: cannot move child types to folders, do nothing
      } else {
        // Root type (prototype) - can be moved to folders or root
        kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveTypeToFolder", "type", type.guid, targetFolderId);
      }
    } else if (draggedRow.kind === "qualities" && kitCommands) {
      const quality = draggedRow.data as Quality;
      kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveQualityToFolder", "quality", quality.guid, targetFolderId);
    } else if (draggedRow.kind === "files" && kitCommands) {
      const file = draggedRow.data as SemioFile;
      kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveFileToFolder", "file", file.guid, targetFolderId);
    } else if (draggedRow.kind === "folders" && kitCommands) {
      const folder = draggedRow.data as Folder;
      kitCommands.moveToFolder("semio.sketchpad.app.kit.canvas.table.moveFolderToFolder", "folder", folder.guid, targetFolderId);
    }

    // Expand the target folder if moving into a folder
    if (shouldExpandFolder && targetFolderId) {
      const folderId = `folder-${targetFolderId}`;
      if (!expandedRows.has(folderId)) {
        kitAppCommands.toggleExpandedRow("semio.sketchpad.app.kit.canvas.table.expandFolder", folderId);
      }
    }
  };

  const handleCreateArtifact = (kind: ArtifactKind) => {
    switch (kind) {
      case "designs": {
        const existingNames = (kit.designs || []).map((d: Design) => d.name);
        const uniqueName = generateUniqueName(t("semio.sketchpad.app.design.defaultName"), existingNames);
        const newDesign: Design = {
          guid: guid(),
          name: uniqueName,
          pieces: [],
          connections: [],
        };
        if (kitCommands) kitCommands.createDesign("semio.sketchpad.app.kit.canvas.table.createDesign", newDesign);
        sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
        break;
      }
      case "types": {
        const existingNames = (kit.types || []).map((t: Type) => t.name);
        const uniqueName = generateUniqueName(t("semio.sketchpad.app.type.defaultName"), existingNames);
        const newType: Type = {
          guid: guid(),
          name: uniqueName,
          ports: [],
        };
        if (kitCommands) kitCommands.createType("semio.sketchpad.app.kit.canvas.table.createType", newType);
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
        if (kitCommands) kitCommands.createQuality("semio.sketchpad.app.kit.canvas.table.createQuality", newQuality);
        sketchpadCommands.navigateToQuality(kit.guid, newQuality.guid);
        break;
      }
      case "files": {
        // TODO: Implement file creation
        break;
      }
      case "folders": {
        const existingNames = (kit.folders || []).map((f: Folder) => f.name);
        const uniqueName = generateUniqueName(t("semio.sketchpad.app.folder.defaultName"), existingNames);
        const newFolder: Folder = {
          guid: guid(),
          name: uniqueName,
        };
        if (kitCommands) kitCommands.createFolder("semio.sketchpad.app.kit.canvas.table.createFolder", newFolder);
        break;
      }
      case "authors": {
        // TODO: Implement author creation
        break;
      }
    }
  };

  const handleCreateChildForRow = (row: TableRow) => {
    if (row.kind === "designs") {
      const design = row.data as Design;
      const existingNames = (kit.designs || []).filter((d: Design) => d.parent === design.guid).map((d: Design) => d.name);
      const uniqueName = generateUniqueName(design.name, existingNames);
      const newDesign: Design = {
        guid: guid(),
        name: uniqueName,
        parent: design.guid,
        pieces: [],
        connections: [],
      };
      if (kitCommands) kitCommands.createDesign("semio.sketchpad.app.kit.canvas.table.createChild", newDesign);
      sketchpadCommands.navigateToDesign(kit.guid, newDesign.guid);
    } else if (row.kind === "types") {
      const type = row.data as Type;
      const existingNames = (kit.types || []).filter((t: Type) => t.parent === type.guid).map((t: Type) => t.name);
      const uniqueName = generateUniqueName(type.name, existingNames);
      const newType: Type = {
        guid: guid(),
        name: uniqueName,
        parent: type.guid,
        ports: [],
      };
      if (kitCommands) kitCommands.createType("semio.sketchpad.app.kit.canvas.table.createChild", newType);
      sketchpadCommands.navigateToType(kit.guid, newType.guid);
    }
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
            kitAppCommands.selectDesigns("semio.sketchpad.app.kit.canvas.table.selectDesignsRange", rangeIds);
          }
        } else {
          kitAppCommands.selectDesign("semio.sketchpad.app.kit.canvas.table.selectDesignShift", designId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.designs.includes(designId)) {
          kitAppCommands.removeDesignFromSelection("semio.sketchpad.app.kit.canvas.table.removeDesignCtrl", designId);
        } else {
          kitAppCommands.addDesignToSelection("semio.sketchpad.app.kit.canvas.table.addDesignCtrl", designId);
        }
      } else {
        kitAppCommands.selectDesign("semio.sketchpad.app.kit.canvas.table.selectDesign", designId);
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
            kitAppCommands.selectTypes("semio.sketchpad.app.kit.canvas.table.selectTypesRange", rangeIds);
          }
        } else {
          kitAppCommands.selectType("semio.sketchpad.app.kit.canvas.table.selectTypeShift", typeId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.types.includes(typeId)) {
          kitAppCommands.removeTypeFromSelection("semio.sketchpad.app.kit.canvas.table.removeTypeCtrl", typeId);
        } else {
          kitAppCommands.addTypeToSelection("semio.sketchpad.app.kit.canvas.table.addTypeCtrl", typeId);
        }
      } else {
        kitAppCommands.selectType("semio.sketchpad.app.kit.canvas.table.selectType", typeId);
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
            kitAppCommands.selectQualities("semio.sketchpad.app.kit.canvas.table.selectQualitiesRange", rangeKeys);
          }
        } else {
          kitAppCommands.selectQuality("semio.sketchpad.app.kit.canvas.table.selectQualityShift", qualityKey);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.qualities.includes(qualityKey)) {
          kitAppCommands.removeQualityFromSelection("semio.sketchpad.app.kit.canvas.table.removeQualityCtrl", qualityKey);
        } else {
          kitAppCommands.addQualityToSelection("semio.sketchpad.app.kit.canvas.table.addQualityCtrl", qualityKey);
        }
      } else {
        kitAppCommands.selectQuality("semio.sketchpad.app.kit.canvas.table.selectQuality", qualityKey);
      }
    } else if (row.kind === "files") {
      const fileGuid = (row.data as SemioFile).guid;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "files" && (r.data as SemioFile).guid === fileGuid);
        if (selection.files.length > 0) {
          const lastSelectedGuid = selection.files[selection.files.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "files" && (r.data as SemioFile).guid === lastSelectedGuid);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangeGuids = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "files")
              .map((r) => (r.data as SemioFile).guid);
            kitAppCommands.selectFiles("semio.sketchpad.app.kit.canvas.table.selectFilesRange", rangeGuids);
          }
        } else {
          kitAppCommands.selectFile("semio.sketchpad.app.kit.canvas.table.selectFileShift", fileGuid);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.files.includes(fileGuid)) {
          kitAppCommands.removeFileFromSelection("semio.sketchpad.app.kit.canvas.table.removeFileCtrl", fileGuid);
        } else {
          kitAppCommands.addFileToSelection("semio.sketchpad.app.kit.canvas.table.addFileCtrl", fileGuid);
        }
      } else {
        kitAppCommands.selectFile("semio.sketchpad.app.kit.canvas.table.selectFile", fileGuid);
      }
    } else if (row.kind === "folders") {
      const folderId = (row.data as Folder).guid;
      if (e.shiftKey) {
        const currentIndex = rows.findIndex((r) => r.kind === "folders" && (r.data as Folder).guid === folderId);
        if (selection.folders && selection.folders.length > 0) {
          const lastSelectedId = selection.folders[selection.folders.length - 1];
          const lastIndex = rows.findIndex((r) => r.kind === "folders" && (r.data as Folder).guid === lastSelectedId);
          if (lastIndex !== -1 && currentIndex !== -1) {
            const start = Math.min(lastIndex, currentIndex);
            const end = Math.max(lastIndex, currentIndex);
            const rangeIds = rows
              .slice(start, end + 1)
              .filter((r) => r.kind === "folders")
              .map((r) => (r.data as Folder).guid);
            kitAppCommands.selectFolders("semio.sketchpad.app.kit.canvas.table.selectFoldersRange", rangeIds);
          }
        } else {
          kitAppCommands.selectFolder("semio.sketchpad.app.kit.canvas.table.selectFolderShift", folderId);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.folders && selection.folders.includes(folderId)) {
          kitAppCommands.removeFolderFromSelection("semio.sketchpad.app.kit.canvas.table.removeFolderCtrl", folderId);
        } else {
          kitAppCommands.addFolderToSelection("semio.sketchpad.app.kit.canvas.table.addFolderCtrl", folderId);
        }
      } else {
        kitAppCommands.selectFolder("semio.sketchpad.app.kit.canvas.table.selectFolder", folderId);
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
            kitAppCommands.selectAuthors("semio.sketchpad.app.kit.canvas.table.selectAuthorsRange", rangeNames);
          }
        } else {
          kitAppCommands.selectAuthor("semio.sketchpad.app.kit.canvas.table.selectAuthorShift", authorName);
        }
      } else if (e.metaKey || e.ctrlKey) {
        if (selection.authors.includes(authorName)) {
          kitAppCommands.removeAuthorFromSelection("semio.sketchpad.app.kit.canvas.table.removeAuthorCtrl", authorName);
        } else {
          kitAppCommands.addAuthorToSelection("semio.sketchpad.app.kit.canvas.table.addAuthorCtrl", authorName);
        }
      } else {
        kitAppCommands.selectAuthor("semio.sketchpad.app.kit.canvas.table.selectAuthor", authorName);
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
    kitAppCommands.toggleSort("semio.sketchpad.app.kit.canvas.table.toggleSort", column);
  };

  const handleFileDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer.types.includes("Files")) {
      setIsDragOver(true);
    }
  };

  const handleFileDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  };

  const handleFileDrop = async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);

    const files = Array.from(e.dataTransfer.files);
    if (files.length === 0) return;

    for (const file of files) {
      // Check if file is a zip file
      if (file.name.toLowerCase().endsWith(".zip")) {
        try {
          console.log(`[DEBUG] Processing zip file: ${file.name}`);
          // Import JSZip dynamically
          const JSZip = (await import("jszip")).default;
          const zip = await JSZip.loadAsync(file);

          // Extract all files from zip
          const folderByGuid = new Map<string, Folder>();
          (kit.folders || []).forEach((existingFolder) => folderByGuid.set(existingFolder.guid, existingFolder));
          const folderPathCache = new Map<string, string>();
          const folderPathMap = new Map<string, string>();
          const resolvePath = (folder: Folder): string => {
            const cached = folderPathCache.get(folder.guid);
            if (cached) return cached;
            const parentFolder = folder.parent ? folderByGuid.get(folder.parent) : undefined;
            const path = parentFolder ? `${resolvePath(parentFolder)}/${folder.name}` : folder.name;
            folderPathCache.set(folder.guid, path);
            return path;
          };
          folderByGuid.forEach((folder) => {
            const path = resolvePath(folder);
            folderPathMap.set(path, folder.guid);
          });
          const ensureFolder = async (parts: string[]): Promise<string | undefined> => {
            let parentGuid: string | undefined = undefined;
            let currentPath = "";
            for (const part of parts) {
              currentPath = currentPath ? `${currentPath}/${part}` : part;
              let folderGuid = folderPathMap.get(currentPath);
              if (!folderGuid) {
                const newFolder: Folder = {
                  guid: guid(),
                  name: part,
                  parent: parentGuid,
                  createdAt: new Date(),
                  updatedAt: new Date(),
                };
                folderGuid = newFolder.guid;
                folderPathMap.set(currentPath, folderGuid);
                folderByGuid.set(folderGuid, newFolder);
                if (kitCommands) await kitCommands.createFolder("semio.sketchpad.app.kit.dropZip.createFolder", newFolder);
              }
              parentGuid = folderGuid;
            }
            return parentGuid;
          };
          let processedFiles = 0;
          for (const zipEntry of Object.values(zip.files)) {
            if (!zipEntry.dir) {
              const relativePath = zipEntry.name;
              const parts = relativePath.split("/").filter((part) => part.length > 0);
              const directories = parts.slice(0, -1);
              const parentFolderGuid = directories.length > 0 ? await ensureFolder(directories) : undefined;
              const fileBlob = await zipEntry.async("blob");
              const extractedFile: SemioFile = {
                guid: guid(),
                name: parts[parts.length - 1] || relativePath,
                path: relativePath,
                folder: parentFolderGuid,
                size: fileBlob.size,
                hash: undefined,
                createdAt: new Date(),
                updatedAt: new Date(),
              };
              await kitCommands?.addFile("semio.sketchpad.app.kit.dropZip", extractedFile, fileBlob);
              processedFiles += 1;
              console.log(`[DEBUG] Extracted and added file ${relativePath} from zip`);
            }
          }
          console.log(`[DEBUG] Successfully extracted ${processedFiles} files from ${file.name}`);
        } catch (error) {
          console.error(`Failed to extract zip file ${file.name}:`, error);
        }
      } else {
        // Handle regular file
        const newFile: SemioFile = {
          guid: guid(),
          name: file.name,
          path: file.name,
          size: file.size,
          hash: undefined,
          createdAt: new Date(),
          updatedAt: new Date(),
        };

        try {
          await kitCommands?.addFile("semio.sketchpad.app.kit.dropFile", newFile, file);
        } catch (error) {
          console.error(`Failed to add file ${file.name}:`, error);
        }
      }
    }
  };

  if (isMobile) {
    return (
      <div
        className="flex flex-col h-full"
        onClick={(e: React.MouseEvent) => {
          if (e.target === e.currentTarget) {
            kitAppCommands.deselectAll("semio.sketchpad.app.kit.canvas.table.deselect");
          }
        }}
      >
        {/* Concept Filter */}
        <ConceptFilter allConcepts={allConcepts} paramName="c" />
        {/* Flexible filter layout with automatic wrapping for mobile */}
        <div className="flex flex-wrap items-center gap-1 p-1 border-b">
          {selectedKind && (
            <Toggle
              type="withAction"
              pressed={true}
              onPressedChange={() => toggleKind(selectedKind)}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact(selectedKind)}
              id="semio.sketchpad.app.kit.kitApp.hideKind"
              actionId="semio.sketchpad.app.kit.kitApp.createArtifact"
            >
              {selectedKind === "designs" && <Layout className="size-4" />}
              {selectedKind === "types" && <Box className="size-4" />}
              {selectedKind === "qualities" && <Award className="size-4" />}
              {selectedKind === "files" && <FileText className="size-4" />}
              {selectedKind === "folders" && <FolderIcon className="size-4" />}
              {selectedKind === "authors" && <User className="size-4" />}
            </Toggle>
          )}
          {selectedName && (
            <Toggle pressed={true} onPressedChange={() => toggleName(selectedName)}>
              {selectedName}
            </Toggle>
          )}
          {selectedConcepts.length > 0 &&
            selectedConcepts.map((concept) => (
              <Toggle key={concept} pressed={true} onPressedChange={() => toggleConcept(concept)} id="semio.sketchpad.app.kit.filter.concept.hide">
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
                id="semio.sketchpad.app.kit.kitApp.showDesigns"
                actionId="semio.sketchpad.app.kit.kitApp.createDesign"
              >
                <Layout className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("types")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("types")}
                id="semio.sketchpad.app.kit.kitApp.showTypes"
                actionId="semio.sketchpad.app.kit.kitApp.createType"
              >
                <Box className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("qualities")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("qualities")}
                id="semio.sketchpad.app.kit.kitApp.showQualities"
                actionId="semio.sketchpad.app.kit.kitApp.createQuality"
              >
                <Award className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("files")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("files")}
                id="semio.sketchpad.app.kit.kitApp.showFiles"
                actionId="semio.sketchpad.app.kit.kitApp.createFile"
              >
                <FileText className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("folders")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("folders")}
                id="semio.sketchpad.app.kit.kitApp.showFolders"
                actionId="semio.sketchpad.app.kit.kitApp.createFolder"
              >
                <FolderIcon className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("authors")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateArtifact("authors")}
                id="semio.sketchpad.app.kit.kitApp.showAuthors"
                actionId="semio.sketchpad.app.kit.kitApp.createAuthor"
              >
                <User className="size-4" />
              </Toggle>
            </>
          )}
          {allConcepts.length > 0 &&
            allConcepts
              .filter((c) => !selectedConcepts.includes(c))
              .map((concept) => (
                <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} id="semio.sketchpad.app.kit.filter.concept.show">
                  {concept}
                </Toggle>
              ))}
          {selectedKind &&
            !selectedName &&
            uniqueNames.length > 0 &&
            uniqueNames.map((name) => (
              <Toggle key={name} pressed={false} onPressedChange={() => toggleName(name)} id="semio.sketchpad.app.kit.filter.name">
                {name}
              </Toggle>
            ))}
          <div className="flex items-center gap-1 flex-1 min-w-[160px]">
            <Input className="flex-1 min-w-0" placeholder={t("semio.sketchpad.common.search")} value={searchQuery} onChange={(e) => kitAppCommands.setFilterSearch("semio.sketchpad.app.kit.filter.search", e.target.value)} />
            <Toggle
              type="dropdown"
              pressed={sortColumn === "artifact"}
              value={sortColumn === "artifact" ? sortDirection : "asc"}
              onValueChange={(value) => {
                kitAppCommands.setSortColumn("semio.sketchpad.app.kit.filter.artifact.sortColumn", "artifact");
                kitAppCommands.setSortDirection("semio.sketchpad.app.kit.filter.artifact.sortDirection", value as "asc" | "desc");
              }}
              items={[
                { value: "asc", label: <ArrowUp className="size-3.5" />, id: "semio.sketchpad.app.sort.ascending" },
                { value: "desc", label: <ArrowDown className="size-3.5" />, id: "semio.sketchpad.app.sort.descending" },
              ]}
              id="semio.sketchpad.app.kit.kitApp.sortByName"
            />
          </div>
        </div>

        {/* Simplified table - only name column, no headers */}
        <DndContext sensors={sensors} onDragStart={handleDragStart} onDragOver={handleDragOver} onDragEnd={handleDragEnd}>
          <ScrollArea className="flex-1">
            <DroppableTableWrapper>
              <div className="flex flex-col">
                {rows.map((row) => {
                  const isSelected =
                    (row.kind === "designs" && selection.designs.includes((row.data as Design).guid)) ||
                    (row.kind === "types" && selection.types.includes((row.data as Type).guid)) ||
                    (row.kind === "qualities" && selection.qualities.includes((row.data as Quality).key)) ||
                    (row.kind === "files" && selection.files.includes((row.data as SemioFile).guid)) ||
                    (row.kind === "folders" && selection.folders.includes((row.data as Folder).guid)) ||
                    (row.kind === "authors" && selection.authors.includes((row.data as Author).name));
                  const isDraggedOver = overId === row.id && activeId !== row.id;
                  const isDragging = activeId === row.id;
                  return (
                    <DraggableRow
                      key={row.id}
                      row={row}
                      isSelected={isSelected}
                      isDraggedOver={isDraggedOver}
                      isDragging={isDragging}
                      onRowClick={handleRowClick}
                      onRowDoubleClick={handleRowDoubleClick}
                      toggleRow={toggleRow}
                      handleCreateChildForRow={handleCreateChildForRow}
                      isMobile={true}
                      selectedKind={selectedKind}
                    />
                  );
                })}
              </div>
            </DroppableTableWrapper>
          </ScrollArea>
        </DndContext>
      </div>
    );
  }

  return (
    <div
      className="flex flex-col h-full"
      onClick={(e: React.MouseEvent) => {
        if (e.target === e.currentTarget) {
          kitAppCommands.deselectAll("semio.sketchpad.app.kit.canvas.table.deselect");
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
            id="semio.sketchpad.app.kit.kitApp.hideKind"
            actionId="semio.sketchpad.app.kit.kitApp.createArtifact"
          >
            {selectedKind === "designs" && <Layout className="size-4" />}
            {selectedKind === "types" && <Box className="size-4" />}
            {selectedKind === "qualities" && <Award className="size-4" />}
            {selectedKind === "files" && <FileText className="size-4" />}
            {selectedKind === "folders" && <FolderIcon className="size-4" />}
            {selectedKind === "authors" && <User className="size-4" />}
          </Toggle>
        )}
        {selectedName && (
          <Toggle pressed={true} onPressedChange={() => toggleName(selectedName)}>
            {selectedName}
          </Toggle>
        )}
        {selectedConcepts.length > 0 &&
          selectedConcepts.map((concept) => (
            <Toggle key={concept} pressed={true} onPressedChange={() => toggleConcept(concept)} id="semio.sketchpad.app.kit.filter.concept.hide">
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
              id="semio.sketchpad.app.kit.kitApp.showDesigns"
              actionId="semio.sketchpad.app.kit.kitApp.createDesign"
            >
              <Layout className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("types")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("types")}
              id="semio.sketchpad.app.kit.kitApp.showTypes"
              actionId="semio.sketchpad.app.kit.kitApp.createType"
            >
              <Box className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("qualities")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("qualities")}
              id="semio.sketchpad.app.kit.kitApp.showQualities"
              actionId="semio.sketchpad.app.kit.kitApp.createQuality"
            >
              <Award className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("files")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("files")}
              id="semio.sketchpad.app.kit.kitApp.showFiles"
              actionId="semio.sketchpad.app.kit.kitApp.createFile"
            >
              <FileText className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("folders")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("folders")}
              id="semio.sketchpad.app.kit.kitApp.showFolders"
              actionId="semio.sketchpad.app.kit.kitApp.createFolder"
            >
              <FolderIcon className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("authors")}
              actionIcon={<Plus className="size-3.5" />}
              onActionClick={() => handleCreateArtifact("authors")}
              id="semio.sketchpad.app.kit.kitApp.showAuthors"
              actionId="semio.sketchpad.app.kit.kitApp.createAuthor"
            >
              <User className="size-4" />
            </Toggle>
          </>
        )}
        {allConcepts.length > 0 &&
          allConcepts
            .filter((c) => !selectedConcepts.includes(c))
            .map((concept) => (
              <Toggle key={concept} pressed={false} onPressedChange={() => toggleConcept(concept)} id="semio.sketchpad.app.kit.filter.concept.show">
                {concept}
              </Toggle>
            ))}
        {selectedKind &&
          !selectedName &&
          uniqueNames.length > 0 &&
          uniqueNames.map((name) => (
            <Toggle key={name} pressed={false} onPressedChange={() => toggleName(name)} id="semio.sketchpad.app.kit.filter.name">
              {name}
            </Toggle>
          ))}
        <Input className="flex-1 min-w-[200px]" placeholder={t("semio.sketchpad.common.search")} value={searchQuery} onChange={(e) => kitAppCommands.setFilterSearch("semio.sketchpad.app.kit.canvas.table.search", e.target.value)} />
      </div>
      <DndContext sensors={sensors} onDragStart={handleDragStart} onDragOver={handleDragOver} onDragEnd={handleDragEnd}>
        <ScrollArea ref={scrollAreaRef} className="flex-1" onDragOver={handleFileDragOver} onDragLeave={handleFileDragLeave} onDrop={handleFileDrop}>
          {isDragOver && (
            <div className="absolute inset-0 bg-active-base/50 border-2 border-dashed border-active-foreground flex items-center justify-center z-10">
              <div className="text-active-foreground text-lg font-medium">Drop files to add to kit</div>
            </div>
          )}
          <DroppableTableWrapper>
            <table className="w-full border-collapse">
              <thead className="sticky top-0 border-b">
                <tr className="h-9">
                  {!selectedKind && (
                    <th className="text-left p-1 font-medium relative group">
                      <div className="flex items-center justify-between w-full">
                        <span>{t("semio.sketchpad.app.kit.kind")}</span>
                        <Toggle
                          type="dropdown"
                          pressed={sortColumn === "kind"}
                          value={sortColumn === "kind" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.kind.sortColumn", "kind");
                            kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.kind.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <ArrowUp className="size-3.5" />, id: "semio.sketchpad.common.sort.ascending" },
                            { value: "desc", label: <ArrowDown className="size-3.5" />, id: "semio.sketchpad.common.sort.descending" },
                          ]}
                          className="px-1 min-w-0"
                        />
                      </div>
                      <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                    </th>
                  )}
                  <th className="text-left p-1 font-medium relative group">
                    <div className="flex items-center justify-between w-full">
                      <span>{t("semio.sketchpad.app.kit.name")}</span>
                      <Toggle
                        type="dropdown"
                        pressed={sortColumn === "artifact"}
                        value={sortColumn === "artifact" ? sortDirection : "asc"}
                        onValueChange={(value) => {
                          kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.artifact.sortColumn", "artifact");
                          kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.artifact.sortDirection", value as "asc" | "desc");
                        }}
                        items={[
                          { value: "asc", label: <ArrowUp className="size-3.5" />, id: "semio.sketchpad.common.sort.ascending" },
                          { value: "desc", label: <ArrowDown className="size-3.5" />, id: "semio.sketchpad.common.sort.descending" },
                        ]}
                        className="px-1 min-w-0"
                      />
                    </div>
                    <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                  </th>
                  <th className="text-left p-1 font-medium relative group">
                    <div className="flex items-center justify-between w-full">
                      <span>{t("semio.sketchpad.app.kit.lastUpdated")}</span>
                      <Toggle
                        type="dropdown"
                        pressed={sortColumn === "updatedAt"}
                        value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                        onValueChange={(value) => {
                          kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.updatedAt.sortColumn", "updatedAt");
                          kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.updatedAt.sortDirection", value as "asc" | "desc");
                        }}
                        items={[
                          { value: "asc", label: <ArrowUp className="size-3.5" />, id: "semio.sketchpad.common.sort.ascending" },
                          { value: "desc", label: <ArrowDown className="size-3.5" />, id: "semio.sketchpad.common.sort.descending" },
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
                          kitAppCommands.setSortColumn("semio.sketchpad.app.kit.header.createdAt.sortColumn", "createdAt");
                          kitAppCommands.setSortDirection("semio.sketchpad.app.kit.header.createdAt.sortDirection", value as "asc" | "desc");
                        }}
                        items={[
                          { value: "asc", label: <ArrowUp className="size-3.5" />, id: "semio.sketchpad.common.sort.ascending" },
                          { value: "desc", label: <ArrowDown className="size-3.5" />, id: "semio.sketchpad.common.sort.descending" },
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
                    (row.kind === "files" && selection.files.includes((row.data as SemioFile).guid)) ||
                    (row.kind === "folders" && selection.folders.includes((row.data as Folder).guid)) ||
                    (row.kind === "authors" && selection.authors.includes((row.data as Author).name));
                  const isDraggedOver = overId === row.id && activeId !== row.id;
                  const isDragging = activeId === row.id;
                  return (
                    <DraggableRow
                      key={row.id}
                      row={row}
                      isSelected={isSelected}
                      isDraggedOver={isDraggedOver}
                      isDragging={isDragging}
                      onRowClick={handleRowClick}
                      onRowDoubleClick={handleRowDoubleClick}
                      toggleRow={toggleRow}
                      handleCreateChildForRow={handleCreateChildForRow}
                      isMobile={false}
                      selectedKind={selectedKind}
                    />
                  );
                })}
              </tbody>
            </table>
          </DroppableTableWrapper>
        </ScrollArea>
      </DndContext>
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

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) { }

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
