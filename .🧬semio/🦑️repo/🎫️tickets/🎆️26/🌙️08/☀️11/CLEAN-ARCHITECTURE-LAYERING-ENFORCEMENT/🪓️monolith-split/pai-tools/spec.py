"""📐 pai spec: the role children of the single `impl PluginApp for VcsArtifactApp`. Each child holds an inherent `impl VcsArtifactApp` block with the moved methods; the trait impl keeps one-line forwarders at the original method positions."""
V = '️'
ROOT = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs'
IMPL_HEADER = "impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {"


def ch(emoji, kebab, mod, methods, doc):
    return dict(emoji=emoji, folder=emoji + V + kebab, mod=mod, methods=methods, doc='//! %s%s %s See [`super`].' % (emoji, V, doc))


CHILDREN = [
    ch('🔒', 'close-terminal', 'close_terminal', ['close_terminal_is_empty'],
       'Close terminal of the mounted artifact instance: the conjunction that says every owned store, disposer, lease, reservation and ladder of the instance has drained.'),
    ch('🧾', 'typed-operation-results', 'typed_operation_results',
       ['tool_public_contracts', 'acknowledge_typed_operation_result', 'take_typed_operation_completion'],
       'Typed operation results of the mounted artifact instance: the public tool contracts, the acknowledgement of a result page and the terminal completion with its history patch.'),
    ch('🕹', 'ephemeral-interaction', 'ephemeral_interaction',
       ['begin_local_interaction_query', 'publish_local_interaction_query_reply', 'ephemeral_snapshot'],
       'Ephemeral interaction of the mounted artifact instance: the local interaction query lane and the ephemeral snapshot of the interaction and presence roots.'),
    ch('📨', 'action-dispatch', 'action_dispatch',
       ['handle_action_invocation', 'resume_task_emit'],
       'Action dispatch of the mounted artifact instance: the invocation of one action address and the resumption of a task emit.'),
    ch('🔀', 'transactions', 'transactions',
       ['transaction_prepare', 'transaction_commit'],
       'Transactions of the mounted artifact instance: the prepare and commit legs of a composed transaction.'),
    ch('🗃', 'document-load', 'document_load',
       ['hydrate_pure_head', 'begin_document_archive_load'],
       'Document load of the mounted artifact instance: the pure head hydration and the start of a document archive load.'),
    ch('🖼', 'render-surface', 'render_surface',
       ['render', 'window_engagements', 'window_measures', 'tool_measures', 'pending_effects', 'context_menu'],
       'Render surface of the mounted artifact instance: the render of a body key, the window engagements and measures, the pending effects poll and the context menu funnel.'),
    ch('🎞', 'media-transfer', 'media_transfer',
       ['produce_media', 'consume_media', 'retain_media_export_result'],
       'Media transfer of the mounted artifact instance: the production and consumption of a media artifact and the retained export result.'),
]
