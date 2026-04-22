// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Cluster.stories.tsx
// Specs: Pure UI proxy to nativeFlatDesign. No domain logic. All designs include connections.
// Summary: Flat input design via nativeFlatDesign; diff/output are local story fixtures.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeFlatDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignId = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.id === nakaginCapsuleTowerDesignId) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "cluster-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "cluster-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cluster-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function ClusterFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>([]);

  React.useEffect(() => {
    let cancelled = false;
    void (async () => {
      const flat = await nativeFlatDesign(kit, rawDesign.id, language);
      if (cancelled) return;
      setFlatInputDesign(flat);
      setSelectedPieceIds((rawDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.id));
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  const designDiff = React.useMemo(
    () =>
      selectedPieceIds.length < 2
        ? undefined
        : {
            pieces: {
              added: [{ id: `storybook-group-${selectedPieceIds.length}`, name: `Storybook group (${selectedPieceIds.length})` }],
              updated: selectedPieceIds.map((id) => ({ piece: { id }, diff: {} })),
            },
          },
    [selectedPieceIds],
  );

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: (flatInputDesign ?? rawDesign) as Design,
      error: !flatInputDesign ? `Loading cluster preview (${language})…` : selectedPieceIds.length < 2 ? "Select at least 2 pieces to cluster." : undefined,
    }),
    [kit, flatInputDesign, selectedPieceIds, designDiff, language],
  );

  return <AlgorithmApp id="cluster" label="Cluster" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Cluster",
  component: ClusterFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <ClusterFrame /> };
