import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';

import { default as Diagram } from "@semio/js/components/ui/Diagram";
import { default as Metabolism } from "../../../../assets/semio/kit_metabolism.json";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories#default-export
const meta = {
  title: 'Studio/Diagram',
  component: Diagram,
  // parameters: {
  // Optional parameter to center the component in the Canvas. More info: https://storybook.js.org/docs/configure/story-layout
  // },
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/writing-docs/autodocs
  // tags: ['autodocs'],
  // More on argTypes: https://storybook.js.org/docs/api/argtypes
  // argTypes: {
  // },
  // Use `fn` to spy on the onClick arg, which will appear in the actions panel once invoked: https://storybook.js.org/docs/essentials/actions#action-args
  // args: {},
} satisfies Meta<typeof Diagram>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const NakaginCapsuleTower: Story = {
  args: {
  },
};

export const Readonly: Story = {
  args: {
    // label: 'Button',
  },
};
