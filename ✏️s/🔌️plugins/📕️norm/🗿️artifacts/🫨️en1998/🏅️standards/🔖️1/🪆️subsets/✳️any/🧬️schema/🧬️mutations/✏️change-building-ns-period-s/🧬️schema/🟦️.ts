/** ✏️ `change-building-ns-period-s` wire twin: the leaf payload `ChangeBuildingNsPeriodS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingNsPeriodS {
  mutation: "changeBuildingNsPeriodS";
  id: string;
  newNonstructuralPeriodS: number;
}

export const parseChangeBuildingNsPeriodS: NormWireReader<ChangeBuildingNsPeriodS> = normWireObject<ChangeBuildingNsPeriodS>({ mutation: normWireRequired(normWireLiteral("changeBuildingNsPeriodS")), id: normWireRequired(normWireString), newNonstructuralPeriodS: normWireRequired(normWireNumber) });
