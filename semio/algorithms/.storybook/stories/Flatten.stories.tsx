// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: Uses the AlgorithmApp shell with DESIGN_INPUT, DESIGN_DIFF_OUTPUT, DESIGN_OUTPUT windows.
// Summary: Flatten story using nativeFlattenDesign with the Storybook language toolbar.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

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
  { id: "flatten-input", kind: WindowKind.DESIGN_INPUT, label: "Input" },
  { id: "flatten-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "flatten-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function FlattenFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [change, setChange] = React.useState<DesignChange | null>(null);
  const [flatDesign, setFlatDesign] = React.useState<any | null>(null);

  React.useEffect(() => {
    let cancelled = false;
    setChange(null);
    setFlatDesign(null);
    void nativeFlattenDesign(kit, rawDesign.guid, language).then((res) => {
      if (cancelled) return;
      if (!res.ok) {
        setChange(null);
        setFlatDesign(null);
        return;
      }
      setChange(res.change);
      setFlatDesign(applyDesignDiff(rawDesign, res.change.forward) as any);
    });
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: rawDesign,
      selectedPieceGuids: [],
      designDiff: change?.forward,
      diffDesign: rawDesign,
      outputDesign: (flatDesign ?? rawDesign) as any,
      error: !change || !flatDesign ? `Loading flatten (${language})…` : undefined,
    }),
    [kit, flatDesign, change, language],
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
