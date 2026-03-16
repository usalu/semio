// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/aggregation/Tree.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { File, Folder, Settings } from "lucide-react";
import React from "react";
import { Button, ControlDef, ControlTree, ControlTreeFolderSettings, Input, Level, LevelProvider, Tree, TreeContent, TreeItem, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖Tree
const meta = {
  title: "semio-elements/Tree",
  component: Tree,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tree>;

export default meta;

type Story = StoryObj<typeof meta>;

const treeSections = [
  {
    id: "tree.story.kit",
    label: "Kit",
    icon: <Folder size={14} />,
    items: [
      {
        id: "tree.story.kit.types",
        label: "Types",
        icon: <Folder size={12} />,
        items: [
          {
            id: "tree.story.kit.types.capsules",
            label: "Capsules",
            icon: <Folder size={12} />,
            items: [
              {
                id: "tree.story.kit.types.capsules.variants",
                label: "Variants",
                icon: <Folder size={12} />,
                items: [
                  { id: "tree.story.kit.types.capsules.variants.j", label: "Capsule J", icon: <File size={12} /> },
                  { id: "tree.story.kit.types.capsules.variants.l", label: "Capsule L", icon: <File size={12} /> },
                  { id: "tree.story.kit.types.capsules.variants.p", label: "Capsule P", icon: <File size={12} /> },
                ],
              },
              {
                id: "tree.story.kit.types.capsules.balcony",
                label: "Balcony",
                icon: <Folder size="XL" />,
                items: [
                  { id: "tree.story.kit.types.capsules.balcony.j", label: "With Balcony J", icon: <File size={12} /> },
                  { id: "tree.story.kit.types.capsules.balcony.l", label: "With Balcony L", icon: <File size={12} /> },
                ],
              },
            ],
          },
          {
            id: "tree.story.kit.types.bases",
            label: "Bases",
            icon: <Folder size={12} />,
            items: [{ id: "tree.story.kit.types.bases.blob", label: "Base Blob", icon: <File size={12} /> }],
          },
        ],
      },
      { id: "tree.story.kit.designs", label: "Designs", icon: <File size={12} /> },
      { id: "tree.story.kit.qualities", label: "Qualities", icon: <File size={12} /> },
    ],
  },
  {
    id: "tree.story.settings",
    label: "Settings",
    icon: <Settings size={14} />,
    content: (
      <>
        <TreeItem label="General">
          <TreeContent>
            <Input id="kit-name-input" value="Metabolism" />
            <Input id="version-input" value="1.0.0" />
          </TreeContent>
        </TreeItem>
        <TreeItem label="Advanced">
          <TreeContent>
            <Button>Export Kit</Button>
          </TreeContent>
        </TreeItem>
      </>
    ),
  },
];

export const Default: Story = {
  args: { sections: treeSections },
  render: (args) => (
    <div className="border p-4">
      <Tree {...args} />
    </div>
  ),
};

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`border p-4 ${getLevelBgClass(level)}`}>
      <Tree sections={treeSections} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { sections: treeSections },
  render: createLevelRender("base"),
};

export const Window: Story = {
  args: { sections: treeSections },
  render: createLevelRender("window"),
};

export const Panel: Story = {
  args: { sections: treeSections },
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  args: { sections: treeSections },
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  args: { sections: treeSections },
  render: createLevelRender("temporary"),
};

const ControlTreeDemo = () => {
  const [values, setValues] = React.useState<Record<string, any>>({
    "Transform/Position/x": 0,
    "Transform/Position/y": 1.5,
    "Transform/Position/z": 0,
    "Transform/Rotation/pitch": 0,
    "Transform/Rotation/yaw": 45,
    "Transform/Rotation/roll": 0,
    "Transform/Scale/uniform": 1.0,
    "Appearance/color": "#3b82f6",
    "Appearance/opacity": 80,
    "Appearance/wireframe": false,
    "Appearance/Material/roughness": 0.5,
    "Appearance/Material/metalness": 0.8,
    "Metadata/name": "My Object",
    "Metadata/description": "A sample object for testing the ControlTree",
    "Metadata/visible": true,
    "Metadata/layer": "default",
  });
  const [filterText, setFilterText] = React.useState("");
  const [folderSettings] = React.useState<Record<string, ControlTreeFolderSettings>>({
    Transform: { path: "Transform", order: 0 },
    "Transform/Position": { path: "Transform/Position", order: 0 },
    "Transform/Rotation": { path: "Transform/Rotation", order: 1 },
    "Transform/Scale": { path: "Transform/Scale", order: 2 },
    Appearance: { path: "Appearance", order: 1 },
    "Appearance/Material": { path: "Appearance/Material", order: 10 },
    Metadata: { path: "Metadata", order: 2, collapsed: true },
  });
  const makeOnChange = (path: string) => (next: any) => setValues((prev) => ({ ...prev, [path]: next }));
  const controls: ControlDef[] = [
    { path: "Transform/Position/x", controlKind: "number", value: values["Transform/Position/x"], onChange: makeOnChange("Transform/Position/x"), meta: { min: -100, max: 100, step: 0.1 } },
    { path: "Transform/Position/y", controlKind: "number", value: values["Transform/Position/y"], onChange: makeOnChange("Transform/Position/y"), meta: { min: -100, max: 100, step: 0.1 } },
    { path: "Transform/Position/z", controlKind: "number", value: values["Transform/Position/z"], onChange: makeOnChange("Transform/Position/z"), meta: { min: -100, max: 100, step: 0.1 } },
    { path: "Transform/Rotation/pitch", controlKind: "slider", value: values["Transform/Rotation/pitch"], onChange: makeOnChange("Transform/Rotation/pitch"), meta: { min: -180, max: 180 } },
    { path: "Transform/Rotation/yaw", controlKind: "slider", value: values["Transform/Rotation/yaw"], onChange: makeOnChange("Transform/Rotation/yaw"), meta: { min: -180, max: 180 } },
    { path: "Transform/Rotation/roll", controlKind: "slider", value: values["Transform/Rotation/roll"], onChange: makeOnChange("Transform/Rotation/roll"), meta: { min: -180, max: 180 } },
    { path: "Transform/Scale/uniform", controlKind: "number", value: values["Transform/Scale/uniform"], onChange: makeOnChange("Transform/Scale/uniform"), meta: { min: 0.01, max: 10, step: 0.01 } },
    { path: "Appearance/color", controlKind: "color", value: values["Appearance/color"], onChange: makeOnChange("Appearance/color") },
    { path: "Appearance/opacity", controlKind: "slider", value: values["Appearance/opacity"], onChange: makeOnChange("Appearance/opacity"), meta: { min: 0, max: 100 } },
    { path: "Appearance/wireframe", controlKind: "boolean", value: values["Appearance/wireframe"], onChange: makeOnChange("Appearance/wireframe") },
    { path: "Appearance/Material/roughness", controlKind: "slider", value: values["Appearance/Material/roughness"], onChange: makeOnChange("Appearance/Material/roughness"), meta: { min: 0, max: 1 } },
    { path: "Appearance/Material/metalness", controlKind: "slider", value: values["Appearance/Material/metalness"], onChange: makeOnChange("Appearance/Material/metalness"), meta: { min: 0, max: 1 } },
    { path: "Metadata/name", controlKind: "string", value: values["Metadata/name"], onChange: makeOnChange("Metadata/name") },
    { path: "Metadata/description", controlKind: "text", value: values["Metadata/description"], onChange: makeOnChange("Metadata/description") },
    { path: "Metadata/visible", controlKind: "boolean", value: values["Metadata/visible"], onChange: makeOnChange("Metadata/visible") },
    { path: "Metadata/layer", controlKind: "select", value: values["Metadata/layer"], onChange: makeOnChange("Metadata/layer"), meta: { options: ["default", "foreground", "background", "hidden"] } },
  ];
  return (
    <LevelProvider level="panel">
      <div className="bg-panel border p-2 w-[320px]">
        <div className="mb-2">
          <input type="text" placeholder="Filter controls..." value={filterText} onChange={(e) => setFilterText(e.target.value)} className="w-full h-6 px-2 text-xs border bg-transparent text-foreground placeholder:text-muted-foreground" />
        </div>
        <ControlTree controls={controls} filterText={filterText} folderSettings={folderSettings} />
      </div>
    </LevelProvider>
  );
};

export const ControlTreeStory: Story = {
  args: { sections: [] },
  render: () => <ControlTreeDemo />,
};

// #region 🔖Alternatives

const alternativesSections = [
  {
    id: "alt.story.design",
    label: "Design",
    icon: <Folder size={14} />,
    items: [
      {
        id: "alt.story.design.tower",
        label: "Tower",
        icon: <Folder size={12} />,
        alternatives: [
          [
            { id: "alt.story.design.tower.a.floor1", label: "Floor 1 (Variant A)", icon: <File size={12} /> },
            { id: "alt.story.design.tower.a.floor2", label: "Floor 2 (Variant A)", icon: <File size={12} /> },
            { id: "alt.story.design.tower.a.floor3", label: "Floor 3 (Variant A)", icon: <File size={12} /> },
          ],
          [
            { id: "alt.story.design.tower.b.floor1", label: "Floor 1 (Variant B)", icon: <File size={12} /> },
            { id: "alt.story.design.tower.b.floor2", label: "Floor 2 (Variant B)", icon: <File size={12} /> },
          ],
          [
            { id: "alt.story.design.tower.c.floor1", label: "Floor 1 (Variant C)", icon: <File size={12} /> },
            { id: "alt.story.design.tower.c.floor2", label: "Floor 2 (Variant C)", icon: <File size={12} /> },
            { id: "alt.story.design.tower.c.floor3", label: "Floor 3 (Variant C)", icon: <File size={12} /> },
            { id: "alt.story.design.tower.c.floor4", label: "Floor 4 (Variant C)", icon: <File size={12} /> },
          ],
        ],
      },
      {
        id: "alt.story.design.base",
        label: "Base",
        icon: <Folder size={12} />,
        items: [{ id: "alt.story.design.base.slab", label: "Slab", icon: <File size={12} /> }],
        alternatives: [
          [
            { id: "alt.story.design.base.x.slab", label: "Slab (Option X)", icon: <File size={12} /> },
            { id: "alt.story.design.base.x.parking", label: "Parking (Option X)", icon: <File size={12} /> },
          ],
          [{ id: "alt.story.design.base.y.slab", label: "Slab (Option Y)", icon: <File size={12} /> }],
        ],
      },
      { id: "alt.story.design.roof", label: "Roof", icon: <File size={12} /> },
    ],
  },
];

export const Alternatives: Story = {
  args: { sections: alternativesSections },
  render: (args) => (
    <div className="border p-4 w-[320px]">
      <Tree {...args} />
    </div>
  ),
};

// #endregion 🔖Alternatives

// #endregion 🔖Tree
