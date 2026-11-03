/** 🪵️ `change-pile-wall-thickness` wire twin: the leaf payload `ChangePileWallThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileWallThickness {
  id: string;
  newWallThickness: number;
}

export const parseChangePileWallThickness: NormWireReader<ChangePileWallThickness> = normWireObject<ChangePileWallThickness>({ id: normWireRequired(normWireString), newWallThickness: normWireRequired(normWireNumber) });
