// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/CopyAndPaste.stories.tsx
// Specs: One story with SELECTION_INPUT (source), DESIGN_INPUT (target below source), VEC_INPUT (coord), and two DESIGN_DIFF_OUTPUT (WithoutCoord + WithCoord). Copy from flattened NCT, paste into second storey target.
// Summary: CopyAndPaste story using nativeFlattenDesign/nativeCopyDesign/nativePasteDesign with the Storybook language toolbar.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignChange, DesignDiff } from "@semio/js";
import { applyDesignDiff } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { SemioDiagram } from "@semio/ui";
import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef, type VecValue } from "../../index";
import { nativeCopyDesign, nativeFlattenDesign, nativePasteDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
import { useAlgorithmLanguage } from "../withLanguage";

import { NakaginCapsuleTowerCopySelection, NakaginCapsuleTowerPasteDesign } from "../../../assets/index";
import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const rawDesign = ((metabolismKit as any).designs ?? []).find((d: any) => d.guid === nakaginCapsuleTowerDesignGuid) as any;
const pasteTargetDesign = NakaginCapsuleTowerPasteDesign as unknown as Design;

/** Omits t_f1_b_c1 and t_f0→t_f1 link so t_f1 is external-stub in clipboard; includes t_f5/regressions. */
const omitFromCopySelectionPieceGuids = new Set<string>(["31be08e1-e75c-4024-86b4-c3c6d3939fbb"]);
const omitFromCopySelectionConnectionGuids = new Set<string>(["b1ecc6c5-722a-4814-9047-a87222bbaa4d"]);
const selectionPieceGuids = Array.from(
  new Set(
    [...(((NakaginCapsuleTowerCopySelection as any).pieces ?? []) as { guid: string }[]).map((p) => p.guid), "9c1ec7a2-13c2-4d23-b7bd-1efe2663d0a9", "5feebbf8-33d9-41ad-a13a-24c271a1860b"].filter((g) => !omitFromCopySelectionPieceGuids.has(g)),
  ),
) as string[];
const selectionConnectionGuids = Array.from(
  new Set(
    [...(((NakaginCapsuleTowerCopySelection as any).connections ?? []) as { guid: string }[]).map((c) => c.guid), "eb8ce9ce-091c-4495-a651-fa703748dfef", "4d5ff333-d70a-43e1-8b7a-8849c8c91405"].filter(
      (g) => !omitFromCopySelectionConnectionGuids.has(g),
    ),
  ),
) as string[];

// #region CopyAndPaste

interface CopyPasteWindowContextValue {
  flatPasteTargetDesign: Design | null;
  withCoordDiff: DesignDiff | undefined;
}

const CopyPasteWindowContext = React.createContext<CopyPasteWindowContextValue>({ flatPasteTargetDesign: null, withCoordDiff: undefined });

const WithCoordDiffWindow: React.FC = () => {
  const { flatPasteTargetDesign, withCoordDiff } = React.useContext(CopyPasteWindowContext);
  const design = flatPasteTargetDesign ?? pasteTargetDesign;
  return <SemioDiagram design={design} designDiff={withCoordDiff} diffEnabled={true} zoomTarget="design" selectionEnabled={false} panEnabled={true} zoomEnabled={true} />;
};

const TargetDesignWindow: React.FC = () => {
  const { flatPasteTargetDesign } = React.useContext(CopyPasteWindowContext);
  const design = flatPasteTargetDesign ?? pasteTargetDesign;
  return <SemioDiagram design={design} diffEnabled={false} zoomTarget="design" selectionEnabled={false} panEnabled={true} zoomEnabled={true} />;
};

const WINDOWS: AlgorithmWindowDef[] = [
  { id: "copypaste-input", kind: WindowKind.SELECTION_INPUT, label: "Source Selection" },
  { id: "copypaste-target", kind: WindowKind.DESIGN_INPUT, label: "Target Design", component: TargetDesignWindow },
  { id: "copypaste-vec", kind: WindowKind.VEC_INPUT, label: "Coord" },
  { id: "copypaste-without-coord-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "WithoutCoord" },
  { id: "copypaste-with-coord-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "WithCoord", component: WithCoordDiffWindow },
];

const LAYOUT = {
  root: {
    kind: "row" as const,
    children: [
      {
        kind: "column" as const,
        size: 33,
        children: [
          { kind: "stack" as const, size: 50, children: [{ kind: "window" as const, windowKindId: "copypaste-input", title: "Source Selection" }] },
          { kind: "stack" as const, size: 30, children: [{ kind: "window" as const, windowKindId: "copypaste-target", title: "Target Design" }] },
          { kind: "stack" as const, size: 20, children: [{ kind: "window" as const, windowKindId: "copypaste-vec", title: "Coord" }] },
        ],
      },
      {
        kind: "stack" as const,
        size: 33,
        children: [{ kind: "window" as const, windowKindId: "copypaste-without-coord-diff", title: "WithoutCoord" }],
      },
      {
        kind: "stack" as const,
        size: 34,
        children: [{ kind: "window" as const, windowKindId: "copypaste-with-coord-diff", title: "WithCoord" }],
      },
    ],
  },
};

function CopyAndPasteFrame() {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const kitWithPasteTarget = React.useMemo(() => ({ ...kit, designs: [...(kit.designs ?? []), pasteTargetDesign] }), [kit]);
  const [flattenChange, setFlattenChange] = React.useState<DesignChange | null>(null);
  const [baseDesign, setBaseDesign] = React.useState<any | null>(null);
  const [flatPasteTargetDesign, setFlatPasteTargetDesign] = React.useState<Design | null>(null);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>(selectionPieceGuids);
  const [selectedConnectionGuids, setSelectedConnectionGuids] = React.useState<string[]>(selectionConnectionGuids);
  const [vec, setVec] = React.useState<VecValue>({ u: 10, v: 10 });
  const [withoutCoordDiff, setWithoutCoordDiff] = React.useState<DesignDiff | undefined>(undefined);
  const [withCoordDiff, setWithCoordDiff] = React.useState<DesignDiff | undefined>(undefined);

  React.useEffect(() => {
    let cancelled = false;
    setFlattenChange(null);
    setBaseDesign(null);
    setFlatPasteTargetDesign(null);
    setWithoutCoordDiff(undefined);
    setWithCoordDiff(undefined);
    void (async () => {
      const [fc, pastefc] = await Promise.all([nativeFlattenDesign(kit, rawDesign.guid, language), nativeFlattenDesign(kitWithPasteTarget, pasteTargetDesign.guid, language)]);
      if (cancelled) return;
      setFlattenChange(fc);
      const bd = applyDesignDiff(rawDesign, fc.forward) as any;
      setBaseDesign(bd);
      setFlatPasteTargetDesign(applyDesignDiff(pasteTargetDesign, pastefc.forward) as Design);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, kitWithPasteTarget, language]);

  React.useEffect(() => {
    if (!baseDesign) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0) {
        if (!cancelled) {
          setWithoutCoordDiff(undefined);
          setWithCoordDiff(undefined);
        }
        return;
      }
      setWithoutCoordDiff(undefined);
      setWithCoordDiff(undefined);
      const copied = await nativeCopyDesign(kit, baseDesign, selectedPieceGuids, selectedConnectionGuids, language);
      if (cancelled) return;
      const [diffNoCoord, diffWithCoord] = await Promise.all([nativePasteDesign(kit, copied, pasteTargetDesign, "original", undefined, language), nativePasteDesign(kit, copied, pasteTargetDesign, "original", { u: vec.u, v: vec.v }, language)]);
      if (!cancelled) {
        setWithoutCoordDiff(diffNoCoord);
        setWithCoordDiff(diffWithCoord);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, baseDesign, selectedPieceGuids, selectedConnectionGuids, vec, language]);

  const flatOrRawPasteTarget = flatPasteTargetDesign ?? pasteTargetDesign;
  const outputDesign = React.useMemo(() => (withCoordDiff ? (applyDesignDiff(flatOrRawPasteTarget, withCoordDiff) as Design) : flatOrRawPasteTarget), [withCoordDiff, flatOrRawPasteTarget]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: baseDesign ?? rawDesign,
      vec,
      onVecChange: setVec,
      vecMin: { u: -50, v: -50 },
      vecMax: { u: 50, v: 50 },
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      selectedConnectionGuids,
      onSelectedConnectionGuidsChange: setSelectedConnectionGuids,
      designDiff: withoutCoordDiff,
      diffDesign: flatPasteTargetDesign ?? pasteTargetDesign,
      outputDesign,
      error:
        !flattenChange || !baseDesign
          ? `Loading copy & paste preview (${language})…`
          : selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0
            ? "Select at least one piece or connection to copy."
            : !withoutCoordDiff || !withCoordDiff
              ? `Loading paste result (${language})…`
              : undefined,
    }),
    [kit, baseDesign, flatPasteTargetDesign, selectedPieceGuids, selectedConnectionGuids, vec, withoutCoordDiff, withCoordDiff, outputDesign, flattenChange, language],
  );

  const windowContextValue = React.useMemo<CopyPasteWindowContextValue>(() => ({ flatPasteTargetDesign, withCoordDiff }), [flatPasteTargetDesign, withCoordDiff]);

  return (
    <CopyPasteWindowContext.Provider value={windowContextValue}>
      <AlgorithmApp id="copypaste" label="Copy & Paste" windows={WINDOWS} defaultLayout={LAYOUT} context={context} className="h-full w-full" />
    </CopyPasteWindowContext.Provider>
  );
}

// #endregion CopyAndPaste

const meta = {
  title: "semio-algorithms/CopyAndPaste",
  component: CopyAndPasteFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = { render: () => <CopyAndPasteFrame /> };
