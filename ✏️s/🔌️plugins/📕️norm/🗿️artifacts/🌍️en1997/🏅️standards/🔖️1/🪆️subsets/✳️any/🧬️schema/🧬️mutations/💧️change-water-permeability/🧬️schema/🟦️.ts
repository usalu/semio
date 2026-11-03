/** 💧 `change-water-permeability` wire twin: the leaf payload `ChangeWaterPermeability`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterPermeability {
  mutation: "changeWaterPermeability";
  id: string;
  newPermeability: number;
}

export const parseChangeWaterPermeability: NormWireReader<ChangeWaterPermeability> = normWireObject<ChangeWaterPermeability>({ mutation: normWireRequired(normWireLiteral("changeWaterPermeability")), id: normWireRequired(normWireString), newPermeability: normWireRequired(normWireNumber) });
