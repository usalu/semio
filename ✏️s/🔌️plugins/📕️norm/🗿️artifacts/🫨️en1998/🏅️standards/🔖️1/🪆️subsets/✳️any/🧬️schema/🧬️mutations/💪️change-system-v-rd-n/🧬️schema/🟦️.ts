/** 💪️ `change-system-v-rd-n` wire twin: the leaf payload `ChangeSystemVRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSystemVRdN {
  mutation: "changeSystemVRdN";
  buildingIndex: number;
  systemIndex: number;
  newBaseShearResistanceN: number;
}

export const parseChangeSystemVRdN: NormWireReader<ChangeSystemVRdN> = normWireObject<ChangeSystemVRdN>({ mutation: normWireRequired(normWireLiteral("changeSystemVRdN")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), systemIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBaseShearResistanceN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
