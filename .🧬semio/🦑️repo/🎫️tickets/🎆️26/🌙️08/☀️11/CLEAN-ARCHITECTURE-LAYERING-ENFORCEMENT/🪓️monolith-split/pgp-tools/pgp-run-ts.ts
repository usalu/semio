import { parseProjectionArguments, projectParsers, renderProjectionReport } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts";

const [root, ...argv] = process.argv.slice(2);
process.stdout.write(renderProjectionReport(projectParsers(parseProjectionArguments(root!, argv))));
