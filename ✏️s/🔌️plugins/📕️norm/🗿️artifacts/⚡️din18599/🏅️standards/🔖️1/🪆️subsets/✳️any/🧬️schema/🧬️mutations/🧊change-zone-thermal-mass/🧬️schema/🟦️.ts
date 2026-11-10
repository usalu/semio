/** 🧊 `change-zone-thermal-mass` wire twin: the leaf payload `ChangeZoneThermalMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din18599ThermalMassClass, parseDin18599ThermalMassClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeZoneThermalMass {
  mutation: "changezonethermalmass";
  zoneId: string;
  newThermalMassClass: Din18599ThermalMassClass;
  newCWirkWhM2k: number;
}

export const parseChangeZoneThermalMass: NormWireReader<ChangeZoneThermalMass> = normWireObject<ChangeZoneThermalMass>({ mutation: normWireRequired(normWireLiteral("changezonethermalmass")), zoneId: normWireRequired(normWireString), newThermalMassClass: normWireRequired(parseDin18599ThermalMassClass), newCWirkWhM2k: normWireRequired(normWireRange(normWireNumber, {"maximum":1000.0,"exclusiveMinimum":0.0})) });
