import { Plus } from "lucide-react";
import { FC, useMemo, useState } from "react";
import { guid } from "../../../lib/utils";
import { Kit, KitShallow } from "../../../semio";
import { useKits, useSketchpadCommands, useSketchpadStore } from "../../../store";
import { Input } from "../Input";
import { ScrollArea } from "../ScrollArea";
import { Toggle } from "../Toggle";

type KitStoreType = "temporary" | "local" | "remote";

type TableRow = {
  id: string;
  name: string;
  version: string;
  type: KitStoreType;
  updatedAt: string;
  createdAt: string;
  kit: KitShallow;
};

const ChevronRight: FC<{ className?: string }> = ({ className }) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={className}>
    <path d="m9 18 6-6-6-6" />
  </svg>
);

const Home: FC = ({}) => {
  const kits = useKits();
  const store = useSketchpadStore();
  const { createKit, navigateToKit } = useSketchpadCommands();
  const [selectedTypes, setSelectedTypes] = useState<KitStoreType[]>(["temporary", "local", "remote"]);
  const [searchQuery, setSearchQuery] = useState("");

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const formatDate = (date?: Date) => (date instanceof Date ? date.toLocaleDateString() : date ? new Date(date).toLocaleDateString() : "");

    kits.forEach((kit) => {
      const kitStore = store.kit(kit.guid);
      let type: KitStoreType = "temporary";
      if (kitStore.isLocallyPersisted && kitStore.isRemotelySynced) type = "remote";
      else if (kitStore.isLocallyPersisted) type = "local";

      if (!selectedTypes.includes(type)) return;
      if (searchQuery && !kit.name.toLowerCase().includes(searchQuery.toLowerCase())) return;

      result.push({
        id: kit.guid,
        name: kit.name,
        version: kit.version || "",
        type,
        updatedAt: formatDate(kit.updatedAt),
        createdAt: formatDate(kit.createdAt),
        kit: kit,
      });
    });

    return result;
  }, [kits, store, selectedTypes, searchQuery]);

  const handleCreateKit = (type: KitStoreType) => {
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

  const toggleType = (type: KitStoreType) => {
    setSelectedTypes((prev) => (prev.includes(type) ? prev.filter((t) => t !== type) : [...prev, type]));
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col gap-2 p-4 border-b">
        <div className="flex flex-wrap gap-2">
          <Toggle
            type="withAction"
            pressed={selectedTypes.includes("temporary")}
            onPressedChange={() => toggleType("temporary")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("temporary")}
            tooltip="Filter temporary kits"
            actionTooltip="Create temporary kit"
          >
            Temporary
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedTypes.includes("local")}
            onPressedChange={() => toggleType("local")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("local")}
            tooltip="Filter local kits"
            actionTooltip="Create local kit"
          >
            Local
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedTypes.includes("remote")}
            onPressedChange={() => toggleType("remote")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateKit("remote")}
            tooltip="Filter remote kits"
            actionTooltip="Create remote kit"
          >
            Remote
          </Toggle>
        </div>
        <Input placeholder="Search kits..." value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 bg-background border-b">
            <tr>
              <th className="text-left p-2 font-medium">Name</th>
              <th className="text-left p-2 font-medium">Version</th>
              <th className="text-left p-2 font-medium">Type</th>
              <th className="text-left p-2 font-medium">Last updated</th>
              <th className="text-left p-2 font-medium">Created</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.id} className="border-b hover:bg-muted/50 cursor-pointer" onClick={() => navigateToKit(row.id)}>
                <td className="p-2">{row.name}</td>
                <td className="p-2">{row.version}</td>
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
