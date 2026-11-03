/** 🔩️ `change-joint-column-web-stress` wire twin: the leaf payload `ChangeJointColumnWebStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnWebStress {
  id: string;
  newColumnWebStress: number;
}

export const parseChangeJointColumnWebStress: NormWireReader<ChangeJointColumnWebStress> = normWireObject<ChangeJointColumnWebStress>({ id: normWireRequired(normWireString), newColumnWebStress: normWireRequired(normWireNumber) });
