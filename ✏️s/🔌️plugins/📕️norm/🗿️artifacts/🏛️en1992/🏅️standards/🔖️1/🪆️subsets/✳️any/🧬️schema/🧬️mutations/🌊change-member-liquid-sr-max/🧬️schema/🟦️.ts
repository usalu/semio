/** 🌊 `change-member-liquid-sr-max` wire twin: the leaf payload `ChangeMemberLiquidSRMax`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLiquidSRMax {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberLiquidSRMax: NormWireReader<ChangeMemberLiquidSRMax> = normWireObject<ChangeMemberLiquidSRMax>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
