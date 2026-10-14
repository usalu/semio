# Receiving Saturation Source Gap

Read-only source findings on2026-10-14; no native or runtime qualification for this gap.

The actual turn Completed branch takes the request continuation and decodes response fields before receiving admission. Receiving retain_response consumes action/args/fault, constructs ManuallyDrop fields, then searches fixed response slots. Saturation places the constructed owner in a local ManuallyDrop and returns OwnershipLimit without retaining the owner anywhere; it leaks rather than returning the original input. RequestRegistry.take_continuation already removed request occupancy, allowing later requests while previous responses still occupy all response slots.

A simple availability check cannot fix source custody across yields. The canonical resident actor input must retain the same original Completed Event and request continuation until an existing response slot is funded and admitted; pending/refusal must leave input and request source in place. Receiving completion must retain request/source authority through full input close, exact output handback and loan return. Do not widen response capacity, duplicate events, refill contexts or add a separate dispatcher. Enduser is extending the genuine existing resident Entry ingress/decoder and has this precise dependency. Root has not applied an incomplete receiver-only adapter.

Defining paths: plugin/⚛️reactor/🔄️turn/🦀️.rs Completed branch; plugin/⚛️reactor/🔄️turn/📥️receiving/🦀️.rs retain_response; plugin/🌐host/📮️requests/🦀️.rs take_continuation. These are under 🧰️framework/🛍️products/💻️os/🔨️modules.
