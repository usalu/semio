# Owned Original Domain Continuation Design

Proposal for Root review, not implemented and not an integration seal. The existing automatic callback_refusal assignment is still a blocker and must be removed; reviewed independent cancellation, current-credit, closing-control and post-attempt CPU corrections have been authored first.

## Ownership Boundary

The body operation should consume its already-paid OriginalDomainReservation and return an owned OriginalDomainBodyReturn<T,R>. R remains Copy progress; actual owned success stays in its producer. A blocked/unentered return contains the unchanged reservation. An entered return contains that exact reservation, the exact original body Result<R,OriginalTurnFailure>, and a separate complete final qualification failure. No new allocation, cloned error, erased primary or dropped success is permitted.

The callback result must be retained independently before the existing retained/work loan returns. The loan callback itself returns only Copy success to its guard; its complete final qualification error is retained separately in the returned continuation. This prevents an enclosing Standard Result transform from replacing an already-owned callback error. A readonly body refusal with zero child CPU is returned with zero child CPU; returning its original owner is not permission to persist it in a producer field at zero cost.

## Separate Funded Insertion

The returned continuation is a genuine owned return value. A later insertion call consumes and returns that same continuation on denial, preserving all original pointers and causes. Once the same initiating source supplies actual item/depth/fuel, insertion moves exactly one pending native cause into its preborn domain slot, reports one metadata item and one actual CPU, and never copies payload bytes or allocates. Body-return refusal and final qualification have distinct fixed slots; if both exist, each selected insertion is a separate turn. R is returned to the producer only after all pending causes are inserted or the unchanged no-cause completion can be resolved.

Neither lifetime stock nor deadline changes. Header handoff reserve remains a lifetime reservation, never a subtraction from every quantum. A child may spend the sole current item/fuel and still return its original owned continuation; insertion waits for a later genuinely issued view instead of deliberately overrunning or minting stock. Foreign/invalid context insertion returns the same owner with a separate zero-effect qualification refusal; it must not write a foreign record into the initiating frame without authority. Cancellation preserves the continuation for same-original paid close.

## Retirement

Both reservation and continuation must implement canonical funded RetireOwned. The continuation owns the prepaid reservation plus the exact pending native causes. Parent retirement retains that whole owner and closes every cause/frame under its actual quote; no implicit Box drop, late dynamic envelope allocation or side storage is allowed. A new generic success API is unnecessary: only Copy progress is allowed.

## Producer Integration Prerequisite

A producer may not silently assign this returned continuation to a persistent field at CPU zero. The actual receiver boundary must either return this owned continuation to the existing owning caller, or have a separately admitted explicit continuation-receiving place. The current InteractiveJob::step Native-error boundary does not directly accept a generic owned return capsule. That seam must be made concrete before Export/Geometry/Clipboard integration, rather than hidden in a private field. Potential defining option: a domain-neutral retained continuation return arm that carries original input custody through the existing caller-owned outcome mechanism. This is not yet chosen or authored.

## Genuine Test Plan

Before body redesign: actual OriginalJobFixtureCaller; zero-child-work readonly native refusal; positive child spending its complete quantum-one current item/fuel; exact pointer/cause identity in the returned owner; separate final qualification failure retained with original callback error; foreign/invalid context insertion refusal preserving owner; denied insertion zero effects; later same-source paid insertion and publication; cancellation and parent-owned paid close. Actual heap and independent Serde cause trees remain mandatory. No fixture-only proof, renewed stock, callback default CPU, or hidden persistent assignment is accepted.
