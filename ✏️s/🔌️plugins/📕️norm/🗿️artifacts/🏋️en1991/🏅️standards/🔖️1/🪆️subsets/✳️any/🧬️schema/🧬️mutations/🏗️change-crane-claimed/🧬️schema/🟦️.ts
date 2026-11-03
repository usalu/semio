/** 🏗️ `change-crane-claimed` wire twin: the leaf payload `ChangeCraneClaimed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneClaimed {
  newCraneClaimed: boolean;
}

export const parseChangeCraneClaimed: NormWireReader<ChangeCraneClaimed> = normWireObject<ChangeCraneClaimed>({ newCraneClaimed: normWireRequired(normWireBoolean) });
