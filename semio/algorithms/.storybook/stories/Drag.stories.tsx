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

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DragFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [vec, setVec] = React.useState({ u: 1, v: -2 });
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    void (async () => {
      const fc = await nativeFlattenDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      setFlattenChange(fc);
      const bd = applyDesignDiff(rawDesign, fc.forward) as any;
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
      error: !flattenChange || !baseDesign ? `Loading drag preview (${language})…` : selectedPieceGuids.length === 0 ? "Select at least one piece to drag." : undefined,
    }),
    [kit, baseDesign, selectedPieceGuids, vec, designDiff, outputDesign, flattenChange, language],
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
