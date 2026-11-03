/** 🏗️ `change-crane-fy` wire twin: the leaf payload `ChangeCraneFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneFy {
  id: string;
  newFy: number;
}

export const parseChangeCraneFy: NormWireReader<ChangeCraneFy> = normWireObject<ChangeCraneFy>({ id: normWireRequired(normWireString), newFy: normWireRequired(normWireNumber) });
