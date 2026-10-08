/** 🪜️ `storey-levels`: level 0 is the building datum; positive levels stack upward, negative levels downward. */

export interface StoreyLevel {
  elevation: number;
  top_elevation: number;
  absolute_elevation: number;
  absolute_top_elevation: number;
}
