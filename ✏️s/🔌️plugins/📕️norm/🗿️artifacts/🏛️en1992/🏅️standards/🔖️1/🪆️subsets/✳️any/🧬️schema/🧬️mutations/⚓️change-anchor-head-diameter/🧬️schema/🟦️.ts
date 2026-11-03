/** ⚓️ `change-anchor-head-diameter` wire twin: the leaf payload `ChangeAnchorHeadDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorHeadDiameter {
  anchorId: string;
  newHeadDiameter: number;
}

export const parseChangeAnchorHeadDiameter: NormWireReader<ChangeAnchorHeadDiameter> = normWireObject<ChangeAnchorHeadDiameter>({ anchorId: normWireRequired(normWireString), newHeadDiameter: normWireRequired(normWireNumber) });
