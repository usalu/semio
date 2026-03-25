// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Move.stories.tsx
// Specs: Uses the AlgorithmApp shell with VEC_INPUT, PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Move story using nativeFlattenDesign with the Storybook language toolbar.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { DesignChange } from "@semio/js";
import { applyDesignDiff } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeFlattenDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "move-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "move-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "move-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "move-output", kind: WindowKind.SCENE, label: "Output" },
];

function MoveFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [vec, setVec] = React.useState({ u: 1, v: -2 });

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

  const designDiff = React.useMemo(() => (selectedPieceGuids.length > 0 ? { pieces: { updated: selectedPieceGuids.map((guid) => ({ piece: { guid }, diff: { center: { ...vec } } })) } } : undefined), [selectedPieceGuids, vec]);

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
      error: !flattenChange || !baseDesign ? `Loading move preview (${language})…` : selectedPieceGuids.length === 0 ? "Select at least one piece to move." : undefined,
    }),
    [kit, baseDesign, selectedPieceGuids, vec, designDiff, outputDesign, flattenChange, language],
  );

  return <AlgorithmApp id="move" label="Move" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Move",
  component: MoveFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <MoveFrame /> };
