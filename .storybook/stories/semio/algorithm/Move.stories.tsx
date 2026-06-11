// #region Header
// semio/algorithm/.storybook/story/Move.stories.tsx
// Specs: Pure UI proxy to flatDesign + movePieces via shared story hooks.
// Summary: IPO move board — vector input, WASM flat input, movePieces diff + SemioScene output.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion Header

import type { Design } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import {
  AlgorithmApp,
  WindowKind,
  mergeKitDesigns,
  movePieces,
  pieceIdsFromWire,
  useAlgorithmAsyncRun,
  useFlatDesignPreview,
  useReconciledPieceSelection,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
} from "@semio/algorithm";

import { MetabolismKit as metabolismKit } from "@semio/asset";
import { DragPieces, MoveStoryDesign, MoveVector } from "@semio/fixture";

const rawDesign = { ...MoveStoryDesign, id: "move-preset-id", name: "Move Preset" };
const defaultPieceIds = pieceIdsFromWire(DragPieces as { pieces?: { id?: string }[] });

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "move-vector", kind: WindowKind.VECTOR_INPUT, label: "Vector" },
  { id: "move-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "move-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "move-output", kind: WindowKind.SCENE, label: "Output" },
];

function MoveFrame() {
  const kit = React.useMemo(() => mergeKitDesigns(metabolismKit, rawDesign as Record<string, unknown>), []);
  const [vector, setVector] = React.useState(MoveVector as { gap: number; shift: number; rise: number });
  const { flatInputDesign, diagramLayoutDiff, loading: flatLoading, ready } = useFlatDesignPreview(kit, rawDesign.id);
  const { selectedPieceIds, setSelectedPieceIds } = useReconciledPieceSelection(defaultPieceIds, rawDesign, defaultPieceIds, ready);

  const { result, loading: runLoading, error } = useAlgorithmAsyncRun(
    ready && selectedPieceIds.length > 0,
    () => movePieces(kit, rawDesign as Design, selectedPieceIds, vector),
    [kit, selectedPieceIds, vector.gap, vector.shift, vector.rise],
  );

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      moveVector: vector,
      onMoveVectorChange: setVector,
      moveVectorMin: { gap: -10, shift: -10, rise: -10 },
      moveVectorMax: { gap: 10, shift: 10, rise: 10 },
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      designDiff: result?.moveDiff,
      diagramLayoutDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (result?.output ?? flatInputDesign ?? rawDesign) as Design,
      error: error ?? (flatLoading ? "Loading move preview…" : selectedPieceIds.length === 0 ? "Select at least one piece to move." : runLoading ? "Loading move result…" : undefined),
    }),
    [kit, flatInputDesign, diagramLayoutDiff, selectedPieceIds, vector, result, flatLoading, runLoading, error],
  );

  return <AlgorithmApp id="move" label="Move" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "🏘️semio🧪algorithms/Move",
  component: MoveFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <MoveFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Move/i, { timeout: 120_000 });
  },
};
