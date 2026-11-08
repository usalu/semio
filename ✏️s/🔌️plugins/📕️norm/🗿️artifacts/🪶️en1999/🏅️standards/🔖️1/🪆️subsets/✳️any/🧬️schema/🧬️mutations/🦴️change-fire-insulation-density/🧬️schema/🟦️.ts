/** 🦴️ `change-fire-insulation-density` wire twin: the leaf payload `ChangeFireInsulationDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireInsulationDensity {
  mutation: "changeFireInsulationDensity";
  fireInsulationId: string;
  newDensity: number;
}

export const parseChangeFireInsulationDensity: NormWireReader<ChangeFireInsulationDensity> = normWireObject<ChangeFireInsulationDensity>({ mutation: normWireRequired(normWireLiteral("changeFireInsulationDensity")), fireInsulationId: normWireRequired(normWireString), newDensity: normWireRequired(normWireNumber) });
