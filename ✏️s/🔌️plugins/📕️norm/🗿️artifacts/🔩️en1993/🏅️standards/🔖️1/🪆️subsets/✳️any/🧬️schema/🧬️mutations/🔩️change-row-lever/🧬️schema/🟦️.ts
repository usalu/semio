/** 🔩️ `change-row-lever` wire twin: the leaf payload `ChangeRowLever`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowLever {
  id: string;
  newLever: number;
}

export const parseChangeRowLever: NormWireReader<ChangeRowLever> = normWireObject<ChangeRowLever>({ id: normWireRequired(normWireString), newLever: normWireRequired(normWireNumber) });
