/** 🏋️ `change-action-my` wire twin: the leaf payload `ChangeActionMy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionMy {
  id: string;
  newActionMy: number;
}

export const parseChangeActionMy: NormWireReader<ChangeActionMy> = normWireObject<ChangeActionMy>({ id: normWireRequired(normWireString), newActionMy: normWireRequired(normWireNumber) });
