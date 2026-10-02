/** 🪢 `change-member-stirrup-spacing` wire twin: the leaf payload `ChangeMemberStirrupSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberStirrupSpacing {
  memberId: string;
  newSpacing: number;
}

export const parseChangeMemberStirrupSpacing: NormWireReader<ChangeMemberStirrupSpacing> = normWireObject<ChangeMemberStirrupSpacing>({ memberId: normWireRequired(normWireString), newSpacing: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
