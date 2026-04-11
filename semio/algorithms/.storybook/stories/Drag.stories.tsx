// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: Uses the AlgorithmApp shell with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Raw kit design in diagrams; nativeFlattenDesign supplies diagramLayoutDiff only; nativeDragPieces returns output + dragDiff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { DesignChange, Design, DesignDiff } from "@semio/js";
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
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>((DragPieces as any).pieces?.map((p: any) => p.guid) ?? []);
  const [vec, setVec] = React.useState(DragOffset);
  const [outputDesign, setOutputDesign] = React.useState<Design | undefined>(undefined);
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);
  const [dragError, setDragError] = React.useState<string | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    // Clear stale preview state immediately on language change to avoid rendering an old diff/output.
    // Keep user inputs (selection, vec) stable so different languages can be compared easily.
    setFlattenChange(null);
    setOutputDesign(undefined);
    setDesignDiff(undefined);
    setDragError(undefined);
    void (async () => {
      const fc = await nativeFlattenDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      if (!fc.ok) {
        setFlattenChange(null);
        return;
      }
      setFlattenChange(fc.change);
      setSelectedPieceGuids((prev) => {
        const pieceGuids = new Set<string>((rawDesign?.pieces ?? []).map((p: any) => p.guid));
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
    if (!flattenChange) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceGuids.length === 0) {
        if (!cancelled) {
          setOutputDesign(undefined);
          setDesignDiff(undefined);
        }
        return;
      }
      setOutputDesign(undefined);
      setDesignDiff(undefined);
      setDragError(undefined);
      try {
        const { output, dragDiff } = await nativeDragPieces(kit, rawDesign as Design, selectedPieceGuids, vec, language);
        if (!cancelled) {
          setDesignDiff(dragDiff);
          setOutputDesign(output);
        }
      } catch (e) {
        if (!cancelled) {
          setDesignDiff(undefined);
          setOutputDesign(undefined);
          setDragError(e instanceof Error ? e.message : String(e));
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [flattenChange, kit, selectedPieceGuids, vec, language]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: rawDesign,
      vec,
      onVecChange: setVec,
      vecMin: { u: -10, v: -10 },
      vecMax: { u: 10, v: 10 },
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff,
      diffDesign: rawDesign,
      diagramLayoutDiff: flattenChange?.forward,
      outputDesign: outputDesign ?? rawDesign,
      error: dragError
        ? dragError
        : !flattenChange
          ? `Loading drag preview (${language})…`
          : selectedPieceGuids.length === 0
            ? "Select at least one piece to drag."
            : !outputDesign
              ? `Loading drag result (${language})…`
              : undefined,
    }),
    [kit, selectedPieceGuids, vec, designDiff, outputDesign, flattenChange, dragError, language],
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
