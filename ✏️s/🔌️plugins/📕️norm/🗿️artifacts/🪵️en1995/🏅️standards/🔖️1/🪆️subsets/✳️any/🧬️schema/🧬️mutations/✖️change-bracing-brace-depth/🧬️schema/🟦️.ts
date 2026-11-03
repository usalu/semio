/** ✖️ `change-bracing-brace-depth` wire twin: the leaf payload `ChangeBracingBraceDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingBraceDepth {
  bracingId: string;
  newValue: number;
}

export const parseChangeBracingBraceDepth: NormWireReader<ChangeBracingBraceDepth> = normWireObject<ChangeBracingBraceDepth>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
