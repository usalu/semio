/** ➖️ `remove-joint-plate` wire twin: the leaf payload `RemoveJointPlate`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveJointPlate {
  index: number;
}

export const parseRemoveJointPlate: NormWireReader<RemoveJointPlate> = normWireObject<RemoveJointPlate>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
