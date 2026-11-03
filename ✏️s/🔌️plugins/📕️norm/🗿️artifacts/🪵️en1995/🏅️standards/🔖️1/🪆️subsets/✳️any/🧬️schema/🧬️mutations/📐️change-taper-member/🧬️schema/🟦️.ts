/** 📐️ `change-taper-member` wire twin: the leaf payload `ChangeTaperMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperMember {
  taperId: string;
  newValue: string;
}

export const parseChangeTaperMember: NormWireReader<ChangeTaperMember> = normWireObject<ChangeTaperMember>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
