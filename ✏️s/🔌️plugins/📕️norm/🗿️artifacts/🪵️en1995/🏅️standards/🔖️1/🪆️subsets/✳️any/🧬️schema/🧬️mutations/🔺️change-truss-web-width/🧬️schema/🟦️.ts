/** 🔺️ `change-truss-web-width` wire twin: the leaf payload `ChangeTrussWebWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTrussWebWidth {
  trussId: string;
  newValue: number;
}

export const parseChangeTrussWebWidth: NormWireReader<ChangeTrussWebWidth> = normWireObject<ChangeTrussWebWidth>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
