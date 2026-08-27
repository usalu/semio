import { mkdirSync, mkdtempSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { spawnSync } from "node:child_process";

//#region 🧪️SourceSpanPreflight
const root = mkdtempSync(join(import.meta.dir, "🧫️run-"));
const leaf = "✏️s/🧪️probe/🧬️mutations/➕️insert-page/🦀️.rs";
const macroSource = join(root, "🦀️macro.rs");
const macroLibrary = join(root, `${process.platform === "win32" ? "" : "lib"}owner_probe.${process.platform === "win32" ? "dll" : process.platform === "darwin" ? "dylib" : "so"}`);
const entry = join(root, "🦀️main.rs");
const run = (name: string, command: string, args: string[]): string => {
  const result = spawnSync(command, args, { cwd: root, encoding: "utf8", timeout: 30_000 });
  writeFileSync(join(root, `🧪️${name}.log`), `${result.stdout}\n${result.stderr}`);
  if (result.status !== 0) throw new Error(`${name} failed: ${result.status}; ${result.error ?? result.stderr}`);
  return result.stdout;
};
mkdirSync(dirname(join(root, leaf)), { recursive: true });
writeFileSync(macroSource, `extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
#[proc_macro_derive(OwnerProbe)]
pub fn owner_probe(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let name = loop {
        match tokens.next() {
            Some(TokenTree::Ident(value)) if value.to_string() == "struct" => break tokens.next().expect("name"),
            Some(_) => (),
            None => panic!("struct missing"),
        }
    };
    let span = name.span();
    let raw = span.local_file().expect("local source");
    let local = raw.canonicalize().expect("source exists");
    format!("impl {} {{ pub const LOCAL: &'static str = {:?}; pub const REPORTED: &'static str = {:?}; pub const RAW: &'static str = {:?}; }}", name, local.to_str().expect("utf8"), span.file(), raw.to_str().expect("utf8")).parse().expect("generated declaration")
}
`);
writeFileSync(join(root, leaf), "#[derive(owner_probe::OwnerProbe)]\npub struct Leaf;\n");
writeFileSync(entry, `#[path = ${JSON.stringify(leaf)}] mod leaf;\nfn main() { println!("{}\\n{}\\n{}", leaf::Leaf::LOCAL, leaf::Leaf::REPORTED, leaf::Leaf::RAW); }\n`);
const parentEntry = join(root, "consumer", "main.rs");
mkdirSync(dirname(parentEntry), { recursive: true });
writeFileSync(parentEntry, `#[path = ${JSON.stringify(`../${leaf}`)}] mod leaf;\nfn main() { println!("{}\\n{}\\n{}", leaf::Leaf::LOCAL, leaf::Leaf::REPORTED, leaf::Leaf::RAW); }\n`);
run("macro", "rustc", ["--edition=2021", "--crate-name", "owner_probe", "--crate-type", "proc-macro", macroSource, "-o", macroLibrary]);
let failures = 0;
for (const mode of ["plain", "remapped", "relative-parent", "relative-parent-remapped"]) {
  const binary = join(root, `probe-${mode}${process.platform === "win32" ? ".exe" : ""}`);
  const remapped = mode.endsWith("remapped"), parent = mode.startsWith("relative-parent");
  const extra = remapped ? ["--remap-path-prefix", `${root}=/virtual/mutation-probe`] : [];
  run(`compile-${mode}`, "rustc", ["--edition=2021", "--crate-name", "owner_probe_main", parent ? relative(root, parentEntry) : entry, "--extern", `owner_probe=${macroLibrary}`, ...extra, "-o", binary]);
  const [local, reported, raw] = run(`runtime-${mode}`, binary, []).trim().split("\n");
  const expectedLocal = realpathSync(join(root, leaf));
  const passed = local === expectedLocal && (parent || !remapped || reported === `/virtual/mutation-probe/${leaf}`);
  console.log(`[DEBUG] ${JSON.stringify({ mode, passed, rawLocalFile: raw, reportedFile: reported, localMatchesActualSource: local === expectedLocal, diagnosticWasRemapped: reported !== raw, sourceRelativeToFixture: relative(root, local!) })}`);
  if (!passed) failures += 1;
}
console.log(`[DEBUG] retained fixture root: ${root}`);
console.log(`[DEBUG] failed regression groups: ${failures}`);
process.exitCode = failures === 0 ? 0 : 1;
//#endregion 🧪️SourceSpanPreflight
