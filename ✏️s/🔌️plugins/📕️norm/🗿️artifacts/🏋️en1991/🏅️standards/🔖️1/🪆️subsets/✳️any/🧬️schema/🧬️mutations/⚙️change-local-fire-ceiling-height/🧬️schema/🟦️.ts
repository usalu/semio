/** ⚙ `change-local-fire-ceiling-height` wire twin: the leaf payload `ChangeLocalFireCeilingHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLocalFireCeilingHeight {
  id: string;
  newCeilingHeight: number;
}

export const parseChangeLocalFireCeilingHeight: NormWireReader<ChangeLocalFireCeilingHeight> = normWireObject<ChangeLocalFireCeilingHeight>({ id: normWireRequired(normWireString), newCeilingHeight: normWireRequired(normWireNumber) });
