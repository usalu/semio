/** 📐️ `change-section-r` wire twin: the leaf payload `ChangeSectionR`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionR {
  id: string;
  newR: number;
}

export const parseChangeSectionR: NormWireReader<ChangeSectionR> = normWireObject<ChangeSectionR>({ id: normWireRequired(normWireString), newR: normWireRequired(normWireNumber) });
