/** 🔩️ `change-fastener-columns` wire twin: the leaf payload `ChangeFastenerColumns`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerColumns {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerColumns: NormWireReader<ChangeFastenerColumns> = normWireObject<ChangeFastenerColumns>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
