/** 🔗 `change-tie-run-spacing` wire twin: the leaf payload `ChangeTieRunSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTieRunSpacing {
  id: string;
  newSpacing: number;
}

export const parseChangeTieRunSpacing: NormWireReader<ChangeTieRunSpacing> = normWireObject<ChangeTieRunSpacing>({ id: normWireRequired(normWireString), newSpacing: normWireRequired(normWireNumber) });
