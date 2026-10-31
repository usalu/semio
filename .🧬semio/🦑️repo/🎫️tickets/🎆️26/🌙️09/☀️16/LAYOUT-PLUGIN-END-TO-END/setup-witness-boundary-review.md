# Setup Witness Boundary Review

Independent read-only source/log diagnosis after canonical setup24933 exited1. Only owning source and existing generated/setup-live-1.log were read. No live cache scan, effects, source edits, retry, jobs or controller action occurred.

## Exact Current Bounds

Owner: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🟦️.ts` and adjacent `🧬️schema/🔣️.json`.

Protocol constants: nameWindow64, chunkBytes65536, maximumDepth64, maximumPathUnits256, progressWorkUnits256, censuses2. LogicalObservationPolicy constants: maximumMetadataWork67108864, maximumResidentRows4160, maximumLogicalBytes68719476736, maximumResultBytes16384. Source line59 constructs publicationLogicalPolicyV1 by copying LogicalObservationPolicy and adding maximumEntries equal to maximumMetadataWork, hence67108864. This is not a256/4000/32 entry ceiling.

## Count Semantics

Logical-frame line79 initializes `count=0` inside each page iteration, immediately before that page opens and enumerates its directory. count increments once for each non-null returned entry, before cursor filtering. It therefore counts all names in that directory during one enumeration pass; it is neither a global tree count nor accumulated across pages. Old names at/before cursor are counted on each rescan, but in a newly reset count. They do not accumulate toward the count predicate across pages.

Separate counters are cumulative: entries increments once per recursive visit at line76, while work increases for every charged operation across the logical frame at line65. Repeated directory scans do consume cumulative work, so paging can exhaust that finite work budget. That would throw `Logical publication metadata work exhausted`, not the observed name-validation error. Because step charges before each read and charge() runs again before count++, a count above maximumEntries=maximumMetadataWork cannot be reached within this same current implementation without earlier work exhaustion. The unsafe-integer count condition is likewise unreachable under the much smaller work ceiling. No per-page count-accumulation bug was found.

## Actual Refusal Predicate and Unknowns

Current line79 refuses when count is unsafe or greater than67108864; name is not a string or empty; name contains backslash, slash or NUL; name equals dot or dot-dot; or paths.join(current,name).length exceeds256. The final path test counts the full joined absolute path in JavaScript string units, including the actual root prefix, not only the child name or payload-relative path.

The generated log prints this exact predicate and `Closed logical publication metadata name required`, but does not render the error cause object. The cause authored by current source would identify reason=count/type/empty/separator/navigation/path plus bounded directory/name/count/pathUnits; those fields are absent from the inspected terminal output. Consequently the actual rejected entry, actual full path, exact offending name characters, and concrete branch cannot be established from the log. The source-budget relationship makes the count branches implausible for this implementation, but it does not provide the missing observed entry facts or justify claiming a specific path failure.

This review does not substitute a filesystem crawl to recover them. No cap increase, entry skip, witness bypass or detach is proposed.

## Publication Stage Evidence

The complete existing setup-live-1.log contains one native-fixed-image accepted stage and one observed stage. It contains zero witnessed, detached, staged, published, attached, verified or settled native-fixed-image stages. The terminal log shows the metadata-name error, followed by OwnedCommandFailure status1/signalnull.

Owning publication `🎛️owner/🟦️.ts:79` awaits captureNativeImagePublicationWitnessV1 in bounded-logical-observation mode before returning the before witness. Publisher line88 sequence emits witnessed only after that payload capture, and performs detach afterward. The observed error is inside the preliminary logical frame (witness source24 and79), before either content census and before witness return. Thus source ordering and log agree that this failed before the witnessed/detach stages; no successful detach or publication is evidenced.
