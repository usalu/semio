import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';

import { default as DesignEditor } from "@semio/js/components/ui/DesignEditor";
import { default as Metabolism } from "@semio/assets/semio/kit_metabolism.json";

const meta = {
  title: 'Studio/DesignEditor',
  component: DesignEditor,
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
} satisfies Meta<typeof DesignEditor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NakaginCapsuleTower: Story = {
  args: {
    kit: Metabolism,
    designId: {
      name: "Nakagin Capsule Tower"
    }

  },
};