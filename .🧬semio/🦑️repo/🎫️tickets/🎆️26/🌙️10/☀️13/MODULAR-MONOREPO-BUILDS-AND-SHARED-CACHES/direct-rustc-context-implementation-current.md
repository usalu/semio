# Direct Rlib Compiler Context Implementation

2026-10-16. This is a Source/neutral qualification of the new direct rlib identity. No Rust compiler, Cargo, image, download, SDK, publication or retirement was executed in this lane. Prior actual Cargo observer94731 remains the independent runtime characterization in `direct-rustc-context-actual-current.md`; it is not a runtime qualification of this implementation.

## Identity and Actual Consumer

`CargoCompilationUnitV1` now requires `packageVersion` and `edition`, populated directly from the actual Cargo metadata package and unit target. Neither comes from the shortened unit id. Both actual materializers require `packageIdentity` (full original packageId, version and provenance), target edition, operation and noncompiler `evidence`; dependency edges require public/noprelude. Existing authored fixtures and actual caller signatures have been updated without optional fields or old-shape support.

The adjacent repo unit context owner defines `direct-rlib-v1`. Its canonical facts include actual package name/full identity/version/provenance, manifest and target (name, entry, kind, role, edition, triple), operation, mode, every observed effective profile field, cfg/features/full compiler cfg snapshot, complete compiler/sysroot/linker identities, exact ordered consumed arguments, every declared semantic environment key/value/presence, and predecessor context digests with externName/public/noprelude. Source/config input byte hashes, synthetic planning observations, predecessor content/rlib digests, output locations, and bookkeeping ids are outside this compiler context. UTF-8 byte ordering defines keys and set-valued cfg/features; strings must be Unicode scalars, argument order remains exact. SHA256 lowercase hex is both the full context digest and actual metadata value.

The actual direct rustc port recomputes this context against the actual predecessor plans, compares it with the planned snapshot before its cfg command/effects, and passes `-Cmetadata=context.metadata`. It authenticates the exact Original/control using the canonical read-only guard; removing the serialized wire does not remove parent ownership. Public/noprelude flags remain represented, but the current adapter refuses either non-default edge before effects instead of inventing unsupported rustc syntax. The graph materializer reports `extern-flags` and propagates the unavailable adapter to consumers.

Full plan.digest hashes the entire normalized unit, full predecessor semantic/artifact identities **and** the new context. It remains source dependent. Existing Nx digest environment/metadata, artifact receipt semanticDigest, dependency receipts, Source rechecks, compiler code inputs and actual consumed-byte checksum checks still use their original complete content identity. Dependency-artifact binding preserves context. The output-directory policy is unchanged; the required edition fact naturally participates in its existing target-based key. No old output was adopted or deleted.

## Compiler Environment Boundary

Exactly two producer observations previously placed in rustc environment (`SEMIO_CARGO_UNIT_EVIDENCE`, `SEMIO_CARGO_PROFILE_DIGEST`) now live in required evidence.compilation/profile and remain hashed by fullplan. Their prior calculation is retained. A bounded Source census via `rg` over all Rust files in framework and s found zero consumers of exactly those two names (exit1, zero output). This is not an exclusion rule for SEMIO prefixes. The relation corpus proves unrelated custom SEMIO_INPUT and SEMIO_CUSTOM remain semantic.

`rustCompilerExecutionEnvironmentV1` removes only `SEMIO_SCRIPT_PROCESS_INVOCATION` for a nonreceiver rustc. Actual Core planning cfg/version probes, Source-admission cfg and dependency-free descriptor cfg use that projection. The direct compiler uses its closed unit environment directly and rejects an Original wire in that environment. Cargo/Node/script parents keep their real Original transport/control. The generic owned command runner supplies an explicit env rather than merging the ambient process env.

The actual compilation still uses the same owned transient TMPDIR/TMP/TEMP and fixed one-worker scheduler variables. Existing Rust Source environment admission refuses transient/unowned literal env!/option_env! observations; its earlier actual TMPDIR oracle proved emitted `# env-dep` changes and refusal. This lane has not rerun that physical oracle. Existing consumed-byte validation is checksum/Source membership verification; it does **not** independently compare every emitted env-dep line. Expanded/unresolved macro/environment mechanisms beyond the current admitted Source policy remain a named boundary. No claim of universally complete environment dependency analysis is made.

## Executed TDD and Qualification

* `rlib-context-red3.log`: genuine feature RED, context absent/undefined, 0pass/1fail after the schema fixture's initial Ajv union-policy mistake was corrected (`red2` is harness-only).
* `rlib-context-environment-red.log`: genuine missing exact rustc environment projector, 5pass/1fail.
* `rlib-context-unicode-red.log`: independent Python hash disagreed for astral/BMP environment-key ordering, 5pass/1fail. UTF-8 canonical ordering corrected the actual first-party owner.
* The first combined two-file helper run (`neutral4`) honestly failed its second file after the first file's receiver afterAll closed. Existing profile regressions now run in a separate owned Bun process under the same already-authored120s Original. There is no receiver reset or grant renewal.
* Final prototype and release each pass **15 laws / 561 assertions** across the new6/143, existing effective-profile6/391 and existing unit/slice3/27 cohorts. The unit/slice cohort explicitly filters32 other laws; none of those physical/ordinary laws are claimed here. New coverage includes independent Python hashlib, Ajv, SQLite relation closure, actual primary reader/materializer under genuine carried control, complete profile/context mutations, exact metadata AST, transport exclusion, unsupported extern refusal, full content invalidation and immutable context snapshots.
* Expanded strict includes the actual Core primary owner/profile tests, both materializers, actual rustc port, receipts, full existing unit tests and selected configuration/slice tests plus the ticket route. The final separately collected prototype and release strict processes each exited0 with no diagnostics (handles47097/82793). Final captured log hashes are recorded below. No declaration stub, root drop or allowJs workaround is used.

Commands use the existing build-mode test/types route with `SEMIO_RLIB_CONTEXT_ONLY=1`; both editor catalogs have explicit prototype/release context test/types entries. All actual child scopes share the existing120s outer grant. Ordinary Cargo/physical commands are not selected.

## Remaining Runtime Work

Actual direct-rustc compilation with this new metadata identity, dep-info and rlib ABI must be qualified on an authorized physical window, followed by a fresh matching captured compiler/SDK and ordinary Nx evidence as appropriate. No runtime cache/warm-time improvement is inferred from Source checks. Persistent charged incremental context slots, authenticated retirement, strict writer exclusivity/concurrency, and durable growth control remain separate future implementation; temporary incremental cleanup, global writer policy, fixed filesystem caps and the4GiB floor are unchanged. Production build-mode semantics and mode names remain unchanged.

## Defining Source Roster

Maximum absolute UTF16 path length in this roster: 170. Both editor catalogs were extended surgically, preserving existing env/order/coedits.

| Source | SHA256 |
|---|---|
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts` | `b4ba3465a8b5787a77b703ad73bcb8eabd8b8a4ba63a01fd39a38d57e0be52fe` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧬️schema/🔣️.json` | `ad6e7732c4d5de4340f675fc8f809dd47dbdd5d157eff97cc0edaf0288e9db78` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts` | `8ed0c7344879a2dc5b7cbf496f66316b31aee8400d08b73a5d798952368c145f` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/🟦️.ts` | `c26fdcc3a7a24addd9cc0c99c451626b6aae0836f596e956d1a06d21363ccd8a` |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` | `8a41c08dbc0da8fb3080f6ea7141004f978905de2183b38ed6b61ff9e36a5ad9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🟦️.ts` | `331996a2f01ba082fbd111b0b15e875d3134704cf0864256b4ad071ea0262df7` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧬️schema/🔣️.json` | `5343de83befe44ce745edeb1073d9fae69b866efce4dd8fad5ee258427b0ab31` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🟦️.ts` | `81c71974e5d18260482dc1d022a081f39c94a4f615168b58a0ae16aa11cbb45e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🟦️.ts` | `668bcb00426828eb7f7e9877045d249354c326e5b5d31fc9a1d6b9df6cb62781` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️graph/🧬️schema/📥️refusal.json` | `71ebd88d004c9b3e92065c650cedcd508e2d01446f9c16b60d8606e2716fb688` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts` | `06bff8725169a89baa1c99677950541c4760846d040ad3efa2ca70f840063ef8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🧫️fixtures/🔣️.json` | `f3165edc73662d3a0105d2d6a9c29f24674b9d803e2dba01e7b81ce72747678e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧪️tests/🟦️.ts` | `aac1ff32312350463b2af4721687a489581398e74961e05c05b8bacd0ebcd2f4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🧪️tests/🟦️.ts` | `6a8a3146ec7b904a53e40ff757dd5a48b8db70f4afd168d0a8b857669c7aabff` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | `158a9369cab9d0ddc74c4a0c7eb6872801dec1765f939321dc76e4b29028cdac` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🟦️.ts` | `287cc1f3eacd616cd3bb15b7907f377b984cbc65afbbd341c7e5ba3639268f34` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧪️tests/🔣️.json` | `823455bf524adaa644b1e0ae81a4f82df88e6850eca9ca1b3b037c077a71834a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧪️tests/🟦️.ts` | `f7405537492b71cfadbb64e61bbee7f4414c792f4ff4d22c924665d0dea133b8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧪️tests/🧬️schema/🔣️.json` | `796cef9510315a9e96129d9496f8142da3690a516bc5e957ffaa931cd069ff42` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/🧭️context/🧬️schema/🔣️.json` | `f16f34dd65e3e75dfd63fa924917076c6c521b96b9c0667f4540eb3ec211e0db` |

## Retained Execution Evidence

| Generated log | Bytes | SHA256 |
|---|---:|---|
| `rlib-context-red2.log` | 2514 | `5b1c79956d1aa5b3c4093f0be944f4f7179d5cb75da34f0821fb0d0002d7e43c` |
| `rlib-context-red3.log` | 2679 | `5b69eb65d771160a0697cd80b916edabd7d8ba644dd2aeb57be256941fbd1eb2` |
| `rlib-context-environment-red.log` | 5720 | `fc0be1dbfb991fd2e3e222220718fb9acead91c6b05f534d3135a14042c772b9` |
| `rlib-context-unicode-red.log` | 5869 | `26e4e021f24e1ceec720c361178ea31abcffa946a4df86d7330c087977f35949` |
| `rlib-context-neutral4.log` | 5710 | `bfc638b0613d1f3484f8dcb70a163256770ad742a2df2124d136e1fbc5fd3d28` |
| `rlib-context-final-prototype.log` | 6908 | `e12c4b58e0b0a58d2727157be1b7ec22a207032ec8cf60da31f999cb10553154` |
| `rlib-context-final-release.log` | 6908 | `96fcf0288a10e10d652e6b6d3d42134d2a8e2b2840cbe25cc234fdcf143ee01d` |
| `rlib-context-collected-types-prototype.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `rlib-context-collected-types-release.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `rlib-context-rust-env-census.log` | 0 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Frozen Handback

All Source in the roster is frozen at this checkpoint. Final prototype46591 and release13173 (test portion) completed naturally with no failed law. Collected strict47097/82793 each exited0. No owned process, image, store, compiler or tools reader remains live. The caller/schema shape intentionally requires a fresh matching captured implementation before any physical ordinary qualification.
