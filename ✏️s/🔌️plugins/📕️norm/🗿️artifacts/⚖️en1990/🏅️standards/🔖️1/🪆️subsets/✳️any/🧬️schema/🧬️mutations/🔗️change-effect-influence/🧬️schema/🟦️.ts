/** 🔗 `change-effect-influence` wire twin: the leaf payload `ChangeEffectInfluence`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeEffectInfluence {
  mutation: "changeEffectInfluence";
  index: number;
  newInfluence: number;
}

export const parseChangeEffectInfluence: NormWireReader<ChangeEffectInfluence> = normWireObject<ChangeEffectInfluence>({ mutation: normWireRequired(normWireLiteral("changeEffectInfluence")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newInfluence: normWireRequired(normWireNumber) });
