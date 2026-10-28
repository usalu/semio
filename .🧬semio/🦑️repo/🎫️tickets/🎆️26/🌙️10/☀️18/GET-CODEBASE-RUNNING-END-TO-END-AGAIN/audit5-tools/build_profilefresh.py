"""🎚 [audit5] Builds the staged batch `profilefresh` from the CURRENT live bytes with anchored edits (exit 1 = an anchor is lost): the dashboard installation digest also covers the Cargo profile policy files its compile consumes, and the compiled-sources law proves it.
usage: python3 -I build_profilefresh.py [--root DIR]   (default: the live checkout above this ticket)"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(C, *[".."] * 7))
OUT = os.path.join(HERE, "staged", "profilefresh")
MOD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"
SCHEMA = MOD + "📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🧬️schema/🔣️.json"
LAW = MOD + "🎛️dashboard/🧪️tests/🧊️execution/🧾️compiled/🟦️.ts"
README = MOD + "🎛️dashboard/README.md"
POLICY = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/📋️constraints/🔣️.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json"]
TEST = '''
test("an installation is stale once the Cargo profile policy its compile consumed changes",async()=>{
 const {renderCargoProfileConfigurationV1}=await import("../../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts");
 const target=({"darwin:arm64":"aarch64-apple-darwin","darwin:x64":"x86_64-apple-darwin","linux:arm64":"aarch64-unknown-linux-gnu","linux:x64":"x86_64-unknown-linux-gnu","win32:x64":"x86_64-pc-windows-msvc"} as Record<string,string>)[process.platform+":"+process.arch]!;
 const policy=renderCargoProfileConfigurationV1({version:1,mode:mode(),target,producer:"artifact",operation:"build"},"Cargo.toml","repository").plan.sources.map(row=>row.path);
 expect(policy.length).toBeGreaterThan(0);expect(installationSchema.definitions.SourceInputProtocol.const.rootFiles).toEqual(expect.arrayContaining(policy));
 for(const[index,path]of policy.entries()){
  const tree=await installedTree("q"+index),file=join(tree.root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",tree.selected,"installed.json"),capture=()=>captureDashboardSources(join(tree.root,fixture.package),tree.root);
  for(const each of policy){mkdirSync(dirname(join(tree.root,each)),{recursive:true});writeFileSync(join(tree.root,each),"{}");}
  writeFileSync(file,JSON.stringify({...tree.record,sources:await capture()}));
  expect(currentDashboardV1(tree.root,tree.selected,tree.root,await capture())?.executable,path).toBe(tree.executable);
  mutate(tree.root,"write",path);
  expect(currentDashboardV1(tree.root,tree.selected,tree.root,await capture()),path).toBeUndefined();
  expect(await staleDashboard(tree.root,tree.selected),path).toStartWith("its sources changed since installation");
 }
});
'''
def edit(text, old, new, name):
    if text.count(old) != 1: sys.exit(f"anchor lost: {name} ({text.count(old)})")
    return text.replace(old, new)
def schema(text):
    old = '"rootFiles": [\n          "rust-toolchain.toml",\n          ".cargo/config.toml"\n        ],'
    new = '"rootFiles": [\n          "rust-toolchain.toml",\n          ".cargo/config.toml",\n' + "".join(f'          "{p}"' + (",\n" if i < len(POLICY) - 1 else "\n") for i, p in enumerate(POLICY)) + '        ],'
    return edit(text, old, new, "schema rootFiles")
def law(text):
    if not text.endswith("\n"): text += "\n"
    if "Cargo profile policy its compile consumed" in text: sys.exit("law already present")
    return text + TEST.lstrip("\n")
def readme(text):
    old = "workspace `Cargo.lock`, and root `rust-toolchain.toml` and `.cargo/config.toml`. Inherited"
    new = "workspace `Cargo.lock`, root `rust-toolchain.toml` and `.cargo/config.toml`, and the Cargo profile policy (profile schema and\npackage constraints) that renders the compile's profile. Inherited"
    return edit(text, old, new, "readme digest")
def sha(b): return hashlib.sha256(b).hexdigest()
def main():
    shutil.rmtree(OUT, ignore_errors=True)
    for d in ("files", "base"): os.makedirs(os.path.join(OUT, d))
    manifest = {}
    for i, (rel, fn) in enumerate([(SCHEMA, schema), (LAW, law), (README, readme)]):
        base = open(os.path.join(ROOT, rel), "rb").read()
        new = fn(base.decode("utf8")).encode("utf8")
        name = f"f{i:02d}{os.path.splitext(rel)[1]}"
        open(os.path.join(OUT, "base", name), "wb").write(base)
        open(os.path.join(OUT, "files", name), "wb").write(new)
        manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    first = template.index('"""') + 3
    head = template[:first] + "🎚 Staged batch `profilefresh` (lane audit5): the dashboard installation digest covers the Cargo profile policy its compile consumes." + template[template.index("\n", first):]
    open(os.path.join(OUT, "stage.py"), "w", encoding="utf8").write(head)
    print(f"built {OUT}: {len(manifest)} files from {ROOT}")
main()
