// #region 🔖Header
// 💻 semio/algorithms/.storybook/stories/BinarySearch.stories.tsx
// Specs: One algorithm per stories file with one story per input scenario.
// Summary: Visualizes and compares binary search implementations by input case.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { Card, CardGrid, Section } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

type BinarySearchStep = {
  left: number;
  right: number;
  middle: number;
  value: number;
};

type BinarySearchRun = {
  index: number;
  steps: BinarySearchStep[];
};

function binarySearchIterative(input: number[], target: number): BinarySearchRun {
  const steps: BinarySearchStep[] = [];
  let left = 0;
  let right = input.length - 1;

  while (left <= right) {
    const middle = Math.floor((left + right) / 2);
    const value = input[middle];
    steps.push({ left, right, middle, value });
    if (value === target) {
      return { index: middle, steps };
    }
    if (value < target) {
      left = middle + 1;
    } else {
      right = middle - 1;
    }
  }

  return { index: -1, steps };
}

function binarySearchRecursive(input: number[], target: number): BinarySearchRun {
  const steps: BinarySearchStep[] = [];

  function walk(left: number, right: number): number {
    if (left > right) {
      return -1;
    }
    const middle = Math.floor((left + right) / 2);
    const value = input[middle];
    steps.push({ left, right, middle, value });
    if (value === target) {
      return middle;
    }
    if (value < target) {
      return walk(middle + 1, right);
    }
    return walk(left, middle - 1);
  }

  return { index: walk(0, input.length - 1), steps };
}

function BinarySearchView({ input, target }: { input: number[]; target: number }) {
  const iterative = binarySearchIterative(input, target);
  const recursive = binarySearchRecursive(input, target);

  return (
    <div className="space-y-4">
      <Section title="Input">
        <div className="space-y-1 p-3 text-sm">
          <div>
            <span className="font-semibold">Array:</span> <span className="font-mono">{JSON.stringify(input)}</span>
          </div>
          <div>
            <span className="font-semibold">Target:</span> <span className="font-mono">{target}</span>
          </div>
        </div>
      </Section>
      <CardGrid>
        {[
          { title: "Iterative Binary Search", run: iterative },
          { title: "Recursive Binary Search", run: recursive },
        ].map(({ title, run }) => (
          <Card key={title} title={title}>
            <div className="space-y-2 text-sm">
              <div>
                <span className="font-semibold">Index:</span> {run.index}
              </div>
              <div className="font-semibold">Steps</div>
              {run.steps.map((step, index) => (
                <div key={index} className="font-mono text-xs">
                  {index + 1}. L={step.left} R={step.right} M={step.middle} V={step.value}
                </div>
              ))}
            </div>
          </Card>
        ))}
      </CardGrid>
    </div>
  );
}

const meta = {
  title: "semio-algorithms/Binary Search",
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta;

export default meta;

type Story = StoryObj<typeof meta>;

export const TargetFound: Story = {
  render: () => <BinarySearchView input={[1, 3, 5, 7, 9, 11, 13]} target={9} />,
};

export const TargetNotFound: Story = {
  render: () => <BinarySearchView input={[2, 4, 6, 8, 10, 12, 14]} target={9} />,
};

export const SmallInput: Story = {
  render: () => <BinarySearchView input={[4, 8, 12]} target={4} />,
};
