/** 🔋 `change-assumed-qf-d` wire twin: the leaf payload `ChangeAssumedQfD`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAssumedQfD {
  newAssumedQfD: number;
}

export const parseChangeAssumedQfD: NormWireReader<ChangeAssumedQfD> = normWireObject<ChangeAssumedQfD>({ newAssumedQfD: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
