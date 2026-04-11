// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Uses the AlgorithmApp shell with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Delete story using nativeFlattenDesign/nativeDeletePieces with the Storybook language toolbar.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { DesignChange } from "@semio/js";
import { applyDesignDiff } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeDeletePieces, nativeFlattenDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DeleteFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [selectedConnectionGuids, setSelectedConnectionGuids] = React.useState<string[]>([]);
  const [designDiff, setDesignDiff] = React.useState<any | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    // Clear stale preview state immediately on language change to avoid rendering an old diff/output.
    setFlattenChange(null);
    setBaseDesign(null);
    setDesignDiff(undefined);
    void (async () => {
      const fc = await nativeFlattenDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      if (!fc.ok) {
        setFlattenChange(null);
        setBaseDesign(null);
        return;
      }
      setFlattenChange(fc.change);
      const bd = applyDesignDiff(rawDesign, fc.change.forward) as any;
      setBaseDesign(bd);
      setSelectedPieceGuids((prev) => {
        const pieceGuids = new Set<string>((bd?.pieces ?? []).map((p: any) => p.guid));
        const filtered = prev.filter((g) => pieceGuids.has(g));
        if (filtered.length > 0) return filtered;
        return (bd?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid);
      });
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  React.useEffect(() => {
    if (!baseDesign) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0) {
        if (!cancelled) setDesignDiff(undefined);
        return;
      }
      // Invalidate stale output while recomputing.
      setDesignDiff(undefined);
      const diffRes = await nativeDeletePieces(kit, baseDesign, selectedPieceGuids, selectedConnectionGuids, language);
      if (!cancelled) setDesignDiff(diffRes.ok ? diffRes.change : undefined);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, baseDesign, selectedPieceGuids, selectedConnectionGuids, language]);

  const outputDesign = React.useMemo(() => (designDiff && baseDesign ? applyDesignDiff(baseDesign, designDiff) : (baseDesign ?? rawDesign)), [designDiff, baseDesign]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign ?? rawDesign,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      selectedConnectionGuids,
      onSelectedConnectionGuidsChange: setSelectedConnectionGuids,
      designDiff,
      outputDesign,
      error: !flattenChange || !baseDesign ? `Loading delete preview (${language})…` : (selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0) ? "Select at least one piece or connection to delete." : !designDiff ? `Loading delete result (${language})…` : undefined,
    }),
    [kit, baseDesign, selectedPieceGuids, selectedConnectionGuids, designDiff, outputDesign, flattenChange, language],
  );

  return <AlgorithmApp id="delete" label="Delete" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Delete",
  component: DeleteFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DeleteFrame /> };
