/** 🧷️ `change-restraint-general-method` wire twin: the leaf payload `ChangeRestraintGeneralMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintGeneralMethod {
  id: string;
  newGeneralMethod: number;
}

export const parseChangeRestraintGeneralMethod: NormWireReader<ChangeRestraintGeneralMethod> = normWireObject<ChangeRestraintGeneralMethod>({ id: normWireRequired(normWireString), newGeneralMethod: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
