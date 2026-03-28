// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Uses the AlgorithmApp shell with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Delete story using nativeFlattenDesign/nativeDeletePieces with the Storybook language toolbar.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { applyDesignDiff } from "@semio/js";
import type { DesignChange } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";
import { nativeDeletePieces, nativeFlattenDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DeleteFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [designDiff, setDesignDiff] = React.useState<any | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    void (async () => {
      const fc = await nativeFlattenDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      setFlattenChange(fc);
      const bd = applyDesignDiff(rawDesign, { pieces: fc.forward.pieces }) as any;
      setBaseDesign(bd);
      setSelectedPieceGuids((bd?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid));
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  React.useEffect(() => {
    if (!baseDesign) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceGuids.length === 0) {
        if (!cancelled) setDesignDiff(undefined);
        return;
      }
      const diff = await nativeDeletePieces(kit, baseDesign, selectedPieceGuids, [], language);
      if (!cancelled) setDesignDiff(diff);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, baseDesign, selectedPieceGuids, language]);

  const outputDesign = React.useMemo(() => (designDiff && baseDesign ? applyDesignDiff(baseDesign, designDiff) : baseDesign), [designDiff, baseDesign]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign ?? rawDesign,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff,
      outputDesign,
      error: !flattenChange || !baseDesign ? `Loading delete preview (${language})…` : selectedPieceGuids.length === 0 ? "Select at least one piece to delete." : undefined,
    }),
    [kit, baseDesign, selectedPieceGuids, designDiff, outputDesign, flattenChange, language],
  );

  return <AlgorithmApp id="delete" label="Delete" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Delete",
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DeleteFrame /> };
