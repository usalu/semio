# Owned Completion Accepted Cancellation Route

Source-only audit, no edits or native execution.

The owned completion route obtains the actual effect authoring receipt once, or resumes the previously retained receipt. EntityIdentityAuthority::resume and pause are scalar continuation transfers: they do not create a new ceiling, invoke an encoding checkpoint, or query cancellation. The observer still checks the actual lease token/publication claim when new identity work occurs. Therefore a cancelled observer does not itself prevent entry into the publication pump or accepted-page retirement framing.

The common pump retains actual grant/depth/fuel limits and prioritizes acknowledged-page cleanup, pending page custody and accepted Store receipt before cancellation/freshness. Bypassing should_yield for retained custody does not issue more items/copy/capacity/release/depth. Actual paid page operations remain admitted through original retirement and the real decoder. Stable UI/download sources are retained in mounted.publication across framing; resumed identity does not replace that source.

One concrete asymmetry remains: publish_mounted_typed_operation_run unconditionally calls follow_derivable_children after a successful pump, while publish_original_emission_run skips it when the actual lease is cancelled. The child follower can construct projection/genesis ownership, retire children and open derived children through the cancelled identity. Thus an accepted frame can advance successfully and then be followed by unrelated cancelled authoring failure or new derivation work. Root was advised to apply the same cancellation guard before this follow step, while always pausing/restoring the actual identity receipt.

The outer publication-unit caller reinserts mounted custody on returned failure. Post-run revalidate already skips original/lease cancellation before resuming authoring. No cancellation-only quota/depth rejection was found before accepted-page handling itself. Actual owned-completion cancellation behavior still needs native regression.


## Neutral follow ordering law

Before root guard change, authored five ownedFollow rows in existing page-admission schema/corpus and sibling follow.test.ts. Context, token, document and instance cancellation all preserve accepted page count1 while forbidding follow/derive; normal execution follows once. Every trace ends by pausing the same original receipt exactly once. Strict Ajv validates the neutral section; SQLite event trace independently counts accepted/follow/derive/pause actions and verifies final pause. Actual run: 1 pass, 0 fail, 11 assertions, 332 ms. No Rust code changed and this model is not native guard execution proof.

## Production Guard

Root added the cancellation guard before the owned completion route follows derivable children: an already cancelled context or actual publication lease returns successful page progress and still pauses the same authoring receipt through the existing common path. No new child derivation is attempted after cancellation. The production file passes AST parsing. Native verification is queued; the neutral trace is not a Rust runtime pass. Root and runtime production are frozen for the next coherent native snapshot.
