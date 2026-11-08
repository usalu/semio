/** 🔶 `change-element-orientation-deg` wire twin: the leaf payload `ChangeElementOrientationDeg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementOrientationDeg {
  mutation: "changeelementorientationdeg";
  elementId: string;
  newOrientationDeg: number;
}

export const parseChangeElementOrientationDeg: NormWireReader<ChangeElementOrientationDeg> = normWireObject<ChangeElementOrientationDeg>({ mutation: normWireRequired(normWireLiteral("changeelementorientationdeg")), elementId: normWireRequired(normWireString), newOrientationDeg: normWireRequired(normWireNumber) });
