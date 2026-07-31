import { parseConstruct } from "../../../../../../spatial/js/query/index.ts";

const q = `MATCH (c:Cell)-[:BOUNDED_BY]->(:Shell)-[:CONTAINS]->(f:Face) RETURN f.id`;
const a = parseConstruct(q);
console.log(JSON.stringify(a.returnClause?.projections[0]?.expr, null, 2));

const w = parseConstruct(`MATCH (s:Surface) WHERE s.exposure = 'external' RETURN s.id`);
console.log("where", JSON.stringify((w.clauses[0] as { where?: unknown }).where, null, 2));
