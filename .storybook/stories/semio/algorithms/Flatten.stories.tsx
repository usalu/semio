// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: Pure UI proxy to flatDesign + flattenedDesign + flattenDesign. Output window applies the full flatten diff (no connections).
// Summary: flatDesign feeds the diff window base (connections preserved); flattenedDesign feeds the output (full diff applied, connections removed); flattenDesign provides the diff itself.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/algorithms";
import { flatDesign, flattenDesign, flattenedDesign } from "@semio/algorithms";

import metabolismKit from "@semio/assets/fixtures/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignId = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.id === nakaginCapsuleTowerDesignId) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "flatten-input", kind: WindowKind.DESIGN_INPUT, label: "Input" },
  { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function FlattenFrame() {
  const kit = metabolismKit as any;
  const [flatPreview, setFlatPreview] = React.useState<Design | null>(null);
  const [flattenedPreview, setFlattenedPreview] = React.useState<Design | null>(null);
  const [flattenDiff, setFlattenDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlatPreview(null);
    setFlattenedPreview(null);
    setFlattenDiff(undefined);
    void (async () => {
      const [flatResult, flattenedResult, flattenResult] = await Promise.all([
        flatDesign(kit, rawDesign.id),
        flattenedDesign(kit, rawDesign.id),
        flattenDesign(kit, rawDesign.id),
      ]);
      if (cancelled) return;
      setFlatPreview(flatResult);
      setFlattenedPreview(flattenedResult);
      setFlattenDiff(flattenResult.ok ? flattenResult.diff.forward : undefined);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatPreview ?? rawDesign) as Design,
      selectedPieceIds: [],
      designDiff: flattenDiff,
      diffDesign: (flatPreview ?? rawDesign) as Design,
      outputDesign: (flattenedPreview ?? flatPreview ?? rawDesign) as Design,
      error: !flatPreview || !flattenedPreview || !flattenDiff ? "Loading flatten…" : undefined,
    }),
    [kit, flatPreview, flattenedPreview, flattenDiff],
  );

  return <AlgorithmApp id="flatten" label="Flatten" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio/algorithms/Flatten",
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
