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
import { ArrowDown, ArrowUp, Clock, Cloud, FileText, HardDrive, Plus } from "lucide-react";
import { FC, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router";
import { ScrollArea } from "../../../elements/aggregation/ScrollArea";
import { Action } from "../../../elements/input/Action";
import { Input } from "../../../elements/input/Input";
import { Toggle } from "../../../elements/input/Toggle";
import i18n from "../../../i18n";
import { generateUniqueName, guid, Kit, KitShallow } from "../../../semio";
import { Canvas, Window } from "../../Canvas";
import { ConceptFilter } from "../../ConceptFilter";
import { useHotkeys } from "../../hotkeys";
import { useAddPanelSection, useFocus, useRemovePanelSection } from "../../Navbar";
import { useAppType, useGetKitKind, useIsMobile, useKits, useNavigation, useSketchpadCommands, useTooltip } from "../../store";
import { docsRegistry } from "../docs/registry";
import { HomeAppFooter } from "./Footer";
import { KitSection } from "./panels/Details";
import { useHome, useHomeCommands } from "./store";

type KitStoreKind = "temporary" | "local" | "remote";

type TableRow = {
  id: string;
  name: string;
  level: number;
  parentId?: string;
  hasChildren: boolean;
  isExpanded: boolean;
  type: KitStoreKind | "docs";
  updatedAt: string;
  createdAt: string;
  kit?: KitShallow;
  docsPath?: string;
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

const Home: FC = ({}) => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams, setSearchParams] = useSearchParams();
  const kits = useKits();
  const getKitKind = useGetKitKind();
  const { createKit, navigateToKit } = useSketchpadCommands();

  const homeState = useHome() as any;
  const homeCommands = useHomeCommands();
  const isMobile = useIsMobile();
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const tooltip = useTooltip();

  const selection = homeState?.selection?.kits || [];

  // Dynamic details panel based on selection
  useEffect(() => {
    if (appType !== "home") return;

    const hasKits = selection.length > 0;
    const hasSingleKit = selection.length === 1;
    const hasMultipleKits = selection.length > 1;

    // Remove previous section
    removeSection("details", "semio.sketchpad.app.kit.title");
    removeSection("details", "semio.sketchpad.app.home.kits.multiple");

    // Only show section if something is selected
    if (hasKits) {
      addSection("details", {
        id: hasSingleKit ? "semio.sketchpad.app.kit.title" : "semio.sketchpad.app.home.kits.multiple",
        translationParams: hasSingleKit ? undefined : { count: selection.length },
        order: 0,
        content: () => <KitSection />,
      });
    }

    return () => {
      removeSection("details", "semio.sketchpad.app.kit.title");
      removeSection("details", "semio.sketchpad.app.home.kits.multiple");
    };
  }, [appType, addSection, removeSection, selection.length]);

  // Get filters from search params (?kind=&name=&version=)
  const selectedKind = searchParams.get("kind") as KitStoreKind | null;
  const selectedName = searchParams.get("name");
  const selectedVersion = searchParams.get("version");

  // Get search query from URL search params
  const searchQuery = searchParams.get("q") || "";

  // Get expanded rows from search params
  const expandedRowsParam = searchParams.getAll("e");
  const expandedRows = new Set(expandedRowsParam);

  const sortColumn = homeState?.sortColumn;
  const sortDirection = homeState?.sortDirection || "asc";

  // Collect unique names
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();
    kits.forEach((kit) => {
      const type = getKitKind(kit.guid) || "temporary";

      if (selectedKind && selectedKind !== type) return;
      nameSet.add(kit.name);
    });
    return Array.from(nameSet).sort();
  }, [kits, getKitKind, selectedKind]);

  // Collect unique versions for the selected name
  const uniqueVersions = useMemo(() => {
    if (!selectedName) return [];
    const versionSet = new Set<string>();
    kits.forEach((kit) => {
      if (kit.name === selectedName) {
        versionSet.add(kit.version || "");
      }
    });
    return Array.from(versionSet).sort();
  }, [kits, selectedName]);

  // Collect all unique concepts from kits
  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kits.forEach((kit) => {
      kit.concepts?.forEach((concept) => conceptSet.add(concept));
    });
    return Array.from(conceptSet).sort();
  }, [kits]);

  // Get selected concepts from search params
  const selectedConcepts = useMemo(() => {
    const conceptsParam = searchParams.get("concepts");
    return conceptsParam ? conceptsParam.split(",").filter(Boolean) : [];
  }, [searchParams]);

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date) => {
      if (!date) return "";
      const parsedDate = date instanceof Date ? date : new Date(date);
      if (isNaN(parsedDate.getTime())) return "";
      return formatDistanceToNow(parsedDate, { addSuffix: true, locale });
    };

    // Add Docs section at the top
    const allDocsPages = docsRegistry.getAllPages();
    const allDocsSections = docsRegistry.getAllSections();

    const docsParentId = "docs-root";
    result.push({
      id: docsParentId,
      name: "Documentation",
      level: 0,
      hasChildren: true,
      isExpanded: expandedRows.has(docsParentId),
      type: "docs",
      updatedAt: "",
      createdAt: "",
      kit: undefined,
      docsPath: undefined,
    });

    if (expandedRows.has(docsParentId)) {
      allDocsSections.forEach((section) => {
        const sectionId = `docs-section-${section.id}`;
        const sectionPages = docsRegistry.getPagesBySection(section.id);

        result.push({
          id: sectionId,
          name: section.label,
          level: 1,
          parentId: docsParentId,
          hasChildren: sectionPages.length > 0,
          isExpanded: expandedRows.has(sectionId),
          type: "docs",
          updatedAt: "",
          createdAt: "",
          kit: undefined,
          docsPath: undefined,
        });

        if (expandedRows.has(sectionId)) {
          sectionPages.forEach((page) => {
            result.push({
              id: `docs-page-${page.path}`,
              name: page.title,
              level: 2,
              parentId: sectionId,
              hasChildren: false,
              isExpanded: false,
              type: "docs",
              updatedAt: "",
              createdAt: "",
              kit: undefined,
              docsPath: page.path,
            });
          });
        }
      });
    }

    const kitGroups = new Map<string, KitShallow[]>();

    kits.forEach((kit) => {
      const type = getKitKind(kit.guid) || "temporary";

      if (selectedKind && selectedKind !== type) return;
      if (searchQuery && !kit.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
      if (selectedName && kit.name !== selectedName) return;
      if (selectedVersion && (kit.version || "") !== selectedVersion) return;
      if (selectedConcepts.length > 0 && !kit.concepts?.some((c) => selectedConcepts.includes(c))) return;

      const key = kit.name;
      if (!kitGroups.has(key)) kitGroups.set(key, []);
      kitGroups.get(key)!.push(kit);
    });

    kitGroups.forEach((groupKits, name) => {
      const parentId = `kit-${name}`;
      const defaultKit = groupKits.find((k) => !k.version);
      const parentKit = defaultKit || groupKits[0];
      const hasChildren = groupKits.some((k) => k.guid !== parentKit.guid);

      const type = getKitKind(parentKit.guid) || "temporary";

      result.push({
        id: parentId,
        name: name,
        level: 0,
        hasChildren,
        isExpanded: expandedRows.has(parentId),
        type,
        updatedAt: formatDate(parentKit.updatedAt),
        createdAt: formatDate(parentKit.createdAt),
        kit: parentKit,
      });

      if (expandedRows.has(parentId) && hasChildren) {
        groupKits.forEach((kit) => {
          if (kit.guid === parentKit.guid) return;
          const kitKind = getKitKind(kit.guid) || "temporary";

          const versionId = `${parentId}-${kit.version || "default"}`;
          result.push({
            id: versionId,
            name: kit.version || "(default)",
            level: 1,
            parentId,
            hasChildren: false,
            isExpanded: false,
            type: kitKind,
            updatedAt: formatDate(kit.updatedAt),
            createdAt: formatDate(kit.createdAt),
            kit: kit,
          });
        });
      }
    });

    if (sortColumn) {
      const topLevelRows = result.filter((r) => r.level === 0);
      const childRows = result.filter((r) => r.level > 0);
      topLevelRows.sort((a, b) => {
        let comparison = 0;
        switch (sortColumn) {
          case "name":
            comparison = a.name.localeCompare(b.name);
            break;
          case "type":
            comparison = a.type.localeCompare(b.type);
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
      topLevelRows.forEach((parent) => {
        sortedResult.push(parent);
        const children = childRows.filter((c) => c.parentId === parent.id);
        children.sort((a, b) => {
          let comparison = 0;
          switch (sortColumn) {
            case "name":
              comparison = a.name.localeCompare(b.name);
              break;
            case "type":
              comparison = a.type.localeCompare(b.type);
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
        sortedResult.push(...children);
      });
      return sortedResult;
    }

    return result;
  }, [kits, getKitKind, selectedKind, searchQuery, selectedName, selectedVersion, expandedRows, sortColumn, sortDirection]);

  const { setFocusItems, setOnFocusItem } = useFocus();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const prevRowsRef = useRef<string>("");

  useEffect(() => {
    const items = rows.map((row) => ({
      id: row.id,
      label: row.name,
      category: row.level === 0 ? "Kits" : "Versions",
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

  const handleCreateKit = (type: KitStoreKind) => {
    const existingNames = kits.map((k) => k.name);
    const uniqueName = generateUniqueName(t("semio.sketchpad.app.kit.defaultName"), existingNames);
    const newKit: Kit = {
      guid: guid(),
      name: uniqueName,
      version: "",
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit("semio.sketchpad.app.home.canvas.table.createKit", newKit, local, remote);
    navigateToKit(newKit.guid);
  };

  const handleCreateVersion = (kitName: string, type: KitStoreKind) => {
    const existingVersions = kits.filter((k) => k.name === kitName).map((k) => k.version || "");
    const uniqueVersion = generateUniqueName(t("semio.sketchpad.app.kit.newVersion"), existingVersions);
    const newKit: Kit = {
      guid: guid(),
      name: kitName,
      version: uniqueVersion,
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit("semio.sketchpad.app.home.canvas.table.createVersion", newKit, local, remote);
    navigateToKit(newKit.guid);
  };

  const toggleKind = (type: KitStoreKind) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedKind === type) {
      newParams.delete("kind");
      newParams.delete("name");
      newParams.delete("version");
    } else {
      newParams.set("kind", type);
      newParams.delete("name");
      newParams.delete("version");
    }
    setSearchParams(newParams);
  };

  // Register hotkeys for filter toggles
  useHotkeys("semio.sketchpad.app.home.filter.kind.temporary.hotkey", () => toggleKind("temporary"));
  useHotkeys("semio.sketchpad.app.home.filter.kind.local.hotkey", () => toggleKind("local"));
  useHotkeys("semio.sketchpad.app.home.filter.kind.remote.hotkey", () => toggleKind("remote"));

  const toggleName = (name: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedName === name) {
      newParams.delete("name");
      newParams.delete("version");
    } else {
      newParams.set("name", name);
      newParams.delete("version");
    }
    setSearchParams(newParams);
  };

  const toggleVersion = (version: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedVersion === version) {
      newParams.delete("version");
    } else {
      newParams.set("version", version);
    }
    setSearchParams(newParams);
  };

  const toggleRow = (rowId: string) => {
    const newParams = new URLSearchParams(searchParams);
    const currentRows = newParams.getAll("e");

    if (currentRows.includes(rowId)) {
      // Remove row
      newParams.delete("e");
      currentRows.filter((r) => r !== rowId).forEach((r) => newParams.append("e", r));
    } else {
      // Add row
      newParams.append("e", rowId);
    }

    setSearchParams(newParams);
  };

  const handleSearchChange = (value: string) => {
    const newParams = new URLSearchParams(searchParams);
    if (value) {
      newParams.set("q", value);
    } else {
      newParams.delete("q");
    }
    setSearchParams(newParams);
  };

  const handleRowClick = (kitId: string, e: React.MouseEvent) => {
    if (e.shiftKey) {
      const currentIndex = rows.findIndex((r) => r.kit.guid === kitId);
      if (selection.length > 0) {
        const lastSelectedId = selection[selection.length - 1];
        const lastIndex = rows.findIndex((r) => r.kit.guid === lastSelectedId);
        if (lastIndex !== -1 && currentIndex !== -1) {
          const start = Math.min(lastIndex, currentIndex);
          const end = Math.max(lastIndex, currentIndex);
          const rangeIds = rows.slice(start, end + 1).map((r) => r.kit.guid);
          homeCommands.selectKits("semio.sketchpad.app.home.canvas.table.selectKitsRange", rangeIds);
        }
      } else {
        homeCommands.selectKit("semio.sketchpad.app.home.canvas.table.selectKitShift", kitId);
      }
    } else if (e.metaKey || e.ctrlKey) {
      if (selection.includes(kitId)) {
        homeCommands.removeKitFromSelection("semio.sketchpad.app.home.canvas.table.removeKitCtrl", kitId);
      } else {
        homeCommands.addKitToSelection("semio.sketchpad.app.home.canvas.table.addKitCtrl", kitId);
      }
    } else {
      homeCommands.selectKit("semio.sketchpad.app.home.canvas.table.selectKit", kitId);
    }
  };

  const handleSortClick = (column: "name" | "type" | "updatedAt" | "createdAt") => {
    homeCommands.toggleSort("semio.sketchpad.app.home.canvas.table.toggleSort", column);
  };

  if (isMobile) {
    return (
      <div
        className="flex flex-col h-full"
        onClick={(e: React.MouseEvent) => {
          if (e.target === e.currentTarget) {
            homeCommands.deselectAll("semio.sketchpad.app.home.canvas.table.deselect");
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
              onActionClick={() => handleCreateKit(selectedKind)}
              id="semio.sketchpad.app.home.filter.kind.show"
              actionId="semio.sketchpad.app.home.filter.kind.create"
            >
              {selectedKind === "temporary" && <Clock className="size-4" />}
              {selectedKind === "local" && <HardDrive className="size-4" />}
              {selectedKind === "remote" && <Cloud className="size-4" />}
            </Toggle>
          )}
          {selectedName && (
            <Toggle pressed={true} onPressedChange={() => toggleName(selectedName)} id="semio.sketchpad.app.home.filter.name">
              {selectedName}
            </Toggle>
          )}
          {selectedVersion !== null && (
            <Toggle pressed={true} onPressedChange={() => toggleVersion(selectedVersion)} id="semio.sketchpad.app.home.filter.version">
              {selectedVersion || <span className="italic opacity-50">{t("semio.sketchpad.app.kit.defaultVersion")}</span>}
            </Toggle>
          )}
          {!selectedKind && (
            <>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("temporary")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateKit("temporary")}
                id="semio.sketchpad.app.home.filter.kind.temporary"
                actionId="semio.sketchpad.app.home.filter.kind.createTemporary"
              >
                <Clock className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("local")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateKit("local")}
                id="semio.sketchpad.app.home.filter.kind.local"
                actionId="semio.sketchpad.app.home.filter.kind.createLocal"
              >
                <HardDrive className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("remote")}
                actionIcon={<Plus className="size-3.5" />}
                onActionClick={() => handleCreateKit("remote")}
                id="semio.sketchpad.app.home.filter.kind.remote"
                actionId="semio.sketchpad.app.home.filter.kind.createRemote"
              >
                <Cloud className="size-4" />
              </Toggle>
            </>
          )}
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
            selectedVersion === null &&
            uniqueVersions.length > 0 &&
            uniqueVersions.map((version) => (
              <Toggle key={version} pressed={false} onPressedChange={() => toggleVersion(version)}>
                {version || <span className="italic opacity-50">{t("semio.sketchpad.app.kit.defaultVersion")}</span>}
              </Toggle>
            ))}
          <div className="flex items-center gap-1 flex-1 min-w-[160px]">
            <Input className="flex-1 min-w-0" placeholder={t("semio.sketchpad.app.home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />
            <Toggle
              type="dropdown"
              value={sortColumn === "name" ? sortDirection : "asc"}
              onValueChange={(value) => {
                homeCommands.setSortColumn("semio.sketchpad.app.home.filter.name.sortColumn", "name");
                homeCommands.setSortDirection("semio.sketchpad.app.home.header.name.sortDirection", value as "asc" | "desc");
              }}
              items={[
                { value: "asc", label: <ArrowUp className="size-3.5" />, id: tooltip("sort.ascending") },
                { value: "desc", label: <ArrowDown className="size-3.5" />, id: tooltip("sort.descending") },
              ]}
              id={tooltip("home.sortByName")}
            />
          </div>
        </div>

        {/* Simplified table - only name column, no headers */}
        <ScrollArea className="flex-1">
          <div className="flex flex-col">
            {rows.map((row) => {
              const isSelected = row.kit ? selection.includes(row.kit.guid) : false;
              const isDocsRow = row.type === "docs";
              return (
                <div
                  key={row.id}
                  className={`border-b p-2 cursor-selectable ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`}
                  role="button"
                  tabIndex={0}
                  onClick={(e) => {
                    if (isDocsRow && row.docsPath) {
                      navigate(`/${row.docsPath}`);
                    } else if (row.kit) {
                      handleRowClick(row.kit.guid, e);
                    }
                  }}
                  onDoubleClick={() => {
                    if (isDocsRow && row.docsPath) {
                      navigate(`/${row.docsPath}`);
                    } else if (row.kit) {
                      navigateToKit(row.kit.guid);
                    }
                  }}
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
                        {row.type === "temporary" && <Clock className="size-4" />}
                        {row.type === "local" && <HardDrive className="size-4" />}
                        {row.type === "remote" && <Cloud className="size-4" />}
                        {row.type === "docs" && <FileText className="size-4" />}
                      </div>
                      <span className="text-left flex-1 min-w-0 truncate">{row.name}</span>
                    </div>
                    <div className="flex items-center gap-0.5 shrink-0">
                      {row.level === 0 && row.type !== "docs" && (
                        <Action
                          level="base"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCreateVersion(row.name, row.type as KitStoreKind);
                          }}
                          id={tooltip("home.createVersion")}
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
    <>
      <HomeAppFooter />
      <Canvas>
        <Window id="home-table">
          <div
            className="flex flex-col h-full"
            onClick={(e: React.MouseEvent) => {
              if (e.target === e.currentTarget) {
                homeCommands.deselectAll("semio.sketchpad.app.home.canvas.table.deselect");
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
                  onActionClick={() => handleCreateKit(selectedKind)}
                  id={tooltip("home.hideKind")}
                  actionId={tooltip("home.createKit")}
                >
                  {selectedKind === "temporary" && <Clock className="size-4" />}
                  {selectedKind === "local" && <HardDrive className="size-4" />}
                  {selectedKind === "remote" && <Cloud className="size-4" />}
                </Toggle>
              )}
              {selectedName && (
                <Toggle pressed={true} onPressedChange={() => toggleName(selectedName)}>
                  {selectedName}
                </Toggle>
              )}
              {selectedVersion !== null && (
                <Toggle pressed={true} onPressedChange={() => toggleVersion(selectedVersion)}>
                  {selectedVersion || <span className="italic opacity-50">{t("semio.sketchpad.app.kit.defaultVersion")}</span>}
                </Toggle>
              )}
              {!selectedKind && (
                <>
                  <Toggle
                    type="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("temporary")}
                    actionIcon={<Plus className="size-3.5" />}
                    onActionClick={() => handleCreateKit("temporary")}
                    id={tooltip("home.showTemporary")}
                    actionId={tooltip("home.createTemporary")}
                  >
                    <Clock className="size-4" />
                  </Toggle>
                  <Toggle
                    type="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("local")}
                    actionIcon={<Plus className="size-3.5" />}
                    onActionClick={() => handleCreateKit("local")}
                    id={tooltip("home.showLocal")}
                    actionId={tooltip("home.createLocal")}
                  >
                    <HardDrive className="size-4" />
                  </Toggle>
                  <Toggle
                    type="withAction"
                    pressed={false}
                    onPressedChange={() => toggleKind("remote")}
                    actionIcon={<Plus className="size-3.5" />}
                    onActionClick={() => handleCreateKit("remote")}
                    id={tooltip("home.showRemote")}
                    actionId={tooltip("home.createRemote")}
                  >
                    <Cloud className="size-4" />
                  </Toggle>
                </>
              )}
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
                selectedVersion === null &&
                uniqueVersions.length > 0 &&
                uniqueVersions.map((version) => (
                  <Toggle key={version} pressed={false} onPressedChange={() => toggleVersion(version)}>
                    {version || <span className="italic opacity-50">{t("semio.sketchpad.app.kit.defaultVersion")}</span>}
                  </Toggle>
                ))}
              <Input className="flex-1 min-w-[200px]" placeholder={t("semio.sketchpad.app.home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />
            </div>
            {/* Concept Filter */}
            <ConceptFilter allConcepts={allConcepts} />
            <ScrollArea ref={scrollAreaRef} className="flex-1">
              <table className="w-full border-collapse">
                <thead className="sticky top-0 border-b">
                  <tr className="h-9">
                    <th className="text-left p-1 font-medium relative group">
                      <div className="flex items-center justify-between w-full">
                        <span>{t("semio.sketchpad.app.home.name")}</span>
                        <Toggle
                          type="dropdown"
                          pressed={sortColumn === "name"}
                          value={sortColumn === "name" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            homeCommands.setSortColumn("semio.sketchpad.app.home.header.name.sortColumn", "name");
                            homeCommands.setSortDirection("semio.sketchpad.app.home.header.name.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <ArrowUp className="size-3.5" />, id: tooltip("sort.ascending") },
                            { value: "desc", label: <ArrowDown className="size-3.5" />, id: tooltip("sort.descending") },
                          ]}
                          className="px-1 min-w-0"
                        />
                      </div>
                      <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                    </th>
                    {!selectedKind && (
                      <th className="text-left p-1 font-medium relative group">
                        <div className="flex items-center justify-between w-full">
                          <span>{t("semio.sketchpad.app.home.kind")}</span>
                          <Toggle
                            type="dropdown"
                            pressed={sortColumn === "type"}
                            value={sortColumn === "type" ? sortDirection : "asc"}
                            onValueChange={(value) => {
                              homeCommands.setSortColumn("semio.sketchpad.app.home.header.type.sortColumn", "type");
                              homeCommands.setSortDirection("semio.sketchpad.app.home.header.type.sortDirection", value as "asc" | "desc");
                            }}
                            items={[
                              { value: "asc", label: <ArrowUp className="size-3.5" />, id: tooltip("sort.ascending") },
                              { value: "desc", label: <ArrowDown className="size-3.5" />, id: tooltip("sort.descending") },
                            ]}
                            className="px-1 min-w-0"
                          />
                        </div>
                        <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                      </th>
                    )}
                    <th className="text-left p-1 font-medium relative group">
                      <div className="flex items-center justify-between w-full">
                        <span>{t("semio.sketchpad.app.home.lastUpdated")}</span>
                        <Toggle
                          type="dropdown"
                          pressed={sortColumn === "updatedAt"}
                          value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            homeCommands.setSortColumn("semio.sketchpad.app.home.header.updatedAt.sortColumn", "updatedAt");
                            homeCommands.setSortDirection("semio.sketchpad.app.home.header.updatedAt.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <ArrowUp className="size-3.5" />, id: tooltip("sort.ascending") },
                            { value: "desc", label: <ArrowDown className="size-3.5" />, id: tooltip("sort.descending") },
                          ]}
                          className="px-1 min-w-0"
                        />
                      </div>
                      <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-accent" />
                    </th>
                    <th className="text-left p-1 font-medium relative group">
                      <div className="flex items-center justify-between w-full">
                        <span>{t("semio.sketchpad.app.home.created")}</span>
                        <Toggle
                          type="dropdown"
                          pressed={sortColumn === "createdAt"}
                          value={sortColumn === "createdAt" ? sortDirection : "asc"}
                          onValueChange={(value) => {
                            homeCommands.setSortColumn("semio.sketchpad.app.home.header.createdAt.sortColumn", "createdAt");
                            homeCommands.setSortDirection("semio.sketchpad.app.home.header.createdAt.sortDirection", value as "asc" | "desc");
                          }}
                          items={[
                            { value: "asc", label: <ArrowUp className="size-3.5" />, id: tooltip("sort.ascending") },
                            { value: "desc", label: <ArrowDown className="size-3.5" />, id: tooltip("sort.descending") },
                          ]}
                          className="px-1 min-w-0"
                        />
                      </div>
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {rows.map((row) => {
                    const isSelected = row.kit ? selection.includes(row.kit.guid) : false;
                    const isDocsRow = row.type === "docs";
                    return (
                      <tr
                        key={row.id}
                        className={`border-b cursor-selectable ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`}
                        onClick={(e) => {
                          if (isDocsRow && row.docsPath) {
                            navigate(`/${row.docsPath}`);
                          } else if (row.kit) {
                            handleRowClick(row.kit.guid, e);
                          }
                        }}
                        onDoubleClick={() => {
                          if (isDocsRow && row.docsPath) {
                            navigate(`/${row.docsPath}`);
                          } else if (row.kit) {
                            navigateToKit(row.kit.guid);
                          }
                        }}
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
                              <span className="text-left flex-1 min-w-0 truncate">{row.name}</span>
                            </div>
                            <div className="flex items-center gap-0.5 shrink-0">
                              {row.level === 0 && row.type !== "docs" && (
                                <Action
                                  level="base"
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    handleCreateVersion(row.name, row.type as KitStoreKind);
                                  }}
                                  id={tooltip("home.createVersion")}
                                >
                                  <Plus />
                                </Action>
                              )}
                            </div>
                          </div>
                        </td>
                        {!selectedKind && (
                          <td className="p-1">
                            {row.type === "temporary" && <Clock className="size-4" />}
                            {row.type === "local" && <HardDrive className="size-4" />}
                            {row.type === "remote" && <Cloud className="size-4" />}
                            {row.type === "docs" && <FileText className="size-4" />}
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
        </Window>
      </Canvas>
    </>
  );
};

export default Home;
