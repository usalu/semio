/** 🏠 `change-roof-surface-extent` wire twin: the leaf payload `ChangeRoofSurfaceExtent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofSurfaceExtent {
  id: string;
  newExtent: number;
}

export const parseChangeRoofSurfaceExtent: NormWireReader<ChangeRoofSurfaceExtent> = normWireObject<ChangeRoofSurfaceExtent>({ id: normWireRequired(normWireString), newExtent: normWireRequired(normWireNumber) });
