/** ➖️ `remove-tower-leg` wire twin: the leaf payload `RemoveTowerLeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTowerLeg {
  index: number;
}

export const parseRemoveTowerLeg: NormWireReader<RemoveTowerLeg> = normWireObject<RemoveTowerLeg>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
