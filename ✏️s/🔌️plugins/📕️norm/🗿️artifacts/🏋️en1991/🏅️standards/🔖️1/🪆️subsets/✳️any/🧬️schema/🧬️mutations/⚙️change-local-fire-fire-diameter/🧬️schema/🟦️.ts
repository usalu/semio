/** ⚙ `change-local-fire-fire-diameter` wire twin: the leaf payload `ChangeLocalFireFireDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLocalFireFireDiameter {
  id: string;
  newFireDiameter: number;
}

export const parseChangeLocalFireFireDiameter: NormWireReader<ChangeLocalFireFireDiameter> = normWireObject<ChangeLocalFireFireDiameter>({ id: normWireRequired(normWireString), newFireDiameter: normWireRequired(normWireNumber) });
