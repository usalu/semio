// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Storybook stories for ValidationTree component with Treppenraum_A example.

// #endregion 🧲Header

// #region 📮Stories
// Storybook stories exercising the validation tree viewer with real evaluation data.
// Stories MUST use the Treppenraum_A example from the OWL ontology validation.

import type { ValidationReport } from "@coda/desktop/renderer";
import { ValidationTree } from "@coda/desktop/renderer";
import type { Meta, StoryObj } from "@storybook/react";
import React from "react";

// 🎨#region 🏷️CodaThemeWrapper
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

// #region 🖼️TreppenraumReport
// 📋The full Treppenraum_A validation report as JSON-compatible data.
const treppenraumReport: ValidationReport = {
  instance: "Treppenraum_A",
  expression: "not (verbindet exactly 2 (Geschoss and (verbundenZu some Rettungsweg) and (in exactly 1 (Nutzungseinheit and (bruttoGrundfläche some xsd:float[<= 200.0f])))))",
  truth: "false",
  tree: {
    id: "n0",
    kind: "Not",
    label: "NOT",
    fragment: "not (...)",
    truth: "false",
    summary: "false because child is true",
    children: [
      {
        id: "n1",
        kind: "ExactCardinality",
        label: "EXACTLY 2 verbindet",
        fragment: "verbindet exactly 2 (...)",
        truth: "true",
        property: "verbindet",
        expectedCardinality: 2,
        matchingCount: 2,
        summary: "true because exactly 2 fillers satisfy the restriction",
        children: [
          {
            id: "n2",
            kind: "Witness",
            label: "Geschoss_EG",
            individual: "Geschoss_EG",
            truth: "true",
            counted: true,
            summary: "counted filler 1 of 2",
            children: [
              {
                id: "n3",
                kind: "And",
                label: "AND",
                fragment: "Geschoss and (verbundenZu some Rettungsweg) and (in exactly 1 (...))",
                truth: "true",
                summary: "true because all children are true",
                children: [
                  {
                    id: "n4",
                    kind: "ClassAssertion",
                    label: "Geschoss",
                    className: "Geschoss",
                    subject: "Geschoss_EG",
                    truth: "true",
                    summary: "Geschoss_EG is an instance of Geschoss",
                    children: [],
                  },
                  {
                    id: "n5",
                    kind: "SomeValuesFrom",
                    label: "SOME verbundenZu Rettungsweg",
                    fragment: "verbundenZu some Rettungsweg",
                    truth: "true",
                    property: "verbundenZu",
                    summary: "true because at least one related instance is a Rettungsweg",
                    children: [
                      {
                        id: "n6",
                        kind: "Witness",
                        label: "Rettungsweg_1",
                        individual: "Rettungsweg_1",
                        truth: "true",
                        summary: "witness for verbundenZu",
                        children: [
                          {
                            id: "n7",
                            kind: "ClassAssertion",
                            label: "Rettungsweg",
                            className: "Rettungsweg",
                            subject: "Rettungsweg_1",
                            truth: "true",
                            summary: "Rettungsweg_1 is an instance of Rettungsweg",
                            children: [],
                          },
                        ],
                      },
                    ],
                  },
                  {
                    id: "n8",
                    kind: "ExactCardinality",
                    label: "EXACTLY 1 in",
                    fragment: "in exactly 1 (...)",
                    truth: "true",
                    property: "in",
                    expectedCardinality: 1,
                    matchingCount: 1,
                    summary: "true because exactly 1 filler satisfies the restriction",
                    children: [
                      {
                        id: "n9",
                        kind: "Witness",
                        label: "NE_01",
                        individual: "NE_01",
                        truth: "true",
                        counted: true,
                        summary: "counted filler",
                        children: [
                          {
                            id: "n10",
                            kind: "And",
                            label: "AND",
                            fragment: "Nutzungseinheit and (bruttoGrundfläche some xsd:float[<= 200.0f])",
                            truth: "true",
                            summary: "true because all children are true",
                            children: [
                              {
                                id: "n11",
                                kind: "ClassAssertion",
                                label: "Nutzungseinheit",
                                className: "Nutzungseinheit",
                                subject: "NE_01",
                                truth: "true",
                                summary: "NE_01 is an instance of Nutzungseinheit",
                                children: [],
                              },
                              {
                                id: "n12",
                                kind: "DataSomeValuesFrom",
                                label: "SOME bruttoGrundfläche xsd:float[<= 200.0f]",
                                fragment: "bruttoGrundfläche some xsd:float[<= 200.0f]",
                                truth: "true",
                                property: "bruttoGrundfläche",
                                summary: "true because at least one value satisfies the datatype restriction",
                                children: [
                                  {
                                    id: "n13",
                                    kind: "DataValue",
                                    label: "180.0",
                                    value: 180.0,
                                    datatype: "xsd:float",
                                    truth: "true",
                                    summary: "180.0 <= 200.0",
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
          },
          {
            id: "n14",
            kind: "Witness",
            label: "Geschoss_1OG",
            individual: "Geschoss_1OG",
            truth: "true",
            counted: true,
            summary: "counted filler 2 of 2",
            children: [
              {
                id: "n15",
                kind: "And",
                label: "AND",
                fragment: "Geschoss and (verbundenZu some Rettungsweg) and (in exactly 1 (...))",
                truth: "true",
                summary: "true because all children are true",
                children: [
                  {
                    id: "n16",
                    kind: "ClassAssertion",
                    label: "Geschoss",
                    className: "Geschoss",
                    subject: "Geschoss_1OG",
                    truth: "true",
                    summary: "Geschoss_1OG is an instance of Geschoss",
                    children: [],
                  },
                  {
                    id: "n17",
                    kind: "SomeValuesFrom",
                    label: "SOME verbundenZu Rettungsweg",
                    fragment: "verbundenZu some Rettungsweg",
                    truth: "true",
                    property: "verbundenZu",
                    summary: "true because at least one related instance is a Rettungsweg",
                    children: [
                      {
                        id: "n18",
                        kind: "Witness",
                        label: "Rettungsweg_2",
                        individual: "Rettungsweg_2",
                        truth: "true",
                        summary: "witness for verbundenZu",
                        children: [
                          {
                            id: "n19",
                            kind: "ClassAssertion",
                            label: "Rettungsweg",
                            className: "Rettungsweg",
                            subject: "Rettungsweg_2",
                            truth: "true",
                            summary: "Rettungsweg_2 is an instance of Rettungsweg",
                            children: [],
                          },
                        ],
                      },
                    ],
                  },
                  {
                    id: "n20",
                    kind: "ExactCardinality",
                    label: "EXACTLY 1 in",
                    fragment: "in exactly 1 (...)",
                    truth: "true",
                    property: "in",
                    expectedCardinality: 1,
                    matchingCount: 1,
                    summary: "true because exactly 1 filler satisfies the restriction",
                    children: [
                      {
                        id: "n21",
                        kind: "Witness",
                        label: "NE_02",
                        individual: "NE_02",
                        truth: "true",
                        counted: true,
                        summary: "counted filler",
                        children: [
                          {
                            id: "n22",
                            kind: "And",
                            label: "AND",
                            fragment: "Nutzungseinheit and (bruttoGrundfläche some xsd:float[<= 200.0f])",
                            truth: "true",
                            summary: "true because all children are true",
                            children: [
                              {
                                id: "n23",
                                kind: "ClassAssertion",
                                label: "Nutzungseinheit",
                                className: "Nutzungseinheit",
                                subject: "NE_02",
                                truth: "true",
                                summary: "NE_02 is an instance of Nutzungseinheit",
                                children: [],
                              },
                              {
                                id: "n24",
                                kind: "DataSomeValuesFrom",
                                label: "SOME bruttoGrundfläche xsd:float[<= 200.0f]",
                                fragment: "bruttoGrundfläche some xsd:float[<= 200.0f]",
                                truth: "true",
                                property: "bruttoGrundfläche",
                                summary: "true because at least one value satisfies the datatype restriction",
                                children: [
                                  {
                                    id: "n25",
                                    kind: "DataValue",
                                    label: "150.0",
                                    value: 150.0,
                                    datatype: "xsd:float",
                                    truth: "true",
                                    summary: "150.0 <= 200.0",
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
          },
        ],
      },
    ],
  },
};
// #endregion 🖼️TreppenraumReport

// #region 📔IncompleteDataReport
// 🌿Example with unknown (gray) nodes due to incomplete data.
const incompleteDataReport: ValidationReport = {
  instance: "Treppenraum_B",
  expression: "not (verbindet exactly 2 (Geschoss and (verbundenZu some Rettungsweg) and (in exactly 1 (Nutzungseinheit and (bruttoGrundfläche some xsd:float[<= 200.0f])))))",
  truth: "unknown",
  tree: {
    id: "u0",
    kind: "Not",
    label: "NOT",
    fragment: "not (...)",
    truth: "unknown",
    summary: "unknown because child is unknown",
    children: [
      {
        id: "u1",
        kind: "ExactCardinality",
        label: "EXACTLY 2 verbindet",
        fragment: "verbindet exactly 2 (...)",
        truth: "unknown",
        property: "verbindet",
        expectedCardinality: 2,
        matchingCount: 1,
        summary: "unknown because only 1 of 2 fillers evaluated so far",
        children: [
          {
            id: "u2",
            kind: "Witness",
            label: "Geschoss_EG",
            individual: "Geschoss_EG",
            truth: "true",
            counted: true,
            summary: "counted filler 1",
            children: [
              {
                id: "u3",
                kind: "And",
                label: "AND",
                fragment: "Geschoss and (verbundenZu some Rettungsweg) and (in exactly 1 (...))",
                truth: "true",
                summary: "true because all children are true",
                children: [
                  {
                    id: "u4",
                    kind: "Class",
                    label: "Geschoss",
                    className: "Geschoss",
                    truth: "true",
                    summary: "Geschoss_EG is an instance of Geschoss",
                    children: [],
                  },
                  {
                    id: "u5",
                    kind: "SomeValuesFrom",
                    label: "SOME verbundenZu Rettungsweg",
                    fragment: "verbundenZu some Rettungsweg",
                    truth: "true",
                    property: "verbundenZu",
                    summary: "true because Rettungsweg_1 is a Rettungsweg",
                    children: [
                      {
                        id: "u6",
                        kind: "Witness",
                        label: "Rettungsweg_1",
                        individual: "Rettungsweg_1",
                        truth: "true",
                        summary: "witness for verbundenZu",
                        children: [],
                      },
                    ],
                  },
                  {
                    id: "u7",
                    kind: "ExactCardinality",
                    label: "EXACTLY 1 in",
                    fragment: "in exactly 1 (...)",
                    truth: "unknown",
                    property: "in",
                    expectedCardinality: 1,
                    matchingCount: 1,
                    summary: "unknown because Nutzungseinheit area value is missing",
                    children: [
                      {
                        id: "u8",
                        kind: "Witness",
                        label: "NE_03",
                        individual: "NE_03",
                        truth: "unknown",
                        counted: true,
                        summary: "counted filler but incomplete",
                        children: [
                          {
                            id: "u9",
                            kind: "And",
                            label: "AND",
                            fragment: "Nutzungseinheit and (bruttoGrundfläche some xsd:float[<= 200.0f])",
                            truth: "unknown",
                            summary: "unknown because bruttoGrundfläche data is missing",
                            children: [
                              {
                                id: "u10",
                                kind: "Class",
                                label: "Nutzungseinheit",
                                className: "Nutzungseinheit",
                                truth: "true",
                                summary: "NE_03 is an instance of Nutzungseinheit",
                                children: [],
                              },
                              {
                                id: "u11",
                                kind: "DataSomeValuesFrom",
                                label: "SOME bruttoGrundfläche xsd:float[<= 200.0f]",
                                fragment: "bruttoGrundfläche some xsd:float[<= 200.0f]",
                                truth: "unknown",
                                property: "bruttoGrundfläche",
                                summary: "unknown because no value is known",
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
          {
            id: "u12",
            kind: "Witness",
            label: "Technikraum_Dach",
            individual: "Technikraum_Dach",
            truth: "false",
            counted: false,
            summary: "not matching",
            children: [],
          },
        ],
      },
    ],
  },
};
// #endregion 📔IncompleteDataReport

// 🌳#region ✏️ValidationTreeMeta
const validationMeta = {
  title: "🖱️ui⚛️react/ValidationTree",
  component: ValidationTree,
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
} satisfies Meta<typeof ValidationTree>;

export default validationMeta;

type ValidationStory = StoryObj<typeof validationMeta>;
// #endregion ✏️ValidationTreeMeta

/**
 * Full Treppenraum_A evaluation: NOT is false because inner EXACTLY 2 is true.
 * All witnesses are green, both floors satisfy the restriction.
 **/
export const TreppenraumReport: ValidationStory = {
  args: {
    report: treppenraumReport,
    defaultExpanded: true,
  },
};

/**
 * Collapsed validation tree (all nodes start collapsed).
 **/
export const CollapsedReport: ValidationStory = {
  args: {
    report: treppenraumReport,
    defaultExpanded: false,
  },
};

/**
 * Incomplete data scenario: some nodes are gray (unknown) due to missing values.
 * Technikraum_Dach is dimmed as a non-matching filler.
 **/
export const IncompleteDataReport: ValidationStory = {
  args: {
    report: incompleteDataReport,
    defaultExpanded: true,
  },
};
/**
 * Multiple witnesses under a SomeValuesFrom: branch navigation shows < 1/3 > to switch.
 **/
export const MultipleWitnessAlternatives: ValidationStory = {
  args: {
    report: {
      instance: "Gebäude_X",
      expression: "hatGeschoss some (Geschoss and (hatNutzung some Büro))",
      truth: "true",
      tree: {
        id: "mw0",
        kind: "SomeValuesFrom",
        label: "SOME hatGeschoss",
        fragment: "hatGeschoss some (Geschoss and (hatNutzung some Büro))",
        truth: "true",
        property: "hatGeschoss",
        summary: "true because at least one floor has office use",
        children: [
          {
            id: "mw1",
            kind: "Witness",
            label: "Geschoss_EG",
            individual: "Geschoss_EG",
            truth: "true",
            summary: "witness 1 of 3",
            children: [
              {
                id: "mw1a",
                kind: "And",
                label: "AND",
                truth: "true",
                summary: "all conditions met",
                children: [
                  { id: "mw1b", kind: "ClassAssertion", label: "Geschoss", className: "Geschoss", subject: "Geschoss_EG", truth: "true", summary: "is a Geschoss", children: [] },
                  { id: "mw1c", kind: "SomeValuesFrom", label: "SOME hatNutzung Büro", fragment: "hatNutzung some Büro", truth: "true", property: "hatNutzung", summary: "has Büro usage", children: [] },
                ],
              },
            ],
          },
          {
            id: "mw2",
            kind: "Witness",
            label: "Geschoss_1OG",
            individual: "Geschoss_1OG",
            truth: "true",
            summary: "witness 2 of 3",
            children: [
              {
                id: "mw2a",
                kind: "And",
                label: "AND",
                truth: "true",
                summary: "all conditions met",
                children: [
                  { id: "mw2b", kind: "ClassAssertion", label: "Geschoss", className: "Geschoss", subject: "Geschoss_1OG", truth: "true", summary: "is a Geschoss", children: [] },
                  { id: "mw2c", kind: "SomeValuesFrom", label: "SOME hatNutzung Büro", fragment: "hatNutzung some Büro", truth: "true", property: "hatNutzung", summary: "has Büro usage", children: [] },
                ],
              },
            ],
          },
          {
            id: "mw3",
            kind: "Witness",
            label: "Geschoss_2OG",
            individual: "Geschoss_2OG",
            truth: "false",
            counted: false,
            summary: "witness 3 of 3 — not matching",
            children: [
              {
                id: "mw3a",
                kind: "And",
                label: "AND",
                truth: "false",
                summary: "not all conditions met",
                children: [
                  { id: "mw3b", kind: "ClassAssertion", label: "Geschoss", className: "Geschoss", subject: "Geschoss_2OG", truth: "true", summary: "is a Geschoss", children: [] },
                  { id: "mw3c", kind: "SomeValuesFrom", label: "SOME hatNutzung Büro", fragment: "hatNutzung some Büro", truth: "false", property: "hatNutzung", summary: "no Büro usage found", children: [] },
                ],
              },
            ],
          },
        ],
      },
    },
    defaultExpanded: true,
  },
};
// #endregion 📮ValidationTreeStories

//#endregion 📮Stories
