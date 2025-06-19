import type { Meta, StoryObj } from '@storybook/react';
import DiffDesign from '@semio/js/components/ui/DiffDesign';

const meta = {
    title: 'UI/DiffDesign',
    component: DiffDesign,
    parameters: {
        layout: 'centered',
    },
    tags: ['autodocs'],
} satisfies Meta<typeof DiffDesign>;

export default meta;
type Story = StoryObj<typeof DiffDesign>;

// Sample design data
const sampleDesign = {
    id: "design1",
    name: "Sample Design",
    properties: {
        width: 100,
        height: 100
    }
};

const sampleDiff = {
    id: "design1",
    name: "Modified Design",
    properties: {
        width: 150,
        height: 100
    }
};

export const Default: Story = {
    args: {
        initialDesign: sampleDesign,
        initialDiff: sampleDiff,
    },
};

export const EmptyDiff: Story = {
    args: {
        initialDesign: sampleDesign,
        initialDiff: sampleDesign,
    },
};

export const ComplexDiff: Story = {
    args: {
        initialDesign: {
            ...sampleDesign,
            properties: {
                ...sampleDesign.properties,
                depth: 50,
                material: "wood"
            }
        },
        initialDiff: {
            ...sampleDesign,
            properties: {
                ...sampleDesign.properties,
                depth: 75,
                material: "metal",
                finish: "polished"
            }
        },
    },
};