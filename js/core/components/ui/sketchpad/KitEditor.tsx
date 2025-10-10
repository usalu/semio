import { Award, Box, FileText, Layout, Plus, User } from "lucide-react";
import { FC, useEffect, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearchParams } from "react-router";
import { generateUniqueName, guid } from "../../../lib/utils";
import { Author, Design, Kit, Quality, File as SemioFile, Type } from "../../../semio";
import { EditorType, useEditorType, useIsInKitScope, useKit, useKitCommands, useKitEditor, useKitEditorCommands, useKitStore, useNavigation, useSketchpadCommands } from "../../../store";
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
            <p className="text-sm text-muted-foreground">{t("kit.notAvailable")}</p>
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
          <p className="text-sm text-muted-foreground">{t("kit.notFound")}</p>
        </TreeItem>
      </TreeSection>
    );
  }
};

const KitEditor: FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const navigation = useNavigation();
  const [searchParams, setSearchParams] = useSearchParams();

  let kit: Kit | null = null;
  try {
    kit = useKit() as Kit;
  } catch (error) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">{t("kit.noKitLoaded")}</p>
      </div>
    );
  }

  if (!kit) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-sm text-muted-foreground">{t("kit.noKitLoaded")}</p>
      </div>
    );
  }

  const kitCommands = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kitEditorCommands = useKitEditorCommands();
  const kitEditor = useKitEditor();

  // Derive artifact kind from search params
  const selectedKind = searchParams.get("k") as ArtifactKind | null;

  // Get name/variant/view filters from search params
  const selectedName = searchParams.get("name");
  const selectedVariant = searchParams.get("variant");
  const selectedView = searchParams.get("view");

  // Get concepts and search from search params
  const selectedConcepts = searchParams.getAll("c");
  const searchQuery = searchParams.get("q") || "";
  const expandedRows = new Set(searchParams.getAll("e"));

  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const editorType = useEditorType();

  const allConcepts = useMemo(() => {
    const conceptSet = new Set<string>();
    kit.designs?.forEach((d: Design) => d.concepts?.forEach((c: string) => conceptSet.add(c)));
    return Array.from(conceptSet).sort();
  }, [kit.designs]);

  // Collect unique names for the selected kind (or unified when no kind selected)
  const uniqueNames = useMemo(() => {
    const nameSet = new Set<string>();
    if (!selectedKind || selectedKind === "designs") {
      kit.designs?.forEach((d: Design) => nameSet.add(d.name));
    }
    if (!selectedKind || selectedKind === "types") {
      kit.types?.forEach((t: Type) => nameSet.add(t.name));
    }
    return Array.from(nameSet).sort();
  }, [kit.designs, kit.types, selectedKind]);

  // Collect unique variants for the selected name
  const uniqueVariants = useMemo(() => {
    if (!selectedName) return [];
    const variantSet = new Set<string>();
    if (!selectedKind || selectedKind === "designs") {
      kit.designs?.forEach((d: Design) => {
        if (d.name === selectedName) {
          variantSet.add(d.variant || "");
        }
      });
    }
    if (!selectedKind || selectedKind === "types") {
      kit.types?.forEach((t: Type) => {
        if (t.name === selectedName) {
          variantSet.add(t.variant || "");
        }
      });
    }
    return Array.from(variantSet).sort();
  }, [kit.designs, kit.types, selectedKind, selectedName]);

  // Collect unique views for the selected name and variant (only for designs)
  const uniqueViews = useMemo(() => {
    if (!selectedName || !selectedVariant || selectedKind !== "designs") return [];
    const viewSet = new Set<string>();
    kit.designs?.forEach((d: Design) => {
      if (d.name === selectedName && (d.variant || "") === selectedVariant) {
        viewSet.add(d.view || "");
      }
    });
    return Array.from(viewSet).sort();
  }, [kit.designs, selectedKind, selectedName, selectedVariant]);

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

    return result;
  }, [kit, selectedKind, selectedName, selectedVariant, selectedView, selectedConcepts, searchQuery, expandedRows]);

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
    const params = new URLSearchParams(searchParams);
    if (selectedKind === kind) {
      // If already selected, remove filter
      params.delete("k");
    } else {
      // Set the selected kind
      params.set("k", kind);
    }
    navigate(`/${kit.guid}?${params.toString()}`);
  };

  const toggleConcept = (concept: string) => {
    const newParams = new URLSearchParams(searchParams);
    const currentConcepts = newParams.getAll("c");

    if (currentConcepts.includes(concept)) {
      // Remove concept
      newParams.delete("c");
      currentConcepts.filter((c) => c !== concept).forEach((c) => newParams.append("c", c));
    } else {
      // Add concept
      newParams.append("c", concept);
    }

    setSearchParams(newParams);
  };

  const toggleName = (name: string) => {
    const params = new URLSearchParams(searchParams);
    if (selectedName === name) {
      params.delete("name");
      params.delete("variant");
      params.delete("view");
    } else {
      params.set("name", name);
      params.delete("variant");
      params.delete("view");
    }
    setSearchParams(params);
  };

  const toggleVariant = (variant: string) => {
    const params = new URLSearchParams(searchParams);
    if (selectedVariant === variant) {
      params.delete("variant");
      params.delete("view");
    } else {
      params.set("variant", variant);
      params.delete("view");
    }
    setSearchParams(params);
  };

  const toggleView = (view: string) => {
    const params = new URLSearchParams(searchParams);
    if (selectedView === view) {
      params.delete("view");
    } else {
      params.set("view", view);
    }
    setSearchParams(params);
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col lg:flex-row lg:items-center gap-2 p-4 border-b">
        <div className="flex flex-wrap gap-2 lg:flex-shrink-0">
          {(!selectedKind || selectedKind === "designs") && (
            <Toggle
              type="withAction"
              pressed={selectedKind === "designs"}
              onPressedChange={() => toggleKind("designs")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("designs")}
              tooltip={selectedKind === "designs" ? t("kitEditor.hideDesigns") : t("kitEditor.showDesigns")}
              actionTooltip={t("kitEditor.createDesign")}
            >
              <Layout className="size-4" />
            </Toggle>
          )}
          {(!selectedKind || selectedKind === "types") && (
            <Toggle
              type="withAction"
              pressed={selectedKind === "types"}
              onPressedChange={() => toggleKind("types")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("types")}
              tooltip={selectedKind === "types" ? t("kitEditor.hideTypes") : t("kitEditor.showTypes")}
              actionTooltip={t("kitEditor.createType")}
            >
              <Box className="size-4" />
            </Toggle>
          )}
          {(!selectedKind || selectedKind === "qualities") && (
            <Toggle
              type="withAction"
              pressed={selectedKind === "qualities"}
              onPressedChange={() => toggleKind("qualities")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("qualities")}
              tooltip={selectedKind === "qualities" ? t("kitEditor.hideQualities") : t("kitEditor.showQualities")}
              actionTooltip={t("kitEditor.createQuality")}
            >
              <Award className="size-4" />
            </Toggle>
          )}
          {(!selectedKind || selectedKind === "files") && (
            <Toggle
              type="withAction"
              pressed={selectedKind === "files"}
              onPressedChange={() => toggleKind("files")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("files")}
              tooltip={selectedKind === "files" ? t("kitEditor.hideFiles") : t("kitEditor.showFiles")}
              actionTooltip={t("kitEditor.createFile")}
            >
              <FileText className="size-4" />
            </Toggle>
          )}
          {(!selectedKind || selectedKind === "authors") && (
            <Toggle
              type="withAction"
              pressed={selectedKind === "authors"}
              onPressedChange={() => toggleKind("authors")}
              actionIcon={<Plus className="size-3.5 opacity-50" />}
              onActionClick={() => handleCreateArtifact("authors")}
              tooltip={selectedKind === "authors" ? t("kitEditor.hideAuthors") : t("kitEditor.showAuthors")}
              actionTooltip={t("kitEditor.createAuthor")}
            >
              <User className="size-4" />
            </Toggle>
          )}
          {allConcepts.length > 0 &&
            allConcepts.map((concept) => (
              <Toggle
                key={concept}
                pressed={selectedConcepts.includes(concept)}
                onPressedChange={() => toggleConcept(concept)}
                tooltip={selectedConcepts.includes(concept) ? t("kitEditor.hideConcept", { concept }) : t("kitEditor.showConcept", { concept })}
              >
                {concept}
              </Toggle>
            ))}
          {uniqueNames.length > 0 &&
            uniqueNames.map((name) => (
              <Toggle key={name} pressed={selectedName === name} onPressedChange={() => toggleName(name)}>
                {name}
              </Toggle>
            ))}
          {uniqueVariants.length > 0 &&
            uniqueVariants.map((variant) => (
              <Toggle key={variant} pressed={selectedVariant === variant} onPressedChange={() => toggleVariant(variant)}>
                {variant}
              </Toggle>
            ))}
          {uniqueViews.length > 0 &&
            uniqueViews.map((view) => (
              <Toggle key={view} pressed={selectedView === view} onPressedChange={() => toggleView(view)}>
                {view}
              </Toggle>
            ))}
        </div>
        <Input className="w-full lg:w-auto lg:flex-1 lg:min-w-[200px]" placeholder={t("common.search")} value={searchQuery} onChange={(e) => kitEditorCommands.setFilterSearch(e.target.value)} />
      </div>
      <ScrollArea className="flex-1">
        <table className="w-full border-collapse">
          <thead className="sticky top-0 bg-background border-b">
            <tr>
              <th className="text-left p-2 font-medium">{t("kitEditor.name")}</th>
              {!selectedKind && <th className="text-left p-2 font-medium">{t("kitEditor.kind")}</th>}
              <th className="text-left p-2 font-medium">{t("kitEditor.lastUpdated")}</th>
              <th className="text-left p-2 font-medium">{t("kitEditor.created")}</th>
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
                    <button
                      className="cursor-pointer hover:underline text-left"
                      onClick={() => {
                        if (row.kind === "designs") sketchpadCommands.navigateToDesign(kit.guid, (row.data as Design).guid);
                        else if (row.kind === "types") sketchpadCommands.navigateToType(kit.guid, (row.data as Type).guid);
                      }}
                    >
                      {row.artifact}
                    </button>
                  </div>
                </td>
                {!selectedKind && (
                  <td className="p-2">
                    {row.kind === "designs" && <Layout className="size-4" />}
                    {row.kind === "types" && <Box className="size-4" />}
                    {row.kind === "qualities" && <Award className="size-4" />}
                    {row.kind === "files" && <FileText className="size-4" />}
                    {row.kind === "authors" && <User className="size-4" />}
                  </td>
                )}
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
