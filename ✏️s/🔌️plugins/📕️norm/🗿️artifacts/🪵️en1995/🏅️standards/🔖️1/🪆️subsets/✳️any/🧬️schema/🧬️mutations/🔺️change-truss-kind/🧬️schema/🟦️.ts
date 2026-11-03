/** 🔺️ `change-truss-kind` wire twin: the leaf payload `ChangeTrussKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type TrussKind, parseTrussKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTrussKind {
  trussId: string;
  newValue: TrussKind;
}

export const parseChangeTrussKind: NormWireReader<ChangeTrussKind> = normWireObject<ChangeTrussKind>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(parseTrussKind) });
