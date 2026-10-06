"""⏯️ S5-GRAPHS-WIRES — wave `tool-run-settle`: the nine reds of the `tool_run` law family (shared plugin law run, 10-05 16:40).

Causes (read from the code and from the run of the existing test binary, `📓️s3-wires-report.md` § Session 5 → T5):

1. A terminal member run never settled. `discard_provisional` / `release_provisional` mark the member `rebase` and move its
   composed read to `stale`, but the driver did nothing in `Finalized | Aborted | Faulted`, so `has_pending_work()` stayed
   true forever (five `member_run_*` laws). → the terminal arm settles the member (`settle_tool_run_member`).
2. A retryable saturation of the child-content retirement ring (`interactive-job.child-root-retirement-saturated`, documented
   as backpressure) faulted the member run. → it keeps the turn instead (`refresh_tool_run_member`), and a read that cannot
   retire yet waits in `stale` without losing the fold's result (`refresh_member_overlay`).
3. The driver returned after ONE retirement unit per turn and ran the generation watch only behind it. One tick of n ops
   displaces n - 1 intermediate overlays, each alias retires over several units, so the settings / base watch starved
   (three laws, measured by W2A in `📓️w2-a-report.md` §6.4). → the watch runs first for every run, retirement runs within
   the turn wall budget (`retire_tool_runs_until`), the runs are driven only while nothing retires.
4. The retarget law asserted "no work" in the very turn the run completed, while that turn's intermediates still retire.
   → it pumps until the run is complete AND quiet (a resident job that counted as work would never go quiet).
5. The running-panel fixture is older than `UiValue`'s exact-integer JSON (`generation: 0.0` → `0`). → re-authored; the
   text equals what the law's own writer (`SEMIO_TOOL_RUN_PANEL_OUT`) emits.
6. Released member ops and published emissions queue for retirement (peer fields) but were not owed work. → `is_work`.

    python3 🧪️s5-graphs-tool-run-settle.py --check   [rust|fixture]   # dry run against the LIVE tree, writes nothing
    python3 🧪️s5-graphs-tool-run-settle.py --apply   [rust|fixture]   # backs up, then writes
    python3 🧪️s5-graphs-tool-run-settle.py --restore [rust|fixture]   # puts the backups back

`rust` = the runtime and the two law files (under the `landing` lock, rules 58 + 67); `fixture` = the panel fixture (a
non-Rust framework file: under the `serve` lock, rule 61); no part = all four.

Fails closed: four explicit files, every anchor must occur exactly once in the live file, `--apply` writes nothing unless
every file of the part converts, `--restore` refuses a file that is no longer what `--apply` wrote.
"""

import pathlib
import sys

TICKET = pathlib.Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
RUNTIME = PLUGIN / "⏯️tool-run/🦀️.rs"
LAWS = PLUGIN / "🧪️tests/🔬️tool-run/🦀️.rs"
MEMBER_LAWS = PLUGIN / "🧪️tests/🧪️tool-run-member/🦀️.rs"
PANEL = PLUGIN / "🧫️fixtures/⏯️tool-run/🪧️panel-running.json"
BACKUP = TICKET / "🗑️generated/s5-graphs-wires/tool-run/before"
WRITER_OUTPUT = TICKET / "🗑️generated/s5-graphs-wires/tool-run/panel-running.after.json"
NAMES = {RUNTIME: "runtime.rs", LAWS: "laws.rs", MEMBER_LAWS: "member-laws.rs", PANEL: "panel-running.json"}
PARTS = {"rust": (RUNTIME, LAWS, MEMBER_LAWS), "fixture": (PANEL,), "all": (RUNTIME, LAWS, MEMBER_LAWS, PANEL)}

EDITS = {
    RUNTIME: [
        (
            "member is_work",
            """    /// 🏃️ Owed work: ops the overlay does not show yet, a refold from the member head, or reads and aliases to retire.
    fn is_work(&self) -> bool {
        self.rebase || self.shown != self.ops.len() || !self.stale.is_empty() || self.owners.as_deref().is_some_and(ToolRunMemberOwners::has_displaced)
    }
""",
            """    /// 🏃️ Owed work: ops the overlay does not show yet, a refold from the member head, or reads, aliases, released ops
    /// and published emissions to retire.
    fn is_work(&self) -> bool {
        self.rebase || self.shown != self.ops.len() || !self.stale.is_empty() || !self.retired_ops.is_empty() || !self.retired_emits.is_empty() || !self.retired_emission_owners.is_empty() || self.owners.as_deref().is_some_and(ToolRunMemberOwners::has_displaced)
    }
""",
            1,
        ),
        (
            "driver turn",
            """    /// ⏯️ One bounded driver turn (≤ `ToolRunDriver::turn_wall_us`): owed dirty scope, retirement, watch, refold,
    /// job steps, finalize.
    pub(crate) async fn drive_tool_run_turn(&mut self) -> Result<(), Fault> {
        self.flush_tool_run_ui_dirty();
        self.drain_tool_run_port();
        let started = semio_framework_job::default_now_us().unwrap_or(0);
        let deadline = started.saturating_add(self.tool_runs.driver.turn_wall_us);
        if let Some(step) = self.tool_run_retire_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)? {
            if !matches!(step, PluginCloseStep::Blocked { .. }) {
                return Ok(());
            }
        }
        let runs: Vec<u64> = self.tool_runs.entries.iter().map(|entry| entry.slot.run).collect();
        let mut outcome = Ok(());
""",
            """    /// ⏯️ One bounded driver turn (≤ `ToolRunDriver::turn_wall_us`): owed dirty scope, the generation watch of every
    /// run, retirement within the wall budget, then — while nothing retires — refold, job steps, finalize. The watch never
    /// waits behind retirement: a settings or base change reaches its run in the turn it becomes visible.
    pub(crate) async fn drive_tool_run_turn(&mut self) -> Result<(), Fault> {
        self.flush_tool_run_ui_dirty();
        self.drain_tool_run_port();
        let started = semio_framework_job::default_now_us().unwrap_or(0);
        let deadline = started.saturating_add(self.tool_runs.driver.turn_wall_us);
        let runs: Vec<u64> = self.tool_runs.entries.iter().map(|entry| entry.slot.run).collect();
        for run in &runs {
            if self.tool_runs.select_run(*run) {
                self.watch_tool_run_generations();
            }
        }
        self.tool_runs.select_primary();
        if self.retire_tool_runs_until(deadline)? {
            return Ok(());
        }
        let mut outcome = Ok(());
""",
            1,
        ),
        (
            "retire within the wall budget",
            """    /// ⏯️ The selected run's share of one driver turn.
    async fn drive_selected_tool_run(&mut self, deadline: u64) -> Result<(), Fault> {
""",
            """    /// 🚰️ Retirement units until nothing retires, a retirement blocks or the turn deadline. `true` while owners still
    /// retire or a unit released nothing: the runs wait for the next turn instead of displacing more.
    fn retire_tool_runs_until(&mut self, deadline: u64) -> Result<bool, Fault> {
        loop {
            match self.tool_run_retire_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)? {
                None | Some(PluginCloseStep::Blocked { .. }) => return Ok(false),
                Some(PluginCloseStep::Pending { released_items, released_bytes }) if released_items > 0 || released_bytes > 0 => {
                    if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
                        return Ok(true);
                    }
                }
                Some(_) => return Ok(true),
            }
        }
    }

    /// ⏯️ The selected run's share of one driver turn.
    async fn drive_selected_tool_run(&mut self, deadline: u64) -> Result<(), Fault> {
""",
            1,
        ),
        (
            "watch leaves the selected share",
            """            ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete => {
                self.watch_tool_run_generations();
                if self.refold_tool_run(deadline) || self.refresh_tool_run_member(deadline)? {
""",
            """            ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete => {
                if self.refold_tool_run(deadline) || self.refresh_tool_run_member(deadline)? {
""",
            1,
        ),
        (
            "terminal member settles",
            """            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => Ok(()),
""",
            """            ToolRunState::Finalized | ToolRunState::Aborted | ToolRunState::Faulted => self.settle_tool_run_member(deadline),
""",
            1,
        ),
        (
            "refresh docstring",
            """    /// the member's place once every op is shown. `true` while the fold still owns this turn; a fold that cannot apply its
    /// ops (they do not decode as the member's vocabulary) faults the run.
""",
            """    /// the member's place once every op is shown. `true` while the fold still owns this turn — also while the retirement
    /// of the reads it replaced is saturated (retryable), so the job waits instead of leasing more; a fold that cannot
    /// apply its ops (they do not decode as the member's vocabulary) faults the run.
""",
            1,
        ),
        (
            "retryable saturation is backpressure",
            """            Ok(None) => Ok(false),
            Err(_) => {
                self.fault_tool_run(run, generation);
                Ok(false)
            }
""",
            """            Ok(None) => Ok(false),
            Err(fault) if fault.retryable => Ok(true),
            Err(_) => {
                self.fault_tool_run(run, generation);
                Ok(false)
            }
""",
            1,
        ),
        (
            "a replaced read waits in stale",
            """            if let Some(previous) = std::mem::replace(&mut member.children, next) {
                self.retire_tool_run_views(vec![previous], &mut member.stale)?;
            }
""",
            """            if let Some(previous) = std::mem::replace(&mut member.children, next) {
                match self.retire_tool_run_views(vec![previous], &mut member.stale) {
                    Err(fault) if !fault.retryable => return Err(fault),
                    _ => {}
                }
            }
""",
            1,
        ),
        (
            "settle_tool_run_member",
            """    /// 🧹️ Retires a member run's composed reads in order through the child-content retirements, one admitted generation
""",
            """    /// 🪦️ A terminal member run holds no provisional op and shows none: the composed reads it replaced retire and its
    /// overlay rests on the member head again, so the run owes nothing. A member that is gone has no head to rest on; a
    /// saturated retirement ring (retryable) is retried on the next turn.
    fn settle_tool_run_member(&mut self, deadline: u64) -> Result<(), Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(()) };
        let outcome = if self.children.get(&(member.slot.clone(), member.child_id.clone())).is_some() {
            self.refresh_member_overlay(&mut member, deadline).map(|_| ())
        } else {
            member.rebase = false;
            let stale = std::mem::take(&mut member.stale);
            self.retire_tool_run_views(stale, &mut member.stale)
        };
        selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot").member = Some(member);
        match outcome {
            Err(fault) if !fault.retryable => Err(fault),
            _ => Ok(()),
        }
    }

    /// 🧹️ Retires a member run's composed reads in order through the child-content retirements, one admitted generation
""",
            1,
        ),
    ],
    LAWS: [
        (
            "retarget law pumps to complete and quiet",
            """    pump_until(&mut app, "initial target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete)).await;
    assert!(!app.tool_runs.has_pending_work(), "a resident job of a complete run is no work");
""",
            """    pump_until(&mut app, "initial target completes and its resident job is no work", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work()).await;
""",
            1,
        ),
    ],
    MEMBER_LAWS: [
        (
            "member pump names what is still owed",
            """    panic!("{what} never settled; state {:?}", app.tool_runs.state());
}

/// 🧾️ The applied history rows a tool transaction of the member tool keys.
""",
            """    let reasons: Vec<u16> = app.tool_runs.steps().map(|steps| steps.iter().map(|step| step.reason).collect()).unwrap_or_default();
    let admission = app.admit_child_content_publication().err().map(|fault| fault.code.0);
    panic!("{what} never settled; state {:?}; member ops {}; ledger work {}; step reasons {reasons:?}; child-content admission refusal {admission:?}", app.tool_runs.state(), app.tool_runs.member_ops().len(), app.tool_runs.has_pending_work());
}

/// 🧾️ The applied history rows a tool transaction of the member tool keys.
""",
            1,
        ),
    ],
    PANEL: [("exact-integer generation", '                    "generation": 0.0,\n', '                    "generation": 0,\n', 4)],
}


def convert(path: pathlib.Path, source: str, refusals: list) -> str:
    text = source
    for what, before, after, count in EDITS[path]:
        found = text.count(before)
        if found != count:
            refusals.append("%s: anchor '%s' occurs %d times, expected %d" % (NAMES[path], what, found, count))
            continue
        text = text.replace(before, after)
    return text


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) in (2, 3) else ""
    part = sys.argv[2] if len(sys.argv) == 3 else "all"
    if mode not in ("--check", "--apply", "--restore") or part not in PARTS:
        raise SystemExit("usage: 🧪️s5-graphs-tool-run-settle.py --check | --apply | --restore [rust|fixture]")
    files = PARTS[part]
    missing = [NAMES[path] for path in files if not path.is_file()]
    if missing or not PLUGIN.is_dir():
        raise SystemExit("[tool-run-settle] REFUSED: missing %s" % ", ".join(missing or ["plugin root"]))
    if mode == "--restore":
        saved = {path: BACKUP / NAMES[path] for path in files}
        absent = [NAMES[path] for path, backup in saved.items() if not backup.is_file()]
        if absent:
            raise SystemExit("[tool-run-settle] REFUSED: no backup of %s" % ", ".join(absent))
        refusals = []
        applied = {path: convert(path, saved[path].read_text(), refusals) for path in files}
        moved = [NAMES[path] for path in files if path.read_text() != applied[path]]
        if refusals or moved:
            raise SystemExit("[tool-run-settle] REFUSED: %s changed since --apply; restore by hand from %s" % (", ".join(moved or refusals), BACKUP))
        for path, backup in saved.items():
            path.write_text(backup.read_text())
        print("[tool-run-settle] restored %d files" % len(saved))
        return
    refusals, converted = [], {}
    for path in files:
        converted[path] = convert(path, path.read_text(), refusals)
    if PANEL in converted and WRITER_OUTPUT.is_file() and not refusals and converted[PANEL] != WRITER_OUTPUT.read_text():
        refusals.append("panel-running.json: the re-authored fixture differs from the law writer's output")
    if refusals:
        raise SystemExit("[tool-run-settle] REFUSED, nothing written:\n  " + "\n  ".join(refusals))
    edits = sum(len(EDITS[path]) for path in files)
    if mode == "--apply":
        BACKUP.mkdir(parents=True, exist_ok=True)
        for path in files:
            (BACKUP / NAMES[path]).write_text(path.read_text())
        for path, text in converted.items():
            path.write_text(text)
    print("[tool-run-settle] %s (%s): %d edits in %d files%s" % ("applied" if mode == "--apply" else "would apply", part, edits, len(files), "; panel fixture equals the law writer's output" if PANEL in converted and WRITER_OUTPUT.is_file() else ""))


main()
