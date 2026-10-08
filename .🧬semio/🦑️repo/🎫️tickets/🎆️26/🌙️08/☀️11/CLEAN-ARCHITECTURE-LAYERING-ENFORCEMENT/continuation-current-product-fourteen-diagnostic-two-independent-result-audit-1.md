# Product Fourteen Supplemental Diagnostics

Direct retained frames confirm list0,Engine1,Kernel1,ScenesMap0. No compiler or whole acceptance claim. Actual stderr values:

      left: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 90224, owner_bytes: 32 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
     right: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 90264, owner_bytes: 32 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
      left: [FixedSlotTableBudget { owner: "kernel_runtime::RetainedSurfaceRegistry", capacity: 64, element_bytes: 25672, owner_bytes: 32 }, FixedSlotTableBudget { owner: "kernel_runtime::CommandDocumentRetirementRegistry", capacity: 512, element_bytes: 584, owner_bytes: 4112 }]
     right: [FixedSlotTableBudget { owner: "kernel_runtime::RetainedSurfaceRegistry", capacity: 64, element_bytes: 24488, owner_bytes: 32 }, FixedSlotTableBudget { owner: "kernel_runtime::CommandDocumentRetirementRegistry", capacity: 512, element_bytes: 584, owner_bytes: 4112 }]

Scenes supplemental success applies only to captured14 and does not accept unrun original whole tests. CurrentEngine90224 andKernel25672 differ held90264/24488. Full source/runtime/binary postguard method completed; original14whole remains failed.
