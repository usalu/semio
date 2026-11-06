# Actor Reserved Return Observer

`StepContext::receive_original_reserved_return` adds the actual redemption to both aggregate retained/work observers and separate reserved-return observers. `OriginalActorContextSlot::dispatch_step` can read the exact reserved observers immediately after the producer and before `StepContext` drops; it currently returns only aggregate fields. The wire must retain those aggregate fields, and guest fresh-grant validation must checked-subtract only the genuine reserved part.

The actor also has a repeated-borrow path within one pending epoch. `OriginalActorContext::borrow_context`, `remaining_work`, and the next-epoch `validate_currency` currently treat aggregate work or effects as fresh. A correct change needs actual paid per-epoch reserved observers shared by every such borrow and a funded reset/baseline at the real epoch transition. It must retain both aggregate and reserved values on subtraction or receipt failure. Capturing only an inner producer callback or inferring a redemption from grant sizes does not establish this.

`actor-observer-cases.json` and its schema provide five language-neutral examples, including a valid aggregate larger than the current fresh grant and invalid over-redemption. The independent SQLite oracle passes all five; the actor Source implementation and native laws are not yet updated.

The nested kernel receipt is fresh-conserved: its spent and remaining fields use actual fresh values after checked subtraction of the Source-observed reserved delta. The independently authenticated fixed header retains the full aggregate for SRE reconciliation. Four schema/Ajv/SQLite nested projection cases pass; this is a model and does not establish an implemented actor observer or native proof.

## Implemented Observer Candidate — 2026-11-06

The actual Source context now binds an optional persistent reserved-effects/work observer only before any redemption. Genuine `receive_original_reserved_return` preflights both the local and persistent counters with checked arithmetic, then posts the exact actual redemption to both. It does not infer redemption from a grant, packet or heap observation.

The actor retains explicitly named `fresh_spent` and `epoch_fresh_start` counters beside authentic reserved totals. `StepContext::drop` first posts the full aggregate delta to the external physical receipt, then checked-subtracts that context's actual reserved delta from the actor's persistent fresh counter. This order allows the next real borrow to pass the genuine Source constructor with its fresh stock, while preserving aggregate wire evidence. Aggregate work and its reserved observer remain paired; remaining work uses checked subtraction. A real new epoch resets the epoch work pair and advances the fresh baseline together with the existing actor epoch transition.

The five portable arithmetic cases still pass the independent SQLite oracle. The defining actor tests now include the same neutral vectors and an actual ReturnPlace admission/preissue/redemption law followed by a repeated same-epoch borrow and a later epoch transition. The redemption slice narrows fresh retained axes and work to zero and requires authentic nonzero reserved effects, covering aggregate effects larger than that current fresh grant. These native laws remain uncompiled, so this remains authored coverage rather than a qualified runtime result.

`OriginalActorCall::receipt` still applies its ordinary fresh-only conservation check to aggregate fields. Poll publication must use the authenticated Source header and accepted reserved ledger; it cannot treat an ordinary nested receipt as aggregate proof. Source header publication is being integrated separately by its owner using an exact successful effect plan, independent actual-delta reconciliation, and no pointer exposure on partial failure or mismatch.

The Poll macro now accepts exactly 19 scalar budget fields and forwards the admitted scope, without raw JSON input decoding or bundle initialization before Source dispatch. The actual Poll body is mounted through the Entry-owned Source header helper; its old raw output path is removed. This source integration remains unqualified until actual compiler and native execution evidence is available.

## Owning Native Qualification

One repository MCP run was issued for `test --manifest-path 🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/📦️packages/🦀️rust/Cargo.toml --lib`, prototype configuration and shared store. It uses a finite four-hour delivery limit. The retained inputs are `actor-context-native-qualification.json` and `.ts`; full progress and output are under `🗑️generated/actor-context-native-qualification.log`. The local session is 33477 and the MCP wrapper PID is 68408.

At issuance, the actual actor context root hash was `855c24ce10ea818a36213b0b4c2ce85f92451117f2e0dba33693c4c147e10d89`; the job context root hash was `a95107a15849aa5884eb938dc1c3aa889ab89ddb524440058cbf47ab222413b3`. These are issuance observations, not a claim that the later compiler Source snapshot contains those exact revisions. The job-context owner is performing a legitimate component split concurrently. The run initially waited for machine memory, before compiler admission. No compiler or native test result has been observed.

## Actual Decoder Retirement And Exposure Integration

The actor context now invokes OriginalJobDecodeOwner.close_on_source through an actual retirement-only borrow of its admitted Source. It binds the admitted clock, epoch, Source retained total, aggregate physical/work recipient and reserved observer. The outer actor close does not post that already-posted physical delta again. Denied structural demand leaves the epoch available for the actual funded retry. Existing native decoder/heap/root-close laws remain present; this change is not a native pass.

Actor Slot and Guest Entry now forward the sealed exposure permit and physical HeaderLoanCell to their actual Source parent. The firstparty Job facade explicitly reexports the real publication cell; no new dependency or invented parent is used.
