/** ✏️ `change-soil-unit-wt-nm3` wire twin: the leaf payload `ChangeSoilUnitWtNM3`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilUnitWtNM3 {
  mutation: "changeSoilUnitWtNM3";
  id: string;
  newUnitWeightNM3: number;
}

export const parseChangeSoilUnitWtNM3: NormWireReader<ChangeSoilUnitWtNM3> = normWireObject<ChangeSoilUnitWtNM3>({ mutation: normWireRequired(normWireLiteral("changeSoilUnitWtNM3")), id: normWireRequired(normWireString), newUnitWeightNM3: normWireRequired(normWireNumber) });
