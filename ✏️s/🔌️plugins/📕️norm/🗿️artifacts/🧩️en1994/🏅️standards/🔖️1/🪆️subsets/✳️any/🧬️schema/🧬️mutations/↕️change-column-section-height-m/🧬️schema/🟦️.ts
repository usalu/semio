/** ↕️ `change-column-section-height-m` wire twin: the leaf payload `ChangeColumnSectionHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSectionHeightM {
  id: string;
  newSectionHeightM: number;
}

export const parseChangeColumnSectionHeightM: NormWireReader<ChangeColumnSectionHeightM> = normWireObject<ChangeColumnSectionHeightM>({ id: normWireRequired(normWireString), newSectionHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
