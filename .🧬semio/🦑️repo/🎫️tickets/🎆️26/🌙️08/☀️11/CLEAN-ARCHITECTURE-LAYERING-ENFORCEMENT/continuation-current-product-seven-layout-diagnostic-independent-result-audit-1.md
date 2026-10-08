# Compiled Seven Slot Diagnostic Independent Result

Direct retained frames show list0, exact EngineCanvas case1 failure, KernelRuntime case2 success, and Scenes case3 failure. The actual inherited stderr log retains both descriptor disagreements:

      left: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 90264, owner_bytes: 32 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
     right: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 86544, owner_bytes: 32 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
      left: [FixedSlotTableBudget { owner: "scenes::AdmittedSurfaceMap<World3dState>", capacity: 256, element_bytes: 28440, owner_bytes: 3136 }]
     right: [FixedSlotTableBudget { owner: "scenes::AdmittedSurfaceMap<World3dState>", capacity: 256, element_bytes: 28400, owner_bytes: 3136 }]

The result is supplementary and preserves original Product seven whole failure. Kernel diagnostic success does not accept unrun whole tests. Engine measured90264 versus86544; Scenes measured28440 versus28400. Current source/model guards and captured runtime bytes were checked by the diagnostic method; external Node package membership was explicitly not rescanned. The Scenes forty-byte difference belongs coupled World held-model verification, so applying its descriptor alone is not admitted.
