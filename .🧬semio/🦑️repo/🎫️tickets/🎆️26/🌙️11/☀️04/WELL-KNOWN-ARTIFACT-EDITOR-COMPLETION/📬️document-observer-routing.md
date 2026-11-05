# Local Document Observer Routing

Extracted production routeLocalDocumentObserverV1 from the actual backbone worker event branch into document-event-messages. The worker delegates to it after the local catalog handler, preserving catalog precedence. Temporary observers consume their own events. An observer matching the exact current client and runtime key returns false so ordinary document handling continues. Stale client observers cannot claim a replacement entry. Scoped messages cannot match an unscoped entry merely because client IDs agree.

Status delivery is strict: persisted true, pending count0 and no writeFailure. Archive bytes are unchanged. Optional status-only observers leave archive delivery intact. No unrelated handler was changed.

Schema-first neutral cases validated with Ajv and actual ordinary-handler invocation prove a failed write notifies the observer false while dispatching the same typed writeFailure to normal saved state. A real encoded full archive reaches both the byte observer and ordinary readBack.archive and bootstrap completion callback.

Test-first red8failures because helper was absent. Final focused suite11/11pass9.42seconds. Canonical broad host config includes the suite. Actual scope18/18 and generic readback19/19 remain separate previously passing suites.
