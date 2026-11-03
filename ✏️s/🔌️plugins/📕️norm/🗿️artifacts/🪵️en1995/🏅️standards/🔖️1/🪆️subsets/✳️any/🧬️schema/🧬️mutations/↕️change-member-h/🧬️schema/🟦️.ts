/** ↕️ `change-member-h` wire twin: the leaf payload `ChangeMemberH`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberH {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberH: NormWireReader<ChangeMemberH> = normWireObject<ChangeMemberH>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
