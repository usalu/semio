/** 💧 `change-member-relative-humidity` wire twin: the leaf payload `ChangeMemberRelativeHumidity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberRelativeHumidity {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberRelativeHumidity: NormWireReader<ChangeMemberRelativeHumidity> = normWireObject<ChangeMemberRelativeHumidity>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
