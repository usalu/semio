/** 🏗️ `change-member-rd-str` wire twin: the leaf payload `ChangeMemberRdStr`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRdStr {
  mutation: "changeMemberRdStr";
  id: string;
  newRdStr: number;
}

export const parseChangeMemberRdStr: NormWireReader<ChangeMemberRdStr> = normWireObject<ChangeMemberRdStr>({ mutation: normWireRequired(normWireLiteral("changeMemberRdStr")), id: normWireRequired(normWireString), newRdStr: normWireRequired(normWireNumber) });
