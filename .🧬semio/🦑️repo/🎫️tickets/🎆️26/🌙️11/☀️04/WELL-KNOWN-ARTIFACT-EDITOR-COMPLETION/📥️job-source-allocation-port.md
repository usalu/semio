# Original Job and Source allocation event port

The language-neutral schema and fixture cover seven allocation event scenarios. The Ajv and SQLite oracle passed all seven; this is evidence about the neutral event model, not Rust compilation or live caller execution.

The decoder source now retains its pending request and observed actual births, and the Source encoding owner retains its original physical recipient and pending request. A matching cancel clears only the reservation; a matching post records actual birth before testing the verdict. Mismatched events preserve the original pending request. Source tests authored against the genuine Job fixture cover real backing success, refusal after a cancellation, and cancellation of an unborn backing. The decoder has genuine Job fixtures authored for actual backing success and unborn cancellation. These native tests have not run; rustc AST parsing passed for both production modules and both test files.

The forwarded decoder currently takes its continuation before producer execution. Shared control cannot yet park the complete loan across panic, so a producer panic may lose continuation custody. The Source effect receipt now stages inline caller-owned receipt and provenance failure slots during unwind. Paid original-caller transfer and distinct operation-primary/provenance paths have authored genuine fixture cases; the Source custody schema passed four Ajv and SQLite oracle cases. Native semantic compilation and tests remain pending. Shared allocation producers such as allocate_vec remain one-phase and are not evidence for the new event contract. No native integration completion is claimed.

Selectors: semio-framework-job-context, feature testkit, test original-source-native-encoding; semio-framework-job-native-decode. Both use the framework Job workspace.

The inline Source failure slot has not yet been proven to receive its own original quote and receipt at installation; the later original-caller transfer is staged separately. A mismatched Source event preserves its original pending request but has no original-port recovery/rebind path yet. Both remain open custody work.

Existing Source integration fixtures were ported away from one-phase allocate_vec to allocate_effect while preserving their assertions. The original Source encoding owner binds its original caller identity on first paid use. The shared allocation progress shape is still being reconciled; the fixture callbacks do not panic inside allocation.

The native fixture allocation attempts now borrow and settle one genuine work action; a quote denied before the effect cancels that loan. The existing Source neutral work corpus and Ajv/SQLite test were updated for that actual action and passed one test with six assertions. The provenance failure examples intentionally perform an unadmitted write to exercise failure custody.

The synthetic refusal cases perform no allocation attempt; they cancel the unused work loan and report zero work. Actual backing attempts commit one action.

The Source flat byte receiver also omits the freshly initialized Vec header from its physical quote and receipt. A separately authored Source header schema and illustrative variable-width fixture pass Ajv/SQLite 3/3; they require refusal before birth when the original copy grant cannot fund that header. The fixture widths are model inputs, not assumptions about any target platform. Existing Source native tests do not yet prove header conservation.

The direct Job decoder fixtures also do not yet quote the fresh Vec header. They exercise genuine callback identity, actual allocation, work and cleanup, but are not complete physical-conservation proof until migrated to the canonical typed backing transaction.

An additional genuine Source construction-recipient fixture is authored for an empty Vec header: it determines the real target size, checks insufficient grant refusal before birth, then uses a separately funded exact grant for work admission, header construction, receipt and paid close. It is independent of the still-unfixed flat byte receiver, and has only passed AST parsing.

A lower native encoding effect transaction has been staged under value/🛫️encode/🎟️effect, with a borrowed view over caller-retained effect/allocation slots and independent failure owners. A finite lower-recipient fixture uses a real Vec backing and keeps independent operation, work, physical, and allocation refusals; additional cases cover denied and unperformed effects. Its schema and SQLite/Ajv oracle pass 4/4. Rust source has only AST parsing; the Source boundary is not yet migrated. IO is mapping the whole lower failure without flattening.

Job and Source owners now stage the non-Copy allocation issuer as a caller-funded, exact-scalar birth with explicit paid close and exclusive callback lending. Genuine deny/exact admission fixtures are authored; the issuer oracle passes 3/3. The scoped encoder detach/rebind API binds both original issuer and recipient and returns the live control on refusal; its identity oracle passes 4/4 and its finite native fixture is unrun. Returning a live control is still insufficient to park a failed turn beyond the borrowed StepContext lifetime.

Source typed vector creation and bounded append actions are staged directly over the lower encoder effect transaction; the wrapper is not publicly constructible or yet lent by a production scope. The append oracle passes 6/6.

After the shared recipient API changed, Source also stages separate original work and construction issuers. It admits each issuer in its own caller-funded exact-scalar action and closes them one at a time. A refused admission leaves its issuer retained but unusable. The Source fixture setup and cleanup were adapted; all native cases remain unrun.

Legacy encoder call-chain inventory: native encoding root still calls its one-phase charge in retirement close, retirement wrapper creation, vector birth, text birth, and append growth. Forwarded/scoped original ports now reject that seam, so these cannot be counted as end-to-end event proof. Notably retirement close calls charge even when the actual requested capacity is zero; a bound original port rejects the call before the real close. Nonzero wrapper/backing paths need retained output/progress/failure slots and cannot be repaired with a local temporary.

Issuer birth accounting is under active correction: a fresh shared change added an initialized boolean to the decoder work issuer, making size_of include possible padding. The owners must use first-party per-issuer initialized-byte declarations (currently being added by IO), not assume all issuers have one u64. Existing issuer tests and claims remain provisional until that port.

The original owners now quote each issuer’s authored `birth_bytes()` rather than `size_of` its possibly padded struct. In particular, the decoder work issuer accounts for its initialized u64 and bool. Genuine exact/one-byte-short issuer fixture cases cover both decoder issuers and all three Source issuers; native tests are unrun.

Source typed append processes at most the existing 65,536-byte span per action and writes the consumed length to the caller’s retained progress slot; larger inputs remain resumable rather than being rejected. Its neutral oracle passes 6/6.

FEM retained row transfer now calls reset_completed after the paid turn instead of overwriting the entire progress owner; an occupied settlement is retained and reported. This is AST-only.

A subsequent FEM marker sweep replaced 26 birth resets, four retained-frame progress overwrites, and 18 work-only performed clears with the checked completion method in both dimensional variants and common receiving. Nine affected files parse; compilation and original-caller conservation remain unverified.

The coordinating root thread reported a workspace-routing discovery timeout. Its sole-native-driver restriction remains in force; no competing compiler or native test was started by this lane. Lower settlement integration, Source owned parking/typed scope, the old encoder one-phase producer migration, and rich IO normal-refusal publication remain outstanding.
