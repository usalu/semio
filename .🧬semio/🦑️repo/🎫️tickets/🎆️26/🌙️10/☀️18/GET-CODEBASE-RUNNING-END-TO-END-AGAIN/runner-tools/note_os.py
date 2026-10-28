"""📝 Adds the 00:50 disk + os findings to result-runner.md (one-shot ticket helper)."""
import os
p = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "result-runner.md")
t = open(p, encoding="utf8").read()
anchor = "\n## Verdicts"
add = ("FOR coordinator: URGENT DISK 00:50: the Rust fixed cache image `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m` (disk17s1, 48 GiB) is FULL: 47 GiB used, 213 MiB free → rn-os `framework-os-mcp-rs:test` failed with `No space left on device (os error 28)` archiving wasmtime and cranelift-codegen rlibs in `…/fixed/m/u/l/pbfbfee/b/prototype/build/`. Every Rust compile that writes there will fail until it is reclaimed (`bun repo cache gc` / the diskbud budget did not keep it below the image size); I delete nothing.\n"
       "FOR coordinator: schemaidx SUPERSEDED: the official rn-os run executed `@semio-tech/dsl-derive-rs:generate-schema-index` as a test dependency, which WROTE the live index (\"schema-document-index: written\") — live now holds exactly the batch bytes (`stage.py check` = applied, `verify.sh` = current). Nothing to apply; note that an official test run rewrites this tracked file.\n")
t = t.replace(anchor, add + anchor, 1)
t = t.rstrip("\n") + "\n" + ("| rn-os | framework-os, plugin-registry, framework-os-dev, framework-os-mcp-rs :: test | 00:10 | 2,194 s | 7,510 MB | 1 | RED 3/4 (one GREEN 16.7 s): (a) `framework-os-mcp-rs:test` = disk full (see URGENT) + a real compile error `semio-framework-os-mcp-consent` lib test: `no method named try_recv` on `BridgeOutboxReceiver` (stale test vs API, 3 errors); "
       "(b) `framework-os-dev:test` 3/56+ fail: `🧹️config` 'stays inside the declared module and source-byte bounds' + 'still reaches every module the playground boots through…' (bounds exceeded), and `🧩️contribution` → Node strip-types `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX` on the angle-bracket cast `<[string,string][]>[…]` at `📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts:64` (old code, newly reached by Node); "
       "(c) `plugin-registry:test`: `🎬️host-activation` 1 fail ('web prepare stays boot-scoped…'), `✅️catalog-complete` 2 fail (strict-decode + WebCrypto oracle; ambient roots / max+1 boundary). Graph re-inferred 795 projects under a 1.9 GB memory budget. Not yet diagnosed further |\n")
open(p, "w", encoding="utf8").write(t)
