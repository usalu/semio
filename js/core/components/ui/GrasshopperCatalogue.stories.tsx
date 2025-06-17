import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import React from 'react';

import { default as GrasshopperCatalogue } from "@semio/js/components/ui/GrasshopperCatalogue";
import { default as Components } from "../../../../assets/grasshopper/components.json";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
    title: 'Grasshopper/Catalogue',
    component: GrasshopperCatalogue,
} satisfies Meta<typeof GrasshopperCatalogue>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Catalogue: Story = {
    args: Components,
};