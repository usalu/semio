# Release Roster Pair Expansion

The 32-root retained Cargo plan supports five additional two-root batches. Each proposed pair comes from separate component workspaces. The current production planner returned exactly ten wasm32-wasip2 release passes, one per proposed component. The pair wrapper sends the two selected roots into the same driver invocation, so the two ordered passes use one crate-set lane and one compiler-image headroom admission. Each root keeps its own link, component stages, descriptor check and confirmation.

The audit recursively hashes every Cargo unit's package identity, target, mode, feature set, complete profile and dependency edges using the predecessor's signature in place of the graph-local index. It compares each root's reachable dependency multiset and excludes the root component itself. Retained graph signatures establish a reuse opportunity. Cargo still checks actual compiler inputs, build-script artifacts and fingerprints at execution.

| Pair | Total unit counts including component root | Shared dependency units | Distinct dependency units | Share |
| --- | ---: | ---: | ---: | ---: |
| Flow Draw + Flow List | 812 / 812 | 810 | 812 | 99.8% |
| Flow Logic + Flow Primitive | 812 / 812 | 810 | 812 | 99.8% |
| Process Concrete + Process Robotic | 911 / 911 | 909 | 911 | 99.8% |
| Flow Brep + Playbook Procedural | 917 / 927 | 916 | 926 | 98.9% |
| CAD AEC Building + CAD Spatial Shape | 767 / 682 | 680 | 767 | 88.7% |

The pair's shared units comprise nearly all dependencies of the smaller root. The current production lane-pool snapshot had two dynamic lanes and no exact crate-set affinity for any of these pairs or their ten singletons. Previous Draw, List, Logic, Primitive and the two CAD singleton logs reported zero reused Cargo artifacts in their captured runs. The Concrete, Robotic, Brep and Playbook Procedural singleton attempts previously stopped at the spatial-kernel session source limit. The subsequent roster source preflight passed that source boundary. Elapsed pair time remains unmeasured.

Release logs show why grouping helps while headroom effectively serializes writers. The cold build prior was 14.4 GiB with 2.9 GiB reserved. Demonstrator waited 886 seconds for its grant, and GIS waited behind the remaining projected Demonstrator writes. The driver ownership lease itself is shared. The headroom budget governs whether two commands can actually write concurrently. A paired driver continues to admit each pass against the live predicted retained footprint, reclaim obsolete outputs when needed and preserve the next pass's dependency identities.

A cross-workspace pair shares the driver's failure lifetime. Its two-pass plan cannot use the single-pass warm pre-link churn retry. If either pass fails, the wrapper discards pair confirmation and submits its bounded singleton attempts. If fresh singleton lanes exist when the new supervisor takes over, the pair can cost more by using a separate crate-set key. The retained graph comparison and lane snapshot do not predict such a later change.

Sourcing Windows has the same 894 common dependency units with either existing Sourcing pair member. With two roots per batch, one Sourcing extension still needs its own run. The Windows and AEC Building cross-family option shares 738 units, but has a 923-unit dependency union versus 767 for the selected CAD pair. The selected pairing leaves the larger standalone Windows workload separate while reducing the retained footprint of the paired lane. Grouped hub Process and Sourcing show 886/1,159 common units in the retained all-hub graph. Those counts come from the broader feature-unified hub selection. A fresh two-package Cargo plan and output footprint estimate would be needed before taking that pairing. Primary CAD, Procedural, GIS and Demonstrator keep their established scheduling priority and exclusions.

The new pairs follow the existing five in the ticket scheduler. A synthetic regression showed that the old scheduler launched each additional candidate singly. With the updated list it dispatches all ten expected pairs under the two-root capacity bound. A separately confirmed sibling leaves a singleton. The paired wrapper's confirmation and fallback fixture also passed. The active supervisor and children were left running. The on-disk change is ready for the release owner's next normal supervisor handover.

At 07:51 PST on November 21, the release owner started supervisor PID 51044. It acquired the roster lock under retry protocol 1 and adopted the existing Demonstrator PID 57325 and GIS PID 18897. Both queue children were still running after adoption.

[Read-only graph comparison script](audit-pair-unit-graphs.py) · [Roster scheduler](roster-pairs.py) · [Previous pairing audit](🔬️native-module-pairing.md)
