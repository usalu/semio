/** 🌾 `change-silo-bulk-density` wire twin: the leaf payload `ChangeSiloBulkDensity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloBulkDensity {
  newSiloBulkDensity: number;
}

export const parseChangeSiloBulkDensity: NormWireReader<ChangeSiloBulkDensity> = normWireObject<ChangeSiloBulkDensity>({ newSiloBulkDensity: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
