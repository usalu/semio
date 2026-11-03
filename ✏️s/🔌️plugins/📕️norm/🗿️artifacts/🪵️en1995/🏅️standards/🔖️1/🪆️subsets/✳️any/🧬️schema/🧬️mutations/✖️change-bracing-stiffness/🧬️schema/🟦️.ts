/** ✖️ `change-bracing-stiffness` wire twin: the leaf payload `ChangeBracingStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingStiffness {
  bracingId: string;
  newValue: number;
}

export const parseChangeBracingStiffness: NormWireReader<ChangeBracingStiffness> = normWireObject<ChangeBracingStiffness>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
