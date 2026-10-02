/** 🧱 `change-attachment` wire twin: the leaf payload `ChangeAttachment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599Attachment, parseDin18599Attachment } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAttachment {
  mutation: "changeAttachment";
  newAttachment: Din18599Attachment;
}

export const parseChangeAttachment: NormWireReader<ChangeAttachment> = normWireObject<ChangeAttachment>({ mutation: normWireRequired(normWireLiteral("changeAttachment")), newAttachment: normWireRequired(parseDin18599Attachment) });
