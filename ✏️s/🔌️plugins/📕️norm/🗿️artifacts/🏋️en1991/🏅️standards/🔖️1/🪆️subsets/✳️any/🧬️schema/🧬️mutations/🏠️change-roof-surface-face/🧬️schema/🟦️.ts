/** 🏠 `change-roof-surface-face` wire twin: the leaf payload `ChangeRoofSurfaceFace`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofSurfaceFace {
  id: string;
  newFace: string;
}

export const parseChangeRoofSurfaceFace: NormWireReader<ChangeRoofSurfaceFace> = normWireObject<ChangeRoofSurfaceFace>({ id: normWireRequired(normWireString), newFace: normWireRequired(normWireString) });
