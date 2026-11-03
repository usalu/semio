/** ✏️ `change-building-accidental-ecc-ratio` wire twin: the leaf payload `ChangeBuildingAccidentalEccRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingAccidentalEccRatio {
  mutation: "changeBuildingAccidentalEccRatio";
  id: string;
  newAccidentalEccentricityRatio: number;
}

export const parseChangeBuildingAccidentalEccRatio: NormWireReader<ChangeBuildingAccidentalEccRatio> = normWireObject<ChangeBuildingAccidentalEccRatio>({ mutation: normWireRequired(normWireLiteral("changeBuildingAccidentalEccRatio")), id: normWireRequired(normWireString), newAccidentalEccentricityRatio: normWireRequired(normWireNumber) });
