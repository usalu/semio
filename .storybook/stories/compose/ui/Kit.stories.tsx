// #region 🧲️Header
// 💻️ compose/ui/.storybook/story/Kit.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup.
// Summary: Default uses full Metabolism kit (GLTF blobs in files) so embedded viewers render; ShallowKitArtifactsOnly keeps shallow JSON for metadata-only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Kit as ComposeKit } from "@semio-tech/compose-react";
import { MetabolismKit as metabolismFullKit } from "@semio-tech/asset";
import { MetabolismShallowKit as metabolismShallowKit } from "@semio-tech/compose-fixture";
import { ComposeKit as Kit } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

// #region 🖥️Data

const kit = metabolismFullKit as unknown as ComposeKit;
const shallowKit = metabolismShallowKit as unknown as ComposeKit;

// #endregion 🖥️Data

// #region ⏱️Kit

const meta: Meta<typeof Kit> = {
  title: "🏘️compose⚛️react/Kit",
  component: Kit,
  tags: ["autodocs"],
  parameters: { layout: "padded" },
};

export default meta;

type Story = StoryObj<typeof Kit>;

const frame = (node: React.ReactNode) => <div className="min-h-[420px] w-full max-w-5xl min-w-[20rem] rounded-md border border-border bg-card p-3 text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: { kit },
  render: (args) => frame(<Kit {...args} />),
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

/** Shallow kit has no `files` / embedded GLTF — viewers fall back to placeholders; use for document/metadata-only checks. */
export const ShallowKitArtifactsOnly: Story = {
  args: { kit: shallowKit },
  render: (args) => frame(<Kit {...args} />),
};

export const OpenArtifact: Story = {
  args: { kit },
  render: (args) => {
    const [lastOpened, setLastOpened] = React.useState("Nothing opened yet");
    return frame(
      <div className="grid gap-3">
        <Kit {...args} onOpenArtifact={(artifact) => setLastOpened(`Opened ${artifact.kind} ${artifact.label}`)} />
        <div className="text-xs text-muted-foreground">{lastOpened}</div>
      </div>,
    );
  },
};

// #endregion ⏱️Kit
