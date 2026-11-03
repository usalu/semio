/** 🪢️ `change-bar-spacing` wire twin: the leaf payload `ChangeBarSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBarSpacing {
  wallId: string;
  id: string;
  newSpacingM: number;
}

export const parseChangeBarSpacing: NormWireReader<ChangeBarSpacing> = normWireObject<ChangeBarSpacing>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newSpacingM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
