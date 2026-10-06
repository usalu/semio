# Retained Surface Physical Field Inventory

The exact Product3 capture and Product4 capture retain identical declarations for RetainedSurface, RetainedSurfaceSlot and RetainedSurfaceRegistry. Full owning renderer source hashes are f996e0f6955db459b56e4ef94542e018ea1430d6866cc5b279cb953a870eed76 and f996e0f6955db459b56e4ef94542e018ea1430d6866cc5b279cb953a870eed76. No field declaration change between these two captures explains the earlier committed 23496 expectation versus the measured 24488 element size. The historical expectation is not an independent old compiler measurement.

RetainedSurfaceRegistry stores one boxed fixed array plus next_generation, generation_exhausted and close_cursor. Its element is Option<RetainedSurfaceSlot>, which stores generation, instance, SurfaceId and the inline RetainedSurface. The inline owner contains state, patch, two fixed KernelUiPatch lists, a document build, three document leases, a patch receipt and two booleans. Nested native type layouts, enum niches and padding can contribute; their individual sizes require actual compiler measurement. Resource capacities and limits remain unchanged.

## RetainedSurface

Declaration SHA: 750f5522e6567467bb8643b19a68c83ff29547ce13530feb48abeedf3b6d4085. Product3/Product4 byte identity: true.

```rust
struct RetainedSurface {
        state: Option<UiSnapshotState>,
        patch: Option<RetainedPatchApply>,
        queued_patches: UiFixedList<KernelUiPatch, UI_DOCUMENT_LEASE_SLOTS>,
        rejected_patches: UiFixedList<KernelUiPatch, UI_DOCUMENT_LEASE_SLOTS>,
        build: Option<RetainedDocumentBuild>,
        build_pending: bool,
        published: Option<UiDocumentLease>,
        closing: Option<UiDocumentLease>,
        exchange_closing: Option<UiDocumentLease>,
        patch_close_complete: bool,
        /// 🎫️ The authority the guest issued with the patch this surface is currently applying —
        /// `TurnResult::ui_patch_receipt`, which `ActorUiPatchReceipt::validate_pairing` pins 1:1 to
        /// the turn's single patch. A rejection has to echo it back on `Event::PatchRejected`.
        patch_receipt: Option<semio_framework::kernel::ActorUiPatchReceipt>,
    }
```

## RetainedSurfaceSlot

Declaration SHA: a3795bfbba29938320112a3b396eb5d91be708def69f04e7c74d0610282439ed. Product3/Product4 byte identity: true.

```rust
struct RetainedSurfaceSlot {
        generation: u64,
        instance: u32,
        surface: SurfaceId,
        owner: RetainedSurface,
    }
```

## RetainedSurfaceRegistry

Declaration SHA: 25609daa4ac4929f69466aba5afbeb1dd46e24970579403e54b746c97657ca53. Product3/Product4 byte identity: true.

```rust
struct RetainedSurfaceRegistry {
        slots: Box<[Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY]>,
        next_generation: u64,
        generation_exhausted: bool,
        close_cursor: usize,
    }
```

The separate six-layout diagnostic has not produced measurements: its second run stops at a missing tracked shortcode build input. This inventory does not attribute the 992-byte delta to a particular field or relax any test/resource contract.
