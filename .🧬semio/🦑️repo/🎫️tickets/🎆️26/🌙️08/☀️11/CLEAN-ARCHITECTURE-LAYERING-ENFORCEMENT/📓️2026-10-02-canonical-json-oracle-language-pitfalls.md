# Canonical JSON Oracle Language Pitfalls

Read-only source/package/log inspection; no jobs. Root selected library serialization/json owner with original semantics except rejecting nonserializable top value.

## Available actual third-party oracle

Root bun.lock:2376 contains fast-json-stable-stringify2.1.0; node_modules/fast-json-stable-stringify/index.js and index.d.ts are physically present. Its :46 object keys use JS default lexicographic sort, serializing keys directly. Thus object {"2":...,"10":...} emits10 before2, while current owned canonicalValue assigns sorted keys into a JS object and JSON.stringify reorders integer index keys numerically2 before10. Direct byte equality with stableStringify would wrongly force a behavior change.

For closed golden **prepared JSON data** only: independently author expected array row order, apply external stableStringify, JSON.parse that text, then JSON.stringify parsed data to recover ECMAScript integer-index enumeration. This validates external nested sorting/escaping plus owned intended numeric-key behavior; golden arrays must independently encode byte-order rows, not call production canonicalValue. Assert exact handwritten expected text in parallel. External parse/reencode is oracle normalization, not production runtime dependency.

## Contract boundary matrix

Object sorting uses UTF16 Object.keys.sort, whereas keyed arrays compare Buffer UTF8 bytes. Include U+10000 versus U+E000: object UTF16 puts astral first, UTF8 array identity puts E000 first. Include integer keys0/2/10 versus nonindex01/4294967295, surrogate/string escaping, NUL-containing identifier values, and ties across multi-field identity keys. Preserve plain array order when any element is unkeyed.

Undefined object values omitted; array undefined/function/symbol become null through JSON.stringify; top undefined/function/symbol must now explicitly reject to uphold string return. BigInt and cycles already fail (cycle currently recursion overflow may differ from third-party TypeError); contract should own refusal state rather than exact engine error message. Dates/custom toJSON are not ordinary JSON corpus: original canonicalValue copies enumerable properties and usually produces {} for Date before serializer, external library calls toJSON. Keep domain narrowed to authored JSON-compatible data plus explicit runtime-negative cases, or specify these behaviors deliberately.

Concrete original edge: target={} then target["__proto__"]=... invokes prototype setter, losing a parsed own __proto__ key rather than serializing it. A closed JSON input with __proto__ is legitimate data; test it explicitly. If fixing, own the change with null-prototype/defineProperty semantics and independent expected JSON; do not silently call it byte-identical extraction. Other integer/string behavior should remain unchanged.

## Full17 original45s evidence/options

Current concurrent receipt has only four30scope row witnesses before45s kill; early owner18.69s/traversal20.35s/inline23.94s pass spans overlap. Concurrency4 and compiler4 did not prove full route benefit. Peak4 permit must remain. Safe optimizations: shrink repeated immutable fixture byte setup via one captured authored snapshot per law; avoid duplicate graph/reference parsing through current canonical executor diagnostic capture; precompile current policy patterns per invocation; optimize actual fresh physical capture/syscall scheduling. None skips native build/runtime/deletion.

Scheduling longest scope law first can expose a lower bound/overlap, but cannot compensate if total actual four-compiler workload exceeds45s. Collect child spawn-start/exit/permit-wait and nonnative stage timings before choosing it. Retain every build args/env/output status and fair permits. Compiler concurrency increase, cached outcomes and aggregating semantically different30mount roots into one binary are not justified.
