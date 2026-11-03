/** ✏️ `change-member-wall-poisson-ratio` wire twin: the leaf payload `ChangeMemberWallPoissonRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberWallPoissonRatio {
  mutation: "changeMemberWallPoissonRatio";
  buildingId: string;
  id: string;
  newWallPoissonRatio: number;
}

export const parseChangeMemberWallPoissonRatio: NormWireReader<ChangeMemberWallPoissonRatio> = normWireObject<ChangeMemberWallPoissonRatio>({ mutation: normWireRequired(normWireLiteral("changeMemberWallPoissonRatio")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newWallPoissonRatio: normWireRequired(normWireNumber) });
