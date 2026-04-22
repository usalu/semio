// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/FindReplaceableTypesInDesigns.stories.tsx
// Specs: Find-replaceable-types renders the full Nakagin Capsule Tower as the source design and a compatible design tree output.
// Summary: Separate Storybook entry for compatible replacement types and designs with live selection-driven tree rendering.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { Kit as KitRuntime, type Design } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef, useAlgorithm } from "../../index";
import { nativeFlatDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import { NakaginCapsuleTowerCopySelection } from "../../../assets/index";
import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = ((metabolismKit as any).designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;

/** Omits t_f1_b_c1 and t_f0→t_f1 link so t_f1 is external-stub in clipboard; includes t_f5/regressions. */
const omitFromCopySelectionPieceGuids = new Set<string>(["31be08e1-e75c-4024-86b4-c3c6d3939fbb"]);
const omitFromCopySelectionConnectionGuids = new Set<string>(["b1ecc6c5-722a-4814-9047-a87222bbaa4d"]);
const selectionPieceGuids = Array.from(
  new Set(
    [...(((NakaginCapsuleTowerCopySelection as any).pieces ?? []) as { guid: string }[]).map((p) => p.guid), "9c1ec7a2-13c2-4d23-b7bd-1efe2663d0a9", "5feebbf8-33d9-41ad-a13a-24c271a1860b"].filter((g) => !omitFromCopySelectionPieceGuids.has(g)),
  ),
) as string[];
const selectionConnectionGuids = Array.from(
  new Set(
    [...(((NakaginCapsuleTowerCopySelection as any).connections ?? []) as { guid: string }[]).map((c) => c.guid), "eb8ce9ce-091c-4495-a651-fa703748dfef", "4d5ff333-d70a-43e1-8b7a-8849c8c91405"].filter(
      (g) => !omitFromCopySelectionConnectionGuids.has(g),
    ),
  ),
) as string[];

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
function buildCompatibleDesignTree(designs: Design[], visibleDesignGuids: Set<string>): CompatibleDesignTreeNode[] {
  const childrenByParent = new Map<string | null, Design[]>();
  for (const nextDesign of designs) {
    const parentGuid = nextDesign.parent?.guid ?? null;
    const nextChildren = childrenByParent.get(parentGuid) ?? [];
    nextChildren.push(nextDesign);
    childrenByParent.set(parentGuid, nextChildren);
  }

  const buildNode = (nextDesign: Design): CompatibleDesignTreeNode | null => {
    const children = (childrenByParent.get(nextDesign.guid) ?? []).map(buildNode).filter((child): child is CompatibleDesignTreeNode => child !== null);
    if (!visibleDesignGuids.has(nextDesign.guid) && children.length === 0) return null;
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
    <div key={node.design.guid} className="space-y-1" style={{ marginLeft: depth * 14 }}>
      <div className={`rounded-md border px-2 py-1 text-xs ${baseClassName}`}>
        <div className="flex items-center justify-between gap-2">
          <span className="truncate font-medium">{node.design.name ?? node.design.guid}</span>
          <span className="shrink-0 font-mono text-[10px] uppercase tracking-[0.18em]">{hasChildren ? "muted" : "highlighted"}</span>
        </div>
      </div>
      {hasChildren && <div className="space-y-1">{node.children.map((child) => renderCompatibleDesignTreeNode(child, depth + 1))}</div>}
    </div>
  );
}

const CompatibleTypesAndDesignsWindow: React.FC = () => {
  const { kit, design, selectedPieceGuids } = useAlgorithm();
  const allTypes = (kit.types ?? []) as { guid: string; name?: string }[];
  const allDesigns = (kit.designs ?? []) as Design[];
  const allPorts = React.useMemo(() => ((kit.families ?? kit.ports ?? []) as any[]).flatMap((entry) => entry.ports ?? [entry]), [kit]);
  const result = React.useMemo(() => KitRuntime.ensure(kit).findReplaceableTypesInDesignsForPiecesInDesignOp(design, allDesigns, kit.types ?? [], allPorts, { pieces: selectedPieceGuids }), [allDesigns, allPorts, design, kit, selectedPieceGuids]);
  const typeByGuid = React.useMemo(() => new Map(allTypes.map((nextType) => [nextType.guid, nextType] as const)), [allTypes]);
  const visibleDesignGuids = React.useMemo(() => new Set(result.designs), [result.designs]);
  const designForest = React.useMemo(() => buildCompatibleDesignTree(allDesigns, visibleDesignGuids), [allDesigns, visibleDesignGuids]);

  return (
    <div className="flex h-full min-h-0 flex-col gap-3 overflow-auto border-border/40 border-t p-3">
      <div className="flex items-start justify-between gap-3">
        <div className="space-y-1">
          <div className="text-sm font-semibold">Compatible Types And Designs</div>
          <div className="text-xs text-muted-foreground">
            {selectedPieceGuids.length} selected pieces, {result.types.length} compatible types, {result.designs.length} compatible designs
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
          {result.types.map((typeGuid) => {
            const nextType = typeByGuid.get(typeGuid);
            return (
              <span key={typeGuid} className="rounded-full border border-border/40 bg-background px-2 py-0.5 text-[11px] text-foreground">
                {nextType?.name ?? typeGuid}
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
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flatSourceDesign, setFlatSourceDesign] = React.useState<Design | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>(selectionPieceGuids);
  const [selectedConnectionGuids, setSelectedConnectionGuids] = React.useState<string[]>(selectionConnectionGuids);

  React.useEffect(() => {
    let cancelled = false;
    setFlatSourceDesign(null);
    void (async () => {
      const flatSrc = await nativeFlatDesign(kit, rawDesign.guid, language);
      if (!cancelled) setFlatSourceDesign(flatSrc);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  const design = (flatSourceDesign ?? rawDesign) as Design;

  const context = React.useMemo<AlgorithmContextValue>(
    () => ({
      kit,
      design,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      selectedConnectionGuids,
      onSelectedConnectionGuidsChange: setSelectedConnectionGuids,
      outputDesign: design,
      error: !flatSourceDesign ? `Loading design (${language})…` : undefined,
    }),
    [design, flatSourceDesign, kit, language, selectedConnectionGuids, selectedPieceGuids],
  );

  return <AlgorithmApp id="find-replaceable-types-in-designs" label="Find Replaceable Types In Designs" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />;
}

//#endregion 🔍FindReplaceableTypesInDesigns

const meta = {
  title: "semio-algorithms/FindReplaceableTypesInDesigns",
  component: FindReplaceableTypesInDesignsFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof FindReplaceableTypesInDesignsFrame>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <FindReplaceableTypesInDesignsFrame />,
};
