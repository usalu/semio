# Dependency Ownership Work-Unit Attribution

Exact publisher is library/caching/bootstrap/dependencies/💾️storage/🟦️.ts:499 runJavascriptDependencyStoreProducerV1. Resource text is dependencies:store-ownership:<operation>:<completedOperations>, modeexclusive, elapsed since this producer start; it emits when completedOperations modulo InstalledPathProtocol.progressWorkUnits is0. Authored progressWorkUnits=64.

Workspace ownership Bun/🟨️.cjs increments cumulative completedOperations per yielded physical operation (:376–389), per replayed frontier record (:330), and explicit membership/readText verification (:414–419 etc). Thus suffix5112256 means cumulative ownership operations. It is neither bytes nor percentage, and does not imply5MiB text read or currentstack.

No per64byte physical read loop was found. readManifestText:63 reads min(remaining,PhysicalLimits.readChunkBytes); current authored readChunkBytes=65536B. The64 number is progresswork cadence. Storeprogress callback checks custody, publishes/yields and checks custody again; it does not itself call store.recheck each64operations.

A concrete expensive repeated path is present: Store fence:505 performs store.recheck plus custody.recheck and input/preservation/descriptor checks. Fence is invoked around preparation/installers/ready/binding stages (:507,511–516). Ownership custody recheck replays full retained path frontier and all manifest texts (:312–349), with currentFence again requiring currentmembership and re-reading texts (:364–373). The progress counter is shared across these fresh fence plans, so millions ofoperations can represent repeated complete physicalownership observations rather than one hugefile. This is a source-supported amplification route, not measured attribution of985seconds or proof of stuckcurrentframe.

The label belongs to dependency Store preparation and does not report test collection. No commands, private runtime query, DBread, signal, Source edits or PIDownership/deadlock claim. Final verifier review remains priority.
