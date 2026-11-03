/** 🕸️ `change-frame-bar-member-id` wire twin: the leaf payload `ChangeFrameBarMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFrameBarMemberId {
  id: string;
  newMemberId: string;
}

export const parseChangeFrameBarMemberId: NormWireReader<ChangeFrameBarMemberId> = normWireObject<ChangeFrameBarMemberId>({ id: normWireRequired(normWireString), newMemberId: normWireRequired(normWireString) });
