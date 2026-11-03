/** ⬇️ `change-column-introduced-force-n` wire twin: the leaf payload `ChangeColumnIntroducedForceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnIntroducedForceN {
  id: string;
  newIntroducedForceN: number;
}

export const parseChangeColumnIntroducedForceN: NormWireReader<ChangeColumnIntroducedForceN> = normWireObject<ChangeColumnIntroducedForceN>({ id: normWireRequired(normWireString), newIntroducedForceN: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
