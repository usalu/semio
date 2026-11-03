/** 🏗️ `change-crane-crane-kq` wire twin: the leaf payload `ChangeCraneCraneKq`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneCraneKq {
  id: string;
  newCraneKq: number;
}

export const parseChangeCraneCraneKq: NormWireReader<ChangeCraneCraneKq> = normWireObject<ChangeCraneCraneKq>({ id: normWireRequired(normWireString), newCraneKq: normWireRequired(normWireNumber) });
