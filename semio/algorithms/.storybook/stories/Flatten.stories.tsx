// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: Uses the AlgorithmApp shell with DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Flatten story using real Diagram-based algorithm windows from @semio/ui.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { applyDesignDiff, flattenDesign } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef, WindowKind } from "../../index";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;
const flattenChange = flattenDesign(metabolismKit as any, rawDesign.guid);
const baseDesign = applyDesignDiff(rawDesign, { pieces: flattenChange.forward.pieces }) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function FlattenFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as any;

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign,
      selectedPieceGuids: [],
      designDiff: {
        pieces: { updated: (baseDesign?.pieces ?? []).slice(0, 6).map((piece: any) => ({ piece: { guid: piece.guid }, note: `flattened-${language}` })) },
        connections: { updated: [] },
      },
      outputDesign: baseDesign,
    }),
    [kit, language],
  );

  return <AlgorithmApp id="flatten" label="Flatten" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Flatten",
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <FlattenFrame /> };
