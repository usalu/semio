/** 🧱️ `change-fastened-joint-sheet-width` wire twin: the leaf payload `ChangeFastenedJointSheetWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointSheetWidth {
  id: string;
  newSheetWidth: number;
}

export const parseChangeFastenedJointSheetWidth: NormWireReader<ChangeFastenedJointSheetWidth> = normWireObject<ChangeFastenedJointSheetWidth>({ id: normWireRequired(normWireString), newSheetWidth: normWireRequired(normWireNumber) });
