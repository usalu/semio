/** 📏️ `change-member-toughness-stress-ratio` wire twin: the leaf payload `ChangeMemberToughnessStressRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberToughnessStressRatio {
  id: string;
  newToughnessStressRatio: number;
}

export const parseChangeMemberToughnessStressRatio: NormWireReader<ChangeMemberToughnessStressRatio> = normWireObject<ChangeMemberToughnessStressRatio>({ id: normWireRequired(normWireString), newToughnessStressRatio: normWireRequired(normWireNumber) });
