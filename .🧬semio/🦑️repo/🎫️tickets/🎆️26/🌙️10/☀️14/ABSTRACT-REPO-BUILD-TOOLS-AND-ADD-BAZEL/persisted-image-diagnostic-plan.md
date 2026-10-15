# Bounded Persisted Image Diagnostic Plan

ALL9 reached its original deadline after 794 completed project images, without a conflict or accepted graph. This plan is read-only source analysis. No diagnostic census, normalizer, observer, storage producer or new runtime has been launched.

## Identity And Existing Boundaries

Production uses `observeProjectInferenceV1` with key `[projectAuthority, registryDigest, native ? commandRevision : null, native || pipeline ? assemblyAuthority : null, root, augmentedDeclaration, selectedTargets ?? null]`. `projectAuthority` incorporates implementation revision and generator contracts. Native command inputs, full project/native-root ownership maps, component roots, dynamic target augmentation, registry and closed selected/all admission affect key preparation. A hand-picked project or its raw authored JSON is not the production key.

The persisted header digest combines observation-module revision, the sealed module-load authority digest and that project key. Acceptance also verifies the result-byte SHA and every retained observation. In-memory project counts establish neither persisted availability nor reuse. Source-current facts alone do not establish key eligibility.

No existing independent read-only exact-key preparation/probe port was found. The normal observer falls through on a miss to opening a packed record store, whose constructor creates its directory before the callback runs. A throwing callback would therefore not be a read-only probe. Adding a production export is also unsuitable for unchanged-image evidence: observation revision hashes its own full Source bytes, and inference Source contributes to implementation revision. Such a change would invalidate the images being examined.

## Proposed Stage One

Use the existing genuine original 60-second diagnostic child/factory and normal shared graph-store custody. Acquire `withRepositoryCacheStoreV1(workspace, "graph", original, callback)` through the established declared storage authority, retaining its native image identity/recheck/control. Do not enter a FactStore, selected writer, dataset, optional maintenance or producer. Normal enforced custody/accounting may perform its own admitted lifecycle bookkeeping; project cache contents are inspected only.

Obtain the actual ALL9 completed owner roots from its retained trace. Derive only their exact canonical global NP names: SHA-256 of root, first 24 hex digits, under the actual graph data directory's `inference/np`. ALL admission uses the global namespace; do not scan selected/dataset namespaces or substitute their entries. Reconcile owners against the current closed registry and distinguish a now-changed declaration from a retained trace entry. The source registry and sampled cache leaves must remain byte/stamp stable across the bounded read.

Suggested hard limits are 794 candidate names from the actual trace, metadata only; at most the first 64 deterministic exact-name records opened; each record at most 4 MiB and total record bytes at most 32 MiB. Report oversized, absent, symlink, wrong-device, nonregular or concurrently changed records separately. Use no-follow opens where available and compare pre/open/post identity, device, inode, link count and size. Read complete bounded records; hash their actual result bytes independently of the header and parse only those bytes. Do not truncate a header or treat partial JSON as a miss.

Use the existing `observationsHold` export on at most eight complete candidate records, with at most 10,000 total fact nodes per candidate including recursively nested analysis observations. Use no claimed stable-directory shortcuts: pass empty stable-directory/symlink sets. Preserve the original signal and awaited check/progress between files and bounded validation batches; every operation remains inside the same original 60 seconds. Expiry is a truthful partial diagnostic, never a reason to extend the policy.

The actual inference provider bootstrap must run through its existing managed dependency custody and call existing `prepareSourceAnalysesV1` before checking analysis facts. Provider identity includes the exact evaluated module URL and implementation revision. An unregistered or different provider is not evidence that a physical Source file changed. Record provider mismatch/unknown separately, retain the actual IDs/revisions and do not invent/rebind providers from receipt data. Imported taxonomy analysis is registered by the real module. Header observations must be structurally admitted and their paths remain within the original declared Source authority before validation; do not use arbitrary cached paths as new read grants.

Classify each bounded sample as: absent, oversized/not sampled, malformed header, result-hash mismatch, invalid/contradictory observations, first stale fact, provider unavailable/mismatched, source-current observations, or original control expired. Preserve the first stale row's kind/path and recorded/current identity where the public validator supplies it. Do not recompute an image, publish a receipt or invoke a producer on any miss.

## Permitted Conclusions

The stage can establish physical persisted availability, complete result-byte integrity and current Source observations for bounded samples under genuine custody. Even a fully current sample must report **exact production key eligibility unknown** unless the same actual production key and sealed authority are obtained truthfully without changing Source. It cannot prove all 794 completed owners are warm, predict complete ALL duration or justify another full retry automatically.

Positive exact warm-prefix reuse would require the unchanged genuine production path to present its actual keys and demonstrate no recomputation; there is currently no separate read-only port for that claim. Do not replace that missing evidence with a header digest, fabricated receipt, synthetic key, partial project list, normalizer-only result or optional cache hit count.

Core owns any retained diagnostic implementation after Root reviews this plan. Low independently reviews custody, bounds and the exact-key limitation. No production Source, quotas, policies, providers or grants are changed by this proposal.
