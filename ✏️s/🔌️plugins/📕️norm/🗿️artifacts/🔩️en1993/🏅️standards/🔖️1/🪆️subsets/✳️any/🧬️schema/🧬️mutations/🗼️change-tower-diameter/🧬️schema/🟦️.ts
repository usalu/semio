/** 🗼️ `change-tower-diameter` wire twin: the leaf payload `ChangeTowerDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerDiameter {
  id: string;
  newDiameter: number;
}

export const parseChangeTowerDiameter: NormWireReader<ChangeTowerDiameter> = normWireObject<ChangeTowerDiameter>({ id: normWireRequired(normWireString), newDiameter: normWireRequired(normWireNumber) });
