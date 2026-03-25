// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: Uses the AlgorithmApp shell with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Drag story using nativeFlattenDesign and nativeDragPieces with the Storybook language toolbar.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { DesignChange, DesignDiff } from "@semio/js";
import { applyDesignDiff } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeDragPieces, nativeFlattenDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import { DragDesign, DragOffset, DragPieces } from "../../../assets/index";

const rawDesign = { ...DragDesign, guid: "drag-preset-guid", name: "Drag Preset" };

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DragFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  
  const kit = React.useMemo(() => ({
    ...metabolismKit,
    designs: [...((metabolismKit as any).designs || []), rawDesign],
  }), []) as any;

  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>((DragPieces as any).pieces?.map((p: any) => p.guid) ?? []);
  const [vec, setVec] = React.useState(DragOffset);
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    // Clear stale preview state immediately on language change to avoid rendering an old diff/output.
    // Keep user inputs (selection, vec) stable so different languages can be compared easily.
    setFlattenChange(null);
    setBaseDesign(null);
    setDesignDiff(undefined);
    void (async () => {
      const fc = await nativeFlattenDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      setFlattenChange(fc);
      const bd = applyDesignDiff(rawDesign, fc.forward) as any;
      setBaseDesign(bd);
      setSelectedPieceGuids((prev) => {
        const pieceGuids = new Set<string>((bd?.pieces ?? []).map((p: any) => p.guid));
        const filtered = prev.filter((g) => pieceGuids.has(g));
        if (filtered.length > 0) return filtered;
        return (DragPieces as any).pieces?.map((p: any) => p.guid) ?? [];
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
      if (selectedPieceGuids.length === 0) {
        if (!cancelled) setDesignDiff(undefined);
        return;
      }
      // Invalidate stale output while recomputing.
      setDesignDiff(undefined);
      const diff = await nativeDragPieces(baseDesign, selectedPieceGuids, vec, language);
      if (!cancelled) setDesignDiff(diff);
    })();
    return () => {
      cancelled = true;
    };
  }, [baseDesign, selectedPieceGuids, vec, language]);

  const outputDesign = React.useMemo(() => (designDiff && baseDesign ? applyDesignDiff(baseDesign, designDiff) : (baseDesign ?? rawDesign)), [designDiff, baseDesign]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign ?? rawDesign,
      vec,
      onVecChange: setVec,
      vecMin: { u: -10, v: -10 },
      vecMax: { u: 10, v: 10 },
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff,
      outputDesign,
      error: !flattenChange || !baseDesign ? `Loading drag preview (${language})…` : selectedPieceGuids.length === 0 ? "Select at least one piece to drag." : !designDiff ? `Loading drag result (${language})…` : undefined,
    }),
    [kit, baseDesign, selectedPieceGuids, vec, designDiff, outputDesign, flattenChange, language],
  );

  return <AlgorithmApp id="drag" label="Drag" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Drag",
  component: DragFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <DragFrame /> };
