/** 🕳️ `change-hollow-gap` wire twin: the leaf payload `ChangeHollowGap`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowGap {
  id: string;
  newGap: number;
}

export const parseChangeHollowGap: NormWireReader<ChangeHollowGap> = normWireObject<ChangeHollowGap>({ id: normWireRequired(normWireString), newGap: normWireRequired(normWireNumber) });
