"""🛬️ The WP3 lifts of PLAN-PLUGIN-CRATES.md Appendix J (and the two intra-package groups), applied in one pass. DRY=1 prints the moved items without writing. All writes roll back on failure."""
import os, sys, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lift3
from lift3 import lift, rollback, relocate, write, rd, delete
P = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
L = '🛬️lifted/'
NAMES = []
def go(src, seeds, dest, mod, **kw):
    taken = lift(P + src, seeds, P + dest, mod, **kw)
    for it in taken:
        import cutplan as cp
        NAMES.append((src, (cp.itemize(it.text)[0]['defname'] or it.name[:50]) if cp.itemize(it.text) else it.name[:50]))
INF_REG = 'ArtifactInferenceServiceRegistry,ArtifactInferenceService,ArtifactInferenceExecutionError,ArtifactInferenceServiceMetadata,ArtifactInferenceExecutionRequest,ArtifactInferencePayloadContract,ArtifactInferenceExecution,WireArtifactInferenceBudget,WireArtifactInferenceCacheMode,WireArtifactInferenceDiagnostic,ArtifactInferenceDocumentBinding,ArtifactInferenceServiceKey,ArtifactInferenceRegistrationConflict,ArtifactInferenceRegistrationError,ArtifactInferenceCommitBinding,ArtifactInferenceExecutable,ArtifactInferenceExecutionStep,ArtifactInferenceStepContext,ArtifactInferenceDemands,ArtifactInference'.split(',')
INF_PROG = 'validate_wire_request_resources,WireArtifactInferenceResult,WireArtifactInferenceRequest,validate_inference_execution,validate_wire_request_metadata,WireArtifactInferenceMetadata,WireInferencePayloadContract,WireArtifactInferenceProvenance,inference_child_dependency_parts,WireInferenceArtifactBinding,wire_list_artifact_inference_services_with_routes,begin_artifact_inference,WireInferenceCommitBinding,wire_list_artifact_inference_services,artifact_inference_service_registry,active_artifact_inferences,cancelled_artifact_inferences,inference_cancelled,bounded_len,ARTIFACT_INFERENCE_SERVICE_REGISTRY,ARTIFACT_INFERENCE_WIRE_VERSION,ACTIVE_ARTIFACT_INFERENCES,CANCELLED_ARTIFACT_INFERENCES,ArtifactInferenceCancellationGuard'.split(',')
try:
    print('L1 preparation -> decl-emission')
    go('🧩️composition/📨️emission/📦️preparation/🦀️.rs', [], '📜️declaration/📨️emission/' + L + '📦️preparation/🦀️.rs', 'lifted_preparation', whole=True, files=['♻️retirement/🦀️.rs'])
    print('L2 inference registry -> decl-registry')
    go('💡️inference/🦀️.rs', INF_REG, '📜️declaration/' + L + '💡️inference/🦀️.rs', 'lifted_registry')
    print('L3 inference wire -> plugin-program')
    go('💡️inference/🦀️.rs', INF_PROG, '🏗️builder/🔌️program/' + L + '💡️inference/🦀️.rs', 'lifted_program')
    print('L4 gateway cold entry points -> plugin-program')
    go('💡️inference/🚪️gateway/🦀️.rs', ['wire_artifact_infer_cold', 'wire_artifact_infer_from_cold', 'wire_artifact_infer_from_context_cold', 'wire_artifact_infer_with_service_cold'], '🏗️builder/🔌️program/' + L + '🚪️gateway/🦀️.rs', 'lifted_program')
    print('L5 history panel -> time-travel-panel')
    go('🎮️command/🦀️.rs', ['ui_history_panel'], '⏪️time-travel/🪟️panel/' + L + '🎮️command/🦀️.rs', 'lifted_panel', stay=['FRAMEWORK_CHECKIN_CONTROLLER_ID', 'uncommitted_command_count', 'history_panel_icon_id', 'history_panel_alternative_row', 'history_panel_unavailable_row', 'history_panel_mutation_row'])
    print('L6 context -> decl-app')
    go('🧵️retained-command/🧬️context/🦀️.rs', [], '📜️declaration/🧩️app/' + L + '🧬️context/🦀️.rs', 'lifted_context', whole=True)
    relocate(P + '🧵️retained-command/🧬️context/📸️fields/🦀️.rs', P + '📜️declaration/🧩️app/' + L + '📸️fields/🦀️.rs')
    if not lift3.DRY:
        ctx = P + '📜️declaration/🧩️app/' + L + '🧬️context/🦀️.rs'
        txt = lift3.rd(ctx)
        import re as _re
        m = _re.findall(r'#\[path="([^"]*📸️fields/🦀️.rs)"\]', txt)
        assert len(m) == 1, m
        write(ctx, txt.replace(m[0], '../📸️fields/🦀️.rs'))
    NAMES.append(('🧵️retained-command/🧬️context', 'context-module'))
    print('L7 media -> decl-app')
    go('🎞️media/⚠️turn/🦀️.rs', ['key_media_error', 'project_media_error'], '📜️declaration/🧩️app/' + L + '🎞️media-turn/🦀️.rs', 'lifted_media_turn', extra=['invalid', 'media_type'])
    go('🎞️media/🛫️intrinsic/🦀️.rs', ['media_projection_error'], '📜️declaration/🧩️app/' + L + '🛫️intrinsic/🦀️.rs', 'lifted_intrinsic')
    print('L8 job -> retained-command')
    go('👷️job/🏛️reserved/🦀️.rs', ['FRAMEWORK_RESERVED_ROUTE_CAPACITY'], '🧵️retained-command/' + L + '🏛️reserved/🦀️.rs', 'lifted_route_capacity', doc='//! 🏛️ Lift file: 1 item, about 20 lines, of `👷️job/🏛️reserved/🦀️.rs` that belong to crate `retained-command` (WP3).\n//!\n//! WP3 owns both crates: the framework reserved route capacity names `ArtifactRetainedWorkCapacity`, so it moves up from `job` to `retained-command`. Until `retained-command` is carved, the source module re-exports the item with `pub use`.')
    print('L9 retained-command -> job')
    go('🧵️retained-command/🔄️full-operation/🦀️.rs', ['TypedCommandFullOperationJob', 'TypedCommandFullOperationJobFactory', 'TypedCommandFullOperationPhase'], '👷️job/' + L + '🔄️full-operation/🦀️.rs', 'lifted_full_operation_job', doc='//! 🔄️ Lift file: the generic bounded full-operation job, its phase and its tool job factory, of `🧵️retained-command/🔄️full-operation/🦀️.rs` that belong to crate `job` (WP3).\n//!\n//! WP3 owns both crates: `QualifiedBoundedFirstStepProof::admits` names the factory type, and the factory\'s `ToolJobFactory` impl names the job, so the three items move down together from `retained-command` to `job` (the plan moved the 16-line factory alone, which would have left its impl naming a `retained-command` type). Until `job` is carved, the source module re-exports them with `pub use`.')
except BaseException as e:
    rollback(); raise
json.dump(NAMES, open(HERE + '/../🗑️generated/pwp3-moved-names.json', 'w'), ensure_ascii=False, indent=0)
print('total items', len(NAMES))
