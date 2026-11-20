# g1-m06 inventory owner progress

The live g1-m06 receipt showed the same 301,380-observation high-water at 275,890 milliseconds with 177 completed scoped-discovery checks and at 435,656 milliseconds with 1,185 checks. A later receipt at 20:04:24 UTC on November 19, 2026 showed 2,399 completed checks at 707,953 milliseconds. Its progress text still contained only the unchanged observation count. The ready step watcher uses changes in the phase text as objective movement and strips elapsed time from its step identity. This gave a slow inventory a path to the idle cutoff despite completed work.

The inventory progress line now includes the measured high-water of completed owner checks in English and German. A check counts when scoped inventory's owner callback returns. One owner can require another check for a different target or input revision. The progress renderer reads the live discovery result or its saved completed row during a later await, within the existing bounded diagnostic receipt. The high-water is owned by the inventory frame and a new count is credited only when it increases. Its label uses `completed owner checks` and `abgeschlossene Projektprüfungen`.

The translated step and its existing elapsed-of-bound suffix still enter the readiness watcher with their original duration fields. The same completion count carries through memo publication. Repeated, lower or missing samples preserve the identity, including when the elapsed warning changes. The existing quiet deadline can expire on a constant count. The separate graph lease activity clock still follows its own established inputs.

The neutral replay holds Source observations at 301,380 across checks 177 and 1,185. SQLite independently calculates the expected high-water. Both language forms credit the increases, let repeats and regressions go quiet and reach overdue at the existing limit. A real inventory-closure fixture exercises two completed inspections for distinct revisions of one owner. The running progress aggregator emits the same qualified step after discovery hands off to memo publication.

The combined inventory, progress, slice receipt and readiness watchdog suites passed 49 tests with 876 assertions. Private process fixtures exercised the existing overdue stop and proved that a bounded presentation can carry a complete step to the watchdog.

The live attempt in the captured receipts used the former phase label. The new qualifier’s uptake requires a later worker sample.
