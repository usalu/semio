/** 📐️ `change-section-area-net` wire twin: the leaf payload `ChangeSectionAreaNet`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionAreaNet {
  id: string;
  newAreaNet: number;
}

export const parseChangeSectionAreaNet: NormWireReader<ChangeSectionAreaNet> = normWireObject<ChangeSectionAreaNet>({ id: normWireRequired(normWireString), newAreaNet: normWireRequired(normWireNumber) });
