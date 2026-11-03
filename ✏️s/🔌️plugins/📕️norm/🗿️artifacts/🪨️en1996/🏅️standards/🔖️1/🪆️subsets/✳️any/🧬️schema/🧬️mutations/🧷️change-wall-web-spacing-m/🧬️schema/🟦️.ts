/** 🧷 `change-wall-web-spacing-m` wire twin: the leaf payload `ChangeWallWebSpacingM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWallWebSpacingM {
  id: string;
  newWebSpacingM: number;
}

export const parseChangeWallWebSpacingM: NormWireReader<ChangeWallWebSpacingM> = normWireObject<ChangeWallWebSpacingM>({ id: normWireRequired(normWireString), newWebSpacingM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
