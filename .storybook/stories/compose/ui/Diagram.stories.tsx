// #region 🧲Header
// 💻 compose/ui/.storybook/story/Diagram.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses design prop directly (no kit/designId).
// Summary: Diagram stories: Default, Diff, Selection, FeaturesDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { Design, Kit, type Design as DesignType, type DesignPlain } from "@semio-tech/compose-react";
import { MetabolismKit as metabolismKit } from "@semio-tech/compose-asset";
import { NakaginCapsuleTowerDiffDesign as nakaginDiff } from "@semio-tech/compose-fixture";
import { ComposeDiagram as Diagram } from "@compose/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

// #region 🖥️Data

const kit = Kit.ensure(metabolismKit as unknown as Kit);
const rawDesign = (metabolismKit.designs ?? []).find((d) => d.id === "9a890dd4-0a9c-48ac-920a-9e62666465ef")! as DesignType;
const flatOp = kit.runFlattenDesign(rawDesign.id);
if (!flatOp.ok) throw new Error(flatOp.errors.map((e) => e.message).join("; "));
const flattenChange = flatOp.diff;
const nakaginDesign = new Design(JSON.parse(JSON.stringify(rawDesign)) as DesignPlain, kit);
nakaginDesign.applyDiff({ pieces: flattenChange.forward.pieces });
const firstPieceId = (nakaginDesign.pieces ?? [])[0]?.id ?? "";
const designDiff = nakaginDiff as any;

// #endregion 🖥️Data

// #region 🧫Diagram

const meta: Meta<typeof Diagram> = {
  title: "🏘️compose⚛️react/Diagram",
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
      pieceIds: ["71e18c51-7752-46bb-917e-31874504b259", "0a23d9c7-b75b-4166-8730-351367df9f8a", "019daa00-0000-7000-b000-000000000001"],
      connectionIds: ["40be9d59-91e8-4d8c-87b5-c5da567a4f9c", "019daa00-0000-7000-b000-000000000011"],
    },
    title: "Diagram",
    onPieceClick: (piece) => console.info("Piece clicked", piece.id),
    onConnectionClick: (connection) => console.info("Connection clicked", connection.id),
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
    defaultSelection: { pieceIds: [firstPieceId], connectionIds: [] },
    diffEnabled: false,
    title: "Selection",
    onPieceClick: (piece) => console.info("Piece clicked", piece.id),
    onConnectionClick: (connection) => console.info("Connection clicked", connection.id),
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
