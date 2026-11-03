/** 🏠 `change-roof-surface-shape` wire twin: the leaf payload `ChangeRoofSurfaceShape`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofSurfaceShape {
  id: string;
  newShape: string;
}

export const parseChangeRoofSurfaceShape: NormWireReader<ChangeRoofSurfaceShape> = normWireObject<ChangeRoofSurfaceShape>({ id: normWireRequired(normWireString), newShape: normWireRequired(normWireString) });
