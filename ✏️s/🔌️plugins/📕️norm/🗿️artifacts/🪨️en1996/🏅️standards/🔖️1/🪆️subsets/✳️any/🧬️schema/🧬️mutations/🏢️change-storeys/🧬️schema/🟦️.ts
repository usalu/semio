/** 🏢️ `change-storeys` wire twin: the leaf payload `ChangeStoreys`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreys {
  newStoreys: number;
}

export const parseChangeStoreys: NormWireReader<ChangeStoreys> = normWireObject<ChangeStoreys>({ newStoreys: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
