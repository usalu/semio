// #region Header
// semio/algorithms/.storybook/stories/Move.stories.tsx
// Specs: Pure UI proxy to nativeFlatDesign + nativeMovePieces. Uses MoveStoryDesign (tilted root plane, rich connection params). Diff panel lists connection numeric fields.
// Summary: Flat input design via nativeFlatDesign; nativeMovePieces returns flat input, output with connections, and move diff; AlgorithmApp details show gap/shift/rise/rotation/turn/tilt/u/v per connection update.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion Header

import type { Design, DesignDiff } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeFlatDesign, nativeMovePieces, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import { DragPieces, MoveStoryDesign, MoveVector } from "../../../assets/index";

const rawDesign = { ...MoveStoryDesign, guid: "move-preset-guid", name: "Move Preset" };

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "move-vector", kind: WindowKind.VECTOR_INPUT, label: "Vector" },
  { id: "move-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "move-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "move-output", kind: WindowKind.SCENE, label: "Output" },
];

function MoveFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;

  const kit = React.useMemo(
    () => ({
      ...metabolismKit,
      designs: [...((metabolismKit as any).designs || []), rawDesign],
    }),
    [],
  ) as any;

  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>((DragPieces as any).pieces?.map((p: any) => p.guid) ?? []);
  const [vector, setVector] = React.useState(MoveVector as { gap: number; shift: number; rise: number });
  const [outputDesign, setOutputDesign] = React.useState<Design | undefined>(undefined);
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);
  const [moveError, setMoveError] = React.useState<string | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlatInputDesign(null);
    setOutputDesign(undefined);
    setDesignDiff(undefined);
    setMoveError(undefined);
    void (async () => {
      const flat = await nativeFlatDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      setFlatInputDesign(flat);
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
    if (!flatInputDesign) return;
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
      setMoveError(undefined);
      try {
        const { output, moveDiff } = await nativeMovePieces(kit, rawDesign as Design, selectedPieceGuids, vector, language);
        if (!cancelled) {
          setDesignDiff(moveDiff);
          setOutputDesign(output);
        }
      } catch (e) {
        if (!cancelled) {
          setDesignDiff(undefined);
          setOutputDesign(undefined);
          setMoveError(e instanceof Error ? e.message : String(e));
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [flatInputDesign, kit, selectedPieceGuids, vector, language]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      moveVector: vector,
      onMoveVectorChange: setVector,
      moveVectorMin: { gap: -10, shift: -10, rise: -10 },
      moveVectorMax: { gap: 10, shift: 10, rise: 10 },
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (outputDesign ?? flatInputDesign ?? rawDesign) as Design,
      error: moveError ? moveError : !flatInputDesign ? `Loading move preview (${language})…` : selectedPieceGuids.length === 0 ? "Select at least one piece to move." : undefined,
    }),
    [kit, flatInputDesign, selectedPieceGuids, vector, designDiff, outputDesign, moveError, language],
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
