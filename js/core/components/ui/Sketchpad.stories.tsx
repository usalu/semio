import type { Meta, StoryObj } from '@storybook/react-vite';
import React from 'react';

import { Sketchpad, Mode } from '@semio/js';

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

export const User: Story = {
    args: {
        userId: "user-test",
    },
};

export const Guest: Story = {
    args: {
        userId: "guest-test",
        mode: Mode.GUEST,
    },
};