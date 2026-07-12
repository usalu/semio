// #region 🧲Header

// .elements/ui/.storybook/story/elements/Toolbar.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { ButtonGroup, ButtonGroupItem, Ribbon, ToggleGroup, ToolbarDivider, ToolbarGroup, ToolbarItem, ToolbarZone, type RibbonRow } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🌙Toolbar

const Eye = createIconComponent("eye");
const EyeOff = createIconComponent("eye-off");
const Hand = createIconComponent("hand");
const Maximize2 = createIconComponent("maximize2");
const MousePointer = createIconComponent("mouse-pointer");
const Move = createIconComponent("move");
const RotateCcw = createIconComponent("rotate-ccw");
const RotateCw = createIconComponent("rotate-cw");
const ZoomIn = createIconComponent("zoom-in");
const ZoomOut = createIconComponent("zoom-out");

const meta = {
  title: "🖱️ui⚛️react/Toolbar",
  component: ToolbarZone,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToolbarZone>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: null,
  },
  render: () => (
    <ToolbarZone>
      <ToolbarGroup>
        <ToolbarItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Move className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </ToolbarItem>
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <ToolbarItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<ZoomIn className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<ZoomOut className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Maximize2 className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </ToolbarItem>
      </ToolbarGroup>
    </ToolbarZone>
  ),
};

export const WithUndoRedo: Story = {
  args: { children: null },
  render: () => (
    <ToolbarZone>
      <ToolbarGroup>
        <ToolbarItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<RotateCcw className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<RotateCw className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </ToolbarItem>
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <ToolbarItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </ToolbarItem>
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <ToolbarItem>
          <ToggleGroup
            kind="multiple"
            defaultValue={["visible"]}
            items={[
              { value: "visible", icon: <Eye className="size-tiny" aria-hidden /> },
              { value: "hidden", icon: <EyeOff className="size-tiny" aria-hidden /> },
            ]}
          />
        </ToolbarItem>
      </ToolbarGroup>
    </ToolbarZone>
  ),
};

export const MultipleZones: Story = {
  args: { children: null },
  render: () => (
    <div className="flex gap-single items-stretch">
      <ToolbarZone>
        <ToolbarGroup>
          <ToolbarItem>
            <ToggleGroup
              kind="single"
              defaultValue="selection"
              items={[
                { value: "selection", id: "ui.toolbar.group.selection", icon: <MousePointer className="size-tiny" aria-hidden />, text: "Selection" },
                { value: "view", id: "ui.toolbar.group.view", icon: <Eye className="size-tiny" aria-hidden />, text: "View" },
              ]}
            />
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarZone>
      <ToolbarZone>
        <ToolbarGroup>
          <ToolbarItem>
            <ButtonGroup>
              <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
              <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
            </ButtonGroup>
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarZone>
      <ToolbarZone>
        <ToolbarGroup>
          <ToolbarItem>
            <ButtonGroup>
              <ButtonGroupItem icon={<ZoomIn className="size-tiny" aria-hidden />} />
              <ButtonGroupItem icon={<ZoomOut className="size-tiny" aria-hidden />} />
            </ButtonGroup>
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarZone>
    </div>
  ),
};

export const RibbonLevels: Story = {
  name: "Ribbon Levels (inline vs. up)",
  args: { children: null },
  render: () => {
    const rows: RibbonRow[] = [
      {
        key: "base",
        content: (
          <ToolbarZone>
            <ToolbarGroup>
              <ToolbarItem>
                <ButtonGroup>
                  <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
                  <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
                </ButtonGroup>
              </ToolbarItem>
            </ToolbarGroup>
          </ToolbarZone>
        ),
      },
      {
        key: "nested",
        content: (
          <ToolbarZone>
            <ToolbarGroup>
              <ToolbarItem>
                <ButtonGroup>
                  <ButtonGroupItem icon={<ZoomIn className="size-tiny" aria-hidden />} />
                  <ButtonGroupItem icon={<ZoomOut className="size-tiny" aria-hidden />} />
                </ButtonGroup>
              </ToolbarItem>
            </ToolbarGroup>
          </ToolbarZone>
        ),
      },
    ];
    return (
      <div className="flex items-end gap-double">
        <div className="flex flex-col items-center gap-single">
          <span className="text-xs text-muted-foreground">inline (footer)</span>
          <Ribbon direction="inline" rows={rows} />
        </div>
        <div className="flex flex-col items-center gap-single">
          <span className="text-xs text-muted-foreground">up (window toolbar)</span>
          <Ribbon direction="up" rows={rows} />
        </div>
      </div>
    );
  },
};

// #endregion 🌙Toolbar
