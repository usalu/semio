/** 🕳️ `change-cutout-member` wire twin: the leaf payload `ChangeCutoutMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutMember {
  cutoutId: string;
  newValue: string;
}

export const parseChangeCutoutMember: NormWireReader<ChangeCutoutMember> = normWireObject<ChangeCutoutMember>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
