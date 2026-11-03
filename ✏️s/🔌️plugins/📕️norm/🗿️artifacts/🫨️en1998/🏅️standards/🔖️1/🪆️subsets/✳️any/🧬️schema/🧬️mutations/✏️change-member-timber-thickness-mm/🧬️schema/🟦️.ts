/** ✏️ `change-member-timber-thickness-mm` wire twin: the leaf payload `ChangeMemberTimberThicknessMm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberTimberThicknessMm {
  mutation: "changeMemberTimberThicknessMm";
  buildingId: string;
  id: string;
  newTimberThicknessMm: number;
}

export const parseChangeMemberTimberThicknessMm: NormWireReader<ChangeMemberTimberThicknessMm> = normWireObject<ChangeMemberTimberThicknessMm>({ mutation: normWireRequired(normWireLiteral("changeMemberTimberThicknessMm")), buildingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newTimberThicknessMm: normWireRequired(normWireNumber) });
