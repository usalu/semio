/** 🦴️ `change-fire-insulation-conductivity` wire twin: the leaf payload `ChangeFireInsulationConductivity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireInsulationConductivity {
  mutation: "changeFireInsulationConductivity";
  fireInsulationId: string;
  newConductivity: number;
}

export const parseChangeFireInsulationConductivity: NormWireReader<ChangeFireInsulationConductivity> = normWireObject<ChangeFireInsulationConductivity>({ mutation: normWireRequired(normWireLiteral("changeFireInsulationConductivity")), fireInsulationId: normWireRequired(normWireString), newConductivity: normWireRequired(normWireNumber) });
