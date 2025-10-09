import { Plus } from "lucide-react";
import { FC, useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { guid } from "../../../lib/utils";
import { Author, Design, Kit, Quality, File as SemioFile, Type } from "../../../semio";
import { EditorType, useEditorType, useIsInKitScope, useKit, useKitCommands, useKitEditorCommands, useKitStore, useSketchpadCommands } from "../../../store";
import { Input } from "../Input";
import { ScrollArea } from "../ScrollArea";
import { Textarea } from "../Textarea";
import { Toggle } from "../Toggle";
import { TreeItem, TreeSection } from "../Tree";
import { useAddPanelSection, useRemovePanelSection } from "./Navbar";

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

const KitDetails: FC = () => {
  const isInKitScope = useIsInKitScope();

  if (!isInKitScope) {
    return null;
  }

  return <KitDetailsForm />;
};

const KitDetailsForm: FC = () => {
  const { t } = useTranslation();

  try {
    const kit = useKit() as Kit;

    if (!kit) {
      return (
        <TreeSection label={t("kit.title")} defaultOpen={true}>
          <TreeItem>
            <p className="text-sm text-muted-foreground">Kit not available</p>
          </TreeItem>
        </TreeSection>
      );
    }

    const kitStore = useKitStore() as any;
    const { startTransaction, finalizeTransaction, abortTransaction } = useKitEditorCommands();

    return (
      <TreeSection label={t("kit.title")} defaultOpen={true}>
        <TreeItem>
          <Input lazy label={t("kit.name")} value={kit.name} onLazyChange={(value) => kitStore.change({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeItem>
        <TreeItem>
          <Input
            lazy
            label={t("kit.version")}
            value={kit.version || ""}
            placeholder={t("kit.versionPlaceholder")}
            onLazyChange={(value) => kitStore.change({ version: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Textarea
            lazy
            label={t("kit.description")}
            value={kit.description || ""}
            placeholder={t("kit.descriptionPlaceholder")}
            onLazyChange={(value) => kitStore.change({ description: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            lazy
            label={t("kit.icon")}
            value={kit.icon || ""}
            placeholder={t("kit.iconPlaceholder")}
            onLazyChange={(value) => kitStore.change({ icon: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            lazy
            label={t("kit.image")}
            value={kit.image || ""}
            placeholder={t("kit.imagePlaceholder")}
            onLazyChange={(value) => kitStore.change({ image: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            lazy
            label={t("kit.homepage")}
            value={kit.homepage || ""}
            placeholder={t("kit.homepagePlaceholder")}
            onLazyChange={(value) => kitStore.change({ homepage: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            lazy
            label={t("kit.license")}
            value={kit.license || ""}
            placeholder={t("kit.licensePlaceholder")}
            onLazyChange={(value) => kitStore.change({ license: value })}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
          />
        </TreeItem>
      </TreeSection>
    );
  } catch (error) {
    console.error("Error rendering kit details:", error);
    return (
      <TreeSection label={t("kit.title")} defaultOpen={true}>
        <TreeItem>
          <p className="text-sm text-muted-foreground">Kit not found or not loaded</p>
        </TreeItem>
      </TreeSection>
    );
  }
};

const KitEditor: FC = () => {
  const { t } = useTranslation();

  let kit: Kit | null = null;
  try {
    kit = useKit() as Kit;
  } catch (error) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">No kit loaded</p>
      </div>
    );
  }

  if (!kit) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">No kit loaded</p>
      </div>
    );
  }

  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const [selectedKinds, setSelectedKinds] = useState<ArtifactKind[]>(["designs", "types", "qualities", "files", "authors"]);
  const [selectedConcepts, setSelectedConcepts] = useState<string[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [expandedRows, setExpandedRows] = useState<Set<string>>(new Set());

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();

  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kit.designs?.forEach((d: Design) => d.concepts?.forEach((c: string) => conceptSet.add(c)));
    return Array.from(conceptSet).sort();
  }, [kit.designs]);

  useEffect(() => {
    if (editorType !== EditorType.KIT) {
      return;
    }

    addSection("details", {
      id: "kit",
      label: "Kit",
      order: 0,
      defaultOpen: true,
      content: () => <KitDetails />,
    });

    return () => {
      removeSection("details", "kit");
    };
  }, [addSection, removeSection, editorType]);

  const rows = useMemo<TableRow[]>(() => {
    const result: TableRow[] = [];
    const formatDate = (date?: Date) => (date instanceof Date ? date.toLocaleDateString() : date ? new Date(date).toLocaleDateString() : "");

    if (selectedKinds.includes("designs")) {
      const designGroups = new Map<string, Design[]>();
      kit.designs?.forEach((design: Design) => {
        const key = design.name;
        if (!designGroups.has(key)) designGroups.set(key, []);
        designGroups.get(key)!.push(design);
      });

      designGroups.forEach((designs, name) => {
        const filteredDesigns = designs.filter((d) => {
          if (selectedConcepts.length > 0 && !d.concepts?.some((c) => selectedConcepts.includes(c))) return false;
          if (searchQuery && !name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
          return true;
        });

        if (filteredDesigns.length === 0) return;

        const parentId = `design-${name}`;
        const hasChildren = filteredDesigns.some((d) => d.variant || d.view);

        result.push({
          id: parentId,
          kind: "designs",
          artifact: name,
          authors: "",
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren,
          isExpanded: expandedRows.has(parentId),
          data: filteredDesigns[0],
        });

        if (expandedRows.has(parentId)) {
          const variantGroups = new Map<string, Design[]>();
          filteredDesigns.forEach((design) => {
            const variantKey = design.variant || "";
            if (!variantGroups.has(variantKey)) variantGroups.set(variantKey, []);
            variantGroups.get(variantKey)!.push(design);
          });

          variantGroups.forEach((variantDesigns, variant) => {
            const variantId = `${parentId}-${variant}`;
            const hasViewChildren = variantDesigns.some((d) => d.view);

            result.push({
              id: variantId,
              kind: "designs",
              artifact: variant || "(default)",
              authors: "",
              updatedAt: "",
              createdAt: "",
              level: 1,
              parentId,
              hasChildren: hasViewChildren,
              isExpanded: expandedRows.has(variantId),
              data: variantDesigns[0],
            });

            if (expandedRows.has(variantId) && hasViewChildren) {
              variantDesigns.forEach((design) => {
                if (design.view) {
                  const viewId = `${variantId}-${design.view}`;
                  result.push({
                    id: viewId,
                    kind: "designs",
                    artifact: design.view,
                    authors: design.authors?.join(", ") || "",
                    updatedAt: formatDate(design.updatedAt),
                    createdAt: formatDate(design.createdAt),
                    level: 2,
                    parentId: variantId,
                    hasChildren: false,
                    isExpanded: false,
                    data: design,
                  });
                }
              });
            }
          });
        }
      });
    }

    if (selectedKinds.includes("types")) {
      const typeGroups = new Map<string, Type[]>();
      kit.types?.forEach((type: Type) => {
        const key = type.name;
        if (!typeGroups.has(key)) typeGroups.set(key, []);
        typeGroups.get(key)!.push(type);
      });

      typeGroups.forEach((types, name) => {
        const filteredTypes = types.filter((t) => {
          if (searchQuery && !name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
          return true;
        });

        if (filteredTypes.length === 0) return;

        const parentId = `type-${name}`;
        const hasChildren = filteredTypes.length > 1 || filteredTypes.some((t) => t.variant);

        result.push({
          id: parentId,
          kind: "types",
          artifact: name,
          authors: "",
          updatedAt: "",
          createdAt: "",
          level: 0,
          hasChildren,
          isExpanded: expandedRows.has(parentId),
          data: filteredTypes[0],
        });

        if (expandedRows.has(parentId) && hasChildren) {
          filteredTypes.forEach((type) => {
            const variantId = `${parentId}-${type.variant || "default"}`;
            result.push({
              id: variantId,
              kind: "types",
              artifact: type.variant || "(default)",
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

    if (selectedKinds.includes("authors")) {
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

    return result;
  }, [kit, selectedKinds, selectedConcepts, searchQuery, expandedRows]);

  const toggleRow = (rowId: string) => {
    setExpandedRows((prev) => {
      const next = new Set(prev);
      if (next.has(rowId)) next.delete(rowId);
      else next.add(rowId);
      return next;
    });
  };

  const handleCreateArtifact = (kind: ArtifactKind) => {
    switch (kind) {
      case "designs": {
        const newDesign: Design = {
          guid: guid(),
          name: "New Design",
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
        const newType: Type = {
          guid: guid(),
          name: "New Type",
          variant: "",
          ports: [],
        };
        kitCommands.createType(newType);
        sketchpadCommands.navigateToType(kit.guid, newType.guid);
        break;
      }
      case "qualities": {
        // TODO: Implement quality creation
        console.log("Create new quality");
        break;
      }
      case "files": {
        // TODO: Implement file creation
        console.log("Create new file");
        break;
      }
      case "authors": {
        // TODO: Implement author creation
        console.log("Create new author");
        break;
      }
    }
  };

  const toggleKind = (kind: ArtifactKind) => {
    setSelectedKinds((prev) => (prev.includes(kind) ? prev.filter((k) => k !== kind) : [...prev, kind]));
  };

  const toggleConcept = (concept: string) => {
    setSelectedConcepts((prev) => (prev.includes(concept) ? prev.filter((c) => c !== concept) : [...prev, concept]));
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col gap-2 p-4 border-b">
        <div className="flex flex-wrap gap-2">
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("designs")}
            onPressedChange={() => toggleKind("designs")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateArtifact("designs")}
            tooltip="Filter designs"
            actionTooltip="Create new design"
          >
            Designs
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("types")}
            onPressedChange={() => toggleKind("types")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateArtifact("types")}
            tooltip="Filter types"
            actionTooltip="Create new type"
          >
            Types
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("qualities")}
            onPressedChange={() => toggleKind("qualities")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateArtifact("qualities")}
            tooltip="Filter qualities"
            actionTooltip="Create new quality"
          >
            Qualities
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("files")}
            onPressedChange={() => toggleKind("files")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateArtifact("files")}
            tooltip="Filter files"
            actionTooltip="Create new file"
          >
            Files
          </Toggle>
          <Toggle
            type="withAction"
            pressed={selectedKinds.includes("authors")}
            onPressedChange={() => toggleKind("authors")}
            actionIcon={<Plus className="size-3.5 opacity-50" />}
            onActionClick={() => handleCreateArtifact("authors")}
            tooltip="Filter authors"
            actionTooltip="Create new author"
          >
            Authors
          </Toggle>
        </div>
        {allConcepts.length > 0 && (
          <div className="flex flex-wrap gap-2">
            {allConcepts.map((concept) => (
              <Toggle key={concept} pressed={selectedConcepts.includes(concept)} onPressedChange={() => toggleConcept(concept)}>
                {concept}
              </Toggle>
            ))}
          </div>
        )}
        <Input placeholder={t("common.search")} value={searchQuery} onChange={(e) => setSearchQuery(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 bg-background border-b">
            <tr>
              <th className="text-left p-2 font-medium">Name</th>
              <th className="text-left p-2 font-medium">Authors</th>
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
                      <button onClick={() => toggleRow(row.id)} className="w-4 h-4 flex items-center justify-center hover:bg-muted">
                        {row.isExpanded ? <ChevronDown className="w-3 h-3" /> : <ChevronRight className="w-3 h-3" />}
                      </button>
                    ) : (
                      <span className="w-4 h-4" />
                    )}
                    <span>{row.artifact}</span>
                  </div>
                </td>
                <td className="p-2">{row.authors}</td>
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

export default KitEditor;
