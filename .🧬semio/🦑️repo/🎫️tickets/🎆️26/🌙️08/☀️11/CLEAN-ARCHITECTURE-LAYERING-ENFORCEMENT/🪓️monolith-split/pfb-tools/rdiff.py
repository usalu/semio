"""🔬️ Full-resolution rustc differential (see 📓️RESOLUTION-ORACLE.md) between a pre-edit snapshot and the live tree.

usage: rdiff.py <target> [--test] [--cfg feature=sut] [--show N]
targets: sim | visual-job | energy-test
Both trees are copied into the scratchpad (never written in the repo), the oracle patches are applied to both copies
(extern crates removed, external glob imports neutralised), rustc runs analysis only (-Zno-codegen) and the sorted
multisets of error messages are compared (file paths and line numbers stripped, because the split moves code).
"""
import collections, os, re, shutil, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ORIG = F + "/🗑️generated/pfb-orig"
SCR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/pfb/oracle"
SIM = "✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim"
VJ = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job"
SUBSET = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
TARGETS = {
    "sim": dict(orig="sim", live=SIM, root="🦀️.rs", crate="semio_s_plugin_energy_engine_sim", fixtures=None),
    "visual-job": dict(orig="visual-job", live=VJ, root="📦️packages/🦀️rust/🦀️.rs", crate="semio_s_artifact_fem_3d_editor_visual_job", fixtures=None),
    "energy-test": dict(orig="energy-test", live=SUBSET + "/🧪️tests/🏛️mutate-energy-model-1", root="🦀️.rs", crate="mutate_energy_model_1", fixtures=SUBSET + "/🧪️tests/🧫️fixtures"),
}
name = sys.argv[1]
test = "--test" in sys.argv
cfgs = [sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--cfg"]
show = int(sys.argv[sys.argv.index("--show") + 1]) if "--show" in sys.argv else 12
target = TARGETS[name]


def safe(path):
    assert os.path.realpath(path).startswith(os.path.realpath(SCR) + os.sep) or os.path.realpath(path) == os.path.realpath(SCR), path


def patch(root):
    for dirpath, _, names in os.walk(root):
        for item in names:
            if not item.endswith(".rs"):
                continue
            path = os.path.join(dirpath, item)
            safe(path)
            if os.path.islink(path):
                continue
            text = open(path, encoding="utf8").read()
            patched = re.sub(r"^\s*(pub\s+)?extern crate [^;]*;\s*$", "", text, flags=re.M)
            patched = re.sub(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(?!crate::|self::|super::)[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z0-9_{}, :*]*)?::\*;\s*$", r"\1const _: () = ();", patched, flags=re.M)
            if patched != text:
                open(path, "w", encoding="utf8").write(patched)



STUB = r"""
#[allow(dead_code, unused_variables)]
pub mod semio_repo_test_host {
    #[derive(Clone, Debug, PartialEq)]
    pub enum Json { Null, Bool(bool), Number(f64), String(String), Array(Vec<Json>), Object(Vec<(String, Json)>) }
    impl Json {
        pub fn get(&self, key: &str) -> Option<&Json> { None }
        pub fn array(&self, key: &str) -> Vec<Json> { Vec::new() }
        pub fn str(&self, key: &str) -> String { String::new() }
    }
    impl std::fmt::Display for Json {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("") }
    }
    pub fn parse_json(text: &str) -> Result<Json, String> { Ok(Json::Null) }
    pub struct Context;
    impl Context { pub fn input_bytes(&self, uri: &str) -> Result<Vec<u8>, String> { Ok(Vec::new()) } }
    pub struct Outcome;
    impl Outcome { pub fn with_raw(raw: Vec<u8>, projection: Json) -> Outcome { Outcome } }
    pub struct Adapter;
    impl Adapter {
        pub fn new(implementation: &str) -> Adapter { Adapter }
        pub fn oracle<F: Fn(&Context) -> Result<Outcome, String> + 'static>(self, scenario: &str, handler: F) -> Adapter { self }
        pub fn subject<F: Fn(&Context) -> Result<Outcome, String> + 'static>(self, scenario: &str, handler: F) -> Adapter { self }
    }
    pub mod law {
        use super::Json;
        pub fn divergence(left: &Json, right: &Json) -> Option<String> { None }
        pub fn mutation_is_observable(kind: &str, applied: &Json, base: &Json, unobservable: &[&str]) -> Result<(), String> { Ok(()) }
        pub fn inverse_restores(kind: &str, restored: &Json, base: &Json) -> Result<(), String> { Ok(()) }
        pub fn round_trip_preserves(left: &Json, right: &Json) -> Result<(), String> { Ok(()) }
        pub fn carrier_is_exact(left: &[u8], right: &[u8]) -> Result<(), String> { Ok(()) }
    }
}
#[allow(dead_code, unused_variables)]
pub mod energy_stub {
    pub fn energy_model_mutation_report_json(before: &str, mutation: &str, after: &str) -> Result<String, String> { Ok(String::new()) }
    pub fn energy_model_identity_report_json(text: &str) -> Result<String, String> { Ok(String::new()) }
}
"""


def stubify(root):
    for dirpath, _, names in os.walk(root):
        for item in names:
            if not item.endswith(".rs"):
                continue
            path = os.path.join(dirpath, item)
            safe(path)
            text = open(path, encoding="utf8").read()
            text = re.sub(r"semio_s_artifact_energy_model::standards::v1::subsets::any::io::text::(?:mutations|snapshot)::", "crate::energy_stub::", text)
            text = re.sub(r"(?<![\w:])semio_repo_test_host::", "crate::semio_repo_test_host::", text)
            if "--without-expect" in sys.argv:
                text = text.replace('.expect("valid retained mutation inverse fixture")', "")
            if dirpath == root and item == "🦀️.rs":
                text += STUB
            open(path, "w", encoding="utf8").write(text)


def build(label, source):
    destination = os.path.join(SCR, label)
    safe(destination)
    os.makedirs(destination)
    subprocess.check_call(["cp", "-R", source + "/.", destination + "/"])
    if target["fixtures"]:
        holder = os.path.join(SCR, label + "-holder")
        safe(holder)
        os.makedirs(holder)
        shutil.move(destination, os.path.join(holder, "case"))
        os.symlink(os.path.join(ROOT, target["fixtures"]), os.path.join(holder, "🧫️fixtures"))
        patch(os.path.join(holder, "case"))
        if "--stub" in sys.argv:
            stubify(os.path.join(holder, "case"))
        return os.path.join(holder, "case")
    patch(destination)
    return destination


shutil.rmtree(SCR, ignore_errors=True)
os.makedirs(SCR)
trees = {"base": build("base", os.path.join(ORIG, target["orig"])), "cand": build("cand", os.path.join(ROOT, target["live"]))}
results = {}
for label, tree in trees.items():
    command = ["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", target["crate"], "-Zno-codegen", "--out-dir", os.path.join(SCR, "out-" + label), "--error-format=short"]
    if test:
        command.append("--test")
    for cfg in cfgs:
        key, _, value = cfg.partition("=")
        command += ["--cfg", f'{key}="{value}"']
    command.append(target["root"])
    run = subprocess.run(["nice", "-n", "10"] + command, cwd=tree, capture_output=True, text=True, timeout=1500)
    results[label] = run.stderr


def normalise(text):
    counter = collections.Counter()
    for line in text.splitlines():
        match = re.match(r".*?:\d+:\d+: (error(?:\[E\d+\])?: .*)", line)
        if match:
            counter[match.group(1)[:200]] += 1
        elif line.startswith("error") and not line.startswith("error: aborting"):
            counter[line[:200]] += 1
    return counter


def warnings(text):
    counter = collections.Counter()
    for line in text.splitlines():
        match = re.match(r".*?:\d+:\d+: (warning: .*)", line)
        if match:
            counter[match.group(1)[:200]] += 1
    return counter


new_warnings = warnings(results["cand"]) - warnings(results["base"])
base, cand = normalise(results["base"]), normalise(results["cand"])
codes = collections.Counter(re.findall(r"error\[(E\d+)\]", results["cand"]))
extra, gone = cand - base, base - cand
print(f"{name}{' --test' if test else ''}{' ' + ' '.join(cfgs) if cfgs else ''}: base errors {sum(base.values())}, cand errors {sum(cand.values())}; codes(cand) {dict(codes.most_common(6))}")
print(f"  new in cand: {sum(extra.values())} ({len(extra)} distinct); gone from base: {sum(gone.values())} ({len(gone)} distinct)")
for key, value in list(extra.items())[:show]:
    print("   +", value, key)
for key, value in list(gone.items())[:show]:
    print("   -", value, key)
open(F + f"/🗑️generated/pfb-rdiff-{name}{'-stub' if '--stub' in sys.argv else ''}{'-test' if test else ''}{'-' + '-'.join(c.replace('=', '_') for c in cfgs) if cfgs else ''}.log", "w", encoding="utf8").write(results["cand"][:200000])
shutil.rmtree(SCR, ignore_errors=True)
print(f"  new warnings in cand: {sum(new_warnings.values())}")
for key, value in new_warnings.items():
    print("   w", value, key)
sys.exit(1 if extra or new_warnings else 0)
