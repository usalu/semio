// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Diagram.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses design prop directly (no kit/designGuid).
// Summary: Diagram stories: Default, Diff, Selection, FeaturesDisabled, NakginDiff.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { applyDesignDiff, flattenDesign, getDesignDiff, type Connection, type Design, type Kit, type Piece } from "@semio/js";
import { SemioDiagram as Diagram } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/metabolism.kit.semio.json";
import nakginDiffDesign from "../../../assets/semio/nakgin-capsule-tower.diff.design.semio.json";

// #region 🔖Data

const rawDesign = (metabolismKit.designs ?? []).find((d) => d.guid === "9a890dd4-0a9c-48ac-920a-9e62666465ef")! as Design;
const flattenChange = flattenDesign(metabolismKit as unknown as Kit, rawDesign.guid);
const nakaginDesign = applyDesignDiff(rawDesign, { pieces: flattenChange.forward.pieces });
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";

const connectionCounts = new Map<string, number>();
(nakaginDesign.connections ?? []).forEach((connection) => {
  connectionCounts.set(connection.connected.piece.guid, (connectionCounts.get(connection.connected.piece.guid) ?? 0) + 1);
  connectionCounts.set(connection.connecting.piece.guid, (connectionCounts.get(connection.connecting.piece.guid) ?? 0) + 1);
});

const removedPiece = (nakaginDesign.pieces ?? []).find((piece) => (connectionCounts.get(piece.guid) ?? 0) === 1) as Piece;
const removedConnection = (nakaginDesign.connections ?? []).find((connection) => connection.connected.piece.guid === removedPiece.guid || connection.connecting.piece.guid === removedPiece.guid) as Connection;
const modifiedPiece = (nakaginDesign.pieces ?? []).find((piece) => Boolean(piece.plane)) as Piece;
const modifiedConnection = (nakaginDesign.connections ?? []).find((connection) => connection.guid !== removedConnection.guid) as Connection;

const diffedDesign: Design = structuredClone(nakaginDesign);
diffedDesign.pieces = (diffedDesign.pieces ?? []).filter((piece) => piece.guid !== removedPiece.guid);
diffedDesign.connections = (diffedDesign.connections ?? []).filter((connection) => connection.guid !== removedConnection.guid);
diffedDesign.pieces = (diffedDesign.pieces ?? []).map((piece) => (piece.guid === modifiedPiece.guid ? { ...piece, center: { u: (piece.center?.u ?? 0) + 3, v: (piece.center?.v ?? 0) + 2 } } : piece));
diffedDesign.connections = (diffedDesign.connections ?? []).map((connection) => (connection.guid === modifiedConnection.guid ? { ...connection, u: (connection.u ?? 0) + 1.5, v: (connection.v ?? 0) - 1 } : connection));
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

// #endregion 🔖Data

// #region 🔖Diagram

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
      pieceGuids: [removedPiece.guid, modifiedPiece.guid, addedPieceGuid],
      connectionGuids: [removedConnection.guid, modifiedConnection.guid, addedConnectionGuid],
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

export const NakginDiff: Story = {
  args: {
    design: nakaginDesign,
    designDiff: nakginDiffDesign as any,
    diffEnabled: true,
    selectionEnabled: false,
    title: "Nakgin Diff",
  },
  render: (args) => frame(<Diagram {...args} />),
};

// #endregion 🔖Diagram
