/** 🧪️ Runs the page chrome verification directly (the quick pipeline level runs it through `repo`). */
import { verifyPageChromeBorders, verifyPageChromeLogo } from "../../../../../../../🧰️framework/🛍️products/📓️print/🧪️tests/🏷️page-chrome/🟦️.ts";

verifyPageChromeBorders();
await verifyPageChromeLogo();
console.log("page chrome: passed");
