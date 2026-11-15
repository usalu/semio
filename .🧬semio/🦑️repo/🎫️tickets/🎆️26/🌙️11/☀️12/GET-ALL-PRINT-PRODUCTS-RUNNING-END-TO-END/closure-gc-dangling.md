# Dangling build-input closures after `cache gc` / `cache cas gc`

L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`

## Root cause

(b) Service. The stack in the host log is `readInventoryRecordLeanV1` (`L/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12`) <- `portableInputs` (`L/🏗️build/🕸️graph/📁️workspace/🟦️.ts`, the `expand` of the generation host) <- `expandStreamV1` in `loadGenerationGraphV1` (`L/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs`). Stored generation fragments hold compact `SEMIO_SOURCE_CLOSURE_<sha>` tokens; the load expands them from `.cache/b/g/m/build-input-closures/<sha>.json`. The only guards were: an in-memory expanded-list memo (`expandedClosuresV1`) that was trusted without checking the file, and a read that threw the raw ENOENT. Nothing in the generation path turned a missing leaf into a miss, so every request failed until the process (and its memo) was replaced. The sibling inference-reuse path (`inferenceInventoryReferencesHoldV1`, `👁️observation/📥️reuse`) already classifies an ENOENT leaf as "does not hold" and recomputes; the legacy sealed `graph.json` image path (`📥️load/🟦️.ts:80`) swallows ENOENT and falls through to inference. The generation path was the only one without it. A token alone cannot be recomputed (the file list lives only in the leaf), so the owners must be re-inferred.

(a) Collector. It collected closures deliberately: `build-input-closures` is a declared CAS area (`🏗️build/🗄️cas/📂️areas/🟦️.ts`, policy `🗄️cas/🧬️schema/🔣️.json`: cap 8 GiB, maxAge 7 days) and both `repo cache gc` (`🧹️cache-gc/⌨️command/🟦️.ts` `collectHostStoreV1`) and `repo cache cas gc` (`🗄️cas/⌨️command/🟦️.ts`) run `collectCasV1`. With the partition above its high share the planner cuts the host total oldest-first. The only protection of an area entry was `minAgeMs` (10 min); the pin mechanism (`🗄️cas/📌️pins`, digests held by a live pid with a lease) was applied to store objects only (`entry.hash`). So every closure older than 10 minutes was an ordinary candidate, regardless of whether the sealed generation referenced it. The reference-proving closure retirement of the graph headroom (`📏️headroom/🧾️closures`, which does read generation fragments) was not involved.

## Fix (existing pin mechanism reused)

1. `🗄️cas/🧹️collector/🟦️.ts`: an area file named `<sha256>.json` is protected while a live holder pins that digest (counted in `protectedBytes`, the "MiB protected" figure) and the eviction pass re-checks the pin right before unlinking, exactly as for store objects.
2. `🗄️cas/📌️pins/🟦️.ts`: `holdCasPinsV1` keeps a growing per-process set pinned (holder `graph-closures.<pid>`), written at once, renewed every lease/3 by an unref'd timer, write failures (full disk) ignored; dead process loses it (existing liveness rule).
3. `🗄️cas/📂️areas/🟦️.ts`: `casPinClosuresV1(digests, env)` resolves the host CAS root and holds the pins; no home means nothing to pin.
4. `🕸️graph/📁️workspace/🟦️.ts` `portableInputs`: pins the closure digests of every expansion before reading them; a memo hit whose leaf no longer exists drops the memo and reads (miss); a read ENOENT is rethrown as the original error tagged `missingClosure=<key>` (code ENOENT kept so the `📥️load/🟦️.ts:80` fallthrough still works).
5. `🕸️graph/🕰️generations/🔌️load/🟨️.mjs`: on `missingClosure` the shared attempt answers `needsBuild` (structural); the builder attempt calls `engine.invalidate()` once per load and answers `needsBuild`, so the next round plans cold, re-infers every owner (republishing the closures) and serves the graph. A second failure (leaf still missing after a cold build) or any other error is thrown unchanged.

## Files changed

- `L/🏗️build/🗄️cas/🧹️collector/🟦️.ts`
- `L/🏗️build/🗄️cas/📌️pins/🟦️.ts`
- `L/🏗️build/🗄️cas/📂️areas/🟦️.ts`
- `L/🏗️build/🕸️graph/📁️workspace/🟦️.ts`
- `L/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs`
- tests: `L/🏗️build/🗄️cas/🧪️tests/🟦️.ts` (pinned closures survive an aged-out collection, fall once released; no home means no pin), `L/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🟦️.ts` (missing closure -> invalidate, cold rebuild, graph equals the Python oracle; still-missing and other errors propagate)

## Tests (light, `repo tools exec --light bun -- test`)

- cas tests: 36 pass, 0 fail
- generation load tests: 92 pass, 0 fail (incl. the new one)
- `🕸️graph/🧪️tests/📤️portable`: 3 pass (extracts `generationHostV1` source, still valid)
- Not run: no light test drives `portableInputs` itself (needs a real graph store under the build test law); its memo/ENOENT-tagging is verified by reading only.

## Follow-ups

- Recovery is a cold re-inference of all owners; a targeted re-derive of only the owners whose fragments reference the missing key would be cheaper (needs a key -> owner lookup).
- No bilingual notice is printed on the recovery (would touch schema, the inline table in `🔌️load/🟨️.mjs` and the `.d.mts`).
- The pinned set grows for the process lifetime (cap 65536) and so also holds closures of superseded generations until the process ends; the headroom retirement (reference-proven) still reclaims those.
- The pin is per host CAS root, closures per workspace: digests are content addresses, so pinning across workspaces over-protects but never under-protects.
- Unit tests of the CAS area write pin files only under scratch roots; any test that runs `portableInputs` against a real HOME will leave a pin file of a dead pid, which the next collection removes.
- This changes graph-service implementation: one graph re-plan on the next `repo run`.
