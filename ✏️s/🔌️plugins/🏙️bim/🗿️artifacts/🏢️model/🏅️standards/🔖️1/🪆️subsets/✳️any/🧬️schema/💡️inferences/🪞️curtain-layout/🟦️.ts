/** 🪞️ `curtain-layout`: the resolved vertical extent, axis length and panel grid of a curtain wall. */

export interface CurtainLayout {
  base_z: number;
  top_z: number;
  height: number;
  length: number;
  area: number;
  u_panels: number;
  v_panels: number;
  panel_width: number;
  panel_height: number;
}
