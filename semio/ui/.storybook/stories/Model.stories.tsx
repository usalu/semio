// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Model.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. SemioModel is an alias of SemioScene with a different default title.
// Summary: Model stories: Default, Diff, Selection, Controlled, FeaturesDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { getDesignDiff, type Connection, type Design, type Piece } from "@semio/js";
import { SemioModel as Model } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";

// #region 🔖Data

const nakaginDesign = (metabolismKit.designs ?? []).find((d) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef")! as Design;
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";

const connectionCounts = new Map<string, number>();
(nakaginDesign.connections ?? []).forEach((connection) => {
  connectionCounts.set(connection.connected.piece.guid, (connectionCounts.get(connection.connected.piece.guid) ?? 0) + 1);
  connectionCounts.set(connection.connecting.piece.guid, (connectionCounts.get(connection.connecting.piece.guid) ?? 0) + 1);
});

const removedPiece = (nakaginDesign.pieces ?? []).find((piece) => (connectionCounts.get(piece.guid) ?? 0) === 1) as Piece;
const removedConnection = (nakaginDesign.connections ?? []).find((connection) => connection.connected.piece.guid === removedPiece.guid || connection.connecting.piece.guid === removedPiece.guid) as Connection;

const diffedDesign: Design = structuredClone(nakaginDesign);
diffedDesign.pieces = (diffedDesign.pieces ?? []).filter((piece) => piece.guid !== removedPiece.guid);
diffedDesign.connections = (diffedDesign.connections ?? []).filter((connection) => connection.guid !== removedConnection.guid);
const addedPieceGuid = "11111111-2222-3333-4444-555555555555";
const addedConnectionGuid = "66666666-7777-8888-9999-000000000000";
diffedDesign.pieces = [...(diffedDesign.pieces ?? []), { ...structuredClone(removedPiece), guid: addedPieceGuid, name: `${removedPiece.name}_added` }];
diffedDesign.connections = [
  ...(diffedDesign.connections ?? []),
  {
    ...structuredClone(removedConnection),
    guid: addedConnectionGuid,
    connecting: removedConnection.connecting.piece.guid === removedPiece.guid ? { ...removedConnection.connecting, piece: { guid: addedPieceGuid } } : removedConnection.connecting,
    connected: removedConnection.connected.piece.guid === removedPiece.guid ? { ...removedConnection.connected, piece: { guid: addedPieceGuid } } : removedConnection.connected,
  },
];
const designDiff = getDesignDiff(nakaginDesign, diffedDesign);

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

// #endregion 🔖Data

// #region 🔖Model

const meta: Meta<typeof Model> = {
  title: "semio/Model",
  component: Model,
  tags: ["autodocs"],
  parameters: { layout: "centered" },
};

export default meta;

type Story = StoryObj<typeof Model>;

const frame = (node: React.ReactNode) => <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    designDiff,
    defaultSelection: { pieceGuids: [firstPieceGuid] },
    title: "Model",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection: Connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => frame(<Model {...args} />),
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
  render: (args) => frame(<Model {...args} />),
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
  render: (args) => frame(<Model {...args} />),
};

export const Controlled: Story = {
  args: {
    design: nakaginDesign,
    kit: minimalKit,
    title: "Controlled",
  },
  render: (args) => {
    const [selection, setSelection] = React.useState({ pieceGuids: [firstPieceGuid] });
    return frame(<Model {...args} selection={selection} onSelectionChange={setSelection} />);
  },
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
  render: (args) => frame(<Model {...args} />),
};

// #endregion 🔖Model
