/** 🪞️ `curtain-layout`: the resolved vertical extent, axis length and panel grid of a curtain wall. */

export type CurtainPanel = "Glass" | "Empty" | { Solid: { material: string } } | { Door: { door_type: string } } | { Window: { window_type: string } };

export interface CellPanel {
  u: number;
  v: number;
  panel: CurtainPanel;
  id: string;
}

export interface CurtainLayout {
  base_z: number;
  top_z: number;
  height: number;
  length: number;
  area: number;
  u_panels: number;
  v_panels: number;
  u_edges: number[];
  v_edges: number[];
  panel?: CurtainPanel;
  overrides: CellPanel[];
  stray: string[];
  repeated: string[];
  ignored_u: number[];
  ignored_v: number[];
}
