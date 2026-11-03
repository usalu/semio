/** 🏗️ `change-member-storey-height` wire twin: the leaf payload `ChangeMemberStoreyHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStoreyHeight {
  mutation: "changeMemberStoreyHeight";
  id: string;
  newStoreyHeight: number;
}

export const parseChangeMemberStoreyHeight: NormWireReader<ChangeMemberStoreyHeight> = normWireObject<ChangeMemberStoreyHeight>({ mutation: normWireRequired(normWireLiteral("changeMemberStoreyHeight")), id: normWireRequired(normWireString), newStoreyHeight: normWireRequired(normWireNumber) });
