// #region Header

// Strip.stories.tsx

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

import { AddIcon, AwardIcon, DocumentIcon, FolderIcon, LayoutIcon, TypeIcon, UserIcon } from "@semio/assets";
import type { Meta, StoryObj } from "@storybook/react";
import { Button, Input, Strip, Toggle } from "../../../../sketchpad/elements";

const meta = {
  title: "Elements/Aggregation/Strip",
  component: Strip,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {
    direction: {
      control: "radio",
      options: ["horizontal", "vertical"],
      description: "Direction of the strip",
    },
  },
} satisfies Meta<typeof Strip>;

export default meta;

type Story = StoryObj<typeof meta>;

export const HorizontalWithToggles: Story = {
  args: {
    direction: "horizontal",
    items: [
      <Toggle
        key="designs"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showDesigns"
        actionId="semio.sketchpad.app.kit.kitApp.createDesign"
        icon={<LayoutIcon className="size-tiny" />}
      />,
      <Toggle
        key="types"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showTypes"
        actionId="semio.sketchpad.app.kit.kitApp.createType"
        icon={<TypeIcon className="size-tiny" />}
      />,
      <Toggle
        key="qualities"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showQualities"
        actionId="semio.sketchpad.app.kit.kitApp.createQuality"
        icon={<AwardIcon className="size-tiny" />}
      />,
      <Toggle
        key="files"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showFiles"
        actionId="semio.sketchpad.app.kit.kitApp.createFile"
        icon={<DocumentIcon className="size-tiny" />}
      />,
      <Toggle
        key="folders"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showFolders"
        actionId="semio.sketchpad.app.kit.kitApp.createFolder"
        icon={<FolderIcon className="size-tiny" />}
      />,
      <Toggle
        key="authors"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showAuthors"
        actionId="semio.sketchpad.app.kit.kitApp.createAuthor"
        icon={<UserIcon className="size-tiny" />}
      />,
    ],
  },
  render: (args) => (
    <div className="w-[600px]">
      <Strip {...args} />
    </div>
  ),
};

export const HorizontalWithMixedElements: Story = {
  args: {
    direction: "horizontal",
    items: [
      <Toggle key="1" pressed={true} onPressedChange={() => {}} id="toggle-1" icon={<LayoutIcon className="size-tiny" />} />,
      <Toggle key="2" pressed={false} onPressedChange={() => {}} id="toggle-2" icon={<TypeIcon className="size-tiny" />} />,
      <Input key="3" id="search" placeholder="Search..." className="flex-1 min-w-[160px]" />,
      <Button key="4" id="action" icon={<AddIcon className="size-tiny" />}>
        Add
      </Button>,
    ],
  },
  render: (args) => (
    <div className="w-[600px]">
      <Strip {...args} />
    </div>
  ),
};

export const VerticalWithToggles: Story = {
  args: {
    direction: "vertical",
    items: [
      <Toggle
        key="designs"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showDesigns"
        actionId="semio.sketchpad.app.kit.kitApp.createDesign"
        icon={<LayoutIcon className="size-tiny" />}
      />,
      <Toggle
        key="types"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showTypes"
        actionId="semio.sketchpad.app.kit.kitApp.createType"
        icon={<TypeIcon className="size-tiny" />}
      />,
      <Toggle
        key="qualities"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showQualities"
        actionId="semio.sketchpad.app.kit.kitApp.createQuality"
        icon={<AwardIcon className="size-tiny" />}
      />,
      <Toggle
        key="files"
        kind="withAction"
        pressed={false}
        onPressedChange={() => {}}
        actionIcon={<AddIcon className="size-tiny" />}
        onActionClick={() => {}}
        id="semio.sketchpad.app.kit.kitApp.showFiles"
        actionId="semio.sketchpad.app.kit.kitApp.createFile"
        icon={<DocumentIcon className="size-tiny" />}
      />,
    ],
  },
  render: (args) => (
    <div className="h-[400px]">
      <Strip {...args} />
    </div>
  ),
};

export const OverflowingHorizontal: Story = {
  args: {
    direction: "horizontal",
    items: Array.from({ length: 20 }, (_, i) => <Toggle key={i} pressed={false} onPressedChange={() => {}} id={`toggle-${i}`} icon={`Item ${i + 1}`} />),
  },
  render: (args) => (
    <div className="w-[600px]">
      <Strip {...args} />
    </div>
  ),
};

export const OverflowingVertical: Story = {
  args: {
    direction: "vertical",
    items: Array.from({ length: 20 }, (_, i) => <Toggle key={i} pressed={false} onPressedChange={() => {}} id={`toggle-${i}`} icon={`Item ${i + 1}`} />),
  },
  render: (args) => (
    <div className="h-[400px]">
      <Strip {...args} />
    </div>
  ),
};
