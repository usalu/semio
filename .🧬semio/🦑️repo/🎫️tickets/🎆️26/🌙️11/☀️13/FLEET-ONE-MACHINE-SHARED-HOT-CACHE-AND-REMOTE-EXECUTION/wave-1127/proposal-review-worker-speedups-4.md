# 🧭 Review: Worker Feedback Wave 4 (02:30, 2026-11-28)

| New point | State | Lane |
|---|---|---|
| Plan one target at a time, service shares reads; restart service only on planning-code changes; long-lived client↔service connections renew + retry | H103 s1 (read identity = own closure) live; H38/H56 overlays | **H112** |
| Versioned machine-wide records (grants), unknown versions counted by declared size | **Done** H110 (live) | verify H111 |
| Hard caps on disk images + automatic cleanup; split 96 GiB cache image; one capacity model; alarm when a rerun recompiles > 20 % | H34/H41/H47/H66 guards/collect/grow/shards; H93 budget design | **H113** (implement H93 allocator as the single capacity model, rerun-recompile alarm, doctor checks fseventsd + swap) |
| Continuous fleet build check (`check --tests` per change, memory-capped); limits warned at 85 % on edit | H95 broken board (staged), H98 preflight (staged), H90 memory preflight | **H114** |
| One shared repo daemon per machine (tooling held ~4 GB of 16 GB) | resident graph service exists; per-agent MCP/CLI processes | **H115** (+ stuck-operation watchdog, cancellation-first law) |
| Tests that assert they observed something; behavioural tests instead of source-grep tests; fewer, larger test binaries per domain | — | **H116** |
| No anonymous kills | **Done** H70/H76/H88 | — |
| Long runs survive tooling edits (apply to next step) | H103 s1, H80, H10 r3 | covered |
| Atomic multi-file edits; affected tests; typed errors; fleet by default; ticket close owns 🗑️generated; capacity check before builds; self-repairing cache | `repo land` live; H92/H91/H98/H100/H90 staged in the train | landing train H101 |
| Stale-build detection at activation; native replay of browser faults; typed wasm panic faults; private dev server per agent | proposal cards already offered | cards |
| Save/load contract checks from one declaration; speed floor benchmark for a real plugin's first open | — | new proposal card |
