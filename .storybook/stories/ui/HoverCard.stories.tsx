// #region 🧲Header

// 🥼︎ compose/js/.storybook/story/elements/display/HoverCard.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Aside, Avatar, AvatarFallback, AvatarImage, Button, Card, CardGrid, HoverCard, HoverCardContent, HoverCardTrigger } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";

const HoverCardExamples = () => (
  <div className="space-y-4">
    <HoverCard>
      <HoverCardTrigger asChild>
        <Button variant="ghost" id="hovercard-trigger-default" onClick={() => { }}>
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

// 🔷#region 🔧HoverCard
const CalendarDays = createIconComponent("calendar-days");

const meta = {
  title: "🖱️ui⚛️react/HoverCard",
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

// #endregion 🔧HoverCard

// 💻#region 📣Aside
export const AsideNote: Story = {
  render: () => (
    <div className="w-96 space-y-4">
      <Aside kind="note" title="Note">
        This is a note callout for general information.
      </Aside>
      <Aside kind="tip" title="Tip">
        This is a tip callout for helpful suggestions.
      </Aside>
      <Aside kind="caution" title="Caution">
        This is a caution callout for important warnings.
      </Aside>
      <Aside kind="danger" title="Danger">
        This is a danger callout for critical alerts.
      </Aside>
    </div>
  ),
};
// #endregion 📣Aside

// 🔷#region 🎬Card
export const CardDefault: Story = {
  render: () => (
    <div className="w-96">
      <Card title="Capsule J" icon="🏠">
        A residential capsule unit with 2.5m × 4.0m footprint, designed for modular living.
      </Card>
    </div>
  ),
};

export const CardGridDefault: Story = {
  render: () => (
    <CardGrid>
      <Card title="Capsule J" icon="🏠">
        Residential unit
      </Card>
      <Card title="Capsule K" icon="🏢">
        Commercial unit
      </Card>
      <Card title="Base" icon="🏛️">
        Foundation module
      </Card>
      <Card title="Capital" icon="🛠️">
        Roof module
      </Card>
    </CardGrid>
  ),
};
// #endregion 🎬Card
