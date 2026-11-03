/** 🏗️ `change-crane-phi` wire twin: the leaf payload `ChangeCranePhi`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCranePhi {
  id: string;
  newPhi: number;
}

export const parseChangeCranePhi: NormWireReader<ChangeCranePhi> = normWireObject<ChangeCranePhi>({ id: normWireRequired(normWireString), newPhi: normWireRequired(normWireNumber) });
