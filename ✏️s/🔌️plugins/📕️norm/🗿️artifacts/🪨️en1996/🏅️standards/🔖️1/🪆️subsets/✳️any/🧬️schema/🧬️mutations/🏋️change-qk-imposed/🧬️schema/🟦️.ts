/** 🏋️ `change-qk-imposed` wire twin: the leaf payload `ChangeQKImposed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeQKImposed {
  wallId: string;
  id: string;
  newQKImposedPa: number;
}

export const parseChangeQKImposed: NormWireReader<ChangeQKImposed> = normWireObject<ChangeQKImposed>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newQKImposedPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
