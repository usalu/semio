// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Diagram.stories.tsx
// Specs: One component per stories file with real semio design data for representative variants.
// Summary: Diagram stories: default, controlled, features off, Design-only, Diff, PieceSelection, ConnectionSelection.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { getDesignDiff, type Connection, type Design, type Piece } from "@semio/js";
import { ConnectionSelection as DiagramConnectionSelection, PieceSelection as DiagramPieceSelection, SemioDiagram as Diagram } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/kit_metabolism.json";

const nakaginCapsuleTowerDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const nakaginCapsuleTowerDesign = (metabolismKit.designs ?? []).find((design) => design.guid === nakaginCapsuleTowerDesignGuid) as Design;
const connectionCounts = new Map<string, number>();

(nakaginCapsuleTowerDesign.connections ?? []).forEach((connection) => {
  connectionCounts.set(connection.connected.piece.guid, (connectionCounts.get(connection.connected.piece.guid) ?? 0) + 1);
  connectionCounts.set(connection.connecting.piece.guid, (connectionCounts.get(connection.connecting.piece.guid) ?? 0) + 1);
});

const removedPiece = (nakaginCapsuleTowerDesign.pieces ?? []).find((piece) => (connectionCounts.get(piece.guid) ?? 0) === 1) as Piece;
const removedConnection = (nakaginCapsuleTowerDesign.connections ?? []).find((connection) => connection.connected.piece.guid === removedPiece.guid || connection.connecting.piece.guid === removedPiece.guid) as Connection;
const modifiedPiece = (nakaginCapsuleTowerDesign.pieces ?? []).find((piece) => Boolean(piece.plane)) as Piece;
const modifiedConnection = (nakaginCapsuleTowerDesign.connections ?? []).find((connection) => connection.guid !== removedConnection.guid) as Connection;

const diffPreviewDesign: Design = structuredClone(nakaginCapsuleTowerDesign);
diffPreviewDesign.pieces = (diffPreviewDesign.pieces ?? []).filter((piece) => piece.guid !== removedPiece.guid);
diffPreviewDesign.connections = (diffPreviewDesign.connections ?? []).filter((connection) => connection.guid !== removedConnection.guid);
diffPreviewDesign.pieces = (diffPreviewDesign.pieces ?? []).map((piece) =>
  piece.guid === modifiedPiece.guid
    ? {
        ...piece,
        center: {
          u: (piece.center?.u ?? 0) + 3,
          v: (piece.center?.v ?? 0) + 2,
        },
      }
    : piece,
);
diffPreviewDesign.connections = (diffPreviewDesign.connections ?? []).map((connection) =>
  connection.guid === modifiedConnection.guid
    ? {
        ...connection,
        u: (connection.u ?? 0) + 1.5,
        v: (connection.v ?? 0) - 1,
      }
    : connection,
);

const addedPieceGuid = "11111111-2222-3333-4444-555555555555";
const addedConnectionGuid = "66666666-7777-8888-9999-000000000000";
const addedPiece: Piece = {
  ...structuredClone(removedPiece),
  guid: addedPieceGuid,
  name: `${removedPiece.name}_added`,
};
const addedConnection: Connection = {
  ...structuredClone(removedConnection),
  guid: addedConnectionGuid,
  connecting:
    removedConnection.connecting.piece.guid === removedPiece.guid
      ? {
          ...removedConnection.connecting,
          piece: { guid: addedPieceGuid },
        }
      : removedConnection.connecting,
  connected:
    removedConnection.connected.piece.guid === removedPiece.guid
      ? {
          ...removedConnection.connected,
          piece: { guid: addedPieceGuid },
        }
      : removedConnection.connected,
};
diffPreviewDesign.pieces = [...(diffPreviewDesign.pieces ?? []), addedPiece];
diffPreviewDesign.connections = [...(diffPreviewDesign.connections ?? []), addedConnection];

const previewDiff = getDesignDiff(nakaginCapsuleTowerDesign, diffPreviewDesign);

const meta: Meta<typeof Diagram> = {
  title: "semio/Diagram",
  component: Diagram,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Diagram>;
type PieceSelectionStory = StoryObj<typeof DiagramPieceSelection>;
type ConnectionSelectionStory = StoryObj<typeof DiagramConnectionSelection>;

const diagramFrame = (node: React.ReactNode) => <div className="h-72 w-72 rounded-md border border-border bg-card p-3 text-foreground shadow-sm">{node}</div>;

// #region 🔖DesignDiffSelectionVariants

/** Renders the flat design only: no diff overlay, selection off. */
export const Design: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    diffEnabled: false,
    selectionEnabled: false,
    title: "Design",
  },
  render: (args) => diagramFrame(<Diagram {...args} />),
};

/** Shows added/removed/modified pieces and connections from `previewDiff`. */
export const Diff: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    designDiff: previewDiff,
    diffEnabled: true,
    selectionEnabled: false,
    title: "Diff",
  },
  render: (args) => diagramFrame(<Diagram {...args} />),
};

/** Constrained `PieceSelection` wrapper: only pieces are selectable. */
export const PieceSelection: PieceSelectionStory = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    title: "Piece Selection",
    defaultSelection: { pieceGuids: [modifiedPiece.guid] },
    onPieceClick: (piece) => console.info("Piece clicked", piece.guid),
  },
  render: (args) => diagramFrame(<DiagramPieceSelection {...args} />),
};

/** Constrained `ConnectionSelection` wrapper: only connections are selectable. */
export const ConnectionSelection: ConnectionSelectionStory = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    title: "Connection Selection",
    defaultSelection: { connectionGuids: [modifiedConnection.guid] },
    onConnectionClick: (connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => diagramFrame(<DiagramConnectionSelection {...args} />),
};

// #endregion 🔖DesignDiffSelectionVariants

export const NakaginCapsuleTower: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    designDiff: previewDiff,
    defaultSelection: {
      pieceGuids: [removedPiece.guid, modifiedPiece.guid, addedPieceGuid],
      connectionGuids: [removedConnection.guid, modifiedConnection.guid, addedConnectionGuid],
    },
    title: "Nakagin Capsule Tower Diagram",
    onPieceClick: (piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => diagramFrame(<Diagram {...args} />),
};

export const Controlled: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    designDiff: previewDiff,
    title: "Controlled Diagram",
  },
  render: (args) => {
    const [selection, setSelection] = React.useState({
      pieceGuids: [modifiedPiece.guid],
      connectionGuids: [modifiedConnection.guid],
    });
    const [zoom, setZoom] = React.useState(1);
    const [pan, setPan] = React.useState({ x: 0, y: 0 });

    return diagramFrame(<Diagram {...args} onPanChange={setPan} onSelectionChange={setSelection} onZoomChange={setZoom} pan={pan} selection={selection} zoom={zoom} />);
  },
};

export const FeaturesDisabled: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginCapsuleTowerDesignGuid,
    designDiff: previewDiff,
    diffEnabled: false,
    defaultSelection: {
      pieceGuids: [removedPiece.guid, modifiedPiece.guid, addedPieceGuid],
      connectionGuids: [removedConnection.guid, modifiedConnection.guid, addedConnectionGuid],
    },
    selectionEnabled: false,
    panEnabled: false,
    zoomEnabled: false,
    title: "Features Disabled",
  },
  render: (args) => diagramFrame(<Diagram {...args} />),
};
