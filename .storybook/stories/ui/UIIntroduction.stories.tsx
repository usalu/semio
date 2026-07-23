// #region 🧲Header

// 🥼︎ .storybook/stories/ui/UIIntroduction.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import type { IntroductionDefinition } from "@semio-tech/framework-core";
import { UIIntroduction } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
// #endregion 🔌Adapters

// 🎓#region 🎓UIIntroduction
// Every step leaves `introduce: null` (full-viewport veil, no cutout) so the story renders
// deterministically without depending on real navbar/panel/utility chrome mounting elsewhere on the page.
const walkthrough: IntroductionDefinition = {
  title: "Welcome to Semio",
  steps: [
    { id: "step.welcome", title: "Welcome", body: "This short walkthrough introduces the design workspace.", introduce: null, show: [], placement: "center", advance: { kind: "next" }, logos: [] },
    { id: "step.canvas", title: "The Canvas", body: "Your design lives here — pan, zoom, and select pieces directly.", introduce: null, show: [], placement: "center", advance: { kind: "next" }, logos: [] },
    { id: "step.done", title: "You're Ready", body: "That's the tour — start designing.", introduce: null, show: [], placement: "center", advance: { kind: "next" }, logos: [] },
  ],
};

const meta = {
  title: "🖱️ui⚛️react/UIIntroduction",
  component: UIIntroduction,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof UIIntroduction>;

export default meta;

type Story = StoryObj<typeof meta>;

function UIIntroductionDemo() {
  const [stepIndex, setStepIndex] = useState(0);
  const [dismissed, setDismissed] = useState<string | null>(null);
  if (dismissed) return <div className="p-double text-xs text-muted-foreground">{dismissed}</div>;
  return <UIIntroduction introduction={walkthrough} stepIndex={stepIndex} onStepIndexChange={setStepIndex} onDismiss={(completed) => setDismissed(completed ? "Completed" : "Skipped")} />;
}

export const FirstStep: Story = {
  render: () => <UIIntroductionDemo />,
};

export const LastStep: Story = {
  name: "Last step (Next → Done)",
  render: () => {
    const [dismissed, setDismissed] = useState<string | null>(null);
    if (dismissed) return <div className="p-double text-xs text-muted-foreground">{dismissed}</div>;
    return <UIIntroduction introduction={walkthrough} stepIndex={walkthrough.steps.length - 1} onStepIndexChange={() => {}} onDismiss={(completed) => setDismissed(completed ? "Completed" : "Skipped")} />;
  },
};
// #endregion 🎓UIIntroduction
