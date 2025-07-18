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
    initialKit: Metabolism,
    designId: {
      name: "Nakagin Capsule Tower"
    },
  },
  render: (args, { loaded: { fileUrls } }) => (
    <DesignEditor {...args} fileUrls={fileUrls} />
  ),
};

export const NakaginCapsuleTowerWithDiff: Story = {
  loaders: [
    async () => ({
      fileUrls: await extractFilesAndCreateUrls("metabolism.zip"),
    }),
  ],
  args: {
    initialKit: Metabolism,
    designId: {
      name: "Nakagin Capsule Tower"
    },
    initialDesignDiff: {
      name: "Modified Nakagin Capsule Tower",
      description: "A heavily modified version with additional capsules, repositioned elements, and new connections showcasing complex architectural transformations.",
      pieces: {
        // Remove some existing pieces
        removed: [
          { id_: "cs_sl0_d0_t_f0_b_c0" },
          // { id_: "cs_sl0_d0_t_f0_b_c1" },
          // { id_: "cs_sl0_d0_t_f1_b_c0" }
        ],
        // Update existing pieces with new positions and properties
        updated: [
          {
            id_: "b",
            description: "Repositioned base with elevated foundation",
            plane: {
              origin: { x: 2.5, y: 1.0, z: 0.5 },
              xAxis: { x: 0.9659, y: 0.2588, z: 0.0 },
              yAxis: { x: -0.2588, y: 0.9659, z: 0.0 }
            },
            center: { x: 2.5, y: 1.0 }
          },
          // {
          //   id_: "cs_sl0_d0_t_f3_b_c0",
          //   description: "Rotated capsule with different variant",
          //   type: { name: "Capsule", variant: "J" },
          //   center: { x: -15.2, y: 8.5 }
          // },
          // {
          //   id_: "cs_sl0_d0_t_f6_b_c1",
          //   description: "Enlarged capsule with balcony variant",
          //   type: { name: "Capsule with Balcony", variant: "L" },
          //   qualities: [
          //     { name: "effective floor area", value: "35.5", unit: "m²" },
          //     { name: "income", value: "2800", unit: "CHF/month" }
          //   ]
          // }
        ],
        // Add completely new pieces
        added: [
          {
            id_: "new_bridge",
            description: "New structural bridge",
            type: { name: "Bridge", variant: "" },
            plane: {
              origin: { x: -13.05, y: -7.7, z: 45.0 },
              xAxis: { x: 1.0, y: 0.0, z: 0.0 },
              yAxis: { x: 0.0, y: 1.0, z: 0.0 }
            },
            center: { x: 3, y: -2 }
          },
          // {
          //   id_: "cs_sl0_d0_t_f15_bridge_01",
          //   description: "High-level observation capsule on bridge",
          //   type: { name: "Capsule", variant: "L" },
          //   qualities: [
          //     { name: "effective floor area", value: "25.5", unit: "m²" },
          //     { name: "income", value: "1200", unit: "CHF/month" }
          //   ]
          // },
          // {
          //   id_: "ci_f16_b_c0",
          //   description: "Additional capital for extended east tower",
          //   type: { name: "Capital", variant: "" },
          //   qualities: [
          //     { name: "construction cost", value: "12000", unit: "CHF" }
          //   ]
          // }
        ]
      },
      connections: {
        // Remove connections to pieces that are being removed
        removed: [
          {
            connected: { piece: { id_: "t_f0_b_c0" } },
            connecting: { piece: { id_: "cs_sl0_d0_t_f0_b_c0" } }
          },
          // {
          //   connected: { piece: { id_: "t_f0_b_c0" } },
          //   connecting: { piece: { id_: "cs_sl0_d0_t_f0_b_c1" } }
          // },
          // {
          //   connected: { piece: { id_: "t_f1_b_c0" } },
          //   connecting: { piece: { id_: "cs_sl0_d0_t_f1_b_c0" } }
          // }
        ],
        // Update existing connections with new parameters
        // updated: [
        //   {
        //     connected: {
        //       piece: { id_: "t_f3_b_c0" },
        //       port: { id_: "c0" }
        //     },
        //     connecting: {
        //       piece: { id_: "cs_sl0_d0_t_f3_b_c0" },
        //       port: { id_: "" }
        //     },
        //     gap: 0.2,
        //     shift: 1.5,
        //     rise: 0.3,
        //     rotation: 45.0,
        //     turn: 15.0,
        //     tilt: -5.0,
        //     x: -2.0,
        //     y: 3.5
        //   }
        // ],
        // Add new connections for the new elements
        // added: [
        //   {
        //     connected: {
        //       piece: { id_: "t_f15_b_c0" },
        //       port: { id_: "c0" }
        //     },
        //     connecting: {
        //       piece: { id_: "new_bridge" },
        //       port: { id_: "e" }
        //     },
        //     description: "Bridge connection to east tower",
        //     gap: 0.0,
        //     shift: 0.0,
        //     rise: 0.0,
        //     rotation: 0.0,
        //     turn: 0.0,
        //     tilt: 0.0,
        //     x: 0.0,
        //     y: 0.0,
        //     qualities: [
        //       { name: "construction cost", value: "5000", unit: "CHF" }
        //     ]
        //   },
        //   {
        //     connected: {
        //       piece: { id_: "t_f15_b_c1" },
        //       port: { id_: "c1" }
        //     },
        //     connecting: {
        //       piece: { id_: "new_bridge" },
        //       port: { id_: "w" }
        //     },
        //     description: "Bridge connection to west tower",
        //     gap: 0.0,
        //     shift: 0.0,
        //     rise: 0.0,
        //     rotation: 180.0,
        //     turn: 0.0,
        //     tilt: 0.0,
        //     x: 0.0,
        //     y: 0.0,
        //     qualities: [
        //       { name: "construction cost", value: "5000", unit: "CHF" }
        //     ]
        //   },
        //   {
        //     connected: {
        //       piece: { id_: "new_bridge" },
        //       port: { id_: "e" }
        //     },
        //     connecting: {
        //       piece: { id_: "cs_sl0_d0_t_f15_bridge_01" },
        //       port: { id_: "" }
        //     },
        //     description: "Observatory capsule mounting",
        //     gap: 0.1,
        //     shift: 0.0,
        //     rise: 2.5,
        //     rotation: 0.0,
        //     turn: 0.0,
        //     tilt: 0.0,
        //     x: 0.0,
        //     y: 0.0,
        //     qualities: [
        //       { name: "construction cost", value: "8000", unit: "CHF" }
        //     ]
        //   },
        //   {
        //     connected: {
        //       piece: { id_: "ci_t_f15_b_c0" },
        //       port: { id_: "" }
        //     },
        //     connecting: {
        //       piece: { id_: "ci_f16_b_c0" },
        //       port: { id_: "" }
        //     },
        //     description: "Tower extension connection",
        //     gap: 0.0,
        //     shift: 0.0,
        //     rise: 0.0,
        //     rotation: 0.0,
        //     turn: 0.0,
        //     tilt: 0.0,
        //     x: 0.0,
        //     y: 0.0,
        //     qualities: [
        //       { name: "construction cost", value: "3000", unit: "CHF" }
        //     ]
        //   }
        // ]
      }
    },
  },
  render: (args, { loaded: { fileUrls } }) => (
    <DesignEditor {...args} fileUrls={fileUrls} />
  ),
};