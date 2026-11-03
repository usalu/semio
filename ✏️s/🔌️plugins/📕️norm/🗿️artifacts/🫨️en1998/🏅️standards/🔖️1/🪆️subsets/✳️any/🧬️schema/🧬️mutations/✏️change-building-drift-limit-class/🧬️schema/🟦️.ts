/** ✏️ `change-building-drift-limit-class` wire twin: the leaf payload `ChangeBuildingDriftLimitClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DriftLimitClass, parseDriftLimitClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBuildingDriftLimitClass {
  mutation: "changeBuildingDriftLimitClass";
  id: string;
  newDriftLimitClass: DriftLimitClass;
}

export const parseChangeBuildingDriftLimitClass: NormWireReader<ChangeBuildingDriftLimitClass> = normWireObject<ChangeBuildingDriftLimitClass>({ mutation: normWireRequired(normWireLiteral("changeBuildingDriftLimitClass")), id: normWireRequired(normWireString), newDriftLimitClass: normWireRequired(parseDriftLimitClass) });
