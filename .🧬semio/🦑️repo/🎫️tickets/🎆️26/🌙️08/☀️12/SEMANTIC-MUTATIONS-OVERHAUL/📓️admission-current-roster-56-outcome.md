# Current Admission Observation After Git Record Repair

The bounded real-workspace run `📓️admission-current-roster-54/🧫️run-1KTP5R` reached one complete, schema-valid observation in 38.534 seconds. The second observation was cooperatively cancelled at the unchanged 55-second deadline; no hard kill or automatic retry occurred. All seven captured producer inputs stayed stable. This is not a stable observation pair or a mutation/content census.

The first result contains 72,297 observations: 71,648 files, 34 symlinks, 16 directories, and 599 absent tracked paths. Origins are 70,879 tracked, 1,077 nonignored-untracked, 341 ignored-generator, and zero explicit-ticket; these origin counts describe provenance, not mutation counts. The 599 absent paths retain their index identities and are nonblocking observations, not proof of deletion intent or evidence loss.

The result is rejected by exactly one blocking diagnostic: `nonregular-node` at `♻️mit-bestand/recherche`. The retained observation is a physical `040000` directory with a tracked stage-zero `160000` Gitlink entry, object ID `92036c7ca0149b43ddea28db8c8e516f983fe718`. No contents of this Gitlink were inspected by this review. No index edit, source restoration, exclusion, or diagnostic suppression was performed.

This boundary is separate from the completed transport repair: frozen packet56 passed 53 reference, 33 grammar, 12 strict physical-path, 12 marker, one byte-name walk, and five isolated Git checks. Existing canonical source tests passed 36 direct and 36 package tests; IO passed nine direct and nine package tests. New N hash is `34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f`. The independent exact-delta review is [retained separately](📓️admission-untracked-records-56-independent-review.md).

Next work is to inspect the existing schema-owned Gitlink/nested-repository boundary and the global census contract. The rejection must not be bypassed by relabeling it complete, adding a skip list, reading nested content without authority, or expanding the deadline until a preferred result appears.
