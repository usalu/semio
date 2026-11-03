/** 📍 `change-bar-point-z` wire twin: the leaf payload `ChangeBarPointZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarPointZ {
  memberId: string;
  barId: string;
  newZ: number;
}

export const parseChangeBarPointZ: NormWireReader<ChangeBarPointZ> = normWireObject<ChangeBarPointZ>({ memberId: normWireRequired(normWireString), barId: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
