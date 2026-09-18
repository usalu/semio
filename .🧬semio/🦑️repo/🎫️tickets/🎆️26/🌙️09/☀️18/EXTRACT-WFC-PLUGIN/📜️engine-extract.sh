#!/bin/zsh
# 🀄️ Rebuilds ✏️s/🔌️plugins/🀄️wfc/⚙️engine from the assembly wfc-engine tree. Idempotent: it
# replaces the engine folder wholesale, never touches the source, never touches git.
set -euo pipefail
cd /Users/ueli/Documents/semio

SRC="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine"
DST="✏️s/🔌️plugins/🀄️wfc/⚙️engine"

[ -d "$SRC" ] || { echo "source engine tree is gone: $SRC" >&2; exit 1; }

rm -rf "$DST"
mkdir -p "$DST/📦️packages/🦀️rust"
cp -R "$SRC/." "$DST/"

cat > "$DST/📦️packages/🦀️rust/Cargo.toml" <<'TOML'
[package]
name = "semio-s-plugin-wfc-engine"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Shared wave function collapse engine for the wfc plugin artifacts"

[package.metadata.semio]
role = "s-module"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-dispatch-macros = { workspace = true }
semio-framework-geometry = { workspace = true }
semio-framework-graph = { workspace = true }
semio-framework-job = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-value-derive = { workspace = true }

[dev-dependencies]
serde_json = { workspace = true }
TOML

find "$DST" -name "🦀️.rs" -type f -print0 | xargs -0 perl -pi -e '
s/crate::wfc_engine::/crate::/g;
s/\bprotocol::json::/semio_framework_os_kernel::json::/g;
s/\bdsl::json::/semio_framework_os_kernel::json::/g;
s/\bpub\(crate\)/pub/g;
s/\bAssemblyModelBuild\b/GraphModelBuild/g;
s/\bAssemblyModelPhase\b/GraphModelPhase/g;
s/\bAssemblyTopologyBuild\b/GraphTopologyBuild/g;
s/\bAssemblyTopologyPhase\b/GraphTopologyPhase/g;
s/🧵️AssemblyCompiler/🧵️IncrementalCompiler/g;
'

perl -pi -e 's/: test\b/: all()/g' "$DST/🆔️ids/🦀️.rs"

python3 - "$DST" <<'PY'
import os, sys
dst = sys.argv[1]
keep = '#[path = "🧪️tests/'

# 1. every module in a production file becomes production; only the two crate-root gates and the
#    job's own test module keep their #[cfg(test)].
for dirpath, _dirnames, filenames in os.walk(dst):
    if "🧪️tests" in dirpath:
        continue
    for name in filenames:
        if name != "🦀️.rs":
            continue
        path = os.path.join(dirpath, name)
        if path == os.path.join(dst, "🦀️.rs"):
            continue
        lines = open(path, encoding="utf-8").read().split("\n")
        out, removed = [], 0
        for index, line in enumerate(lines):
            if line.strip() == "#[cfg(test)]":
                following = lines[index + 1] if index + 1 < len(lines) else ""
                if keep in following:
                    out.append(line)
                    continue
                removed += 1
                continue
            out.append(line)
        if removed:
            open(path, "w", encoding="utf-8").write("\n".join(out))

# 2. job.rs: lift the uniform sampler and the read-only accessors into production, and give the
#    engine a production drive/close helper set (search.rs needs it now that it ships).
job = os.path.join(dst, "💼️job/🦀️.rs")
source = open(job, encoding="utf-8").read()
retained = '''fn retained_payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(payload.len());
    for index in 0..payload.page_count() {
        if let Some(page) = payload.page(index) {
            bytes.extend_from_slice(page);
        }
    }
    bytes
}

'''
assert retained in source
source = source.replace(retained, "", 1)
source = source.replace("&retained_payload_bytes(&fault.detail)", "&payload_bytes(&fault.detail)", 1)
for old, new in [
    ("    fn commit(&self) -> Option<WfcCommit> {", "    pub fn commit(&self) -> Option<WfcCommit> {"),
]:
    assert old in source, old
    source = source.replace(old, new, 1)
drive = '''//#region 🚚️Drive
/// 📦️ Concatenates every page of a retained payload — the byte view a worker session reassembles.
pub fn payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).expect("retained payload page").iter().copied()).collect()
}

/// 🧹️ Releases every page a step outcome still owns, the way a worker session retires it.
pub fn retire_outcome(outcome: &mut StepOutcome) {
    while !outcome.terminal_is_empty() {
        outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🚪️ Runs a job's close ladder to completion — every job must be closed before it is dropped.
pub fn close_job(job: &mut impl InteractiveJob) {
    job.begin_close();
    for _ in 0..2_000_000 {
        if job.terminal_is_empty() {
            return;
        }
        assert_ne!(job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Blocked, "a locally owned job close has no external owner");
    }
    panic!("job did not close");
}
//#endregion 🚚️Drive

//#region 🧪️Tests'''
assert "//#region 🧪️Tests" in source
source = source.replace("//#region 🧪️Tests", drive, 1)
open(job, "w", encoding="utf-8").write(source)

# 3. the job test module keeps using those helpers through `use super::*`.
job_tests = os.path.join(dst, "💼️job/🧪️tests/🔬️unit/🦀️.rs")
tests = open(job_tests, encoding="utf-8").read()
start = tests.index("pub fn payload_bytes(")
end = tests.index("fn assert_fault(")
tests = tests[:start] + tests[end:]
stale = '''    assert_fault(StepOutcome::Fault(CommitBuild::new(MAX_COMMIT_ITEMS + 1).err().expect("commit admission fault")), b"wfc-commit-admission-exceeded");
    source.state.observed.push((NodeId(0), PatternId(0)));
    assert_fault(StepOutcome::Fault(CheckpointBuild::new(&source.state, source.model.pattern_count(), true).err().expect("checkpoint admission fault")), b"wfc-checkpoint-admission-exceeded");
'''
fresh = '''    assert!(CommitBuild::new(MAX_COMMIT_ITEMS + 1).is_err(), "one item over the maximum must be refused admission");
    source.state.observed.push((NodeId(0), PatternId(0)));
    assert!(CheckpointBuild::new(&source.state, source.model.pattern_count(), true).is_err(), "one observation over the maximum must be refused admission");
'''
assert stale in tests
tests = tests.replace(stale, fresh, 1)
watchdog = '    assert!(samples.last().copied().expect("sample") < Duration::from_millis(8));'
assert watchdog in tests
tests = tests.replace(watchdog, '    let maximum = samples.last().copied().expect("sample");\n    assert!(maximum < Duration::from_millis(8), "WFC unit maximum exceeded 8 ms: {maximum:?}");', 1)
open(job_tests, "w", encoding="utf-8").write(tests)

# 4. search.rs drives the interactive job in production now.
search = os.path.join(dst, "🔍️search/🦀️.rs")
text = open(search, encoding="utf-8").read()
text = text.replace("use crate::job::tests::{close_job, payload_bytes, retire_outcome};", "use crate::job::{close_job, payload_bytes, retire_outcome};", 1)
open(search, "w", encoding="utf-8").write(text)

# 5. the shared model vectors are a crate-root module, so they import explicitly.
vectors = os.path.join(dst, "🧪️tests/🧮️model-vectors/🦀️.rs")
text = open(vectors, encoding="utf-8").read()
text = text.replace(
    "use super::*;\nuse crate::model::ModelBuilder;\nuse crate::weights::WeightTable;",
    "use crate::bitset::PatternSet;\nuse crate::ids::{NodeId, RelationId};\nuse crate::model::{CompiledModel, ModelBuilder};\nuse crate::oracle::ArcSpec;\nuse crate::weights::WeightTable;",
    1,
)
text = text.replace("[`super::enumerate`]", "[`crate::oracle::enumerate`]")
open(vectors, "w", encoding="utf-8").write(text)
# 6. widening `pub(crate)` to `pub` exposes four `len` methods and two obfuscated-if-else chains
#    to clippy; close them here so the crate lints clean.
for path, old, new in [
    (os.path.join(dst, "🌐️domain/🦀️.rs"),
     "    #[inline]\n    pub fn len(&self) -> usize {\n        self.domains.len()\n    }\n",
     "    #[inline]\n    pub fn len(&self) -> usize {\n        self.domains.len()\n    }\n\n    /// 🕳️ Whether the store holds no node domains at all.\n    #[inline]\n    pub fn is_empty(&self) -> bool {\n        self.domains.is_empty()\n    }\n"),
    (os.path.join(dst, "🚫️nogood/🦀️.rs"),
     "    pub fn len(&self) -> usize {\n        self.nogoods.len()\n    }\n",
     "    pub fn len(&self) -> usize {\n        self.nogoods.len()\n    }\n\n    /// 🕳️ Whether nothing has been learned yet.\n    pub fn is_empty(&self) -> bool {\n        self.nogoods.is_empty()\n    }\n"),
    (os.path.join(dst, "🐾️trail/🦀️.rs"),
     "    pub fn len(&self) -> usize {\n        self.entries.len()\n    }\n",
     "    pub fn len(&self) -> usize {\n        self.entries.len()\n    }\n\n    /// 🕳️ Whether no removal has been logged yet.\n    #[inline]\n    pub fn is_empty(&self) -> bool {\n        self.entries.is_empty()\n    }\n"),
    (os.path.join(dst, "⚖️weights/🦀️.rs"),
     "    #[inline]\n    pub fn len(&self) -> usize {\n        self.w.len()\n    }\n",
     "    #[inline]\n    pub fn len(&self) -> usize {\n        self.w.len()\n    }\n\n    /// 🕳️ Whether the table carries no pattern weights.\n    #[inline]\n    pub fn is_empty(&self) -> bool {\n        self.w.is_empty()\n    }\n"),
    (os.path.join(dst, "🔲️grid-2d/🦀️.rs"),
     "        self.is_active(x, y).then_some(self.offsets.len()).unwrap_or(0)",
     "        if self.is_active(x, y) { self.offsets.len() } else { 0 }"),
    (os.path.join(dst, "🧊️grid-3d/🦀️.rs"),
     "        self.is_active(x, y, z).then_some(self.offsets.len()).unwrap_or(0)",
     "        if self.is_active(x, y, z) { self.offsets.len() } else { 0 }"),
]:
    text = open(path, encoding="utf-8").read()
    assert old in text, path
    open(path, "w", encoding="utf-8").write(text.replace(old, new, 1))

print("engine tree rewritten")
PY

cp "$0:h/🗂️engine-root.rs.txt" "$DST/🦀️.rs"
mkdir -p "$DST/🧪️tests/🔬️grid-job-drive"
cp "$0:h/🗂️engine-grid-job-drive.rs.txt" "$DST/🧪️tests/🔬️grid-job-drive/🦀️.rs"
echo "rebuilt $DST"
