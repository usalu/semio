/** 🌉️ `change-member-bridge-beta` wire twin: the leaf payload `ChangeMemberBridgeBeta`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeMemberBridgeBeta {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberBridgeBeta: NormWireReader<ChangeMemberBridgeBeta> = normWireObject<ChangeMemberBridgeBeta>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
