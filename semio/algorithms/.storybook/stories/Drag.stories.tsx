// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: Uses the AlgorithmApp shell with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Drag story using real Diagram-based algorithm windows from @semio/ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DragFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as any;
  const baseDesign = React.useMemo(() => (kit.designs ?? []).find((design: any) => design.guid === nakaginCapsuleTowerDesignGuid) as any, [kit]);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [vec, setVec] = React.useState({ u: 1, v: -2 });

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    setSelectedPieceGuids((baseDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid));
  }, [baseDesign, selectedPieceGuids.length]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      designGuid: nakaginCapsuleTowerDesignGuid,
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
      outputKit: kit,
      outputDesignGuid: nakaginCapsuleTowerDesignGuid,
    }),
    [kit, language, selectedPieceGuids, vec],
  );

  return <AlgorithmApp id="drag" label="Drag" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Design/Drag",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DragFrame /> };
