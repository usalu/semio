# FEM Exclusive Cleanup Review

Read-only source; no semantic compilation or native execution. Physical and logical uniqueness jointly detect an in-flight alias drop; safe strong handles provide no independent alias after both counts reach one. This is not a proof for unsafe raw-handle misuse.

Public callback issue: 🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs:54 takes root before invoking arbitrary release callback. A panic leaves partial custody and stale len/height; fail-closed handles then unwind. Builder 📥️build/🦀️.rs:27 takes levels before callback completion. A stateful predicate can succeed in whole-builder preflight then fail the repeated list check; only debug_assert protects the already-taken list, so release builds lose custody or hit fail-closed Drop. Infallible sealed callbacks or unwind-safe custody and a single reliable preflight are needed for the public generic contract. FEM private callbacks appear simple. Findings sent to owner and coordinator.

The former infinite allocation retry is absent. Work accounting, terminal correctness and semantic compilation remain unverified.

## Revised ColdRetire port

Current source replaces arbitrary callback with a documented pure, infallible ColdRetire contract and one full preflight before private prechecked drain. Node/material implementations check physical and logical uniqueness of their sealed strings before allocation-free string handback. Leaf and branch builder allocation failures restore their rows or roots; carry allocation refusal leaves the carry and levels retained. The source tests assert alias refusal without mutation, then no-allocation teardown. No concrete safe strong-handle alias race or type-bound blocker found; RetireOwned already supplies Send and static. These are source observations, not a native or semantic compiler pass. Public implementations must honor the documented trait contract.
