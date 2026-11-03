/** 🏗️ `change-crane-fatigue-phi` wire twin: the leaf payload `ChangeCraneFatiguePhi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneFatiguePhi {
  id: string;
  newFatiguePhi: number;
}

export const parseChangeCraneFatiguePhi: NormWireReader<ChangeCraneFatiguePhi> = normWireObject<ChangeCraneFatiguePhi>({ id: normWireRequired(normWireString), newFatiguePhi: normWireRequired(normWireNumber) });
