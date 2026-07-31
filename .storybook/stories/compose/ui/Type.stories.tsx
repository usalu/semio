// #region 🧲️Header
// 💻️ compose/ui/.storybook/story/Type.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Uses a type prop directly. Kit is optional for 3D representations.
// Summary: Type stories: Default, ConnectorsOnly, RepresentationOnly, Selection, FeaturesDisabled.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { ConnectorGraphDto as Connector, Kit, Type as ComposeKind } from "@semio-tech/compose-react";
import { ComposeType as TypeView } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import { MetabolismKit as metabolismKit } from "@semio-tech/asset";

// #region 🖥️Data

const rawKind = (metabolismKit.types ?? []).find((kind) => kind.name === "Tambour")! as ComposeKind;
const storyKind: ComposeKind = {
  ...rawKind,
  representations: (rawKind.representations ?? []).slice(0, 1),
} as ComposeKind;

const usedFileIds = new Set((storyKind.representations ?? []).map((representation) => representation.file?.id).filter(Boolean));
const minimalKit: Kit = {
  id: (metabolismKit as any).id,
  name: (metabolismKit as any).name,
  types: [storyKind],
  files: (metabolismKit.files ?? []).filter((file: any) => usedFileIds.has(file.id)),
} as Kit;

const firstConnectorId = (storyKind.connectors ?? [])[0]?.id ?? "";

// #endregion 🖥️Data

// #region 🧱️Type

const meta: Meta<typeof TypeView> = {
  title: "🏘️compose⚛️react/Type",
  component: TypeView,
  tags: ["autodocs"],
  parameters: { layout: "centered" },
};

export default meta;

type Story = StoryObj<typeof TypeView>;

const frame = (node: React.ReactNode) => <div className="h-96 w-96 rounded-md border border-border bg-card text-foreground shadow-sm">{node}</div>;

export const Default: Story = {
  args: {
    type: storyKind,
    kit: minimalKit,
    defaultSelection: { connectorIds: firstConnectorId ? [firstConnectorId] : [] },
    title: "Type",
    onConnectorClick: (connector: Connector) => console.info("Connector clicked", connector.id),
  },
  render: (args) => frame(<TypeView {...args} />),
};

export const ConnectorsOnly: Story = {
  args: {
    type: storyKind,
    showRepresentation: false,
    title: "Connectors Only",
    defaultSelection: { connectorIds: firstConnectorId ? [firstConnectorId] : [] },
  },
  render: (args) => frame(<TypeView {...args} />),
};

export const RepresentationOnly: Story = {
  args: {
    type: storyKind,
    kit: minimalKit,
    showConnectors: false,
    title: "Representation Only",
  },
  render: (args) => frame(<TypeView {...args} />),
};

export const Selection: Story = {
  args: {
    type: storyKind,
    kit: minimalKit,
    defaultSelection: { connectorIds: firstConnectorId ? [firstConnectorId] : [] },
    hoverEnabled: true,
    title: "Selection",
  },
  render: (args) => frame(<TypeView {...args} />),
};

export const FeaturesDisabled: Story = {
  args: {
    type: storyKind,
    kit: minimalKit,
    selectionEnabled: false,
    hoverEnabled: false,
    showGrid: false,
    showGizmo: false,
    title: "Features Disabled",
  },
  render: (args) => frame(<TypeView {...args} />),
};

// #endregion 🧱️Type
