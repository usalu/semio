/** 🌀 `change-dynamic-element-strouhal` wire twin: the leaf payload `ChangeDynamicElementStrouhal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDynamicElementStrouhal {
  id: string;
  newStrouhal: number;
}

export const parseChangeDynamicElementStrouhal: NormWireReader<ChangeDynamicElementStrouhal> = normWireObject<ChangeDynamicElementStrouhal>({ id: normWireRequired(normWireString), newStrouhal: normWireRequired(normWireNumber) });
