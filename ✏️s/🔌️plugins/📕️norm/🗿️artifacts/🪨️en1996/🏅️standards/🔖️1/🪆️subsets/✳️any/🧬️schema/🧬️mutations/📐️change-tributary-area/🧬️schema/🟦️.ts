/** 📐️ `change-tributary-area` wire twin: the leaf payload `ChangeTributaryArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTributaryArea {
  wallId: string;
  id: string;
  newTributaryAreaM2: number;
}

export const parseChangeTributaryArea: NormWireReader<ChangeTributaryArea> = normWireObject<ChangeTributaryArea>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newTributaryAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });
