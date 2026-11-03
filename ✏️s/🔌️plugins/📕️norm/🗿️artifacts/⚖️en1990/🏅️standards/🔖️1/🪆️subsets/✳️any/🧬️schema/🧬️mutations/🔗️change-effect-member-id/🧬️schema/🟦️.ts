/** 🔗 `change-effect-member-id` wire twin: the leaf payload `ChangeEffectMemberId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEffectMemberId {
  mutation: "changeEffectMemberId";
  index: number;
  newMemberId: string;
}

export const parseChangeEffectMemberId: NormWireReader<ChangeEffectMemberId> = normWireObject<ChangeEffectMemberId>({ mutation: normWireRequired(normWireLiteral("changeEffectMemberId")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newMemberId: normWireRequired(normWireString) });
