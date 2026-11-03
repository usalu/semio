/** 📐️ `change-section-designation` wire twin: the leaf payload `ChangeSectionDesignation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionDesignation {
  id: string;
  newDesignation: string;
}

export const parseChangeSectionDesignation: NormWireReader<ChangeSectionDesignation> = normWireObject<ChangeSectionDesignation>({ id: normWireRequired(normWireString), newDesignation: normWireRequired(normWireString) });
