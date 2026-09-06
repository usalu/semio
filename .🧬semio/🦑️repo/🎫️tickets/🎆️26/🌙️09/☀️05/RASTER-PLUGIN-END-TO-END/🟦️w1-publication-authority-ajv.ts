import Ajv from "ajv";
const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🔏️publication-authority/";
const fixture = await Bun.file(root + "🔣️.json").json();
const schema = await Bun.file(root + "🧬️.schema.json").json();
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
if (!validate(fixture)) { console.log("AJV FAIL", JSON.stringify(validate.errors, null, 2)); process.exit(1); }
console.log("AJV OK");
const routes = fixture.owners.flatMap((o: any) => o.groups.flatMap((g: any) => g.routes));
console.log("routes", routes.length, "unique", new Set(routes).size);
const laws = Object.values(fixture.laws).every(Boolean);
console.log("laws all true:", laws);
const hostOnlyExclusive = fixture.owners.every((o: any) => o.groups.every((g: any) => !g.lanes.includes("HostOnly") || g.lanes.length === 1));
console.log("hostOnlyExclusive:", hostOnlyExclusive);
const blockerRule = fixture.owners.every((o: any) => o.groups.every((g: any) => g.status === "Migrated" ? g.blocker === undefined : Boolean(g.blocker)));
console.log("blocker rule:", blockerRule);
// cross-check against the editor source's publication contracts
const src = await Bun.file("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/" + fixture.owners[0].source).text();
const actual = new Map([...src.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((m) => [m[1]!, [...m[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((l) => l[1]!)]));
const expected = new Map(fixture.owners[0].groups.flatMap((g: any) => g.routes.map((r: string) => [r, g.lanes])));
const eq = (a: string[], b: string[]) => JSON.stringify([...a].sort()) === JSON.stringify([...b].sort());
console.log("contract keys match:", eq([...actual.keys()], [...expected.keys()]));
console.log("lanes match:", [...expected].every(([r, l]) => eq(actual.get(r as string) ?? [], l as string[])));
