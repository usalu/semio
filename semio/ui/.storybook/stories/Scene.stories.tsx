// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Scene.stories.tsx
// Specs: One component per stories file with real semio design data for representative variants.
// Summary: Showcases the semio 3D Scene with Metabolism kit Nakagin Capsule Tower design data.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Piece } from "@semio/js";
import { SemioScene as Scene } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/kit_metabolism.json";

// #region 🔖Scene

const nakaginDesignGuid = "9a890dd4-0a9c-48ac-920a-9e62666465ef";
const nakaginDesign = (metabolismKit.designs ?? []).find((d) => d.guid === nakaginDesignGuid)!;
const firstPieceGuid = (nakaginDesign.pieces ?? [])[0]?.guid ?? "";

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

export const NakaginCapsuleTower: Story = {
  args: {
    kit: metabolismKit,
    designGuid: nakaginDesignGuid,
    title: "Nakagin Capsule Tower Scene",
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
    designGuid: nakaginDesignGuid,
    defaultSelection: { pieceGuids: [firstPieceGuid] },
    title: "Nakagin Capsule Tower With Selection",
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
    designGuid: nakaginDesignGuid,
    showGrid: false,
    showGizmo: false,
    title: "Nakagin Capsule Tower Without Grid And Gizmo",
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
    designGuid: nakaginDesignGuid,
    selectionEnabled: false,
    title: "Nakagin Capsule Tower Selection Disabled",
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
    designGuid: nakaginDesignGuid,
    title: "Controlled Nakagin Capsule Tower",
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
