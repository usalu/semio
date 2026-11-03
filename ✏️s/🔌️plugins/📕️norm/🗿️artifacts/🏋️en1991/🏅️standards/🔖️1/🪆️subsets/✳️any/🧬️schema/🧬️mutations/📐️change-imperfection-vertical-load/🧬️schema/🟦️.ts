/** 📐 `change-imperfection-vertical-load` wire twin: the leaf payload `ChangeImperfectionVerticalLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImperfectionVerticalLoad {
  id: string;
  newVerticalLoad: number;
}

export const parseChangeImperfectionVerticalLoad: NormWireReader<ChangeImperfectionVerticalLoad> = normWireObject<ChangeImperfectionVerticalLoad>({ id: normWireRequired(normWireString), newVerticalLoad: normWireRequired(normWireNumber) });
