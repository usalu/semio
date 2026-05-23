// #region 🧲Header
// 💻 semio/algorithms/.storybook/stories/CopyAndPaste.stories.tsx
// Specs: Copy/paste proxies via shared story hooks and kit runners.
// Summary: IPO copy/paste — source selection, target preview, paste diff/output.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Design, PasteDesignAnchoringKind } from "@semio/react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { within } from "storybook/test";
import * as React from "react";

import { SemioDiagram } from "@semio/ui";
import {
  AlgorithmApp,
  Kit,
  NAKAGIN_CAPSULE_TOWER_DESIGN_ID,
  WindowKind,
  designFromKit,
  mergeKitWithStoryDesign,
  nakaginPasteTargetDesign,
  nakaginStoryCopySelection,
  useCopyPastePreview,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
  type VecValue,
} from "@semio/algorithms";

import metabolismKit from "@semio/assets/fixtures/metabolism.kit.semio.json";

const rawDesign = designFromKit(metabolismKit, NAKAGIN_CAPSULE_TOWER_DESIGN_ID)!;
const pasteTargetDesign = nakaginPasteTargetDesign();
const { pieceIds: selectionPieceIds, connectionIds: selectionConnectionIds } = nakaginStoryCopySelection();

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
        {Kit.pasteAnchoringOptions.map((o) => (
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
  ...WINDOWS_WITHOUT.slice(0, 3),
  { id: "cp-vec", kind: WindowKind.VEC_INPUT, label: "Coordinate" },
  ...WINDOWS_WITHOUT.slice(3),
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
  const kit = metabolismKit;
  const kitWithPasteTarget = React.useMemo(() => mergeKitWithStoryDesign(kit, pasteTargetDesign), [kit]);
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>(selectionPieceIds);
  const [selectedConnectionIds, setSelectedConnectionIds] = React.useState<string[]>(selectionConnectionIds);
  const [vec, setVec] = React.useState<VecValue>({ u: 10, v: 10 });
  const [pasteAnchoring, setPasteAnchoring] = React.useState<PasteDesignAnchoringKind>("original");

  const preview = useCopyPastePreview({
    kit,
    kitWithTarget: kitWithPasteTarget,
    sourceDesignId: String(rawDesign.id ?? NAKAGIN_CAPSULE_TOWER_DESIGN_ID),
    targetDesignId: pasteTargetDesign.id,
    pasteTarget: pasteTargetDesign,
    selectedPieceIds,
    selectedConnectionIds,
    mode,
    vec,
    pasteAnchoring,
  });

  const context: AlgorithmContextValue = React.useMemo(
    () => ({
      kit,
      design: (preview.source.flatInputDesign ?? rawDesign) as Design,
      diagramLayoutDiff: preview.source.diagramLayoutDiff,
      ...(mode === "with" ? { vec, onVecChange: setVec, vecMin: { u: -50, v: -50 }, vecMax: { u: 50, v: 50 } } : {}),
      selectedPieceIds,
      onSelectedPieceIdsChange: setSelectedPieceIds,
      selectedConnectionIds,
      onSelectedConnectionIdsChange: setSelectedConnectionIds,
      designDiff: preview.designDiff,
      diffDesign: (preview.target.flatInputDesign ?? pasteTargetDesign) as Design,
      outputDesign: preview.outputDesign as Design,
      error: preview.error ?? (preview.loading ? "Loading copy & paste preview…" : !preview.hasSelection ? "Select at least one piece or connection to copy." : preview.runLoading ? "Loading paste result…" : undefined),
    }),
    [kit, preview, selectedPieceIds, selectedConnectionIds, vec, mode],
  );

  const windowContextValue = React.useMemo<CopyPasteWindowContextValue>(
    () => ({ flatPasteTargetDesign: preview.target.flatInputDesign ?? undefined, pasteAnchoring, onPasteAnchoringChange: setPasteAnchoring }),
    [preview.target.flatInputDesign, pasteAnchoring],
  );

  return (
    <CopyPasteWindowContext.Provider value={windowContextValue}>
      <AlgorithmApp id={`copypaste-${mode}`} label="Copy & Paste" windows={mode === "with" ? WINDOWS_WITH : WINDOWS_WITHOUT} defaultLayout={mode === "with" ? LAYOUT_WITH : LAYOUT_WITHOUT} context={context} className="h-full w-full" />
    </CopyPasteWindowContext.Provider>
  );
}

const meta = {
  title: "semio/algorithms/CopyAndPaste",
  component: CopyAndPasteFrame,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof CopyAndPasteFrame>;

export default meta;

type Story = StoryObj<typeof meta>;

const playCopyPaste = async ({ canvasElement }: { canvasElement: HTMLElement }) => {
  await within(canvasElement).findByText(/Copy\s*&\s*Paste/i, { timeout: 120_000 });
};

export const WithoutCoordinate: Story = {
  args: { mode: "without" },
  render: (args) => <CopyAndPasteFrame {...args} />,
  play: playCopyPaste,
};

export const WithCoordinate: Story = {
  args: { mode: "with" },
  render: (args) => <CopyAndPasteFrame {...args} />,
  play: playCopyPaste,
};
