// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Cluster.stories.tsx
// Specs: IPO (Input/Process/Output) story with GoldenLayout for Design Cluster.
// Summary: Shows base design, clustered diff, and clustered output design.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { SemioDiagram as Diagram, PieceSelection, UI, type BreadcrumbItemData, Section, Card } from "@semio/ui";
import { applyDesignDiff, createClusteredDesign, findDesignInKit, replaceClusterWithDesign, type Design, type DesignDiff, type Kit } from "@semio/js";
import { AlgorithmLanguage, useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

type IpoState = {
  language: AlgorithmLanguage;
  kit: Kit;
  baseDesign: Design;
  selectedPieceGuids: string[];
  designDiff?: DesignDiff;
  diffKit: Kit;
  outputKit: Kit;
  outputDesign: Design;
  error?: string;
  setSelectedPieceGuids: (next: string[]) => void;
};

const IpoContext = React.createContext<IpoState | null>(null);

function useIpoState(): IpoState {
  const ctx = React.useContext(IpoContext);
  if (!ctx) throw new Error("Cluster IPO stories must be wrapped in IpoContext");
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
  title: "semio-algorithms/Design/Cluster",
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

const IPO_BREADCRUMB: BreadcrumbItemData[] = [{ content: "Design" }, { content: "Cluster" }];

const descriptionText = "Cluster groups selected connected pieces into a new nested Design, replacing the original pieces.";

const requirementsText = [
  "Input: Design and PieceSelection (pieces to cluster)",
  "Process: create a clustered Design and replace selected pieces with a Design reference",
  "Output: Design with clustered reference (and updated external connections)",
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
  const { selectedPieceGuids, error } = useIpoState();
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
            Selected pieces: <span className={selectedPieceGuids.length >= 2 ? "text-foreground" : "text-muted-foreground"}>{selectedPieceGuids.length}</span>
          </div>
          {error && <div className="p-2 text-xs text-destructive font-mono rounded-md bg-destructive/5 border border-destructive/20">{error}</div>}
        </div>
      </Section>
    </div>
  );
}

function InputsPane() {
  const { kit, baseDesign, selectedPieceGuids, setSelectedPieceGuids } = useIpoState();

  return (
    <div className="h-full flex flex-col gap-2">
      <Section title="Input">
        <div className="h-96 w-full rounded-md border border-element bg-card overflow-hidden">
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
  const { diffKit, baseDesign, designDiff, error } = useIpoState();
  return (
    <div className="h-full flex flex-col gap-2">
      <Section title="Diff">
        {error ? (
          <div className="p-2 text-sm text-destructive font-mono">{error}</div>
        ) : (
          <div className="h-96 w-full rounded-md border border-element bg-card overflow-hidden">
            <Diagram kit={diffKit} designGuid={baseDesign.guid} designDiff={designDiff} diffEnabled={true} selectionEnabled={false} />
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
  description: "ipo-cluster-description",
  requirements: "ipo-cluster-requirements",
  inputs: "ipo-cluster-inputs",
  diffs: "ipo-cluster-diffs",
  outputs: "ipo-cluster-outputs",
  descriptionTitle: "Description",
  requirementsTitle: "Requirements",
  inputsTitle: "Inputs",
  diffsTitle: "Diffs",
  outputsTitle: "Outputs",
};

function IpoFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as unknown as Kit;

  const baseDesign = React.useMemo(() => findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design, [kit]);

  const [selectedPieceGuids, setSelectedPieceGuids] = React.useState<string[]>([]);

  React.useEffect(() => {
    if (selectedPieceGuids.length > 0) return;
    const defaultPieces = (baseDesign.pieces ?? []).slice(0, 3).map((p) => p.guid);
    setSelectedPieceGuids(defaultPieces);
  }, [baseDesign.pieces, selectedPieceGuids.length]);

  const { designDiff, diffKit, outputKit, outputDesign, error } = React.useMemo(() => {
    if (selectedPieceGuids.length < 2) {
      return {
        designDiff: undefined,
        diffKit: kit,
        outputKit: kit,
        outputDesign: baseDesign,
        error: "Select at least 2 pieces to cluster.",
      };
    }

    try {
      const clusteredName = `Clustered design (${selectedPieceGuids.length} pieces)`;
      const { clusteredDesign, externalConnections } = createClusteredDesign(baseDesign, selectedPieceGuids, clusteredName);
      const change = replaceClusterWithDesign(baseDesign, selectedPieceGuids, clusteredDesign, externalConnections);
      const nextDesignDiff = change.forward as DesignDiff;
      const outDesign = applyDesignDiff(baseDesign, nextDesignDiff);

      const designs = kit.designs ?? [];
      const diffDesigns = [...designs, clusteredDesign];
      const diffNextKit: Kit = { ...kit, designs: diffDesigns };

      const outputDesigns = designs.map((d) => (d.guid === outDesign.guid ? outDesign : d));
      outputDesigns.push(clusteredDesign);
      const outputNextKit: Kit = { ...kit, designs: outputDesigns };

      return {
        designDiff: nextDesignDiff,
        diffKit: diffNextKit,
        outputKit: outputNextKit,
        outputDesign: outDesign,
        error: undefined,
      };
    } catch (e: any) {
      return {
        designDiff: undefined,
        diffKit: kit,
        outputKit: kit,
        outputDesign: baseDesign,
        error: String(e?.message ?? e),
      };
    }
  }, [baseDesign, kit, selectedPieceGuids]);

  const state: IpoState = {
    language,
    kit,
    baseDesign,
    selectedPieceGuids,
    designDiff,
    diffKit,
    outputKit,
    outputDesign,
    error,
    setSelectedPieceGuids,
  };

  const windowConfig: any = React.useMemo(() => {
    const { description, requirements, inputs, diffs, outputs } = defaultWindowIds;
    return {
      windowKinds: [
        { id: description, label: "Description", component: DescriptionPane },
        { id: requirements, label: "Requirements", component: RequirementsPane },
        { id: inputs, label: "Inputs", component: InputsPane },
        { id: diffs, label: "Diffs", component: DiffsPane },
        { id: outputs, label: "Outputs", component: OutputsPane },
      ],
      defaultLayout: createIpoDefaultLayout({
        ...defaultWindowIds,
      }),
    };
  }, []);

  const apps = [
    {
      id: "ipo-cluster",
      label: "Cluster",
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
