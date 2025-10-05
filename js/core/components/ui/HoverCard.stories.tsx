// #region Header

// HoverCard.stories.tsx

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
import { CalendarDays } from "lucide-react";
import { Avatar, AvatarFallback, AvatarImage } from "./Avatar";
import { Button } from "./Button";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "./HoverCard";

const meta = {
  title: "Elements/HoverCard",
  component: HoverCard,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof HoverCard>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Basic: Story = {
  render: () => (
    <HoverCard>
      <HoverCardTrigger asChild>
        <Button variant="link">@semio</Button>
      </HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="flex justify-between space-x-4">
          <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" />
            <AvatarFallback>SM</AvatarFallback>
          </Avatar>
          <div className="space-y-1">
            <h4 className="text-sm font-semibold">@semio</h4>
            <p className="text-sm">Parametric design and engineering platform</p>
            <div className="flex items-center pt-2">
              <CalendarDays className="mr-2 h-4 w-4 opacity-70" />
              <span className="text-xs text-muted-foreground">Joined January 2025</span>
            </div>
          </div>
        </div>
      </HoverCardContent>
    </HoverCard>
  ),
};

export const OnText: Story = {
  render: () => (
    <div className="text-sm">
      Hover over{" "}
      <HoverCard>
        <HoverCardTrigger asChild>
          <span className="underline cursor-pointer">this text</span>
        </HoverCardTrigger>
        <HoverCardContent>
          <p>This is additional information that appears when you hover.</p>
        </HoverCardContent>
      </HoverCard>{" "}
      to see more details.
    </div>
  ),
};

export const WithoutAvatar: Story = {
  render: () => (
    <HoverCard>
      <HoverCardTrigger asChild>
        <Button variant="outline">Hover for info</Button>
      </HoverCardTrigger>
      <HoverCardContent>
        <div className="space-y-2">
          <h4 className="text-sm font-semibold">Feature Information</h4>
          <p className="text-sm">This feature allows you to create parametric designs with real-time collaboration.</p>
        </div>
      </HoverCardContent>
    </HoverCard>
  ),
};

export const DifferentSides: Story = {
  render: () => (
    <div className="flex gap-4">
      <HoverCard>
        <HoverCardTrigger asChild>
          <Button variant="outline">Top</Button>
        </HoverCardTrigger>
        <HoverCardContent side="top">
          <p className="text-sm">Content appears on top</p>
        </HoverCardContent>
      </HoverCard>

      <HoverCard>
        <HoverCardTrigger asChild>
          <Button variant="outline">Right</Button>
        </HoverCardTrigger>
        <HoverCardContent side="right">
          <p className="text-sm">Content appears on right</p>
        </HoverCardContent>
      </HoverCard>

      <HoverCard>
        <HoverCardTrigger asChild>
          <Button variant="outline">Bottom</Button>
        </HoverCardTrigger>
        <HoverCardContent side="bottom">
          <p className="text-sm">Content appears on bottom</p>
        </HoverCardContent>
      </HoverCard>

      <HoverCard>
        <HoverCardTrigger asChild>
          <Button variant="outline">Left</Button>
        </HoverCardTrigger>
        <HoverCardContent side="left">
          <p className="text-sm">Content appears on left</p>
        </HoverCardContent>
      </HoverCard>
    </div>
  ),
};
