/** 🏗️ `change-member-label-en` wire twin: the leaf payload `ChangeMemberLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLabelEn {
  mutation: "changeMemberLabelEn";
  id: string;
  newLabelEn: string;
}

export const parseChangeMemberLabelEn: NormWireReader<ChangeMemberLabelEn> = normWireObject<ChangeMemberLabelEn>({ mutation: normWireRequired(normWireLiteral("changeMemberLabelEn")), id: normWireRequired(normWireString), newLabelEn: normWireRequired(normWireString) });
