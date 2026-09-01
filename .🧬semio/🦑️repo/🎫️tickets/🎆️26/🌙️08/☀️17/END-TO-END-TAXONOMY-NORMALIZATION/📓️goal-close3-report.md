# Report — `🎠️kernel` / `📡️replication`

Baseline `bb06c41f73f0122fbed315b7487428b976f99921`. Row-by-row enumeration in `📓️goal-close3-census.md`.

```
                before              after (real pasted output)
kernel        moves=50 unresolved=7   moves=50 unresolved=1
replication   moves=64 unresolved=8   moves=64 unresolved=0   <-- APPLYABLE NOW
```

**`📡️replication` reaches `unresolved=0`** — apply it. `🎠️kernel`'s last row (FIX-DEMONSTRATOR's
`paths` array) is a correct refusal, not a bug: narrowing the file-wide `for…of` conservatism to fix
it broke two already-passing tests that require exactly that broad conservatism; reverted.

Fixes: 5 new/extended reference-scan mechanisms in `🧹️normalization/🟦️.ts` (`policyReadFileSafe`
support, `.dependency-cruiser.cjs` boundary arrays, TS/Rust/Markdown comment-path scanning), one
`🔣️taxonomy.json` disambiguation (`json-fixture-case.inferWithoutEmoji=false`, mirrors
`asset-video-subject`), and replication's `🧪️vitest.config.ts` rewritten to kernel's glob pattern.
All touched unit suites pass (only two PRE-EXISTING, unrelated missing-ticket-fixture failures remain,
confirmed present at baseline HEAD too, in `typescript-path-collection`/`markdown-inline-references`/
`frozen-markdown-coordinates`).
