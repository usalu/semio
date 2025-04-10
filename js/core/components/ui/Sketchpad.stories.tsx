import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React from 'react';

import { Sketchpad } from '@semio/js';
import { SketchpadMode } from '@semio/js/components/ui/Sketchpad';

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
    args: {
        mode: SketchpadMode.FULL,
    },
}; 