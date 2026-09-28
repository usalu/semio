#!/usr/bin/env python3
"""🩹️ T14 rule-22 (test-only) fix of the two SDK lib reds P9 measured (`wp-p9`, 14:2x) — no non-test code changes.

1. `activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations`: the framework's `cancelTypedOperation`
   (`operation_progress::cancellation_action_definition`, declared `resumable_framework` → Migrated) is served by the
   direct arm at the head of `dispatch_action` (`dispatch_operation_cancellation`), never by a tool factory — the same
   residue class as `setActiveUtility`. The law names it in the directly-routed residue and pins its direct arm
   (whitespace-insensitive, so rustfmt cannot break the pin).
2. `a_scene_that_declares_no_lanes_still_publishes_one_childless_surface`: `TableScene` now splits `columns`/`rows` lanes,
   so the law uses `IconRenderScene`, the scene its own `SceneDoc::split_lanes` doc names as a no-lane doc.

usage: sdk-lib-reds.py [--write] [--root <tree>]   (default: dry run on the live tree)"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
PATH = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
HUNKS = [
    (
        """        // 2. `setActiveUtility`, which the builder injects as a Migrated action whenever the app
        //    declares utilities and which `handle_action_invocation` routes DIRECTLY to
        //    `dispatch_emit` — it is deliberately served without a tool factory, so a missing
        //    registration is not the dead-action defect it would be for any other migrated verb.
""",
        """        // 2. Migrated verbs the framework serves without a tool factory, so a missing registration
        //    is not the dead-action defect it would be for any other migrated verb: `setActiveUtility`
        //    (injected whenever the app declares utilities; `handle_action_invocation` routes it
        //    DIRECTLY to `dispatch_emit`) and `cancelTypedOperation` (every app's operation-progress
        //    cancel; the head of `dispatch_action` routes it DIRECTLY to `dispatch_operation_cancellation`).
""",
    ),
    (
        """let framework_directly_routed_migrated: std::collections::BTreeSet<String> = ["setActiveUtility"].into_iter().map(String::from).collect();""",
        """let framework_directly_routed_migrated: std::collections::BTreeSet<String> = ["cancelTypedOperation", "setActiveUtility"].into_iter().map(String::from).collect();""",
    ),
    (
        """            "the only reason `setActiveUtility` may carry no factory is its direct `dispatch_emit` arm in `handle_action_invocation`"
        );
""",
        """            "the only reason `setActiveUtility` may carry no factory is its direct `dispatch_emit` arm in `handle_action_invocation`"
        );
        assert!(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🦀️.rs")).split_whitespace().collect::<String>().contains("ifaction==CANCEL_TYPED_OPERATION_ACTION_ID{returnself.dispatch_operation_cancellation(args,meta).await;}"),
            "the only reason `cancelTypedOperation` may carry no factory is its direct `dispatch_operation_cancellation` arm at the head of `dispatch_action`"
        );
""",
    ),
    (
        """        let scene = semio_framework_ui_scene::TableScene::base("[]", "[]");
        let (projection, lanes) = project_scene_surface(crate::app::scene_surface("results", semio_framework_ui_contract::SurfaceKind::Table, &scene).unwrap());
        assert!(lanes.is_empty());
        assert_eq!(projection["component"]["type"], "surface");
        assert_eq!(artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_ui_scene::TableScene>(&serde_json::to_string(&projection).unwrap()).unwrap(), scene);
""",
        """        let scene = semio_framework_ui_scene::IconRenderScene { request_json: r#"{"meshId":"box"}"#.into(), footer: None, frame_json: None };
        let (projection, lanes) = project_scene_surface(crate::app::scene_surface("preview", semio_framework_ui_contract::SurfaceKind::IconRender, &scene).unwrap());
        assert!(lanes.is_empty());
        assert_eq!(projection["component"]["type"], "surface");
        assert_eq!(artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_ui_scene::IconRenderScene>(&serde_json::to_string(&projection).unwrap()).unwrap(), scene);
""",
    ),
]
text = PATH.read_text(encoding="utf-8")
problems, applied, pending = [], 0, 0
for old, new in HUNKS:
    if text.count(new) == 1 and (old not in new or text.count(old) == 1):
        applied += 1
        continue
    if text.count(old) != 1:
        problems.append(f"anchor found {text.count(old)} times: {old.strip()[:90]!r}")
        continue
    text = text.replace(old, new)
    pending += 1
if problems:
    print("\n".join(problems))
    print(f"{PATH.relative_to(ROOT)}: {len(problems)} problem(s)")
    sys.exit(1)
if pending == 0:
    print(f"nothing to do (applied) {applied}/{len(HUNKS)} hunks")
    sys.exit(0)
if WRITE:
    PATH.write_text(text, encoding="utf-8")
print(f"{PATH.relative_to(ROOT)}: {'written' if WRITE else 'dry-run'} {pending} hunk(s), {applied} already applied, 0 problems")
