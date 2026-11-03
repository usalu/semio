/** 🏋️ `change-action-n` wire twin: the leaf payload `ChangeActionN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionN {
  id: string;
  newActionN: number;
}

export const parseChangeActionN: NormWireReader<ChangeActionN> = normWireObject<ChangeActionN>({ id: normWireRequired(normWireString), newActionN: normWireRequired(normWireNumber) });
