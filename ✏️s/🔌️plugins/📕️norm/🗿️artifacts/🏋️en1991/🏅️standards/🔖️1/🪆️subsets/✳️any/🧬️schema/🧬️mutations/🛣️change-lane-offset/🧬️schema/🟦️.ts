/** 🛣️ `change-lane-offset` wire twin: the leaf payload `ChangeLaneOffset`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLaneOffset {
  id: string;
  newOffset: number;
}

export const parseChangeLaneOffset: NormWireReader<ChangeLaneOffset> = normWireObject<ChangeLaneOffset>({ id: normWireRequired(normWireString), newOffset: normWireRequired(normWireNumber) });
