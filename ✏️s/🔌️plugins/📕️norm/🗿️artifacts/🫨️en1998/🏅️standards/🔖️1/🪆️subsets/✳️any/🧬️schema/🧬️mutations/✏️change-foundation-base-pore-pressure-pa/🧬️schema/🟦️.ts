/** ✏️ `change-foundation-base-pore-pressure-pa` wire twin: the leaf payload `ChangeFoundationBasePorePressurePa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationBasePorePressurePa {
  mutation: "changeFoundationBasePorePressurePa";
  index: number;
  newBasePorePressurePa: number;
}

export const parseChangeFoundationBasePorePressurePa: NormWireReader<ChangeFoundationBasePorePressurePa> = normWireObject<ChangeFoundationBasePorePressurePa>({ mutation: normWireRequired(normWireLiteral("changeFoundationBasePorePressurePa")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newBasePorePressurePa: normWireRequired(normWireNumber) });
