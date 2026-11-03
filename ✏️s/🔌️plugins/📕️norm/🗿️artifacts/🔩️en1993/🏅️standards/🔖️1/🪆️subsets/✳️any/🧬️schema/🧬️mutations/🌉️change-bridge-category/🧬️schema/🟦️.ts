/** 🌉️ `change-bridge-category` wire twin: the leaf payload `ChangeBridgeCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeCategory {
  id: string;
  newCategory: number;
}

export const parseChangeBridgeCategory: NormWireReader<ChangeBridgeCategory> = normWireObject<ChangeBridgeCategory>({ id: normWireRequired(normWireString), newCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
