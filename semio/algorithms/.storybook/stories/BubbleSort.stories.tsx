// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/BubbleSort.stories.tsx
// Specs: One algorithm per stories file with one story per input scenario.
// Summary: Visualizes and compares bubble sort implementations by input case.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { Card, CardGrid, Section } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

type BubbleSortRun = {
  output: number[];
  passes: number[][];
  comparisons: number;
  swaps: number;
};

type BubbleSortKind = "naive" | "early-exit";

function runBubbleSort(input: number[], kind: BubbleSortKind): BubbleSortRun {
  const arr = [...input];
  const passes: number[][] = [];
  let comparisons = 0;
  let swaps = 0;

  for (let i = 0; i < arr.length; i++) {
    let changed = false;
    for (let j = 0; j < arr.length - i - 1; j++) {
      comparisons += 1;
      if (arr[j] > arr[j + 1]) {
        [arr[j], arr[j + 1]] = [arr[j + 1], arr[j]];
        swaps += 1;
        changed = true;
      }
    }
    passes.push([...arr]);
    if (kind === "early-exit" && !changed) {
      break;
    }
  }

  return { output: arr, passes, comparisons, swaps };
}

function BubbleSortView({ input }: { input: number[] }) {
  const naive = runBubbleSort(input, "naive");
  const earlyExit = runBubbleSort(input, "early-exit");

  return (
    <div className="space-y-4">
      <Section title="Input">
        <div className="p-3 text-sm font-mono">{JSON.stringify(input)}</div>
      </Section>
      <CardGrid>
        {[
          { title: "Naive Bubble Sort", run: naive },
          { title: "Early Exit Bubble Sort", run: earlyExit },
        ].map(({ title, run }) => (
          <Card key={title} title={title}>
            <div className="space-y-2 text-sm">
              <div>
                <span className="font-semibold">Output:</span> <span className="font-mono">{JSON.stringify(run.output)}</span>
              </div>
              <div className="grid grid-cols-2 gap-2">
                <div>Comparisons: {run.comparisons}</div>
                <div>Swaps: {run.swaps}</div>
              </div>
              <div className="space-y-1">
                <div className="font-semibold">Passes</div>
                {run.passes.map((pass, index) => (
                  <div key={index} className="font-mono text-xs">
                    {index + 1}. {JSON.stringify(pass)}
                  </div>
                ))}
              </div>
            </div>
          </Card>
        ))}
      </CardGrid>
    </div>
  );
}

const meta = {
  title: "semio-algorithms/Bubble Sort",
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const RandomInput: Story = {
  render: () => <BubbleSortView input={[5, 1, 4, 2, 8]} />,
};

export const NearlySortedInput: Story = {
  render: () => <BubbleSortView input={[1, 2, 3, 5, 4, 6, 7]} />,
};

export const ReverseSortedInput: Story = {
  render: () => <BubbleSortView input={[9, 8, 7, 6, 5, 4, 3]} />,
};
