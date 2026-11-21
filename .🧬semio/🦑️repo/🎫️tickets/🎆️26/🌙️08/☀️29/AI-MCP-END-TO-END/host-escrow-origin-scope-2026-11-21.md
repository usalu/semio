# Host escrow origin scope

After the escrow reclamation fix, native Note and WFC reach and settle Input receipt 4. Both later fail in issued-origin validation before applying the ledger. The failure witness for native Note identifies a new Infrastructure origin at epoch 577722. Its copy ceiling was 1,471,583 bytes. The header yielded 2,199,043 bytes of fresh copy and the current snapshot added 2,295 bytes of held copy. This origin had zero prior held and redeemed amounts.

The producer and host share the conservation equation for a single issuer. Fresh effects, cumulative redemptions, and remaining held currency must fit the issued ceiling. A release returns unused custody and reduces the held position. Snapshot retirement removes settled rows with zero outstanding currency and leaves the issuer's accumulated redemption intact.

The guest reactor generated the fixed receipt from its resident actor-lifetime progress counters. The start offset was zero even though the actor had preserved physical spend from earlier receipt dispatches. This caused the same historical copy to reappear in successive receipts until it exceeded a later loan. Each receipt now uses the actual pre-dispatch baseline bound by the same Source before birth and refresh work. The encoded aggregate includes observed reserved-return effects and the exact header completion demand. The host uses validated SRE redemptions to derive the fresh spend once and still enforces the actual issued ceiling on fresh plus redeemed and held amounts.

The host now matches the fixed header's input fields to the exact issued descriptor and validates all aggregate spend axes after the authenticated redemption split. A prior redemption can exceed the current capacity or release quota. Only genuinely fresh effects use that quota. The fixed receipt also preserves genuine overage for refusal.

The shared receipt fixture covers resident baselines, previously funded redemption, bootstrap carry, counter regression and genuine overage. Four Bun tests passed across 28 assertions. The Rust receipt test uses the same fixture. The temporary numeric log was removed after diagnosis.
