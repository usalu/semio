/** 🌉️ `change-bridge-crossbeam-spacing` wire twin: the leaf payload `ChangeBridgeCrossbeamSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeCrossbeamSpacing {
  id: string;
  newCrossbeamSpacing: number;
}

export const parseChangeBridgeCrossbeamSpacing: NormWireReader<ChangeBridgeCrossbeamSpacing> = normWireObject<ChangeBridgeCrossbeamSpacing>({ id: normWireRequired(normWireString), newCrossbeamSpacing: normWireRequired(normWireNumber) });
