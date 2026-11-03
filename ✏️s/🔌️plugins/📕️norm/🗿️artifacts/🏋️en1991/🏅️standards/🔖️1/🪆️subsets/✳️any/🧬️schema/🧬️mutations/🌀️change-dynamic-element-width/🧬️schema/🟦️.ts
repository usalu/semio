/** 🌀 `change-dynamic-element-width` wire twin: the leaf payload `ChangeDynamicElementWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDynamicElementWidth {
  id: string;
  newWidth: number;
}

export const parseChangeDynamicElementWidth: NormWireReader<ChangeDynamicElementWidth> = normWireObject<ChangeDynamicElementWidth>({ id: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
