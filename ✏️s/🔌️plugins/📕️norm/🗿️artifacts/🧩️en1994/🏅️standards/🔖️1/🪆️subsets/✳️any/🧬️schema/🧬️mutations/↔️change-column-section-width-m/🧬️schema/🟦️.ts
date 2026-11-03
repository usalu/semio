/** ↔️ `change-column-section-width-m` wire twin: the leaf payload `ChangeColumnSectionWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSectionWidthM {
  id: string;
  newSectionWidthM: number;
}

export const parseChangeColumnSectionWidthM: NormWireReader<ChangeColumnSectionWidthM> = normWireObject<ChangeColumnSectionWidthM>({ id: normWireRequired(normWireString), newSectionWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
