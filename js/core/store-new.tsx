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
