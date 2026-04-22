// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/Delete.stories.tsx
// Specs: Pure UI proxy to nativeFlatDesign + nativeDeletePieces. No domain logic. All designs include connections.
// Summary: Flat input design via nativeFlatDesign; nativeDeletePieces returns diff; Design.applyDiff computes output.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff, DesignPlain, Kit as KitPlain } from "@semio/js";
import { Design as DesignEntity, Kit } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef } from "../../index";
import { nativeDeletePieces, nativeFlatDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = (metabolismKit.designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "delete-input", kind: WindowKind.SELECTION_INPUT, label: "Input" },
  { id: "delete-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "delete-output", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

function DeleteFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [selectedConnectionGuids, setSelectedConnectionGuids] = React.useState<string[]>([]);
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlatInputDesign(null);
    setDesignDiff(undefined);
    void (async () => {
      const flat = await nativeFlatDesign(kit, rawDesign.guid, language);
      if (cancelled) return;
      setFlatInputDesign(flat);
      setSelectedPieceGuids((prev) => {
        const pieceGuids = new Set<string>((rawDesign?.pieces ?? []).map((p: any) => p.guid));
        const filtered = prev.filter((g) => pieceGuids.has(g));
        if (filtered.length > 0) return filtered;
        return (rawDesign?.pieces ?? []).slice(0, 3).map((piece: any) => piece.guid);
      });
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, language]);

  React.useEffect(() => {
    if (!flatInputDesign) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0) {
        if (!cancelled) setDesignDiff(undefined);
        return;
      }
      setDesignDiff(undefined);
      const diffRes = await nativeDeletePieces(kit, flatInputDesign, selectedPieceGuids, selectedConnectionGuids, language);
      if (!cancelled) setDesignDiff(diffRes.ok ? diffRes.diff : undefined);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, flatInputDesign, selectedPieceGuids, selectedConnectionGuids, language]);

  const outputDesign = React.useMemo(() => {
    if (!flatInputDesign) return rawDesign as Design;
    if (!designDiff) return flatInputDesign as Design;
    const k = Kit.ensure(kit as KitPlain);
    const plain = (flatInputDesign as DesignEntity).toPlain?.() ?? (JSON.parse(JSON.stringify(flatInputDesign)) as DesignPlain);
    const next = new DesignEntity(plain, k);
    next.applyDiff(designDiff);
    return next;
  }, [designDiff, flatInputDesign, kit]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatInputDesign ?? rawDesign) as Design,
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      selectedConnectionGuids,
      onSelectedConnectionGuidsChange: setSelectedConnectionGuids,
      designDiff,
      diffDesign: (flatInputDesign ?? rawDesign) as Design,
      outputDesign: outputDesign as Design,
      error: !flatInputDesign
        ? `Loading delete preview (${language})…`
        : selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0
          ? "Select at least one piece or connection to delete."
          : !designDiff
            ? `Loading delete result (${language})…`
            : undefined,
    }),
    [kit, flatInputDesign, selectedPieceGuids, selectedConnectionGuids, designDiff, outputDesign, language],
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
