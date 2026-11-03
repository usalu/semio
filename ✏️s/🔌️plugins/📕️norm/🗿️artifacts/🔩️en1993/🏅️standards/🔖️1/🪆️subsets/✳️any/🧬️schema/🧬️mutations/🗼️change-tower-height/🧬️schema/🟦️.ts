/** 🗼️ `change-tower-height` wire twin: the leaf payload `ChangeTowerHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerHeight {
  id: string;
  newHeight: number;
}

export const parseChangeTowerHeight: NormWireReader<ChangeTowerHeight> = normWireObject<ChangeTowerHeight>({ id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });
