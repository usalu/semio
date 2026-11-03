/** 📐️ `change-section-iz` wire twin: the leaf payload `ChangeSectionIz`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionIz {
  id: string;
  newIz: number;
}

export const parseChangeSectionIz: NormWireReader<ChangeSectionIz> = normWireObject<ChangeSectionIz>({ id: normWireRequired(normWireString), newIz: normWireRequired(normWireNumber) });
