/** 🕳️ `change-hollow-label` wire twin: the leaf payload `ChangeHollowLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHollowLabel {
  id: string;
  newLabel: string;
}

export const parseChangeHollowLabel: NormWireReader<ChangeHollowLabel> = normWireObject<ChangeHollowLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
