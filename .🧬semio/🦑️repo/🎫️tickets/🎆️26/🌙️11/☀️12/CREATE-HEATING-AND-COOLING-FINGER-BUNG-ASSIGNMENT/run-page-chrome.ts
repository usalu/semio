import * as chrome from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🟦️.ts";

for (const [name, verify] of Object.entries(chrome)) {
  if (typeof verify !== "function") continue;
  try { (verify as () => void)(); console.log("pass " + name); } catch (error) { console.log("FAIL " + name + ": " + String((error as Error).message).split("\n")[0]); }
}
