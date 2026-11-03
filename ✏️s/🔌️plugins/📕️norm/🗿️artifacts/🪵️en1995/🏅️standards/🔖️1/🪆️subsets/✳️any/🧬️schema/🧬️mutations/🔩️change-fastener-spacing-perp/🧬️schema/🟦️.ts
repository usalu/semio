/** 🔩️ `change-fastener-spacing-perp` wire twin: the leaf payload `ChangeFastenerSpacingPerp`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerSpacingPerp {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerSpacingPerp: NormWireReader<ChangeFastenerSpacingPerp> = normWireObject<ChangeFastenerSpacingPerp>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
