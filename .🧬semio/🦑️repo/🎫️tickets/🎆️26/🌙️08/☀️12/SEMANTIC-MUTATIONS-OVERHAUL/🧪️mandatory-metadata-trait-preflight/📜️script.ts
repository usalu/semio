import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

//#region 🧪️MandatoryMetadataTraitPreflight
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️vectors.json"), "utf8"));
const root = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const common = `
#[derive(Clone, Copy)]
struct Descriptor { kind: &'static str }
trait Leaf { const DESCRIPTOR: Descriptor; }
trait Mutation<P> {
    const DESCRIPTORS: &'static [Descriptor];
    fn descriptor(&self) -> &'static Descriptor;
}
struct Insert<T>(T);
struct Remove<T>(T);
impl<T> Leaf for Insert<T> { const DESCRIPTOR: Descriptor = Descriptor { kind: "insert-item" }; }
impl<T> Leaf for Remove<T> { const DESCRIPTOR: Descriptor = Descriptor { kind: "remove-item" }; }
enum Operations<T> { Insert(Insert<T>), Remove(Remove<T>) }
`;
const implementations: Record<string, string> = {
  "generic-static-roster": `${common}
impl<T> Mutation<Vec<T>> for Operations<T> {
    const DESCRIPTORS: &'static [Descriptor] = &[<Insert<T> as Leaf>::DESCRIPTOR, <Remove<T> as Leaf>::DESCRIPTOR];
    fn descriptor(&self) -> &'static Descriptor { match self { Self::Insert(_) => &<Insert<T> as Leaf>::DESCRIPTOR, Self::Remove(_) => &<Remove<T> as Leaf>::DESCRIPTOR } }
}
fn report<'a>(value: &'a str) -> String {
    let operation = Operations::Insert(Insert(value));
    let removed = Operations::Remove(Remove(value));
    assert_eq!(operation.descriptor().kind, "insert-item");
    assert_eq!(removed.descriptor().kind, "remove-item");
    <Operations<&'a str> as Mutation<Vec<&'a str>>>::DESCRIPTORS.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>().join(",")
}
fn main() { let owned = String::from("borrowed"); println!("{};{}", report(&owned), <Operations<u32> as Mutation<Vec<u32>>>::DESCRIPTORS.iter().map(|descriptor| descriptor.kind).collect::<Vec<_>>().join(",")); }
`,
  "required-roster-omission": `${common}
impl<T> Mutation<Vec<T>> for Operations<T> { fn descriptor(&self) -> &'static Descriptor { &<Insert<T> as Leaf>::DESCRIPTOR } }
fn main() {}
`,
  "required-instance-descriptor-omission": `${common}
impl<T> Mutation<Vec<T>> for Operations<T> { const DESCRIPTORS: &'static [Descriptor] = &[<Insert<T> as Leaf>::DESCRIPTOR]; }
fn main() {}
`,
  "supertrait-qualified-constant": `${common}
trait Kind: Leaf {}
impl<T> Kind for Insert<T> {}
fn main() { println!("{}", <Insert<u32> as Kind>::DESCRIPTOR.kind); }
`,
  "manual-provenance-is-forgeable": `
struct Provenance { owner: &'static str }
trait Leaf { const PROVENANCE: Provenance; }
struct Forged;
impl Leaf for Forged { const PROVENANCE: Provenance = Provenance { owner: "claimed/🧬️mutations/insert-item" }; }
fn main() { println!("{}", Forged::PROVENANCE.owner); }
`,
};
const results = [];
for (const vector of fixture.cases) {
  const directory = join(root, vector.name);
  mkdirSync(directory);
  const source = join(directory, "🦀️.rs");
  const binary = join(directory, `probe${process.platform === "win32" ? ".exe" : ""}`);
  writeFileSync(source, implementations[vector.name]);
  const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name", "metadata_probe", source, "-o", binary], { encoding: "utf8", timeout: 60_000 });
  writeFileSync(join(directory, "🧪️compiler.log"), `${compiled.stdout}\n${compiled.stderr}\n${compiled.error ?? ""}`);
  let passed = compiled.status === 0 ? vector.compiles : !vector.compiles && !!vector.diagnostic && compiled.stderr.includes(vector.diagnostic);
  let stdout: string | undefined;
  if (compiled.status === 0) {
    const runtime = spawnSync(binary, [], { encoding: "utf8", timeout: 10_000 });
    writeFileSync(join(directory, "🧪️runtime.log"), `${runtime.stdout}\n${runtime.stderr}\n${runtime.error ?? ""}`);
    stdout = runtime.stdout.trim();
    passed &&= runtime.status === 0 && stdout === vector.stdout;
  }
  const result = { name: vector.name, compiles: compiled.status === 0, status: compiled.status, passed, stdout };
  results.push(result);
  console.log(`[DEBUG] ${JSON.stringify(result)}`);
}
writeFileSync(join(root, "🔣️results.json"), JSON.stringify({ prototypeOnly: true, results }, null, 2));
console.log(`[DEBUG] trait prototype cases=${results.length} failures=${results.filter((result) => !result.passed).length} artifacts=${root}`);
process.exitCode = results.every((result) => result.passed) ? 0 : 1;
//#endregion 🧪️MandatoryMetadataTraitPreflight
