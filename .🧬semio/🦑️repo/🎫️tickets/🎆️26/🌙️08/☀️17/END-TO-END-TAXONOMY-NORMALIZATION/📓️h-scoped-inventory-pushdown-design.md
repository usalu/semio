# Scoped Inventory Pushdown Design

## Decision

Push a conservative, literal repository-relative prefix into only the tracked and ordinary-untracked Git enumerations. Retain the existing NFC-aware `inScope` filter after enumeration, retain `--exclude-standard`, retain ignored-generator and explicit-ticket admission unchanged, and retain the exact unscoped commands when `scope` is absent.

The scoped pathspec must not be formed by mechanically appending every opaque exclusion. On Git 2.54.0, the disjoint exclusion `:(exclude,top,literal)temp/compose` makes the otherwise valid writer-scope query return zero rows. The writer must include only opaque exclusions that can intersect the positive prefix. This is equivalent exclusion authority: a disjoint literal positive prefix cannot admit an opaque path in the first place.

For `✏️s/🔌️plugins/✒️writer`, the safe positive pathspec selects 252 tracked rows rather than 64,915, a 99.612% reduction in tracked rows materialized and parsed. The benchmarked inventory has 445 final entries because it synthesizes 193 directory entries around those 252 physical leaves. No elapsed-time claim is made until the identical benchmark is rerun.

## Read-only scope and evidence

No production, test, schema, script, Git state, `compose/**`, `temp/compose/**`, or `temp-compose/**` content was changed or read. The opaque names below came from source/schema metadata and Git pathspec arguments, not filesystem traversal.

The motivating retained benchmark is `📓️s-scoped-inventory-phase-benchmark.md`:

- scope: `✏️s/🔌️plugins/✒️writer`
- result: 445 entries, 33 violations, 3,469 ms
- before first progress event: 3,003 ms, or 86.6%
- reported work: 193 directory events/81 ms, 252 file events/270 ms, 197 reference events/81 ms, finalization/8 ms
- conclusion: full tracked and untracked enumeration occurs before the current scope filter and before the first event

Read-only command evidence on Darwin, Bun 1.3.14, Git 2.54.0, with repository `core.ignorecase=true`, `core.precomposeunicode=true`, and `core.symlinks=true`:

```text
$ git ls-files --stage -- ':(top,literal)✏️s/🔌️plugins/✒️writer' | wc -l
252

$ git ls-files --others --exclude-standard -- ':(top,literal)✏️s/🔌️plugins/✒️writer' | wc -l
0

$ git ls-files --stage -- . | wc -l
64915

$ git ls-files --stage -- ':(top,literal)✏️s/🔌️plugins/✒️writer' \
    ':(exclude,top,literal)temp/compose' | wc -l
0

$ git ls-files --stage -- ':(top,literal)✏️s/🔌️plugins/✒️writer' \
    ':(exclude,top,literal)✏️s/🔌️plugins/✒️writer/🗿️artifacts' | wc -l
14
```

The last two probes demonstrate both sides of the rule: a disjoint `temp/compose` exclusion is unsafe in this Git/pathspec combination, while an exclusion below the positive scope works as intended.

The index currently contains four mode-`160000` entries, all outside the benchmarked scope. Scoped behavior therefore still needs an explicit indexed-leaf-ancestor safeguard; it cannot assume that every lexical directory ancestor is a normal directory.

## Current ownership and exact hot path

Production owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`.

- lines 1713–1722: `normalizeRelative` performs NFC normalization; `sourceRelative` canonicalizes separators and rejects escape/absolute/NUL input.
- lines 1733–1755: reusable no-follow ancestor and opaque-input guards.
- lines 1758–1767: `isExcluded` and `inScope`; `inScope` includes the exact scope, descendants, and lexical ancestors.
- lines 2010–2029: `gitRows` and `untrackedGitPaths` always use positive `.` plus every opaque exclusion.
- lines 2040–2070: explicit-ticket admission recursively walks the admitted ticket root without following symlink leaves.
- lines 2081–2099: ignored-generator admission recursively walks exactly configured ignored output roots with `lstat` and byte-sorted children.
- lines 3765–3791: `inventoryTaxonomy` calls all four enumerators first and only then applies `inScope` in memory.
- lines 3804–3890: the first progress event is emitted from directory or file canonicalization.
- lines 2888–2917 and 3906: references emit only per-file events, with no empty-phase event.
- lines 3939–3946: final digest assembly and `complete` event.

CLI owner: root `📜️script.ts`.

- lines 18504–18511: `taxonomyCliInventoryOptions` does not pass a progress callback.
- lines 18615–18639: inventory receives no engine progress; only later shard publication writes progress to stderr.

Permanent test owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`.

- lines 3159–3183: current coverage proves repeatable canonical bytes, a physical-file census against `fast-glob`, at least one progress event, and absence of `compose/` entries.
- the current test does not prove Git pushdown, closed phase ordering, scoped-versus-full census parity, exact literal metacharacters, NFC/NFD scope equivalence, untracked/ignored/ticket admission parity, or scope behavior at a symlink/gitlink ancestor.

## Frozen pathspec contract

### 1. Scope preflight

1. Resolve the repository root exactly as today.
2. Normalize `scope` with `normalizeRelative`; empty/`.` remains the unscoped case.
3. Reject a scope equal to or below either loaded opaque exclusion before any scoped filesystem access or Git command.
4. Examine every proper lexical ancestor with `lstat`, never `stat` or `realpath`. If an existing ancestor is a symlink or non-directory, do not optimize: use the full positive `.` and retain the final `inScope` filter.
5. Also probe every proper ancestor for an exact stage-0 index entry with an argv-based, no-shell command equivalent to:

   ```text
   git rev-parse --verify --quiet --end-of-options :<ancestor>
   ```

   A successful exact index lookup means that ancestor is an indexed leaf (regular file, symlink, or gitlink rather than an index directory). Fall back to positive `.`. This preserves the old ancestor-inclusive scoped result without walking through the leaf. Exit 1 means no exact index entry; other failures are errors. The leaf scope itself is not probed as an ancestor, so an exact symlink/file scope remains enumerable.

This fallback is rare and correctness-first. It prevents the optimized command from silently dropping the indexed ancestor that the current `inScope` relation admits.

### 2. Unicode-safe positive prefix

Git literal pathspecs are byte exact, but current scope matching NFC-normalizes both the supplied scope and each enumerated candidate. Passing an NFC scope directly would miss an NFD index spelling on Linux and other mixed-normalization histories.

Compute the longest leading sequence of scope segments for which every segment is normalization-stable, meaning `segment.normalize("NFD") === segment`. Stop before the first decomposition-sensitive segment. Examples:

| Scope | Positive prefix |
| --- | --- |
| `✏️s/🔌️plugins/✒️writer` | exact scope |
| `owners/café/case` | `owners` |
| `café/case` | `.` |
| empty/`.` | `.` |

An emoji/VS16 segment that is stable under NFC/NFD remains eligible. This is not VS16 folding: the final existing `inScope` predicate remains the semantic authority. Always run `inScope` after Git enumeration, even when the full scope became the positive prefix.

### 3. Intersection-aware opaque exclusions

Let `prefix` be `.` or the conservative positive prefix. An opaque exclusion intersects the positive set iff:

- `prefix === "."`; or
- `exclusion === prefix`; or
- `exclusion.startsWith(prefix + "/")`; or
- `prefix.startsWith(exclusion + "/")`.

The last case must already have been rejected as an opaque scope, but retaining the symmetric predicate makes the helper fail closed. Byte-sort the selected exclusions and render each as `:(exclude,top,literal)<path>`.

Do not append disjoint exclusions. Their paths are impossible beneath a literal positive prefix, and the current Git 2.54.0 evidence shows that doing so can erase all scoped results.

### 4. Exact Git argv

For a safe nonempty prefix:

```text
git ls-files --stage -z -- \
  :(top,literal)<prefix> \
  <intersecting opaque exclusions only>

git ls-files --others --exclude-standard -z -- \
  :(top,literal)<prefix> \
  <intersecting opaque exclusions only>
```

For no scope, normalization fallback, or an indexed/non-directory ancestor fallback, retain the existing full positive and all exclusions:

```text
git ls-files --stage -z -- . \
  :(exclude,top,literal)compose \
  :(exclude,top,literal)temp/compose

git ls-files --others --exclude-standard -z -- . \
  :(exclude,top,literal)compose \
  :(exclude,top,literal)temp/compose
```

Use `execFileSync`/argv, never a shell string; retain `--`, `-z`, stage-0 filtering, the 256 MiB buffer limit, `sourceRelative`, byte sorting, `--exclude-standard`, and the final `isExcluded` plus `inScope` checks. Do not use Git glob magic. Literal magic is required for `*`, `?`, `[`, `]`, `!`, `:`, spaces, emoji, and selector bytes in names.

## Admission and determinism invariants

- **Tracked:** only the candidate set is narrowed. Stage parsing and worktree existence checks remain unchanged.
- **Ordinary untracked:** only the Git walk is narrowed. `--exclude-standard` remains authoritative, so ignored content does not leak into ordinary untracked admission.
- **Ignored generated:** leave `ignoredGeneratorRows` unchanged in this packet. It must continue to admit only schema-owned ignored generator output roots, use `lstat`, avoid symlink following, and byte-sort children. Its new phase exposes its cost for a later, separately proved root-intersection optimization.
- **Explicit ticket:** leave `explicitTicketRows` unchanged. Ticket admission, nested `.git` stopping, opaque checks, and symlink behavior remain identical. Its new phase likewise makes its cost visible.
- **Parents:** retain `inScope`'s ancestor clause and current synthesized-parent loop. A normal directory does not need to appear in the Git index; parents are recreated from admitted leaves.
- **Ordering:** retain Buffer byte ordering of source paths and final entries. Progress event order is not hashed.
- **Digests:** scope, entries, violations, `sourceTreeDigest`, and `inventoryDigest` must be byte-identical for the same fixture before and after the optimization.
- **Full inventory:** absence of scope takes the exact old `.` path; no command, admission, ordering, or digest behavior changes.
- **Case:** a case-insensitive filesystem or `core.ignorecase=true` may let Git return a superset. The existing case-sensitive, NFC `inScope` filter remains final.
- **Backslash:** input still converts `\` to `/`. A literal POSIX filename containing a backslash is not a portable repository path and must not be reinterpreted as pathspec syntax.

## Closed phase telemetry

Freeze these inventory phase IDs and order:

1. `setup`
2. `tracked-enumeration`
3. `untracked-enumeration`
4. `ignored-generator-admission`
5. `explicit-ticket-admission`
6. `directories`
7. `files`
8. `references`
9. `complete`

Emit `setup 0/1` immediately on entry and `setup 1/1` after taxonomy load, scope validation, and the initial cancellation check. For each blocking enumeration/admission call, emit `0/1` immediately before it and `1/1` immediately after it; report the normalized scope as `path` when present. A unit total is honest because Git does not expose its result count before the synchronous command returns. Check cancellation before and after every blocking phase.

For `directories`, `files`, and `references`, emit `0/N` before the loop and retain per-item `index+1/N` events. An empty phase is represented by the single closed `0/0` event. `complete` remains the terminal event. Do not invent percentages or phase totals from final inventory size.

Root `📜️script.ts` must pass one inventory progress sink through `taxonomyCliInventoryOptions` for inventory, plan, and any other operation that inventories. The sink writes only stderr, using the existing start/end/every-100 throttling. JSON stdout must remain canonical and machine-only. Shard publication keeps its separate `write-shards` events after inventory `complete`.

The synchronous Git call cannot be cancelled mid-process. This packet guarantees cancellation at phase boundaries; replacing it with a polled child process is a separate mechanism change and is not needed to obtain the scoped candidate reduction.

## Acceptance tests

### Language-neutral golden

Add a permanent physical JSON leaf, for example:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️scoped-inventory-pathspec/🔣️.json`

The JSON contract should contain schema version 1, the two opaque exclusions, and exact cases for: unscoped, exact writer-like scope, scope above one opaque root, scope disjoint from both roots, first-segment NFD sensitivity, later-segment NFD sensitivity, and literal metacharacters. Each row records normalized scope, conservative prefix, exact positive pathspec, and exact selected exclusion pathspecs. A non-TypeScript implementation must be able to reproduce identical JSON bytes.

### Bun/TypeScript unit and integration coverage

Use only disposable repositories created by the existing fixture helper; configure and mutate Git only inside those fixtures and clean them in `finally`.

1. **Pure argv golden:** compare the pure pathspec builder to every language-neutral JSON case. Assert `--`, ordering, literal magic, and omission of disjoint exclusions.
2. **Tracked parity:** create scoped and unrelated tracked leaves. Compare optimized scoped census fields (`sourcePath`, `nodeKind`, `mode`, `size`, `contentHash`) to the same current full-enumeration/reference filter. Assert unrelated leaves never enter the scoped inventory.
3. **Untracked parity:** include one scoped untracked leaf, one unrelated untracked leaf, and one Git-ignored ordinary leaf. The scoped untracked leaf is admitted; unrelated and ordinary ignored leaves are not.
4. **Generator parity:** configure one ignored generated output under scope and one outside. The in-scope owned output remains admitted only through generator authority.
5. **Ticket parity:** put tracked/untracked ticket evidence both in and out of the semantic scope; retain the current explicit-ticket admission result.
6. **Opaque proof:** create fixture-local unreadable/sentinel bytes at both opaque roots, then inventory a disjoint scope and a parent scope. Assert neither opaque path is returned, sentinel bytes and modes are unchanged, and no filesystem content read is observable. Never use the real workspace opaque roots.
7. **Unicode:** commit an NFD-spelled directory/leaf and request the NFC-equivalent scope. Assert the conservative prefix plus final `inScope` reproduces current behavior on Linux, macOS, and Windows-compatible checkouts.
8. **Literal names:** use a scope containing spaces and Git metacharacters (`[]*?!:` where the platform permits). Assert exact membership, no expansion, and identical output ordering.
9. **Symlink leaf:** an exact scoped symlink is inventoried as a symlink and is never followed.
10. **Symlink ancestor:** scope below a tracked or untracked symlink triggers the full-enumeration fallback and returns the same ancestor-only census as the pre-pushdown behavior.
11. **Gitlink ancestor:** install a fixture-only mode-`160000` index entry and scope below it. Assert exact-ancestor preflight triggers full fallback and preserves the prior census; no nested repository is read.
12. **Closed telemetry:** assert the exact phase order above, `0/1` then `1/1` for every enumeration/admission phase, `0/0` for empty work phases, terminal `complete`, and cancellation observed at each phase boundary.
13. **Determinism:** run scoped inventory twice with reversed fixture creation order. Canonical inventory bytes, source digest, inventory digest, and phase sequence are identical.
14. **Full regression:** scope absent uses the exact existing argv and produces the same canonical inventory bytes as the pre-pushdown golden.
15. **CLI purity:** focused root CLI execution emits phases on stderr and valid canonical JSON alone on stdout.

### Third-party parity

Use the existing test-only `fast-glob` dependency with `followSymbolicLinks:false`, `dot:true`, and a literal/escaped scoped root to independently census the disposable physical fixture. Compare its in-scope file and symlink membership to the engine census after applying the language-neutral scope relation. Git remains the tracked/untracked authority; `fast-glob` is an independent physical-output cross-check, not a replacement for Git ignore or index semantics.

Run the repository's existing focused Bun selector for repo-lib normalization, then the no-space/equal-form Nx selector that selects the same test. Record commands, pass/fail counts, assertions, elapsed time, and cache status. Finally rerun the exact writer benchmark and compare canonical source/inventory digests before comparing phase timings.

## Minimal writer packet

1. In normalization inventory helpers, add a pure conservative-prefix/intersection-aware pathspec builder.
2. Add exact proper-ancestor preflight (`lstat` without follow plus stage-0 `git rev-parse` lookup) and select full fallback when an ancestor is a leaf.
3. Change only `gitRows` and `untrackedGitPaths` to accept optional validated scope/pathspecs; pass scope from `inventoryTaxonomy`.
4. Keep both existing final filters and all parsing/sorting/admission logic.
5. Add the four unit enumeration/admission phases plus closed `setup`, directory, file, and reference telemetry.
6. Wire engine progress to the root CLI's stderr sink without changing JSON stdout or shard publication.
7. Add the language-neutral JSON golden and focused tests above. Do not optimize ignored-generator or explicit-ticket traversal in this packet.
8. Rerun the identical writer benchmark. Accept only if entries, violations, source digest, and inventory digest are identical; phase order is closed; opaque sentinels are untouched; and elapsed/candidate evidence is recorded.

## Expected impact and sign-off gates

Expected immediate candidate impact for the frozen writer scope:

- tracked rows parsed: 64,915 → 252
- reduction: 64,663 rows, or 99.612%
- ordinary scoped untracked rows in the current tree: 0
- invisible interval: replaced by explicit setup/enumeration/admission phases

Runtime improvement is intentionally unresolved. Sign-off requires the same-scope rebenchmark and byte-identical digests. A larger representative scope may be benchmarked only after that equivalence gate. The 998-second retained full-inventory run remains the only full baseline, and the unscoped path must remain behaviorally unchanged.

## Golden checkpoint

The language-neutral golden now exists at:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️scoped-inventory-pathspec/🔣️.json
```

It freezes seven cases: unscoped, exact stable scope, scope above one opaque root, disjoint scope, first-segment decomposition sensitivity, later-segment decomposition sensitivity, and literal Git metacharacters/spaces. Independent checks parsed the JSON, asserted schema version 1 and seven rows, and proved every non-null input NFC-normalizes to its recorded normalized scope. Production builder and integration tests remain pending transaction release.
