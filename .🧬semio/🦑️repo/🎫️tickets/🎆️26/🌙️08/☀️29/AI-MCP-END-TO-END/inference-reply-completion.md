# Inference Reply Completion

The shared ActionAdapter now requires a guest reply to echo the requested inference schema. The real inference router already correlates the full request identity. The adapter also enforces schema identity for other artifact channels before accepting the payload, with the expected and received schemas in the error details.

The MCP guest execution path performs one cold call and retires its origin before returning. It has no continuation worker or retained previous state. A matching partial reply remains valid at the artifact channel boundary. At one-shot MCP settlement it now yields the existing FAILED state with complete false, its JSON payload when present, and its exact payload byte count. Both synchronous inference and submitted guest jobs use that settlement before success or an approval offer. Existing cancellation handling still takes precedence.

The six-case neutral corpus covers a complete JSON reply, partial JSON, partial opaque bytes, absent partial bytes, and replies for another schema with either completion flag. The independent Bun SQLite comparison passed 18 assertions. A focused native harness loaded both production modules and linked the exact 605 cached crate dependencies from the Cargo dispatch graph. Both tests passed. The full Dispatch and InferenceTools crate sources also passed independent exact native metadata checks.
