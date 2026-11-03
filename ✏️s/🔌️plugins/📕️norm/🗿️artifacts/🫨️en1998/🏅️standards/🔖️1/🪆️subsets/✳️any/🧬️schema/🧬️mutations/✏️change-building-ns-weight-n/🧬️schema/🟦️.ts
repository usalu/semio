/** ✏️ `change-building-ns-weight-n` wire twin: the leaf payload `ChangeBuildingNsWeightN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingNsWeightN {
  mutation: "changeBuildingNsWeightN";
  id: string;
  newNonstructuralWeightN: number;
}

export const parseChangeBuildingNsWeightN: NormWireReader<ChangeBuildingNsWeightN> = normWireObject<ChangeBuildingNsWeightN>({ mutation: normWireRequired(normWireLiteral("changeBuildingNsWeightN")), id: normWireRequired(normWireString), newNonstructuralWeightN: normWireRequired(normWireNumber) });
