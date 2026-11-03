/** 🔗 `change-effect-action-id` wire twin: the leaf payload `ChangeEffectActionId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEffectActionId {
  mutation: "changeEffectActionId";
  index: number;
  newActionId: string;
}

export const parseChangeEffectActionId: NormWireReader<ChangeEffectActionId> = normWireObject<ChangeEffectActionId>({ mutation: normWireRequired(normWireLiteral("changeEffectActionId")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newActionId: normWireRequired(normWireString) });
