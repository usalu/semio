/** ✏️ `change-member-rho` wire twin: the leaf payload `ChangeMemberRho`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRho {
  mutation: "changeMemberRho";
  buildingId: string;
  id: string;
  newRho: number;
}

export const parseChangeMemberRho: NormWireReader<ChangeMemberRho> = normWireObject<ChangeMemberRho>({ mutation: normWireRequired(normWireLiteral("changeMemberRho")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newRho: normWireRequired(normWireNumber) });
