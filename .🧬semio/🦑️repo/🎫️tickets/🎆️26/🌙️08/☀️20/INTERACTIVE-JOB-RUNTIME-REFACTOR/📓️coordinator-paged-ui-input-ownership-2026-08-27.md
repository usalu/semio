# Coordinator Paged UI Input Ownership Review

## Inspected Boundary

Read the complete native operation paging report, OwnedUiInstance constructor/maintenance/patch ownership source, and JSON document/string modules. A wrong guessed directory spelling was corrected using rg; it was not evidence loss.

OwnedUiInstance currently captures document limits but no shared resident byte/page lease. Fixed page size, maxPatchBytes, declared field length and actor-count estimates do not reserve resident ownership. UI owns the schema-first host-shared pool and exact lifetime/builder children; composition injection is coordinated with Demonstrator. No new cap, completed admission implementation or physical JS heap bound is inferred.

## Exact Ownership Split

Demonstrator owns Kernel return outer framing and the proposed private OwnedKernelReturnInputField/Fragment/Release types. UI owns OwnedUiOperationPayloadBuilder/Payload and copied/cancelled evidence at retained/operations/wire/pages. The public shapes are a coordination proposal, not source-ready provenance or a mounted path.

UI token names are OwnedUiOperationInputCopied and OwnedUiOperationInputCancelled. Their private verifiers bind fragment, field, builder, BigInt offset and fixed numeric length. Copy success requires leased independent destination pages and detached source readers. Cancellation requires detached source readers but does not pretend bytes were copied; partial destination pages remain under their own bounded close. The peer release receipt keeps its original raw owner on refusal. Field/range metadata alone never mints native authority.

Source must originate from an exact privately captured return-page owner. Records larger than4096bytes may not retain the previous raw page while awaiting another. Each current fragment is copied into separately admitted destination storage and released before the next page; no whole-operation reconstruction, structural callback reader or arbitrary caller-provided byte provider is allowed. Retired page leases are returned only after actual bounded final-reader cleanup, not publication or raw input ACK.

## Additional Review Findings

The JSON document/generic wrapper could perform child terminal work then unlink32bytes in the same grant; UI reports actual RED/GREEN after splitting wrapper release into its own admitted transition. String lookup had a similar terminal child plus128byte transfer; it is assigned to the same coherent repair. The current OwnedSceneReader chunk bound is256bytes, not an unbounded array.

OwnedUiInstance.advanceMaintenance currently overrides every child kind with pending. This hides blocked/rejected states, and it dequeues before the child call. UI owns an actual private-child refusal/throw regression and queue/root retention repair after releasing the current packet. No current test result is retroactively credited with these newly assigned cases.

These source reviews do not establish live host adoption, complete resident census, numeric conversion or8ms behavior.

