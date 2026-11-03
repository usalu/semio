/** 🌀 `change-dynamic-first-frequency` wire twin: the leaf payload `ChangeDynamicFirstFrequency`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDynamicFirstFrequency {
  id: string;
  newFirstFrequency: number;
}

export const parseChangeDynamicFirstFrequency: NormWireReader<ChangeDynamicFirstFrequency> = normWireObject<ChangeDynamicFirstFrequency>({ id: normWireRequired(normWireString), newFirstFrequency: normWireRequired(normWireNumber) });
