import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';

import { default as DesignEditor } from "@semio/js/components/ui/DesignEditor";
import { default as Metabolism } from "@semio/assets/semio/kit_metabolism.json";
import { extractFilesAndCreateUrls } from '../../lib/utils';

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
  loaders: [
    async () => ({
      fileUrls: await extractFilesAndCreateUrls("metabolism.zip"),
    }),
  ],
  args: {
    kit: Metabolism as any,
    designId: {
      name: "Nakagin Capsule Tower"
    },
    selection: { selectedPieceIds: [], selectedConnections: [] },
    onSelectionChange: fn(),
    onPieceCreate: fn(),
    onPiecesUpdate: fn(),
    onSelectionDelete: fn(),
    onUndo: fn(),
    onRedo: fn(),
  },
  render: (args, { loaded: { fileUrls } }) => (
    <DesignEditor {...args} fileUrls={fileUrls} />
  ),
};