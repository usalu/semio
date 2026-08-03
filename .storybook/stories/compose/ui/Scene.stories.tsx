// #region 🧲️Header
// 💻️ compose/ui/.storybook/story/Scene.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses design prop directly. Kit is optional for 3D representations.
// Summary: Scene stories: Default, Diff, Selection, FeaturesDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { Design, Kit, type Connection, type Design as DesignType, type DesignPlain, type Piece } from "@semio-tech/compose-react";
import { MetabolismKit as metabolismKit, NakaginCapsuleTowerDiffDesign as nakaginDiff } from "@semio-tech/compose-fixture";
import { ComposeScene as Scene } from "@semio-tech/ui-react";
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

// 🏗️Build minimal kit with only types and files referenced by the design.
const usedTypeIds = new Set((nakaginDesign.pieces ?? []).map((p) => p.type?.id).filter(Boolean));
const minimalTypes = (metabolismKit.types ?? [])
  .filter((t: any) => usedTypeIds.has(t.id))
  .map((t: any) => {
    const representations = (t.representations ?? []).slice(0, 1);
    return { ...t, representations };
  });
const usedFileIds = new Set(minimalTypes.flatMap((t: any) => (t.representations ?? []).map((m: any) => m.file?.id).filter(Boolean)));
const minimalFiles = (metabolismKit.files ?? []).filter((f: any) => usedFileIds.has(f.id));
const minimalKit = { types: minimalTypes, files: minimalFiles } as any;

// #endregion 🖥️Data

// #region 📍️Scene

const meta: Meta<typeof Scene> = {
  title: "🏘️compose⚛️react/Scene",
  component: Scene,
  tags: ["autodocs"],
  parameters: { layout: "centered" },
};

export default meta;

type Story = StoryObj<typeof Scene>;

const frame = (node: React.ReactNode) => <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    defaultSelection: { pieceIds: [firstPieceId] },
    title: "Scene",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.id),
    onConnectionClick: (connection: Connection) => console.info("Connection clicked", connection.id),
  },
  render: (args) => frame(<Scene {...args} />),
};

export const Diff: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    diffEnabled: true,
    selectionEnabled: false,
    title: "Diff",
  },
  render: (args) => frame(<Scene {...args} />),
};

export const Selection: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    defaultSelection: { pieceIds: [firstPieceId] },
    diffEnabled: false,
    title: "Selection",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.id),
  },
  render: (args) => frame(<Scene {...args} />),
};

export const FeaturesDisabled: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    showGrid: false,
    showGizmo: false,
    selectionEnabled: false,
    diffEnabled: false,
    title: "Features Disabled",
  },
  render: (args) => frame(<Scene {...args} />),
};

export const ZoomToDesign: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    zoomTarget: "design",
    title: "Zoom To Design",
  },
  render: (args) => frame(<Scene {...args} />),
};

export const ZoomToDiff: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    zoomTarget: "diff",
    title: "Zoom To Diff",
  },
  render: (args) => frame(<Scene {...args} />),
};

export const ZoomNone: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    zoomTarget: "none",
    title: "Zoom None",
  },
  render: (args) => frame(<Scene {...args} />),
};

// #endregion 📍️Scene
