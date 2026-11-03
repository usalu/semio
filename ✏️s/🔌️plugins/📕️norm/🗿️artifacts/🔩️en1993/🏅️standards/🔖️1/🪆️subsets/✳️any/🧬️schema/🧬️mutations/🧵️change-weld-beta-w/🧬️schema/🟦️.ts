/** 🧵️ `change-weld-beta-w` wire twin: the leaf payload `ChangeWeldBetaW`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldBetaW {
  id: string;
  newBetaW: number;
}

export const parseChangeWeldBetaW: NormWireReader<ChangeWeldBetaW> = normWireObject<ChangeWeldBetaW>({ id: normWireRequired(normWireString), newBetaW: normWireRequired(normWireNumber) });
