/** 🗂️ `remove-fatigue-detail` wire twin: the leaf payload `RemoveFatigueDetail`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFatigueDetail {
  mutation: "removeFatigueDetail";
  id: string;
}

export const parseRemoveFatigueDetail: NormWireReader<RemoveFatigueDetail> = normWireObject<RemoveFatigueDetail>({ mutation: normWireRequired(normWireLiteral("removeFatigueDetail")), id: normWireRequired(normWireString) });
