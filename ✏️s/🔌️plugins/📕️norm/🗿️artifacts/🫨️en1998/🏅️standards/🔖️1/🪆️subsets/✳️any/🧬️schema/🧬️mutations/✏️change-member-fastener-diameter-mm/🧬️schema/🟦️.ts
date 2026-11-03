/** ✏️ `change-member-fastener-diameter-mm` wire twin: the leaf payload `ChangeMemberFastenerDiameterMm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberFastenerDiameterMm {
  mutation: "changeMemberFastenerDiameterMm";
  buildingId: string;
  id: string;
  newFastenerDiameterMm: number;
}

export const parseChangeMemberFastenerDiameterMm: NormWireReader<ChangeMemberFastenerDiameterMm> = normWireObject<ChangeMemberFastenerDiameterMm>({ mutation: normWireRequired(normWireLiteral("changeMemberFastenerDiameterMm")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newFastenerDiameterMm: normWireRequired(normWireNumber) });
