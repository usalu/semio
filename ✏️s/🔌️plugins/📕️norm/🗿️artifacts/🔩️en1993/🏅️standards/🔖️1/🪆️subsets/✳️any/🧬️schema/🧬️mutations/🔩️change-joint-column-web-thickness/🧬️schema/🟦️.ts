/** 🔩️ `change-joint-column-web-thickness` wire twin: the leaf payload `ChangeJointColumnWebThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnWebThickness {
  id: string;
  newColumnWebThickness: number;
}

export const parseChangeJointColumnWebThickness: NormWireReader<ChangeJointColumnWebThickness> = normWireObject<ChangeJointColumnWebThickness>({ id: normWireRequired(normWireString), newColumnWebThickness: normWireRequired(normWireNumber) });
