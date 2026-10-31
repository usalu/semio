# Nx Native Funding Preflight

The required ledger budget is now part of the existing closed Nx Capacity contract. Pure tests passed3tests/124assertions; migrated existing graph/output capacity laws passed2tests/42assertions (57unrelated tests filtered). Independent Ajv and SQLite validate the neutral corpus.

The real supported pool snapshot76716 exited0 (1test/7assertions,2.80s). At sequence71, effective/backing/declared capacity was1,610,612,736 bytes, used1,545,469,952, remaining65,142,784. Canonical creator charge67,239,936 exceeded remaining by2,097,152. All16 active allocations contributed: committed repository store536,870,912 plus15 namespace charges, including one reserved allocation with absent physical child. Absence does not establish abandonment or authorize reclamation.

The schema-first quote derives charge from nativeStorageFundedChildChargeV1 and shares the actual producer declaration digest. It admits zero new bytes only for the exact single committed compiler-state child: owner, charge, allocation association, canonical parent identity, geometry, policy digest and namespace association must match, and the physical child must exist. Descriptor presence alone never grants free birth. Reserved/retiring, absent-child, unfunded and ambiguous associations are protected. The existing prior-association auto-commit was removed; the genuine new creator lifecycle still commits its own reservation through its existing canonical handoff.

Runtime reads and releases the fresh shared journal before entering poolEX. Detached snapshot DATA enters one pass; it grants no authority. Both logical/physical ceilings remain and the final Native reserve guard still arbitrates concurrent changes. Insufficient funding raises the existing RoomNeeded outcome. The strict idle eviction path remains responsible for actual retirement, then fresh observation recomputes demand. No rank, budget, recovery guard, refund or generic capacity catch was added. Current protected reserved rows remain charged.

Tests cover8capacity and21association vectors, including the observed shortage, hypothetical eligible retirement, boundary bytes, store/reserved/committed/retiring charges, physical/logical refusal, descriptor-only birth, exact committed identity, device rebound, malformed associations, absent reserved children with either parent-directory state and duplicate matching rows. The third law inspects actual Source admission ordering and preserves the new-creator commit and strict committed retirement branches. That law is Source evidence, not a runtime stress claim.

Actual complete compiled request68160 authoritatively exited0:3tests/31assertions,67.53s. All5 modes ran: forwarded control, public publication, fixed cache, native graph and existing shared inventory. Three fixture-only issues revealed after earlier guards were crossed were corrected without production guard changes: canonical namespace root is nx, ordinary receiver modes reuse their actual inherited Original envelope, and native graph diagnostic coordinate is acquired through the real diagnostics binder in a settled genuine scope.

Earlier failures remain recorded:97152 crossed namespace acquisition but failed the fixture's wrong graph-root binding;14793 public publication passed before fixed mode rejected an invalid nested resource envelope;47188 publication and fixed mode passed before native mode required the diagnostic binding. Each authoritative terminal preceded its causal fixture correction. None is claimed an all-mode pass.

The sequence71 snapshot and final complete-run snapshot91 show actual roster turnover while total used/capacity stayed unchanged and the reserved row stayed exactly protected. Intervening runs occurred, so the turnover is not independently localized to68160 alone. The acceptance is actual complete public Source request execution and unchanged strict guard behavior, not a claim that one specified victim was retired by that exact run.

Same canonical managed prototype dev94822 is now live and admitted, verifying Source closures. It reuses the separately owned standard backend8791 when readiness is reached. No frontend listener or application readiness is claimed yet. Root owns the full semantic gate; unit/watcher/full browser remain required.

Exact focused commands (all through repo with English/ticket/generated environment):
```
repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/🟦️.ts
repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts --test-name-pattern 'finite asymmetric namespace reservations|retained rollback bytes'
repo tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📡️request/🟦️.ts
```

Generated evidence: nx-funding-pressure-red.log, nx-funding-pressure-pure.log, nx-funding-pressure-wired.log, nx-funding-existing-capacity.log, runtime-entry-nx-request-diagnostics-fixed.log, nx-pool-observed.json. Source and fixtures are frozen for current managed admissions.

## Pressure Source Ledger

- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧬️schema/🔣️.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🪙️funding/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🪙️funding/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🪙️funding/🔣️schema.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🪙️funding/🔣️schema.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/🔣️.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/🔣️.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/📋️schema.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🪙️funding/📋️schema.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔣️.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🔣️.json)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts)
- [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🔣️.json](/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🔣️.json)

The already-owned request fixture script was additionally corrected as described; no public receiver guard was relaxed.
