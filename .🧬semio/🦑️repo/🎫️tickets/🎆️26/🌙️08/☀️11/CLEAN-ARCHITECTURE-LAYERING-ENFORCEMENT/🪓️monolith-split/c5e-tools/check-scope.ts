import { checkParsers, renderParserCheck } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";
const scopes = process.argv.slice(2);
const check = checkParsers({ root: "/Users/ueli/Documents/semio", scopes });
process.stdout.write(renderParserCheck(check));
