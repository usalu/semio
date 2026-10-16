CRATES = [
 ("contract", ["errors#*", "handles#*", "audit#*", "schema#🔖️RevisionStamp", "schema#🔖️InvocationReport", "schema#🔖️PreparedActionReport", "schema#🔖️SearchHit", "schema#🔖️JobStatus", "schema#🔖️ContextSummary", "owner_only#*"]),
 ("wire", ["protocol#*", "notify#*"]),
 ("schema", ["schema#*"]),
 ("catalog", ["catalog#*", "search#*", "conformance#*", "registry#*"]),
 ("policy", ["policy#*", "rendezvous#*", "agent_credential#*"]),
 ("bridge", ["bridge#*", "ui#*"]),
 ("transport", ["transport#*"]),
 ("dispatch", ["actions#*"]),
 ("workspace", ["workspace*", "shell_channel#*", "context#*"]),
 ("inference", ["inference*"]),
 ("tools", ["artifact#*", "prompts#*"]),
 ("runtime", ["root#*", "entrypoint#*"]),
]
