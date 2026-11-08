/** 🦴️ `change-fire-insulation-specific-heat` wire twin: the leaf payload `ChangeFireInsulationSpecificHeat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireInsulationSpecificHeat {
  mutation: "changeFireInsulationSpecificHeat";
  fireInsulationId: string;
  newSpecificHeat: number;
}

export const parseChangeFireInsulationSpecificHeat: NormWireReader<ChangeFireInsulationSpecificHeat> = normWireObject<ChangeFireInsulationSpecificHeat>({ mutation: normWireRequired(normWireLiteral("changeFireInsulationSpecificHeat")), fireInsulationId: normWireRequired(normWireString), newSpecificHeat: normWireRequired(normWireNumber) });
