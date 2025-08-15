import { Camera, ConnectionId, DesignDiff, DiagramPoint, Kit, KitDiff, KitId, PieceId, PortId, Type, TypeDiff, TypeId } from "./semio";

export enum Mode {
  USER = "user",
  GUEST = "guest",
}

export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

export enum Layout {
  NORMAL = "normal",
  TOUCH = "touch",
}

// Yjs alias value types used in this store (precise, no any)
type YAuthor = Y.Map<string>;
type YAuthors = Y.Map<YAuthor>;
type YQuality = Y.Map<string>;
type YQualities = Y.Array<YQuality>;
type YStringArray = Y.Array<string>;
type YLeafMapString = Y.Map<string>;
type YLeafMapNumber = Y.Map<number>;
type YVec3 = YLeafMapNumber;
type YPlane = Y.Map<YVec3>;

type YRepresentationVal = string | YStringArray | YQualities;
type YRepresentation = Y.Map<YRepresentationVal>;
type YRepresentationMap = Y.Map<YRepresentation>;

type YPortVal = string | number | boolean | YLeafMapNumber | YQualities | YStringArray;
type YPort = Y.Map<YPortVal>;
type YPortMap = Y.Map<YPort>;

type YPieceVal = string | YLeafMapString | YLeafMapNumber | YPlane | YQualities;
type YPiece = Y.Map<YPieceVal>;
type YPieceMap = Y.Map<YPiece>;

type YSide = Y.Map<YLeafMapString>;
type YConnectionVal = string | number | YQualities | YSide;
type YConnection = Y.Map<YConnectionVal>;
type YConnectionMap = Y.Map<YConnection>;

type YTypeVal = string | number | boolean | YAuthors | YQualities | YRepresentationMap | YPortMap;
type YType = Y.Map<YTypeVal>;
type YTypeMap = Y.Map<YType>;

type YDesignVal = string | YAuthors | YQualities | YPieceMap | YConnectionMap;
type YDesign = Y.Map<YDesignVal>;
type YDesignMap = Y.Map<YDesign>;

type YDesignEditorStoreVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YQualities | YStringArray;
type YDesignEditorStore = Y.Map<YDesignEditorStoreVal>;
type YDesignEditorStoreMap = Y.Map<YDesignEditorStore>;

type YIdMap = Y.Map<string>;
type YKitVal = string | YTypeMap | YDesignMap | YIdMap | YQualities;
type YKit = Y.Map<YKitVal>;

// Typed key maps and getters
type YKitKeysMap = {
  uuid: string;
  name: string;
  description: string;
  icon: string;
  image: string;
  version: string;
  preview: string;
  remote: string;
  homepage: string;
  license: string;
  created: string;
  updated: string;
  types: YTypeMap;
  designs: YDesignMap;
  typeIds: YIdMap;
  designIds: YIdMap;
  pieceIds: YIdMap;
  connectionIds: YIdMap;
  portIds: YIdMap;
  representationIds: YIdMap;
  qualities: YQualities;
};
const getKit = <K extends keyof YKitKeysMap>(m: YKit, k: K): YKitKeysMap[K] => m.get(k as string) as YKitKeysMap[K];

type YTypeKeysMap = {
  name: string;
  description: string;
  icon: string;
  image: string;
  variant: string;
  stock: number;
  virtual: boolean;
  unit: string;
  representations: YRepresentationMap;
  ports: YPortMap;
  authors: YAuthors;
  qualities: YQualities;
  created: string;
  updated: string;
};
const getType = <K extends keyof YTypeKeysMap>(m: YType, k: K): YTypeKeysMap[K] => m.get(k as string) as YTypeKeysMap[K];

type YDesignKeysMap = {
  name: string;
  description: string;
  icon: string;
  image: string;
  variant: string;
  view: string;
  unit: string;
  created: string;
  updated: string;
  authors: YAuthors;
  pieces: YPieceMap;
  connections: YConnectionMap;
  qualities: YQualities;
};
const getDesign = <K extends keyof YDesignKeysMap>(m: YDesign, k: K): YDesignKeysMap[K] => m.get(k as string) as YDesignKeysMap[K];

type YRepresentationKeysMap = {
  url: string;
  description: string;
  tags: YStringArray;
  qualities: YQualities;
};
const getRep = <K extends keyof YRepresentationKeysMap>(m: YRepresentation, k: K): YRepresentationKeysMap[K] => m.get(k as string) as YRepresentationKeysMap[K];

type YPortKeysMap = {
  id_: string;
  description: string;
  mandatory: boolean;
  family: string;
  compatibleFamilies: YStringArray;
  direction: YVec3;
  point: YVec3;
  t: number;
  qualities: YQualities;
};
const getPort = <K extends keyof YPortKeysMap>(m: YPort, k: K): YPortKeysMap[K] => m.get(k as string) as YPortKeysMap[K];

type YPieceKeysMap = {
  id_: string;
  description: string;
  type: YLeafMapString; // { name, variant }
  plane: YPlane;
  center: YLeafMapNumber; // { x,y }
  qualities: YQualities;
};
const getPiece = <K extends keyof YPieceKeysMap>(m: YPiece, k: K): YPieceKeysMap[K] => m.get(k as string) as YPieceKeysMap[K];

type YSideKeysMap = {
  piece: YLeafMapString; // { id_ }
  port: YLeafMapString; // { id_ }
};
const getSide = <K extends keyof YSideKeysMap>(m: YSide, k: K): YSideKeysMap[K] => m.get(k as string) as YSideKeysMap[K];

type YConnectionKeysMap = {
  connected: YSide;
  connecting: YSide;
  description: string;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
  qualities: YQualities;
};
const getConn = <K extends keyof YConnectionKeysMap>(m: YConnection, k: K): YConnectionKeysMap[K] => m.get(k as string) as YConnectionKeysMap[K];

type YDesignEditorStoreKeysMap = {
  fullscreenPanel: string;
  selectedPieceIds: YStringArray;
  selectedConnections: YStringArray;
  selectedPiecePortPieceId: string;
  selectedPiecePortPortId: string;
  designDiffPiecesAdded: YStringArray;
  designDiffPiecesRemoved: YStringArray;
  designDiffPiecesUpdated: YStringArray;
  designDiffConnectionsAdded: YStringArray;
  designDiffConnectionsRemoved: YStringArray;
  designDiffConnectionsUpdated: YStringArray;
  isTransactionActive: boolean;
  presenceCursorX: number;
  presenceCursorY: number;
  presenceCameraPositionX: number;
  presenceCameraPositionY: number;
  presenceCameraPositionZ: number;
  presenceCameraForwardX: number;
  presenceCameraForwardY: number;
  presenceCameraForwardZ: number;
};
const getDesignEditorStore = <K extends keyof YDesignEditorStoreKeysMap>(m: YDesignEditorStore, k: K): YDesignEditorStoreKeysMap[K] => m.get(k as string) as YDesignEditorStoreKeysMap[K];

// Sketchpad state interface
interface SketchpadState {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorId?: string;
}

// Typed key maps for sketchpad state
type YSketchpadVal = string | boolean;
type YSketchpad = Y.Map<YSketchpadVal>;

type YSketchpadKeysMap = {
  mode: string;
  theme: string;
  layout: string;
  activeDesignEditorId: string;
};
const getSketchpadStore = <K extends keyof YSketchpadKeysMap>(m: YSketchpad, k: K): YSketchpadKeysMap[K] => m.get(k as string) as YSketchpadKeysMap[K];

type KitState = {
  kit: Kit;
};

type KitActions = {
  create: {
    kit: (kit: Kit) => void;
    type: (kitId: KitId, type: Type) => void;
  };
  update: {
    kit: (diff: KitDiff) => void;
    type: (kitId: KitId, diff: TypeDiff) => void;
  };
  delete: {
    type: (kitId: KitId, id: TypeId) => void;
  };
};

type KitStore = KitState & KitActions;

export interface DesignEditorSelection {
  selectedPieceIds: PieceId[];
  selectedConnections: ConnectionId[];
  selectedPiecePortId?: { pieceId: PieceId; portId: PortId };
}

export type DesignEditorFullscreenPanel = "none" | "diagram" | "model";

export interface DesignEditorPresence {
  cursor?: DiagramPoint;
  camera?: Camera;
}

export interface DesignEditorPresenceOther extends DesignEditorPresence {
  name: string;
}

type DesignEditorState = {
  fullscreenPanel: DesignEditorFullscreenPanel;
  selection: DesignEditorSelection;
  designDiff: DesignDiff;
  isTransactionActive: boolean;
  presence: DesignEditorPresence;
  others: DesignEditorPresenceOther[];
};

type DesignEditorActions = {
  set: {
    fullscreenPanel: (fullscreenPanel: DesignEditorFullscreenPanel) => void;
    selection: (selection: DesignEditorSelection) => void;
    designDiff: (designDiff: DesignDiff) => void;
    isTransactionActive: (isTransactionActive: boolean) => void;
    presence: (presence: DesignEditorPresence) => void;
    others: (others: DesignEditorPresenceOther[]) => void;
  };
};

type DesignEditorStore = DesignEditorState & DesignEditorActions;

type SketchpadState = {
  mode: Mode;
  theme: Theme;
  layout: Layout;
  activeDesignEditorId?: string;
};

type SketchpadActions = {
  set: {
    mode: (mode: Mode) => void;
    theme: (theme: Theme) => void;
    layout: (layout: Layout) => void;
    activeDesignEditorId: (id?: string) => void;
  };
};

type SketchpadStore = SketchpadState &
  SketchpadActions & {
    kits: KitStore[];
    designEditors: DesignEditorStore[];
  };

export function useKit<T>(selector?: (kit: Kit) => T, id?: KitId): T {}

export function useDesignEditor<T>(selector?: (editor: DesignEditorState) => T, id?: string): T {}

export function useSketchpad<T>(selector?: (sketchpad: SketchpadState) => T, id?: string): T {}
