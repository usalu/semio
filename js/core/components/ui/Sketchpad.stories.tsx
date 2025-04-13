import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React from 'react';

import { Sketchpad, Mode, Theme } from '@semio/js';

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
    title: 'Studio/Sketchpad',
    component: Sketchpad,
    parameters: {
        layout: 'fullscreen',
    },
    decorators: [
        (Story) => (
            <div className="w-full h-[800px]">
                <Story />
            </div>
        ),
    ],
} satisfies Meta<typeof Sketchpad>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Example: Story = {
};