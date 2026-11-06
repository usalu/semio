# Window Publication Error Types

WindowConfigOwnerRegistry::advance at plugin/🪟️window/🎚️config/🧭️owner-registry/🦀️.rs:187 returns Result<ArtifactStoreOneItemAdvance, Fault>. WindowTransientOwnerRegistry::advance at plugin/🪟️window/🫧️transient/🦀️.rs:543 has the same raw Fault error type. Neither API returns PluginLifecycleFailure or an already-paired receipt. Pairing their actual Fault and its retained progress with the enclosing receiving receipt therefore does not need to overwrite an existing lifecycle receipt.

A distinct custody defect appears in transient advance lines 548-554: it shadows the passed NativeDecodeControl with a new_retained control and an always-true callback, then passes that replacement to its owner. The input control from the original caller must reach the actual owner. A genuine original-decoder identity and receipt test is needed; a type-only test does not establish this.

No production edits or native builds were performed.
