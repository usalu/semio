// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Uses the AlgorithmApp shell with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Delete story using real Diagram-based algorithm windows from @semio/ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DeleteFrame() {
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
      design: baseDesign,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff: { pieces: { removed: selectedPieceGuids.map((guid) => ({ guid })) }, connections: { updated: [] } },
      outputDesign: baseDesign,
      error: selectedPieceGuids.length === 0 ? "Select at least one piece to delete." : undefined,
    }),
    [baseDesign, kit, selectedPieceGuids],
  );

  return <AlgorithmApp id="delete" label="Delete" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Delete",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DeleteFrame /> };
