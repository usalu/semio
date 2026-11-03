/** 🫧 `change-member-liquid-rho-p-eff` wire twin: the leaf payload `ChangeMemberLiquidRhoPEff`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLiquidRhoPEff {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberLiquidRhoPEff: NormWireReader<ChangeMemberLiquidRhoPEff> = normWireObject<ChangeMemberLiquidRhoPEff>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
