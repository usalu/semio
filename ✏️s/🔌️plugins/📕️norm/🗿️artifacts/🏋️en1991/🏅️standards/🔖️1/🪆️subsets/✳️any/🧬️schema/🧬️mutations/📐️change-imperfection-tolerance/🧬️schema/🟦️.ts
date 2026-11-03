/** 📐 `change-imperfection-tolerance` wire twin: the leaf payload `ChangeImperfectionTolerance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImperfectionTolerance {
  id: string;
  newTolerance: number;
}

export const parseChangeImperfectionTolerance: NormWireReader<ChangeImperfectionTolerance> = normWireObject<ChangeImperfectionTolerance>({ id: normWireRequired(normWireString), newTolerance: normWireRequired(normWireNumber) });
