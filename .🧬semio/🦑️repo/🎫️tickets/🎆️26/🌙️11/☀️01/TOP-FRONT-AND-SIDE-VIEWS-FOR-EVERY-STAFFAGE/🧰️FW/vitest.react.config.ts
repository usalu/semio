import { resolve } from "node:path";
import base from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

export default {
  ...base,
  resolve: { ...base.resolve, alias: [...base.resolve!.alias as any[], { find: /^@semio-tech\/framework-transient$/u, replacement: resolve("C:/git/semio/🧰️framework/🔨️modules/🫧️transient/🟦️.ts") }] },
};
