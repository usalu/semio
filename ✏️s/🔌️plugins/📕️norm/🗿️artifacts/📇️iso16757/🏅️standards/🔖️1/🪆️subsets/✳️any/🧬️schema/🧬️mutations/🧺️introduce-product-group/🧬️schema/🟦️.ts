/** 🧺️ `introduce-product-group` wire twin: the leaf payload `IntroduceProductGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseProductGroup, type ProductGroup } from "../../../📸️snapshot/🟦️.ts";

export interface IntroduceProductGroup {
  productGroup: ProductGroup;
  index: number | null;
}

export const parseIntroduceProductGroup: NormWireReader<IntroduceProductGroup> = normWireObject<IntroduceProductGroup>({ productGroup: normWireRequired(parseProductGroup), index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
