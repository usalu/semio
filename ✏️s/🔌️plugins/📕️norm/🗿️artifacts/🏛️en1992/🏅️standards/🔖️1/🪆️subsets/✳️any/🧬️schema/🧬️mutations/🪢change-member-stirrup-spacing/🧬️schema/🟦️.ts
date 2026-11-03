/** 🪢 `change-member-stirrup-spacing` wire twin: the leaf payload `ChangeMemberStirrupSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberStirrupSpacing {
  memberId: string;
  newSpacing: number;
}

export const parseChangeMemberStirrupSpacing: NormWireReader<ChangeMemberStirrupSpacing> = normWireObject<ChangeMemberStirrupSpacing>({ memberId: normWireRequired(normWireString), newSpacing: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
