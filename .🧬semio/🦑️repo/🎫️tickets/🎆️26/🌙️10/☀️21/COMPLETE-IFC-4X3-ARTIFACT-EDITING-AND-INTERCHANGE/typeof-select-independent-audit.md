# Independent TYPEOF SELECT Counterexample And Closure Audit

## Counterexample Is Now Authenticated By Primary IFC Semantics

The prior retained-express-type-inverse-audit.md correctly left general SELECT closure as an unauthenticated normative candidate. New primary evidence establishes a real released IFC validation failure: official IfcFillAreaStyle MaxOneColour limits colour assignments to one and expresses that constraint by querying IFCColour membership in TYPEOF. The pinned EXP lines 6769-6772 contain that same rule. IfcColour is a SELECT containing IfcColourSpecification, and IfcColourRgb is its subtype. [Official IfcFillAreaStyle](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcFillAreaStyle.htm), [official IfcColour](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcColour.htm).

Independently read root's colour-select-probe.ts, generated one/two-colour STEP fixtures and generated/colour-select/reference.json. The two-colour source uses two distinct valid RGB entity instances, so SET uniqueness does not erase them. Baseline full reference validation reports complete=true, valid=true and MAXONECOLOUR=true for both one and two colours. The one-colour case is the positive control; the two-colour case must fail that named rule. This authenticates the concrete conformance bug without relying on translated ISO text or a shared incomplete implementation.

Root's debug summary originally looked for mixed-case MaxOneColour while saved rule names are uppercase; its saved reference JSON has the correct named results and is the evidence audited here. No source correction was made by this reviewer.

## Independent Third-Party Metadata Boundary

Executed ticket colour-select-metadata-audit.py via its small MetadataOracle interface. Actual py3.13 resolved IfcOpenShell 0.8.5 and recorded generated/colour-select/independent-metadata.json. It independently confirms RGB ancestry includes ColourSpecification, IfcColour's direct selection includes that declaration, and FillStyleSelect includes Colour. This is metadata evidence for the primary rule's expected category membership.

That environment's actual TYPEOF helper omits SELECT names and is explicitly marked helperIsNormativeOracle=false. Root separately retains an IfcOpenShell 0.9 environment with the same reported helper issue. Neither environment's erroneous acceptance can override official rule semantics. Upstream issue/closed PR metadata is corroborating implementation context, not normative evidence or proof that a fix is shipped. No native-produced semantic output exists.

## Stable TS Closure Source Review

Reviewed evaluator.ts typeOf helper at approximately lines 94-105 after root's stable checkpoint, plus neutral typeof-select cases/schema/tests. It starts from existing actual runtime type seeds (or primitive names), builds a reverse member-to-SELECT map from the released catalog, then walks containing SELECT parents to a fixed point. It does not infer arbitrary named aliases merely because they share primitive REAL/STRING domains. Existing qualified partial-entity seeds are not expanded through a newly invented ancestry traversal.

The map is per evaluator. Construction uses a local map and tick checks per declaration/member, and installs the cache only after complete preparation. Cancellation/work refusal cannot expose a partially built cache as success. Closure expansion ticks for each seed/parent and uses a seen set to bound cycles/duplicates. Existing source/runtime types arrays are copied before extending names. The static Object.values catalog roster remains an eager finite metadata allocation; this is reference TS scheduling, not native original-wallet proof.

A direct-call normalization edge was reported to root: raw seed dedup precedes schema-prefix normalization, so simultaneously supplying bare and IFC4X3_ADD2-qualified versions of the same type may emit duplicate qualified strings inside the nominal SET. Normal producer seeds inspected here are bare; no pinned model failure from that edge was established. Root is adding direct/qualified/source-unchanged regressions and can normalize final names without mutating input seeds. This report is not a claim that those follow-up edits were already reviewed.

## Tests And Remaining Native Work

Root reports the new 17-test source/neutral suite progressed from 9 pass/8 fail to 17 pass/25 assertions; one/two-colour full reference models now produce their expected named outcomes. This reviewer inspected the tests and saved baseline/metadata evidence without duplicating the root suite. The tests include work/cancellation during index preparation; root is extending qualified seed/unchanged-source coverage.

The retained native TypeOf source reviewed earlier lacks this closure, so its corresponding false outcome remains a source prediction until authentic native execution. The native worker owns a separately retained, original-funded fix and tests. Do not copy the TS cache into native as a cold hidden traversal, mutate canonical arrays/order, enable template semantic admission, or claim native receipt correctness from TS success. Full native compilation, semantic outputs, cancellation/close/receipt conservation and registered publication remain unproved.

## Native continuation follow-up

Read-only review of `machine-population.rs` TYPEOF stages 4–8 and `machine-typeof-tests.rs` found no new confirmed defect in this narrow closure addition. Stage 4 retains an edge index, then reads its live successor after closure appends; therefore newly discovered SELECT values enter the same finite queue. Stage 5 scans one immutable declaration/member at a time; stages 6–7 copy and deduplicate a candidate through the existing retained tasks before appending. Stage 8 advances the declaration rather than repeating its matching member. Initial runtime seeds are collected before `frame.a` becomes the current closure seed. No new primitive-to-defined-alias inference is introduced.

The shared machine advance path checks environment identity before mutation, obtains the five-axis demand before work, separates table growth from semantic work, and retains the existing controlled close. The authored native tests cover thirteen ordered neutral expression rows, the one/two-colour full population rule, foreign source rejection, denied positive demand axes, and cancellation at each stage 4–8. These are source assertions only: authentic compiler ingress still refuses before rustc, so neither type checking nor runtime custody/semantic success is established.

The current TypeScript helper now deduplicates its formatted qualified output as well as its traversal keys, resolving the earlier mixed bare/qualified duplicate observation. Its locally constructed reverse index is published only after all cooperative construction ticks finish. This review does not approve the entire retained EXPRESS engine or enable its admission gate.
