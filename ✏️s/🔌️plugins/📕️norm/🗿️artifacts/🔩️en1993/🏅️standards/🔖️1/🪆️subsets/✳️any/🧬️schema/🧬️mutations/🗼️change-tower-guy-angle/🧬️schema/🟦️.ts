/** 🗼️ `change-tower-guy-angle` wire twin: the leaf payload `ChangeTowerGuyAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerGuyAngle {
  id: string;
  newGuyAngle: number;
}

export const parseChangeTowerGuyAngle: NormWireReader<ChangeTowerGuyAngle> = normWireObject<ChangeTowerGuyAngle>({ id: normWireRequired(normWireString), newGuyAngle: normWireRequired(normWireNumber) });
