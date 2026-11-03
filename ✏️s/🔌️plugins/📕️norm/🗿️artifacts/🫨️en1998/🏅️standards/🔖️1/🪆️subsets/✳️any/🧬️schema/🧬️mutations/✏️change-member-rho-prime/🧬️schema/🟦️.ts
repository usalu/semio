/** ✏️ `change-member-rho-prime` wire twin: the leaf payload `ChangeMemberRhoPrime`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRhoPrime {
  mutation: "changeMemberRhoPrime";
  buildingId: string;
  id: string;
  newRhoPrime: number;
}

export const parseChangeMemberRhoPrime: NormWireReader<ChangeMemberRhoPrime> = normWireObject<ChangeMemberRhoPrime>({ mutation: normWireRequired(normWireLiteral("changeMemberRhoPrime")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newRhoPrime: normWireRequired(normWireNumber) });
