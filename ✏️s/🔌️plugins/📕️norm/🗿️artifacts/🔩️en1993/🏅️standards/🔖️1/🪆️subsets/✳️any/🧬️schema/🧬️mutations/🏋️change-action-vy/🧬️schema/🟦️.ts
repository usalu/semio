/** 🏋️ `change-action-vy` wire twin: the leaf payload `ChangeActionVy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionVy {
  id: string;
  newActionVy: number;
}

export const parseChangeActionVy: NormWireReader<ChangeActionVy> = normWireObject<ChangeActionVy>({ id: normWireRequired(normWireString), newActionVy: normWireRequired(normWireNumber) });
