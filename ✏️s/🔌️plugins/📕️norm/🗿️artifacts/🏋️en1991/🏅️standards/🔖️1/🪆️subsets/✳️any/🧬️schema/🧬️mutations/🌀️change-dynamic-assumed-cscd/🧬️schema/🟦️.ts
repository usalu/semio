/** 🌀 `change-dynamic-assumed-cscd` wire twin: the leaf payload `ChangeDynamicAssumedCscd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDynamicAssumedCscd {
  id: string;
  newAssumedCscd: number;
}

export const parseChangeDynamicAssumedCscd: NormWireReader<ChangeDynamicAssumedCscd> = normWireObject<ChangeDynamicAssumedCscd>({ id: normWireRequired(normWireString), newAssumedCscd: normWireRequired(normWireNumber) });
