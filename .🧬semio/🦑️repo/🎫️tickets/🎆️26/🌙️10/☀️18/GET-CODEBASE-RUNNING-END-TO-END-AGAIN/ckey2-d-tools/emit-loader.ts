import{sourceClosureLoaderV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ckey2.ts";
import{writeFileSync}from"node:fs";
const out=process.argv[2]!;
writeFileSync(out+".file.mjs",sourceClosureLoaderV1(false));writeFileSync(out+".stream.mjs",sourceClosureLoaderV1(true));
