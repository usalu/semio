// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: Pure UI proxy to flatDesign + flattenedDesign + flattenDesign via shared story hooks.
// Summary: IPO flatten board — flat diff base, full flatten output, forward diff panel.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import { AlgorithmApp, NAKAGIN_CAPSULE_TOWER_DESIGN_ID, WindowKind, designFromKit, useFlattenPreview, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/algorithms";

import { MetabolismKit as metabolismKit } from "@semio/assets";

const rawDesign = designFromKit(metabolismKit, NAKAGIN_CAPSULE_TOWER_DESIGN_ID)!;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "flatten-input", kind: WindowKind.DESIGN_INPUT, label: "Input" },
  { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function FlattenFrame() {
  const kit = metabolismKit;
  const { flatPreview, flattenedPreview, flattenDiff, loading } = useFlattenPreview(kit, rawDesign.id);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatPreview ?? rawDesign) as Design,
      selectedPieceIds: [],
      designDiff: flattenDiff,
      diffDesign: (flatPreview ?? rawDesign) as Design,
      outputDesign: (flattenedPreview ?? flatPreview ?? rawDesign) as Design,
      error: loading ? "Loading flatten…" : undefined,
    }),
    [kit, flatPreview, flattenedPreview, flattenDiff, loading],
  );

  return <AlgorithmApp id="flatten" label="Flatten" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "🏘️semio🧪algorithms/Flatten",
  component: FlattenFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <FlattenFrame />,
  play: async ({ canvasElement }) => {
    await within(canvasElement).findByText(/Flatten/i, { timeout: 120_000 });
  },
};
