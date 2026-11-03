/** ➖️ `remove-fatigue-stress` wire twin: the leaf payload `RemoveFatigueStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueStress {
  index: number;
}

export const parseRemoveFatigueStress: NormWireReader<RemoveFatigueStress> = normWireObject<RemoveFatigueStress>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
