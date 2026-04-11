// #region 🧲Header
// 💻 semio/ui/.storybook/stories/Diagram.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses design prop directly (no kit/designGuid).
// Summary: Diagram stories: Default, Diff, Selection, FeaturesDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { applyDesignDiff, flattenDesign, type Design, type Kit } from "@semio/js";
import { SemioDiagram as Diagram } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import nakaginDiff from "../../../assets/semio/nakgin-capsule-tower.diff.design.semio.json";

// #region 🖥️Data

const rawDesign = (metabolismKit.designs ?? []).find((d) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef")! as Design;
const flatOp = flattenDesign(metabolismKit as unknown as Kit, rawDesign.guid);
if (!flatOp.ok) throw new Error(flatOp.errors.map((e) => e.message).join("; "));
const flattenChange = flatOp.change;
const nakaginDesign = applyDesignDiff(rawDesign, { pieces: flattenChange.forward.pieces });
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";
const designDiff = nakaginDiff as any;

// #endregion 🖥️Data

// #region 🧫Diagram

const meta: Meta<typeof Diagram> = {
  title: "semio/Diagram",
  component: Diagram,
  tags: ["autodocs"],
  parameters: { layout: "centered" },
};

export default meta;

type Story = StoryObj<typeof Diagram>;

const frame = (node: React.ReactNode) => <div className="h-72 w-72 rounded-md border border-border bg-card p-3 text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: {
    design: nakaginDesign,
    designDiff,
    defaultSelection: {
      pieceGuids: ["71e18c51-7752-46bb-917e-31874504b259", "0a23d9c7-b75b-4166-8730-351367df9f8a", "019daa00-0000-7000-b000-000000000001"],
      connectionGuids: ["40be9d59-91e8-4d8c-87b5-c5da567a4f9c", "019daa00-0000-7000-b000-000000000011"],
    },
    title: "Diagram",
    onPieceClick: (piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => frame(<Diagram {...args} />),
};

export const Diff: Story = {
  args: {
    design: nakaginDesign,
    designDiff,
    diffEnabled: true,
    selectionEnabled: false,
    title: "Diff",
  },
  render: (args) => frame(<Diagram {...args} />),
};

export const Selection: Story = {
  args: {
    design: nakaginDesign,
    defaultSelection: { pieceGuids: [firstPieceGuid], connectionGuids: [] },
    diffEnabled: false,
    title: "Selection",
    onPieceClick: (piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => frame(<Diagram {...args} />),
};

export const FeaturesDisabled: Story = {
  args: {
    design: nakaginDesign,
    designDiff,
    diffEnabled: false,
    selectionEnabled: false,
    panEnabled: false,
    zoomEnabled: false,
    title: "Features Disabled",
  },
  render: (args) => frame(<Diagram {...args} />),
};

export const ZoomToDesign: Story = {
  args: {
    design: nakaginDesign,
    designDiff,
    zoomTarget: "design",
    title: "Zoom To Design",
  },
  render: (args) => frame(<Diagram {...args} />),
};

export const ZoomToDiff: Story = {
  args: {
    design: nakaginDesign,
    designDiff,
    zoomTarget: "diff",
    title: "Zoom To Diff",
  },
  render: (args) => frame(<Diagram {...args} />),
};

export const ZoomNone: Story = {
  args: {
    design: nakaginDesign,
    designDiff,
    zoomTarget: "none",
    title: "Zoom None",
  },
  render: (args) => frame(<Diagram {...args} />),
};

// #endregion 🧫Diagram
