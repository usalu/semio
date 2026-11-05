# Publication Page Admission Investigation

The actual actor TypedOperationResultPage::try_new calls bytes.to_vec and stores PageBody::Cold. try_serialize_cold first materializes a JSON String and then calls try_new. Neither API receives an original context or receiving control. This is direct code evidence, not a native heap result.

Current production callers include cancellation rejection, store publication receipts, fault pages, effect/event counts, UI and terminal pages, plus interaction publication. Original fixed-body APIs already exist: from_original requires a 4096-capacity Vec; producer presentation and receiver/producer retirement are separately funded. Worker and latest-wins refusal paths already demonstrate caller-funded fixed backing.

The next investigation is how to reserve backing before an irreversible store commit and retain the exact receipt across denied page admission, while writing the small known schemas without an unbounded ToValue/String intermediate. The current original-emission runtime regression and native publication laws must expose the real physical mismatch before changing this mechanism. No production page changes have been made for this investigation.

Read-only design inventory is recorded in 📬️result-page-funding-design.md. Existing JsonWriteSource/JsonWriteNode can write borrowed finite payloads without a ToValue tree/String. Existing OperationDiagnosticPublication demonstrates actual fixed4096 birth, bounded initialization and from_original handoff. A pending result intent must retain accepted receipt/state across denied page admission; current PendingPublicationOutcome does not carry the accepted LaneItemReceipt. No production page-admission edit yet.
