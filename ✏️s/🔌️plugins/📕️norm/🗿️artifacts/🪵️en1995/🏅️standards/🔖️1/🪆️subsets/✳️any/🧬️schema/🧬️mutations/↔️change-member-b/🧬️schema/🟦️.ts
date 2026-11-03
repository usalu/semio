/** ↔️ `change-member-b` wire twin: the leaf payload `ChangeMemberB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberB {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberB: NormWireReader<ChangeMemberB> = normWireObject<ChangeMemberB>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
