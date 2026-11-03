/** ✖️ `change-bracing-length` wire twin: the leaf payload `ChangeBracingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingLength {
  bracingId: string;
  newValue: number;
}

export const parseChangeBracingLength: NormWireReader<ChangeBracingLength> = normWireObject<ChangeBracingLength>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
