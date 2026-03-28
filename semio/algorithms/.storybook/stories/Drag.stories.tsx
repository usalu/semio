// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: Uses the AlgorithmApp shell with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Drag story using real Diagram-based algorithm windows from @semio/ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { applyDesignDiff, flattenDesign } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;
const flattenChange = flattenDesign(metabolismKit as any, rawDesign.guid);
const baseDesign = applyDesignDiff(rawDesign, { pieces: flattenChange.forward.pieces }) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DragFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as any;
  const initialSelection = React.useMemo(() => (baseDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid), []);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>(initialSelection);
  const [vec, setVec] = React.useState({ u: 1, v: -2 });

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign,
      vec,
      onVecChange: setVec,
      vecMin: { u: -10, v: -10 },
      vecMax: { u: 10, v: 10 },
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff: {
        pieces: { updated: selectedPieceGuids.map((guid) => ({ piece: { guid }, diff: { center: { ...vec }, language } })) },
        connections: { updated: [] },
      },
      outputDesign: baseDesign,
    }),
    [baseDesign, kit, language, selectedPieceGuids, vec],
  );

  return <AlgorithmApp id="drag" label="Drag" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Drag",
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DragFrame /> };
