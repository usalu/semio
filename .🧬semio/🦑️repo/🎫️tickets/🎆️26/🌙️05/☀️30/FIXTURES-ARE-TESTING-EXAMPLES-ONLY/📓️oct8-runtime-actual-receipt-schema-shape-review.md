# Actual Runtime Receipt Schema Shape

Native-build writer at 227–282 retains raw Cargo compiler messages. Their extra reason/executable/fresh/profile/target fields must be admitted by raw message/target/profile schemas. runtimeTarget/runtimeWorkspace are added only by verification graph projection at 422/460, not raw producer capture at native-build 345 or relay 21. Make them optional nullable on shared CompilerInput, or separate raw/projected DTOs explicitly.

Actual producer nullability: depInfo.baseDirectory may be null; input.kind and input.sha256 may be null; artifact.sha256 and optional stagedSha256 may be null; build-resource sha256 may be null; compiler-resource text/sha256/producerUnit/callerUnit may be null. Compiler-resource records additionally retain raw producer/caller identities. Include those fields before closing normalized envelopes. Current TS string-only declarations understate these refused/unresolved evidence states.

Observation includes version plus invocation manifest/cwd/command/args/buildDirectory/builtAtMs/status/cancelled/units/buildScripts, then observedAtMs/invocationInputs/buildResources/compilerResourceRoot/compilerResources. buildDirectory and compilerResourceRoot are nullable. Preserve consumer-supported optional invocationInputs/buildResources where intended, but verify current producer's always-emitted forms independently. Closed normalized unit/resource envelopes require complete declared fields; raw Cargo payloads need additionalProperties true.

Verification duplicates RuntimeTrunkObservation at 81 and RuntimeActorCargoSelection at 238. Replace with imports and explicit first-party reexports from canonical schema implementations, preserving consumers. No source changes or producer run.
