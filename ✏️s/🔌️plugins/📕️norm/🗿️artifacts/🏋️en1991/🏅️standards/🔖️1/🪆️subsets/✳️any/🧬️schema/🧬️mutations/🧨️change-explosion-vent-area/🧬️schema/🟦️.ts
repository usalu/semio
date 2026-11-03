/** 🧨 `change-explosion-vent-area` wire twin: the leaf payload `ChangeExplosionVentArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeExplosionVentArea {
  id: string;
  newVentArea: number;
}

export const parseChangeExplosionVentArea: NormWireReader<ChangeExplosionVentArea> = normWireObject<ChangeExplosionVentArea>({ id: normWireRequired(normWireString), newVentArea: normWireRequired(normWireNumber) });
