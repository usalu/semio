import { checkParsers, renderParserCheck } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";

const [root, strict] = process.argv.slice(2);
const check = checkParsers({ root: root!, strict: strict === "strict" });
process.stdout.write(renderParserCheck(check));
