// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/display/Avatar.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Avatar, AvatarFallback, AvatarImage, DraggableAvatar, Level, LevelProvider, TableAvatar, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖Avatar
const meta = {
  title: "Elements/Display/Avatar",
  component: Avatar,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Avatar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <div className="flex items-center gap-4">
      <div className="flex items-center gap-3">
        <Avatar className="h-12 w-12">
          <AvatarImage src="https://github.com/shadcn.png" alt="Kisho Kurokawa" />
          <AvatarFallback className="bg-accent text-accent-foreground">KK</AvatarFallback>
        </Avatar>
        <div className="flex flex-col">
          <span className="text-sm font-medium">Kisho Kurokawa</span>
          <span className="text-xs text-muted-foreground">Lead Architect</span>
        </div>
      </div>
      <div className="relative">
        <Avatar className="h-10 w-10">
          <AvatarImage src="https://invalid-url.com/image.png" alt="Designer" />
          <AvatarFallback className="bg-red-500 text-white">DS</AvatarFallback>
        </Avatar>
        <span className="absolute bottom-0 right-0 size-tiny rounded-full bg-green-500 border-2 border-background"></span>
      </div>
      <div className="flex -space-x-2">
        <Avatar className="border-2 border-background h-small w-8">
          <AvatarFallback className="text-xs">AR</AvatarFallback>
        </Avatar>
        <Avatar className="border-2 border-background h-small w-8">
          <AvatarFallback className="text-xs">EN</AvatarFallback>
        </Avatar>
        <Avatar className="border-2 border-background h-small w-8">
          <AvatarFallback className="text-xs">DS</AvatarFallback>
        </Avatar>
      </div>
    </div>
  ),
};

const AvatarDemo = () => (
  <div className="flex items-center gap-4">
    <div className="flex items-center gap-3">
      <Avatar className="h-12 w-12">
        <AvatarImage src="https://github.com/shadcn.png" alt="Kisho Kurokawa" />
        <AvatarFallback className="bg-accent text-accent-foreground">KK</AvatarFallback>
      </Avatar>
      <div className="flex flex-col">
        <span className="text-sm font-medium">Kisho Kurokawa</span>
        <span className="text-xs text-muted-foreground">Lead Architect</span>
      </div>
    </div>
    <div className="relative">
      <Avatar className="h-10 w-10">
        <AvatarImage src="https://invalid-url.com/image.png" alt="Designer" />
        <AvatarFallback className="bg-red-500 text-white">DS</AvatarFallback>
      </Avatar>
      <span className="absolute bottom-0 right-0 size-tiny rounded-full bg-green-500 border-2 border-background"></span>
    </div>
    <div className="flex -space-x-2">
      <Avatar className="border-2 border-background h-small w-8">
        <AvatarFallback className="text-xs">AR</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-background h-small w-8">
        <AvatarFallback className="text-xs">EN</AvatarFallback>
      </Avatar>
      <Avatar className="border-2 border-background h-small w-8">
        <AvatarFallback className="text-xs">DS</AvatarFallback>
      </Avatar>
    </div>
  </div>
);

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <AvatarDemo />
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

// #endregion 🔖Avatar

// #region 🔖DraggableAvatar
export const DraggableAvatarDefault: Story = {
  render: () => (
    <div className="flex items-center gap-4">
      <DraggableAvatar content="KK" title="Kisho Kurokawa" />
      <DraggableAvatar content="KT" title="Kenzo Tange" isSelected />
      <DraggableAvatar content="FM" title="Fumihiko Maki" isHovered />
      <DraggableAvatar content="AI" title="Arata Isozaki" shouldFade />
    </div>
  ),
};
// #endregion 🔖DraggableAvatar

// #region 🔖TableAvatar
export const TableAvatarDefault: Story = {
  render: () => (
    <div className="flex items-center gap-4">
      <TableAvatar name="Kisho Kurokawa" icon="https://github.com/shadcn.png" />
      <TableAvatar name="Kenzo Tange" />
      <TableAvatar name="Fumihiko Maki" isSelected />
      <TableAvatar name="Arata Isozaki" isHovered />
    </div>
  ),
};
// #endregion 🔖TableAvatar
