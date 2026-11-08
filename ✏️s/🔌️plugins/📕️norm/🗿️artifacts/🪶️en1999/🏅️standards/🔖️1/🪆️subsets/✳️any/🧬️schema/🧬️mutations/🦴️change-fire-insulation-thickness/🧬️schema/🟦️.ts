/** 🦴️ `change-fire-insulation-thickness` wire twin: the leaf payload `ChangeFireInsulationThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireInsulationThickness {
  mutation: "changeFireInsulationThickness";
  fireInsulationId: string;
  newThickness: number;
}

export const parseChangeFireInsulationThickness: NormWireReader<ChangeFireInsulationThickness> = normWireObject<ChangeFireInsulationThickness>({ mutation: normWireRequired(normWireLiteral("changeFireInsulationThickness")), fireInsulationId: normWireRequired(normWireString), newThickness: normWireRequired(normWireNumber) });
