# Current Normalization Root Ownership

The controlled physical Cargo workspace explicitly binds to one physical repository root. The current authored normalization caller transform wraps one full test in one command scope; original test bodies can create multiple distinct physical fixture repositories. Therefore that wrapper alone does not establish correct per-fixture operation ownership. This is a source-derived pending risk, not an observed whole-runtime failure.

The chosen correction after the current strict diagnostic cut will give each inventory, planning, verification and apply invocation an explicit nested command scope with its own real cleanup. Pure admitted inventory facts can survive that completed scope; each later physical planning or apply invocation obtains its own admitted operation. Apply owns both its pre-publication and fresh post-publication discovery views, preserving cumulative limits inside that apply command. Every scope uses the same unchanged closed policy and actual closeSystemOwners path. No physical root cache is shared across different fixture repositories, and no fake publication transition is used merely to reset discovery.

The production CLI keeps its one actual explicit command owner because its invocation has one repository root. Full original test roster and long/default command remain required before acceptance.

The epoch-4 authored caller producer now implements these nested scopes for inventory, plan, verify, apply and complete async-generator consumption. It also targets the cancellation option factory invocation structurally, preserving all assertions while passing the explicit owner. This is staged source only; no whole-runtime success or source publication is claimed.
