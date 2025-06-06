import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React from 'react';

import { default as GrasshopperComponent } from "@semio/js/components/ui/GrasshopperComponent";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
    title: 'Grasshopper/Component',
    component: GrasshopperComponent,
} satisfies Meta<typeof GrasshopperComponent>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const ModelType: Story = {
    args: {
        name: 'Model Type',
        nickname: '~Typ',
        description: 'Construct, deconstruct or modify a type.',
    },
};

export const LinearConnections: Story = {
    args: {
        name: 'Linear Connections',
        nickname: '---',
        description: 'Connect two types together.',
    },
};