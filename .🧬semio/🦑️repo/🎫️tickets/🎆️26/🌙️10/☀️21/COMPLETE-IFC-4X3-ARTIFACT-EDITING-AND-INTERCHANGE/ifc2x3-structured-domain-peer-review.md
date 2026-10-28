# Own IFC2x3 Structured Domain Peer Review

Read-only fresh source review of catalog/domain/core.{rs,ts},part21.{rs,ts}, own2Details/domain_edit.rs and controlled planner. Existing source-test results supplied by author are not independently rerun here; no native compilation/executionclaim.

## Exact numeric adapter

Part21 adapter borrows coefficientstrings; no conversion to IFC4 values or binary64. Decimal normalization strips leadingzeros, strips trailingzeros while increasing base10power, canonicalizes zerosign and compares significantdigits/power/sign. This correctly makes1,1.0,10E-1 equal while preserving exactdecimal coefficient differences abovebinary64 range. Inti64 unsigned_abs conversion handles MINi64 safely; fixedmaximum19digits allocation is bounded. Native poweri128 covers i32exponent-u32scale+coefficientlength. TS exponent/scale range checks mirror nativefieldtypes; JSnumber power range remains exact for feasible coefficientstrings. numbersIdentical intentionally preserves representation/sign/scale/exponent to distinguish unchanged malformedvalue from semanticnumericequality.

No definite numeric-normalization or native lifetime/type defect identified by thisstaticreview. Numeric predicate validates lexical/range structure, not aliasWHERE (PositiveLength>0 etc); report must retain structuralscope and not call itfullEXPRESS numericvalidation.

## Generic mounts and ownership

core.rs imports only parent IfcCatalog/IfcTypeExpression and exposes DomainValue borrowedviews. Own2Details mounts core.rs andpart21.rs directly under itsownmodule, importing its owncatalognominaltypes. Allresolvedpathattributes point to existingintendedsharedfiles. part21.rs uses super::domain core ofthatownmodule, so traitimplementation is the actual own2Part21Value and catalogidentity; no4snapshotconversion/adaptorintermediategraph.

Own2domain_edit walks prepared UpsertInstance contents and originalcapturedrow, keeps types/constituentcounts fixed except explicitdraftoperation, detectsderivedslots, validates changedargument andaffectedaggregateancestors. Referencecallback scans originalsnapshotallinstances andconstituents withtick, countsduplicates, requires exactlyone admittedtarget. It doesnot lookup mutationtarget against unrelatedfreshstate. Planner anddomaincheck use samecapturedsnapshot; normalizedreferenceevent is rederivedunder sameoriginaltickbeforecheck. Explicit typevariantdraft bypass remains intentionalincompleteconstruction, notcompletedinstancevalidation.

Controlled planner still performs instanceclone/ToValue/edited_subtree/FromValue aspreviousboundedone-shotgate afterinstanceprewalk. Domainvalidation itself isone-shotcontrolled: tickexhaustion refuseswithoutpublication, notcursorresumption. ifc-domain-cursor-design.md correctlyseparates thisnextgate. No newcontext or fallbackunlimitedcontext wasfound in the adapter/check seam.

## Repair and equality scope

Cardinalitydeficit/excess cannotworsen; SET/UNIQUE semanticgroupmultiplicity cannotincreasebeyondpriorcount orone. Changedleafstrictvalidation leaves unrelatedinvalidsiblingsalone. Wholeargumentreplace strictchecks completechangedsubtree unless representationidentical; thisis consistentincrementalrepairpolicy. Emptyoptionalaggregate vsrequiredaggregate distinction remains: optional ARRAYelements mustnotmakewholeaggregateoptional. Explicit unset markersarehandledseparatelyfromderived.

Tests worth retainingasfuturecursorregressions: i64MIN andadjacent>2^53 exactvalues; realcoefficientleading/trailingzeros andnegativezero; exponentMIN/scaleMAX normalizedcomparison; SETnumericduplicate1vs1.0; removeoneinvalidsiblingwithoutcheckingunrelatedsibling; nestedSETreplacement increasinggroupcount; callbackcancel duringcoefficientdigitcomparison andoriginalreferenceduplicate-IDscan; originalpendingmutation custody on everyrefusal. These remain scopedfielddomainproof, notpopulationinverse/globalWHERE ormountedhostexecution.

Main independent execution after delivery:12actualown2groups pass with zero focused diagnostics, including structured domain, repair, exact decimal, typed reference/SELECT, and existing selection/details behavior at this source state. Native execution remains unverified.
