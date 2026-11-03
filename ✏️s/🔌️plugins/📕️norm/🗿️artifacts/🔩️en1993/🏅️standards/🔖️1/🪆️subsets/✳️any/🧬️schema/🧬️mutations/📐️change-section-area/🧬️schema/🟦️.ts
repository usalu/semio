/** 📐️ `change-section-area` wire twin: the leaf payload `ChangeSectionArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionArea {
  id: string;
  newArea: number;
}

export const parseChangeSectionArea: NormWireReader<ChangeSectionArea> = normWireObject<ChangeSectionArea>({ id: normWireRequired(normWireString), newArea: normWireRequired(normWireNumber) });
