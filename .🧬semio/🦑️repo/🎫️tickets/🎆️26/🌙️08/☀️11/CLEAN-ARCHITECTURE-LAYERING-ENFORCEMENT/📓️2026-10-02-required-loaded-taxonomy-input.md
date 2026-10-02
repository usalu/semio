# Required Loaded Taxonomy Input

Read-only source inspection; no jobs or edits.

## Actual construction and consumers

Taxonomy owner417 currently exports LoadedTaxonomy with optional input. Private parseTaxonomy478 returns that same shape with path/matcher/schema facts but no input (1227–1236). Only live production parse call is loader1258. Loader always captures nonnull SemanticOwnedInputFileSnapshot, verifies absolute witnesses, and returns input1264. Optionality reflects internal parser convenience, not actual public loader behavior.

Actual input consumers: source admission collect242 checks path/input and292 publishes input.contentHash; normalization current-source authority4294/4304 requires captured schema/catalog receipts; generator current-source comparison5392 checks input.path/hash/size/bytes. Those receipt equality guards remain meaningful even after static input required: making a field mandatory does not prove matching source bytes or catalog authority. Other normalization helpers only consume schema/exclusions/matcher; GeneratorInputTaxonomy is already a narrow Pick and does not need physical input.

Coherent cut: private ParsedTaxonomy owns path/matcher/schema/discoverySchema/exclusions/fileKinds/directoryKinds, no input member. Private ParsedTaxonomyFacts owns content-derived fields only, no path/matcher/input. Public LoadedTaxonomy includes same readable facts plus mandatory path/matcher/input. Use one facts interface shared structurally if desired, not a legacy alias permitting absent input. Cache Map<string,ParsedTaxonomyFacts>; parse returns ParsedTaxonomy, loader destructures path/matcher only and attaches actual required input. Never export parse-only shape as loaded authority.

Update parser destructuring `_input` removal (no input exists on ParsedTaxonomy), cache type, public interface and optional chaining in direct loaded tests. No need to broaden GeneratorInputTaxonomy, or construct fake input for ordinary schema-only helpers. Existing type-only normalization LoadedTaxonomy function parameters can stay where actual loaded object is supplied; purpose-specific helper parameters may narrow to Pick rather than weakening global loaded contract.

## Harness impact

Taxonomy-pattern compiler test146 structurally locates LoadedTaxonomy pathMatcher member: preserve that directly declared field if retaining exact law shape, or own an explicit updated inherited-field structural law. Its local Loaded type197 already requires input, so no semantic weakening needed. AST helper strips/imports declarations; new private interfaces are included automatically in combined text but compiler-selected type closure must include any new names if strict typing checks inspect parser return.

Taxonomy-input test41/54 can read first.input.contentHash/next.input.contentHash directly. `loaded?.input?.contentHash`104 can drop only second optional chain because loaded remains optional until assertion. Witness cache setup destructures required real input from first actual load; no fake snapshot introduced. IO compiled fixture adapter supplies `{path,input:{contentHash},...}` intentionally as isolated stand-in; preserve these bounded test-only injections but do not expose public loaded construction callback. Current-source fixture types/schema must retain actual required bytes/mode/size/path receipts, not merely contentHash.

## Landed eight witness rows

Test67 now registers each of four witness actions separately for Bun and TypeScript (eight laws) with original5s law budget. Exact environment includes actual verifyNoFollowDirectoryChain; semantic capture happens once, intended acceptance/refusal is asserted before independent Node physical kind, avoiding oracle scheduling cost obscuring regression. Input verifier66+ rechecks present ordinary directory/nonlink dev/ino/mode and deliberately ignores timestamps; taxonomy1250–1253 retains chain and verifies after capture, before cache lookup. No missing free helper dependency found in this witness environment. Matcher isolated declaration list185 includes new verifier and chain.

Residual authority limits: exported collect accepts structurally constructible LoadedTaxonomy, so mandatory field alone is not an unforgeable receipt. Preserve actual path/content/input equality validations. Root witness snapshot narrows observed ancestry replacement, not atomic directory-handle traversal/ABA guarantees. No passing claim for current eight rows made by this audit.
