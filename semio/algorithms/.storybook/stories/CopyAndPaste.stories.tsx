// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/CopyAndPaste.stories.tsx
// Specs: Copy/paste proxies through native adapters and the native copy/paste selectors.
// Summary: Copy/paste Storybook shells with native adapters, source selection, and diff/output previews.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, DesignDiff, DesignPlain, PasteDesignAnchoringKind } from "@semio/js";
import { Design as DesignEntity, Kit, type Kit as KitType } from "@semio/js";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { SemioDiagram } from "@semio/ui";
import { AlgorithmApp, WindowKind, type AlgorithmContextValue, type AlgorithmWindowDef, type VecValue } from "../../index";
import { nativeCopyDesign, nativeFlatDesign, nativePasteDesign, type NativeAlgorithmLanguage } from "../../nativeAlgorithmAdapter";
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

const PASTE_ANCHOR_LABELS: Record<PasteDesignAnchoringKind, string> = {
  original: "Original",
  middle: "Middle (bbox)",
  centroid: "Centroid",
  bottomLeft: "Bottom left",
  bottomRight: "Bottom right",
  topLeft: "Top left",
  topRight: "Top right",
};

const PASTE_DESIGN_ANCHOR_OPTIONS: readonly { anchoringKind: PasteDesignAnchoringKind; label: string }[] = Kit.pasteDesignAnchoringKinds.map((anchoringKind) => ({
  anchoringKind,
  label: PASTE_ANCHOR_LABELS[anchoringKind],
}));

interface CopyPasteWindowContextValue {
  flatPasteTargetDesign: Design | undefined;
  pasteAnchoring: PasteDesignAnchoringKind;
  onPasteAnchoringChange: (next: PasteDesignAnchoringKind) => void;
}

const CopyPasteWindowContext = React.createContext<CopyPasteWindowContextValue>({
  flatPasteTargetDesign: undefined,
  pasteAnchoring: "original",
  onPasteAnchoringChange: () => {},
});

const TargetDesignWindow: React.FC = () => {
  const { flatPasteTargetDesign } = React.useContext(CopyPasteWindowContext);
  return <SemioDiagram design={(flatPasteTargetDesign ?? pasteTargetDesign) as Design} diffEnabled={false} zoomTarget="design" selectionEnabled={false} panEnabled={true} zoomEnabled={true} />;
};

const PasteAnchoringWindow: React.FC = () => {
  const { pasteAnchoring, onPasteAnchoringChange } = React.useContext(CopyPasteWindowContext);
  return (
    <div className="flex h-full min-h-0 flex-col gap-2 overflow-auto border-border/40 border-t p-3">
      <label className="text-xs font-medium text-muted-foreground" htmlFor="semio-algorithms-paste-anchoring">
        Paste anchoring
      </label>
      <select
        id="semio-algorithms-paste-anchoring"
        className="border-input bg-background text-foreground w-full max-w-full rounded-md border px-2 py-1.5 text-sm"
        value={pasteAnchoring}
        onChange={(e) => onPasteAnchoringChange(e.target.value as PasteDesignAnchoringKind)}
      >
        {PASTE_DESIGN_ANCHOR_OPTIONS.map((o) => (
          <option key={o.anchoringKind} value={o.anchoringKind}>
            {o.label}
          </option>
        ))}
      </select>
    </div>
  );
};

const WINDOWS_WITHOUT: AlgorithmWindowDef[] = [
  { id: "cp-src", kind: WindowKind.SELECTION_INPUT, label: "Source Selection" },
  { id: "cp-tgt", kind: WindowKind.DESIGN_INPUT, label: "Target Design", component: TargetDesignWindow },
  { id: "cp-anchor", kind: WindowKind.DESIGN_INPUT, label: "Paste anchoring", component: PasteAnchoringWindow },
  { id: "cp-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cp-out", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

const WINDOWS_WITH: AlgorithmWindowDef[] = [
  { id: "cp-src", kind: WindowKind.SELECTION_INPUT, label: "Source Selection" },
  { id: "cp-tgt", kind: WindowKind.DESIGN_INPUT, label: "Target Design", component: TargetDesignWindow },
  { id: "cp-anchor", kind: WindowKind.DESIGN_INPUT, label: "Paste anchoring", component: PasteAnchoringWindow },
  { id: "cp-vec", kind: WindowKind.VEC_INPUT, label: "Coordinate" },
  { id: "cp-diff", kind: WindowKind.DESIGN_DIFF_OUTPUT, label: "Diff" },
  { id: "cp-out", kind: WindowKind.DESIGN_OUTPUT, label: "Output" },
];

const LAYOUT_WITHOUT = {
  root: {
    kind: "row" as const,
    children: [
      {
        kind: "column" as const,
        size: 34,
        children: [
          { kind: "stack" as const, size: 40, children: [{ kind: "window" as const, windowKindId: "cp-src", title: "Source Selection" }] },
          { kind: "stack" as const, size: 35, children: [{ kind: "window" as const, windowKindId: "cp-tgt", title: "Target Design" }] },
          { kind: "stack" as const, size: 25, children: [{ kind: "window" as const, windowKindId: "cp-anchor", title: "Paste anchoring" }] },
        ],
      },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "cp-diff", title: "Diff" }] },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "cp-out", title: "Output" }] },
    ],
  },
};

const LAYOUT_WITH = {
  root: {
    kind: "row" as const,
    children: [
      {
        kind: "column" as const,
        size: 34,
        children: [
          { kind: "stack" as const, size: 36, children: [{ kind: "window" as const, windowKindId: "cp-src", title: "Source Selection" }] },
          { kind: "stack" as const, size: 24, children: [{ kind: "window" as const, windowKindId: "cp-tgt", title: "Target Design" }] },
          { kind: "stack" as const, size: 18, children: [{ kind: "window" as const, windowKindId: "cp-anchor", title: "Paste anchoring" }] },
          { kind: "stack" as const, size: 22, children: [{ kind: "window" as const, windowKindId: "cp-vec", title: "Coordinate" }] },
        ],
      },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "cp-diff", title: "Diff" }] },
      { kind: "stack" as const, size: 33, children: [{ kind: "window" as const, windowKindId: "cp-out", title: "Output" }] },
    ],
  },
};

export type CopyPasteStoryMode = "without" | "with";

function CopyAndPasteFrame({ mode }: { mode: CopyPasteStoryMode }) {
  const language = useAlgorithmLanguage() as NativeAlgorithmLanguage;
  const kit = metabolismKit as any;
  const kitWithPasteTarget = React.useMemo(() => ({ ...kit, designs: [...(kit.designs ?? []), pasteTargetDesign] }), [kit]);
  const [flatSourceDesign, setFlatSourceDesign] = React.useState<Design | null>(null);
  const [flatPasteTargetDesign, setFlatPasteTargetDesign] = React.useState<Design | null>(null);
  const [layoutReady, setLayoutReady] = React.useState(false);
  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>(selectionPieceGuids);
  const [selectedConnectionGuids, setSelectedConnectionGuids] = React.useState<string[]>(selectionConnectionGuids);
  const [vec, setVec] = React.useState<VecValue>({ u: 10, v: 10 });
  const [pasteAnchoring, setPasteAnchoring] = React.useState<PasteDesignAnchoringKind>("original");
  const [designDiff, setDesignDiff] = React.useState<DesignDiff | undefined>(undefined);

  const windows = mode === "with" ? WINDOWS_WITH : WINDOWS_WITHOUT;
  const defaultLayout = mode === "with" ? LAYOUT_WITH : LAYOUT_WITHOUT;

  React.useEffect(() => {
    let cancelled = false;
    setFlatSourceDesign(null);
    setFlatPasteTargetDesign(null);
    setLayoutReady(false);
    setDesignDiff(undefined);
    void (async () => {
      const [flatSrc, flatTgt] = await Promise.all([nativeFlatDesign(kit, rawDesign.guid, language), nativeFlatDesign(kitWithPasteTarget, pasteTargetDesign.guid, language)]);
      if (cancelled) return;
      setFlatSourceDesign(flatSrc);
      setFlatPasteTargetDesign(flatTgt);
      setLayoutReady(flatSrc !== null && flatTgt !== null);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, kitWithPasteTarget, language]);

  React.useEffect(() => {
    if (!layoutReady || !flatSourceDesign) return;
    let cancelled = false;
    void (async () => {
      if (selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0) {
        if (!cancelled) setDesignDiff(undefined);
        return;
      }
      setDesignDiff(undefined);
      const copyRes = await nativeCopyDesign(kit, flatSourceDesign, selectedPieceGuids, selectedConnectionGuids, language);
      if (cancelled) return;
      if (!copyRes.ok) return;
      const copied = copyRes.diff;
      const coordinate = mode === "with" ? { u: vec.u, v: vec.v } : undefined;
      const diff = await nativePasteDesign(kit, copied, pasteTargetDesign, pasteAnchoring, coordinate, language);
      if (!cancelled) setDesignDiff(diff);
    })();
    return () => {
      cancelled = true;
    };
  }, [kit, flatSourceDesign, layoutReady, selectedPieceGuids, selectedConnectionGuids, language, mode, pasteAnchoring, mode === "with" ? vec.u : 0, mode === "with" ? vec.v : 0]);

  const outputDesign = React.useMemo(() => {
    if (!flatPasteTargetDesign) return pasteTargetDesign as Design;
    if (!designDiff) return flatPasteTargetDesign as Design;
    const k = Kit.ensure(kitWithPasteTarget as KitType);
    const plain = (flatPasteTargetDesign as DesignEntity).toPlain?.() ?? (JSON.parse(JSON.stringify(flatPasteTargetDesign)) as DesignPlain);
    const next = new DesignEntity(plain, k);
    next.applyDiff(designDiff);
    return next;
  }, [designDiff, flatPasteTargetDesign, kitWithPasteTarget]);

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (flatSourceDesign ?? rawDesign) as Design,
      ...(mode === "with"
        ? {
            vec,
            onVecChange: setVec,
            vecMin: { u: -50, v: -50 },
            vecMax: { u: 50, v: 50 },
          }
        : {}),
      selectedPieceGuids,
      onSelectedPieceGuidsChange: setSelectedPieceGuids,
      selectedConnectionGuids,
      onSelectedConnectionGuidsChange: setSelectedConnectionGuids,
      designDiff,
      diffDesign: (flatPasteTargetDesign ?? pasteTargetDesign) as Design,
      outputDesign,
      error: !layoutReady
        ? `Loading copy & paste preview (${language})…`
        : selectedPieceGuids.length === 0 && selectedConnectionGuids.length === 0
          ? "Select at least one piece or connection to copy."
          : !designDiff
            ? `Loading paste result (${language})…`
            : undefined,
    }),
    [kit, flatSourceDesign, flatPasteTargetDesign, layoutReady, selectedPieceGuids, selectedConnectionGuids, vec, designDiff, outputDesign, language, mode],
  );

  const windowContextValue = React.useMemo<CopyPasteWindowContextValue>(() => ({ flatPasteTargetDesign: flatPasteTargetDesign ?? undefined, pasteAnchoring, onPasteAnchoringChange: setPasteAnchoring }), [flatPasteTargetDesign, pasteAnchoring]);

  return (
    <CopyPasteWindowContext.Provider value={windowContextValue}>
      <AlgorithmApp id={`copypaste-${mode}`} label="Copy & Paste" windows={windows} defaultLayout={defaultLayout} context={context} className="h-full w-full" />
    </CopyPasteWindowContext.Provider>
  );
}

// #endregion CopyAndPaste

const meta = {
  title: "semio-algorithms/CopyAndPaste",
  component: CopyAndPasteFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof CopyAndPasteFrame>;

export default meta;

type Story = StoryObj<typeof meta>;

export const WithoutCoordinate: Story = {
  args: { mode: "without" },
  render: (args) => <CopyAndPasteFrame {...args} />,
};

export const WithCoordinate: Story = {
  args: { mode: "with" },
  render: (args) => <CopyAndPasteFrame {...args} />,
};
