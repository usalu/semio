/** 🏢 `change-fire-occupancy` wire twin: the leaf payload `ChangeFireOccupancy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1991FireOccupancy, parseEn1991FireOccupancy } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFireOccupancy {
  newFireOccupancy: En1991FireOccupancy;
}

export const parseChangeFireOccupancy: NormWireReader<ChangeFireOccupancy> = normWireObject<ChangeFireOccupancy>({ newFireOccupancy: normWireRequired(parseEn1991FireOccupancy) });
