// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: Pure UI proxy to flatDesign + dragPieces via shared story hooks.
// Summary: IPO drag board — WASM flat input, layout diff, dragPieces output + diff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import {
  AlgorithmApp,
  WindowKind,
  dragPieces,
  mergeKitDesigns,
  pieceIdsFromWire,
  useAlgorithmAsyncRun,
  useFlatDesignPreview,
  useReconciledPieceSelection,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
} from "@semio/algorithms";

import metabolismKit from "@semio/assets/fixtures/metabolism.kit.semio.json";
import { DragDesign, DragOffset, DragPieces } from "@semio/assets";

const rawDesign = { ...DragDesign, id: "drag-preset-id", name: "Drag Preset" };
const defaultPieceIds = pieceIdsFromWire(DragPieces as { pieces?: { id?: string }[] });

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DragFrame() {
  const kit = React.useMemo(() => mergeKitDesigns(metabolismKit, rawDesign as Record<string, unknown>), []);
  const [vec, setVec] = React.useState(DragOffset);
  const { flatInputDesign, diagramLayoutDiff, loading: flatLoading, ready } = useFlatDesignPreview(kit, rawDesign.id);
  const { selectedPieceIds, setSelectedPieceIds } = useReconciledPieceSelection(defaultPieceIds, rawDesign, defaultPieceIds, ready);

  const { result, loading: runLoading, error } = useAlgorithmAsyncRun(
    ready && selectedPieceIds.length > 0,
    () => dragPieces(kit, rawDesign as Design, selectedPieceIds, vec),
    [kit, selectedPieceIds, vec.u, vec.v],
  );

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      vec,
      onVecChange: setVec,
      vecMin: { u: -10, v: -10 },
      vecMax: { u: 10, v: 10 },
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      designDiff: result?.dragDiff,
      diagramLayoutDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (result?.output ?? flatInputDesign ?? rawDesign) as Design,
      error: error ?? (flatLoading ? "Loading drag preview…" : selectedPieceIds.length === 0 ? "Select at least one piece to drag." : runLoading ? "Loading drag result…" : undefined),
    }),
    [kit, flatInputDesign, diagramLayoutDiff, selectedPieceIds, vec, result, flatLoading, runLoading, error],
  );

  return <AlgorithmApp id="drag" label="Drag" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio/algorithms/Drag",
  component: DragFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <DragFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Drag/i, { timeout: 120_000 });
  },
};
