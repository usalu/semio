#!/usr/bin/env python3
"""👥️ C13 prepared set P4 v2 (T6 row 18, guest-linked SDK): a selection reaches the peers on the turn that commits it.

Measured 2026-09-29 on hub 7800 p33 (probe `c13presence4–8`, runs `.🧬semio/🌐hub/s14-c13-runs/`): a selection made in A's own
view (note, draw, puzzle 3d, puzzle 2d) never reaches B's presence — every Presence peer A sends carries `interaction: null`. The
browser-actor child publishes `AppFrame::Ephemeral` on every command turn, but always with an empty interaction: the interaction
verbs (`interactionSelect`, `selectAll`, `clearSelection`, …) are framework-reserved jobs — the command turn only admits them
(`Effect::SpawnJob`), the host runs the job, its `JobCompleted` hands the output back (`plugin_complete_reserved_spawned_job`,
which runs one commit unit) and later reactor turns finish the commit (`plugin_continue_typed_operations`). Only `plugin_exchange`
frames `AppFrame::Ephemeral`, so no snapshot taken after the commit ever reaches the worker, whose heartbeat stamps the last one.

v2 (session 15): v1 framed only on continuation turns and its law never ran the job — the overlay proof (`p4-overlay-proof.sh s15-1`)
showed the law red WITH v1 (one empty frame, the commit never ran). The runtime cell remembers the snapshot it last framed
(`RuntimeAppCell::published_ephemeral`); `plugin_exchange` keeps framing one per exchange and records it; both commit sites — the
`JobCompleted` unit and every continuation unit — append the instance's current snapshot when the unit published anything and the
snapshot differs from the recorded one (`frame_changed_ephemeral`, one `ephemeral_frame` helper). Law: the host's job completion
plus continuation turns frame the committed selection on the turn that frames the commit's result; a finished instance publishes
nothing more.

usage: p4-ephemeral-on-continuation.py [--root <repo or overlay>] (--dry-run | --write | --revert)
  --write   byte-backs up every touched file to `.🧬semio/🌐hub/s14-c13-w3-backup/c13-p4/before/` (+ `after/`) first
            (an overlay root backs up to `c13-p4-<overlay dir name>/` so the live tree's backup is never overwritten)
  --revert  restores `before/` for every file whose live bytes still equal `after/`; a file edited since is listed and kept"""
import json
import pathlib
import sys

args = sys.argv[1:]
root = pathlib.Path(args[args.index("--root") + 1]) if "--root" in args else pathlib.Path("/Users/ueli/Documents/semio")
modes = [mode for mode in ("--dry-run", "--write", "--revert") if mode in args]
if len(modes) != 1:
    sys.exit("usage: p4-ephemeral-on-continuation.py [--root <dir>] (--dry-run | --write | --revert)")
mode = modes[0]
live = pathlib.Path("/Users/ueli/Documents/semio")
backup = pathlib.Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-w3-backup") / ("c13-p4" if root.resolve() == live else f"c13-p4-{root.name}")

SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
LAWS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"

HUNKS: dict[str, list[tuple[str, str]]] = {
    SDK: [
        (
            """        maintenance_pressure: AtomicBool,
        #[cfg(target_arch = "wasm32")]
        maintenance_probe_turns: AtomicU64,""",
            """        maintenance_pressure: AtomicBool,
        /// 👥️ The [`EphemeralSnapshot`] this instance last framed onto an `AppFrame::Ephemeral` — a typed-operation unit frames
        /// the current one only when it differs, so a reserved job that commits a selection, presence or tool run after its
        /// command turn publishes it on the turn that commits it.
        published_ephemeral: std::sync::Mutex<Option<EphemeralSnapshot>>,
        #[cfg(target_arch = "wasm32")]
        maintenance_probe_turns: AtomicU64,""",
        ),
        (
            """                host_crossings: AtomicU32::new(0),
""",
            """                host_crossings: AtomicU32::new(0),
                published_ephemeral: std::sync::Mutex::new(None),
""",
        ),
        (
            """        if let Ok(EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run }) = ephemeral.await {
            let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
                let mut bytes = Vec::new();
                protocol::encode_presence_tool_run(&tool_run, &mut bytes);
                bytes
            });
            frames.push(protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run });
        }
""",
            """        if let Ok(snapshot) = ephemeral.await {
            if let Ok(cell) = runtime_instance_cell(runtime, instance_id) {
                *cell.published_ephemeral.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(snapshot.clone());
            }
            frames.push(ephemeral_frame(snapshot));
        }
""",
        ),
        (
            """    /// 📮️ Publishes admitted operations without requiring another user command — as many units of ONE""",
            """    /// 👥️ `snapshot` as the `AppFrame::Ephemeral` it is published as (its tool run in the presence encoding).
    fn ephemeral_frame(snapshot: EphemeralSnapshot) -> protocol::AppFrame {
        let EphemeralSnapshot { presence, presence_generation, transient_generation, interaction, tool_run } = snapshot;
        let tool_run = tool_run.map_or_else(Vec::new, |tool_run| {
            let mut bytes = Vec::new();
            protocol::encode_presence_tool_run(&tool_run, &mut bytes);
            bytes
        });
        protocol::AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run }
    }

    /// 👥️ Appends the `AppFrame::Ephemeral` a typed-operation unit owes: the instance's current snapshot when the unit published
    /// anything and the snapshot differs from the one `cell` last framed (which it then records).
    fn frame_changed_ephemeral<PA: PluginApp>(cell: &RuntimeAppCell<PA>, app: &PA, output: &mut PluginExchangeOutput) {
        if output.frames.is_empty() && output.effects.is_empty() && output.events.is_empty() && output.typed_operation_results.is_empty() {
            return;
        }
        let snapshot = resolve_ready(app.ephemeral_snapshot());
        let mut published = cell.published_ephemeral.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if published.as_ref() == Some(&snapshot) {
            return;
        }
        *published = Some(snapshot.clone());
        output.frames.push(resolve_ready(protocol::encode_app_frame(&ephemeral_frame(snapshot))));
    }

    /// 📮️ Publishes admitted operations without requiring another user command — as many units of ONE""",
        ),
        (
            """    /// next turn: the round-robin cursor is the fairness authority and one turn's output carries one
    /// receiver.
    pub async fn plugin_continue_typed_operations""",
            """    /// next turn: the round-robin cursor is the fairness authority and one turn's output carries one
    /// receiver. A unit that changed the instance's ephemeral snapshot (a reserved interaction commit applying a
    /// selection) frames it on the same turn ([`frame_changed_ephemeral`]).
    pub async fn plugin_continue_typed_operations""",
        ),
        (
            """                Ok(mut active) => advance_typed_operation_output(&mut active.app, instance)?,""",
            """                Ok(mut active) => {
                    let mut output = advance_typed_operation_output(&mut active.app, instance)?;
                    frame_changed_ephemeral(&cell, &active.app, &mut output);
                    output
                }""",
        ),
        (
            """    /// continuation finishes it on this and later turns. A job the app never admitted is a no-op so
    /// fill and other isolated jobs keep their existing JobCompleted path.
    pub async fn plugin_complete_reserved_spawned_job""",
            """    /// continuation finishes it on this and later turns. A job the app never admitted is a no-op so
    /// fill and other isolated jobs keep their existing JobCompleted path. A unit that changed the instance's ephemeral
    /// snapshot frames it on this turn ([`frame_changed_ephemeral`]).
    pub async fn plugin_complete_reserved_spawned_job""",
        ),
        (
            """            Ok(true) => {
                with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    advance_typed_operation_output(&mut instance.app, instance_id)
                })
                .await
            }""",
            """            Ok(true) => {
                with_instances_mut(runtime, |list| {
                    let mut instance = find_instance(list, instance_id)?;
                    let mut output = advance_typed_operation_output(&mut instance.app, instance_id)?;
                    let cell = instance.cell;
                    frame_changed_ephemeral(cell, &instance.app, &mut output);
                    Ok(output)
                })
                .await
            }""",
        ),
    ],
    LAWS: [
        (
            """    /// 🧵️ Every world/graph pick is one framework-reserved `interactionSelect` dispatch, and it must fit""",
            """    /// 👥️ A selection a reserved interaction job commits after its command turn is framed on the turn that commits it — the
    /// command turn's snapshot predates the job, so without that frame the browser actor's presence heartbeat stamped an empty
    /// interaction and no peer ever saw another human's selection (C13, hub 7800 p33: note, draw, puzzle 3d, puzzle 2d). The
    /// host's `JobCompleted` and the reactor's continuation turns are the turns that commit it; a finished instance publishes
    /// nothing more.
    #[semio_framework_async_macros::async_test]
    async fn a_reserved_selection_is_framed_on_the_turn_that_commits_it() {
        let id = meta().instance_id;
        let mut app = interaction_app_raw().await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta()).await.expect("seed label");
        let admitted = app.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &meta()).await.expect("interactionSelect admitted");
        let (job, completed) = finish_reserved_spawn_job(&admitted).await;
        let runtime = super::PluginRuntime::new();
        let cell = std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let mut pending = Some(super::plugin_complete_reserved_spawned_job(&runtime, id, job, completed).await);
        let (mut framed, mut committed): (Vec<(usize, Vec<String>)>, Vec<usize>) = (Vec::new(), Vec::new());
        for turn in 0..4_096usize {
            if let Some(output) = pending.take() {
                for bytes in &output.frames {
                    match protocol::decode_app_frame(bytes).await.unwrap() {
                        protocol::AppFrame::Invocation { in_reply_to: 0, .. } => committed.push(turn),
                        protocol::AppFrame::Ephemeral { interaction, .. } => {
                            let mut position = 0usize;
                            let selected = if interaction.is_empty() { Vec::new() } else { protocol::decode_presence_interaction(&interaction, &mut position).await.expect("interaction bytes decode").domains.into_iter().flat_map(|domain| domain.selected).collect() };
                            framed.push((turn, selected));
                        }
                        _ => {}
                    }
                }
                for page in output.typed_operation_results {
                    super::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            let (next, scan) = super::plugin_continue_typed_operations(&runtime, super::TypedOperationGrant::UNIT).await.unwrap();
            match next {
                Some((receiver, output)) => {
                    assert_eq!(receiver, id);
                    pending = Some(output);
                }
                None if !scan.runnable && !scan.contended => break,
                None => {}
            }
        }
        let (turn, selected) = framed.last().cloned().expect("the committed selection is framed");
        assert_eq!(selected, vec!["item-1".to_string()], "the last framed snapshot carries the committed selection: {framed:?}");
        assert!(committed.contains(&turn), "the selection is framed on the turn that frames the commit's result: framed {framed:?}, committed on {committed:?}");
        let (after, _) = super::plugin_continue_typed_operations(&runtime, super::TypedOperationGrant::UNIT).await.unwrap();
        assert!(after.is_none(), "a finished instance publishes nothing more");
        assert!(!cell.instance.lock().unwrap().app.has_pending_typed_operations());
        drop(cell);
        super::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            super::plugin_step_close_cleanup(&runtime).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
    }

    /// 🧵️ Every world/graph pick is one framework-reserved `interactionSelect` dispatch, and it must fit""",
        ),
    ],
}

if mode == "--revert":
    manifest = json.loads((backup / "manifest.json").read_text(encoding="utf-8"))
    kept = 0
    for relative in manifest["files"]:
        live = root / relative
        if live.read_bytes() != (backup / "after" / relative).read_bytes():
            kept += 1
            print(f"{relative}: KEPT (edited since the write)")
            continue
        live.write_bytes((backup / "before" / relative).read_bytes())
        print(f"{relative}: reverted")
    print(f"REVERTED: {kept} file(s) kept")
    sys.exit(1 if kept else 0)

problems = 0
writes: dict[pathlib.Path, str] = {}
for relative, hunks in HUNKS.items():
    path = root / relative
    original = path.read_text(encoding="utf-8")
    text = original
    for old, new in hunks:
        if old in new:
            state = "already applied" if text.count(new) == 1 else "applies" if text.count(old) == 1 else f"MISSING (old {text.count(old)}, new {text.count(new)})"
        else:
            state = "applies" if text.count(old) == 1 else "already applied" if text.count(new) == 1 and text.count(old) == 0 else f"MISSING (old {text.count(old)}, new {text.count(new)})"
        if state == "applies":
            text = text.replace(old, new)
        if state.startswith("MISSING"):
            problems += 1
        print(f"{relative}: {state}: {old.strip().splitlines()[0][:90]}")
    if text != original:
        writes[path] = text
write = mode == "--write" and problems == 0
if write:
    files = [str(path.relative_to(root)) for path in writes]
    for path, text in writes.items():
        relative = path.relative_to(root)
        for side, data in (("before", path.read_bytes()), ("after", text.encode("utf-8"))):
            target = backup / side / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    (backup / "manifest.json").write_text(json.dumps({"root": str(root), "files": files}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for path, text in writes.items():
        path.write_text(text, encoding="utf-8")
print(f"{'WRITTEN' if write else 'DRY RUN'}: {problems} problem(s), {len(writes)} file(s)")
sys.exit(1 if problems else 0)
