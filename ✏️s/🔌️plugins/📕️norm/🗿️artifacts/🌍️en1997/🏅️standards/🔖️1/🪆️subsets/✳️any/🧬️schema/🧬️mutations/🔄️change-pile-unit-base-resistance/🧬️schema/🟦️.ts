/** 🔄️ `change-pile-unit-base-resistance` wire twin: the leaf payload `ChangePileUnitBaseResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileUnitBaseResistance {
  mutation: "changePileUnitBaseResistance";
  id: string;
  newUnitBaseResistance: number;
}

export const parseChangePileUnitBaseResistance: NormWireReader<ChangePileUnitBaseResistance> = normWireObject<ChangePileUnitBaseResistance>({ mutation: normWireRequired(normWireLiteral("changePileUnitBaseResistance")), id: normWireRequired(normWireString), newUnitBaseResistance: normWireRequired(normWireNumber) });
