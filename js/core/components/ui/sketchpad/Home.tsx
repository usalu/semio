import { Plus } from "lucide-react";
import { FC, useMemo, useState } from "react";
import { guid } from "../../../lib/utils";
import { Kit, KitShallow } from "../../../semio";
import { useKits, useSketchpadCommands, useSketchpadStore } from "../../../store";
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
  const kits = useKits();
  const store = useSketchpadStore();
  const { createKit, navigateToKit } = useSketchpadCommands();
  const [selectedKinds, setSelectedKinds] = useState<KitStoreKind[]>(["temporary", "local", "remote"]);
  const [searchQuery, setSearchQuery] = useState("");
  const [expandedRows, setExpandedRows] = useState<Set<string>>(new Set());

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const formatDate = (date?: Date) => (date instanceof Date ? date.toLocaleDateString() : date ? new Date(date).toLocaleDateString() : "");
    const kitGroups = new Map<string, KitShallow[]>();

    kits.forEach((kit) => {
      const kitStore = store.kit(kit.guid);
      let type: KitStoreKind = "temporary";
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) type = "remote";
      else if (kitStore.isLocallyPersisted) type = "local";

      if (!selectedKinds.includes(type)) return;
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
  }, [kits, store, selectedKinds, searchQuery, expandedRows]);

  const handleCreateKit = (type: KitStoreKind) => {
    const newKit: Kit = {
      guid: guid(),
      name: "New Kit",
      version: "1.0.0",
      types: [],
      designs: [],
    };
    const local = type === "local" || type === "remote";
    const remote = type === "remote";
    createKit(newKit, local, remote);
    navigateToKit(newKit.guid);
  };

  const toggleKind = (type: KitStoreKind) => {
    setSelectedKinds((prev) => (prev.includes(type) ? prev.filter((t) => t !== type) : [...prev, type]));
  };

  const toggleRow = (rowId: string) => {
    setExpandedRows((prev) => {
      const next = new Set(prev);
      if (next.has(rowId)) next.delete(rowId);
      else next.add(rowId);
      return next;
    });
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col lg:flex-row lg:items-center gap-2 p-4 border-b">
        <div className="flex flex-wrap gap-2">
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("temporary")}
            onPressedChange={() => toggleKind("temporary")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("temporary")}
            tooltip="Filter temporary kits"
            actionTooltip="Create temporary kit"
          >
            Temporary
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("local")}
            onPressedChange={() => toggleKind("local")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("local")}
            tooltip="Filter local kits"
            actionTooltip="Create local kit"
          >
            Local
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("remote")}
            onPressedChange={() => toggleKind("remote")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("remote")}
            tooltip="Filter remote kits"
            actionTooltip="Create remote kit"
          >
            Remote
          </Toggle>
        </div>
        <Input className="lg:flex-1 lg:min-w-[200px]" placeholder="Search kits..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 bg-background border-b">
            <tr>
              <th className="text-left p-2 font-medium">Name</th>
              <th className="text-left p-2 font-medium">Kind</th>
              <th className="text-left p-2 font-medium">Last updated</th>
              <th className="text-left p-2 font-medium">Created</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.id} className="border-b hover:bg-muted/50">
                <td className="p-2">
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
                    <span className="cursor-pointer" onClick={() => navigateToKit(row.kit.guid)}>
                      {row.name}
                    </span>
                  </div>
                </td>
                <td className="p-2 capitalize">{row.type}</td>
                <td className="p-2">{row.updatedAt}</td>
                <td className="p-2">{row.createdAt}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </ScrollArea>
    </div>
  );
};

export default Home;
