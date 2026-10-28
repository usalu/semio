# Units Implementation Peer Review

Actual current TS source loaded with node:module.stripTypeScriptTypes(mode transform) into a real ticket generated module, then file-imported. Three independent minimal graph probes below executed. Own generated module directory removed. Native source read only; no compiler execution.

## Confirmed input defects

1. Conversion dimensions sparse array: Array(7) with only index0=1n is accepted and resolves0.0254. Current dimensions.args.some skips missing indices. Require Array.isArray, exact length7, and explicit dense index0..6 validation (or indexed loop). Native slice iteration covers every slot.
2. Assignment Units=[length#3,area#7,area#7] resolves0.0254 despite repeated reference#7 violating SET. Native also lacks assigned-ref seen state. Track exact assignment IDs in bounded state, independent of selected lengthunit; reject duplicates including monetary/area/unknown entries. Distinct same-scale lengthunits already correctly refuse.
3. advance with maximumWork0 and control returning cancel yields pending because control is never called. If cancellation is meant observable with no work, check control once before loop without advancing work. Define whether zero-grant is allowed to observe cancellation; do not report tested cancellation responsiveness beyond this boundary.

## Source and limits contract

Neither Work stores an actual graph borrow: native advance<G>(&mut self, graph:&G,...) accepts a different G value every turn. SourceVersion and length only detect a declared change; swapping graph with same version/length is possible. TS graph is similarly mutable. Require caller custody and immutable-version contract explicitly, or bind lifetime/sourceidentity in constructor. Do not claim cursor borrows samegraph by Rust type system as written.

TS constructor Object.values(limits).some only validates present enumerable values; malformed runtime {} bypasses required limits and undefined comparisons disable entities/references/depth caps. Validate each named field and copy/freeze limits at admission; readonly reference itself can be mutated externally. Reject unknown graph.standard at runtime if exposed to arbitrary JS callers. Typed interface callers alone are insufficient malformed-runtime proof.

## Native close complexity and ownership

HashMap.keys().next for each removed entry starts a fresh capacity scan. Draining n entries can cost quadratic bucket scans while reporting one workunit per deletion; HashSet repeated iter has same pattern (depth bounded but still repeatedscan). A BTreeMap<u64,usize> index and BTreeSet<u64> visited with pop_first provide simple bounded logarithmic removal and deterministic order without retained borrowing iterator. Alternatively retain explicit insertion IDs and pop/delete, but this adds another owned buffer and cannot eliminate hash capacity scan on physicaldeallocation. Use existing framework retained collection if original physical receipt integration is required.

Removing all entries does not release HashMap/Set capacity. Current close returns true after entriesempty while backing allocations persist until Workdrop. Thus it establishes scalar logical drain, not paid physical release/terminal emptiness. Native collections also allocate/reallocate during index.insert without original retained grant or bytes receipts; maximumWork alone bounds logical steps, not backing copy/allocation. Keep the scope honest until mounted owner uses defining controlled storage/retirement. TS Map deletion likewise has no claimed physical byte receipt.

## Numerical and parity checks

Prefix powers, positive finite product checks, typed PositiveLength aliases, IFC4-only NonNegativeLength, offset0 and2x3offset rejection match scoped contract. Native finite arrays protect sparse inputs. No obvious borrow/type error in native entity helper or slice chain identified by staticread. Exact numeric output and fullhost lifecycle remain unexecuted native.

## Minimal probe graph

#1 Project args8=#2; #2 UnitAssignment Units=(#3); #3 ConversionBasedUnit(#4,LENGTHUNIT,inch,#5); #4 DimensionalExponents=(1,0,0,0,0,0,0); #5 MeasureWithUnit(IFCLENGTHMEASURE(.0254),#6); #6 SIUnit(*,LENGTHUNIT,$,METRE). Alter #4 to sparse7tuple for probe1; add #7 SIUnit(*,AREAUNIT,$,SQUARE_METRE) and Units=(#3,#7,#7) for probe2. Both actualresults resolved project1/unit3/scale.0254. Probe3 uses unchangedgraph and workgrant0 with cancelcallback; actualpending.

## Fresh Fix Verification

Fresh actual source suite executed independently: 94 tests passed,0failed,0TypeScriptdiagnostics. Includes sparse dimensions, duplicate nonlengthrefs, zero-grantcancel, exactsourceidentity, reborrowedadapters, statusAJV,3unchangedofficialfixtures and own2vectors. Generated9module directory removed. Prior three executed defect findings are resolved. Limits are now copied/frozen with allthree namedfields, and sourceversionu64 validation present.

Native uses BTreeMap/BTreeSet with pop_first for index/visited/assignedunits close; prior repeatedHashMapcapacityscan finding resolved. Sourceidentity adapters use std::ptr::from_ref(originaldocument), not wrapperaddress. Conditional Some(identity) guard permits firstadmission then detects source swaps; sourceversion/countguard retained. All5 native include_str targets were filesystemresolved and exist (neutralfixture bothversions plus3officialfiles). No obvious native lifetime/borrow/type mismatch was identified by staticread, no compilerexecutionclaim.

Remaining scope: native BTree insertion/removal still performs allocation/release without physicalretainedbyte receipts; cursor is logical-work-controlled, mountedsourceowner integration must fund custody/retirement separately. Sourceidentity does not prevent caller mutation within same sourceobject/version; capturedimmutableownercontract remains necessary. Current tests prove scalarlookup drain and successful close, not mountedStorelease lifecycle.
