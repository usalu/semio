from pathlib import Path

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))
script = next((space / "📦️packages").rglob("📜️script.ts"))
text = script.read_text()
if "persistenceDataClassOracle" in text:
    print("oracle already present")
else:
    oracle_fn = r'''
/** 🗃️ Proves PersistenceDataClass routing: hub origin, ephemeral block/promote, schema-first enum. */
export function persistenceDataClassOracle(repoRoot: string): number {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧬️schema/persistence-data-class/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(schema);
  const validateBinding = ajv.compile({ $ref: `${schema.$id}#/$defs/ClassifiedPersistenceBinding` });
  const validateLane = ajv.compile({ $ref: `${schema.$id}#/$defs/ClassifiedWireLane` });
  for (const c of fixture.cases) {
    assert(validateBinding(c.binding), JSON.stringify(validateBinding.errors));
    if (c.wireLane) assert(validateLane(c.wireLane), JSON.stringify(validateLane.errors));
  }
  const core = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/