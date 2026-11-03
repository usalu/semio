/** 🔩️ `change-joint-plate-width` wire twin: the leaf payload `ChangeJointPlateWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateWidth {
  id: string;
  newPlateWidth: number;
}

export const parseChangeJointPlateWidth: NormWireReader<ChangeJointPlateWidth> = normWireObject<ChangeJointPlateWidth>({ id: normWireRequired(normWireString), newPlateWidth: normWireRequired(normWireNumber) });
