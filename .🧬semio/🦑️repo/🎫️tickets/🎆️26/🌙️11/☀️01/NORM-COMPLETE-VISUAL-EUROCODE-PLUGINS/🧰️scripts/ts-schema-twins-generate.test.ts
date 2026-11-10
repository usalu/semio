import { test } from "bun:test";
import { renderTwinReport, runTwins } from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts";
test("generate twins under scope", () => {
  console.log(renderTwinReport("generate", runTwins("/home/deck/semio", ["generate", "--under", process.env.UNDER!, "--explain"])));
});
