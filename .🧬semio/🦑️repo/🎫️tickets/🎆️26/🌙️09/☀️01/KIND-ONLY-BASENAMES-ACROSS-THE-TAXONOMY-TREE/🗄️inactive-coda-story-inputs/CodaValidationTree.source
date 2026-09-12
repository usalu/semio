// #region 🧲️Header
// 💻️ .storybook/stories/coda/Coda✅ValidationTree.stories.tsx
// Specs: Companion to `CodaTrees.stories.tsx` (read that file's header first) — same gap analysis, applied to
// `ValidationTree` instead of `OntologyTree`. A CSF file may only have one default-exported `meta`, so the
// two components get separate files even though they close the same coverage gap.
// Summary: `.storybook/stories/ui/✅ValidationTree.stories.tsx` only ever gives `ExactCardinality` a `n/n` chip
// (`hasValidationCardinalityBadge` in renderer.tsx also lights up for `MinCardinality`/`MaxCardinality`, never
// exercised) and never instantiates `AllValuesFrom` / `DataAllValuesFrom` / `DataHasValue` /
// `DatatypeRestriction` at the instance (truth-annotated) level. This story evaluates the same
// uncovered-kinds Wohnung expression against a real instance, mixing true/false/unknown truth per node.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ValidationReport } from "@semio-tech/coda-desktop/renderer";
import { ValidationTree } from "@semio-tech/coda-desktop/renderer";
import type { ComponentType, ReactNode } from "react";
// #endregion 🔌️Adapters

// #region 🎨️CodaThemeWrapper
/** @emoji 🎨️ Coda-specific CSS variables the tree components read (mirrors `.storybook/stories/ui/✅ValidationTree.stories.tsx`). */
const CodaThemeWrapper = ({ children }: { children: ReactNode }) => (
  <div
    style={{
      fontFamily: '"Inter", "Segoe UI", system-ui, -apple-system, sans-serif',
      padding: "24px",
      ["--color-compliant" as string]: "#22c55e",
      ["--color-violated" as string]: "#ef4444",
      ["--color-unknown" as string]: "#a3a3a3",
      ["--color-coda-50" as string]: "#f0f7ff",
      ["--color-coda-100" as string]: "#e0efff",
      ["--color-coda-700" as string]: "#0058a7",
      ["--color-surface" as string]: "#ffffff",
      ["--color-surface-alt" as string]: "#f8fafc",
      ["--color-surface-hover" as string]: "#f1f5f9",
      ["--color-border" as string]: "#e2e8f0",
      ["--color-text" as string]: "#0f172a",
      ["--color-text-secondary" as string]: "#475569",
      ["--color-text-tertiary" as string]: "#94a3b8",
    }}
  >
    {children}
  </div>
);
// #endregion 🎨️CodaThemeWrapper

// #region 🕳️UncoveredKindsReport
/** @emoji 🕳️ Wohnung_7 evaluation: MinCardinality passes (2 ≥ 1), MaxCardinality passes (2 ≤ 4) — both get the
 * `n/n` badge `hasValidationCardinalityBadge` grants ExactCardinality elsewhere but no existing story shows
 * for Min/Max. AllValuesFrom/DataAllValuesFrom/DataHasValue/DatatypeRestriction round out the 6-kind gap. */
const uncoveredKindsReport: ValidationReport = {
  instance: "Wohnung_7",
  expression: "Wohnung and (hatRaum only Wohnraum) and (hatFenster min 1) and (hatFenster max 4) and (hatBalkon value true) and (wohnflaeche some xsd:float[< 10.0f])",
  truth: "unknown",
  tree: {
    id: "n0",
    kind: "And",
    label: "AND",
    fragment: "Wohnung and (hatRaum only Wohnraum) and (hatFenster min 1) and (hatFenster max 4) and (hatBalkon value true) and (wohnflaeche some xsd:float[< 10.0f])",
    truth: "unknown",
    summary: "unknown because one child (data-value restriction) is unknown",
    children: [
      {
        id: "n1",
        kind: "AllValuesFrom",
        label: "ONLY hatRaum Wohnraum",
        fragment: "hatRaum only Wohnraum",
        truth: "true",
        property: "hatRaum",
        summary: "true because every related room is a Wohnraum",
        children: [
          { id: "n1a", kind: "ClassAssertion", label: "Wohnraum", className: "Wohnraum", subject: "Zimmer_1", truth: "true", summary: "Zimmer_1 is an instance of Wohnraum", children: [] },
          { id: "n1b", kind: "ClassAssertion", label: "Wohnraum", className: "Wohnraum", subject: "Zimmer_2", truth: "true", summary: "Zimmer_2 is an instance of Wohnraum", children: [] },
        ],
      },
      {
        id: "n2",
        kind: "MinCardinality",
        label: "MIN 1 hatFenster",
        fragment: "hatFenster min 1",
        truth: "true",
        property: "hatFenster",
        expectedCardinality: 1,
        matchingCount: 2,
        summary: "true because 2 fillers satisfy the minimum of 1",
        children: [
          { id: "n2a", kind: "Witness", label: "Fenster_1", individual: "Fenster_1", truth: "true", counted: true, summary: "counted filler", children: [] },
          { id: "n2b", kind: "Witness", label: "Fenster_2", individual: "Fenster_2", truth: "true", counted: true, summary: "counted filler", children: [] },
        ],
      },
      {
        id: "n3",
        kind: "MaxCardinality",
        label: "MAX 4 hatFenster",
        fragment: "hatFenster max 4",
        truth: "true",
        property: "hatFenster",
        expectedCardinality: 4,
        matchingCount: 2,
        summary: "true because 2 fillers does not exceed the maximum of 4",
        children: [
          { id: "n3a", kind: "Witness", label: "Fenster_1", individual: "Fenster_1", truth: "true", counted: true, summary: "counted filler", children: [] },
          { id: "n3b", kind: "Witness", label: "Fenster_2", individual: "Fenster_2", truth: "true", counted: true, summary: "counted filler", children: [] },
        ],
      },
      {
        id: "n4",
        kind: "DataHasValue",
        label: "VALUE hatBalkon true",
        fragment: "hatBalkon value true",
        truth: "false",
        property: "hatBalkon",
        summary: "false because hatBalkon is false for Wohnung_7",
        children: [{ id: "n4a", kind: "DataValue", label: "false", value: "false", datatype: "xsd:boolean", truth: "false", summary: "false !== true", children: [] }],
      },
      {
        id: "n5",
        kind: "DataAllValuesFrom",
        label: "ONLY hatBalkon xsd:boolean",
        fragment: "hatBalkon only xsd:boolean",
        truth: "true",
        property: "hatBalkon",
        datatype: "xsd:boolean",
        summary: "true because the only hatBalkon value is a well-typed xsd:boolean",
        children: [{ id: "n5a", kind: "DataValue", label: "false", value: "false", datatype: "xsd:boolean", truth: "true", summary: "false is a valid xsd:boolean", children: [] }],
      },
      {
        id: "n6",
        kind: "DataSomeValuesFrom",
        label: "SOME wohnflaeche xsd:float[< 10.0f]",
        fragment: "wohnflaeche some xsd:float[< 10.0f]",
        truth: "unknown",
        property: "wohnflaeche",
        summary: "unknown because wohnflaeche has not been measured yet",
        children: [{ id: "n6a", kind: "DatatypeRestriction", label: "xsd:float[< 10.0f]", datatype: "xsd:float", restriction: "< 10.0", truth: "unknown", summary: "no wohnflaeche value known", children: [] }],
      },
    ],
  },
};
// #endregion 🕳️UncoveredKindsReport

// #region 🌳️ValidationTreeStory
const meta = {
  title: "🧠️coda/ValidationTree",
  component: ValidationTree,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
  decorators: [(Story: ComponentType) => <CodaThemeWrapper><Story /></CodaThemeWrapper>],
} satisfies Meta<typeof ValidationTree>;

export default meta;

type Story = StoryObj<typeof meta>;

/** @emoji 🕳️ Min/Max cardinality badges + AllValuesFrom/DataAllValuesFrom/DataHasValue/DatatypeRestriction — no `ui/ValidationTree` story exercises any of these. */
export const UncoveredKinds: Story = {
  args: {
    report: uncoveredKindsReport,
    defaultExpanded: true,
  },
};
// #endregion 🌳️ValidationTreeStory
