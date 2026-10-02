# Discovery Ephemeral Import and Pattern Corrections

Read-only source inspection; no jobs executed.

## Concrete first-party runtime owner

Discovery line11 imports ephemeralBox/ephemeralMap through external package @semio-tech/framework. Actual definitions live at `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:29–108`: EphemeralBox, OsTransient, defaultOsTransient and ephemeralBox/Map/Set/WeakMap form a self-contained native Map/Set/WeakMap closure. Their public return types use only owned EphemeralBox and system collection types. No foreign library interface is required by these functions.

Discovery consumers: cachedTaxonomy/cachedCatalogTaxonomy1190–1191, gitIndexFileModeCache5894, scanCache10280 and gitlinkBoundaryCache10402. Preserve their exact keys and shared identity/reset semantics.

A direct import `../../../../../🔨️modules/🎠️kernel/🟦️.ts` reaches the real current definition and removes the package alias, but the whole kernel also imports manifest, shard client, actors and resident value runtime (kernel1–23). It is not a narrow dependency closure. The coherent smallest owner cut is moving the complete EphemeralLane declaration closure into an owned kernel ephemeral subdomain, then all consumers import that physical leaf directly. Do not move just wrappers while leaving defaultOsTransient in the broad kernel; that restores the closure. Do not recreate module-local boxes/maps and drop cross-module transient identity. Kernel facade can use direct imports/reexports if required by its existing public API; discovery needs no facade import. Schema/language-neutral identity/reset laws should own the new leaf before extraction.

## Three demonstrably malformed positive rows

Current fixture `library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🔣️.json`:

* windows-separators: path `🧪️root\🔣️.json`, pattern `🔣️root.json`, expected true. After path normalization the first becomes `🧪️root/🔣️.json`; no matcher can equate this with the unrelated basename. Keep pattern inventory unchanged by changing path to a backslash-normalized spelling of the declared pattern if the purpose is leading-dot separator normalization, or use existing declared `🧪️root/**/🔣️.json` pattern and update unique pattern cardinality intentionally. Patterns are POSIX contracts; Windows normalization applies to input paths.
* middle-globstar-zero: path `🦀️root.rs`, pattern `🧪️root/**/🦀️.rs`, expected true. Correct positive zero directory path is `🧪️root/🦀️.rs`; the root literal is not optional. Retain many row `🧪️root/a/b/🦀️.rs`.
* unicode-question: path `😀.rootts`, pattern `root?.ts`, expected true. Correct single-code-point positive is `root😀.ts`. Regex /u means question matches one Unicode code point, not grapheme cluster; picomatch fixture oracle uses flags:u. A supplementary emoji demonstrates that correctly, whereas current path places unrelated prefix and wrong dot.

These corrections need no provider grammar changes. Changing only paths preserves the authored unique normalized pattern count14 where an existing pattern is retained. Fixture root uniqueNormalizedPatterns=14 and schema const14, test asserts Set(pattern.normalize(NFC)).size===14, and compiler expression count laws use that number. If Windows row's pattern changes, recalculate actual unique roster and handcraft schema/fixture together; never force cardinality by adding irrelevant decoys.

Parser validation rejects backslash in authored contract patterns. Do not normalize pattern separators as a compatibility behavior. The matcher normalizes paths and NFC patterns only; the test's independent oracle normalizes the path separately. Keep these two inputs distinct in the language-neutral contract.

## Exact neutral store rename closure

Source census finds OsTransient/defaultOsTransient TypeScript references only in kernel owner and `framework/🧪️tests/🧪️docklayoutstore/🟦️.ts`. The latter receives OsTransient through dependencies destructuring at4 and constructs two isolated stores552–553; rename that injected dependency field along with caller harness contract. Existing law531–567 proves function init is not called, no-op function identity survives, isolation, keyed box reuse and reset clears identity. Preserve those assertions. No dedicated transient schema/fixture or third-party oracle was found in the inspected kernel fixture/test subtrees; do not claim that existing dock test supplies a closed multi-language contract.

Exact move: EphemeralBox type; class with boxes/maps/sets/weakMaps and box/map/set/weakMap/reset methods; default store allocation; four ephemeral helper functions. None depends on kernel imports or later definitions. Rename class/store declarations and helper references to TransientStore/defaultTransientStore, doc links and dock dependency field; kernel exports actual new names explicitly, no Os alias. Existing ephemeralBox/Map/Set/WeakMap callers retain their API and exact keyed identity by importing the one physical default store leaf.

Small closed language-neutral operations fixture can model create-store, box init/get, map/set mutation and reset, with integer reference IDs as identity observations. Native JS Map/Set/WeakMap provide system collection oracle; an independently implemented test-only keyed registry can compare identity/reset traces without foreign runtime dependency. Include same key across store instances, function-valued init retained untouched, same key different collection kinds, reset yielding new collection and old returned collection still usable. WeakMap oracle needs explicit object handles rather than JSON string keys. Existing dock tests remain original runtime route; add canonical leaf producer registration rather than claiming extraction-only module bundling is behavioral proof.
