// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Cluster.stories.tsx
// Specs: Uses the AlgorithmApp shell with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Cluster story using real Diagram-based algorithm windows from @semio/ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "cluster-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "cluster-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cluster-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function ClusterFrame() {
  const kit = metabolismKit as any;
  const baseDesign = React.useMemo(() => (kit.designs ?? []).find((design: any) => design.guid === nakaginCapsuleTowerDesignGuid) as any, [kit]);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    setSelectedPieceGuids((baseDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid));
  }, [baseDesign, selectedPieceGuids.length]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      designGuid: nakaginCapsuleTowerDesignGuid,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff:
        selectedPieceGuids.length < 2
          ? undefined
          : {
              pieces: {
                added: [{ guid: `cluster-${selectedPieceGuids.length}`, name: `Clustered (${selectedPieceGuids.length} pieces)` }],
                updated: selectedPieceGuids.map((guid) => ({ piece: { guid } })),
              },
              connections: { updated: [] },
            },
      diffKit: kit,
      outputKit: kit,
      outputDesignGuid: nakaginCapsuleTowerDesignGuid,
      error: selectedPieceGuids.length < 2 ? "Select at least 2 pieces to cluster." : undefined,
    }),
    [kit, selectedPieceGuids],
  );

  return <AlgorithmApp id="cluster" label="Cluster" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Design/Cluster",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <ClusterFrame /> };
