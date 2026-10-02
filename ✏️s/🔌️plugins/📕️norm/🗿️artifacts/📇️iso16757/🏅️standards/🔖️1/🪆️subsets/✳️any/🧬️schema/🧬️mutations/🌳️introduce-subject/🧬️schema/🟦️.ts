/** 🌳️ `introduce-subject` wire twin: the leaf payload `IntroduceSubject`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSubject, type Subject } from "../../../📸️snapshot/🟦️.ts";

export interface IntroduceSubject {
  subject: Subject;
  index: number | null;
}

export const parseIntroduceSubject: NormWireReader<IntroduceSubject> = normWireObject<IntroduceSubject>({ subject: normWireRequired(parseSubject), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
