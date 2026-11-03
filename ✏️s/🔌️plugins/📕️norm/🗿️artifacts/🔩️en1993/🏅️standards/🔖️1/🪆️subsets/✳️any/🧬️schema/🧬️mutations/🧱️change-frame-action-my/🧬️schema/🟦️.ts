/** 🧱️ `change-frame-action-my` wire twin: the leaf payload `ChangeFrameActionMy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionMy {
  id: string;
  newMy: number;
}

export const parseChangeFrameActionMy: NormWireReader<ChangeFrameActionMy> = normWireObject<ChangeFrameActionMy>({ id: normWireRequired(normWireString), newMy: normWireRequired(normWireNumber) });
