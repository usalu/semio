/** 🧱️ `change-plated-equivalent-span` wire twin: the leaf payload `ChangePlatedEquivalentSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedEquivalentSpan {
  id: string;
  newEquivalentSpan: number;
}

export const parseChangePlatedEquivalentSpan: NormWireReader<ChangePlatedEquivalentSpan> = normWireObject<ChangePlatedEquivalentSpan>({ id: normWireRequired(normWireString), newEquivalentSpan: normWireRequired(normWireNumber) });
