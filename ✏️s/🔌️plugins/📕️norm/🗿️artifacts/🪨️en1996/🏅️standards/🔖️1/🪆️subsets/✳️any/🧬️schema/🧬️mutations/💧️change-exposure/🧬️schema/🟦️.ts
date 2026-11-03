/** 💧️ `change-exposure` wire twin: the leaf payload `ChangeExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1996ExposureClass, parseEn1996ExposureClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeExposure {
  id: string;
  newExposure: En1996ExposureClass;
}

export const parseChangeExposure: NormWireReader<ChangeExposure> = normWireObject<ChangeExposure>({ id: normWireRequired(normWireString), newExposure: normWireRequired(parseEn1996ExposureClass) });
