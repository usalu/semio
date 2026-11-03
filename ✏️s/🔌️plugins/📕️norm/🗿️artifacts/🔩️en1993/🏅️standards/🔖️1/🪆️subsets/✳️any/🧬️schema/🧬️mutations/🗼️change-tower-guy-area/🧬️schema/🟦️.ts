/** 🗼️ `change-tower-guy-area` wire twin: the leaf payload `ChangeTowerGuyArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerGuyArea {
  id: string;
  newGuyArea: number;
}

export const parseChangeTowerGuyArea: NormWireReader<ChangeTowerGuyArea> = normWireObject<ChangeTowerGuyArea>({ id: normWireRequired(normWireString), newGuyArea: normWireRequired(normWireNumber) });
