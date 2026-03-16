// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/aggregation/Collapsible.stories.tsx

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
import { Box, ChevronDown } from "lucide-react";
import { useState } from "react";
import { Button, Collapsible, CollapsibleContent, CollapsibleTrigger, Level, LevelProvider, Section, Steps, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖Collapsible
const meta = {
  title: "semio-elements/Collapsible",
  component: Collapsible,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Collapsible>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(true);
    return (
      <Collapsible open={isOpen} onOpenChange={setIsOpen} className="w-96 space-y-2">
        <div className="flex items-center justify-between space-x-4 pb-2">
          <div className="flex items-center gap-double">
            <Box className="size-tiny" />
            <h4 className="text-sm font-semibold">Capsule Variants (3)</h4>
          </div>
          <CollapsibleTrigger asChild>
            <Button variant="ghost">
              <ChevronDown className={`size-small transition-transform ${isOpen ? "rotate-180" : ""}`} />
              <span className="sr-only">Toggle</span>
            </Button>
          </CollapsibleTrigger>
        </div>
        <div className="border px-4 py-2 text-sm flex items-center justify-between">
          <span>Capsule J</span>
          <span className="text-xs text-muted-foreground">2.5m × 4.0m</span>
        </div>
        <CollapsibleContent className="space-y-2">
          <div className="border px-4 py-2 text-sm">Capsule L</div>
          <div className="border px-4 py-2 text-sm">Capsule P</div>
        </CollapsibleContent>
      </Collapsible>
    );
  },
};

const CollapsibleDemo: React.FC<{ level: Level }> = ({ level }) => {
  const [isOpen, setIsOpen] = useState(true);
  return (
    <LevelProvider level={level}>
      <div className={`p-4 ${getLevelBgClass(level)}`}>
        <Collapsible open={isOpen} onOpenChange={setIsOpen} className="w-96 space-y-2">
          <div className="flex items-center justify-between space-x-4 pb-2">
            <div className="flex items-center gap-double">
              <Box className="size-tiny" />
              <h4 className="text-sm font-semibold">Capsule Variants (3)</h4>
            </div>
            <CollapsibleTrigger asChild>
              <Button variant="ghost">
                <ChevronDown className={`size-small transition-transform ${isOpen ? "rotate-180" : ""}`} />
                <span className="sr-only">Toggle</span>
              </Button>
            </CollapsibleTrigger>
          </div>
          <div className="border px-4 py-2 text-sm flex items-center justify-between">
            <span>Capsule J</span>
            <span className="text-xs text-muted-foreground">2.5m × 4.0m</span>
          </div>
          <CollapsibleContent className="space-y-2">
            <div className="border px-4 py-2 text-sm">Capsule L</div>
            <div className="border px-4 py-2 text-sm">Capsule P</div>
          </CollapsibleContent>
        </Collapsible>
      </div>
    </LevelProvider>
  );
};

export const Base: Story = {
  render: () => <CollapsibleDemo level="base" />,
};

export const Window: Story = {
  render: () => <CollapsibleDemo level="window" />,
};

export const Panel: Story = {
  render: () => <CollapsibleDemo level="panel" />,
};

export const Overlay: Story = {
  render: () => <CollapsibleDemo level="overlay" />,
};

export const Temporary: Story = {
  render: () => <CollapsibleDemo level="temporary" />,
};

// #endregion 🔖Collapsible

// #region 🔖Section
export const SectionDefault: Story = {
  render: () => (
    <div className="w-96">
      <Section title="Module Types" id="module-types">
        <p className="text-sm">Capsule J, Capsule K, Base, Tambour A, Capital</p>
      </Section>
      <Section title="Design Properties" id="design-properties">
        <p className="text-sm">Volume: 25.0 m³, Area: 10.0 m², Connections: 4</p>
      </Section>
    </div>
  ),
};
// #endregion 🔖Section

// #region 🔖Steps
export const StepsDefault: Story = {
  render: () => (
    <div className="w-96">
      <Steps>
        <div className="border p-single">
          <h3 className="font-semibold">Step 1: Create a Kit</h3>
          <p className="text-sm text-muted-foreground">Initialize a new kit with types and qualities.</p>
        </div>
        <div className="border p-single">
          <h3 className="font-semibold">Step 2: Add Types</h3>
          <p className="text-sm text-muted-foreground">Define the building blocks of your design.</p>
        </div>
        <div className="border p-single">
          <h3 className="font-semibold">Step 3: Create a Design</h3>
          <p className="text-sm text-muted-foreground">Compose types into a complete design.</p>
        </div>
      </Steps>
    </div>
  ),
};
// #endregion 🔖Steps

