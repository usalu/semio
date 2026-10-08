# Job Payload Physical Release Authority

Root's initializer cancellation review identified the current shared job page accounting mismatch. This report records read-only proof and coordinated scope; no production repair or passing test receipt is claimed yet.

The job module owns one16384-byte Box backing per admitted payload page. charge_payload_page at job/🦀️.rs427 accrues per-turn byte grants into a charged field and reports each increment as released_bytes while retaining the page. The final turn frees the whole backing but reports only its final increment. RetainedJobPayload.close_step and both staged/rejected RetainedJobPayloadWriter branches use this helper.

The existing language-neutral physical-close corpus explicitly expects a16383-byte short grant to report16383 released bytes, preserve the exact pointer, then report only1 byte when the subsequent full grant frees the16384-byte page. The independent JSON Patch oracle repeats that accounting. The mounted short-grant native law explicitly expects repeated4096-byte prepayments to free one16384-byte backing. These tests therefore do not currently prove same-turn physical allocation accounting.

The canonical Value authority distinguishes work, copy, allocation and release. A physical allocation remains owned until its full extent fits that turn's release grant; the release is reported once. Its actual native127/128-byte law passed with exact owner retention below extent. The analogous job law must report zero bytes for a short grant, preserve exact pointer/content/ledger, and report the full16384-byte release when admitted. Repeated short grants must not silently become release escrow.

Coordinated ownership: editors/plugins will own the narrow payload helper, payload/writer charged-field removal and exact-demand observations, plus existing physical-close schema/corpus/native and independent source laws. Root owns InteractiveJob default demand and initializer override, plus Worker/Mounted caller propagation. StepOutcome and payload/writer demand must be explicit because their callers also own returned PreviewReady/Checkpoint/Fault pages. Domain work units stay separate from physical-release bytes. No timeout, terminal-empty assertion, cancellation, ledger limit or reader guard may be relaxed.

Execution order: preserve current native gates; author strict language-neutral boundary schema/corpus and native actual red before production changes; align the existing independent JSON Patch oracle; implement exact extent authority; manually align callers/fixtures to declared physical demand; rerun meaningful current Cad, Value, Store and one-unit command gates. Every retained executable is registered through build_pipeline before final acceptance.

## Actual Exact-Page Red and Canonical Repair

The registered native strict-page run exited1: both retained_payload_physical_close_preserves_short_pages_until_the_exact_backing_grant and retained_writer_physical_close_preserves_staged_and_rejected_backing failed. Independent Ajv plus JSON Patch source oracle exited1 with refusedBytes16383 instead of0 and releasedBytes1 instead of16384. These are semantic red results, separate from the earlier corrected schema meta-URI admission failure.

Canonical repair removes accumulated release credits. A current grant smaller than the actual16KiB Box consumes0 and keeps the exact allocation; one adequate current grant releases16KiB. Payload, writer, and StepOutcome expose next_close_byte_demand for their exact active owner. Root owns initializer/Worker/Mounted demand propagation. Neutral native laws now repeat short grants, verify original pointer/content/ledger conservation, and separate zero items from sufficient byte authority. The mounted law retains its fixed phase sequence and one-item work; its release authority queries each phase and accounts the domain one-byte Box as well as its16KiB fault page. Whole retained_ownership_tests retry and independent source retry are pending; no green receipt claimed.

Independent exact-page source actual EXIT0:1pass0fail, strict Ajv fixture validation plus JSON Patch expected output. Log job-payload-exact-page-source-green.log. A stale console sentence about accrued staging was subsequently corrected in the wrapper; numerical assertions and source logic are unchanged. Native whole retained-ownership remains pending.

## Actual Whole Ownership Family Receipt

`job-payload-exact-page-green-retry.log` completed uncached Nx successfully: 34 native cases run, 34 passed, zero skipped, native 0.745 seconds. The selection was the entire `retained_ownership_tests` family. Source probe `job-payload-exact-page-source-green.log` independently passed one strict Ajv/JSON Patch oracle. Physical payload/writer pages refuse every one-below grant without progress or pointer/content loss and release the full allocation exactly once. Runtime page release authority remains distinct from work/copy credit.
