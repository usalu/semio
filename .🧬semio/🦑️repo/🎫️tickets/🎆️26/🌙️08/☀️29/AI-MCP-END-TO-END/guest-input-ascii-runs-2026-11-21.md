# Bounded ASCII runs in owned Actor Input

The owned JSON grammar receives the original immutable slice through `OriginalJsonOutputReceiving`. After UTF-8 validation, a string turn reserves up to 256 work units at the current measure or materialize frontier. The quote also uses remaining input, item, and work allowances. A materialize quote additionally uses the space already admitted for the string and the caller's remaining physical copy grant after receiver metadata.

The parser examines the contiguous prefix inside the admitted effect. A quote can cover more units than the observed prefix if a delimiter, escape, control byte, or non-ASCII byte occurs within the quoted span. The parser records the exact prefix work before updating the measurement or writing, and reports one copied byte per materialized byte. The batched and scalar grammar report the same totals for semantic work and physical receipts. A buffered string reuses its measured allocation. The scalar grammar consumes the first exceptional byte and produces its ordinary offset, value, or refusal. Cancellation can be observed before the bounded run or at a progress checkpoint.

The independent `serde_json` oracle and a one-unit grammar walk agree with the batched output on ordinary punctuation, escapes, multibyte scalars, surrogate pairs, a 330-byte string crossing the 256-unit cap, a control byte, an invalid escape, and an unclosed string. The native test compares the complete grammar receipt and work on both schedules, requires fewer actions for the batched schedule, and checks that copy denial and cancellation both leave zero receipt before the affected write.

The focused native test passed using the repository's pinned Rust toolchain, the locked offline framework workspace, and a serial target isolated under the ticket's generated directory. The regular repository command waited on machine capacity. The older direct-link cache could not supply the exact native dependency metadata, so the focused test used a new target with the same sources and pinned compiler.

The broader native run passed all 11 unit tests across `semio-framework-pack-json`, `semio-framework-pack-json-output-receiving`, and `semio-plugin-actor-input`. The InstanceOpen case compared Note and WFC wire values against `serde_json`, reached ready under both the infrastructure and bounded grants, and retired both instances. The unique-output suite checked invalid UTF-8, cancellation across typed prefixes, and original-source retirement. The borrowed Actor Input case verified the original wire identity and its return boundary.

The serial native build took 14m 02s, and the five Actor Input tests ran in 6.95s. The framework-kernel compiler process was observed at approximately 864 MiB resident during one sample, and the Data volume had 15 GiB available near the end of the build.

The subsequent native schema and receiving-frame run passed all nine tests in 29.10s with approximately 498 MiB maximum resident size. The owned record matcher driver passed both tests in 22.54s with approximately 388 MiB maximum resident size.

After the flat record matcher and its derive landed, the focused `mcp_instance_open_admits_the_descriptor_grant` test passed again against both optimizations together. The fixture ran in 3.02s after the 11m 05s rebuild. The command took 670.06s wall and reached approximately 1.65 GiB maximum resident size.

The direct matcher rerun after a later one-unit-short Work assertion passed the native schema case. The flat matcher case stopped with an original producer retirement panic. The matcher owner received the failing fixture and isolated target for diagnosis.

The source and fixture paths for this change are below the repository's 256-character limit. The isolated Cargo layout generated temporary build paths up to 277 UTF-16 code units under the ticket's generated directory. The matcher owner will remove the isolated target after the follow-up.

The earlier Wasm sample recorded 1.347 billion engine operators on the first Actor Input. Its engine count remains the baseline for component-level verification.
