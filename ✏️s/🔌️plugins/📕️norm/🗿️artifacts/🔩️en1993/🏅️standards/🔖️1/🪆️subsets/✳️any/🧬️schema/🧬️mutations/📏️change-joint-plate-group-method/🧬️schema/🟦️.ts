/** 📏️ `change-joint-plate-group-method` wire twin: the leaf payload `ChangeJointPlateGroupMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BoltGroupMethod, parseBoltGroupMethod } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeJointPlateGroupMethod {
  id: string;
  newGroupMethod: BoltGroupMethod;
}

export const parseChangeJointPlateGroupMethod: NormWireReader<ChangeJointPlateGroupMethod> = normWireObject<ChangeJointPlateGroupMethod>({ id: normWireRequired(normWireString), newGroupMethod: normWireRequired(parseBoltGroupMethod) });
