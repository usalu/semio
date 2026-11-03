/** 💧 `change-water-piezometric-depth` wire twin: the leaf payload `ChangeWaterPiezometricDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterPiezometricDepth {
  mutation: "changeWaterPiezometricDepth";
  id: string;
  newPiezometricDepth: number;
}

export const parseChangeWaterPiezometricDepth: NormWireReader<ChangeWaterPiezometricDepth> = normWireObject<ChangeWaterPiezometricDepth>({ mutation: normWireRequired(normWireLiteral("changeWaterPiezometricDepth")), id: normWireRequired(normWireString), newPiezometricDepth: normWireRequired(normWireNumber) });
