import {runCanonicalGoTests} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
await runCanonicalGoTests(process.argv[2]!, ["-count=1", ...process.argv.slice(3)], { env: { ...process.env, GOWORK: `${root}/go.work` } });
console.log("runCanonicalGoTests ok");
