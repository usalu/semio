/** 🪵️ `change-pile-section-id` wire twin: the leaf payload `ChangePileSectionId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileSectionId {
  id: string;
  newSectionId: string;
}

export const parseChangePileSectionId: NormWireReader<ChangePileSectionId> = normWireObject<ChangePileSectionId>({ id: normWireRequired(normWireString), newSectionId: normWireRequired(normWireString) });
