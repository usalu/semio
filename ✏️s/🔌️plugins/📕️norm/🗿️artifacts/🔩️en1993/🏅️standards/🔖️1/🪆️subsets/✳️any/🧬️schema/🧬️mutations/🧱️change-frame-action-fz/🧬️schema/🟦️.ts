/** 🧱️ `change-frame-action-fz` wire twin: the leaf payload `ChangeFrameActionFz`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionFz {
  id: string;
  newFz: number;
}

export const parseChangeFrameActionFz: NormWireReader<ChangeFrameActionFz> = normWireObject<ChangeFrameActionFz>({ id: normWireRequired(normWireString), newFz: normWireRequired(normWireNumber) });
