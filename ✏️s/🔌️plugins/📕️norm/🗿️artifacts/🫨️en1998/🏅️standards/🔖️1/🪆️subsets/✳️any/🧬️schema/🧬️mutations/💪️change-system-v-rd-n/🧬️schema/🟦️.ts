/** 💪️ `change-system-v-rd-n` wire twin: the leaf payload `ChangeSystemVRdN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSystemVRdN {
  mutation: "changeSystemVRdN";
  buildingId: string;
  id: string;
  newBaseShearResistanceN: number;
}

export const parseChangeSystemVRdN: NormWireReader<ChangeSystemVRdN> = normWireObject<ChangeSystemVRdN>({ mutation: normWireRequired(normWireLiteral("changeSystemVRdN")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newBaseShearResistanceN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
