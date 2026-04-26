// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: Pure UI proxy to flatDesign + dragPieces. No domain logic. All designs include connections.
// Summary: Flat input design via flatDesign (semio/rs WASM); dragPieces returns flat input, output with connections, and drag diff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { dragPieces, flatDesign } from "../../index";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import { DragDesign, DragOffset, DragPieces } from "../../../assets/index";

const rawDesign = { ...DragDesign, id: "drag-preset-id", name: "Drag Preset" };

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "drag-vec", kind: WindowKind.VEC_INPUT, label: "Vec" },
  { id: "drag-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "drag-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "drag-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DragFrame() {
  const kit = React.useMemo(
    () => ({
      ...metabolismKit,
      designs: [...((metabolismKit as any).designs || []), rawDesign],
    }),
    [],
  ) as any;

  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>((DragPieces as any).pieces?.map((p: any) => p.id) ?? []);
  const [vec, setVec] = React.useState(DragOffset);
  const [outputDesign, setOutputDesign] = React.useState<Design | undefined>(undefined);
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);
  const [dragError, setDragError] = React.useState<string | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlatInputDesign(null);
    setOutputDesign(undefined);
    setDesignDiff(undefined);
    setDragError(undefined);
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
      setDragError(undefined);
      try {
        const { output, dragDiff } = await dragPieces(kit, rawDesign as Design, selectedPieceIds, vec);
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
  }, [flatInputDesign, kit, selectedPieceIds, vec]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      vec,
      onVecChange: setVec,
      vecMin: { u: -10, v: -10 },
      vecMax: { u: 10, v: 10 },
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (outputDesign ?? flatInputDesign ?? rawDesign) as Design,
      error: dragError ? dragError : !flatInputDesign ? "Loading drag preview…" : selectedPieceIds.length === 0 ? "Select at least one piece to drag." : !outputDesign ? "Loading drag result…" : undefined,
    }),
    [kit, flatInputDesign, selectedPieceIds, vec, designDiff, outputDesign, dragError],
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
