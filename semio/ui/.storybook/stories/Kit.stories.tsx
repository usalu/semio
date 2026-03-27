// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Kit.stories.tsx
// Specs: One component per stories file with real semio kit data for representative variants.
// Summary: Kit stories: default, controlled, designs-only, types-only, ports-only, selection disabled, data disabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { SemioKit as Kit } from "@semio/ui";
import type { KitProps, KitSelection } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismKit from "../../../assets/semio/kit_metabolism.json";
import type { Kit as SemioKitType } from "@semio/js";

const meta: Meta<typeof Kit> = {
  title: "semio/Kit",
  component: Kit,
  tags: ["autodocs"],
  parameters: { layout: "centered" },
};

export default meta;

type Story = StoryObj<typeof Kit>;

const frame = (node: React.ReactNode) => <div className="w-96 rounded-md border border-border bg-card p-3 text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType },
  render: (args) => frame(<Kit {...args} />),
};

export const Controlled: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType },
  render: (args) => {
    const [selection, setSelection] = React.useState<KitSelection>({ designGuids: [], typeGuids: [], portGuids: [] });
    return frame(<Kit {...args} selection={selection} onSelectionChange={setSelection} />);
  },
};

export const DesignsOnly: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType, typeDataEnabled: false, portDataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const TypesOnly: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType, designDataEnabled: false, portDataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const PortsOnly: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType, designDataEnabled: false, typeDataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const SelectionDisabled: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType, selectionEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const DesignSelectionOnly: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType, typeSelectionEnabled: false, portSelectionEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const DataDisabled: Story = {
  args: { kit: metabolismKit as unknown as SemioKitType, dataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};
