/** ⚙ `change-snow-guard-slope-length` wire twin: the leaf payload `ChangeSnowGuardSlopeLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSnowGuardSlopeLength {
  id: string;
  newSlopeLength: number;
}

export const parseChangeSnowGuardSlopeLength: NormWireReader<ChangeSnowGuardSlopeLength> = normWireObject<ChangeSnowGuardSlopeLength>({ id: normWireRequired(normWireString), newSlopeLength: normWireRequired(normWireNumber) });
