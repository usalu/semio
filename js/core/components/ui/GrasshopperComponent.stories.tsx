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
        inputs: [
            {
                name: 'Type',
                nickname: '~Typ',
                description: 'The type to modify.',
                kind: 'Type',
            },
            {
                name: 'Modifier',
                nickname: '~Mod',
                description: 'The modifier to apply to the type.',
                kind: 'Type',
            },
        
        ],  
        outputs: [
            {
                name: 'Modified Type',
                nickname: '~Typ',
                description: 'The modified type.',
                kind: 'Type',
            },
            {
                name: 'Original Type',
                nickname: '~Typ',
                description: 'The original type before modification.',
                kind: 'Type',
            },
            {
                name: 'Modifier',
                nickname: '~Mod',
                description: 'The modifier applied to the type.',
                kind: 'Type',
            }            
        ],
    },
};


export const LinearConnections: Story = {
    args: {
        name: 'Linear Connections',
        nickname: '---',
        description: 'Connect two types together.',
        inputs: [
            {
                name: 'Type A',
                nickname: '~A',
                description: 'The first type to connect.',
                kind: 'Type',
            },
            {
                name: 'Type B',
                nickname: '~B',
                description: 'The second type to connect.',
                kind: 'Type',
            },
        ],
    },
};