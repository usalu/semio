// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Scene.stories.tsx
// Specs: One component per stories file with real semio design data for representative variants.
// Summary: Showcases the semio 3D Scene with Metabolism kit Flat design data.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Piece } from "@semio/js";
import { SemioScene as Scene } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/kit_metabolism.json";

// #region 🔖Scene

const flatDesignGuid = "79fa8945-b47d-4896-965f-f921067cbae2";
const flatDesign = (metabolismKit.designs ?? []).find((d) => d.guid === flatDesignGuid)!;
const firstPieceGuid = (flatDesign.pieces ?? [])[0]?.guid ?? "";

const meta: Meta<typeof Scene> = {
  title: "semio/Scene",
  component: Scene,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
  },
};

export default meta;

type Story = StoryObj<typeof Scene>;

export const FlatDesign: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    title: "Flat Design Scene",
    onPieceClick: (piece: Piece) => console.info("Piece clicked", piece.guid),
  },
  render: (args) => (
    <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">
      <Scene {...args} />
    </div>
  ),
};

export const WithSelection: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    defaultSelection: { pieceGuids: [firstPieceGuid] },
    title: "Scene With Selection",
  },
  render: (args) => (
    <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">
      <Scene {...args} />
    </div>
  ),
};

export const NoGridNoGizmo: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    showGrid: false,
    showGizmo: false,
    title: "Scene Without Grid And Gizmo",
  },
  render: (args) => (
    <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">
      <Scene {...args} />
    </div>
  ),
};

export const SelectionDisabled: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    selectionEnabled: false,
    title: "Scene Selection Disabled",
  },
  render: (args) => (
    <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">
      <Scene {...args} />
    </div>
  ),
};

export const Controlled: Story = {
  args: {
    kit: metabolismKit,
    designGuid: flatDesignGuid,
    title: "Controlled Scene",
  },
  render: (args) => {
    const [selection, setSelection] = React.useState({ pieceGuids: [firstPieceGuid] });
    return (
      <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">
        <Scene {...args} selection={selection} onSelectionChange={setSelection} />
      </div>
    );
  },
};

// #endregion 🔖Scene
