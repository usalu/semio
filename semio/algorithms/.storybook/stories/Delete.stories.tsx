// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Uses AlgorithmApp with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: IPO story for Design Delete using the standardized AlgorithmApp shell.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/ui";
import { WindowKind } from "@elements/ui";
import { applyDesignDiff, findDesignInKit, removePiecesAndConnectionsFromDesign, type Design, type DesignDiff, type Kit } from "@semio/js";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

const DEFAULT_LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "delete-input", title: "Input" }] },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "delete-diff", title: "Diff" }] },
      { kind: "stack" as const, size: 34, children: [{ kind: "window" as const, windowKindId: "delete-output", title: "Output" }] },
    ],
  },
};

function DeleteFrame() {
  const kit = metabolismKit as unknown as Kit;
  const baseDesign = React.useMemo(() => findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design, [kit]);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    setSelectedPieceGuids((baseDesign.pieces ?? []).slice(0, 3).map((p) => p.guid));
  }, [baseDesign, selectedPieceGuids.length]);

  const { designDiff, outputKit, error } = React.useMemo(() => {
    try {
      if (selectedPieceGuids.length === 0) return { designDiff: undefined, outputKit: kit, error: "Select at least one piece to delete." };
      const pieceSet = new Set(selectedPieceGuids);
      const connectionIdsToRemove = (baseDesign.connections ?? []).filter((c) => pieceSet.has(c.connected.piece.guid) || pieceSet.has(c.connecting.piece.guid)).map((c) => c.guid);
      const change = removePiecesAndConnectionsFromDesign(kit, baseDesign.guid, selectedPieceGuids, connectionIdsToRemove);
      const diff = change.forward as DesignDiff;
      const outDesign = applyDesignDiff(baseDesign, diff);
      return { designDiff: diff, outputKit: { ...kit, designs: (kit.designs ?? []).map((d) => (d.guid === outDesign.guid ? outDesign : d)) }, error: undefined };
    } catch (e: any) {
      return { designDiff: undefined, outputKit: kit, error: String(e?.message ?? e) };
    }
  }, [baseDesign, kit, selectedPieceGuids]);

  const context: AlgorithmContextValue = {
    kit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    selectedPieceGuids,
    onSelectedPieceGuidsChange: setSelectedPieceGuids,
    designDiff,
    outputKit,
    outputDesignGuid: nakaginCapsuleTowerDesignGuid,
    error,
  };

  return <AlgorithmApp id="delete" label="Delete" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Design/Delete",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DeleteFrame /> };
