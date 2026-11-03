/** 📐️ `change-section-w-pl-y` wire twin: the leaf payload `ChangeSectionWPlY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionWPlY {
  id: string;
  newWPlY: number;
}

export const parseChangeSectionWPlY: NormWireReader<ChangeSectionWPlY> = normWireObject<ChangeSectionWPlY>({ id: normWireRequired(normWireString), newWPlY: normWireRequired(normWireNumber) });
