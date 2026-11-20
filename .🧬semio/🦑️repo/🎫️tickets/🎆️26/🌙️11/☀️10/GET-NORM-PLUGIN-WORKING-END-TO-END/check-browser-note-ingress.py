"""🔍 Exercise the native action manifest receiver with the exact browser command bytes."""

import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
workspace = ticket.parents[6]
generated = ticket / "🗑️generated"
lane = workspace / ".cache/b/r/m/u/l" / sys.argv[1]
build = lane / "wasm32-wasip2/prototype/build"
source = generated / "browser-note-ingress.rs"
source.write_text('''
use semio_framework_actor_retained_context::OriginalActorContext;
use semio_framework_actor_retained_turn::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_async::poll::resolve_ready;
use semio_framework_job::StepBudget;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer};
use semio_plugin_reactor_app::NoPluginApp;
use semio_plugin_plugin_program::decode_wire_serialized;
#[cfg(live_surface_source)]
#[path = "PLACEHOLDER_SURFACE_SOURCE"]
mod surface_context;
#[cfg(not(live_surface_source))]
use semio_plugin_plugin_program::surface_context;
use surface_context::{OriginalSurfaceRefresh, SurfaceContexts};
use semio_framework::ViewModel;
use semio_plugin_reactor_lifecycle::life_instance_registry::{PluginRuntime, RuntimeActorAuthority};
use semio_plugin_runtime_ops::ops_ingress::OriginalDecodedPluginCommand;

fn clock() -> Option<u64> { Some(1) }

fn main() {
    let omit_view = std::env::args().any(|arg| arg == "without-view");
    let direct_surface = std::env::args().any(|arg| arg == "direct-surface");
    let max_copy: usize = std::env::args().find_map(|arg| arg.strip_prefix("copy=").map(|n| n.parse().unwrap())).unwrap_or(usize::MAX);
    let max_items: usize = std::env::args().find_map(|arg| arg.strip_prefix("items=").map(|n| n.parse().unwrap())).unwrap_or(usize::MAX);
    let cancel_at: Option<usize> = std::env::args().find_map(|arg| arg.strip_prefix("cancel-at=").map(|n| n.parse().unwrap()));
    let bytes = include_bytes!("din4108-note.bin");
    assert_eq!(bytes[0], 1, "expected addressed browser command");
    let mut next = &bytes[1..];
    fn varint(input: &mut &[u8]) -> u64 {
        let mut value = 0;
        for shift in (0..64).step_by(7) {
            let (byte, rest) = input.split_first().unwrap();
            *input = rest;
            value |= u64::from(byte & 127) << shift;
            if byte & 128 == 0 { return value; }
        }
        panic!("wire length exceeds u64")
    }
    let seq = varint(&mut next);
    let command_len = varint(&mut next) as usize;
    let (command, mut next) = next.split_at(command_len);
    let view_len = varint(&mut next) as usize;
    assert_eq!(view_len, next.len());
    let mut view_state = next.to_vec();
    println!("[DEBUG] browser view bytes {}", view_state.len());
    if omit_view { view_state.clear(); }
    let mut refresh = None;
    let mut refresh_admission = Default::default();
    let mut surfaces = SurfaceContexts::default();
    let mut surface_count = 0;
    let mut view = if direct_surface {
        let input: ViewModel = resolve_ready(decode_wire_serialized(next)).unwrap();
        surface_count = input.window_instances.len();
        for window in &input.window_instances {
            let mut projection = input.clone();
            projection.window_id = Some(window.id.clone());
            surfaces.insert(window.id.clone(), "window-kind".to_owned(), projection).unwrap();
        }
        use semio_framework_value::retained_clone::{RetainedCloneClose, RetainedCloneOwner, RetainedCloneSource, RetainedCloneSourceAdmission};
        println!("[DEBUG] sizes cursor {}, source {}, source admission {}, retire {}", std::mem::size_of::<RetainedCloneOwner<ViewModel>>(), std::mem::size_of::<RetainedCloneSource<ViewModel, ViewModel>>(), std::mem::size_of::<RetainedCloneSourceAdmission<ViewModel, ViewModel, ()>>(), std::mem::size_of::<RetainedCloneClose>());
        println!("[DEBUG] live roster {} surfaces; refresh owner {} bytes", surface_count, std::mem::size_of::<OriginalSurfaceRefresh>());
        Some(std::sync::Arc::new(input))
    } else { None };
    if direct_surface {
        let body = std::mem::size_of::<OriginalSurfaceRefresh>();
        let quote = semio_framework_job::original_construction_settled_turn_quote::<Box<OriginalSurfaceRefresh>>(semio_framework_job::OriginalConstructionDemand { items: 1, copy_bytes: body + 4, capacity_bytes: body, release_bytes: 0, depth: 1 }, 1).unwrap();
        println!("[DEBUG] exact in-place birth turn quote {quote:?}");
    }
    let original_view = view.as_ref().map(std::sync::Arc::as_ptr);
    let mut refresh_ready = false;
    let mut owner = OriginalDecodedPluginCommand::new(semio_os_kernel::AppCommand::Command { seq, command: command.to_vec(), view_state });
    let runtime = PluginRuntime::<NoPluginApp>::new(semio_plugin_mounted_owner::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).unwrap();
    runtime.instance_actors.borrow_mut().insert_admitted(1, RuntimeActorAuthority::new("local").unwrap());
    let mut input = RetainedTurnInput { operation: 1, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 3_600_000_000, fuel: 50_000_000 },
        grant: RetainedCloneGrant { maximum_items: 8_388_608, maximum_copy_bytes: 1_073_741_824,
            maximum_capacity_bytes: 536_870_912, maximum_release_bytes: 536_870_912, maximum_depth: 64 } };
    let mut actor = OriginalActorContext::new((), input).unwrap();
    for _ in 0..100 {
        let ready = actor.admit_step(input, StepBudget::new(input.work.fuel, input.work.absolute_deadline_us, input.grant), clock).unwrap().ready;
        input.epoch += 1; input.work = actor.remaining_work().unwrap(); input.grant = actor.remaining_grant().unwrap();
        if ready { break; }
    }
    actor.take_input().unwrap();
    let mut wire_ready = false;
    for turn in 0..1_024 {
        actor.begin_dispatch(input).unwrap();
        let mut progress = RetainedCloneProgress::default();
        let slice = if wire_ready {
            RetainedCloneGrant {
                maximum_copy_bytes: input.grant.maximum_copy_bytes.min(max_copy),
                maximum_items: input.grant.maximum_items.min(max_items),
                ..input.grant
            }
        } else { input.grant };
        let mut cx = actor.borrow_context(input, StepBudget::new(input.work.fuel, input.work.absolute_deadline_us, slice), &mut progress, clock).unwrap();
        if !wire_ready {
            cx.acknowledge_return_rows_locally();
            wire_ready = cx.admit_original_core_wire_delivery().unwrap() && cx.admit_original_escrow_publisher().unwrap();
        } else if direct_surface {
            if cancel_at == Some(turn) {
                assert!(refresh.is_some() && view.is_none());
                cx.original_source_parent().unwrap().cancellation().unwrap().cancel_now();
                refresh_ready = true;
            }
            let before = cx.retained_progress();
            if refresh.is_none() {
                OriginalSurfaceRefresh::admit(&mut cx, &mut refresh_admission, &mut refresh).unwrap();
            } else if !refresh_ready {
                loop {
                    let before = (cx.retained_progress(), cx.fuel_remaining());
                    if refresh.as_mut().unwrap().advance(&mut surfaces, &mut view, &mut cx).unwrap() {
                        assert_eq!(view.as_ref().map(std::sync::Arc::as_ptr), original_view);
                        for window in &view.as_ref().unwrap().window_instances {
                            let projection = surfaces.borrow(&window.id).unwrap();
                            assert_eq!(projection.view_state.window_id.as_deref(), Some(window.id.as_str()));
                            assert_eq!(projection.view_state.active_window_kind_id.as_deref(), Some(window.window_kind_id.as_str()));
                        }
                        println!("browser view projected through {surface_count} mounted surface roles on turn {turn}");
                        refresh_ready = true;
                        break;
                    }
                    if cx.should_yield() || (cx.retained_progress(), cx.fuel_remaining()) == before { break; }
                }
            } else if cx.with_original_retirement(|cx| refresh.as_mut().unwrap().close_step(cx)).unwrap() {
                assert!(refresh.as_ref().unwrap().terminal_is_empty());
                assert_eq!(view.as_ref().map(std::sync::Arc::as_ptr), if cancel_at.is_some() { None } else { original_view });
                use semio_plugin_job::original_steps::forgotten;
                if cx.with_original_retirement(|cx| forgotten(cx, OriginalSurfaceRefresh::release_demand(), || drop(refresh.take()))).unwrap() {
                    if let Some(cut) = cancel_at {
                        println!("browser original surface refresh cancelled at turn {cut} and retired the captured input on turn {turn}");
                    } else {
                        println!("browser original surface refresh closed with its input restored on turn {turn}");
                    }
                    std::process::exit(0);
                }
            }
            if turn < 28 || turn % 100 == 0 { println!("[DEBUG] surface step {turn} admitted {} view {} copied {} grant {}", refresh.is_some(), view.is_some(), cx.retained_progress().copied_bytes - before.copied_bytes, cx.retained_grant().maximum_copy_bytes); }
        } else {
            let before = cx.retained_progress();
            let observer: &mut Observer<'_> = &mut |_| true;
            let mut identity = EntityIdentityAuthority::new(1 << 24, observer).unwrap();
            let result = resolve_ready(owner.advance_manifest_step(&runtime, 1, &mut identity, &mut cx));
            if turn < 20 || turn % 250 == 0 { println!("[DEBUG] note {turn}: receiving {}, copied {}", owner.receiving.is_some(), cx.retained_progress().copied_bytes - before.copied_bytes); }
            if let Err(error) = result {
                let message = error.primary.message();
                assert!(message.contains("instance") || omit_view && message.contains("requires its view"), "unexpected manifest error {error:?}");
                println!("browser note reached mounted surface refresh dispatch on turn {turn}: {message}");
                std::process::exit(0);
            }
        }
        drop(cx);
        input.epoch += 1; input.work = actor.remaining_work().unwrap(); input.grant = actor.remaining_grant().unwrap();
    }
    panic!("browser command or surface projection did not complete");
}
'''.replace("PLACEHOLDER_SURFACE_SOURCE", str(workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs")))
output = generated / "browser-note-ingress.wasm"
args = ["--edition=2021", "--target=wasm32-wasip2", "-Copt-level=0", "-Ccodegen-units=8",
        "-Cpanic=abort", "-Clink-arg=-zstack-size=8388608", str(source), "-o", str(output)]
args += ["-Ldependency=" + str(path) for path in lane.glob("**/out") if "/build/" in str(path)]
crates = ["semio-framework-actor-retained-context", "semio-framework-actor-retained-turn", "semio-framework-async",
          "semio-framework", "semio-framework-job", "semio-framework-value", "semio-os-kernel", "semio-plugin-reactor-app", "semio-plugin-plugin-program",
          "semio-plugin-reactor-lifecycle", "semio-plugin-runtime-ops", "semio-plugin-mounted-owner", "semio-plugin-job", "semio-ui-contract"]
for crate in crates:
    candidates = list((build / crate).glob("*/out/*.rlib"))
    if not candidates:
        raise RuntimeError(f"compiled crate absent from lane {lane.name}: {crate}")
    library = max(candidates, key=lambda path: path.stat().st_mtime)
    for suffix in [".rlib", ".rmeta"]:
        args += ["--extern", crate.replace("-", "_") + "=" + str(library.with_suffix(suffix))]
live_surface = "--live-surface" in sys.argv
if live_surface:
    args += ["--cfg", "live_surface_source"]
flags = generated / "browser-note-ingress.args"
flags.write_text("\n".join(args) + "\n")
subprocess.run([os.environ.get("RUSTC", "rustc"), "@" + str(flags)], check=True)
narrow = ["direct-surface", "copy=32768", "items=64"]
for scenario in ([narrow, narrow + ["cancel-at=20"]] if live_surface or "--built-surface" in sys.argv else [[], ["without-view"], ["direct-surface"], narrow]):
    subprocess.run(["node_modules/.bin/jco", "run", "--jco-dir", str(generated / "browser-note-jco"), str(output), *scenario], cwd=workspace, check=True)
