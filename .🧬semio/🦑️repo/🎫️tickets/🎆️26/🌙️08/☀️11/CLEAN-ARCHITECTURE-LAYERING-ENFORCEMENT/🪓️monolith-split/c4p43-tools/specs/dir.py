from specs.lib import *

CH = [
    ch('📖', 'events-commands-and-admin', 'events_commands_and_admin', [(61, 667)],
       'Events, commands and administration of the directory schema: the space kinds, visibility and roles, the artifact hash, the actor and clock, the event log records and pages, the directory command with its request, receipt and error codes and the administration intents, progress, connection and audit records.'),
    ch('🪟', 'views-and-document-open', 'views_and_document_open', [(670, 1394)],
       'Views and document opening of the directory schema: the space, member, user, connection and document views, the administration pages and the document open intent, package, surface and plan with their validation.'),
    ch('🛰', 'execution-target-artifacts-and-stream', 'execution_target_artifacts_and_stream', [(1397, 1858)],
       'Execution target, artifacts and stream of the directory schema: the execution target lease fields and status codes, the descriptor digest validation, the artifact frontier, checkpoint and retention records, the invite and document views and the directory stream message.'),
]

SPEC = dict(
    orig='dir.rs',
    file=DIR + '/🦀️.rs',
    children=CH,
    external=mounted(DIR + '/🦀️.rs', M + '/📇️directory/🦀️.rs'),
)
