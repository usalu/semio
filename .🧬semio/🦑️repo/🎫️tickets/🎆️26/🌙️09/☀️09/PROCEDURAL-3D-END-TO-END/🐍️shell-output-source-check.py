
from pathlib import Path
import json
from tempfile import TemporaryDirectory
import subprocess

e = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep")
owner = (e / "〰️analytic-curves/🦀️.rs").read_text()
body = (e / "〰️analytic-curves/🧾️topology-output/🦀️.rs").read_text()
fixture = json.loads((e / "🧪️tests/🔬️unit/🔣️shell-selection.json").read_text())
prelude = "\nuse std::{cell::Cell, collections::BTreeMap, marker::PhantomData, mem::{take, size_of}};\ntype Dictionary = BTreeMap<String, Value>;\n#[derive(Debug)]\nenum Atom { String(String) }\n#[derive(Debug)]\nenum Value { Atom(Atom), Dictionary(Dictionary) }\n#[derive(Default, Clone, Copy)]\nstruct Progress { copied_items: usize, copied_bytes: usize, retained_capacity_bytes: usize }\nimpl Progress {\n    fn fits(self, grant: Grant) -> bool { self.copied_items <= 1 && self.copied_bytes <= grant.maximum_copy_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes }\n}\n#[derive(Default)]\nstruct RetirementDemand { copy_bytes: usize, capacity_bytes: usize, depth: usize, spare: usize }\n#[derive(Clone, Copy)]\nstruct Grant { maximum_copy_bytes: usize, maximum_capacity_bytes: usize }\nfn retained_clone_grant_admits(grant: Grant, demand: RetirementDemand) -> bool {\n    demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.depth <= 100\n}\nstruct RetainedAllocationEffect<'a> { remaining: bool, grant: Grant, lifetime: PhantomData<&'a ()> }\nimpl RetainedAllocationEffect<'_> {\n    fn grant(&self) -> Grant { self.grant }\n    fn performed(&mut self, units: usize) -> Result<bool, ValueError> { assert_eq!(units, 1); Ok(std::mem::replace(&mut self.remaining, false)) }\n}\n#[derive(Debug)]\nstruct ValueError;\nenum ValueRefusalKind { AllocationFailed }\nimpl ValueError {\n    fn literal(_: ValueRefusalKind, _: &str) -> Self { Self }\n    fn into_value_error(self) -> Self { self }\n}\nstruct Step(Progress);\nimpl Step { fn progress(self) -> Progress { self.0 } }\nenum RetainedWorkPoll<T> { Pending { }, Complete(Outcome<T>) }\nstruct Outcome<T> { result: Result<T, ValueError> }\nstruct BudgetedOwnedEntry { data: Option<(Dictionary, String, Value)>, closed: bool }\nimpl BudgetedOwnedEntry {\n    fn new(data: Dictionary, key: String, value: Value) -> Self { Self { data: Some((data, key, value)), closed: false } }\n    fn next_close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand::default()) }\n    fn next_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand::default()) }\n    fn step_progress(&self) -> Progress { item() }\n    fn step(&mut self, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<Option<Dictionary>> {\n        assert!(effect.performed(1).unwrap());\n        let (mut data, key, value) = self.data.take().unwrap();\n        data.insert(key, value);\n        RetainedWorkPoll::Complete(Outcome { result: Ok(Some(data)) })\n    }\n    fn close_step(&mut self, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<Step> {\n        assert!(effect.performed(1).unwrap());\n        self.closed = true;\n        RetainedWorkPoll::Complete(Outcome { result: Ok(Step(item())) })\n    }\n    fn terminal_is_empty(&self) -> bool { self.closed }\n}\n#[derive(Clone, Copy)] enum TopologyList { Shells, Solids }\nstruct TopologySelection(Vec<u64>, TopologyList);\nimpl TopologySelection { fn count(&self) -> usize { self.0.len() } fn kind(&self) -> TopologyList { self.1 } }\nstruct TopologyAlias([u8; 64]);\nimpl TopologyAlias { fn handle(&self)->&str { std::str::from_utf8(&self.0).unwrap() } }\n#[derive(Default)]\nstruct Session { live: Cell<usize>, staged: Cell<bool> }\nimpl Session {\n    fn close_staged_geometry_claim(&self, _: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {\n        assert!(effect.performed(1)?);\n        assert!(self.staged.replace(false));\n        Ok(item())\n    }\n    fn topology_member_depth(&self, _: &TopologySelection, _: usize) -> Result<usize, ValueError> { Ok(1) }\n    fn topology_member(&self, selection: &TopologySelection, index: usize) -> Result<TopologyAlias, ValueError> {\n        let handle = format!(\"{:064x}\", selection.0[index]);\n        Ok(TopologyAlias(handle.as_bytes().try_into().unwrap()))\n    }\n    fn topology_alias_publication_demand(&self, _: &TopologyAlias, _: bool, _: usize) -> Result<RetirementDemand, ValueError> { Ok(RetirementDemand::default()) }\n    fn step_topology_alias_publication(&self, alias: &TopologyAlias, live: &mut String, claim: &mut String, staged: &mut bool, effect: &mut RetainedAllocationEffect<'_>) -> Result<(bool, Progress), ValueError> {\n        assert!(effect.performed(1)?);\n        assert_eq!(live, alias.handle());\n        if !*staged {\n            assert_eq!(claim, alias.handle());\n            assert!(!self.staged.replace(true));\n            *staged = true;\n            return Ok((false, item()));\n        }\n        assert!(self.staged.replace(false));\n        *staged = false;\n        self.live.set(self.live.get() + 1);\n        Ok((true, item()))\n    }\n}\n"
prelude += """
struct NativeTopologyReach(Vec<u64>);
#[derive(Clone, Copy)]
enum ReachedTopology { Edge }
impl ReachedTopology { fn channel(self) -> &'static str { "edge" } }
mod deconstruct {
    pub struct LabelSelection(pub Vec<usize>);
    impl LabelSelection {
        pub fn len(&self) -> usize { self.0.len() }
        pub fn depth(&self, _: usize) -> Result<usize, super::ValueError> { Ok(1) }
        pub fn member(&self, rank: usize) -> Result<usize, super::ValueError> { Ok(self.0[rank]) }
    }
}
impl Session {
    fn reached_topology_depth(&self, _: &NativeTopologyReach, _: ReachedTopology, _: usize) -> Result<usize, ValueError> { Ok(1) }
    fn reached_topology_alias(&self, reach: &NativeTopologyReach, _: ReachedTopology, rank: usize) -> Result<TopologyAlias, ValueError> {
        let handle = format!("{:064x}", reach.0[rank]);
        Ok(TopologyAlias(handle.as_bytes().try_into().unwrap()))
    }
}
"""
finish = "\nfn effect(grant: Grant) -> RetainedAllocationEffect<'static> {\n    RetainedAllocationEffect { remaining: true, grant, lifetime: PhantomData }\n}\nfn list(value: &Value) -> &Dictionary {\n    let Value::Dictionary(dictionary) = value else { panic!(\"dictionary required\") };\n    dictionary\n}\nfn text(value: &Value) -> &str {\n    let Value::Atom(Atom::String(text)) = value else { panic!(\"text required\") };\n    text\n}\nfn test(labels: &[u64], kind: TopologyList, cancel: bool) {\n    let session = Session::default();\n    let mut output = actual::TopologyOutput::new(TopologySelection(labels.to_vec(), kind));\n    for _ in 0..10000 {\n        let initial = output.demand(&session, None, 0).unwrap();\n        let demand = output.demand(&session, None, initial.copy_bytes).unwrap();\n        let mut grant = Grant { maximum_copy_bytes: initial.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes };\n        if output.staged_claim().is_some() && cancel {\n            assert_eq!(session.live.get(), 0);\n            assert!(session.staged.get());\n            assert_eq!(output.close_claim(&session, &mut effect(grant)).unwrap().copied_items, 1);\n            assert!(output.staged_claim().is_none());\n            return;\n        }\n        if demand.copy_bytes > 0 {\n            grant.maximum_copy_bytes = 0;\n            let (answer, receipt) = output.step(&session, None, &mut effect(grant)).unwrap();\n            assert!(answer.is_none());\n            assert_eq!(receipt.copied_items, 0);\n            grant.maximum_copy_bytes = initial.copy_bytes;\n        }\n        let (answer, receipt) = output.step(&session, None, &mut effect(grant)).unwrap();\n        assert!(receipt.fits(grant));\n        if let Some(output) = answer {\n            assert!(!cancel);\n            assert_eq!(session.live.get(), labels.len());\n            let (channel, member_kind) = match kind { TopologyList::Shells => (\"shells\", \"shell\"), TopologyList::Solids => (\"solids\", \"solid\") };\n            let list_value = list(&output[channel]);\n            assert_eq!(text(&list_value[\"$schema\"]), \"list\");\n            assert_eq!(list_value.len(), labels.len() + 1);\n            for (index, label) in labels.iter().enumerate() {\n                let row = list(&list_value[&index.to_string()]);\n                assert_eq!(text(&row[\"$schema\"]), \"geometry\");\n                assert_eq!(text(&row[\"kind\"]), member_kind);\n                assert_eq!(text(&row[\"handle\"]), format!(\"{label:064x}\"));\n            }\n            return;\n        }\n    }\n    panic!(\"shell selection did not complete\");\n}\n"
finish += """
fn drive_reached(session: &Session, output: &mut actual::TopologyOutput, reach: &NativeTopologyReach) -> Dictionary {
    for _ in 0..10000 {
        let demand = output.demand(session, Some(reach), 0).unwrap();
        let grant = Grant { maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes };
        let (answer, receipt) = output.step(session, Some(reach), &mut effect(grant)).unwrap();
        assert!(receipt.fits(grant));
        if let Some(answer) = answer { return answer; }
    }
    panic!("typed topology list did not complete");
}
fn test_reached() {
    let session = Session::default();
    let reach = NativeTopologyReach(vec![30, 5, 11]);
    let mut output = actual::TopologyOutput::reached(ReachedTopology::Edge, 3, Dictionary::new());
    let base = drive_reached(&session, &mut output, &reach);
    assert!(output.reset("selectedEdges", ReachedTopology::Edge, 0, Some(deconstruct::LabelSelection(vec![1, 2, 0])), base).is_none());
    let base = drive_reached(&session, &mut output, &reach);
    assert_eq!(output.reset_empty("errors", base).unwrap().0, [1, 2, 0]);
    let final_output = drive_reached(&session, &mut output, &reach);
    for (port, originals) in [("edge", [30, 5, 11]), ("selectedEdges", [5, 11, 30])] {
        let members = list(&final_output[port]);
        for (index, label) in originals.into_iter().enumerate() {
            let row = list(&members[&index.to_string()]);
            assert_eq!(text(&row["$schema"]), "edge");
            assert_eq!(text(&row["handle"]), format!("{label:064x}"));
            assert!(!row.contains_key("kind"));
        }
    }
    assert_eq!(list(&final_output["errors"]).len(), 1);
}
"""
body = "\n".join(line for line in body.splitlines() if not line.startswith(("//!","use semio_framework", "use semio_s_spatial_kernel","#[derive(semio_framework","semio_framework_value::")))
texts = owner[owner.index("fn text_demand"):owner.index("fn geometry_handle")]
writer = owner[owner.index("fn write_text"):owner.index("impl OperatorJob for OriginalGeometryJob")]
program = prelude + texts + writer + "mod actual {\n" + body + "\n}\n" + finish + "fn main() {\n"
for row in fixture["cases"]:
    labels = repr(row["expected"])
    program += f"test(&{labels}, TopologyList::Shells, false);\ntest(&{labels}, TopologyList::Shells, true);\n"
explode = json.loads((e / "🧪️tests/🔬️unit/🔣️explode.json").read_text())
labels = repr(explode["members"])
program += f"test(&{labels}, TopologyList::Solids, false);\ntest(&{labels}, TopologyList::Solids, true);\ntest(&[], TopologyList::Solids, false);\n"
program += "test_reached();\n}\n"
generated = Path(__file__).parent / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as work:
    source = Path(work) / "shell.rs"
    binary = source.with_suffix("")
    source.write_text(program)
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition", "2024", "-A", "dead_code", str(source), "-o", str(binary)], check=True)
    subprocess.run([str(binary)], check=True)
print("Native topology output passed shell, repeated solid and empty output plus typed deconstruction and staged-claim cancellation cases with fixture adapters.")
