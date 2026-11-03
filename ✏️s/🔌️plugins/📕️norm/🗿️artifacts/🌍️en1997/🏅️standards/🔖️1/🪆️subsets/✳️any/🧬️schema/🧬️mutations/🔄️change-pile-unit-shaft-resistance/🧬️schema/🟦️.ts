/** 🔄️ `change-pile-unit-shaft-resistance` wire twin: the leaf payload `ChangePileUnitShaftResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileUnitShaftResistance {
  mutation: "changePileUnitShaftResistance";
  id: string;
  newUnitShaftResistance: number;
}

export const parseChangePileUnitShaftResistance: NormWireReader<ChangePileUnitShaftResistance> = normWireObject<ChangePileUnitShaftResistance>({ mutation: normWireRequired(normWireLiteral("changePileUnitShaftResistance")), id: normWireRequired(normWireString), newUnitShaftResistance: normWireRequired(normWireNumber) });
