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
import { Avatar, AvatarFallback, AvatarImage, Button, HoverCard, HoverCardContent, HoverCardTrigger } from "../../../../sketchpad/elements";

const HoverCardExamples = () => (
  <div className="space-y-4">
    <HoverCard>
      <HoverCardTrigger asChild>
        <Button variant="ghost" id="hovercard-trigger-default" onClick={() => {}}>
          Kisho Kurokawa
        </Button>
      </HoverCardTrigger>
      <HoverCardContent className="w-80" side="bottom" align="start">
        <div className="flex justify-between space-x-4">
          <Avatar className="h-12 w-12">
            <AvatarImage src="https://github.com/shadcn.png" />
            <AvatarFallback>KK</AvatarFallback>
          </Avatar>
          <div className="space-y-1">
            <h4 className="text-sm font-semibold">Kisho Kurokawa</h4>
            <p className="text-sm">Lead Architect, Metabolism Movement</p>
            <p className="text-sm text-muted-foreground">Pioneer of modular architecture and prefabricated construction systems.</p>
            <div className="flex items-center pt-2">
              <CalendarDays className="mr-2 size-tiny opacity-70" />
              <span className="text-xs text-muted-foreground">Nakagin Tower, 1972</span>
            </div>
          </div>
        </div>
      </HoverCardContent>
    </HoverCard>
    <div className="text-sm">
      Hover over{" "}
      <HoverCard>
        <HoverCardTrigger asChild>
          <span className="underline cursor-pointer">this text</span>
        </HoverCardTrigger>
        <HoverCardContent side="top">
          <p className="text-sm">This is additional information that appears when you hover.</p>
        </HoverCardContent>
      </HoverCard>{" "}
      to see more details.
    </div>
  </div>
);

// #region HoverCard
const meta = {
  title: "Elements/Display/HoverCard",
  component: HoverCardExamples,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof HoverCard>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <HoverCardExamples />,
};

// #endregion HoverCard
