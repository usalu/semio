/** 📐 `change-imperfection-height` wire twin: the leaf payload `ChangeImperfectionHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImperfectionHeight {
  id: string;
  newHeight: number;
}

export const parseChangeImperfectionHeight: NormWireReader<ChangeImperfectionHeight> = normWireObject<ChangeImperfectionHeight>({ id: normWireRequired(normWireString), newHeight: normWireRequired(normWireNumber) });
