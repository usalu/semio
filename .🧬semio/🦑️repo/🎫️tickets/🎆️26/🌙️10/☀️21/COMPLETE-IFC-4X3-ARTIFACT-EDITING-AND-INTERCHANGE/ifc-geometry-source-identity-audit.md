# Borrowed Geometry Source Identity

Read-only source audit. A stable original-source token is appropriate as an additional cursor guard; it must not be presented as ownership, immutable revision authority or lifetime proof.

Use native source_identity from std::ptr::from_ref(the original captured document) as usize, and TS sourceIdentity as the original captured document object. Recreating reader adapters then preserves identity. Do not use adapter address (temporary eachturn), address of a moved inline document, Vec data pointer (empty allocations can share dangling pointer), an entity ID, or graph length alone. The mounted captured snapshot/read keeps original document allocation alive until controlled close; sourceVersion/canonicalrevision/generation continue independently. Without custody, address reuse creates ABA and a pointer does not stop mutation of same object.

Existing first-party analogues:

- 🌱️value/♻️retirement/🔗️arc-custody/📦️original/🦀️.rs SharedOriginalStorage<T>::identity and SharedOriginal<T>::identity expose original payload allocation address under counted immutable aliases. This is concrete allocation-backed identity, not a random ID.
- 🌱️value/📸️snapshot/👁️read/🦀️.rs SnapshotRead::lease_identity returns registry identity, slot and lease generation. This identifies a particular admitted lease and is intentionally different across new leases to same snapshot. commit_authority_matches(generation,revision) checks current Store authority. Generic geometry need not require exact leaseidentity; owner can validate original lease separately while graph identifies same captured document.
- AppRenderOperationContext matches complete appinstance/base/generation/canonicalrevision; borrowed GeometryGraph need not import platform context or Store types.

Keep interface first-party/domainneutral: TS readonly sourceIdentity:object and Rust fn source_identity(&self)->usize. Record firstidentity on initial admission and compare everyadvance, alongside standard and sourceversion. Required identity is preferable to default0; accepting default0 defeats distinct-source guard. Same source object with versionchanged must fail. Different source object with sameversion/count must fail. New adapters over same source must succeed. Invalidsource on pending or completed read should not silently preserve stale resolved scale.

Current LengthUnitWork owns no graph borrow in its Rusttype; it reborrows graph eachadvance. Document this exact contract. If stronger compile-time binding is desired, LengthUnitWork<'a,G> stores &'a G, but readerwrapper regeneration then changes integration shape; do not claim lifetimebinding merely from pointertoken.

High retained geometry executor notified to coordinate one shared graph seam rather than parallel identity APIs. No production edits.
