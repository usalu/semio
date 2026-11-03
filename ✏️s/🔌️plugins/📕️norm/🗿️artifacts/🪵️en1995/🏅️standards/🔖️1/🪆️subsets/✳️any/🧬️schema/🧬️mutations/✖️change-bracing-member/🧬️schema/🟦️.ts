/** ✖️ `change-bracing-member` wire twin: the leaf payload `ChangeBracingMember`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingMember {
  bracingId: string;
  newValue: string;
}

export const parseChangeBracingMember: NormWireReader<ChangeBracingMember> = normWireObject<ChangeBracingMember>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireString) });
