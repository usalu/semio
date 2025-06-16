import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React from 'react';

import { default as GrasshopperCatalogue } from "@semio/js/components/ui/GrasshopperCatalogue";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
    title: 'Grasshopper/Catalogue',
    component: GrasshopperCatalogue,
} satisfies Meta<typeof GrasshopperCatalogue>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Catalogue: Story = {
    args: {
        components: [
            {
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
            // {
            //     name: 'Linear Connections',
            //     nickname: '---',
            //     description: 'Connect two types together.',
            //     inputs: [
            //         {
            //             name: 'Type A',
            //             nickname: '~A',
            //             description: 'The first type to connect.',
            //             kind: 'Type',
            //         },
            //         {
            //             name: 'Type B',
            //             nickname: '~B',
            //             description: 'The second type to connect.',
            //             kind: 'Type',
            //         },
            //     ],
            //     outputs: [
            //         {
            //             name: 'Connected Type',
            //             nickname: '~Typ',
            //             description: 'The connected type.',
            //             kind: 'Type',
            //         },
            //     ],
            // },
        ],
    },
};