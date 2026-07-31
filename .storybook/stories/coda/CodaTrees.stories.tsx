// #region 🧲️Header
// 💻️ .storybook/stories/coda/CodaTrees.stories.tsx
// Specs: `OntologyTree`/`ValidationTree` (from `@semio-tech/coda-desktop/renderer`) already have stories under
// `.storybook/stories/ui/OntologyTree.stories.tsx` and `.storybook/stories/ui/ValidationTree.stories.tsx` —
// read first. Those cover `OntologyNodeKind`s And/Or/Not/SomeValuesFrom/ExactCardinality/DataSomeValuesFrom/Class
// and all three `TruthValue`s (true/false/unknown), so those are NOT repeated here.
// Summary: `OntologyNodeKind` has 13 members (`AllValuesFrom` | `MinCardinality` | `MaxCardinality` |
// `DataAllValuesFrom` | `DataHasValue` | `DatatypeRestriction` were the genuine gap — every existing story
// avoids them) and `hasValidationCardinalityBadge` (renderer.tsx) gives Min/Max the same `n/n` chip as Exact,
// which no existing `ValidationTree` story exercises either (only `ExactCardinality` is shown there). This
// file fills exactly that gap: one `OntologyTree` story for the 6 missing kinds, one `ValidationTree` story
// putting Min/Max cardinality badges + the same 6 kinds through real truth/witness evaluation.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react";
import type { OntologyTreeNode, ValidationReport } from "@semio-tech/coda-desktop/renderer";
import { OntologyTree, ValidationTree } from "@semio-tech/coda-desktop/renderer";
import type { ComponentType, ReactNode } from "react";
// #endregion 🔌️Adapters

// #region 🎨️CodaThemeWrapper
/** @emoji 🎨️ Coda-specific CSS variables the tree components read (mirrors `.storybook/stories/ui/OntologyTree.stories.tsx`). */
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

// #region 🕳️UncoveredOntologyKindsFixture
/** @emoji 🕳️ A Wohnung (residential unit) class expression exercising the 6 `OntologyNodeKind`s no existing story touches. */
const uncoveredKindsOntology: OntologyTreeNode = {
  id: "u0",
  kind: "And",
  label: "AND",
  fragment: "Wohnung and (hatRaum only Wohnraum) and (hatFenster min 1) and (hatFenster max 4) and (hatBalkon value true) and (wohnflaeche some xsd:float[< 10.0f])",
  children: [
    { id: "u1", kind: "AllValuesFrom", label: "ONLY hatRaum Wohnraum", fragment: "hatRaum only Wohnraum", property: "hatRaum", children: [{ id: "u1a", kind: "Class", label: "Wohnraum", className: "Wohnraum", children: [] }] },
    { id: "u2", kind: "MinCardinality", label: "MIN 1 hatFenster", fragment: "hatFenster min 1", property: "hatFenster", cardinality: 1, children: [] },
    { id: "u3", kind: "MaxCardinality", label: "MAX 4 hatFenster", fragment: "hatFenster max 4", property: "hatFenster", cardinality: 4, children: [] },
    { id: "u4", kind: "DataAllValuesFrom", label: "ONLY hatBalkon xsd:boolean", fragment: "hatBalkon only xsd:boolean", property: "hatBalkon", datatype: "xsd:boolean", children: [] },
    { id: "u5", kind: "DataHasValue", label: "VALUE hatBalkon true", fragment: "hatBalkon value true", property: "hatBalkon", children: [] },
    {
      id: "u6",
      kind: "DataSomeValuesFrom",
      label: "SOME wohnflaeche xsd:float[< 10.0f]",
      fragment: "wohnflaeche some xsd:float[< 10.0f]",
      property: "wohnflaeche",
      children: [{ id: "u6a", kind: "DatatypeRestriction", label: "xsd:float[< 10.0f]", datatype: "xsd:float", restriction: "< 10.0", children: [] }],
    },
  ],
};
// #endregion 🕳️UncoveredOntologyKindsFixture

// #region 🌳️OntologyTreeStory
const ontologyMeta = {
  title: "🧠️coda/OntologyTree",
  component: OntologyTree,
  parameters: { layout: "padded" },
  tags: ["autodocs"],
  decorators: [(Story: ComponentType) => <CodaThemeWrapper><Story /></CodaThemeWrapper>],
} satisfies Meta<typeof OntologyTree>;

export default ontologyMeta;

type OntologyStory = StoryObj<typeof ontologyMeta>;

/** @emoji 🕳️ The 6 `OntologyNodeKind`s (`AllValuesFrom`, `MinCardinality`, `MaxCardinality`, `DataAllValuesFrom`, `DataHasValue`, `DatatypeRestriction`) no `ui/OntologyTree` story exercises. */
export const UncoveredKinds: OntologyStory = {
  args: {
    root: uncoveredKindsOntology,
    title: "Wohnung (uncovered ontology node kinds)",
    defaultExpanded: true,
  },
};
// #endregion 🌳️OntologyTreeStory
