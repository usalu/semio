/** 🔺️ `change-truss-span` wire twin: the leaf payload `ChangeTrussSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussSpan {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussSpan: NormWireReader<ChangeTrussSpan> = normWireObject<ChangeTrussSpan>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
