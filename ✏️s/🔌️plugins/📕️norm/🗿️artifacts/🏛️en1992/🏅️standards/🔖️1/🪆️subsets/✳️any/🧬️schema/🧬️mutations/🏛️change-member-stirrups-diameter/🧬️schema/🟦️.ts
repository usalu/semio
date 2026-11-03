/** 🏛️ `change-member-stirrups-diameter` wire twin: the leaf payload `ChangeMemberStirrupsDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStirrupsDiameter {
  memberId: string;
  newStirrupsDiameter: number;
}

export const parseChangeMemberStirrupsDiameter: NormWireReader<ChangeMemberStirrupsDiameter> = normWireObject<ChangeMemberStirrupsDiameter>({ memberId: normWireRequired(normWireString), newStirrupsDiameter: normWireRequired(normWireNumber) });
