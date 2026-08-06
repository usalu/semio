// #region 🧲️Header

// 🥼️ .storybook/stories/ui/🖼️IconShotFrame.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { clipIconSvgMarkupToEllipse, IconShotFrame, iconShotFrameStyle } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

// 🖼️#region 🔖️IconShotFrame
const meta = {
  title: "🖱️ui⚛️react/IconShotFrame",
  component: IconShotFrame,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof IconShotFrame>;

export default meta;

type Story = StoryObj<typeof meta>;

const ShotBackdrop = () => <div className="size-full bg-gradient-to-br from-accent to-accent-secondary" />;

export const Default: Story = {
  args: {
    width: 256,
    height: 256,
    shape: "rectangle",
    children: <ShotBackdrop />,
  },
  render: (args) => (
    <div className="relative size-40 border">
      <IconShotFrame {...args} />
    </div>
  ),
};

export const EllipseShape: Story = {
  name: "Ellipse Shape",
  args: {
    width: 256,
    height: 256,
    shape: "ellipse",
    children: <ShotBackdrop />,
  },
  render: (args) => (
    <div className="relative size-40 border">
      <IconShotFrame {...args} />
    </div>
  ),
};

export const LandscapeNoBadge: Story = {
  name: "Landscape, No Badge",
  args: {
    width: 512,
    height: 256,
    shape: "rectangle",
    badge: false,
    background: "var(--muted)",
    children: <ShotBackdrop />,
  },
  render: (args) => (
    <div className="relative h-32 w-56 border">
      <IconShotFrame {...args} />
    </div>
  ),
};

// #endregion 🔖️IconShotFrame

// #region ⭕️clipIconSvgMarkupToEllipse
const sampleSvgMarkup = `<svg width="120" height="120" viewBox="0 0 120 120" xmlns="http://www.w3.org/2000/svg"><rect width="120" height="120" fill="var(--accent)"/><rect x="10" y="10" width="100" height="100" fill="var(--accent-secondary)"/></svg>`;
const clippedSvgMarkup = clipIconSvgMarkupToEllipse(sampleSvgMarkup, 120, 120);

export const ClipIconSvgMarkupToEllipseStory: Story = {
  name: "clipIconSvgMarkupToEllipse",
  args: { width: 0, height: 0, children: null },
  render: () => (
    <div className="flex items-center gap-8">
      <div className="flex flex-col items-center gap-2">
        <p className="text-xs text-muted-foreground">Source markup</p>
        <div className="size-30" dangerouslySetInnerHTML={{ __html: sampleSvgMarkup }} />
      </div>
      <div className="flex flex-col items-center gap-2">
        <p className="text-xs text-muted-foreground">Clipped to ellipse</p>
        <div className="size-30" dangerouslySetInnerHTML={{ __html: clippedSvgMarkup }} />
      </div>
    </div>
  ),
};

// #endregion ⭕️clipIconSvgMarkupToEllipse

// #region 🖼️iconShotFrameStyle
export const IconShotFrameStyleStory: Story = {
  name: "iconShotFrameStyle",
  args: { width: 0, height: 0, children: null },
  render: () => {
    const portraitStyle = iconShotFrameStyle(9, 16);
    const landscapeStyle = iconShotFrameStyle(16, 9);
    return (
      <div className="flex items-center gap-8 text-xs">
        <div className="flex flex-col items-center gap-2">
          <div className="flex h-32 w-32 items-center justify-center border">
            <div style={portraitStyle} className="bg-accent" />
          </div>
          <p className="text-muted-foreground">portrait 9×16 → {JSON.stringify(portraitStyle)}</p>
        </div>
        <div className="flex flex-col items-center gap-2">
          <div className="flex h-32 w-32 items-center justify-center border">
            <div style={landscapeStyle} className="bg-accent" />
          </div>
          <p className="text-muted-foreground">landscape 16×9 → {JSON.stringify(landscapeStyle)}</p>
        </div>
      </div>
    );
  },
};

// #endregion 🖼️iconShotFrameStyle
