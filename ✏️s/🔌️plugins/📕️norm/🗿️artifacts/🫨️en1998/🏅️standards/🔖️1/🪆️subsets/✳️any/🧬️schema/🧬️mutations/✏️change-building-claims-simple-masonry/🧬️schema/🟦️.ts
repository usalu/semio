/** ✏️ `change-building-claims-simple-masonry` wire twin: the leaf payload `ChangeBuildingClaimsSimpleMasonry`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingClaimsSimpleMasonry {
  mutation: "changeBuildingClaimsSimpleMasonry";
  id: string;
  newClaimsSimpleMasonry: boolean;
}

export const parseChangeBuildingClaimsSimpleMasonry: NormWireReader<ChangeBuildingClaimsSimpleMasonry> = normWireObject<ChangeBuildingClaimsSimpleMasonry>({ mutation: normWireRequired(normWireLiteral("changeBuildingClaimsSimpleMasonry")), id: normWireRequired(normWireString), newClaimsSimpleMasonry: normWireRequired(normWireBoolean) });
