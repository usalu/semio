const {canonicalGoPlan} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts");
const p = canonicalGoPlan(process.argv[2]!);
console.log(JSON.stringify({packages: p.packages, sources: Object.keys(p.sourceReplacements).length, tests: Object.keys(p.testReplacements).length}));
