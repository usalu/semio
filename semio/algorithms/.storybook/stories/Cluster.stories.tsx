// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Cluster.stories.tsx
// Specs: Uses AlgorithmApp with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: IPO story for Design Cluster using the standardized AlgorithmApp shell.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/ui";
import { WindowKind } from "@elements/ui";
import { applyDesignDiff, createClusteredDesign, findDesignInKit, replaceClusterWithDesign, type Design, type DesignDiff, type Kit } from "@semio/js";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "cluster-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "cluster-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cluster-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

const DEFAULT_LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "cluster-input", title: "Input" }] },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "cluster-diff", title: "Diff" }] },
      { kind: "stack" as const, size: 34, children: [{ kind: "window" as const, windowKindId: "cluster-output", title: "Output" }] },
    ],
  },
};

function ClusterFrame() {
  const kit = metabolismKit as unknown as Kit;
  const baseDesign = React.useMemo(() => findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design, [kit]);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    setSelectedPieceGuids((baseDesign.pieces ?? []).slice(0, 3).map((p) => p.guid));
  }, [baseDesign.pieces, selectedPieceGuids.length]);

  const { designDiff, diffKit, outputKit, error } = React.useMemo(() => {
    if (selectedPieceGuids.length < 2) return { designDiff: undefined, diffKit: kit, outputKit: kit, error: "Select at least 2 pieces to cluster." };
    try {
      const clusteredName = `Clustered (${selectedPieceGuids.length} pieces)`;
      const { clusteredDesign, externalConnections } = createClusteredDesign(baseDesign, selectedPieceGuids, clusteredName);
      const change = replaceClusterWithDesign(baseDesign, selectedPieceGuids, clusteredDesign, externalConnections);
      const diff = change.forward as DesignDiff;
      const outDesign = applyDesignDiff(baseDesign, diff);
      const designs = kit.designs ?? [];
      return {
        designDiff: diff,
        diffKit: { ...kit, designs: [...designs, clusteredDesign] },
        outputKit: { ...kit, designs: [...designs.map((d) => (d.guid === outDesign.guid ? outDesign : d)), clusteredDesign] },
        error: undefined,
      };
    } catch (e: any) {
      return { designDiff: undefined, diffKit: kit, outputKit: kit, error: String(e?.message ?? e) };
    }
  }, [baseDesign, kit, selectedPieceGuids]);

  const context: AlgorithmContextValue = {
    kit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    selectedPieceGuids,
    onSelectedPieceGuidsChange: setSelectedPieceGuids,
    designDiff,
    diffKit,
    outputKit,
    outputDesignGuid: nakaginCapsuleTowerDesignGuid,
    error,
  };

  return <AlgorithmApp id="cluster" label="Cluster" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Design/Cluster",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <ClusterFrame /> };
