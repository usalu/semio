// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: Pure UI proxy to nativeFlatDesign + nativeFlattenedDesign + nativeFlattenDesign. Output window applies the full flatten diff (no connections).
// Summary: nativeFlatDesign feeds the diff window base (connections preserved); nativeFlattenedDesign feeds the output (full diff applied, connections removed); nativeFlattenDesign provides the diff itself.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeFlatDesign, nativeFlattenDesign, nativeFlattenedDesign, type NativeAlgorithmLanguage } from "../../index";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignId = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.id === nakaginCapsuleTowerDesignId) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "flatten-input", kind: WindowKind.DESIGN_INPUT, label: "Input" },
  { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function FlattenFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flatDesign, setFlatDesign] = React.useState<Design | null>(null);
  const [flattenedDesign, setFlattenedDesign] = React.useState<Design | null>(null);
  const [flattenDiff, setFlattenDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlatDesign(null);
    setFlattenedDesign(null);
    setFlattenDiff(undefined);
    void (async () => {
      const [flatResult, flattenedResult, flattenResult] = await Promise.all([
        nativeFlatDesign(kit, rawDesign.id, language),
        nativeFlattenedDesign(kit, rawDesign.id, language),
        nativeFlattenDesign(kit, rawDesign.id, language),
      ]);
      if (cancelled) return;
      setFlatDesign(flatResult);
      setFlattenedDesign(flattenedResult);
      setFlattenDiff(flattenResult.ok ? flattenResult.diff.forward : undefined);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatDesign ?? rawDesign) as Design,
      selectedPieceIds: [],
      designDiff: flattenDiff,
      diffDesign: (flatDesign ?? rawDesign) as Design,
      outputDesign: (flattenedDesign ?? flatDesign ?? rawDesign) as Design,
      error: !flatDesign || !flattenedDesign || !flattenDiff ? `Loading flatten (${language})…` : undefined,
    }),
    [kit, flatDesign, flattenedDesign, flattenDiff, language],
  );

  return <AlgorithmApp id="flatten" label="Flatten" windows={WINDOWS} context={context} className="h-full w-full" />;
}

const meta = {
  title: "semio-algorithms/Flatten",
  component: FlattenFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <FlattenFrame /> };
