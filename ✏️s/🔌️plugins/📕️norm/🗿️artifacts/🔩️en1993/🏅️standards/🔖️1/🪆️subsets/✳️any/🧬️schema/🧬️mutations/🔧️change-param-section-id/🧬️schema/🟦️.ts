/** 🔧️ `change-param-section-id` wire twin: the leaf payload `ChangeParamSectionId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeParamSectionId {
  id: string;
  newSectionId: string;
}

export const parseChangeParamSectionId: NormWireReader<ChangeParamSectionId> = normWireObject<ChangeParamSectionId>({ id: normWireRequired(normWireString), newSectionId: normWireRequired(normWireString) });
