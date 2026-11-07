STATUS: wasm check GREEN (verfolgen-12 rc 0, 0 errors, 5 gis crates; 🗑️generated/1105b/verfolgen-12.txt). Native pane smoke FAILS in framework: "original lifecycle receipt exceeds its exact phase loan" on document load (W-verfolgen-3.txt:81314), UNVERIFIED beyond that.

- Retirement patch store-replay-retire-unborn-registry-birth APPLIED (replay/🦀️.rs:495).
- Tests (pane-smoke/🦀️.rs): verfolgen_pane_loads_applies_undoes_and_closes (existing) and verfolgen_pane_body_renders_in_every_locale (new, case data 🧫️cases.json, en+de). Fixed 2 test-code compile faults (unit/🦀️.rs:108 view_state .into(); PluginApp import in pane-smoke). Both now compile (W-verfolgen-3, rc 1: 0 passed, 2 failed).
- Failure: InvariantViolated "original lifecycle receipt exceeds its exact phase loan", progress copied_items 2 vs loan maximum_items 1; stack plugin instance-ladders 🦀️.rs:374 <- instance-app 📥️staging/🪜️maintenance-ladder/🦀️.rs:47 <- plugin-app-impl 🦀️.rs:230 <- laws load_document_text. Routed to main. First-load work_spent not reached.
- Next: rerun tst.sh W-verfolgen-4 (1105b-tools/verfolgen/tst.sh) after the framework fix.
