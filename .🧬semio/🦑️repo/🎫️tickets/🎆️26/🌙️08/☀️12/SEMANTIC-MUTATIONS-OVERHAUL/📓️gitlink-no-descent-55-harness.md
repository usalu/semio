# Gitlink No-Descent Harness Hardening

Only the ticket controller and neutral schema changed. Normalization, discovery, source-admission schemas, and root inventory were not edited.

The controller now supplies an explicit captured Buffer byte comparator to the extracted walker instead of injecting the nonexported production binding. It forwards the optional sixth walker fence argument. Its collector/prepare wrappers accept the future prepared boundary fields repoRoot, scope, taxonomyPath, ticketDir, cancelFile, indexRows, and repositoryFences. The synthetic index rows are only injected Git stage input; no public taxonomy fence field is invented.

The neutral schema now requires all thirteen cases. The controller rejects duplicate case IDs, has one check command for both current RED and future GREEN, and accepts a rejection only when it is an intentional gitlink/repository-boundary error. Missing names, undefined helpers, and ReferenceError cannot satisfy a reject case.

Current-source execution:

~~~sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️gitlink-no-descent-55/📜️script.ts' check
~~~

Observed actual current RED: 54 assertions across 13 cases, 23 failures. The output is retained at 🧪️gitlink-no-descent-55/🧫️runs/check-0fd2fd48-fa18-4d96-9769-8c9ec571705f/result.json. It demonstrates current traversal into exact/ancestor/NFD/conflicted gitlink roots and the absence of intentional boundary rejects for child/ticket/scope/taxonomy/cancel inputs.

Current extracted normalization hash: 34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f. Hardened controller hash: af58715766b26001a9e477c3258b088b88c7c163d0d98ec7ef832b18d154deb6.

Limitation: current normalization does not yet persist repositoryFences or the prepared full-index contract. The harness records that limitation explicitly and remains RED until root-owned source implementation provides it.

NFD raw-fence extension:

The retained thirteen-case packet now makes `nfd-fence` carry an exact NFD index-fence spelling and NFC-equivalent descendant root and scope spellings. The test keeps the raw NFD spelling in the vector and asserts it remains distinct after NFC comparison. It invokes the actual extracted walker with the NFC descendant and requires a deliberate repository-boundary rejection before any descendant `lstat` call.

The same command observed the held current normalization source as a desired RED: 58 assertions across 13 cases, with 25 failures. The two additional failures are the missing NFC-equivalent raw-fence rejection and the forbidden NFC child probe. Output is retained at 🧪️gitlink-no-descent-55/🧫️runs/check-ba23bddb-c976-40f7-b8f7-5ec6ef930476/result.json.

Captured hashes for this boundary: normalization `34ca6ab7cdf9bee2738766d88d463be76541c405666f52fe6a59c272e3a9588f`; schema `520cfbc7c180c5363654f4aebe76dc70957af8353c9726420bfe5a9c71666b05`; vectors `3ca0ccb0d03809e96ca9f48ae8855254b8ab24fa55976de02cccaf4c7825fc64`; controller `8f38170a48a3857e048ab3fe179f9ad76c077d8f17aea77b3d4484a2eb96ecf6`.

No production files changed. This is a captured current-source RED only; the harness is frozen pending root's coherent helper/signature splice and a non-synthetic GREEN replay.

Untracked Git process-boundary RED:

The neutral packet now declares the raw Gitlink fence arguments separately from filesystem walking: an ordinary Gitlink and an NFD Gitlink fence must each appear byte-for-byte as an anchored `--exclude=/…` argument before `--` and as `:(exclude,literal)…` after it. The controller extracts and invokes the actual `sourceAdmissionUntrackedRows` declaration with its planned fourth fence argument while capturing `execFileSync`; it does not run Git or materialize a fence.

The current source changed during root's helper splice, from the held `34ca6ab7…` to `e6caeab15c32601e9cf9afdb4015fe174b7aa79b6201ce58e652b2e0c3b77c9b`. Against that exact source, the extracted untracked function invoked `git ls-files --others` but omitted all six required raw fence forms: both anchored exclusions, both literal pathspecs, and both raw byte identities. This is the retained genuine untracked RED at 🧪️gitlink-no-descent-55/🧫️runs/check-8301f4d8-d847-4ffd-8f1d-1c6b8bea79d4/result.json.

The same in-flight source added private repository helpers not yet injected into the older walker extraction, causing unrelated `sourceAdmissionAssertRepositoryPath is not defined` extraction failures. Those failures are recorded as source-drift limitations and are not treated as intentional boundaries. The untracked probe itself completed without ReferenceError. Captured hashes: schema `548b855523d18f48686096705203d998e21f409cb2eb201b286e785d32cea130`; vectors `531f71b8865724ee738fe47d15e81126cebee53750980d225a7a290bd21d91d3`; controller `ee2203223f04396495fd30a1b3cf32c8d14a624627d9298eeb3ecc8dcf3bf7d0`.

Coherent helper-signature replay:

The controller now captures the actual repository containment/assertion declarations and injects them into the extracted walker, collector, prepare, and cancellation declarations. It passes the prepared object to the collector, the required fence list to cancellation and walk, and the required fourth fence argument to the untracked reader. The raw literal pathspec expectation follows the current Git grammar exactly: `:(exclude,top,literal)` followed by the byte-preserved fence spelling. No vector outcome was weakened.

The ticket gate is green: 64 assertions across the retained 13 cases, zero failures. Receipt: 🧪️gitlink-no-descent-55/🧫️runs/check-1c963f79-0fea-4122-8a64-e56f7e87880c/result.json. Its exact captured hashes are normalization `aece45f7980f07b393f23e2b0b3cacf7cd1aa8d857d2a63998f7361410a703be`, schema `548b855523d18f48686096705203d998e21f409cb2eb201b286e785d32cea130`, vectors `f5523ac6c9187b7a304dec4d07dd51a1519a3c994b138f9e3ae9634b639a9ef1`, and controller `51d07dbfd034f8abe3f91a21eb2785c5f4c591044a070ba09583d38971da4c5a`.

The canonical source-admission IO suite was then updated only at its private declaration joins and ran through the retained verifier: nine tests passed at both the direct owner and package entry point. Full retained receipt: 📓️admission-verification-53-io-2026-08-28T00-05-00-619Z-🧫️run-SX6hpL.md. This verifies extracted helper behavior; the separate full-inventory refusal guard remains outside this packet.
