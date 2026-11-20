# Observation collection memory reporting

The slice worker's observation diagnostic multiplied the combined entry count of live scopes by the mean size of entries sampled from the largest scope. A fixture with 3-, 80- and 5-entry captures and the existing module-load map reported 90 entries beside that one map's sample. The differently sized rows made that estimate unsuitable for the combined collection.

The diagnostic now uses the largest map's entry count and sample for one estimate. The label states the number of tracked maps and their combined entry count. One pass over weak references identifies the largest map and both counts. The existing bound of 16 sampled entries applies, and no extra passes visit the map contents. Spilled captures lack a cheap mapped entry count and are excluded from the map label and estimate. The change is confined to the diagnostic and leaves the cache record format and graph semantics intact.

The language-neutral mixed-size fixture exercises the real capture and final collection reporter. It checks that all sample rows come from the 80-entry map and that an active spilled capture does not change the map estimate. An independent SQLite query checks the sampled byte estimate, largest entry count, sample limit and combined entries.

The slice collection, read-step and final-progress suites passed 22 tests and 223 assertions. Scratch was removed and no live graph read was started.

## Unsampled weak collection records

The coordinator heap census reported 21,641,787 WeakRef objects while 4,224 of 4,791 whole-graph owners were complete. That census does not attribute instances to individual constructors. A separate private process exercised 4,096 real source captures with no diagnostic consumer. All 4,096 additional WeakRef objects survived explicit garbage collection because the reporter kept each wrapper in a strong set until somebody requested a report. Sampling the old reporter released those wrappers. The observation and closure-verdict reporters now use a FinalizationRegistry to remove each dead target’s tracking entry even when no report runs. Live targets remain iterable, and sampling still prunes dead entries promptly. In a fresh process the same 4,096 captures retained two additional WeakRef objects after collection; Python independently verified weak collection release. Forty-two adjacent tests passed with 25,686 assertions. The active coordinator read predates the fix. Its total weak-reference count cannot be assigned to this single defect.

## Census timing

The pressure gate attached the footprint from before collection to a census taken after collection. A coordinator relief event fell from 9,295 to 9,094 MiB but labeled the resulting census 9,295 MiB. The gate now labels that census with the refreshed post-collection footprint. A bilingual fixture with an independent SQLite measurement lookup reproduced the old result and verifies both translations. Eleven footprint tests passed with 62 assertions. The original relief line already preserved both measurements.

## Replacement physical read

Near 4,255 of 4,791 settled owners, the fresh coordinator read reported 176,951 WeakRef objects among 12,144,380 objects, 3,354 MiB managed and 2,045 MiB external. The stopped read near 4,252 owners had reported 26,645,228 WeakRef objects. The replacement used child workers, updated code and a different cache population. These physical counts establish a substantially smaller reported population on the new run without assigning the entire difference to the isolated diagnostic fix. Neither census is a peak across every process.
