/** 🔷 `change-element-tilt-deg` wire twin: the leaf payload `ChangeElementTiltDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementTiltDeg {
  mutation: "changeelementtiltdeg";
  elementId: string;
  newTiltDeg: number;
}

export const parseChangeElementTiltDeg: NormWireReader<ChangeElementTiltDeg> = normWireObject<ChangeElementTiltDeg>({ mutation: normWireRequired(normWireLiteral("changeelementtiltdeg")), elementId: normWireRequired(normWireString), newTiltDeg: normWireRequired(normWireNumber) });
