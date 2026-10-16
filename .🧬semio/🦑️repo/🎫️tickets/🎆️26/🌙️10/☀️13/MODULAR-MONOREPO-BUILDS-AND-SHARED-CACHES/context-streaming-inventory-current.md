# Context Observer Streaming Inventory

2026-10-16. Root identified an actual preallocation gap in the staged five-phase Cargo observer: `await readdir(path)` materialized a complete directory listing before the existing8192-entry census checked its children. The actual Cargo observer remains unexecuted; this correction does not change compiler context/metadata, incremental persistence or production storage APIs.

## Source Correction

`📐️context/📚️inventory/🟦️.ts` now owns the same complete read-only fixture census. It uses `opendir(...,{bufferSize:1})` and explicit `read()` calls, synchronously charging each returned entry before retaining its name. Root counts once, reserved children are not charged again during descent, and all pending name arrays share the same total entry accounting. Names are sorted only after bounded enumeration. Each directory handle closes before descent, so the scanner itself retains at most one directory handle at a time. The existing settlement helper preserves primary Error or undefined over a close error and makes standalone close failure fatal.

The existing actual observer now directly calls this helper instead of its inline readdir traversal. Full perphase roster, regular-file/directory-only checks, symlink refusal, exact file byte totals, actual carried control checks and complete Source/artifact checks remain. The unchanged defaults are8192entries(includingroot),4096files and2MiB total logical files. The schema permits only finite narrower DATA limits for the tiny negative vectors; no policy declaration is mutated. Node directory-buffer size1 and a continuation every64 charged entries are schema-owned. This does not claim atomic beneath-root opening or adversarial filesystem mutation protection.

## Genuine RED and Independent Tiny Oracle

The language-neutral corpus now includes one actual tiny directory with two files (one nested directory, one UTF8 value), plus exact-limit and entry/file/byte minus-one vectors. The test receives a genuinely issued control from the existing registered build-mode test receiver. Its only ordinary files are inside the ticket generated root and its exact temporary directory is removed before callback settlement. No Native store, image, compiler, listener, registry namespace or lease protocol is opened or modified.

The positive complete roster agrees with independent Bun.Glob traversal; SQLite independently computes exact file count, ordered file rows and UTF8 byte sum. Ajv validates every narrower vector. The exact four-entry boundary (root+directory+twofiles) passes; three entries, one file and one fewer byte each refuse. A TypeScript AST check verifies the actual observer calls the extracted owner, and the owner uses opendir/read/close with no readdir call. It also binds the schema maxima back to the unchanged actual observer limits. This is bounded ordinary-directory/Source qualification, not a Native or Cargo result.

| Gate | Handle | Terminal | Result |
| --- | --- | --- | --- |
| Missing bounded enumerator RED | direct terminal | exit1 | context-inventory-red1.log; missing module before test execution |
| Prototype tiny/neutral |98611|exit0|16tests/641assertions,1.85s |
| Initial prototype strict |43809|exit1|one own SQLite test-query result type error; no Native error |
| Release tiny/neutral |16198|exit0|16tests/641assertions,1.478s |
| Final expanded strict prototype |68360|exit0|strict ES2022/noUnchecked |
| Final expanded strict release |6791|exit0|strict ES2022/noUnchecked |

The strict correction only supplies the actual SQLite row type through its query generic; it does not change the SQL or test runtime behavior. All actual handles are terminal. No ce-* fixture directory remains. The previously recorded Native moving-prepare failures remain retained, and no Native API import/export was restored or removed by this lane. No new command, script, editor route, budget, mode, namespace or capacity was added.

## Handback

Observer Source is frozen at the hashes below. The existing five-phase physical context command is still held until Root closes the coherent Native successor and explicitly schedules it. No actual metadata equality, warm incremental reuse or timing improvement is established by these cheap gates.

## Exact SHA256

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/📚️inventory/🟦️.ts`: `18f5a759549854c90ac21af08754fc5ba4263c8a6e8f80a95e1ebeb2d41d3fdb`; absolute UTF16 length162.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🦀️.ts`: `b3aaef25d618ca2b6f48b56e011abe7a0736cac274f21015bf75925b7ee13c23`; absolute UTF16 length149.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧬️schema/🔣️.json`: `60771c600c11925985589b6da1c00338ab9b783525b0a0d0ab2ab0c1c6b5eecb`; absolute UTF16 length161.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🔣️.json`: `771be9132a769dd21a2690dabe66d2877f29414e259f971d995ba791bdc376d5`; absolute UTF16 length151.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧪️tests/🟦️.ts`: `0dc2901499839140f60660bb509dd5da6c4669bc3c1099a915ea67b2466a4874`; absolute UTF16 length158.
- `generated/context-inventory-red1.log`: `4059cbe43585878c86a40e54da16050734a7b12e1a76363271bd58d4ee0f0f2f`.
- `generated/context-inventory-prototype1.log`: `84f178fd80621d606d0971e0cec9a1fe44d8b0c2fc55a503515aa61060589ca6`.
- `generated/context-inventory-release1.log`: `3076cebb47301d9a3f84117056ba1c8558f58859260825a9da320e048b315b81`.
- `generated/context-inventory-types-prototype1.log`: `f681c233a523b78e1c376a65e307f50260aa5cf890596028c7a7dc85c770ded0`.
- `generated/context-inventory-types-prototype2.log`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- `generated/context-inventory-types-release2.log`: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
