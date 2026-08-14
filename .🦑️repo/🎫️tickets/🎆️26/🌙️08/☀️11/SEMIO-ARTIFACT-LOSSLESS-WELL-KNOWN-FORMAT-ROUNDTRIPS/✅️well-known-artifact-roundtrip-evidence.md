# Well-Known Artifact Roundtrip Evidence

## Scope

This continuation verifies the six supplied native fixtures through import, exact export, snapshot persistence, diff algebra, mutation/inverse/absorb laws, and public I/O routing.

| Format | Bytes | SHA-256 |
| --- | ---: | --- |
| PDF | 6,346,331 | `83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3` |
| DWG | 148,638 | `52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7` |
| SVG | 423,414 | `62a1922aad9e06ba7d4a55fe13c360f286eb720873bcf6d8e6ffd1d52e782fc9` |
| MP4 | 16,086,051 | `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7` |
| PPTX | 16,341,544 | `477900b1746139840890bc4edb653c488f3d18f9da34d231332b5db41d4caa8a` |
| IFC2X3 | 21,282,588 | `f4dbc661d555bbf92fb80a40443f6b6b540fa0a833b85d78487930368147b593` |

## Invalidated Whole-Source Approach

The initial implementation persisted the complete native file image and replayed it when a semantic fingerprint matched. The developer explicitly rejected that design on 2026-08-14 because it bypassed reconstruction. The central build was stopped before it could be treated as acceptance evidence. No result from the whole-source implementation counts toward completion.

The replacement invariant is a lossless format-specific physical model: PDF syntax records, DWG pages/records, XML lexical tokens, MP4 box trees, ZIP physical records with per-member compressed payloads, and STEP physical tokens. Snapshot/artifact codecs, diff algebra, mutations and native writers must carry and reconstruct those models without a whole-file byte field or unchanged-source export bypass.

## Superseded Integration Attempt

The centralized compiler exposed stale Semio bridge leaves after the source contract changed. Existing MP4, PDF, SVG, XML, and DWG serializers/deserializers now construct source-free authored snapshots explicitly, preserve DWG native bytes through `ArtifactSource`, and keep the AC1018 implementation bound to its own snapshot and codec instead of the AC1024 type.

That source-backed compiler attempt is superseded by the structural rework and is not a passing claim for the final implementation.

## Runtime Gate

Pending completion of the structural implementation. A ticket-local `test-quick` build was interrupted with exit 130 immediately after the requirement changed; `🧪️central-test-quick.log` is retained only as diagnostic history.

## Repository Infrastructure

The repository MCP failed to start with `Broken pipe`. The continuation therefore reused the already-open ticket `2026/08/11/SEMIO-ARTIFACT-LOSSLESS-WELL-KNOWN-FORMAT-ROUNDTRIPS` and its recorded `🎯aioptimizedrepo` association without opening a duplicate. Ticket closure must be retried through repository MCP after the runtime gate succeeds.
