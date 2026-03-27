// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Model.stories.tsx
// Specs: One component per stories file with real semio design data for representative variants.
// Summary: Showcases the semio 3D Model viewer with Metabolism kit Nakagin Capsule Tower design data, including diff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { getDesignDiff, type Connection, type Design, type Piece } from "@semio/js";
import { SemioModel as Model } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/kit_metabolism.json";

// #region 🔖Model

const nakaginDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const nakaginDesign = (metabolismKit.designs ?? []).find((d) => d.guid === nakaginDesignGuid)! as Design;
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";

const connectionCounts = new Map<string, number>();
(nakaginDesign.connections ?? []).forEach((connection) => {
  connectionCounts.set(connection.connected.piece.guid, (connectionCounts.get(connection.connected.piece.guid) ?? 0) + 1);
  connectionCounts.set(connection.connecting.piece.guid, (connectionCounts.get(connection.connecting.piece.guid) ?? 0) + 1);
});
const removedPiece = (nakaginDesign.pieces ?? []).find((piece) => (connectionCounts.get(piece.guid) ?? 0) === 1) as Piece;
const removedConnection = (nakaginDesign.connections ?? []).find((connection) => connection.connected.piece.guid === removedPiece.guid || connection.connecting.piece.guid === removedPiece.guid) as Connection;

const diffPreviewDesign: Design = structuredClone(nakaginDesign);
diffPreviewDesign.pieces = (diffPreviewDesign.pieces ?? []).filter((piece) => piece.guid !== removedPiece.guid);
diffPreviewDesign.connections = (diffPreviewDesign.connections ?? []).filter((connection) => connection.guid !== removedConnection.guid);
const addedPieceGuid = "11111111-2222-3333-4444-555555555555";
const addedConnectionGuid = "66666666-7777-8888-9999-000000000000";
diffPreviewDesign.pieces = [...(diffPreviewDesign.pieces ?? []), { ...structuredClone(removedPiece), guid: addedPieceGuid, name: `${removedPiece.name}_added` }];
diffPreviewDesign.connections = [
  ...(diffPreviewDesign.connections ?? []),
  {
    ...structuredClone(removedConnection),
    guid: addedConnectionGuid,
    connecting: removedConnection.connecting.piece.guid === removedPiece.guid ? { ...removedConnection.connecting, piece: { guid: addedPieceGuid } } : removedConnection.connecting,
    connected: removedConnection.connected.piece.guid === removedPiece.guid ? { ...removedConnection.connected, piece: { guid: addedPieceGuid } } : removedConnection.connected,
  },
];
const previewDiff = getDesignDiff(nakaginDesign, diffPreviewDesign);

const meta: Meta<typeof Model> = {
  title: "semio/Model",
  component: Model,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Model>;

const modelFrame = (node: React.ReactNode) => (
  <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">{node}</div>
);

export const NakaginCapsuleTower: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    title: "Nakagin Capsule Tower Model",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
  },
  render: (args) => modelFrame(<Model {...args} />),
};

export const Diff: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    designDiff: previewDiff,
    diffEnabled: true,
    selectionEnabled: false,
    title: "Diff",
  },
  render: (args) => modelFrame(<Model {...args} />),
};

export const DefaultDiff: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    defaultDesignDiff: previewDiff,
    title: "Default Diff (Uncontrolled)",
  },
  render: (args) => modelFrame(<Model {...args} />),
};

export const WithSelection: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    defaultSelection: { pieceGuids: [firstPieceGuid] },
    title: "Nakagin Capsule Tower With Selection",
  },
  render: (args) => modelFrame(<Model {...args} />),
};

export const Controlled: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    title: "Controlled Nakagin Capsule Tower",
  },
  render: (args) => {
    const [selection, setSelection] = React.useState({ pieceGuids: [firstPieceGuid] });
    return modelFrame(<Model {...args} selection={selection} onSelectionChange={setSelection} />);
  },
};

// #endregion 🔖Model
