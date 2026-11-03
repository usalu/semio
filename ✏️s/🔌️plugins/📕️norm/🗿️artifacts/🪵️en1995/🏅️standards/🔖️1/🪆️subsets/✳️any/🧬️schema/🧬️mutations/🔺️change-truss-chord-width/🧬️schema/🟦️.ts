/** 🔺️ `change-truss-chord-width` wire twin: the leaf payload `ChangeTrussChordWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussChordWidth {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussChordWidth: NormWireReader<ChangeTrussChordWidth> = normWireObject<ChangeTrussChordWidth>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
