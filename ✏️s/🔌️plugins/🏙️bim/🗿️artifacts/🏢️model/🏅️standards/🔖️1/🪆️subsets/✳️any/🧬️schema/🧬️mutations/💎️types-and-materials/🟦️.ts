/** 💎️ BIM model direct mutations for materials and element types, including their thermal data. */
import type { BeamType, CeilingType, ColumnType, CurtainGrid, CurtainPanel, CurtainWallType, DoorLeaves, DoorType, Layer, Material, MaterialCategory, Profile, Rgb, RoofType, SlabType, Swing, WallType, WindowType } from "../../🟦️.ts";

export interface CreateMaterial {
  mutation: "createMaterial";
  id: string;
  material: Material;
}

export interface DeleteMaterial {
  mutation: "deleteMaterial";
  id: string;
}

export interface SetMaterial {
  mutation: "setMaterial";
  id: string;
  name?: string;
  category?: MaterialCategory;
  color?: Rgb;
  density?: number;
  conductivity?: number;
  specific_heat?: number;
}

export interface CreateWallType {
  mutation: "createWallType";
  id: string;
  wall_type: WallType;
}

export interface DeleteWallType {
  mutation: "deleteWallType";
  id: string;
}

export interface SetWallType {
  mutation: "setWallType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateSlabType {
  mutation: "createSlabType";
  id: string;
  slab_type: SlabType;
}

export interface DeleteSlabType {
  mutation: "deleteSlabType";
  id: string;
}

export interface SetSlabType {
  mutation: "setSlabType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateRoofType {
  mutation: "createRoofType";
  id: string;
  roof_type: RoofType;
}

export interface DeleteRoofType {
  mutation: "deleteRoofType";
  id: string;
}

export interface SetRoofType {
  mutation: "setRoofType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateColumnType {
  mutation: "createColumnType";
  id: string;
  column_type: ColumnType;
}

export interface DeleteColumnType {
  mutation: "deleteColumnType";
  id: string;
}

export interface SetColumnType {
  mutation: "setColumnType";
  id: string;
  name?: string;
  profile?: Profile;
  material?: string;
}

export interface CreateBeamType {
  mutation: "createBeamType";
  id: string;
  beam_type: BeamType;
}

export interface DeleteBeamType {
  mutation: "deleteBeamType";
  id: string;
}

export interface SetBeamType {
  mutation: "setBeamType";
  id: string;
  name?: string;
  profile?: Profile;
  material?: string;
}

export interface CreateWindowType {
  mutation: "createWindowType";
  id: string;
  window_type: WindowType;
}

export interface DeleteWindowType {
  mutation: "deleteWindowType";
  id: string;
}

export interface SetWindowType {
  mutation: "setWindowType";
  id: string;
  name?: string;
  width?: number;
  height?: number;
  sill?: number;
  frame_width?: number;
  frame_depth?: number;
  panes?: number;
  material?: string;
}

export interface CreateDoorType {
  mutation: "createDoorType";
  id: string;
  door_type: DoorType;
}

export interface DeleteDoorType {
  mutation: "deleteDoorType";
  id: string;
}

export interface SetDoorType {
  mutation: "setDoorType";
  id: string;
  name?: string;
  width?: number;
  height?: number;
  frame_width?: number;
  frame_depth?: number;
  leaves?: DoorLeaves;
  swing?: Swing;
  material?: string;
}

export interface CreateCeilingType {
  mutation: "createCeilingType";
  id: string;
  ceiling_type: CeilingType;
}

export interface DeleteCeilingType {
  mutation: "deleteCeilingType";
  id: string;
}

export interface SetCeilingType {
  mutation: "setCeilingType";
  id: string;
  name?: string;
  layers?: Layer[];
}

export interface CreateCurtainWallType {
  mutation: "createCurtainWallType";
  id: string;
  curtain_wall_type: CurtainWallType;
}

export interface SetCurtainWallType {
  mutation: "setCurtainWallType";
  id: string;
  name?: string;
  u_grid?: CurtainGrid;
  v_grid?: CurtainGrid;
  interior_mullion?: Profile;
  border_mullion?: Profile;
  panel?: CurtainPanel;
  panel_material?: string;
  mullion_material?: string;
}

export interface DeleteCurtainWallType {
  mutation: "deleteCurtainWallType";
  id: string;
}

export interface SetTypeThermalData {
  mutation: "setTypeThermalData";
  id: string;
  u_value?: Record<string, unknown>;
  g_value?: Record<string, unknown>;
  frame_fraction?: Record<string, unknown>;
}
