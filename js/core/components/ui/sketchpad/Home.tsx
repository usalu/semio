import { formatDistanceToNow } from "date-fns";
import { de, enUS } from "date-fns/locale";
import { Clock, Cloud, HardDrive, Plus } from "lucide-react";
import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router";
import i18n from "../../../i18n";
import { generateUniqueName, guid } from "../../../lib/utils";
import { Kit, KitShallow } from "../../../semio";
import { useKits, useNavigation, useSketchpadCommands, useSketchpadStore, useTooltip } from "../../../store";
import { Input } from "../Input";
import { ScrollArea } from "../ScrollArea";
import { Toggle } from "../Toggle";

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

  // Get kind from search params instead of path
  const kindParam = searchParams.get("k");
  const selectedKind = ["temporary", "local", "remote"].includes(kindParam || "") ? (kindParam as KitStoreKind) : undefined;

  // Get search query from URL
  const searchQuery = searchParams.get("q") || "";

  // Get expanded rows from search params
  const expandedRowsParam = searchParams.getAll("e");
  const expandedRows = new Set(expandedRowsParam);

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

    return result;
  }, [kits, store, selectedKind, searchQuery, expandedRows]);

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

  const toggleKind = (type: KitStoreKind) => {
    const newParams = new URLSearchParams(searchParams);
    if (selectedKind === type) {
      // If already selected, remove the filter
      newParams.delete("k");
    } else {
      // Set the kind filter
      newParams.set("k", type);
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

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col lg:flex-row lg:items-center gap-1 p-1 border-b">
        <div className="flex flex-wrap gap-1 lg:flex-shrink-0">
          <Toggle
            type="withAction"
            pressed={selectedKind === "temporary"}
            onPressedChange={() => toggleKind("temporary")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("temporary")}
            tooltip={selectedKind === "temporary" ? tooltip("home.hideTemporary") : tooltip("home.showTemporary")}
            actionTooltip={tooltip("home.createTemporary")}
          >
            <Clock className="size-4" />
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKind === "local"}
            onPressedChange={() => toggleKind("local")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("local")}
            tooltip={selectedKind === "local" ? tooltip("home.hideLocal") : tooltip("home.showLocal")}
            actionTooltip={tooltip("home.createLocal")}
          >
            <HardDrive className="size-4" />
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKind === "remote"}
            onPressedChange={() => toggleKind("remote")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("remote")}
            tooltip={selectedKind === "remote" ? tooltip("home.hideRemote") : tooltip("home.showRemote")}
            actionTooltip={tooltip("home.createRemote")}
          >
            <Cloud className="size-4" />
          </Toggle>
        </div>
        <Input className="w-full lg:w-auto lg:flex-1 lg:min-w-[200px]" placeholder={t("home.searchPlaceholder")} value={searchQuery} onChange={(e) => handleSearchChange(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 border-b">
            <tr className="h-9">
              <th className="text-left p-1 font-medium">{t("home.name")}</th>
              {!selectedKind && <th className="text-left p-1 font-medium">{t("home.kind")}</th>}
              <th className="text-left p-1 font-medium">{t("home.lastUpdated")}</th>
              <th className="text-left p-1 font-medium">{t("home.created")}</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.id} className="border-b hover:bg-muted/50">
                <td className="p-1">
                  <div className="flex items-center gap-1" style={{ paddingLeft: `${row.level * 24}px` }}>
                    {row.hasChildren ? (
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          toggleRow(row.id);
                        }}
                        className="w-4 h-4 flex items-center justify-center hover:bg-muted"
                      >
                        {row.isExpanded ? <ChevronDown className="w-3 h-3" /> : <ChevronRight className="w-3 h-3" />}
                      </button>
                    ) : (
                      <span className="w-4 h-4" />
                    )}
                    <button className="cursor-pointer hover:underline text-left" onClick={() => navigateToKit(row.kit.guid)}>
                      {row.name}
                    </button>
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
            ))}
          </tbody>
        </table>
      </ScrollArea>
    </div>
  );
};

export default Home;
