# Retained Renderer Patch Packet

## Source and Runtime Evidence

The coordinator re-read `UiDocumentStore` validation, patch application, hydration, tree flattening, and notification, plus the matching PluginRuntime wire/application call sites. The existing diagnostic already executes the actual patch function on valid 4,096-node and 20,000-node trees; functional assertions pass, but many calls exceed eight milliseconds. Its concurrent-compilation/Bun caveat remains: this is not an isolated browser input-latency measurement.

Current synchronous work includes whole patch byte estimation and UTF-8 encoding; cloning the entire node map; applying every operation, including subtree deletion; validating the whole graph and dangling nodes; recursively flattening BuiltNode children; whole snapshot stringify/encoding/hash; and comparing every old/new node before dispatching all listeners. One-node edits can therefore walk unrelated parts of a large tree. Delaying the start of this function does not split its work.

## Required Coherent Boundary

After the current renderer contract/type repair, replace the interactive path with a retained transaction whose decode, byte accounting, mutation, validation, reconciliation, hash/publication, and notification frontiers are explicit. Each front must resume under item/byte/time/cancellation grants. The working root stays private until validation succeeds; acceptance swaps the exact root and revision together. Rejection or supersession leaves the prior root/reference/revision unchanged and notifies no accepted change.

Avoid a whole Map clone in a setup or completion step. Persistent bounded-page or bounded-node backing must keep unchanged records shared and preserve old captured readers. An unbounded chain of overlays is not a replacement unless lookup and incremental consolidation have their own bounded contracts. Avoid shifting large arrays, spreading entire children lists onto a stack, or one-shot disposal of rejected drafts. Work already done in the worker must not be rebuilt from scratch in the UI callback.

Track touched node IDs during accepted preparation instead of rediscovering them with a whole-store comparison. Observer delivery must also yield under a bounded frontier; subscription changes, reentrant callbacks, node removal, and a newer transaction arriving during delivery need deterministic rules. UI readers must never observe mixed root/revision publication even when notification spans multiple turns.

## Verification

Preserve the existing cross-language rejection corpus exactly, including its deliberately unchecked initial snapshots followed by a rejected patch. Add language-neutral fixtures for every-stage cancellation, stale-base/superseded preparation, large subtree deletion, duplicate keys, cycles, deep/wide graphs, long UTF-8 fields, multiple surfaces, captured old readers, and reentrant/unsubscribed listeners. Differentially compare final snapshots, exact rejection categories, and event order against the existing reference path in tests only.

The current full React test checkpoint is 470 passing tests, with type repair in progress. Run the complete suite and typecheck after integration, then build fresh Wasm and test real browser input/ack/preview/commit flows under large-tree and resize storms. Final eight-millisecond and device/accessibility gates require runtime measurements, not source assertions or a mock-Wasm DOM pass.

## Status

This is a reviewed follow-up packet, not an implementation claim. The renderer executor is first repairing the outstanding production/fixture type contracts; no second agent should overlap those files without coordination.
