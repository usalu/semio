/** 🔩️ `change-row-label` wire twin: the leaf payload `ChangeRowLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowLabel {
  id: string;
  newLabel: string;
}

export const parseChangeRowLabel: NormWireReader<ChangeRowLabel> = normWireObject<ChangeRowLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
