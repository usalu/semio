# Patch: `assert_pane_smoke_by_example` lacks redo (generator, statik)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🪧️pane-smoke-law/🦀️.rs`, fn `assert_pane_smoke_by_example`.
After `assert_eq!(probe(app), loaded, "pane smoke: undo must restore the loaded snapshot");` and before `close_registered_fixture_app(...)` insert:

    settle_history_verb(app, "redo", receiver, mounted_policy, identity, cx).await;
    assert_eq!(probe(app), applied, "pane smoke: redo must reapply the typed command");
