// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Pure UI proxy to flatDesign + deletePieces. No domain logic. All designs include connections.
// Summary: Flat input design via flatDesign (semio/rs WASM); deletePieces returns diff; Design.applyDiff computes output.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff, DesignPlain } from "@semio/react/host";
import { Design as DesignEntity } from "@semio/react/host";
import type { Meta, StoryObj } from "@storybook/react-vite";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { deletePieces, flatDesign } from "../../index";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignId = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.id === nakaginCapsuleTowerDesignId) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DeleteFrame() {
  const kit = metabolismKit as any;
  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>([]);
  const [selectedConnectionIds, setSelectedConnectionIds] = React.useState<string[]>([]);
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlatInputDesign(null);
    setDesignDiff(undefined);
    void (async () => {
      const flat = await flatDesign(kit, rawDesign.id);
      if (cancelled) return;
      setFlatInputDesign(flat);
      setSelectedPieceIds((prev) => {
        const pieceIds = new Set<string>((rawDesign?.pieces ?? []).map((p: any) => p.id));
        const filtered = prev.filter((g) => pieceIds.has(g));
        if (filtered.length > 0) return filtered;
        return (rawDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.id);
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
      if (selectedPieceIds.length === 0 && selectedConnectionIds.length === 0) {
        if (!cancelled) setDesignDiff(undefined);
        return;
      }
      setDesignDiff(undefined);
      const diffRes = await deletePieces(kit, flatInputDesign, selectedPieceIds, selectedConnectionIds);
      if (!cancelled) setDesignDiff(diffRes.ok ? diffRes.diff : undefined);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, flatInputDesign, selectedPieceIds, selectedConnectionIds]);

  const outputDesign = React.useMemo(() => {
    if (!flatInputDesign) return rawDesign as Design;
    if (!designDiff) return flatInputDesign as Design;
    const plain = (flatInputDesign as DesignEntity).toPlain?.() ?? (JSON.parse(JSON.stringify(flatInputDesign)) as DesignPlain);
    const next = new DesignEntity(plain);
    next.applyDiff(designDiff);
    return next;
  }, [designDiff, flatInputDesign]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      selectedConnectionIds,
      onSelectedConnectionIdsChange: setSelectedConnectionIds,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: outputDesign as Design,
      error: !flatInputDesign
        ? "Loading delete preview…"
        : selectedPieceIds.length === 0 && selectedConnectionIds.length === 0
          ? "Select at least one piece or connection to delete."
          : !designDiff
            ? "Loading delete result…"
            : undefined,
    }),
    [kit, flatInputDesign, selectedPieceIds, selectedConnectionIds, designDiff, outputDesign],
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
