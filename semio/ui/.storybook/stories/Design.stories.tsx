// #region 🧲Header
// 💻 semio/ui/.storybook/stories/Design.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses design prop directly. Kit is optional for 3D models.
// Summary: Design stories: Default, Diff, Selection, FeaturesDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { asKitInstance, Design, type Connection, type Design as DesignType, type DesignPlain, type Kit, type Piece } from "@semio/js";
import { SemioDesign as DesignView } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import nakaginDiff from "../../../assets/semio/nakgin-capsule-tower.diff.design.semio.json";

// #region 🖥️Data

const kit = asKitInstance(metabolismKit as unknown as Kit);
const rawDesign = (metabolismKit.designs ?? []).find((d) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef")! as DesignType;
const flatOp = kit.runFlattenDesign(rawDesign.guid);
if (!flatOp.ok) throw new Error(flatOp.errors.map((e) => e.message).join("; "));
const flattenChange = flatOp.diff;
const nakaginDesign = new Design(JSON.parse(JSON.stringify(rawDesign)) as DesignPlain, kit);
nakaginDesign.applyDiff({ pieces: flattenChange.forward.pieces });
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";
const designDiff = nakaginDiff as any;

const usedTypeGuids = new Set((nakaginDesign.pieces ?? []).map((p) => p.type?.guid).filter(Boolean));
const minimalTypes = (metabolismKit.types ?? [])
  .filter((t: any) => usedTypeGuids.has(t.guid))
  .map((t: any) => {
    const models = (t.models ?? []).slice(0, 1);
    return { ...t, models };
  });
const usedFileGuids = new Set(minimalTypes.flatMap((t: any) => (t.models ?? []).map((m: any) => m.file?.guid).filter(Boolean)));
const minimalFiles = (metabolismKit.files ?? []).filter((f: any) => usedFileGuids.has(f.guid));
const minimalKit = { types: minimalTypes, files: minimalFiles } as any;

// #endregion 🖥️Data

// #region 📌Design

const meta: Meta<typeof DesignView> = {
  title: "semio/Design",
  component: DesignView,
  tags: ["autodocs"],
  parameters: { layout: "centered" },
};

export default meta;

type Story = StoryObj<typeof DesignView>;

const frame = (node: React.ReactNode) => <div className="h-96 w-3xl rounded-md border border-border bg-card text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    defaultSelection: { pieceGuids: [firstPieceGuid] },
    title: "Design",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection: Connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => frame(<DesignView {...args} />),
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
  render: (args) => frame(<DesignView {...args} />),
};

export const Selection: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    defaultSelection: { pieceGuids: [firstPieceGuid] },
    diffEnabled: false,
    title: "Selection",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
  },
  render: (args) => frame(<DesignView {...args} />),
};

export const FeaturesDisabled: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    showGrid: false,
    showGizmo: false,
    selectionEnabled: false,
    diffEnabled: false,
    sceneRatio: 0.7,
    title: "Features Disabled",
  },
  render: (args) => frame(<DesignView {...args} />),
};

export const ZoomToDesign: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    zoomTarget: "design",
    title: "Zoom To Design",
  },
  render: (args) => frame(<DesignView {...args} />),
};

export const ZoomToDiff: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    zoomTarget: "diff",
    title: "Zoom To Diff",
  },
  render: (args) => frame(<DesignView {...args} />),
};

export const ZoomNone: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    zoomTarget: "none",
    title: "Zoom None",
  },
  render: (args) => frame(<DesignView {...args} />),
};

// #endregion 📌Design
