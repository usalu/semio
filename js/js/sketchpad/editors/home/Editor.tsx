import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import { ArrowDown, ArrowUp, Clock, Cloud, HardDrive, Plus } from "lucide-react";
import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router";
import { ScrollArea } from "../../../elements/aggregation/ScrollArea";
import { Action } from "../../../elements/input/Action";
import { Input } from "../../../elements/input/Input";
import { Toggle } from "../../../elements/input/Toggle";
import i18n from "../../../i18n";
import { generateUniqueName, guid, Kit, KitShallow } from "../../../semio";
import { useIsMobile, useKits, useNavigation, useSketchpadCommands, useSketchpadStore, useTooltip } from "../../store";
import { useHome, useHomeCommands } from "./store";

type KitStoreKind = "temporary" | "local" | "remote";

type TableRow = {
  id: string;
  name: string;
  level: number;
  parentId?: string;
  hasChildren: boolean;
  isExpanded: boolean;
  type: KitStoreKind;
  updatedAt: string;
  createdAt: string;
  kit: KitShallow;
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
  const store = useSketchpadStore();
  const { createKit, navigateToKit } = useSketchpadCommands();
  const tooltip = useTooltip();
  const homeState = useHome() as any;
  const homeCommands = useHomeCommands();
  const isMobile = useIsMobile();

  // Get filters from search params (?kind=&name=&version=)
  const selectedKind = searchParams.get("kind") as KitStoreKind | null;
  const selectedName = searchParams.get("name");
  const selectedVersion = searchParams.get("version");

  // Get search query from URL search params
  const searchQuery = searchParams.get("q") || "";

  // Get expanded rows from search params
  const expandedRowsParam = searchParams.getAll("e");
  const expandedRows = new Set(expandedRowsParam);

  const selection = homeState?.selection?.kits || [];
  const sortColumn = homeState?.sortColumn;
  const sortDirection = homeState?.sortDirection || "asc";

  // Collect unique names
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();
    kits.forEach((kit) => {
      const kitStore = store.kit(kit.guid);
      let type: KitStoreKind = "temporary";
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) type = "remote";
      else if (kitStore.isLocallyPersisted) type = "local";

      if (selectedKind && selectedKind !== type) return;
      nameSet.add(kit.name);
    });
    return Array.from(nameSet).sort();
  }, [kits, store, selectedKind]);

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

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const locale = i18n.language === "de" ? de : enUS;
    const formatDate = (date?: Date) => {
      if (!date) return "";
      const parsedDate = date instanceof Date ? date : new Date(date);
      if (isNaN(parsedDate.getTime())) return "";
      return formatDistanceToNow(parsedDate, { addSuffix: true, locale });
    };
    const kitGroups = new Map<string, KitShallow[]>();

    kits.forEach((kit) => {
      const kitStore = store.kit(kit.guid);
      let type: KitStoreKind = "temporary";
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) type = "remote";
      else if (kitStore.isLocallyPersisted) type = "local";

      if (selectedKind && selectedKind !== type) return;
      if (searchQuery && !kit.name.toLowerCase().includes(searchQuery.toLowerCase())) return;
      if (selectedName && kit.name !== selectedName) return;
      if (selectedVersion && (kit.version || "") !== selectedVersion) return;

      const key = kit.name;
      if (!kitGroups.has(key)) kitGroups.set(key, []);
      kitGroups.get(key)!.push(kit);
    });

    kitGroups.forEach((groupKits, name) => {
      const parentId = `kit-${name}`;
      const hasChildren = groupKits.length > 1 || groupKits.some((k) => k.version);

      const kitStore = store.kit(groupKits[0].guid);
      let type: KitStoreKind = "temporary";
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) type = "remote";
      else if (kitStore.isLocallyPersisted) type = "local";

      result.push({
        id: parentId,
        name: name,
        level: 0,
        hasChildren,
        isExpanded: expandedRows.has(parentId),
        type,
        updatedAt: "",
        createdAt: "",
        kit: groupKits[0],
      });

      if (expandedRows.has(parentId) && hasChildren) {
        groupKits.forEach((kit) => {
          const kitStore = store.kit(kit.guid);
          let kitKind: KitStoreKind = "temporary";
          if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) kitKind = "remote";
          else if (kitStore.isLocallyPersisted) kitKind = "local";

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
  }, [kits, store, selectedKind, searchQuery, selectedName, selectedVersion, expandedRows, sortColumn, sortDirection]);

  const handleCreateKit = (type: KitStoreKind) => {
    const existingNames = kits.map((k) => k.name);
    const uniqueName = generateUniqueName(t("kit.defaultName"), existingNames);
    const newKit: Kit = {
      guid: guid(),
      name: uniqueName,
      version: "",
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit(newKit, local, remote);
    navigateToKit(newKit.guid);
  };

  const handleCreateVersion = (kitName: string, type: KitStoreKind) => {
    const existingVersions = kits.filter((k) => k.name === kitName).map((k) => k.version || "");
    const uniqueVersion = generateUniqueName("", existingVersions);
    const newKit: Kit = {
      guid: guid(),
      name: kitName,
      version: uniqueVersion,
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit(newKit, local, remote);
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
          homeCommands.selectKits(rangeIds);
        }
      } else {
        homeCommands.selectKit(kitId);
      }
    } else if (e.metaKey || e.ctrlKey) {
      if (selection.includes(kitId)) {
        homeCommands.removeKitFromSelection(kitId);
      } else {
        homeCommands.addKitToSelection(kitId);
      }
    } else {
      homeCommands.selectKit(kitId);
    }
  };

  const handleSortClick = (column: "name" | "type" | "updatedAt" | "createdAt") => {
    homeCommands.toggleSort(column);
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
              onActionClick={() => handleCreateKit(selectedKind)}
              tooltip={tooltip("home.hideKind")}
              actionTooltip={tooltip("home.createKit")}
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
              {selectedVersion || <span className="italic opacity-50">{t("kit.defaultVersion")}</span>}
            </Toggle>
          )}
          {!selectedKind && (
            <>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("temporary")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateKit("temporary")}
                tooltip={tooltip("home.showTemporary")}
                actionTooltip={tooltip("home.createTemporary")}
              >
                <Clock className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("local")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateKit("local")}
                tooltip={tooltip("home.showLocal")}
                actionTooltip={tooltip("home.createLocal")}
              >
                <HardDrive className="size-4" />
              </Toggle>
              <Toggle
                type="withAction"
                pressed={false}
                onPressedChange={() => toggleKind("remote")}
                actionIcon={<Plus className="size-3.5 opacity-50" />}
                onActionClick={() => handleCreateKit("remote")}
                tooltip={tooltip("home.showRemote")}
                actionTooltip={tooltip("home.createRemote")}
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
                {version || <span className="italic opacity-50">{t("kit.defaultVersion")}</span>}
              </Toggle>
            ))}
          <div className="flex items-center gap-1 flex-1 min-w-[160px]">
            <Input className="flex-1 min-w-0" placeholder={t("home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />
            <Toggle
              type="dropdown"
              value={sortColumn === "name" ? sortDirection : "asc"}
              onValueChange={(value) => {
                homeCommands.setSortColumn("name");
                homeCommands.setSortDirection(value as "asc" | "desc");
              }}
              items={[
                { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
              ]}
              tooltip={t("home.sortByName")}
            />
          </div>
        </div>

        {/* Simplified table - only name column, no headers */}
        <ScrollArea className="flex-1">
          <div className="flex flex-col">
            {rows.map((row) => {
              const isSelected = selection.includes(row.kit.guid);
              return (
                <div key={row.id} className={`border-b p-2 cursor-pointer ${isSelected ? "bg-primary text-primary-foreground" : "hover:bg-hover-base"}`} onClick={(e) => handleRowClick(row.kit.guid, e)}>
                  <div className="flex items-center gap-2 justify-between" style={{ paddingLeft: `${row.level * 16}px` }} onClick={(e) => e.stopPropagation()}>
                    <div className="flex items-center gap-2 flex-1 min-w-0">
                      {row.hasChildren ? (
                        <button onClick={() => toggleRow(row.id)} className="w-5 h-5 flex items-center justify-center hover:bg-muted shrink-0">
                          {row.isExpanded ? <ChevronDown className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
                        </button>
                      ) : (
                        <span className="w-5 h-5 shrink-0" />
                      )}
                      <div className="shrink-0">
                        {row.type === "temporary" && <Clock className="size-4" />}
                        {row.type === "local" && <HardDrive className="size-4" />}
                        {row.type === "remote" && <Cloud className="size-4" />}
                      </div>
                      <span
                        className="cursor-pointer hover:underline text-left flex-1 min-w-0 truncate"
                        onClick={(e) => {
                          e.stopPropagation();
                          navigateToKit(row.kit.guid);
                        }}
                      >
                        {row.name}
                      </span>
                    </div>
                    <div className="flex items-center gap-0.5 shrink-0">
                      {row.level === 0 && (
                        <Action
                          level="base"
                          onClick={(e) => {
                            e.stopPropagation();
                            handleCreateVersion(row.name, row.type);
                          }}
                          tooltip={t("home.createVersion")}
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
            onActionClick={() => handleCreateKit(selectedKind)}
            tooltip={tooltip("home.hideKind")}
            actionTooltip={tooltip("home.createKit")}
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
            {selectedVersion || <span className="italic opacity-50">{t("kit.defaultVersion")}</span>}
          </Toggle>
        )}
        {!selectedKind && (
          <>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("temporary")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateKit("temporary")}
              tooltip={tooltip("home.showTemporary")}
              actionTooltip={tooltip("home.createTemporary")}
            >
              <Clock className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("local")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateKit("local")}
              tooltip={tooltip("home.showLocal")}
              actionTooltip={tooltip("home.createLocal")}
            >
              <HardDrive className="size-4" />
            </Toggle>
            <Toggle
              type="withAction"
              pressed={false}
              onPressedChange={() => toggleKind("remote")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateKit("remote")}
              tooltip={tooltip("home.showRemote")}
              actionTooltip={tooltip("home.createRemote")}
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
              {version || <span className="italic opacity-50">{t("kit.defaultVersion")}</span>}
            </Toggle>
          ))}
        <Input className="flex-1 min-w-[200px]" placeholder={t("home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 border-b">
            <tr className="h-9">
              <th className="text-left p-1 font-medium relative group">
                <div className="flex items-center justify-between w-full">
                  <span>{t("home.name")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "name"}
                    value={sortColumn === "name" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      homeCommands.setSortColumn("name");
                      homeCommands.setSortDirection(value as "asc" | "desc");
                    }}
                    items={[
                      { value: "asc", label: <ArrowUp className="size-3.5" />, tooltip: t("sort.ascending") },
                      { value: "desc", label: <ArrowDown className="size-3.5" />, tooltip: t("sort.descending") },
                    ]}
                    className="px-1 min-w-0"
                  />
                </div>
                <div className="absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-primary" />
              </th>
              {!selectedKind && (
                <th className="text-left p-1 font-medium relative group">
                  <div className="flex items-center justify-between w-full">
                    <span>{t("home.kind")}</span>
                    <Toggle
                      type="dropdown"
                      pressed={sortColumn === "type"}
                      value={sortColumn === "type" ? sortDirection : "asc"}
                      onValueChange={(value) => {
                        homeCommands.setSortColumn("type");
                        homeCommands.setSortDirection(value as "asc" | "desc");
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
                  <span>{t("home.lastUpdated")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "updatedAt"}
                    value={sortColumn === "updatedAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      homeCommands.setSortColumn("updatedAt");
                      homeCommands.setSortDirection(value as "asc" | "desc");
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
                  <span>{t("home.created")}</span>
                  <Toggle
                    type="dropdown"
                    pressed={sortColumn === "createdAt"}
                    value={sortColumn === "createdAt" ? sortDirection : "asc"}
                    onValueChange={(value) => {
                      homeCommands.setSortColumn("createdAt");
                      homeCommands.setSortDirection(value as "asc" | "desc");
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
              const isSelected = selection.includes(row.kit.guid);
              return (
                <tr key={row.id} className={`border-b ${isSelected ? "bg-active-base text-active-foreground" : "hover:bg-hover-base"}`} onClick={(e) => handleRowClick(row.kit.guid, e)} role="button" tabIndex={0}>
                  <td className="p-1" onClick={(e) => e.stopPropagation()}>
                    <div className="flex items-center gap-1 justify-between" style={{ paddingLeft: `${row.level * 24}px` }}>
                      <div className="flex items-center gap-1 flex-1 min-w-0">
                        {row.hasChildren ? (
                          <button onClick={() => toggleRow(row.id)} className="w-4 h-4 flex items-center justify-center hover:bg-muted shrink-0">
                            {row.isExpanded ? <ChevronDown className="w-3 h-3" /> : <ChevronRight className="w-3 h-3" />}
                          </button>
                        ) : (
                          <span className="w-4 h-4 shrink-0" />
                        )}
                        <span
                          className="hover:underline text-left flex-1 min-w-0 truncate"
                          onClick={(e) => {
                            e.stopPropagation();
                            navigateToKit(row.kit.guid);
                          }}
                          role="link"
                          tabIndex={0}
                        >
                          {row.name}
                        </span>
                      </div>
                      <div className="flex items-center gap-0.5 shrink-0">
                        {row.level === 0 && (
                          <Action
                            level="base"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleCreateVersion(row.name, row.type);
                            }}
                            tooltip={t("home.createVersion")}
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

export default Home;
