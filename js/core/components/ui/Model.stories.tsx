import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';
import React from 'react';

import { default as Model } from "@semio/js/components/ui/Model";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
    title: 'Studio/Model',
    component: Model,
} satisfies Meta<typeof Model>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Example: Story = {
};