/** 🏗️ `change-member-frame-height` wire twin: the leaf payload `ChangeMemberFrameHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberFrameHeight {
  mutation: "changeMemberFrameHeight";
  id: string;
  newFrameHeight: number;
}

export const parseChangeMemberFrameHeight: NormWireReader<ChangeMemberFrameHeight> = normWireObject<ChangeMemberFrameHeight>({ mutation: normWireRequired(normWireLiteral("changeMemberFrameHeight")), id: normWireRequired(normWireString), newFrameHeight: normWireRequired(normWireNumber) });
