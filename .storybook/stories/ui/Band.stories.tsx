// #region 🧲Header

// 🥼︎ semio/js/.storybook/stories/elements/aggregation/Band.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🎩Band
import { Band, Button, Input, Strip, Toggle } from "@ui/react";
import { AddIcon, AwardIcon, DocumentIcon, FolderIcon, LayoutIcon, TypeIcon, UserIcon } from "@semio/assets";
import type { Meta, StoryObj } from "@storybook/react";

const meta = {
  title: "elements/react/Band",
  component: Band,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Band>;

export default meta;

type Story = StoryObj<typeof meta>;

export const HorizontalWithToggles: Story = {
  args: {
    id: "band-horizontal-with-toggles",
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
    ].map((content) => ({ content })),
  },
  render: (args) => (
    <div className="w-[600px]">
      <Band {...args} />
    </div>
  ),
};

export const HorizontalWithMixedElements: Story = {
  args: {
    id: "band-horizontal-mixed",
    items: [
      <Toggle key="1" pressed={true} onPressedChange={() => {}} id="toggle-1" icon={<LayoutIcon className="size-tiny" />} />,
      <Toggle key="2" pressed={false} onPressedChange={() => {}} id="toggle-2" icon={<TypeIcon className="size-tiny" />} />,
      <Input key="3" id="search" placeholder="Search..." className="flex-1 min-w-[160px]" />,
      <Button key="4" id="action" icon={<AddIcon className="size-tiny" />}>
        Add
      </Button>,
    ].map((content) => ({ content })),
  },
  render: (args) => (
    <div className="w-[600px]">
      <Band {...args} />
    </div>
  ),
};

export const VerticalWithToggles: Story = {
  args: {
    id: "band-non-scrollable-with-toggles",
    scrollable: false,
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
    ].map((content) => ({ content })),
  },
  render: (args) => (
    <div className="w-[400px]">
      <Band {...args} />
    </div>
  ),
};

export const OverflowingHorizontal: Story = {
  args: {
    id: "band-overflowing-horizontal",
    items: Array.from({ length: 20 }, (_, i) => <Toggle key={i} pressed={false} onPressedChange={() => {}} id={`toggle-${i}`} icon={`Item ${i + 1}`} />).map((content) => ({ content })),
  },
  render: (args) => (
    <div className="w-[600px]">
      <Band {...args} />
    </div>
  ),
};

export const OverflowingVertical: Story = {
  args: {
    id: "band-overflowing-non-scrollable",
    scrollable: false,
    items: Array.from({ length: 20 }, (_, i) => <Toggle key={i} pressed={false} onPressedChange={() => {}} id={`toggle-${i}`} icon={`Item ${i + 1}`} />).map((content) => ({ content })),
  },
  render: (args) => (
    <div className="w-[600px]">
      <Band {...args} />
    </div>
  ),
};

// #endregion 🎩Band

// 🔷#region 📢Strip
export const StripHorizontal: Story = {
  args: { ...HorizontalWithToggles.args },
  render: () => (
    <div className="w-[600px]">
      <Strip
        id="strip-horizontal"
        items={[
          { content: <Toggle key="designs" pressed={false} onPressedChange={() => {}} id="strip-toggle-1" icon={<LayoutIcon className="size-tiny" />} /> },
          { content: <Toggle key="types" pressed={true} onPressedChange={() => {}} id="strip-toggle-2" icon={<TypeIcon className="size-tiny" />} /> },
          { content: <Input key="search" id="strip-search" placeholder="Search..." className="min-w-[160px]" /> },
          {
            content: (
              <Button key="add" id="strip-add" icon={<AddIcon className="size-tiny" />}>
                Add
              </Button>
            ),
          },
        ]}
      />
    </div>
  ),
};

export const StripNonScrollable: Story = {
  args: { ...HorizontalWithToggles.args },
  render: () => (
    <div className="w-[600px]">
      <Strip
        id="strip-non-scrollable"
        scrollable={false}
        items={[
          { content: <Toggle key="1" pressed={false} onPressedChange={() => {}} id="strip-ns-1" icon={<LayoutIcon className="size-tiny" />} /> },
          { content: <Toggle key="2" pressed={true} onPressedChange={() => {}} id="strip-ns-2" icon={<TypeIcon className="size-tiny" />} /> },
          { content: <span className="text-sm">Fixed layout strip</span> },
        ]}
      />
    </div>
  ),
};
// #endregion 📢Strip
