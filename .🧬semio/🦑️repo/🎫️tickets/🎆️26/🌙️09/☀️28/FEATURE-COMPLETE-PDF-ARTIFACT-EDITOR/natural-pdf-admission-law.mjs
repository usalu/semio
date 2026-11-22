import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import Ajv from "ajv";

const read = (path) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const fixture = read("./natural-pdf-admission.json");
const schema = read("./natural-pdf-admission.schema.json");
const ajv = new Ajv({ strict: true });
ajv.addSchema(schema, "admission");
assert.equal(ajv.validate("admission", fixture), true, ajv.errorsText());

const database = new DatabaseSync(":memory:");
const query = database.prepare(`
  WITH inputs AS (SELECT json(?) trusted, json(?) offer, json(?) control, json(?) page, json(?) live)
  SELECT
    trusted ->> '$.fresh' AND live ->> '$.fresh'
    AND trusted ->> '$.operation' = control ->> '$.loan.operation'
    AND trusted ->> '$.operation' = live ->> '$.operation'
    AND trusted ->> '$.generation' = control ->> '$.loan.generation'
    AND trusted ->> '$.generation' = live ->> '$.generation'
    AND trusted ->> '$.instance' = control ->> '$.loan.instance'
    AND trusted ->> '$.instance' = live ->> '$.instance'
    AND trusted ->> '$.instance' = control ->> '$.owner'
    AND trusted ->> '$.declaration.formatKind' = offer ->> '$.declaration.formatKind'
    AND trusted ->> '$.declaration.extension' = offer ->> '$.declaration.extension'
    AND trusted ->> '$.declaration.mediaType' = offer ->> '$.declaration.mediaType'
    AND trusted ->> '$.declaration.binary' = offer ->> '$.declaration.binary'
    AND trusted ->> '$.declaration.formatKind' = control ->> '$.loan.declaration.formatKind'
    AND trusted ->> '$.declaration.extension' = control ->> '$.loan.declaration.extension'
    AND trusted ->> '$.declaration.mediaType' = control ->> '$.loan.declaration.mediaType'
    AND trusted ->> '$.declaration.binary' = control ->> '$.loan.declaration.binary'
    AND offer ->> '$.extent' BETWEEN 1 AND (trusted ->> '$.maximum')
    AND offer ->> '$.extent' = control ->> '$.loan.extent'
    AND offer ->> '$.extent' = control ->> '$.extent'
    AND page ->> '$.owner' = control ->> '$.owner'
    AND page ->> '$.instance' = live ->> '$.instance'
    AND page ->> '$.generation' = live ->> '$.generation'
    AND page ->> '$.seq' = control ->> '$.seq'
    AND page ->> '$.kind' = control ->> '$.kind'
    AND page ->> '$.metadata' = offer ->> '$.extent'
    AND page ->> '$.pageIndex' = page ->> '$.nextPage'
    AND page ->> '$.pageIndex' < page ->> '$.pageCount'
    AND page ->> '$.pageCount' = ((offer ->> '$.extent') + 4095) / 4096
    AND page ->> '$.len' = min(4096, (offer ->> '$.extent') - (page ->> '$.pageIndex') * 4096)
    AS expected
  FROM inputs
`);

function overlay(base, delta = {}) {
  return Object.fromEntries([...new Set([...Object.keys(base), ...Object.keys(delta)])].map((key) => {
    const value = delta[key];
    if (value === undefined) return [key, base[key]];
    return [key, value && typeof value === "object" ? overlay(base[key] ?? {}, value) : value];
  }));
}

const rs = JSON.stringify;
const operation = ({ operation, instance, generation }) =>
  `NaturalSourceOperation { operation: 0x${operation}, generation: 0x${generation}, instance: ${instance} }`;
const declaration = ({ formatKind, extension, mediaType, binary }) =>
  `NaturalSourceDeclaration { format_kind: ${rs(formatKind)}, extension: ${rs(extension)}, media_type: ${rs(mediaType)}, binary: ${binary} }`;
const offerSource = ({ declaration: selected, extent }) =>
  `NaturalSourceOffer { declaration: ${declaration(selected)}, extent: ${extent} }`;
const beginSource = ({ loan, owner, seq, kind, extent }) =>
  `NaturalSourceBegin { loan: NaturalSourceLoan { operation: ${operation(loan)}, declaration: ${declaration(loan.declaration)}, extent: ${loan.extent} }, owner: ${owner}, seq: 0x${seq}, kind: ${kind}, extent: ${extent} }`;
const pageSource = (page) =>
  `NaturalSourcePage { owner: ${page.owner}, generation: 0x${page.generation}, command_index: ${page.commandIndex}, command_count: ${page.commandCount}, instance: ${page.instance}, seq: 0x${page.seq}, kind: ${page.kind}, page_index: ${page.pageIndex}, page_count: ${page.pageCount}, item_count: ${page.itemCount}, metadata: ${page.metadata} }`;

const tests = [];
for (const row of fixture.cases) {
  const trusted = overlay(fixture.trusted, row.trusted);
  const offer = overlay(fixture.offer, row.offer);
  const begin = overlay(fixture.begin, row.begin);
  const page = overlay(fixture.page, row.page);
  const live = overlay(trusted, row.live);
  const wire = Object.entries({ trusted, offer, begin, page }).every(([type, value]) => ajv.validate(`admission#/definitions/${type}`, value));
  const relation = query.get(...[trusted, offer, begin, page, live].map(JSON.stringify));
  assert.equal(wire && Boolean(relation.expected), row.expected === "accepted", row.name);

  if (row.wireOnly) {
    assert.equal(wire, false, row.name);
    continue;
  }
  tests.push(`
    #[test]
    fn case_${tests.length}() {
        let prepared = PreparedNaturalSource::issue(${operation(trusted)}, ${trusted.fresh}, ${declaration(trusted.declaration)}, ${trusted.maximum}, ${offerSource(offer)});
        let begin = ${beginSource(begin)};
        let binding = prepared.as_ref().and_then(|source| {
            let issued = source.loan();
            assert_eq!(issued.extent, ${offer.extent});
            source.bind(begin, ${operation(trusted)}, ${trusted.fresh})
        });
        let result = binding.is_some_and(|binding| {
            binding.admits_control(begin, ${operation(live)}, ${live.fresh})
                && binding.admits_page(${pageSource(page)}, ${page.len}, ${page.nextPage}, ${operation(live)}, ${live.fresh})
        });
        assert_eq!(result, ${row.expected === "accepted"}, ${rs(row.name)});
    }`);
}
const completion = database.prepare(`
  SELECT CASE
    WHEN stage = 'Published' THEN 'Accepted'
    WHEN stage = 'Failed' THEN 'Refused'
    WHEN stage IN ('Sealed', 'Lent') THEN 'Pending'
    WHEN stage = 'Receive' AND acknowledged = (CAST(extent AS INTEGER) + 4095) / 4096 AND NOT pending THEN 'Pending'
    WHEN stage = 'Receive' THEN 'Refused'
    ELSE 'Stale'
  END AS status
  FROM (SELECT ? stage, ? acknowledged, ? pending, ? extent)
`);
for (const row of fixture.completion) {
  const result = completion.get(row.stage, row.acknowledged, Number(row.pendingPage), fixture.offer.extent);
  assert.equal(result.status, row.expected, row.name);
  tests.push(`
    #[test]
    fn case_${tests.length}() {
        assert_eq!(Stage::${row.stage}.complete(${fixture.offer.extent}, ${row.acknowledged}, ${row.pendingPage}), NaturalSourceStatus::${row.expected}, ${rs(row.name)});
    }`);
}
database.close();

const generated = new URL("./🗑️generated/natural-pdf-admission/", import.meta.url);
mkdirSync(generated, { recursive: true });
writeFileSync(new URL("law.rs", generated), `#[path = "../../natural-pdf-admission.rs"] mod admission;
use admission::*;
${tests.join("\n")}
`);
console.log(`[DEBUG] Ajv and SQLite agreed with all ${fixture.cases.length + fixture.completion.length} expected outcomes. Wrote ${tests.length} Rust cases.`);
