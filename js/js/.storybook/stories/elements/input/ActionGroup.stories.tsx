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
import { Copy, Download, ExternalLink } from "lucide-react";
import { ActionGroup, ActionGroupItem, Level, LevelProvider, getLevelBgClass } from "../../../../sketchpad/elements";

// #region ActionGroup
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

const ActionGroupDemo = () => (
  <ActionGroup id="action-group-demo">
    <ActionGroupItem id="action-group-demo-copy">
      <Copy />
    </ActionGroupItem>
    <ActionGroupItem id="action-group-demo-download">
      <Download />
    </ActionGroupItem>
    <ActionGroupItem id="action-group-demo-external">
      <ExternalLink />
    </ActionGroupItem>
  </ActionGroup>
);

export const Default: Story = {
  render: () => <ActionGroupDemo />,
};

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <ActionGroupDemo />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  render: createLevelRender("base"),
};

export const Window: Story = {
  render: createLevelRender("window"),
};

export const Panel: Story = {
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  render: createLevelRender("temporary"),
};

// #endregion ActionGroup
