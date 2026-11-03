/** ✏️ `change-member-wall-eff-height-m` wire twin: the leaf payload `ChangeMemberWallEffHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallEffHeightM {
  mutation: "changeMemberWallEffHeightM";
  buildingId: string;
  id: string;
  newWallEffectiveHeightM: number;
}

export const parseChangeMemberWallEffHeightM: NormWireReader<ChangeMemberWallEffHeightM> = normWireObject<ChangeMemberWallEffHeightM>({ mutation: normWireRequired(normWireLiteral("changeMemberWallEffHeightM")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallEffectiveHeightM: normWireRequired(normWireNumber) });
