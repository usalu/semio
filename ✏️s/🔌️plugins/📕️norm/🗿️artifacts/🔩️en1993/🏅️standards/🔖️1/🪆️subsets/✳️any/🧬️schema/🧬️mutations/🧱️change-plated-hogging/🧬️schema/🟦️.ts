/** 🧱️ `change-plated-hogging` wire twin: the leaf payload `ChangePlatedHogging`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedHogging {
  id: string;
  newHogging: number;
}

export const parseChangePlatedHogging: NormWireReader<ChangePlatedHogging> = normWireObject<ChangePlatedHogging>({ id: normWireRequired(normWireString), newHogging: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
