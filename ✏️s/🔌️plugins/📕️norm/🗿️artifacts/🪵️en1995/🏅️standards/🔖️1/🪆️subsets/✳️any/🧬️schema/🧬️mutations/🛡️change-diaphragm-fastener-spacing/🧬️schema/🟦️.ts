/** 🛡️ `change-diaphragm-fastener-spacing` wire twin: the leaf payload `ChangeDiaphragmFastenerSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmFastenerSpacing {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmFastenerSpacing: NormWireReader<ChangeDiaphragmFastenerSpacing> = normWireObject<ChangeDiaphragmFastenerSpacing>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
