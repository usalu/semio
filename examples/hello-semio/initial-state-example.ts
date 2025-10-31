// Example initial state for Sketchpad with kit, type, and design ready to connect pieces

import type { ExtendedInitialState } from "../../js/js/sketchpad/store";
import { Access, Expertise, Layout, Mode, Theme } from "../../js/js/sketchpad/store";
import type { Kit } from "../../js/js/semio";
import { guid } from "../../js/js/semio";

// Generate consistent IDs for the example
const kitId = guid();
const typeId = guid();
const designId = guid();
const piece1Id = guid();
const piece2Id = guid();
const port1Id = guid();
const port2Id = guid();

export const exampleInitialState = {
  // Sketchpad state
  navigation: `/kits/${kitId}/designs/${designId}`,
  navigationHistory: ["/", `/kits/${kitId}`, `/kits/${kitId}/designs/${designId}`],
  navigationHistoryIndex: 2,
  recentSearches: [],
  recentFocusItems: {},
  access: Access.USER,
  theme: Theme.SYSTEM,
  layout: Layout.NORMAL,
  expertise: Expertise.NORMAL,
  mode: Mode.USER,
  appSettings: {
    design: {
      snappiness: 10,
      gridSize: 24,
    },
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
  hotkeyOverrides: {},
};

// Kit structure
export const exampleKit: Kit = {
  guid: kitId,
  name: "Example Kit",
  version: "1.0.0",
  types: [
    {
      guid: typeId,
      name: "Connector Block",
      variant: "",
      representations: [
        {
          guid: guid(),
          tags: [],
          file: "/models/cube.glb",
          description: "3D model representation",
          attributes: [],
        },
      ],
      ports: [
        {
          guid: port1Id,
          point: { x: 0.5, y: 0, z: 0 },
          direction: { x: 1, y: 0, z: 0 },
          t: 0,
          mandatory: false,
          family: "",
          compatibleFamilies: [],
          description: "Connection port 1",
          attributes: [],
        },
        {
          guid: port2Id,
          point: { x: -0.5, y: 0, z: 0 },
          direction: { x: -1, y: 0, z: 0 },
          t: 0.5,
          mandatory: false,
          family: "",
          compatibleFamilies: [],
          description: "Connection port 2",
          attributes: [],
        },
      ],
      props: [],
      stock: 100,
      virtual: false,
      unit: "m",
      location: undefined,
      authors: [],
      concepts: ["hello-semio", "example"],
      icon: "",
      image: "",
      description: "A simple connector block with two ports",
      attributes: [],
    },
  ],
  designs: [
    {
      guid: designId,
      name: "Example Design",
      variant: "",
      view: JSON.stringify({
        position: { x: 0, y: 5, z: 5 },
        forward: { x: 0, y: -0.707, z: -0.707 },
        up: { x: 0, y: 0.707, z: -0.707 },
      }),
      pieces: [
        {
          guid: piece1Id,
          type: typeId,
          design: undefined,
          plane: {
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 },
          },
          center: { x: 0, y: 0 },
          scale: 1,
          mirrorPlane: undefined,
          isHidden: false,
          isLocked: false,
          color: undefined,
          description: "First piece",
          attributes: [],
        },
        {
          guid: piece2Id,
          type: typeId,
          design: undefined,
          plane: undefined,
          center: { x: 2, y: 0 },
          scale: 1,
          mirrorPlane: undefined,
          isHidden: false,
          isLocked: false,
          color: undefined,
          description: "Second piece (unconnected)",
          attributes: [],
        },
      ],
      connections: [],
      stats: [],
      props: [],
      layers: [
        {
          path: "default",
          isHidden: false,
          isLocked: false,
          color: undefined,
          description: "Default layer",
          attributes: [],
        },
      ],
      activeLayer: "default",
      groups: [],
      canScale: false,
      canMirror: false,
      unit: "m",
      location: undefined,
      authors: [],
      concepts: ["hello-semio", "example"],
      icon: "",
      image: "",
      description: "A simple design with two unconnected pieces ready to be connected",
      attributes: [],
    },
  ],
  qualities: [],
  files: [],
  authors: [],
  remote: "",
  homepage: "",
  license: "MIT",
  concepts: ["hello-semio", "example"],
  icon: "",
  image: "",
  description: "An example kit demonstrating the basic structure",
  attributes: [],
};

// Design app state - active with selection ready to connect
export const exampleDesignAppState = {
  // Selection state
  selectedPieces: [piece1Id, piece2Id],
  selectedConnections: [],
  selectedPorts: [],
  hoveredPiece: undefined,
  hoveredConnection: undefined,
  hoveredPort: undefined,
  
  // Tool state
  activeTool: "selection-normal",
  
  // Panel visibility
  panels: {
    workbench: true,
    details: true,
    settings: false,
    tools: true,
    hud: false,
    stats: false,
    chat: false,
    toolbar: true,
  },
  
  // Transaction state
  isTransactionActive: false,
  
  // Window layout
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
};

// Full initial state with kits included
export const fullInitialState: ExtendedInitialState = {
  ...exampleInitialState,
  
  // Kits to be created on initialization
  kits: [
    {
      kit: exampleKit,
      local: true,  // Persist to IndexedDB
      remote: false, // Don't sync to remote
    },
  ],
};

// Usage example in your app:
// ```tsx
// import { fullInitialState } from './initial-state-example';
// 
// function App() {
//   return (
//     <Sketchpad 
//       id="hello-semio-example"
//       initialState={fullInitialState}
//     />
//   );
// }
// ```
//
// This will:
// 1. Create the Sketchpad with specified settings (expertise: NORMAL, mode: USER)
// 2. Create the example kit with one type (Connector Block with 2 ports)
// 3. Create the example design with two pieces ready to connect
// 4. Navigate to the design app view
// 5. Select both pieces, ready for the user to connect them
//
// The design app will be in "selection-normal" tool mode with both pieces selected,
// making it easy to demonstrate the connection workflow.

// Note: After kits are created, they're stored in Y.js structures and synced to IndexedDB.
// The initial state is only applied on first load or when explicitly provided.
