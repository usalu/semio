// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Move.stories.tsx
// Specs: Uses AlgorithmApp with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: IPO story for Design Move using the standardized AlgorithmApp shell.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/ui";
import { WindowKind } from "@elements/ui";
import { applyDesignDiff, dragPiecesInDesign, findDesignInKit, type Design, type DesignDiff, type Kit } from "@semio/js";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "move-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "move-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "move-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "move-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

const DEFAULT_LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      { kind: "stack" as const, size: 15, children: [{ kind: "window" as const, windowKindId: "move-vec", title: "Vec" }] },
      { kind: "stack" as const, size: 28, children: [{ kind: "window" as const, windowKindId: "move-input", title: "Input" }] },
      { kind: "stack" as const, size: 28, children: [{ kind: "window" as const, windowKindId: "move-diff", title: "Diff" }] },
      { kind: "stack" as const, size: 29, children: [{ kind: "window" as const, windowKindId: "move-output", title: "Output" }] },
    ],
  },
};

function normalizeDragDiff(baseDesign: Design, dragDiff: DesignDiff): DesignDiff {
  const basePieces = baseDesign.pieces ?? [];
  const updatedPieces = dragDiff.pieces?.updated as any[] | undefined;
  if (!updatedPieces || updatedPieces.length === 0) return dragDiff;
  return {
    ...dragDiff,
    pieces: {
      ...dragDiff.pieces,
      updated: updatedPieces.map((u) => {
        const base = basePieces.find((p) => p.guid === u.piece?.guid);
        const delta = u.diff?.center;
        if (!base?.center || !delta || typeof delta.u !== "number" || typeof delta.v !== "number") return u;
        return { ...u, diff: { ...u.diff, center: { u: (base.center.u ?? 0) + delta.u, v: (base.center.v ?? 0) + delta.v } } };
      }),
    },
  };
}

function MoveFrame() {
  const kit = metabolismKit as unknown as Kit;
  const baseDesign = React.useMemo(() => findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design, [kit]);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [vec, setVec] = React.useState({ u: 1, v: -2 });

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    setSelectedPieceGuids((baseDesign.pieces ?? []).slice(0, 3).map((p) => p.guid));
  }, [baseDesign, selectedPieceGuids.length]);

  const { designDiff, outputKit, error } = React.useMemo(() => {
    try {
      if (selectedPieceGuids.length === 0) return { designDiff: undefined, outputKit: kit, error: undefined };
      const piecesDesign = { guid: "", name: "", pieces: selectedPieceGuids.map((g) => ({ guid: g })) } as Design;
      const normalized = normalizeDragDiff(baseDesign, dragPiecesInDesign(baseDesign, piecesDesign, vec));
      const moveDiff: DesignDiff = { ...normalized, connections: undefined };
      const outDesign = applyDesignDiff(baseDesign, moveDiff);
      return { designDiff: moveDiff, outputKit: { ...kit, designs: (kit.designs ?? []).map((d) => (d.guid === outDesign.guid ? outDesign : d)) }, error: undefined };
    } catch (e: any) {
      return { designDiff: undefined, outputKit: kit, error: String(e?.message ?? e) };
    }
  }, [baseDesign, kit, selectedPieceGuids, vec.u, vec.v]);

  const context: AlgorithmContextValue = {
    kit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    vec,
    onVecChange: setVec,
    vecMin: { u: -10, v: -10 },
    vecMax: { u: 10, v: 10 },
    selectedPieceGuids,
    onSelectedPieceGuidsChange: setSelectedPieceGuids,
    designDiff,
    outputKit,
    outputDesignGuid: nakaginCapsuleTowerDesignGuid,
    error,
  };

  return <AlgorithmApp id="move" label="Move" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Design/Move",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <MoveFrame /> };
