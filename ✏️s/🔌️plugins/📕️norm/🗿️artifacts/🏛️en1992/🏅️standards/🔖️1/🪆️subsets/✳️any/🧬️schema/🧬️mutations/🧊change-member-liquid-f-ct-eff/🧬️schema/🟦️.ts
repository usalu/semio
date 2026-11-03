/** 🧊 `change-member-liquid-f-ct-eff` wire twin: the leaf payload `ChangeMemberLiquidFCtEff`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLiquidFCtEff {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberLiquidFCtEff: NormWireReader<ChangeMemberLiquidFCtEff> = normWireObject<ChangeMemberLiquidFCtEff>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
