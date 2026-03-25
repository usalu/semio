// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Cluster.stories.tsx
// Specs: Uses the AlgorithmApp shell with PIECES_SELECTION_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Cluster story using nativeFlattenDesign with the Storybook language toolbar.
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
  { id: "cluster-input", kind: WindowKind.PIECES_SELECTION_INPUT, label: "Input" },
  { id: "cluster-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cluster-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function ClusterFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);

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

  const designDiff = React.useMemo(
    () =>
      selectedPieceGuids.length < 2
        ? undefined
        : {
            pieces: {
              added: [{ guid: `cluster-${selectedPieceGuids.length}`, name: `Clustered (${selectedPieceGuids.length} pieces)` }],
              updated: selectedPieceGuids.map((guid) => ({ piece: { guid }, diff: {} })),
            },
          },
    [selectedPieceGuids],
  );

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign ?? rawDesign,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      designDiff,
      diffDesign: baseDesign ?? rawDesign,
      outputDesign: baseDesign ?? rawDesign,
      error: !flattenChange || !baseDesign ? `Loading cluster preview (${language})…` : selectedPieceGuids.length < 2 ? "Select at least 2 pieces to cluster." : undefined,
    }),
    [kit, baseDesign, selectedPieceGuids, designDiff, flattenChange, language],
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
