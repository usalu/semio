/** 🧱 `change-roof-drift-obstruction-height` wire twin: the leaf payload `ChangeRoofDriftObstructionHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRoofDriftObstructionHeight {
  id: string;
  newDriftObstructionHeight: number;
}

export const parseChangeRoofDriftObstructionHeight: NormWireReader<ChangeRoofDriftObstructionHeight> = normWireObject<ChangeRoofDriftObstructionHeight>({ id: normWireRequired(normWireString), newDriftObstructionHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
