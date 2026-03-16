import type { Meta, StoryObj } from '@storybook/react';

const meta: Meta = {
  title: 'Elements/Button',
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
export type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  render: () => <button className="bg-blue-500 text-white px-4 py-2 rounded">Primary Button</button>,
};

export const Secondary: Story = {
  render: () => <button className="bg-gray-500 text-white px-4 py-2 rounded">Secondary Button</button>,
};
