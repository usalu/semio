/** 🏗️ `change-crane-flange-thickness` wire twin: the leaf payload `ChangeCraneFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneFlangeThickness {
  id: string;
  newFlangeThickness: number;
}

export const parseChangeCraneFlangeThickness: NormWireReader<ChangeCraneFlangeThickness> = normWireObject<ChangeCraneFlangeThickness>({ id: normWireRequired(normWireString), newFlangeThickness: normWireRequired(normWireNumber) });
