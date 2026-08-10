const m = await import("/Users/ueli/Documents/semio/\ud83e\uddf0\ufe0fframework/\ud83d\udecd\ufe0fproducts/\ud83e\udd91\ufe0frepo/\ud83d\udd28\ufe0fmodules/\ud83d\udcda\ufe0flibrary/\ud83d\udd0d\ufe0fdiscovery/\ud83d\udfe6\ufe0fcomponent.ts");
const problems = m.validateTaxonomy();
console.log('PROBLEMS', JSON.stringify(problems, null, 2));
console.log('COUNT', problems.length);
const checks = [["🧬️schema/📸️snapshot/📝️text", true], ["🧬️schema/🧬️mutations/🏗️foo/🦠️mutation", true], ["🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🏗️txt", true], ["🏗️builder", true], ["not-a-facet", false]];
let fail = 0;
for (const [p, expect] of checks) {
  const got = m.artifactFacetPathIsDeclared(p);
  const ok = got === expect;
  if (!ok) fail++;
  console.log(ok ? 'OK' : 'FAIL', p, 'got', got, 'expect', expect);
}
if (problems.length !== 0 || fail !== 0) process.exit(1);
console.log('VALIDATE_TAXONOMY_OK');
