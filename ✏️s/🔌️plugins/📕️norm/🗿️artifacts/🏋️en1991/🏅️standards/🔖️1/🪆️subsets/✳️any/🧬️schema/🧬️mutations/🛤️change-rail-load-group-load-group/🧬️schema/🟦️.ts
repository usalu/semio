/** 🛤 `change-rail-load-group-load-group` wire twin: the leaf payload `ChangeRailLoadGroupLoadGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRailLoadGroupLoadGroup {
  id: string;
  newLoadGroup: string;
}

export const parseChangeRailLoadGroupLoadGroup: NormWireReader<ChangeRailLoadGroupLoadGroup> = normWireObject<ChangeRailLoadGroupLoadGroup>({ id: normWireRequired(normWireString), newLoadGroup: normWireRequired(normWireString) });
