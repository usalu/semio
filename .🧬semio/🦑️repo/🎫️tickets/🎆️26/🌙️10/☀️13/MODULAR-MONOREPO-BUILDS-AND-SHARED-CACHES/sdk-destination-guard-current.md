# SDK Destination Guard

The destination owner now streams both initial and final directory membership, admits only an exact immutable subset of the captured publication inputs, preserves root and child directory modes, and counts copied entries independently of its synthetic root and initial inputs. This is destination-tree correctness work; it does not claim native descriptor-anchored protection against adversarial pathname substitution.

## Source and exact input seam

- Destination owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🟦️.ts`.
- Root coordinated the existing private derivative-copy custody checker to return the exact privately captured `NxSdkPublicationInputsV1` copied-byte view. The derivative clone passes that view as the required fourth destination-helper argument. Projection inputs are insufficient: publication receipts can add other exact files.
- The argument is finite immutable work data. It grants no store, publication, resource, Original, or filesystem custody. The existing accepted-copy checker still owns authorization.
- Initial input paths validate against the existing capacity `InputPath` schema. The helper copies at most 32 input byte arrays and 16 MiB before any directory scan. Only those declared paths and their necessary actual ancestor directories can be admitted. Missing declared files are allowed, so an empty tiny clone stage remains valid.
- Every present input must be a same-device regular single-link file with no write bits and exactly the declared size and bytes. Reads use a no-follow descriptor, a bounded 64 KiB chunk, retained descriptor/path identity, and full retained parent checks. Initial directories cannot be empty unrelated names. Final checks retain admitted input identity, mode, size, mtime and ctime without another content read.

## Bounded iteration, identity and lifetime

Both rosters use `opendir` iteration rather than `readdir` arrays. Initial iteration refuses its first unknown name and cannot accumulate more rows than the declared file count plus declared directory ancestors. Final iteration checks each observed name against the finite retained expected set, refuses its first unknown/duplicate/over-limit item, and compares the final exact count. This bounds retained state before consuming hostile directory tails.

All root and nested destination parent checks retain device, inode, directory kind and mode. Parent lstat calls are awaited. Legitimate directory mtime/ctime changes from creating known members remain allowed. Existing enforcing-prefix checks remain present.

Creation reserves a path and count before its first asynchronous parent check, so concurrent accepted registrations cannot overrun the finite copied-entry ceiling or register a duplicate. Exactly `maximumEntries` copied nodes are allowed: the synthetic root and bounded initial publication inputs do not consume that budget. Only copied rows enter the existing Source-layer scheduler; initial rows and root receive separate bounded verification because their paths are not Source `node_modules` requests.

Accepted pending operations still drain before closure, including caught detached refusals. A direct callback Error or thrown undefined remains primary through that drain. No Core, Native, Source-tree owner, canonical SDK publication, ticket lifecycle, or git/worktree mutation was performed by this lane.

## Neutral fixture and independent oracle

`📤️copy/🧪️tests/destination.json` and `destination.schema.json` are a language-neutral corpus and schema. Ajv validates the corpus independently. An in-memory SQLite roster stores expected input blobs and copied entry rows; Node filesystem reads independently compare bytes and actual directory counts. The corpus supplies the reduced exact entry ceiling, directory modes, first-unknown iterator bound, array-read ceiling and input bounds.

Focused actual-store tests cover:

1. Unknown initial names fail before the body; exact package and patch input subsets succeed; changed bytes and unknown patch names fail.
2. Exactly three copied nodes succeed with an initial package file excluded; the fourth fails. Root and child directory mode changes fail.
3. Actual directories containing 32 unknown names stop at the first yielded item during both initial and final scans; an observer forbids roster `readdir` calls.
4. Accepted asynchronous parent checks drain before propagating a thrown undefined callback; caught detached parent refusal still fails scope completion.
5. Thirty-two absent declared inputs are allowed; a 33rd, >16 MiB, invalid path/kind, writable input, hardlinked input, and symlinked input fail.

The existing `sdk-publication-copy` prototype/release launch and ticket `copy` route run these laws; no new executable route or permanent script was added.

## Validation evidence

The first genuine red was recorded before implementation: the existing destination accepted an arbitrary initial `intruder` file, entered the body and resolved. It also independently exposed the existing Source synchronous-ancestry observer failure.

The final helper Source is frozen. The destination lane's final prototype runtime passed **6 tests / 243 expectations / 0 failures** in 11.54 s, including the integrated copier. It observed zero synchronous Source subtree reads, maximum writer concurrency 16, 48 completed copies/links, and no active work at completion. Final prototype strict passed. The root lane independently reported the same 6/243 prototype result in 9.24 s.

An earlier release runtime passed all five focused destination tests; its combined file still had the separately owned Source observer failure of 318 synchronous reads. Root restored the actual Source leaf rechecks to asynchronous lstat/readlink without removing parent or prefix checks; the later green prototype observed this correction. The root lane subsequently reported final integrated **release 6 tests / 243 expectations / 0 failures** in 9.53 s and expanded strict release exit 0. Root expanded strict prototype also exited 0. These successful children use the frozen destination helper Source.

| Gate | Terminal result |
| --- | --- |
| Destination lane prototype runtime, handle 90628 | Exit 0, 6/243, 11.54 s |
| Destination lane prototype strict, handle 83518 | Exit 0 |
| Root prototype runtime, handle 84233 | Exit 0, 6/243, 9.24 s |
| Root release runtime, handle 51975 | Exit 0, 6/243, 9.53 s |
| Root expanded strict prototype, handle 27683 | Exit 0 |
| Root expanded strict release, handle 76615 | Exit 0 |

One final test-only assertion checks that a forged runtime kind fails before filesystem acceptance; it was appended after the 243-expectation prototype child started. The helper kind check was already present in that successful Source epoch. The next full suite should therefore contain 244 expectations.

Generated evidence remains under the ticket `🗑️generated` for the root lane to inspect and remove when closing the ticket. All terminal handles are collected after each bounded run.
