# Wave 6 policy run note

`bun ./📜️script.ts policy` exit **1** (same as Wave 2b).

`runPolicyExit` exits non-zero on any `priority: "high"` breach and prints **no summary** to stdout —
only the `[DEBUG] runPolicyScript …` lines. Log: `🧪wave6-policy.txt`.

Residual high breaches are expected: taxonomy/registry still flags missing TS mutation/engine stubs
and undeclared `#[path]` glue entries across plugins (see `🧪wave6-registry-check.txt`, 513 items).
These are structural completeness residuals, not leftover document-mutation `Operation` identifiers.
