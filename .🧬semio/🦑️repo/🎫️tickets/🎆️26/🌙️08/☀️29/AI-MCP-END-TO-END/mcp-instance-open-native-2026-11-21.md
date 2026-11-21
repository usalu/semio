# MCP InstanceOpen Native Receiving

The Note and WFC MCP fixtures reproduce the first editor event with instance 0, activation generation and request sequence 1, empty configuration, assets and quotas, and the descriptor’s single `artifacts.write` capability at `plugin` scope. The grant carries the canonical decimal token `"0"` and omits expiry. The poll limits use the published host driver’s operation policy and the configured lifetime as a deterministic deadline offset from zero. Serde validates the fixtures and produces compact wire in the same field order as the host Event projection.

The native comparison uses the published Infrastructure grant of 65,536 items, 16 MiB copy, 64 MiB capacity and release, and depth 64. The bounded caller gives 256 items and 1 MiB in each byte currency per turn, with the same decoder limits and retained caller identity. Both scopes report Ready and the input returned at the full wire extent. Whole typed output matches each neutral fixture. Every turn’s capacity and release receipt matches the physical allocator, and both the input owner and caller retire.

| Input | Scope | Parsed bytes | Turns to Ready | Work | Copied items | Copy bytes | Capacity bytes | Released bytes |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Note | Infrastructure | 601/601 | 1 | 18,153 | 12,289 | 5,731,881 | 2,412,040 | 2,411,495 |
| Note | Bounded | 601/601 | 49 | 18,405 | 12,541 | 5,771,377 | 2,412,040 | 2,411,495 |
| WFC | Infrastructure | 602/602 | 1 | 18,157 | 12,292 | 5,732,095 | 2,412,042 | 2,411,496 |
| WFC | Bounded | 602/602 | 49 | 18,406 | 12,541 | 5,771,123 | 2,412,042 | 2,411,496 |

The exact grant payload terminates under the full Infrastructure budget with roughly 5.7 MB of receipted native copy and the bounded scope reaches the same output and physical allocation totals. The reported live interpreter traffic above 8 GB precedes the first receipt on a different accounting axis. This probe exercises readiness and wire disposition in the underlying native receiver. WASI instruction execution and the exported SRE1 receipt writer are outside its measurements.

The focused native suite passed all five tests using the optimized pinned relink against 534 exact dependencies. The independent Ajv oracle passed all 11 neutral cases, including the capability token and expiry shape.
