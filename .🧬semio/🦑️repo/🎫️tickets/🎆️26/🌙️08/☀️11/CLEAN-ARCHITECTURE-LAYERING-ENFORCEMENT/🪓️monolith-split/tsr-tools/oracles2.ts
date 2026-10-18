import * as S from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts";
const root = "/Users/ueli/Documents/semio";
for (const name of Object.keys(S).filter((k) => /Oracle$/.test(k))) {
  try { console.log(`${name}: checks=${(S as any)[name](root)} clean`); } catch (e) { console.log(`${name}: FAIL ${(e as Error).message.split("\n").slice(0, 3).join(" | ").slice(0, 400)}`); }
}
