// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Kit.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses the shallow kit prop directly.
// Summary: Kit stories: Default, Controlled, DesignsOnly, TypesOnly, PortsOnly, SelectionDisabled, DataDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Kit as SemioKit } from "@semio/js";
import type { KitSelection } from "@semio/ui";
import { SemioKit as Kit } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import metabolismShallowKit from "../../../assets/semio/metabolism.shallow.kit.semio.json";

// #region 🔖Data

const kit = metabolismShallowKit as unknown as SemioKit;

// #endregion 🔖Data

// #region 🔖Kit

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
  args: { kit },
  render: (args) => frame(<Kit {...args} />),
};

export const Controlled: Story = {
  args: { kit },
  render: (args) => {
    const [selection, setSelection] = React.useState<KitSelection>({ designGuids: [], typeGuids: [], portGuids: [] });
    return frame(<Kit {...args} selection={selection} onSelectionChange={setSelection} />);
  },
};

export const DesignsOnly: Story = {
  args: { kit, typeDataEnabled: false, portDataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const TypesOnly: Story = {
  args: { kit, designDataEnabled: false, portDataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const PortsOnly: Story = {
  args: { kit, designDataEnabled: false, typeDataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const SelectionDisabled: Story = {
  args: { kit, selectionEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

export const DataDisabled: Story = {
  args: { kit, dataEnabled: false },
  render: (args) => frame(<Kit {...args} />),
};

// #endregion 🔖Kit
