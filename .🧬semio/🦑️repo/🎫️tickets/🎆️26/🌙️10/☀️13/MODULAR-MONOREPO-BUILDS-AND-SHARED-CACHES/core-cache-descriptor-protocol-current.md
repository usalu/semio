# Core Cache Descriptor Protocol Correction

## Scope and Genuine Failure

The frozen cache audit identified that the actual Core invocation uses `lsof -nP -F0pcfatn`, while the parser split only newline fields. The authored neutral counterexample is status 0 / null signal / reason exit / empty stderr, monitored root `{root}`, guard pid 71 with no permitted paths, and stdout `p72\0cproducer\0\nf3\0aw\0tREG\0n{root}/i/0/output\0\n`. Each `\0` is an actual NUL. The prior implementation admitted the foreign PID 72 file. The executed `core-cache-descriptor-red.log` is genuine RED: seven passing tests, one failure, 57 assertions, expected refusal but received admission. No Native, image, lsof, maintenance, Cargo, compiler, SDK or deletion operation was executed.

The protocol reference is the host's installed authoritative `/usr/share/man/man8/lsof.8`, OUTPUT and OUTPUT FOR OTHER PROGRAMS: fields end in newline by default; `0` requests NUL field termination; NUL process/file sets additionally end in newline; `p` starts a process and `f` starts a file set; optional fields may be absent. OUTPUT documents printable C, caret and hexadecimal encoding for nonprintable name bytes. The reference SHA256 is `b3e81bbde414e488160ead258804afbb09438c49b3aaba2e59f185314cc8ac1e`. The report relies on that documented lsof wire contract, not arbitrary caller text as a source of kernel authority.

## Implemented Closed Protocols

The shared parser now requires an explicit fourth argument, `nul-pcfatn` or `newline-pn`. There is no auto-detection, compatibility overload or fallback. Actual Core `-F0pcfatn` binds NUL mode; pruning budget execution and the existing build-maintenance `-Fpn` calls bind newline mode. Their actual invocation and parser selection are checked together through TypeScript AST and SQLite. The current destination/Native EX, parent charge/floor, recensus and ownership logic is unchanged.

NUL parsing keeps each whole field, including embedded newline in a filename, until its NUL terminator; only a newline immediately after the NUL terminator ends a set. It requires a valid process/file start, a positive safe integer process identity, available selected fields in their correct set, a file name for every file set, no duplicate fields or repeated process identity, and complete final framing. Names from all file sets reach rooted exclusion, irrespective of read/write access field. Newline mode accepts only the actually selected `p` and `n` fields, rejects NULs and requires the final newline. Unknown, ownerless, duplicate, truncated, empty or malformed records refuse instead of being ignored.

The private finite policy is schema-authored and captured/frozen at module initialization: 16,777,216 wire bytes, 524,288 fields, 65,536 bytes per field, 262,144 sets and 32 guard paths. Existing narrower command output/time ceilings continue to apply. All counters are checked while consuming the wire; no whole split into unbounded field arrays is used. Invalid UTF-8/encoded NUL, malformed escapes, foreign rooted records, missing exact guard coverage and unsuccessful/partial observations refuse. Root itself `/` is not an admitted cache root.

Known C/caret/hex name encodings are decoded with strict UTF-8. Printable literal escape spellings are intrinsically ambiguous in lsof's documented printable representation, so monitored root/held-guard paths containing backslash, ASCII controls/DEL or a caret-control spelling explicitly refuse; they are never normalized into a different physical guard. Ordinary Unicode roots, including escaped multibyte names in the existing repository paths, remain supported. Arbitrary descendant names remain whole NUL fields and are inspected; raw/escaped newline foreign names cannot change process ownership. Newline mode relies on the documented lsof printable encoding for filename newlines; it is not a decoder for arbitrary raw multiline path records. This is a conservative refusal boundary, not a new path-to-authority mechanism.

## Tests and Current Strict Limit

The language-neutral corpus has 34 protocol records. It covers the actual NUL failure, exact held guard and guard absence, unrelated root prefixes, foreign read/write names and root directory handles, raw newline inside a NUL name, escaped UTF-8/control names, wrong formats, missing terminators, ownerless/malformed/duplicate fields, unknown fields, unsafe PIDs and invalid encoded bytes. Ajv validates each record. A genuinely received Original carries the bounded independent Python child: Python codecs/re plus a separate process/file-set decoder and SQLite rooted membership/required-guard queries produce exactly the same expected admission list. No fake provider/store/control is accepted. Independent SQLite also checks all three actual Source protocol bindings.

Additional actual tests exercise the exact field-size boundary, over-limit aggregate wire and field cardinality, failed status/reason/stderr, escaped byte decoding, unknown format refusal and ambiguous monitored-root refusal. Existing nine descriptor/guard laws and the existing pruning pressure planner law remain in the received test route. Its JSON fixture now reflects actual `-Fpn` fields rather than a synthetic `f3` row. Strict admission required explicit literal kind/version checks in that fixture; no ambient declarations, type suppression or root removal was used. The old maintenance script's missing required wait field is now explicit `false`, preserving nonblocking exclusion rather than inventing an implicit wait mode. The script was type-checked only and was not executed.

| Final owned route | Actual result |
| --- | --- |
| `core-cache-descriptor-frozen-prototype.log` | Core 11 tests / 150 assertions; Rust integration 23 / 316; existing descriptor+planner regression 3 / 59. All pass: 37 tests / 525 assertions. |
| `core-cache-descriptor-frozen-release.log` | Same 37 tests / 525 assertions, all pass. |
| `core-cache-descriptor-frozen-types-prototype.log` | Expanded actual Core, both production deletion-owner callers, existing pruning test and all original state/context/receipt roots retained. Exit 1 exclusively at the active repository Cargo execution line 87: required `control` absent from `RepositoryExactCargoLawOptions`. No descriptor/owned fixture diagnostic. |
| `core-cache-descriptor-frozen-types-release.log` | Same current external missing-control diagnostic and exit 1. No unexecuted strict successor is claimed. |

Earlier intermediate strict logs are retained: the exact new protocol AST test expected string needed an explicit missing-entry check; the old pressure fixture required JSON literal narrowing and the old maintenance call required explicit wait. These owned diagnostics were corrected. Moving component-pass and build-selection diagnostics appeared in earlier snapshots and disappeared from the final pair; those peer owners were not modified in this cut. Root owns the final ExactCargoLaw migration and integration-wide strict successor.

The existing build-mode state `test` and `types` commands were extended; they are already in both editor catalogs. No new human command, runtime dependency, executable filename or permanent script was added. All original type roots/strict flags remain present. Production four-GiB/eight-slot/one-GiB recovery/eight-GiB creation conditions, CPU/global writer, lease namespace and prior receipts/witnesses are untouched. Physical cache pressure remains unqualified and held. All own handles are terminal at handback.

## Exact Frozen Source Hashes

| Source path | SHA256 |
| --- | --- |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/📂️descriptors/🟦️.ts` | `0415d50eb72ef2390941c5695a8232695455541143d4b0428e46ed7cef9ff51f` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧬️schema/🔣️.json` | `000756a89bc2fb272eb21f856068c3c6ec0c9b846482171a58b141ae71fdac50` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🔣️.json` | `1f620d9f15420fea14fc2b62403dc243a38d009c7810e415f68fc271737c5cf6` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | `814f51e2ef73b4c819bdccaa4f4e75b66a8578916e7f245a0459fcec19c1f532` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/💾️cache/💾️volume/🟦️.ts` | `c80a86c2212220606f2bfa2aa1537d7c661c5499b7899770d9491438fd4c07c3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts` | `2a5eed4139d8a3e7ea2a176d604b8ca69316452aad0e12202436ee00b1340027` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧪️tests/🟦️.ts` | `c3bf49c2d71a55798092417e2ea009b73adedc61b764da627f462b1a49bf0ba5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/📉️pressure/🧪️tests/📂️descriptors.json` | `548b92880f3ac00dc84c38888c93f425020fddf6746d80cfac6b9fccc0944cb0` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-maintenance/📜️script.ts` | `99d499847ed881b73cf2c0300314368b5802e07f79433f2a949ebde227e0de7e` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | `f27fc0fbe80310d12eb7ccce24945fc248fcb97e4ff4a0c3066b32937e5fc692` |

## Retained Evidence Hashes

| Generated log | SHA256 |
| --- | --- |
| `core-cache-descriptor-final-prototype.log` | `99774910fb311a00ee450a8fcf20acbf4b9f9e6ead550e046296124aff9f9679` |
| `core-cache-descriptor-final-prototype2.log` | `88b7d8e628f2f78db594a0ffa895d8014a41c626723f85aee3dbfa7bd58c26f8` |
| `core-cache-descriptor-final-release.log` | `fa320ff41393a16e1a628510d1647c22de0eb307ba7daf94ce664bc72bb061a5` |
| `core-cache-descriptor-frozen-prototype.log` | `2f180da30228d0b943e3b7fe7e2de4bcf67e1e0060df5e1d5d22749d5eaeba1c` |
| `core-cache-descriptor-frozen-release.log` | `00287576f8936f2a468250ede9ef5cdde8dd25e78337c26f35011e2d9a27e6cb` |
| `core-cache-descriptor-frozen-types-prototype.log` | `b556a8d92c32ed560ced3db3e9b05fb88f45a8c18284e6709adcbca8bfbaf138` |
| `core-cache-descriptor-frozen-types-release.log` | `b556a8d92c32ed560ced3db3e9b05fb88f45a8c18284e6709adcbca8bfbaf138` |
| `core-cache-descriptor-red.log` | `9b3caa47768d535cafaf88099df9976263b69ad726ba58984c18f7507fe35cce` |
| `core-cache-descriptor-test-prototype1.log` | `917c1fa49368f9d9306f8836029ac2ec50db028d8c680a0588bab78c035475a6` |
| `core-cache-descriptor-test-prototype2.log` | `decdc238cc18a84a5f462256ec67da9ff0e26a14aba341b9c030cc6cae8435c7` |
| `core-cache-descriptor-types-prototype1.log` | `c7240a4bad406cf78adc7960b36074c56918bc5cffb4579c3e73ed3abe26d1b5` |
| `core-cache-descriptor-types-prototype2.log` | `9bf442a1d35246acc2ad1a60d7f17d69115993ec56a44a6104ab46c820329bd6` |
