# Final Extension Actor And Port Source Review

Read-only scoped review of the last source handoffs. No builds or tests executed; no independent runtime pass claim. No new actionable source gap found in the reviewed boundaries.

## Extension Actor Byte Grant

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:44294` exposes the retained next byte demand under a shared registry borrow without advancing its cursor. Registry demand follows the current retained owner, then its metadata frontier; each demand represents the next allocation admission. Resource shell destruction and intrusive metadata node/owner/String/empty-Vec backing releases remain separately admitted. Sorted handler Vec capacity must reach zero before terminal; terminal inline bundle removal frees no heap.

The reactor at `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:1023` passes actual `budget.max_patch_bytes`, and zero fuel supplies zero item/byte grants. Retirement preserves the owner when blocked or AwaitingInput. Ordinary invocation remains sealed through the registry and bundle; only reserved empty-payload cancel/resume controls bypass ordinary active invocation. Cancellation retains state and resumption does not reopen it.

The native actor law `🔌️plugin/🧪️tests/🔬️extension-retirement/🦀️.rs:304–335` constructs real 128KiB manifest text, reaches its actual demand, verifies 64KiB refusal, zero-fuel retention, cancellation retention, and progress after explicit resume with a 2MiB poll budget accompanied by QuotaChanged. It then verifies sealed ordinary invocation and drains to terminal. The portable fixture is `🔌️plugin/🧫️fixtures/🧩️extension-retirement/🔣️.json:3`, its schema pins 65536/131072/2097152, and TypeScript/Ajv oracle lines 27–29 validate the same allocation ordering. The law demonstrates larger poll-budget admission; it does not assert that the QuotaChanged event itself rewrites a budget—the reactor intentionally ignores that variant's policy mutation while preserving its bridge.

## BREP And Flow Actual Ownership

BREP guest resource source `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:2133,2170–2180` owns SessionCapture, drives actual capture retirement, requires capture emptiness, exposes measured shell demand, and guards sealing/cancel/resume after empty capture. The real guest fixture `🧪️tests/🔬️extension-guest-standalone/🦀️.rs:11–15` adapts its explicit grant to the bundle demand and verifies reported credit against that supplied grant. This resolves the former raw-Session shell omission.

Neutral Flow `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🦀️.rs:11–55` requires seal, close slice, terminal and demand APIs. Its retained wrapper preserves the concrete Box through nested resource retirement, validates budget/terminal receipts, admits actual Box size, and refuses unfinished Drop. Host and evaluation-session predicates retain this wrapper until empty. Host cold retirement adapts to demand; evaluation-session cold retirement supplies explicit unlimited cold grants. Interactive grants pass unchanged.

SessionPort at `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:1464–1501` holds a separate child authority and producing-root capture anchor. The anchor prevents family destruction during admitted child-state shell release; subsequent anchor release either delegates to a live root or owns actual final family retirement. Zero/insufficient grants and family pause preserve both owners; denied ports are also explicitly retired. SessionCapture's per-handle allocator clones clear scoped receipt pointers, and only immediately consumed handles receive the stack receipt. Final predicates require actual owner emptiness rather than native-payload emptiness alone.

The introduced Playbook owner uses the same SessionCapture boundary; import storage and registry retirement finish before its capture. Earlier detailed coordinates and fixture review remain in the preserved flow-port and allocator follow-up reports. No generic wrapper expansion or existing backend progress-ledger audit was performed.
