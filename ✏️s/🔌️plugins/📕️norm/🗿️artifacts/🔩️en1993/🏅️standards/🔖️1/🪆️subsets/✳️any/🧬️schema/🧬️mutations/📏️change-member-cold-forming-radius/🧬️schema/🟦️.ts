/** 📏️ `change-member-cold-forming-radius` wire twin: the leaf payload `ChangeMemberColdFormingRadius`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberColdFormingRadius {
  id: string;
  newColdFormingRadius: number;
}

export const parseChangeMemberColdFormingRadius: NormWireReader<ChangeMemberColdFormingRadius> = normWireObject<ChangeMemberColdFormingRadius>({ id: normWireRequired(normWireString), newColdFormingRadius: normWireRequired(normWireNumber) });
