/** 🏗️ `change-member-rd-fat` wire twin: the leaf payload `ChangeMemberRdFat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRdFat {
  mutation: "changeMemberRdFat";
  id: string;
  newRdFat: number;
}

export const parseChangeMemberRdFat: NormWireReader<ChangeMemberRdFat> = normWireObject<ChangeMemberRdFat>({ mutation: normWireRequired(normWireLiteral("changeMemberRdFat")), id: normWireRequired(normWireString), newRdFat: normWireRequired(normWireNumber) });
