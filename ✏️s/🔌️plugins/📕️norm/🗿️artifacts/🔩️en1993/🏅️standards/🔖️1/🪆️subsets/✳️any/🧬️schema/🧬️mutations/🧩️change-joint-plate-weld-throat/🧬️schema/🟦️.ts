/** 🧩️ `change-joint-plate-weld-throat` wire twin: the leaf payload `ChangeJointPlateWeldThroat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateWeldThroat {
  id: string;
  newWeldThroat: number;
}

export const parseChangeJointPlateWeldThroat: NormWireReader<ChangeJointPlateWeldThroat> = normWireObject<ChangeJointPlateWeldThroat>({ id: normWireRequired(normWireString), newWeldThroat: normWireRequired(normWireNumber) });
