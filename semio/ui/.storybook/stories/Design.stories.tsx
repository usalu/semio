// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Design.stories.tsx
// Specs: One component per stories file with real semio design data for representative variants.
// Summary: Showcases the semio Design split view (Scene + Diagram) with Metabolism kit data.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { type Connection, type Piece } from "@semio/js";
import { SemioDesign as DesignView } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/kit_metabolism.json";

// #region 🔖Design

const nakaginDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const nakaginDesign = (metabolismKit.designs ?? []).find((d) => d.guid === nakaginDesignGuid)!;
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";

const meta: Meta<typeof DesignView> = {
  title: "semio/Design",
  component: DesignView,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof DesignView>;

export const NakaginCapsuleTowerSplitView: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    title: "Nakagin Capsule Tower (Split View)",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection: Connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => (
    <div className="h-96 w-3xl rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

export const DiagramOnly: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    title: "Nakagin (Diagram Only)",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
  },
  render: (args) => (
    <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

export const CustomRatio: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    sceneRatio: 0.7,
    title: "Nakagin Capsule Tower (70% Scene)",
  },
  render: (args) => (
    <div className="h-96 w-3xl rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

export const WithSelection: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    defaultSelection: { pieceGuids: [firstPieceGuid], connectionGuids: [] },
    title: "Nakagin Capsule Tower With Selection",
  },
  render: (args) => (
    <div className="h-96 w-3xl rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

export const NoGridNoGizmo: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    showGrid: false,
    showGizmo: false,
    title: "Nakagin Capsule Tower Without Grid And Gizmo",
  },
  render: (args) => (
    <div className="h-96 w-3xl rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

// #endregion 🔖Design
