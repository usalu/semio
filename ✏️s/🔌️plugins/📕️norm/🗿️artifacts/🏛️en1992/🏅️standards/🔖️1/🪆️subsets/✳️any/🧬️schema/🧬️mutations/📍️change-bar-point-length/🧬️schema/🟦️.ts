/** 📍 `change-bar-point-length` wire twin: the leaf payload `ChangeBarPointLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarPointLength {
  memberId: string;
  barId: string;
  newLength: number;
}

export const parseChangeBarPointLength: NormWireReader<ChangeBarPointLength> = normWireObject<ChangeBarPointLength>({ memberId: normWireRequired(normWireString), barId: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
