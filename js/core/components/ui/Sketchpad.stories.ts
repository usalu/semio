import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { Sketchpad } from '@semio/js';

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
  title: 'Studio/Sketchpad',
  component: Sketchpad,
} satisfies Meta<typeof Sketchpad>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Example: Story = {
  args: {
    // label: 'Button',
  },
};
