# 🔤️ Slug-Shortening Rule (path budget, gis proof)

Mechanical, deterministic algorithm applied to one path segment (a hyphen-kebab English sentence
used as a test-scenario directory name). Same input always produces the same output.

## Steps, in order

1. **Tokenize** on `-`.
2. **Fraction-phrase substitution** (fixed lookup, longest match first): `<N>-and-a-half` → `<N+1
   converted digit>-5` is NOT how it works — actually applied as: `<word-number>` immediately
   followed by `and-a-half` is replaced by `<digit>-5` (the `-5` stands for the fractional `.5`;
   dots are not legal in scenario ids, see below, so a hyphen substitutes for the decimal point).
   Example: `two-and-a-half` → `2-5`.
3. **Number-word → digit** for any remaining standalone spelled-out integer 1–20 (`one`→`1` …
   `twenty`→`20`). Deterministic fixed dictionary. Existing digits pass through untouched.
4. **Stop-word removal.** Drop tokens that are pure grammatical filler and carry no test-identity
   meaning: `a an the of its every`. Relational/directional prepositions (`after before between
   from to`) are KEPT — they encode ordering and range, which is exactly what these tests assert.
5. **Cap enforcement (~40 UTF-8 bytes).** If the result still exceeds the cap, drop tokens from a
   secondary, lower-priority **downgradable-modifier** list (`single only just simply really
   very basically`) — quantifiers/intensifiers that read as filler once the scenario is scoped to
   its one test directory. Applied left-to-right, one token at a time, until at/under cap or the
   list is exhausted. If still over cap after that, KEEP the overage rather than cut a salient
   noun or verb — the cap is a guideline, the 240-byte path budget is the hard constraint (verified
   per-path at the end, not per-segment).
6. **Uniqueness within parent directory.** Compute the candidate for every sibling under the same
   `🧪️tests/` parent. On collision, re-add the shortest word from the ORIGINAL sentence (by byte
   length) that disambiguates the pair — never a numeric suffix, since a number carries no meaning
   about what makes the two scenarios different.

## Constraint fed back into the rule: no dots in scenario ids

`mutationCatalogProblems` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts:648` enforces
`MUTATION_ID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*$/` on every scenario `id`, and requires
`directoryName === "🧪️" + id` byte-for-byte. Kebab-case, digits allowed, **no dots**. This is why
step 2 encodes a decimal fraction as `N-5` rather than `N.5`.

## Worked examples (all 14 gis offenders)

| original (bytes) | new (bytes) |
|---|---|
| `adds-a-lighthouse-position-after-the-harbor` (43) | `adds-lighthouse-position-after-harbor` (37) |
| `adds-a-tram-route-after-the-ferry` (33) | `adds-tram-route-after-ferry` (27) |
| `adds-the-old-town-region-after-the-harbor-district` (50) | `adds-old-town-region-after-harbor-district` (42) |
| `imports-a-single-harbor-position-descriptor` (43) | `imports-harbor-position-descriptor` (34) — step 5 drops `single` |
| `moves-the-bus-route-to-the-front` (32) | `moves-bus-route-to-front` (24) |
| `moves-the-harbor-position-to-the-end` (36) | `moves-harbor-position-to-end` (28) |
| `moves-the-park-region-between-the-two-districts` (47) | `moves-park-region-between-2-districts` (37) |
| `raises-the-exaggeration-from-one-to-two-and-a-half` (50) | `raises-exaggeration-from-1-to-2-5` (33) — steps 2+3 |
| `removes-the-lighthouse-position` (31) | `removes-lighthouse-position` (27) |
| `removes-the-old-town-region` (27) | `removes-old-town-region` (23) |
| `removes-the-tram-route` (22) | `removes-tram-route` (18) |
| `rewrites-the-ferry-route-payload` (32) | `rewrites-ferry-route-payload` (28) |
| `rewrites-the-harbor-district-region-payload` (43) | `rewrites-harbor-district-region-payload` (39) |
| `rewrites-the-harbor-position-payload` (36) | `rewrites-harbor-position-payload` (32) |

No collisions occur among these 14 (each lives under a distinct mutation's `🧪️tests/`, and all 14
post-rule strings are mutually distinct anyway), so step 6 never fires in this batch — implemented
in the tool regardless, for when a plugin's names do collide.

## Where the rule strains (report candidates)

- `raises-exaggeration-from-1-to-2-5`: the `-5` decimal-substitute is the one place meaning takes a
  real hit — a reader must already know the `N-5` convention to recover "2.5" instead of guessing
  "2, then 5" or "2-5 range". Documented here specifically so it is reviewable.
- `adds-old-town-region-after-harbor-district` sits at 42 bytes, 2 over the ~40 guideline; the
  next word droppable would be `region`, the created entity's own kind — judged not worth losing
  for 2 bytes given the 240-byte path budget is met either way.
