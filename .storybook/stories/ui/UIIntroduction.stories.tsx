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

// 🎉 An interaction-gated step whose checklist rows tick off one at a time — click "Complete next" to
// watch a row's own label celebrate (conic-gradient ring, see `[data-celebrated="true"]` in `ui.css`)
// the instant it flips from pending to done, on top of the check icon swap.
const interactionWalkthrough: IntroductionDefinition = {
  title: "Try the Viewport",
  steps: [
    {
      id: "step.viewport",
      title: "Try the Viewport",
      body: "Complete each interaction below.",
      introduce: null,
      show: [],
      placement: "center",
      interactions: [
        { on: { kind: "zoom", id: "puzzle3d-main" }, label: "Zoom in or out" },
        { on: { kind: "pan", id: "puzzle3d-main" }, label: "Pan the view" },
        { on: { kind: "orbit", id: "puzzle3d-main" }, label: "Orbit around the model" },
      ],
      ordered: false,
      logos: [],
      demonstrations: [],
    },
  ],
};

export const WithInteractions: Story = {
  name: "Interaction checklist",
  render: () => {
    const [completed, setCompleted] = useState<readonly number[]>([]);
    const total = interactionWalkthrough.steps[0].interactions?.length ?? 0;
    return (
      <>
        <UIIntroduction introduction={interactionWalkthrough} stepIndex={0} completedInteractionIndices={completed} onStepIndexChange={() => {}} onDismiss={() => {}} />
        {completed.length < total && (
          <button
            type="button"
            className="fixed bottom-double left-double z-tutorial rounded bg-primary px-double py-single text-xs text-primary-foreground"
            onClick={() => setCompleted((prev) => [...prev, prev.length])}
          >
            Complete next
          </button>
        )}
      </>
    );
  },
};

const gestureGalleryTarget = { kind: "screenNormalized" as const, x: 0.5, y: 0.5 };

const gestureGalleryWalkthrough: IntroductionDefinition = {
  title: "Gesture Gallery",
  steps: [
    {
      id: "step.gestures",
      title: "Gesture Gallery",
      body: "Stay idle to cycle each ghost-cursor demonstration — left click, right click, double click, left drag, middle drag, scroll, and Alt + right orbit.",
      introduce: null,
      show: [],
      placement: "center",
      interactions: [],
      ordered: false,
      logos: [],
      demonstrations: [
        { gesture: { kind: "leftClick", at: gestureGalleryTarget } },
        { gesture: { kind: "rightClick", at: gestureGalleryTarget } },
        { gesture: { kind: "doubleClick", at: gestureGalleryTarget } },
        {
          gesture: {
            kind: "drag",
            from: { kind: "screenNormalized", x: 0.35, y: 0.5 },
            to: { kind: "screenNormalized", x: 0.65, y: 0.5 },
          },
        },
        {
          gesture: {
            kind: "drag",
            from: { kind: "screenNormalized", x: 0.35, y: 0.55 },
            to: { kind: "screenNormalized", x: 0.65, y: 0.45 },
            button: "middle",
          },
          cursor: "move",
        },
        { gesture: { kind: "scroll", at: gestureGalleryTarget, deltaY: -100 } },
        {
          gesture: {
            kind: "orbit",
            from: { kind: "screenNormalized", x: 0.35, y: 0.5 },
            to: { kind: "screenNormalized", x: 0.65, y: 0.5 },
          },
        },
      ],
    },
  ],
};

export const GestureGallery: Story = {
  name: "Gesture gallery",
  render: () => <UIIntroduction introduction={gestureGalleryWalkthrough} stepIndex={0} onStepIndexChange={() => {}} onDismiss={() => {}} />,
};
// #endregion 🎓UIIntroduction
