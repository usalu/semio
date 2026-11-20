import { resolve } from "node:path";
import baseConfig from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts";

export default (environment: Parameters<typeof baseConfig>[0]) => {
  const base = baseConfig(environment);
  return { ...base, plugins: [], build: { ...base.build, rollupOptions: { ...base.build?.rollupOptions, input: resolve(import.meta.dirname, "staffage-worker-entry.ts") } } };
};
