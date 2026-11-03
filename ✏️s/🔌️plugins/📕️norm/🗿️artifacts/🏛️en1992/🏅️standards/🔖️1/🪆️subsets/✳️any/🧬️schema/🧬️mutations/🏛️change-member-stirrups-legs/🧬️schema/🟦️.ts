/** 🏛️ `change-member-stirrups-legs` wire twin: the leaf payload `ChangeMemberStirrupsLegs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStirrupsLegs {
  memberId: string;
  newStirrupsLegs: number;
}

export const parseChangeMemberStirrupsLegs: NormWireReader<ChangeMemberStirrupsLegs> = normWireObject<ChangeMemberStirrupsLegs>({ memberId: normWireRequired(normWireString), newStirrupsLegs: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
