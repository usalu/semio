// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/display/Tooltip.stories.tsx

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

import { Button, Level, LevelProvider, Tooltip, TooltipContent, TooltipTrigger, getLevelBgClass } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { Plus, Settings, Trash2 } from "lucide-react";

const TooltipExamples = () => (
  <div className="space-y-4">
    <div className="flex gap-double">
      <Tooltip>
        <TooltipTrigger asChild>
          <Button id="tooltip-trigger-default">
            <Plus />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">
          <p>Add new capsule instance to the design</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="default" onClick={() => {}}>
            <Settings />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="right">
          Settings <span className="text-xs ml-1 opacity-60">(Ctrl+,)</span>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline" onClick={() => {}}>
            <Trash2 />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="top">
          <p>Delete selected item</p>
        </TooltipContent>
      </Tooltip>
    </div>
    <div className="text-sm">
      This is some text with{" "}
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="underline cursor-help">a tooltip</span>
        </TooltipTrigger>
        <TooltipContent side="left">
          <p>Additional information</p>
        </TooltipContent>
      </Tooltip>{" "}
      inline.
    </div>
  </div>
);

// #region 🔖Tooltip
const meta = {
  title: "semio-elements/Tooltip",
  component: TooltipExamples,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tooltip>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <TooltipExamples />,
};

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <TooltipExamples />
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

// #endregion 🔖Tooltip
