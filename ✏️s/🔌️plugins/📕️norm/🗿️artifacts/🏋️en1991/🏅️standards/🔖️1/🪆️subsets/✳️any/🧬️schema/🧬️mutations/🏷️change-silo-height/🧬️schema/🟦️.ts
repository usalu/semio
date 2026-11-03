/** 🏷️ `change-silo-height` wire twin: the leaf payload `ChangeSiloHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloHeight {
  newSiloHeight: number;
}

export const parseChangeSiloHeight: NormWireReader<ChangeSiloHeight> = normWireObject<ChangeSiloHeight>({ newSiloHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
