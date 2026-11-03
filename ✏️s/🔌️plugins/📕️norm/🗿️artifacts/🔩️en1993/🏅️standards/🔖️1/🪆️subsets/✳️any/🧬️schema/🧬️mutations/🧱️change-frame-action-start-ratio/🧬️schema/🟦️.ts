/** 🧱️ `change-frame-action-start-ratio` wire twin: the leaf payload `ChangeFrameActionStartRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionStartRatio {
  id: string;
  newStartRatio: number;
}

export const parseChangeFrameActionStartRatio: NormWireReader<ChangeFrameActionStartRatio> = normWireObject<ChangeFrameActionStartRatio>({ id: normWireRequired(normWireString), newStartRatio: normWireRequired(normWireNumber) });
