// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Pure UI proxy to flatDesign + deletePieces via shared story hooks.
// Summary: IPO delete board — selection input, delete diff, output with applied diff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import {
  AlgorithmApp,
  WindowKind,
  NAKAGIN_CAPSULE_TOWER_DESIGN_ID,
  deletePieces,
  designFromKit,
  previewDesignWithAppliedDiff,
  useAlgorithmAsyncRun,
  useFlatDesignPreview,
  useReconciledPieceSelection,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
} from "@semio/algorithms";

import { MetabolismKit as metabolismKit } from "@semio/assets";

const rawDesign = designFromKit(metabolismKit, NAKAGIN_CAPSULE_TOWER_DESIGN_ID)!;
const defaultPieceIds = ((rawDesign.pieces as { id: string }[] | undefined) ?? []).slice(0, 3).map((p) => p.id);

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DeleteFrame() {
  const kit = metabolismKit;
  const [selectedConnectionIds, setSelectedConnectionIds] = React.useState<string[]>([]);
  const { flatInputDesign, diagramLayoutDiff, loading: flatLoading, ready } = useFlatDesignPreview(kit, rawDesign.id);
  const { selectedPieceIds, setSelectedPieceIds } = useReconciledPieceSelection([], rawDesign, defaultPieceIds, ready);

  const hasSelection = selectedPieceIds.length > 0 || selectedConnectionIds.length > 0;
  const { result: diffRes, loading: runLoading } = useAlgorithmAsyncRun(
    ready && hasSelection,
    async () => {
      const res = await deletePieces(flatInputDesign!, selectedPieceIds, selectedConnectionIds);
      return res.ok ? res.diff : undefined;
    },
    [kit, flatInputDesign, selectedPieceIds, selectedConnectionIds],
  );

  const designDiff = diffRes as DesignDiff | undefined;
  const outputDesign = previewDesignWithAppliedDiff(flatInputDesign, designDiff, rawDesign);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      diagramLayoutDiff,
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      selectedConnectionIds,
      onSelectedConnectionIdsChange: setSelectedConnectionIds,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: outputDesign as Design,
      error: flatLoading
        ? "Loading delete preview…"
        : !hasSelection
          ? "Select at least one piece or connection to delete."
          : runLoading
            ? "Loading delete result…"
            : undefined,
    }),
    [kit, flatInputDesign, diagramLayoutDiff, selectedPieceIds, selectedConnectionIds, designDiff, outputDesign, flatLoading, hasSelection, runLoading],
  );

  return <AlgorithmApp id="delete" label="Delete" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio/algorithms/Delete",
  component: DeleteFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <DeleteFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Delete/i, { timeout: 120_000 });
  },
};
