"""🧪 Runs the actual archive maintenance predicate and selector against the portable wake cases."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ticket = Path(__file__).parent
ladder = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs").read_text(encoding="utf-8")
fixture = json.loads((ticket / "note-archive-maintenance-wake.json").read_text(encoding="utf-8"))


def definition(signature):
    start = ladder.index(signature)
    depth = 0
    for index, char in enumerate(ladder[ladder.index("{", start):], ladder.index("{", start)):
        if char == "{":
            depth += 1
        if char == "}":
            depth -= 1
            if depth == 0:
                return ladder[start:index + 1]
    raise ValueError(signature)


native = """
const DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS: usize = 16;
const ARTIFACT_LIVE_OUTPUT_SLOTS: usize = 1;
type ValueError = ();
struct RetainedCloneGrant { maximum_items: usize, maximum_copy_bytes: usize }
struct RetirementDemand { copy_bytes: usize }
mod semio_plugin_ownership {
    pub mod bounded_ownership {
        pub fn plugin_grant_funds(demand: crate::RetirementDemand, stock: crate::RetainedCloneGrant) -> bool {
            demand.copy_bytes <= stock.maximum_copy_bytes
        }
    }
}
struct Archive { terminal: bool, awaits_merge: bool }
impl Archive {
    fn terminal(&self) -> bool { self.terminal }
    fn awaits_merge(&self) -> bool { self.awaits_merge }
}
struct Registry(Option<Archive>);
impl Registry {
    fn entry(&self, index: usize) -> Option<(u64, &Archive)> {
        assert_eq!(index, 0);
        self.0.as_ref().map(|archive| (1, archive))
    }
}
struct Ingest { document_archive_loads: Registry }
struct Close { close_document_archive_cursor: usize }
struct Maintenance { maintenance_document_archive_cursor: usize }
struct Ladder { ingest: Ingest, close: Close, maintenance: Maintenance }
impl Ladder {
    fn document_archive_demands(&self, _closing: bool, _body: usize) -> Result<RetirementDemand, ValueError> {
        assert!(self.document_archive_target(false).is_some());
        Ok(RetirementDemand { copy_bytes: 64 })
    }
"""
native += definition("pub fn document_archive_maintenance_is_runnable")
native += definition("fn document_archive_target")
native += "\n}\nfn main() {\n"


def rust_bool(condition):
    return str(bool(condition)).lower()


for index, row in enumerate(fixture):
    terminal = rust_bool(row["state"] not in ["pending", "running"])
    awaiting = rust_bool(row["phase"] == "MergeParent" and not row["terminalTarget"])
    archive = f"Some(Archive {{ terminal: {terminal}, awaits_merge: {awaiting} }})" if row["live"] else "None"
    native += f"""
    let ladder = Ladder {{
        ingest: Ingest {{ document_archive_loads: Registry({archive}) }},
        close: Close {{ close_document_archive_cursor: 0 }},
        maintenance: Maintenance {{ maintenance_document_archive_cursor: 0 }},
    }};
    let stock = RetainedCloneGrant {{ maximum_items: {row["items"]}, maximum_copy_bytes: {64 if row["fits"] else 63} }};
    assert_eq!(ladder.document_archive_maintenance_is_runnable(stock).unwrap(), {rust_bool(row["wake"])}, "case {index}");
    """
native += f'println!("[DEBUG] archive maintenance wake matched {len(fixture)} Rust cases");\n}}'
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as directory:
    binary = Path(directory) / ("wake.exe" if os.name == "nt" else "wake")
    subprocess.run(["repo", "tools", "exec", "--light", "rustc", "--", "--edition=2021", "-o", str(binary), "-"], input=native, text=True, check=True)
    subprocess.run(["repo", "capacity", "run", "--memory=64", "--", str(binary)], timeout=45, check=True)
