/** 📐️ `introduce-property-definition` wire twin: the leaf payload `IntroducePropertyDefinition`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parsePropertyDefinition, type PropertyDefinition } from "../../../📸️snapshot/🟦️.ts";

export interface IntroducePropertyDefinition {
  propertyDefinition: PropertyDefinition;
  index: number | null;
}

export const parseIntroducePropertyDefinition: NormWireReader<IntroducePropertyDefinition> = normWireObject<IntroducePropertyDefinition>({ propertyDefinition: normWireRequired(parsePropertyDefinition), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
