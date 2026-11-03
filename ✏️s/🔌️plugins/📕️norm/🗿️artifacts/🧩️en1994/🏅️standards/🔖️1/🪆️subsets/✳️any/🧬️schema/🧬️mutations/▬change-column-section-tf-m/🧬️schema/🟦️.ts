/** ▬ `change-column-section-tf-m` wire twin: the leaf payload `ChangeColumnSectionTfM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSectionTfM {
  id: string;
  newSectionTfM: number;
}

export const parseChangeColumnSectionTfM: NormWireReader<ChangeColumnSectionTfM> = normWireObject<ChangeColumnSectionTfM>({ id: normWireRequired(normWireString), newSectionTfM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
