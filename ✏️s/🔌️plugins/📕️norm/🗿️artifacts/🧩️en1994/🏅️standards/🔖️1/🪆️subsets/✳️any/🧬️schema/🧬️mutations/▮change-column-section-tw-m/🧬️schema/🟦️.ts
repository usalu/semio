/** ▮ `change-column-section-tw-m` wire twin: the leaf payload `ChangeColumnSectionTwM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSectionTwM {
  id: string;
  newSectionTwM: number;
}

export const parseChangeColumnSectionTwM: NormWireReader<ChangeColumnSectionTwM> = normWireObject<ChangeColumnSectionTwM>({ id: normWireRequired(normWireString), newSectionTwM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
