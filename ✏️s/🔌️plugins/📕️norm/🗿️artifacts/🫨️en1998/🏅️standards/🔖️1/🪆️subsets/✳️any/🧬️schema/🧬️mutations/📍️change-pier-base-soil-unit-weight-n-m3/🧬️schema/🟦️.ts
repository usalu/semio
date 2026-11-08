/** 📍️ `change-pier-base-soil-unit-weight-n-m3` wire twin: the leaf payload `ChangePierBaseSoilUnitWeightNM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBaseSoilUnitWeightNM3 {
  mutation: "changePierBaseSoilUnitWeightNM3";
  id: string;
  newSoilUnitWeightNM3: number;
}

export const parseChangePierBaseSoilUnitWeightNM3: NormWireReader<ChangePierBaseSoilUnitWeightNM3> = normWireObject<ChangePierBaseSoilUnitWeightNM3>({ mutation: normWireRequired(normWireLiteral("changePierBaseSoilUnitWeightNM3")), id: normWireRequired(normWireString), newSoilUnitWeightNM3: normWireRequired(normWireNumber) });
