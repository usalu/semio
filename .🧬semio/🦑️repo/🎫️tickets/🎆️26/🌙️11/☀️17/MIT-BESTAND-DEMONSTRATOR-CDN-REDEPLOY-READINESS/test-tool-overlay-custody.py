"""🧪️ Compile the actual Rust ToolOverlay dispatch and cleanup classifier with deterministic owner inputs."""
import json
import re
import sqlite3
import subprocess
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "MODULE.bazel").is_file())
plugin = next((root / "🧰️framework").glob("*/💻️os/*/🔌️plugin"))
ladder = (plugin / "🪜️maintenance-ladder/🦀️.rs").read_text()
render = (plugin / "🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs").read_text()
cleanup = (plugin / "⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs").read_text()
vocabulary = (plugin / "⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧹️cleanup-vocabulary/🦀️.rs").read_text()
fixture = json.loads((ticket / "🔣️tool-overlay-custody.json").read_text())
output = ticket / "🗑️generated" / "overlay-custody"
output.mkdir(parents=True, exist_ok=True)

arm = re.search(r"^\s*MaintenanceUnit::ToolOverlay\s*=>.*?(?=^\s*MaintenanceUnit::ToolRuns)", ladder, re.M | re.S)[0]
helpers = re.findall(r"^    (?:pub )?fn (?:tool_overlay_action_source_pending|tool_overlay_waits_for_foreground)\b.*?^    }", render, re.M | re.S)
classifier = re.search(r"^pub fn runtime_live_cleanup_nonterminal_status\b.*?^}", cleanup, re.M | re.S)[0]
limit = re.search(r"^pub const RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT: u32 = \d+;", vocabulary, re.M)[0]
cases = [
    '("%s", %s, %s)' % (row["name"], "None" if row["actionSlot"] is None else "Some(%d)" % row["actionSlot"], str(row["follower"]).lower())
    for row in fixture["cases"]
]
program = r"""
use std::sync::atomic::{AtomicU32,Ordering};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedCloneProgress { copied_items: usize }
#[derive(Clone, Copy)]
pub enum PluginLifecycleStep {
    AwaitingInput { reason: &'static str },
    Progress(RetainedCloneProgress),
    Blocked { reason: &'static str },
}
#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeMaintenanceStatus { Ready, Fault(RuntimeCleanupFault) }
#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeCleanupFault { ZeroProgress }
enum MaintenanceUnit { ToolOverlay }
struct Tools {
    original_tool_follow: Option<()>,
    original_tool_tag_capture: Option<()>,
    original_tool_tag_retirement: Option<()>,
    original_scrub_dispatch: Option<()>,
    original_press_settlement: Option<()>,
    original_typing_settlement: Option<()>,
}
struct Ladder { tools: Tools, calls: usize, retired_items: usize }
impl Ladder {
    fn shared(&self) -> &Self { self }
    fn as_render(&mut self) -> &mut Self { self }
    fn tool_overlay_retirement_step(&mut self, _: (), _: (), closing: bool) -> Result<RetainedCloneProgress, ()> {
        assert!(!closing);
        self.calls += 1;
        let items = usize::from(self.tools.original_tool_follow.is_some());
        self.retired_items += items;
        Ok(RetainedCloneProgress { copied_items: items })
    }
    $HELPER
    fn advance(&mut self) -> Result<PluginLifecycleStep, ()> {
        let (cx,grant) = ((),());
        match MaintenanceUnit::ToolOverlay { $ARM }
    }
}
fn progressed(progress: RetainedCloneProgress) -> PluginLifecycleStep { PluginLifecycleStep::Progress(progress) }
$CLASSIFIER
$LIMIT
fn main() {
    for (name,slot,follower) in [$CASES] {
        let mut ladder=Ladder { tools: Tools {
            original_tool_follow: follower.then_some(()),
            original_tool_tag_capture: (slot==Some(0)).then_some(()),
            original_tool_tag_retirement: (slot==Some(1)).then_some(()),
            original_scrub_dispatch: (slot==Some(2)).then_some(()),
            original_press_settlement: (slot==Some(3)).then_some(()),
            original_typing_settlement: (slot==Some(4)).then_some(()),
        }, calls: 0, retired_items: 0 };
        let count=AtomicU32::new(0);
        let mut verdict="idle";
        for _ in 0..RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT {
            let step=ladder.advance().unwrap();
            let status=runtime_live_cleanup_nonterminal_status(false,Some(step),&count);
            verdict=match (step,status) {
                (_,RuntimeMaintenanceStatus::Fault(_))=>"stalled",
                (PluginLifecycleStep::AwaitingInput { .. },_)=>"awaiting",
                (PluginLifecycleStep::Progress(receipt),_) if receipt.copied_items!=0=>"progress",
                _=>"idle",
            };
        }
        if verdict=="awaiting" {
            assert_eq!(count.load(Ordering::SeqCst),0);
            assert_eq!((ladder.calls,ladder.retired_items),(0,0));
            match slot.unwrap() {
                0=>ladder.tools.original_tool_tag_capture=None,
                1=>ladder.tools.original_tool_tag_retirement=None,
                2=>ladder.tools.original_scrub_dispatch=None,
                3=>ladder.tools.original_press_settlement=None,
                4=>ladder.tools.original_typing_settlement=None,
                _=>unreachable!(),
            }
            assert!(matches!(ladder.advance().unwrap(),PluginLifecycleStep::Progress(receipt) if receipt.copied_items==0));
            assert_eq!(ladder.calls,1);
            ladder.tools.original_tool_follow=Some(());
            assert!(matches!(ladder.advance().unwrap(),PluginLifecycleStep::Progress(receipt) if receipt.copied_items==1));
        }
        println!("{name} {verdict}");
    }
}
"""
for placeholder, value in {
    "ARM": arm, "HELPER": "\n".join(helpers), "CLASSIFIER": classifier,
    "LIMIT": limit, "CASES": ",".join(cases),
}.items():
    program = program.replace("$" + placeholder, value)

source = output / "probe.rs"
executable = output / "probe"
source.write_text(program)
subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024", "-Awarnings", str(source), "-o", str(executable)], cwd=root, check=True)
actual = dict(line.split() for line in subprocess.check_output([str(executable)], text=True).splitlines())
with sqlite3.connect(":memory:") as oracle:
    for row in fixture["cases"]:
        expected = oracle.execute(
            "SELECT CASE WHEN ?2 THEN 'progress' WHEN ?1 IS NOT NULL THEN 'awaiting' ELSE 'stalled' END",
            (row["actionSlot"],row["follower"]),
        ).fetchone()[0]
        assert expected == row["expected"]
        assert expected == actual[row["name"]], (row["name"], expected, actual[row["name"]])
        print(row["name"], actual[row["name"]])
