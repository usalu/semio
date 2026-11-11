/** 🧊 `change-zone-thermal-mass` wire twin: the leaf payload `ChangeZoneThermalMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { parseThermalMassClass, type ThermalMassClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeZoneThermalMass {
  mutation: "changezonethermalmass";
  zoneId: string;
  newThermalMassClass: ThermalMassClass;
  newCWirkWhM2k: number;
}

export const parseChangeZoneThermalMass: NormWireReader<ChangeZoneThermalMass> = normWireObject<ChangeZoneThermalMass>({ mutation: normWireRequired(normWireLiteral("changezonethermalmass")), zoneId: normWireRequired(normWireString), newThermalMassClass: normWireRequired(parseThermalMassClass), newCWirkWhM2k: normWireRequired(normWireRange(normWireNumber, {"maximum":1000,"exclusiveMinimum":0})) });
