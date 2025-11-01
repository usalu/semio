// #region Header

// Sketchpad.stories.tsx

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

import Sketchpad from "./Sketchpad";
import type { CompleteState } from "./store";
import { Access, Expertise, Layout, Mode, Theme } from "./store";

const meta = {
  title: "Sketchpad",
  component: Sketchpad,
  parameters: {
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <div className="w-full h-[750px]">
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof Sketchpad>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};

// Example with initial state - Hello Semio tutorial
const helloSemioInitialState: CompleteState = {
  sketchpad: {
    navigation: "/kits/hello-kit/designs/hello-design",
    navigationHistory: ["/", "/kits/hello-kit", "/kits/hello-kit/designs/hello-design"],
    navigationHistoryIndex: 2,
    recentSearches: [],
    recentFocusItems: {},
    access: Access.USER,
    theme: Theme.SYSTEM,
    layout: Layout.NORMAL,
    expertise: Expertise.NORMAL,
    mode: Mode.DEV,
    appSettings: {
      design: { snappiness: 10, gridSize: 24 },
      type: {},
      kit: {},
    },
    panelSizes: {
      toolbarHeight: 52,
      workbenchWidth: 230,
      toolsWidth: 230,
      hudWidth: 230,
      statsWidth: 230,
      detailsWidth: 230,
      chatWidth: 230,
      settingsWidth: 230,
      consoleHeight: 200,
    },
    isFullscreen: false,
    isNavbarExpanded: false,
    isMobile: false,
    activeInteraction: "",
    hotkeyOverrides: {},
    activeHotkeySetting: "",
  },
  kits: [
    {
      guid: "hello-kit",
      local: true,
      remote: false,
      kit: {
        guid: "hello-kit",
        name: "Hello Semio Kit",
        version: "1.0.0",
        types: [
          {
            guid: "connector-type",
            name: "Connector Block",
            variant: "",
            representations: [
              {
                guid: "rep-1",
                tags: [],
                file: "/models/cube.glb",
                description: "3D cube model",
              },
            ],
            ports: [
              {
                guid: "port-1",
                point: { x: 0.5, y: 0, z: 0 },
                direction: { x: 1, y: 0, z: 0 },
                t: 0,
                mandatory: false,
                family: "",
                compatibleFamilies: [],
                description: "Right port",
              },
              {
                guid: "port-2",
                point: { x: -0.5, y: 0, z: 0 },
                direction: { x: -1, y: 0, z: 0 },
                t: 0.5,
                mandatory: false,
                family: "",
                compatibleFamilies: [],
                description: "Left port",
              },
            ],
            stock: 100,
            virtual: false,
            unit: "m",
            concepts: ["hello-semio", "tutorial"],
            icon: "",
            image: "",
            description: "A simple connector block with two ports for the Hello Semio tutorial",
          },
        ],
        designs: [
          {
            guid: "hello-design",
            name: "Hello Design",
            variant: "",
            view: JSON.stringify({
              position: { x: 0, y: 5, z: 5 },
              forward: { x: 0, y: -0.707, z: -0.707 },
              up: { x: 0, y: 0.707, z: -0.707 },
            }),
            pieces: [
              {
                guid: "piece-1",
                type: "connector-type",
                plane: {
                  origin: { x: 0, y: 0, z: 0 },
                  xAxis: { x: 1, y: 0, z: 0 },
                  yAxis: { x: 0, y: 1, z: 0 },
                },
                center: { x: 0, y: 0 },
                scale: 1,
                isHidden: false,
                isLocked: false,
                description: "First piece (fixed)",
              },
              {
                guid: "piece-2",
                type: "connector-type",
                center: { x: 2, y: 0 },
                scale: 1,
                isHidden: false,
                isLocked: false,
                description: "Second piece (ready to connect)",
              },
            ],
            connections: [],
            layers: [
              {
                path: "default",
                isHidden: false,
                isLocked: false,
                description: "Default layer",
              },
            ],
            activeLayer: "default",
            groups: [],
            unit: "m",
            concepts: ["hello-semio", "tutorial"],
            icon: "",
            image: "",
            description: "A simple design with two unconnected pieces for the Hello Semio tutorial",
          },
        ],
        qualities: [],
        files: [],
        authors: [],
        remote: "",
        homepage: "https://github.com/usalu/semio",
        license: "LGPL-3.0",
        concepts: ["hello-semio", "tutorial"],
        icon: "",
        image: "",
        description: "Example kit for the Hello Semio tutorial demonstrating basic semio concepts",
      },
    },
  ],
  kitApps: {},
  typeApps: {},
  qualityApps: {},
  designApps: {
    "hello-kit": {
      "hello-design": {
        selectedPieces: ["piece-1", "piece-2"],
        selectedConnections: [],
        selectedPorts: [],
        activeTool: "selection-normal",
        panelVisibility: {
          workbench: true,
          details: true,
          settings: false,
          tools: true,
          hud: false,
          stats: false,
          chat: false,
          toolbar: true,
        },
        isTransactionActive: false,
        windows: [
          {
            id: "scene",
            type: "scene",
          },
          {
            id: "diagram",
            type: "diagram",
          },
        ],
      },
    },
  },
  tutorials: {
    activeTutorial: null,
    playbackState: null,
    recordingState: null,
    tutorialsByKey: {},
  },
};

export const ReadyToConnect: Story = {
  args: {
    id: "ready-to-connect-story",
    initialState: {
      kits: helloSemioInitialState.kits,
      navigation: "/kits/hello-kit/designs/hello-design",
      navigationHistory: ["/", "/kits/hello-kit", "/kits/hello-kit/designs/hello-design"],
      navigationHistoryIndex: 2,
      mode: Mode.DEV,
    },
  },
};
