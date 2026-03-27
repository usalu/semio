// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Move.stories.tsx
// Specs: Uses the AlgorithmApp shell with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Move story using real Diagram-based algorithm windows from @semio/ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { applyDesignDiff, flattenDesign } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef") as any;
const flattenChange = flattenDesign(metabolismKit as any, rawDesign.guid);
const nakaginDesign = applyDesignDiff(rawDesign, { pieces: flattenChange.forward.pieces }) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "move-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "move-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "move-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "move-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function MoveFrame() {
  const kit = metabolismKit as any;
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [vec, setVec] = React.useState({ u: 1, v: -2 });

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    setSelectedPieceGuids((nakaginDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid));
  }, [selectedPieceGuids.length]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: nakaginDesign,
      vec,
      onVecChange: setVec,
      vecMin: { u: -10, v: -10 },
      vecMax: { u: 10, v: 10 },
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff: { pieces: { updated: selectedPieceGuids.map((guid) => ({ piece: { guid }, diff: { center: { ...vec } } })) } },
      outputDesign: nakaginDesign,
    }),
    [kit, selectedPieceGuids, vec],
  );

  return <AlgorithmApp id="move" label="Move" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Move",
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <MoveFrame /> };
