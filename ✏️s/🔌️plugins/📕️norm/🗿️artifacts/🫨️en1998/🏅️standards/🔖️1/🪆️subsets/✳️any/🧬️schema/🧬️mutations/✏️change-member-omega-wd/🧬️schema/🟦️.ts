/** ✏️ `change-member-omega-wd` wire twin: the leaf payload `ChangeMemberOmegaWd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberOmegaWd {
  mutation: "changeMemberOmegaWd";
  buildingId: string;
  id: string;
  newOmegaWd: number;
}

export const parseChangeMemberOmegaWd: NormWireReader<ChangeMemberOmegaWd> = normWireObject<ChangeMemberOmegaWd>({ mutation: normWireRequired(normWireLiteral("changeMemberOmegaWd")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newOmegaWd: normWireRequired(normWireNumber) });
