/** 🛣️ `change-lane-width` wire twin: the leaf payload `ChangeLaneWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLaneWidth {
  id: string;
  newWidth: number;
}

export const parseChangeLaneWidth: NormWireReader<ChangeLaneWidth> = normWireObject<ChangeLaneWidth>({ id: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
