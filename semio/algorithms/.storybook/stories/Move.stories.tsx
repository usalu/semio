// #region Header
// semio/algorithms/.storybook/stories/Move.stories.tsx
// Specs: Pure UI proxy to flatDesign + movePieces. Uses MoveStoryDesign (tilted root plane, rich connection params). Diff panel lists connection numeric fields.
// Summary: Flat input design via flatDesign (semio/rs WASM); movePieces returns flat input, output with connections, and move diff; AlgorithmApp details show gap/shift/rise/rotation/turn/tilt/u/v per connection update.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion Header

import type { Design, DesignDiff } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { flatDesign, movePieces } from "../../index";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import { DragPieces, MoveStoryDesign, MoveVector } from "../../../assets/index";

const rawDesign = { ...MoveStoryDesign, id: "move-preset-id", name: "Move Preset" };

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "move-vector", kind: WindowKind.VECTOR_INPUT, label: "Vector" },
  { id: "move-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "move-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "move-output", kind: WindowKind.SCENE, label: "Output" },
];

function MoveFrame() {
  const kit = React.useMemo(
    () => ({
      ...metabolismKit,
      designs: [...((metabolismKit as any).designs || []), rawDesign],
    }),
    [],
  ) as any;

  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>((DragPieces as any).pieces?.map((p: any) => p.id) ?? []);
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
      const flat = await flatDesign(kit, rawDesign.id);
      if (cancelled) return;
      setFlatInputDesign(flat);
      setSelectedPieceIds((prev) => {
        const pieceIds = new Set<string>((rawDesign?.pieces ?? []).map((p: any) => p.id));
        const filtered = prev.filter((g) => pieceIds.has(g));
        if (filtered.length > 0) return filtered;
        return (DragPieces as any).pieces?.map((p: any) => p.id) ?? [];
      });
    })();
    return () => {
      cancelled = true;
    };
  }, [kit]);

  React.useEffect(() => {
    if (!flatInputDesign) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceIds.length === 0) {
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
        const { output, moveDiff } = await movePieces(kit, rawDesign as Design, selectedPieceIds, vector);
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
  }, [flatInputDesign, kit, selectedPieceIds, vector]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      moveVector: vector,
      onMoveVectorChange: setVector,
      moveVectorMin: { gap: -10, shift: -10, rise: -10 },
      moveVectorMax: { gap: 10, shift: 10, rise: 10 },
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (outputDesign ?? flatInputDesign ?? rawDesign) as Design,
      error: moveError ? moveError : !flatInputDesign ? "Loading move preview…" : selectedPieceIds.length === 0 ? "Select at least one piece to move." : undefined,
    }),
    [kit, flatInputDesign, selectedPieceIds, vector, designDiff, outputDesign, moveError],
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
