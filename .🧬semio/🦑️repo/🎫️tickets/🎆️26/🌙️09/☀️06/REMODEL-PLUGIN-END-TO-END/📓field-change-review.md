# Field-change draft review

Indexed replacement lines 34-35 quote requested element count but call Vec::try_reserve_exact before checking actual capacity. A permitted allocator may return a larger capacity, exceeding the admitted capacity before the refusal is detected. The owner retains custody and reports actual effects, but admission still needs an exact-layout or otherwise truthful allocator contract.

Scalar equality failure now propagates the enclosing retained receipt through OriginalRetirementFailure. Native compilation and runtime remain unverified.

Current draft bounded reread: record field construction retains the actual child and receipt before reporting a marker mismatch; indexed exact backing distinguishes no allocation, failed allocation, and actual allocated bytes. No new concrete defect was established in those explicit turns. Nested equality accounting and Energy quote-time inspections remain unverified. No native test result is claimed.

Energy root-link equality reread: for fields 1–7 advance selects both text fields before checking child work demand and physical admission, but returns zero work when either denies. Selection is explicitly counted on successful paths; consistency of denial accounting requires resolution. No native execution is claimed.

The comparator now reports selected-field work on later denial, resolving the previously identified zero-receipt path in its advance body. Its parent still projects sources in quote calls before later physical denial reports zero work; the parent boundary remains open. No native run is claimed.

Design recommendation: use a source-free fixed upper bound and coarse physical check to admit the parent quote stage under the actual caller work limit. Count actual source inspections and report a work-only turn with zero physical progress when the later full grant fails; subtract actual quote work before invoking child. The real enclosing issuer must reserve and commit that work, including refusal/error paths. If existing interfaces cannot admit and report this stage, add an explicit retained quote phase rather than manufacturing a loan.

Boxed record birth reread: constructor composes T equality cursor, equality-owner four fields, two lazy field preparations each with a four-field clone cursor, empty value/retirement and four booleans, plus field/record wrapper fields and one allocation. The precise 37 fixed-operation convention could not be independently reconstructed from constructors alone; no concrete underquote established. Composable owned constructor work constants would make the quote auditable.
