/** 📏️ `change-member-tough-stress-ratio` wire twin: the leaf payload `ChangeMemberToughStressRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberToughStressRatio {
  id: string;
  newToughnessStressRatio: number;
}

export const parseChangeMemberToughStressRatio: NormWireReader<ChangeMemberToughStressRatio> = normWireObject<ChangeMemberToughStressRatio>({ id: normWireRequired(normWireString), newToughnessStressRatio: normWireRequired(normWireNumber) });
