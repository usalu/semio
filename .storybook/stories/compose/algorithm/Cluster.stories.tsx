// #region 🧲️Header
// 💻️ compose/algorithm/.storybook/story/Cluster.stories.tsx
// Specs: Pure UI proxy to flatDesign; cluster diff is a local story fixture.
// Summary: IPO cluster board — piece selection + stub group diff for Storybook.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Design } from "@semio-tech/compose-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import { AlgorithmApp, NAKAGIN_CAPSULE_TOWER_DESIGN_ID, WindowKind, designFromKit, useFlatDesignPreview, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio-tech/compose-algorithm";

import { MetabolismKit as metabolismKit } from "@semio-tech/semio-asset";

const rawDesign = designFromKit(metabolismKit, NAKAGIN_CAPSULE_TOWER_DESIGN_ID)!;
const defaultPieceIds = ((rawDesign.pieces as { id: string }[] | undefined) ?? []).slice(0, 3).map((p) => p.id);

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "cluster-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "cluster-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cluster-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function ClusterFrame() {
  const kit = metabolismKit;
  const { flatInputDesign, diagramLayoutDiff, loading } = useFlatDesignPreview(kit, String(rawDesign.id ?? NAKAGIN_CAPSULE_TOWER_DESIGN_ID));
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>(defaultPieceIds);

  const designDiff = React.useMemo(
    () =>
      selectedPieceIds.length < 2
        ? undefined
        : {
            pieces: {
              added: [{ id: `storybook-group-${selectedPieceIds.length}`, name: `Storybook group (${selectedPieceIds.length})` }],
              updated: selectedPieceIds.map((id) => ({ piece: { id }, diff: {} })),
            },
          },
    [selectedPieceIds],
  );

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      diagramLayoutDiff,
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (flatInputDesign ?? rawDesign) as Design,
      error: loading ? "Loading cluster preview…" : selectedPieceIds.length < 2 ? "Select at least 2 pieces to cluster." : undefined,
    }),
    [kit, flatInputDesign, diagramLayoutDiff, selectedPieceIds, designDiff, loading],
  );

  return <AlgorithmApp id="cluster" label="Cluster" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "🏘️compose🧪️algorithms/Cluster",
  component: ClusterFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <ClusterFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Cluster/i, { timeout: 120_000 });
  },
};
