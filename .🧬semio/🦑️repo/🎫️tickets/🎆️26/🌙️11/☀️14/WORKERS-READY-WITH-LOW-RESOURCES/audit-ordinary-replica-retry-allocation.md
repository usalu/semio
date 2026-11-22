# Ordinary Replica Retry Allocation

At 17:47 UTC the Windows worker `g1-m05` matched the coordinator's staged tree and reported 5,072 pending incoming operations, 226,034 lifetime sharing-violation observations and 34 incomplete-pack observations. The counters show repeated sharing failures. They do not identify the files, their lock holders, or how many distinct operations are affected. The stage match and journal cursor coverage prove intake and the staged tree. The worker had an adopted workspace attempt and `ready=false`.

The steady-state queue previously decoded the complete incoming record before checking a still-active retry cooldown. The sweep of its retained temporary put also decoded the full record every round. The scheduler now skips cooled ordinary entries before creating materialization jobs when the exact winning operation and clock are still current. Retained puts compare the compact manifest columns for operation, clock, length, mode and digest. Stage-related work, structural clash checks and the guard against concurrent materialization remain in place.

The native SQLite-backed fixture measured two full record decodes per cooled turn on the old path and zero after the change. A later winning revision still materializes its new bytes immediately. Nine focused cases passed with 83 assertions, covering unrelated ready work, retained puts, concurrent turns and a missing queued descendant. The complete native race suite then passed 36 tests with 265 assertions and the latest native self-check passed all module, schema and catalogue checks. Live Windows throughput and native Ready remain to be measured on the worker.

The Windows worker reported the new replica digest `5833bd94f16502c16334f2093620201382811632f28abe01fca301731285a7c9` at 17:59 UTC. Its incoming queue still contained 5,078 operations and sharing observations continued. The code change removes avoidable queue work while those file conflicts remain.

[Stage and rollout evidence](audit-durable-replica-stage.md)
