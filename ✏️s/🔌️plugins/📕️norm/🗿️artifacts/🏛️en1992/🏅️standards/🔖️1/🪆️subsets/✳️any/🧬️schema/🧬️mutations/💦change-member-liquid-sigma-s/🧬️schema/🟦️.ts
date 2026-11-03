/** 💦 `change-member-liquid-sigma-s` wire twin: the leaf payload `ChangeMemberLiquidSigmaS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberLiquidSigmaS {
  memberId: string;
  newValue: number;
}

export const parseChangeMemberLiquidSigmaS: NormWireReader<ChangeMemberLiquidSigmaS> = normWireObject<ChangeMemberLiquidSigmaS>({ memberId: normWireRequired(normWireString), newValue: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
