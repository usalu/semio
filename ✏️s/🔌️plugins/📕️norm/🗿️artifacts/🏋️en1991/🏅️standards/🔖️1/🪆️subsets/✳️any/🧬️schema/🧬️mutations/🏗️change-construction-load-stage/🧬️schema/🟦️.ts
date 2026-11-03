/** 🏗 `change-construction-load-stage` wire twin: the leaf payload `ChangeConstructionLoadStage`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConstructionLoadStage {
  id: string;
  newStage: string;
}

export const parseChangeConstructionLoadStage: NormWireReader<ChangeConstructionLoadStage> = normWireObject<ChangeConstructionLoadStage>({ id: normWireRequired(normWireString), newStage: normWireRequired(normWireString) });
