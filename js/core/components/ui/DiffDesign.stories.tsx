// #region Header

// DiffDesign.stories.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import DiffDesign from '@semio/js/components/ui/DiffDesign';
import type { Meta, StoryObj } from '@storybook/react';

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