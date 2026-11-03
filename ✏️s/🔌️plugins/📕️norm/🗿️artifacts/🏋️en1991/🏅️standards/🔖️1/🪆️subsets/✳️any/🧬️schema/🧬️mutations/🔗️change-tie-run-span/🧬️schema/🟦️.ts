/** 🔗 `change-tie-run-span` wire twin: the leaf payload `ChangeTieRunSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunSpan {
  id: string;
  newSpan: number;
}

export const parseChangeTieRunSpan: NormWireReader<ChangeTieRunSpan> = normWireObject<ChangeTieRunSpan>({ id: normWireRequired(normWireString), newSpan: normWireRequired(normWireNumber) });
