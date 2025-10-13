// #region Header

// File.stories.tsx

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

import type { Meta, StoryObj } from "@storybook/react";
import File from "./File";

const meta = {
  title: "Elements/File",
  component: File,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {
    src: { control: false },
    environment: { control: false },
    roughness: { control: false },
    metalness: { control: false },
  },
} satisfies Meta<typeof File>;

export default meta;
type Story = StoryObj<typeof meta>;

// Note: These stories require actual .glb/.gltf files to work properly
// The examples below use placeholder paths - replace with actual file paths

export const Default: Story = {
  args: {
    src: "/models/metabolism/capsule-j.glb",
    environment: "/environments/studio.hdr",
    metalness: 0.8,
    roughness: 0.3,
  },
  render: (args) => (
    <div className="w-[600px] h-[400px]">
      <File {...args} />
    </div>
  ),
};

export const Variants: Story = {
  args: {
    src: "/models/capsule.glb",
  },
  render: () => (
    <div className="flex flex-col gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Default</p>
        <div className="w-96 h-96">
          <File src="/models/capsule.glb" />
        </div>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">With Environment</p>
        <div className="w-96 h-96">
          <File src="/models/base.glb" environment="/environments/studio.hdr" />
        </div>
      </div>
    </div>
  ),
};

export const Basic: Story = {
  args: {
    src: "/models/example.glb",
  },
  render: (args) => (
    <div className="w-96 h-96">
      <File {...args} />
    </div>
  ),
};

export const WithEnvironment: Story = {
  args: {
    src: "/models/example.glb",
    environment: "/environments/studio.hdr",
  },
  render: (args) => (
    <div className="w-96 h-96">
      <File {...args} />
    </div>
  ),
};

export const MetalMaterial: Story = {
  args: {
    src: "/models/example.glb",
    metalness: 1.0,
    roughness: 0.2,
  },
  render: (args) => (
    <div className="w-96 h-96">
      <File {...args} />
    </div>
  ),
};

export const RoughMaterial: Story = {
  args: {
    src: "/models/example.glb",
    metalness: 0.0,
    roughness: 0.9,
  },
  render: (args) => (
    <div className="w-96 h-96">
      <File {...args} />
    </div>
  ),
};

export const CustomSize: Story = {
  args: {
    src: "/models/example.glb",
  },
  render: (args) => (
    <div className="w-[600px] h-[400px]">
      <File {...args} />
    </div>
  ),
};

export const MultipleViewers: Story = {
  args: {
    src: "/models/model1.glb",
  },
  render: () => (
    <div className="grid grid-cols-2 gap-4">
      <div className="w-64 h-64 border rounded-md">
        <File src="/models/model1.glb" />
      </div>
      <div className="w-64 h-64 border rounded-md">
        <File src="/models/model2.glb" />
      </div>
    </div>
  ),
};

export const Placeholder: Story = {
  args: {
    src: "",
  },
  render: () => (
    <div className="w-96 h-96 border flex items-center justify-center bg-muted/20">
      <div className="text-center space-y-2">
        <p className="text-sm font-medium">3D Model Viewer</p>
        <p className="text-xs text-muted-foreground">Provide a .glb or .gltf file to display</p>
      </div>
    </div>
  ),
};
