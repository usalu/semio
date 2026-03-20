// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/Flatten.stories.tsx
// Specs: IPO (Input/Process/Output) story with GoldenLayout for Design Flatten.
// Summary: Shows base design, flatten design diff, and flattened output design.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

import { Diagram, UI, type BreadcrumbItemData, Section, Card } from "@semio/ui";
import { applyDesignDiff, findDesignInKit, flattenDesign, type Design, type DesignDiff, type Kit } from "@semio/js";
import { AlgorithmLanguage, useAlgorithmLanguage } from "../withLanguage";

import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

type IpoState = {
  language: AlgorithmLanguage;
  kit: Kit;
  baseDesign: Design;
  designDiff?: DesignDiff;
  outputKit: Kit;
  outputDesign: Design;
  error?: string;
};

const IpoContext = React.createContext<IpoState | null>(null);

function useIpoState(): IpoState {
  const ctx = React.useContext(IpoContext);
  if (!ctx) throw new Error("Flatten IPO stories must be wrapped in IpoContext");
  return ctx;
}

function createIpoWindowConfig(windowIds: {
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
}): { windowConfig: any; defaultLayout: any } {
  const defaultLayout = {
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

  return { windowConfig: undefined, defaultLayout };
}

const meta = {
  title: "semio-algorithms/Design/Flatten",
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

const IPO_BREADCRUMB: BreadcrumbItemData[] = [{ content: "Design" }, { content: "Flatten" }];

const descriptionText = "Flatten turns a nested Design hierarchy into a flat representation, making connections and piece planes easier to reason about.";

const requirementsText = [
  "Input: Design (Nakagin Capsule Tower)",
  "Process: flatten nested Design pieces and update planes/centers",
  "Output: flattened Design",
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
  return (
    <div className="h-full flex flex-col">
      <Section title="Requirements">
        <div className="space-y-2 text-sm">
          {requirementsText.map((line, idx) => (
            <div key={idx} className="font-mono text-xs">
              - {line}
            </div>
          ))}
        </div>
      </Section>
    </div>
  );
}

function InputsPane() {
  const { kit, baseDesign } = useIpoState();
  return (
    <div className="h-full flex flex-col gap-2">
      <Section title="Input">
        <div className="h-96 w-full rounded-md border border-element bg-card overflow-hidden">
          <Diagram kit={kit} designGuid={baseDesign.guid} selectionEnabled={false} diffEnabled={false} />
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
  description: "ipo-flatten-description",
  requirements: "ipo-flatten-requirements",
  inputs: "ipo-flatten-inputs",
  diffs: "ipo-flatten-diffs",
  outputs: "ipo-flatten-outputs",
  descriptionTitle: "Description",
  requirementsTitle: "Requirements",
  inputsTitle: "Inputs",
  diffsTitle: "Diffs",
  outputsTitle: "Outputs",
};

function IpoFrame() {
  const language = useAlgorithmLanguage();
  const kit = metabolismKit as unknown as Kit;

  const baseDesign = React.useMemo(() => {
    return findDesignInKit(kit, nakaginCapsuleTowerDesignGuid) as Design;
  }, [kit]);

  const [outputKit, outputDesign, designDiff, error] = React.useMemo(() => {
    try {
      const change =
        language === AlgorithmLanguage.RUST
          ? flattenDesign(kit, nakaginCapsuleTowerDesignGuid)
          : language === AlgorithmLanguage.PYTHON
            ? flattenDesign(kit, nakaginCapsuleTowerDesignGuid)
            : language === AlgorithmLanguage.GO
              ? flattenDesign(kit, nakaginCapsuleTowerDesignGuid)
              : flattenDesign(kit, nakaginCapsuleTowerDesignGuid);

      const diff = change.forward as DesignDiff;
      const outDesign = applyDesignDiff(baseDesign, diff);
      const nextKit: Kit = {
        ...kit,
        designs: (kit.designs ?? []).map((d) => (d.guid === outDesign.guid ? outDesign : d)),
      };
      return [nextKit, outDesign, diff, undefined] as const;
    } catch (e: any) {
      return [kit, baseDesign, undefined, String(e?.message ?? e)] as const;
    }
  }, [baseDesign, kit, language]);

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
      defaultLayout: createIpoWindowConfig({
        ...defaultWindowIds,
      }).defaultLayout,
    };
  }, []);

  const state: IpoState = {
    language,
    kit,
    baseDesign,
    designDiff,
    outputKit,
    outputDesign,
    error,
  };

  const apps = [
    {
      id: "ipo-flatten",
      label: "Flatten",
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
