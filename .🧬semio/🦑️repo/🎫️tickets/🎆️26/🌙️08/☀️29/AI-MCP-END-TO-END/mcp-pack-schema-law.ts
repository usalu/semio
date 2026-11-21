import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";

const scratch = resolve(import.meta.dir, "🗑️generated");
const build = join(scratch, "native-mcp/debug/build");
const owner = resolve("🧰️framework/🔨️modules/🎒️pack/🌱️value");
const output = join(scratch, "mcp-pack-schema-law" + (process.platform === "win32" ? ".exe" : ""));
const input = join(scratch, "mcp-pack-schema-law.rs");
const paths = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).flatMap(entry =>
  readdirSync(join(build, entry.name)).map(variant => join(entry.name, variant)).filter(variant => existsSync(join(build, variant, "out"))));
const fingerprintFile = (path: string, crate: string) => join(build, path, "fingerprint", `lib-${crate}`);
const packages = (crate: string) => paths.filter(path => path.startsWith(crate.replaceAll("_", "-") + sep));
const graphCrate = "semio_framework_pack_record_schema";
const [anchor] = packages(graphCrate).filter(path => existsSync(fingerprintFile(path, graphCrate)))
  .sort((a, b) => statSync(fingerprintFile(b, graphCrate)).mtimeMs - statSync(fingerprintFile(a, graphCrate)).mtimeMs);
if (!anchor) throw Error("Missing prebuilt Pack schema dependency bindings");
const fingerprint = readFileSync(fingerprintFile(anchor, graphCrate) + ".json", "utf8");
const dependencies = Array.from(fingerprint.matchAll(/"(semio_[a-z_]+)",false,(\d+)/gu), ([, crate, value]) => {
  const identity = Buffer.from(BigInt(value).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
  const path = packages(crate).find(candidate => {
    const recorded = fingerprintFile(candidate, crate);
    return existsSync(recorded) && readFileSync(recorded, "utf8").trim() === identity;
  });
  if (!path) throw Error(`Missing direct Pack dependency ${crate}`);
  const library = join(path, "out", `lib${crate}-${path.split(sep).at(-1)}`);
  return ["--extern", `${crate}=${library}.rmeta`, "--extern", `${crate}=${library}.rlib`];
}).flat();
const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔑️schema-hash/🔣️.json"), "utf8"));
const cases = fixture.cases.filter((row: { name: string }) => row.name !== "every-shape")
  .map((row: { name: string; canonicalHex: string; schemaHash: string }) => `(${JSON.stringify(row.name)},${JSON.stringify(row.canonicalHex)},${JSON.stringify(row.schemaHash)})`);
writeFileSync(input, `#![allow(dead_code)]
#[path = ${JSON.stringify(join(owner, "🧬️schema/🦀️.rs"))}] mod schema;
use semio_framework_dsl_record::{FieldSpec, RecordLayout, RecordSpec, RecordSpecProducer, Shape};
fn inline(fields: Vec<FieldSpec>) -> RecordSpec { RecordSpec::new(None, RecordLayout::Inline, fields) }
macro_rules! producer { ($function:ident) => { RecordSpecProducer { ordinary: $function, decoding: |_| Ok($function()), encoding: |_| Ok($function()) } }; }
fn inner() -> RecordSpec { inline(vec![FieldSpec::new(0,"x",Shape::Float)]) }
fn changed() -> RecordSpec { inline(vec![FieldSpec::new(0,"x",Shape::Int)]) }
fn other() -> RecordSpec { inline(vec![FieldSpec::new(0,"mode",Shape::Enum(vec![("beta".into(),1),("alpha".into(),0)]))]) }
fn nested(inner: RecordSpecProducer) -> RecordSpec { inline(vec![FieldSpec::new(2,"maybe",Shape::Record(producer!(other))).optional(),FieldSpec::new(0,"inner",Shape::Record(inner)),FieldSpec::new(1,"items",Shape::List(Box::new(Shape::Record(inner))))]) }
fn node() -> RecordSpec { inline(vec![FieldSpec::new(0,"name",Shape::Text),FieldSpec::new(1,"children",Shape::List(Box::new(Shape::Record(producer!(node))))),FieldSpec::new(2,"alias",Shape::Record(producer!(twin))).optional()]) }
fn twin() -> RecordSpec { inline(vec![FieldSpec::new(2,"alias",Shape::Record(producer!(node))).optional(),FieldSpec::new(1,"children",Shape::List(Box::new(Shape::Record(producer!(twin))))),FieldSpec::new(0,"name",Shape::Text)]) }
fn hex(bytes: &[u8]) -> String { bytes.iter().map(|byte| format!("{byte:02x}")).collect() }
fn main() {
    for (name, canonical, hash) in [${cases.join(",")}] {
        let record = match name {
            "flat" => inline(vec![FieldSpec::new(2,"b",Shape::Text),FieldSpec::new(1,"a",Shape::Int)]),
            "nested" => nested(producer!(inner)),
            "nested-inner-field-changed" => nested(producer!(changed)),
            "recursive" => node(),
            _ => panic!("unexpected fixture {name}"),
        };
        let graph = schema::PackSchemaGraph::of(&record);
        assert_eq!(hex(&graph.canonical_bytes()), canonical, "{name} canonical bytes");
        assert_eq!(hex(&schema::schema_hash(&record)), hash, "{name} schema hash");
        println!("[DEBUG] {name} graph matches canonical bytes and hash");
    }
    assert_eq!(schema::PackSchemaGraph::of(&node()), schema::PackSchemaGraph::of(&twin()));
}
`);
const search = paths.flatMap(path => ["-L", `dependency=${join(path, "out")}`]);
const list = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (list.exitCode) process.exit(list.exitCode);
const rustup = JSON.parse(list.stdout.toString()).find((item: { id: string }) => item.id === "rustup")?.command;
const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust compiler unavailable");
const compiled = Bun.spawnSync([rustup, "run", toolchain, "rustc", "--edition=2024", "-Cdebuginfo=0", "-Cstrip=symbols", input, "-o", output, ...dependencies, ...search], { cwd: build, stdout: "inherit", stderr: "inherit" });
if (compiled.exitCode) process.exit(compiled.exitCode);
process.exit(Bun.spawnSync([output], { stdout: "inherit", stderr: "inherit" }).exitCode);
