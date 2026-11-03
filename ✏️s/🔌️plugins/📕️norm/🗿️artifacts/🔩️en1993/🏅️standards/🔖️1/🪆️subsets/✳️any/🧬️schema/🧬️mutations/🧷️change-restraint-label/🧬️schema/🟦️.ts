/** 🧷️ `change-restraint-label` wire twin: the leaf payload `ChangeRestraintLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRestraintLabel {
  id: string;
  newLabel: string;
}

export const parseChangeRestraintLabel: NormWireReader<ChangeRestraintLabel> = normWireObject<ChangeRestraintLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
