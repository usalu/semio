# 🌳️ Baseline — the statute already exists, and the tooling already reads by it

## The statute (not authored here; found, verified, quoted)

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:

* `physicalLeafRendering.filename` = **`"file-kind-emoji-and-extension-chain"`**
* `_treePurityComment`: *"Files use registered kind-only basenames; semantic concerns live in
  registered emoji-plus-slug directories."*

And the renderer, `📚️library/🔍️discovery/🟦️component.ts:1174`:

```ts
return kind ? kind.extensionChains.map((extension) => `${kind.emoji}${extension}`) : [];
```

So a canonical basename is **exactly** emoji + extension chain. `🦀️component.rs` is not one. This is not
cosmetic: `canonicalFilenameForKind` is what discovery, the mutation gates and the policy engine use to
FIND files, so a non-canonical file is **invisible to the tooling**, not merely misnamed.

Two supporting registries make the rule decidable rather than a matter of taste:

* `fixedFilenameContracts` (74) — `pathPattern` + `authority` + `configurability`. `Cargo.toml`,
  `package.json`, `__init__.py` are `"unconfigurable"`: externally imposed, therefore allowed.
* `configurableEntryContracts` (6) — names a tool DEFAULTS to but that can be redirected, with the
  redirect declared: `🦀️.rs` via `Cargo.toml:lib.path` / `bin[].path` / `package.build`, `🟦️.ts` and
  `🟦️.tsx` via `package.json:exports`. So `lib.rs`/`main.rs` are **not** mandatory here.

## Measured baseline

| | |
|---|---|
| files scanned (governed trees `✏️s`, `🧰️framework`, `🌎️hub`) | — |
| already canonical | **8,351** |
| emoji-prefixed basenames carrying a semantic word | **33,372** across 1,898 shapes |

Largest shapes: `🔣️component.json` 9663, `🦀️component.rs` 8036, `🟦️component.ts` 4318,
`🔣️payload.schema.json` 1670, `🔗️component.graphql` 1488, `🛰️component.proto` 1487, the five grammar
kinds 448 each, `📖️component.grammar.semio` 491, `📡️component.protocol.semio` 449.

**Every target name was checked against `fileKinds` before any rename — all 20 are registered.
Nothing is invented.**

## Why the substitution is safe

The emoji prefix makes `<emoji>component.<ext>` a **unique token**. Measured:

| token | total occurrences | emoji-prefixed | bare |
|---|---|---|---|
| `component.rs` | 30,888 | 30,465 | **423** |
| `component.ts` | 6,476 | 6,292 | **184** |
| `component.json` | 15,222 | 15,218 | **4** |

98.6% of `.rs` references and 99.97% of `.json` references carry the emoji, so a literal token
substitution cannot collide. The bare tail (~611) is handled by inspection, not by the script.

`📜️script.ts:1596`'s `match[2] === "component"` is **not** affected: it matches the Rust MODULE name
from `mod component;`, which a file rename does not change.

## Two things measurement overturned

1. **Asset collections are not breaches.** A first pass flagged `🔣️alert-circle.svg`, `🔤️latin.ttf`
   and 200+ icons/fonts. There the slug IS the identity — collapsing them to `🔣️.svg` would destroy
   the collection. The policy engine already agrees: `policyEmojiSiblingIdentityIsStructural` treats
   `🖼️assets` members as structural. The rule binds **component leaves**, not member collections.
2. **The 330 `🔣️.json` + `🔣️component.json` pairs are NOT duplicates.** Six identical header lines
   suggested a copy-not-move; hashing showed all 330 **differ**. Same key set, different values — the
   legacy file is RICHER (`binaryTag: 0`, `textOpcode`, more `requiredLanguageSurfaces`). Deleting
   either loses content, so these need per-artifact judgement and are excluded from the mechanical wave.

## Wave plan

* **Collision-free families** (`🦀️component.rs`, `🟦️component.ts`, `🔗️`, `🛰️`, the grammar kinds,
  `🚫️component.absent`) — `🦀️.rs` and `🦀️component.rs` essentially never coexist, so these migrate
  mechanically with the token substitution. ~18,000 files.
* **Colliding families** — 330 json descriptor pairs, 32 `🟨️host-shim.js`+`🟨️plugin-worker.js`,
  14 `payload`+`wire` schemas, 7 `component`+`schema`, 2 `component.rs`+`protocol.rs`. Judgement each.
* **Gate** — no policy currently reports basename breaches (`policyPackageLanguagePurityBreaches`
  covers package purity only). One must be added, landed reporting-only, then flipped.
