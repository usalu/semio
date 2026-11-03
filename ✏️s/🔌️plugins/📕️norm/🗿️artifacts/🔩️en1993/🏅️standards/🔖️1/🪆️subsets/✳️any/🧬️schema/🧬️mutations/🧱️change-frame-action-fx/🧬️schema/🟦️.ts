/** 🧱️ `change-frame-action-fx` wire twin: the leaf payload `ChangeFrameActionFx`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionFx {
  id: string;
  newFx: number;
}

export const parseChangeFrameActionFx: NormWireReader<ChangeFrameActionFx> = normWireObject<ChangeFrameActionFx>({ id: normWireRequired(normWireString), newFx: normWireRequired(normWireNumber) });
