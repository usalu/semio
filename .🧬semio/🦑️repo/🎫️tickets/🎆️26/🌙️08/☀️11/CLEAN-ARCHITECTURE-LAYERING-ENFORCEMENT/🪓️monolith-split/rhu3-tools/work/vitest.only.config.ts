/** 🧪️ Test-only wrapper of the renderer vitest config that runs only the files named in `RHU3_ONLY` (comma separated absolute paths), at the same alias and environment settings. */
import base from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
const only = (process.env.RHU3_ONLY ?? "").split(",").filter(Boolean);
export default { ...base, test: { ...base.test, include: only, includeSource: [], exclude: [] } };
