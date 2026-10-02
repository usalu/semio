/** 🌦️ `change-member-exposure` wire twin: the leaf payload `ChangeMemberExposure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ExposureClass, parseExposureClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMemberExposure {
  memberId: string;
  newExposure: ExposureClass;
}

export const parseChangeMemberExposure: NormWireReader<ChangeMemberExposure> = normWireObject<ChangeMemberExposure>({ memberId: normWireRequired(normWireString), newExposure: normWireRequired(parseExposureClass) });
