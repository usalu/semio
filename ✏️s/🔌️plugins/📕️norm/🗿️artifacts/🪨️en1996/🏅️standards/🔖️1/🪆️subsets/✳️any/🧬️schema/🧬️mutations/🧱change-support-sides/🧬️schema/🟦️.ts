/** 🧱 `change-support-sides` wire twin: the leaf payload `ChangeSupportSides`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSupportSides {
  id: string;
  newSupportSides: number;
}

export const parseChangeSupportSides: NormWireReader<ChangeSupportSides> = normWireObject<ChangeSupportSides>({ id: normWireRequired(normWireString), newSupportSides: normWireRequired(normWireRange(normWireInteger, {"minimum":2,"maximum":4})) });
