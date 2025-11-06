// #region Header

// ActionGroup.stories.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import type { Meta, StoryObj } from "@storybook/react";
import { Copy, Download, ExternalLink, Maximize2, X } from "lucide-react";

import { ActionGroup, ActionGroupItem } from "./ActionGroup";

const meta = {
  title: "Elements/Input/ActionGroup",
  component: ActionGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ActionGroup>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <ActionGroup id="action-group-default">
      <ActionGroupItem id="action-group-default-copy">
        <Copy />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-default-download">
        <Download />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-default-external">
        <ExternalLink />
      </ActionGroupItem>
    </ActionGroup>
  ),
};

export const WindowControls: Story = {
  render: () => (
    <ActionGroup id="action-group-window-controls">
      <ActionGroupItem id="action-group-window-controls-external">
        <ExternalLink />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-window-controls-maximize">
        <Maximize2 />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-window-controls-close">
        <X />
      </ActionGroupItem>
    </ActionGroup>
  ),
};

export const WithDestructive: Story = {
  render: () => (
    <ActionGroup id="action-group-destructive">
      <ActionGroupItem id="action-group-destructive-copy">
        <Copy />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-destructive-download">
        <Download />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-destructive-delete" variant="destructive">
        <X />
      </ActionGroupItem>
    </ActionGroup>
  ),
};

export const WithLabel: Story = {
  render: () => (
    <ActionGroup id="action-group-with-label" showLabel>
      <ActionGroupItem id="action-group-with-label-copy">
        <Copy />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-with-label-download">
        <Download />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-with-label-external">
        <ExternalLink />
      </ActionGroupItem>
    </ActionGroup>
  ),
};

export const LevelBase: Story = {
  render: () => (
    <div className="flex flex-col gap-4 bg-background-base p-4">
      <ActionGroup id="action-group-level-base" level="base">
        <ActionGroupItem id="action-group-level-base-copy">
          <Copy />
        </ActionGroupItem>
        <ActionGroupItem id="action-group-level-base-download">
          <Download />
        </ActionGroupItem>
        <ActionGroupItem id="action-group-level-base-external">
          <ExternalLink />
        </ActionGroupItem>
      </ActionGroup>
    </div>
  ),
};

export const LevelPanel: Story = {
  render: () => (
    <div className="flex flex-col gap-4 bg-background-panel p-4">
      <ActionGroup id="action-group-level-panel" level="panel">
        <ActionGroupItem id="action-group-level-panel-copy">
          <Copy />
        </ActionGroupItem>
        <ActionGroupItem id="action-group-level-panel-download">
          <Download />
        </ActionGroupItem>
        <ActionGroupItem id="action-group-level-panel-external">
          <ExternalLink />
        </ActionGroupItem>
      </ActionGroup>
    </div>
  ),
};

export const LevelTemporary: Story = {
  render: () => (
    <div className="flex flex-col gap-4 bg-background-temporary p-4">
      <ActionGroup id="action-group-level-temporary" level="temporary">
        <ActionGroupItem id="action-group-level-temporary-copy">
          <Copy />
        </ActionGroupItem>
        <ActionGroupItem id="action-group-level-temporary-download">
          <Download />
        </ActionGroupItem>
        <ActionGroupItem id="action-group-level-temporary-external">
          <ExternalLink />
        </ActionGroupItem>
      </ActionGroup>
    </div>
  ),
};
