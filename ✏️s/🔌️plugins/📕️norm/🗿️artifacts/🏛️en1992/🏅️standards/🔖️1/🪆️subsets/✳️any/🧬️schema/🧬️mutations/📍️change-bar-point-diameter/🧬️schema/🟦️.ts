/** 📍 `change-bar-point-diameter` wire twin: the leaf payload `ChangeBarPointDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarPointDiameter {
  memberId: string;
  barId: string;
  newDiameter: number;
}

export const parseChangeBarPointDiameter: NormWireReader<ChangeBarPointDiameter> = normWireObject<ChangeBarPointDiameter>({ memberId: normWireRequired(normWireString), barId: normWireRequired(normWireString), newDiameter: normWireRequired(normWireNumber) });
