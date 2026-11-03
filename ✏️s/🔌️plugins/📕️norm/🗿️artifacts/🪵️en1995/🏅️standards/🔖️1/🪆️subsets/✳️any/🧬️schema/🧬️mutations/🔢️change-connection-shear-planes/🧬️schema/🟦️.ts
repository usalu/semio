/** 🔢️ `change-connection-shear-planes` wire twin: the leaf payload `ChangeConnectionShearPlanes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnectionShearPlanes {
  connectionId: string;
  newValue: number;
}

export const parseChangeConnectionShearPlanes: NormWireReader<ChangeConnectionShearPlanes> = normWireObject<ChangeConnectionShearPlanes>({ connectionId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
