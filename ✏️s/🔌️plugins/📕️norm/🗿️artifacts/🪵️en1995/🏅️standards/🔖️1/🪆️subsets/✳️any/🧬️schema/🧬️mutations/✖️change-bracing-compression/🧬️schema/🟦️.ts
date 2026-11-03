/** ✖️ `change-bracing-compression` wire twin: the leaf payload `ChangeBracingCompression`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingCompression {
  bracingId: string;
  newValue: number;
}

export const parseChangeBracingCompression: NormWireReader<ChangeBracingCompression> = normWireObject<ChangeBracingCompression>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
