/** 🌾 `change-bulk-solid-internal-friction` wire twin: the leaf payload `ChangeBulkSolidInternalFriction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBulkSolidInternalFriction {
  id: string;
  newInternalFriction: number;
}

export const parseChangeBulkSolidInternalFriction: NormWireReader<ChangeBulkSolidInternalFriction> = normWireObject<ChangeBulkSolidInternalFriction>({ id: normWireRequired(normWireString), newInternalFriction: normWireRequired(normWireNumber) });
