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

const flatDesignGuid = "79fa8945-b47d-4896-965f-f921067cbae2";
const flatDesign = (metabolismKit.designs ?? []).find((d) => d.guid === flatDesignGuid)!;
const firstPieceGuid = (flatDesign.pieces ?? [])[0]?.guid ?? "";

const nakaginDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

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

export const FlatDesignSplitView: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    title: "Flat Design (Split View)",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
    onConnectionClick: (connection: Connection) => console.info("Connection clicked", connection.guid),
  },
  render: (args) => (
    <div className="h-96 w-[48rem] rounded-md border border-border bg-card text-foreground shadow-sm">
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
    designGuid: flatDesignGuid,
    sceneRatio: 0.7,
    title: "Custom Ratio (70% Scene)",
  },
  render: (args) => (
    <div className="h-96 w-[48rem] rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

export const WithSelection: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    defaultSelection: { pieceGuids: [firstPieceGuid], connectionGuids: [] },
    title: "Design With Selection",
  },
  render: (args) => (
    <div className="h-96 w-[48rem] rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

export const NoGridNoGizmo: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    showGrid: false,
    showGizmo: false,
    title: "Design Without Grid And Gizmo",
  },
  render: (args) => (
    <div className="h-96 w-[48rem] rounded-md border border-border bg-card text-foreground shadow-sm">
      <DesignView {...args} />
    </div>
  ),
};

// #endregion 🔖Design
