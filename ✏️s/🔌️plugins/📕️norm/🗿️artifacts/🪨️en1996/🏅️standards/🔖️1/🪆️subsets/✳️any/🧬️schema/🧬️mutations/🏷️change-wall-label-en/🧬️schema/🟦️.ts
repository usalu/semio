/** 🏷️ `change-wall-label-en` wire twin: the leaf payload `ChangeWallLabelEn`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallLabelEn {
  id: string;
  newLabelEn: string;
}

export const parseChangeWallLabelEn: NormWireReader<ChangeWallLabelEn> = normWireObject<ChangeWallLabelEn>({ id: normWireRequired(normWireString), newLabelEn: normWireRequired(normWireString) });
