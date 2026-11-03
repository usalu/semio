/** 📏️ `change-buckling-length-plane` wire twin: the leaf payload `ChangeBucklingLengthPlane`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BucklingPlane, parseBucklingPlane } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBucklingLengthPlane {
  id: string;
  newPlane: BucklingPlane;
}

export const parseChangeBucklingLengthPlane: NormWireReader<ChangeBucklingLengthPlane> = normWireObject<ChangeBucklingLengthPlane>({ id: normWireRequired(normWireString), newPlane: normWireRequired(parseBucklingPlane) });
