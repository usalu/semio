# Upper Job Payload Ownership

The page-return machine now calls the two existing public functions on semio_framework_job_payload directly: ledger_demands and close_ledger. The prior lexical payload_retirement module belongs privately to the extracted Context crate. No function implementation, outcome, grant or CPU receipt changes; this removes an invalid cross-crate lexical reference without exposing Context internals. The existing page-return owning laws remain mandatory and unexecuted.
