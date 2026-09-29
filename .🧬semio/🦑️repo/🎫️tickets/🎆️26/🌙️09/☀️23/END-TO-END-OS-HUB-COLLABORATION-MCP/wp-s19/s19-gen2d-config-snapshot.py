#!/usr/bin/env python3
"""🎨️ S19 set `gen2d-config-snapshot` (guest, T6 round 4+): generation2d's `setActiveExample` publishes the example's
config as ONE `Generation2dConfigMutation::Snapshot { config }` (plus its document ops), but the app's own retained
config preparation (`generation2d_config_publication_bytes` + the preparation's `advance`) admitted only `SetShowMode`
and `SetSelectedGeneration` → every example load through the registered typed-operation lane faulted
`generation2d-config-unsupported-mutation` (measured: SDK declared-verb census, `s14-s19-logs/census-t7-1.txt`, the
boot example of `probe_declared_verbs::<Generation2dPlayApp>` could not settle). The preparation now admits the whole
config: its bytes are the config's own text envelope (`generation2d_config_text_bytes`), the post state is the carried
config, the inverse is the base config. Law `every_roster_example_loads_through_the_registered_lane`.
usage: s19-gen2d-config-snapshot.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

EDITOR = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
TESTS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

BYTES_OLD = '''        Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id } => selected_generation_id.as_ref().map_or(0, String::len),
        _ => return Err("generation2d-config-unsupported-mutation".into()),
'''
BYTES_NEW = '''        Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id } => selected_generation_id.as_ref().map_or(0, String::len),
        Generation2dConfigMutation::Snapshot { config } => generation2d_config_text_bytes(config),
        _ => return Err("generation2d-config-unsupported-mutation".into()),
'''
ADVANCE_OLD = '''            Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id } => {
                next.selected_generation_id.clone_from(selected_generation_id);
                Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: base.get().selected_generation_id.clone() }
            }
            _ => return Err("generation2d-config-unsupported-mutation".into()),
'''
ADVANCE_NEW = '''            Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id } => {
                next.selected_generation_id.clone_from(selected_generation_id);
                Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: base.get().selected_generation_id.clone() }
            }
            Generation2dConfigMutation::Snapshot { config } => {
                next = config.clone();
                Generation2dConfigMutation::Snapshot { config: base.get().clone() }
            }
            _ => return Err("generation2d-config-unsupported-mutation".into()),
'''
LAW = '''
//#region 🎨️RegisteredExampleLoad
/// 🎨️ LAW: every roster example loads through the registered typed-operation lane. `setActiveExample` publishes the
/// example's config as ONE `Snapshot` config mutation, which the retained config preparation must admit — it refused
/// it (`generation2d-config-unsupported-mutation`), so the SDK's declared-verb fixture could not even boot the demo.
#[semio_framework_async_macros::async_test]
async fn every_roster_example_loads_through_the_registered_lane() {
    let _serial = crate::publication_authority::lock();
    let roster = <Generation2dPlayApp as ArtifactEditor>::examples();
    assert!(!roster.is_empty(), "generation2d offers examples");
    for example in roster {
        let mut app = app_with_registry().await;
        crate::editor::generation2d::unit_tests::context::dispatch(&mut app, Generation2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example.id().into() })).await;
        close(app);
    }
}
//#endregion 🎨️RegisteredExampleLoad
'''


def editor(text):
    return lib.chain(lib.replace_once(BYTES_OLD, BYTES_NEW), lib.replace_once(ADVANCE_OLD, ADVANCE_NEW))(text)


def tests(text):
    if "fn every_roster_example_loads_through_the_registered_lane" in text:
        return text
    anchor = "//#endregion 🌱️HubGenesis\n"
    assert text.count(anchor) == 1, "HubGenesis region end"
    return text.replace(anchor, anchor + LAW)


lib.run("gen2d-config-snapshot", [(EDITOR, editor), (TESTS, tests)], sys.argv[1:])
