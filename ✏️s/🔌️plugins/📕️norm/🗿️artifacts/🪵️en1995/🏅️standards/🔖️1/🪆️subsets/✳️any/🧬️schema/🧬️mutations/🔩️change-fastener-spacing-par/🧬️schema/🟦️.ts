/** 🔩️ `change-fastener-spacing-par` wire twin: the leaf payload `ChangeFastenerSpacingPar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerSpacingPar {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerSpacingPar: NormWireReader<ChangeFastenerSpacingPar> = normWireObject<ChangeFastenerSpacingPar>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
