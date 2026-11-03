/** 🥶️ `change-cf-support-reaction` wire twin: the leaf payload `ChangeCfSupportReaction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfSupportReaction {
  id: string;
  newSupportReaction: number;
}

export const parseChangeCfSupportReaction: NormWireReader<ChangeCfSupportReaction> = normWireObject<ChangeCfSupportReaction>({ id: normWireRequired(normWireString), newSupportReaction: normWireRequired(normWireNumber) });
