/** ✖️ `change-bracing-spacing` wire twin: the leaf payload `ChangeBracingSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingSpacing {
  bracingId: string;
  newValue: number;
}

export const parseChangeBracingSpacing: NormWireReader<ChangeBracingSpacing> = normWireObject<ChangeBracingSpacing>({ bracingId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
