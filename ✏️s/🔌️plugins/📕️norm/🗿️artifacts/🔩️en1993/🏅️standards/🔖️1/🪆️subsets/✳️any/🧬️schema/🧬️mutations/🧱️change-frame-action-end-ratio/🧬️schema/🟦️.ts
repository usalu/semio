/** 🧱️ `change-frame-action-end-ratio` wire twin: the leaf payload `ChangeFrameActionEndRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameActionEndRatio {
  id: string;
  newEndRatio: number;
}

export const parseChangeFrameActionEndRatio: NormWireReader<ChangeFrameActionEndRatio> = normWireObject<ChangeFrameActionEndRatio>({ id: normWireRequired(normWireString), newEndRatio: normWireRequired(normWireNumber) });
