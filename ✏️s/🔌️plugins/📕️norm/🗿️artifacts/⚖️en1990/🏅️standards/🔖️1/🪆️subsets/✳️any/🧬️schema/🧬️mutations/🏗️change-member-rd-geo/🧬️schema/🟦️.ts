/** 🏗️ `change-member-rd-geo` wire twin: the leaf payload `ChangeMemberRdGeo`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRdGeo {
  mutation: "changeMemberRdGeo";
  id: string;
  newRdGeo: number;
}

export const parseChangeMemberRdGeo: NormWireReader<ChangeMemberRdGeo> = normWireObject<ChangeMemberRdGeo>({ mutation: normWireRequired(normWireLiteral("changeMemberRdGeo")), id: normWireRequired(normWireString), newRdGeo: normWireRequired(normWireNumber) });
