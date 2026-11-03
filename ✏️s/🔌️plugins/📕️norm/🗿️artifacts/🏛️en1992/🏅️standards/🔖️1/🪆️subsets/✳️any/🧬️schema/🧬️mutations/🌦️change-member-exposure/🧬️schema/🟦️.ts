/** 🌦️ `change-member-exposure` wire twin: the leaf payload `ChangeMemberExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ExposureClass, parseExposureClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberExposure {
  memberId: string;
  newExposure: ExposureClass;
}

export const parseChangeMemberExposure: NormWireReader<ChangeMemberExposure> = normWireObject<ChangeMemberExposure>({ memberId: normWireRequired(normWireString), newExposure: normWireRequired(parseExposureClass) });
