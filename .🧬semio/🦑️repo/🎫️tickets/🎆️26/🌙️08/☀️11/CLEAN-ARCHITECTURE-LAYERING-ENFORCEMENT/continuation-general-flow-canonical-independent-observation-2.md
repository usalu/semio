# General Flow Publication Current Observation

{"schema":"semio.general-flow.independent-observation/v1","terminalHash":"91e576b2b089c61bc8b772e6fb8e1a8536802e1176b1ba4a51509cabc65d7b9b","checks":{"completed":true,"twoWrites":true,"fourJournalRows":true,"modelSealed":true,"gateSealed":true,"producerSealed":true,"pairsExact":true,"journalPlan":true,"journalEvents":true,"eventPairs":true,"physicalAfters":[{"path":"Cargo.toml","exact":true},{"path":"✏️s/Cargo.toml","exact":true}],"post":true,"providerCurrent":true,"wholeRootAccepted":true}}

Observed current physical after images against exact model-1, sealed gate, plan and four append journal rows. Fsync behavior is evidenced by the sealed producer source; persistence across a crash was not independently exercised. No whole compiler or deletion claim.
