// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: Uses AlgorithmApp with DESIGN_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: IPO story for Design Flatten using the standardized AlgorithmApp shell.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, type AlgorithmContextValue, type AlgorithmWindowDef } from "@semio/ui";
import { WindowKind } from "@elements/ui";
import { applyDesignDiff, findDesignInKit, flattenDesign, type Design, type DesignDiff, type Kit } from "@semio/js";
import { AlgorithmLanguage, useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "flatten-input", kind: WindowKind.DESIGN_INPUT, label: "Input" },
  { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

const DEFAULT_LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "flatten-input", title: "Input" }] },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "flatten-diff", title: "Diff" }] },
      { kind: "stack" as const, size: 34, children: [{ kind: "window" as const, windowKindId: "flatten-output", title: "Output" }] },
    ],
  },
};

function FlattenFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as unknown as Kit;
  const baseDesign = React.useMemo(() => findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design, [kit]);

  const { designDiff, outputKit, error } = React.useMemo(() => {
    try {
      const change = flattenDesign(kit, nakaginCapsuleTowerDesignGuid);
      const diff = change.forward as DesignDiff;
      const outDesign = applyDesignDiff(baseDesign, diff);
      return { designDiff: diff, outputKit: { ...kit, designs: (kit.designs ?? []).map((d) => (d.guid === outDesign.guid ? outDesign : d)) }, error: undefined };
    } catch (e: any) {
      return { designDiff: undefined, outputKit: kit, error: String(e?.message ?? e) };
    }
  }, [baseDesign, kit, language]);

  const context: AlgorithmContextValue = {
    kit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    selectedPieceGuids: [],
    designDiff,
    outputKit,
    outputDesignGuid: nakaginCapsuleTowerDesignGuid,
    error,
  };

  return <AlgorithmApp id="flatten" label="Flatten" windows={WINDOWS} defaultLayout={DEFAULT_LAYOUT} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Design/Flatten",
  parameters: { layout: "padded" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <FlattenFrame /> };
