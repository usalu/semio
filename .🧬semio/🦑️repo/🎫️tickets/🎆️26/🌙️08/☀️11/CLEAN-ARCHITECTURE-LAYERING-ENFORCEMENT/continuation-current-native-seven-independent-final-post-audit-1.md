# Current Seven Independent Final Runtime Audit

Direct retained terminals and logs were read independently. Route objects match their plans; all three owning source postchecks are available with null post refusal, and all dispatcher runtime/package postchecks report null refusal and false cancellation. Product is an actual negative owning result: its owning refusal is the child exit error, not null. No publication release is issued.

```json
[
  {
    "owner": "ui",
    "owningExit": 0,
    "owningRefusal": null,
    "dispatcherExit": 0,
    "dispatcherRefusal": null,
    "sourcePostUnavailable": false,
    "postRefusals": null,
    "cancelled": false,
    "logSha256": "890739bd5ca76ed95f2f31d65c5e39f8715fadbfc39241628f1f5f914b4e3546",
    "evidence": []
  },
  {
    "owner": "board",
    "owningExit": 0,
    "owningRefusal": null,
    "dispatcherExit": 0,
    "dispatcherRefusal": null,
    "sourcePostUnavailable": false,
    "postRefusals": null,
    "cancelled": false,
    "logSha256": "58f2203d7a1c88c32090b88599fcb90258248c98fbdd030be6a7efadc036913f",
    "evidence": [
      "test board::ports::directed_dag::tests::selected_nodes_cursor_matches_language_neutral_control_escapes_and_external_json ... ok",
      "test world::tests::world_live_captured_authority_retains_start_ids_and_refuses_replaced_owner ... ok",
      "test result: ok. 545 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s",
      "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s"
    ]
  },
  {
    "owner": "product",
    "owningExit": 1,
    "owningRefusal": "Error: /Users/ueli/.bun/bin/bun exited with status 1",
    "dispatcherExit": 1,
    "dispatcherRefusal": null,
    "sourcePostUnavailable": false,
    "postRefusals": null,
    "cancelled": false,
    "logSha256": "b96f6b94a1230b7669473d195a9d6fae3116ef4b15c5704373f09f534d9f8127",
    "evidence": [
      "    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1685 filtered out; finished in 0.07s",
      "      left: [FixedSlotTableBudget { owner: \"engine_canvas::EngineSurfaceRegistry\", capacity: 256, element_bytes: 90264, owner_bytes: 32 }, FixedSlotTableBudget { owner: \"engine_canvas::StagedEngineScenes\", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: \"engine_canvas::EngineCanvasBuildContext\", capacity: 256, element_bytes: 384, owner_bytes: 72 }]",
      "     right: [FixedSlotTableBudget { owner: \"engine_canvas::EngineSurfaceRegistry\", capacity: 256, element_bytes: 86544, owner_bytes: 32 }, FixedSlotTableBudget { owner: \"engine_canvas::StagedEngineScenes\", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: \"engine_canvas::EngineCanvasBuildContext\", capacity: 256, element_bytes: 384, owner_bytes: 72 }]"
    ]
  }
]
```

UI original whole passed 787 with one leaky classification and zero skips; Board passed545 with both new native causal laws explicit in log. Product original unfiltered/default policy failed slot layout law, with measured90264 versus committed86544. Its unrun tests are not accepted. External historical layout attribution remains unproven; no budget change is admitted by this report.

Direct summary lines:

- ui: Summary [  10.695s] 787 tests run: 787 passed (1 leaky), 0 skipped
- product: Summary [   2.536s] 264/1675 tests run: 263 passed, 1 failed, 11 skipped
