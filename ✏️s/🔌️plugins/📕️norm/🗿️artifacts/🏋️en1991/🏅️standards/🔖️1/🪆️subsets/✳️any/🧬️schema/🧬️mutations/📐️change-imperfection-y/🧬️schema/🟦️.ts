/** 📐 `change-imperfection-y` wire twin: the leaf payload `ChangeImperfectionY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeImperfectionY {
  id: string;
  newY: number;
}

export const parseChangeImperfectionY: NormWireReader<ChangeImperfectionY> = normWireObject<ChangeImperfectionY>({ id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
