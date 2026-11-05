# Documentless Identity ABI

The authoritative identity read previously queried document_base_revision even when document_identity returned None. The default documentless app cannot supply a document revision, so the operation falsely emitted an error.

The command now reads revision only for a present document and uses zero for absence. This matches the existing channel schema and neutral unavailable identity fixture: null parent, canonical decimal revision zero, frame bytes [31,8,18,0,0]. Present documents still propagate a revision failure.

A real PluginAppDelegate wrapper advertises no document and faults/counts revision reads. The mounted regression sends the actual encoded ReadDocumentIdentity command through plugin_exchange, matches the existing neutral binary response, checks no revision query occurred and closes its real delegated app under the existing original fixture caller. It adds no alternate production helper or special test dispatch.

Rust execution is queued in the live sequential plan as documentless-identity-abi.

Independent ticket documentless-identity-oracle.mjs passed: existing protobufjs Writer reproduces all three command/response wire vectors byte-for-byte, including absent identity with revision zero; AJV validates all identities against the owned schema. No external dependency added.
