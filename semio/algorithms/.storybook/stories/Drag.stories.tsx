// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Drag.stories.tsx
// Specs: IPO (Input/Process/Output) story with GoldenLayout for Design Drag.
// Summary: Shows selected pieces, drag diff, and dragged output design.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { SemioDiagram as Diagram, PieceSelection, UI, type BreadcrumbItemData, Section, Card } from "@semio/ui";
import { applyDesignDiff, dragPiecesInDesign, findDesignInKit, type Design, type DesignDiff, type Kit } from "@semio/js";
import { AlgorithmLanguage, useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

type Vec2 = { u: number; v: number };

type IpoState = {
  language: AlgorithmLanguage;
  kit: Kit;
  baseDesign: Design;
  selectedPieceGuids: string[];
  vec: Vec2;
  designDiff?: DesignDiff;
  outputKit: Kit;
  outputDesign: Design;
  error?: string;
  setSelectedPieceGuids: (next: string[]) => void;
  setVec: (next: Vec2) => void;
};

const IpoContext = React.createContext<IpoState | null>(null);

function useIpoState(): IpoState {
  const ctx = React.useContext(IpoContext);
  if (!ctx) throw new Error("Drag IPO stories must be wrapped in IpoContext");
  return ctx;
}

function createIpoDefaultLayout(windowIds: {
  description: string;
  requirements: string;
  inputs: string;
  diffs: string;
  outputs: string;
  descriptionTitle: string;
  requirementsTitle: string;
  inputsTitle: string;
  diffsTitle: string;
  outputsTitle: string;
}): any {
  return {
    root: {
      type: "column",
      content: [
        {
          type: "row",
          size: 25,
          content: [
            {
              type: "stack",
              size: 33.33,
              content: [{ type: "component", componentName: windowIds.description, title: windowIds.descriptionTitle, componentState: {} }],
            },
            {
              type: "stack",
              size: 66.67,
              content: [{ type: "component", componentName: windowIds.requirements, title: windowIds.requirementsTitle, componentState: {} }],
            },
          ],
        },
        {
          type: "row",
          size: 75,
          content: [
            {
              type: "stack",
              size: 33.33,
              content: [{ type: "component", componentName: windowIds.inputs, title: windowIds.inputsTitle, componentState: {} }],
            },
            {
              type: "stack",
              size: 33.33,
              content: [{ type: "component", componentName: windowIds.diffs, title: windowIds.diffsTitle, componentState: {} }],
            },
            {
              type: "stack",
              size: 33.33,
              content: [{ type: "component", componentName: windowIds.outputs, title: windowIds.outputsTitle, componentState: {} }],
            },
          ],
        },
      ],
    },
  };
}

const meta = {
  title: "semio-algorithms/Design/Drag",
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

const IPO_BREADCRUMB: BreadcrumbItemData[] = [{ content: "Design" }, { content: "Drag" }];

const descriptionText = "Drag offsets the selected connected pieces by a vector, updating piece centers and adjusting orphaned connection offsets.";

const requirementsText = [
  "Input: Vec (u,v) and PieceSelection (pieces to drag)",
  "Process: compute a DesignDiff from the selected pieces and apply it to produce output",
  "Output: design where the computed drag diff is applied",
  "Language: selected implementation language (calculation uses available in-browser adapter)",
];

function DescriptionPane() {
  const { language } = useIpoState();
  return (
    <div className="h-full flex flex-col">
      <Section title="Description">
        <div className="space-y-2">
          <p className="text-sm text-muted-foreground">{descriptionText}</p>
          <Card className="p-2">
            <div className="text-xs text-muted-foreground">Implementation</div>
            <div className="font-mono text-sm">{language}</div>
          </Card>
        </div>
      </Section>
    </div>
  );
}

function RequirementsPane() {
  const { vec, selectedPieceGuids, error } = useIpoState();
  return (
    <div className="h-full flex flex-col">
      <Section title="Requirements">
        <div className="space-y-2 text-sm">
          {requirementsText.map((line, idx) => (
            <div key={idx} className="font-mono text-xs">
              - {line}
            </div>
          ))}
          <div className="pt-1 text-xs font-mono">
            Vec: {vec.u.toFixed(2)}, {vec.v.toFixed(2)}
          </div>
          <div className="text-xs font-mono">
            Selected pieces: <span className={selectedPieceGuids.length > 0 ? "text-foreground" : "text-muted-foreground"}>{selectedPieceGuids.length}</span>
          </div>
          {error && <div className="p-2 text-xs text-destructive font-mono rounded-md bg-destructive/5 border border-destructive/20">{error}</div>}
        </div>
      </Section>
    </div>
  );
}

function InputsPane() {
  const { kit, baseDesign, selectedPieceGuids, setSelectedPieceGuids, vec, setVec } = useIpoState();

  return (
    <div className="h-full flex flex-col gap-2">
      <Section title="Input">
        <div className="space-y-2 p-2">
          <div className="flex items-center justify-between gap-2">
            <div className="text-xs font-mono text-muted-foreground">u</div>
            <input className="w-28 rounded-md border border-element bg-background px-2 py-1 text-sm font-mono" type="number" step="0.1" value={vec.u} onChange={(e) => setVec({ ...vec, u: Number(e.target.value) })} />
          </div>
          <div className="flex items-center justify-between gap-2">
            <div className="text-xs font-mono text-muted-foreground">v</div>
            <input className="w-28 rounded-md border border-element bg-background px-2 py-1 text-sm font-mono" type="number" step="0.1" value={vec.v} onChange={(e) => setVec({ ...vec, v: Number(e.target.value) })} />
          </div>
        </div>
        <div className="h-72 w-full rounded-md border border-element bg-card overflow-hidden">
          <PieceSelection
            kit={kit}
            designGuid={baseDesign.guid}
            selection={{ pieceGuids: selectedPieceGuids }}
            onSelectionChange={(next) => setSelectedPieceGuids(next.pieceGuids ?? [])}
            selectionEnabled={true}
            diffEnabled={false}
            panEnabled={false}
            zoomEnabled={true}
          />
        </div>
      </Section>
    </div>
  );
}

function DiffsPane() {
  const { kit, baseDesign, designDiff, error } = useIpoState();
  return (
    <div className="h-full flex flex-col gap-2">
      <Section title="Diff">
        {error ? (
          <div className="p-2 text-sm text-destructive font-mono">{error}</div>
        ) : (
          <div className="h-96 w-full rounded-md border border-element bg-card overflow-hidden">
            <Diagram kit={kit} designGuid={baseDesign.guid} designDiff={designDiff} diffEnabled={true} selectionEnabled={false} />
          </div>
        )}
      </Section>
    </div>
  );
}

function OutputsPane() {
  const { outputKit, outputDesign } = useIpoState();
  return (
    <div className="h-full flex flex-col gap-2">
      <Section title="Output">
        <div className="h-96 w-full rounded-md border border-element bg-card overflow-hidden">
          <Diagram kit={outputKit} designGuid={outputDesign.guid} diffEnabled={false} selectionEnabled={false} />
        </div>
      </Section>
    </div>
  );
}

const defaultWindowIds = {
  description: "ipo-drag-description",
  requirements: "ipo-drag-requirements",
  inputs: "ipo-drag-inputs",
  diffs: "ipo-drag-diffs",
  outputs: "ipo-drag-outputs",
  descriptionTitle: "Description",
  requirementsTitle: "Requirements",
  inputsTitle: "Inputs",
  diffsTitle: "Diffs",
  outputsTitle: "Outputs",
};

function normalizeDragDiffForDiagram(baseDesign: Design, dragDiff: DesignDiff): DesignDiff {
  // dragPiecesInDesign encodes piece center updates as a delta (offset.u/offset.v),
  // but Diagram's applyDesignDiff expects piece center to be an absolute center.
  const basePieces = baseDesign.pieces ?? [];
  const updatedPieces = dragDiff.pieces?.updated as any[] | undefined;
  if (!updatedPieces || updatedPieces.length === 0) return dragDiff;

  const nextUpdated = updatedPieces.map((u) => {
    const pieceGuid = u.piece?.guid;
    const basePiece = basePieces.find((p) => p.guid === pieceGuid);
    const deltaCenter = u.diff?.center;
    if (!basePiece?.center || !deltaCenter || typeof deltaCenter.u !== "number" || typeof deltaCenter.v !== "number") return u;
    return {
      ...u,
      diff: {
        ...u.diff,
        center: {
          u: (basePiece.center.u ?? 0) + deltaCenter.u,
          v: (basePiece.center.v ?? 0) + deltaCenter.v,
        },
      },
    };
  });

  return {
    ...dragDiff,
    pieces: {
      ...dragDiff.pieces,
      updated: nextUpdated,
    },
  };
}

function IpoFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as unknown as Kit;
  const baseDesign = React.useMemo(() => findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design, [kit]);

  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);
  const [vec, setVec] = React.useState<Vec2>({ u: 1, v: -2 });

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    const defaultPieces = (baseDesign.pieces ?? []).slice(0, 3).map((p) => p.guid);
    setSelectedPieceGuids(defaultPieces);
  }, [baseDesign, selectedPieceGuids.length]);

  const { designDiff, outputKit, outputDesign, error } = React.useMemo(() => {
    try {
      if (selectedPieceGuids.length === 0) {
        return { designDiff: undefined, outputKit: kit, outputDesign: baseDesign, error: undefined };
      }

      const piecesDesign = { guid: "", name: "", pieces: selectedPieceGuids.map((g) => ({ guid: g })) } as Design;
      const rawDiff = dragPiecesInDesign(baseDesign, piecesDesign, { u: vec.u, v: vec.v });
      const normalized = normalizeDragDiffForDiagram(baseDesign, rawDiff);

      const outDesign = applyDesignDiff(baseDesign, normalized);
      const nextKit: Kit = {
        ...kit,
        designs: (kit.designs ?? []).map((d) => (d.guid === outDesign.guid ? outDesign : d)),
      };

      return { designDiff: normalized, outputKit: nextKit, outputDesign: outDesign, error: undefined };
    } catch (e: any) {
      return { designDiff: undefined, outputKit: kit, outputDesign: baseDesign, error: String(e?.message ?? e) };
    }
  }, [baseDesign, kit, selectedPieceGuids, vec.u, vec.v]);

  const state: IpoState = {
    language,
    kit,
    baseDesign,
    selectedPieceGuids,
    vec,
    designDiff,
    outputKit,
    outputDesign,
    error,
    setSelectedPieceGuids,
    setVec,
  };

  const windowConfig: any = React.useMemo(() => {
    return {
      windowKinds: [
        { id: defaultWindowIds.description, label: "Description", component: DescriptionPane },
        { id: defaultWindowIds.requirements, label: "Requirements", component: RequirementsPane },
        { id: defaultWindowIds.inputs, label: "Inputs", component: InputsPane },
        { id: defaultWindowIds.diffs, label: "Diffs", component: DiffsPane },
        { id: defaultWindowIds.outputs, label: "Outputs", component: OutputsPane },
      ],
      defaultLayout: createIpoDefaultLayout({ ...defaultWindowIds }),
    };
  }, []);

  const apps = [
    {
      id: "ipo-drag",
      label: "Drag",
      windowConfig,
    },
  ] as any;

  return (
    <IpoContext.Provider value={state}>
      <div className="h-[720px] w-full">
        <UI apps={apps} breadcrumbItems={IPO_BREADCRUMB} defaultAppId={apps[0]!.id} />
      </div>
    </IpoContext.Provider>
  );
}

export const Default: Story = {
  render: () => <IpoFrame />,
};
