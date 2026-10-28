# Native MVD Preparation Independent Plan

Planning source checkpoint: current Direct `direct-work.rs` stages0–4, `concepts/population.rs`, shared source/index and the already reviewed Direct clause. No production edits, no native type/runtime proof. Coordinate Terminal's actual request-index lookup requirements before finalizing the shared interface; copying Direct's whole wrapper would recreate ownership/control logic.

## Exact reusable cut

A single internal preparation owner can hold: original endpoint binding/state, first-turn unborn snapshot Arc, unborn/request retained-source slots, request uniqueness index, and the existing IfcMvdPopulationWork<R>. Move only Direct stages0–4 into it. Concrete wrappers then own preparation plus their local body/Direct/Terminal clause and report donation state. Do not move domain report/diagnostic/semantic phases into preparation.

Stage0 validates cheap typed count limits, binds exact caller/source/request/epoch, retains and pays one snapshot Arc **before** fallible request-source birth/posting, and retains any born request error alias. Stage1 advances the real request index under original child demand. Stage2 validates each actual request row/domain and rejects duplicate keys via the completed index. Stage3 calls population.receive_donated to move the same Arc/request donor before posting. Stage4 advances the single graph container/index preparation. Source fields are never read before stage2 completes. Retaining an Arc header is not graph inspection.

The owned request key index is distinct from the source ID index; neither is a redundant graph clone. One source index lives solely in population. The request index remains useful after donation: Terminal discovery maps owner IDs back to request target ordinals. Keep its actual completed identity and source request pin available rather than rebuilding a HashMap or source index per clause.

## Narrow closed request policy

Existing IfcSourceIds already provides `source_len` and fallible canonical positive-u64 `source_id`; existing IfcSourceIndex provides original demands/advance/identity/find. Reuse this implementation and one shared trait/type definition rather than including a new module that accidentally creates nominally distinct parallel index APIs. A crate-private sealed request policy can extend that key view with bounded row validation. Its surface needs count, key and `validate_row(row)` only (or a retained bounded row-validation child if a future request genuinely needs more than fixed words). Domain code supplies immutable typed fields; do not accept a caller callback, arbitrary dynamic validator or graph reference.

Direct key is segment ID; row policy validates exact horizontal/vertical/cant kind, with typed boolean applicability. Terminal key is layout ID; same exact kinds, boolean/null applicability. Body key is representation ID; boolean/null applicability and closed static Body policy belong to the concrete wrapper. The typed request does not require general reflection/JSON copying. No whole request clone, default applicability, inferred ID or canonical-hash authority is introduced. Static validator/profile identity should be pinned if the implementation permits multiple projected policies for one request type; a closed generic type otherwise supplies immutable policy identity.

## Borrowed route and retirement contract

Expose prepared/unavailable/pending status by delegating to the existing population. `with_view` must retain actual shape+index proof, not a generic ready bit. `with_request` is only request authority and carries actual status: unavailable clause entry must insist Unavailable, while prepared entry pins exact index/rows. A request-index view for Terminal should be one-turn borrowed and carry actual index identity with the same original source/request/type/epoch/caller binding; it must not mint graph authority. HRTB prevents lending borrowed refs as an asynchronous result, while Copy pins permit continuation checks. Stable boxed index/held source prevents pointer reuse during parent moves.

Every parent transition includes its own original metadata quote/debit, and delegated turns use zero parent copied-items so child posts physical work once. Birth-demand types must be explicitly converted into full RetirementDemand. Error owners and child receipts/context posting refusals remain retained; no `?` may discard a locally born source/request/index. Denied first receive must leave Arc counts and donors stationary. Cancellation closes only under original retirement scope; expiration uses retirement_should_yield for closing actions. Retire clause/query/output and request-index continuation before population/source/request, including interrupted active close owners. A child Complete is not terminal while its owner slot remains.

Keep output take domain-specific: preparation authenticates the same borrowed route/caller, clause take checks complete result and empty recipient, and wrapper records donation under its actual original quote. Generic preparation readiness alone must never publish a report.

## Required extraction tests

Freeze Direct's real reports before/after, all existing target/caller/axis/cancel/close laws and exact source pointers. Exercise all three request policies through one preparation implementation, malformed row before graph access, duplicate key, rootless empty/unavailable population, parent move and stable request/source index identity. Assert first accepted request turn adds exactly one snapshot alias through request-validation failures; receive_donated causes no second clone; receipt failure preserves both source/request donors; cancelled prefix closes before actual source owner retirement. Terminal two+target source-order mapping must use completed request index identity. Body family helpers must be paid/pinned children, not alternate source readers.

This is one reusable lifecycle cut, not a new whole-MVD orchestration or view-conformance layer. Native execution/capability/Store/publication remain false until genuine tests can run and actual integration is complete.
