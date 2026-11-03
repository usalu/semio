/** ✏️ `change-foundation-base-friction-deg` wire twin: the leaf payload `ChangeFoundationBaseFrictionDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBaseFrictionDeg {
  mutation: "changeFoundationBaseFrictionDeg";
  index: number;
  newBaseFrictionAngleDeg: number;
}

export const parseChangeFoundationBaseFrictionDeg: NormWireReader<ChangeFoundationBaseFrictionDeg> = normWireObject<ChangeFoundationBaseFrictionDeg>({ mutation: normWireRequired(normWireLiteral("changeFoundationBaseFrictionDeg")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBaseFrictionAngleDeg: normWireRequired(normWireNumber) });
