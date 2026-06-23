// #region 🧲Header
// 💻 compose/algorithm/.storybook/story/FindReplaceableTypesInDesigns.stories.tsx
// Specs: Find-replaceable-types renders the full Nakagin Capsule Tower as the source design and a compatible design tree output.
// Summary: Separate Storybook entry for compatible replacement types and designs with live selection-driven tree rendering.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import {
  AlgorithmApp,
  NAKAGIN_CAPSULE_TOWER_DESIGN_ID,
  WindowKind,
  designFromKit,
  designsFromKit,
  findReplaceableTypesForSelection,
  nakaginStoryCopySelection,
  typesFromKit,
  useAlgorithm,
  useFlatDesignPreview,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
  type Design,
} from "@compose/algorithm";

import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import { MetabolismKit as metabolismKit } from "@compose/asset";

const rawDesign = designFromKit(metabolismKit, NAKAGIN_CAPSULE_TOWER_DESIGN_ID)!;
const { pieceIds: selectionPieceIds, connectionIds: selectionConnectionIds } = nakaginStoryCopySelection();

//#region 🔍FindReplaceableTypesInDesigns
// Summary: Renders the full Nakagin Capsule Tower source selection and a live compatible tree from the selected pieces.
// Specs: Uses the Nakagin copy-selection asset for the source window, then derives compatible types and designs from live selection state.

type CompatibleDesignTreeNode = {
  design: Design;
  children: CompatibleDesignTreeNode[];
};

/**
 * Builds the visible design tree for the compatible design output.
 */
function buildCompatibleDesignTree(designs: Design[], visibleDesignIds: Set<string>): CompatibleDesignTreeNode[] {
  const childrenByParent = new Map<string | null, Design[]>();
  for (const nextDesign of designs) {
    const parentId = nextDesign.parent?.id ?? null;
    const nextChildren = childrenByParent.get(parentId) ?? [];
    nextChildren.push(nextDesign);
    childrenByParent.set(parentId, nextChildren);
  }

  const buildNode = (nextDesign: Design): CompatibleDesignTreeNode | null => {
    const children = (childrenByParent.get(nextDesign.id) ?? []).map(buildNode).filter((child): child is CompatibleDesignTreeNode => child !== null);
    if (!visibleDesignIds.has(nextDesign.id) && children.length === 0) return null;
    return { design: nextDesign, children };
  };

  return (childrenByParent.get(null) ?? []).map(buildNode).filter((node): node is CompatibleDesignTreeNode => node !== null);
}

/**
 * Renders a single compatible-design node, muting parents and highlighting leaves.
 */
function renderCompatibleDesignTreeNode(node: CompatibleDesignTreeNode, depth = 0): React.ReactElement {
  const hasChildren = node.children.length > 0;
  const baseClassName = hasChildren ? "border-border/50 bg-muted/35 text-muted-foreground" : "border-success/40 bg-success/10 text-foreground";
  return (
    <div key={node.design.id} className="space-y-1" style={{ marginLeft: depth * 14 }}>
      <div className={`rounded-md border px-2 py-1 text-xs ${baseClassName}`}>
        <div className="flex items-center justify-between gap-2">
          <span className="truncate font-medium">{node.design.name ?? node.design.id}</span>
          <span className="shrink-0 font-mono text-[10px] uppercase tracking-[0.18em]">{hasChildren ? "muted" : "highlighted"}</span>
        </div>
      </div>
      {hasChildren && <div className="space-y-1">{node.children.map((child) => renderCompatibleDesignTreeNode(child, depth + 1))}</div>}
    </div>
  );
}

const CompatibleTypesAndDesignsWindow: React.FC = () => {
  const { kit, selectedPieceIds } = useAlgorithm();
  const allTypes = typesFromKit(kit);
  const allDesigns = designsFromKit(kit) as Design[];
  const result = React.useMemo(() => findReplaceableTypesForSelection({ pieces: selectedPieceIds }), [selectedPieceIds]);
  const typeById = React.useMemo(() => new Map(allTypes.map((nextType) => [nextType.id, nextType] as const)), [allTypes]);
  const visibleDesignIds = React.useMemo(() => new Set(result.designs), [result.designs]);
  const designForest = React.useMemo(() => buildCompatibleDesignTree(allDesigns, visibleDesignIds), [allDesigns, visibleDesignIds]);

  return (
    <div className="flex h-full min-h-0 flex-col gap-3 overflow-auto border-border/40 border-t p-3">
      <div className="flex items-start justify-between gap-3">
        <div className="space-y-1">
          <div className="text-sm font-semibold">Compatible Types And Designs</div>
          <div className="text-xs text-muted-foreground">
            {selectedPieceIds.length} selected pieces, {result.types.length} compatible types, {result.designs.length} compatible designs
          </div>
        </div>
        <div className="text-right text-[11px] leading-4 text-muted-foreground">
          <div>Parents muted</div>
          <div>Leaves highlighted</div>
        </div>
      </div>
      <div className="space-y-2">
        <div className="text-xs font-medium text-muted-foreground">Compatible types</div>
        <div className="flex flex-wrap gap-1.5">
          {result.types.map((typeId) => {
            const nextType = typeById.get(typeId);
            return (
              <span key={typeId} className="rounded-full border border-border/40 bg-background px-2 py-0.5 text-[11px] text-foreground">
                {nextType?.name ?? typeId}
              </span>
            );
          })}
        </div>
      </div>
      <div className="space-y-2">
        <div className="text-xs font-medium text-muted-foreground">Compatible design tree</div>
        <div className="space-y-1">{designForest.map((node) => renderCompatibleDesignTreeNode(node))}</div>
      </div>
    </div>
  );
};

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "fr-src", kind: WindowKind.SELECTION_INPUT, label: "Source Selection" },
  { id: "fr-out", kind: WindowKind.DESIGN_OUTPUT, label: "Compatible Tree", component: CompatibleTypesAndDesignsWindow },
];

const DEFAULT_LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      {
        kind: "stack" as const,
        size: 40,
        children: [{ kind: "window" as const, windowKindId: "fr-src", title: "Source Selection" }],
      },
      {
        kind: "stack" as const,
        size: 60,
        children: [{ kind: "window" as const, windowKindId: "fr-out", title: "Compatible Tree" }],
      },
    ],
  },
};

function FindReplaceableTypesInDesignsFrame() {
  const kit = metabolismKit;
  const { flatInputDesign, diagramLayoutDiff, loading } = useFlatDesignPreview(kit, String(rawDesign.id ?? NAKAGIN_CAPSULE_TOWER_DESIGN_ID));
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>(selectionPieceIds);
  const [selectedConnectionIds, setSelectedConnectionIds] = React.useState<string[]>(selectionConnectionIds);

  const design = (flatInputDesign ?? rawDesign) as Design;

  const context = React.useMemo<AlgorithmContextValue>(
    () => ({
      kit,
      design,
      diagramLayoutDiff,
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      selectedConnectionIds,
      onSelectedConnectionIdsChange: setSelectedConnectionIds,
      outputDesign: design,
      error: loading ? "Loading design…" : undefined,
    }),
    [design, diagramLayoutDiff, kit, loading, selectedConnectionIds, selectedPieceIds],
  );

  return <AlgorithmApp id="find-replaceable-types-in-designs" label="Find Replaceable Types In Designs" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />;
}

//#endregion 🔍FindReplaceableTypesInDesigns

const meta = {
  title: "🏘️compose🧪algorithms/FindReplaceableTypesInDesigns",
  component: FindReplaceableTypesInDesignsFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof FindReplaceableTypesInDesignsFrame>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <FindReplaceableTypesInDesignsFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Find Replaceable Types In Designs/i, { timeout: 120_000 });
  },
};
