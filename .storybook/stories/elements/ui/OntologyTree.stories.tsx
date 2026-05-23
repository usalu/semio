// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Storybook stories for OntologyTree and ValidationTree components.

// #endregion 🧲Header

// #region 📮Stories
// Storybook stories exercising ontology and validation tree viewers.
// Stories MUST provide example data matching the OWL Treppenraum ontology.

import type { OntologyTreeNode } from "@coda/desktop/renderer";
import { OntologyTree } from "@coda/desktop/renderer";
import type { Meta, StoryObj } from "@storybook/react";
import React from "react";

// #region 🏷️CodaThemeWrapper
// 🎨Provides the coda-specific CSS variables needed by tree components.
const CodaThemeWrapper = ({ children }: { children: React.ReactNode }) => (
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
// #endregion 🏷️CodaThemeWrapper

// #region 🗡️OntologyTreeExampleData
// Ontology tree for NecessaryStaircaseInSeparateStairwell (English OWL example).

const necessaryStaircaseOntology: OntologyTreeNode = {
  id: "o0",
  kind: "And",
  label: "AND",
  fragment: "not(ExternalStaircase) and not(in some (BuildingClass1 or BuildingClass2)) and not(connects exactly 2 (...))",
  children: [
    {
      id: "o1",
      kind: "Not",
      label: "NOT ExternalStaircase",
      fragment: "not(ExternalStaircase)",
      children: [
        {
          id: "o2",
          kind: "Class",
          label: "ExternalStaircase",
          className: "ExternalStaircase",
          children: [],
        },
      ],
    },
    {
      id: "o3",
      kind: "Not",
      label: "NOT (in some (BuildingClass1 or BuildingClass2))",
      fragment: "not(in some (BuildingClass1 or BuildingClass2))",
      children: [
        {
          id: "o4",
          kind: "SomeValuesFrom",
          label: "SOME in",
          fragment: "in some (BuildingClass1 or BuildingClass2)",
          property: "in",
          children: [
            {
              id: "o5",
              kind: "Or",
              label: "OR",
              fragment: "BuildingClass1 or BuildingClass2",
              children: [
                {
                  id: "o6",
                  kind: "Class",
                  label: "BuildingClass1",
                  className: "BuildingClass1",
                  children: [],
                },
                {
                  id: "o7",
                  kind: "Class",
                  label: "BuildingClass2",
                  className: "BuildingClass2",
                  children: [],
                },
              ],
            },
          ],
        },
      ],
    },
    {
      id: "o8",
      kind: "Not",
      label: "NOT (connects exactly 2 (...))",
      fragment: "not(connects exactly 2 (Storey and (isConnectedTo some EscapeRoute) and (in exactly 1 (UsageUnit and (totalGrossFloorArea some xsd:float[<= 200.0f])))))",
      children: [
        {
          id: "o9",
          kind: "ExactCardinality",
          label: "EXACTLY 2 connects",
          fragment: "connects exactly 2 (...)",
          property: "connects",
          cardinality: 2,
          children: [
            {
              id: "o10",
              kind: "And",
              label: "AND",
              fragment: "Storey and (isConnectedTo some EscapeRoute) and (in exactly 1 (...))",
              children: [
                {
                  id: "o11",
                  kind: "Class",
                  label: "Storey",
                  className: "Storey",
                  children: [],
                },
                {
                  id: "o12",
                  kind: "SomeValuesFrom",
                  label: "SOME isConnectedTo EscapeRoute",
                  fragment: "isConnectedTo some EscapeRoute",
                  property: "isConnectedTo",
                  children: [
                    {
                      id: "o13",
                      kind: "Class",
                      label: "EscapeRoute",
                      className: "EscapeRoute",
                      children: [],
                    },
                  ],
                },
                {
                  id: "o14",
                  kind: "ExactCardinality",
                  label: "EXACTLY 1 in",
                  fragment: "in exactly 1 (...)",
                  property: "in",
                  cardinality: 1,
                  children: [
                    {
                      id: "o15",
                      kind: "And",
                      label: "AND",
                      fragment: "UsageUnit and (totalGrossFloorArea some xsd:float[<= 200.0f])",
                      children: [
                        {
                          id: "o16",
                          kind: "Class",
                          label: "UsageUnit",
                          className: "UsageUnit",
                          children: [],
                        },
                        {
                          id: "o17",
                          kind: "DataSomeValuesFrom",
                          label: "SOME totalGrossFloorArea xsd:float[<= 200.0f]",
                          fragment: "totalGrossFloorArea some xsd:float[<= 200.0f]",
                          property: "totalGrossFloorArea",
                          datatype: "xsd:float",
                          restriction: "<= 200.0",
                          children: [],
                        },
                      ],
                    },
                  ],
                },
              ],
            },
          ],
        },
      ],
    },
  ],
};
// #endregion 🗡️OntologyTreeExampleData

// 🌳#region 🌧️OntologyTreeStory
const ontologyMeta = {
  title: "elements/react/OntologyTree",
  component: OntologyTree,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
  decorators: [
    (Story: React.ComponentType) => (
      <CodaThemeWrapper>
        <Story />
      </CodaThemeWrapper>
    ),
  ],
} satisfies Meta<typeof OntologyTree>;

export default ontologyMeta;

type OntologyStory = StoryObj<typeof ontologyMeta>;

/**
 * The NecessaryStaircaseInSeparateStairwell class expression tree.
 * Shows the pure ontology structure without instance data.
 **/
export const NecessaryStaircaseOntology: OntologyStory = {
  args: {
    root: necessaryStaircaseOntology,
    title: "NecessaryStaircaseInSeparateStairwell",
    defaultExpanded: true,
  },
};

/**
 * Collapsed ontology tree (all nodes collapsed by default).
 **/
export const CollapsedOntology: OntologyStory = {
  args: {
    root: necessaryStaircaseOntology,
    title: "NecessaryStaircaseInSeparateStairwell (Collapsed)",
    defaultExpanded: false,
  },
};

/**
 * Simple single-class ontology node.
 **/
export const SimpleClass: OntologyStory = {
  args: {
    root: {
      id: "s0",
      kind: "Class",
      label: "Storey",
      className: "Storey",
      children: [],
    },
    title: "Simple Class",
  },
};
// #endregion 🌧️OntologyTreeStory

// #endregion 📮Stories
