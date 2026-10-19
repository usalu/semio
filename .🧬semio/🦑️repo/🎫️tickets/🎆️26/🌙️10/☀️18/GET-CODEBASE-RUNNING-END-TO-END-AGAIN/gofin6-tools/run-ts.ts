const {runCanonicalGoTests} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts");
const [moduleRoot, gowork] = process.argv.slice(2);
await runCanonicalGoTests(moduleRoot!, ["-count=1"], {env: {...process.env, GOWORK: gowork!}});
console.log("runCanonicalGoTests ok");
