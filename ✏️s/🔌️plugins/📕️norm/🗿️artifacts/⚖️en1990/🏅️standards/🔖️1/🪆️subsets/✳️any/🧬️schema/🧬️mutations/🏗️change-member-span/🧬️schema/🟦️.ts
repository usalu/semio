/** 🏗️ `change-member-span` wire twin: the leaf payload `ChangeMemberSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberSpan {
  mutation: "changeMemberSpan";
  id: string;
  newSpan: number;
}

export const parseChangeMemberSpan: NormWireReader<ChangeMemberSpan> = normWireObject<ChangeMemberSpan>({ mutation: normWireRequired(normWireLiteral("changeMemberSpan")), id: normWireRequired(normWireString), newSpan: normWireRequired(normWireNumber) });
