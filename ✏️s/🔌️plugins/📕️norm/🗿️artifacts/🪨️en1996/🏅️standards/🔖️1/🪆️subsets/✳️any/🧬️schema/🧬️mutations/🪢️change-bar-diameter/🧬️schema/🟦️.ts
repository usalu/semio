/** 🪢️ `change-bar-diameter` wire twin: the leaf payload `ChangeBarDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarDiameter {
  wallId: string;
  id: string;
  newDiameterM: number;
}

export const parseChangeBarDiameter: NormWireReader<ChangeBarDiameter> = normWireObject<ChangeBarDiameter>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newDiameterM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
