/** 🏢️ `change-storey-building-height` wire twin: the leaf payload `ChangeStoreyBuildingHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStoreyBuildingHeight {
  id: string;
  newBuildingHeight: number;
}

export const parseChangeStoreyBuildingHeight: NormWireReader<ChangeStoreyBuildingHeight> = normWireObject<ChangeStoreyBuildingHeight>({ id: normWireRequired(normWireString), newBuildingHeight: normWireRequired(normWireNumber) });
