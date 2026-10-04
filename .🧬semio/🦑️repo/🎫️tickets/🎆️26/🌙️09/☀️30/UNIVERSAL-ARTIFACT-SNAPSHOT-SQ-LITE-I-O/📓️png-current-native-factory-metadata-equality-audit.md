# PNG Current Native Factory Metadata Equality Audit

Read-only actual source and actual protocol-file SHA256 calculation; no runtime factory invocation/test and no edits. PNG artifact root is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png`.

Root `🦀️.rs:35–38` calls native_codec_executables before definition assembly. Its native factory49–55 uses actual `ArtifactCodec::of::<PngSnapshot,PngMutation>("stdio.png")`, overrides extension to png and computes SHA256 of the actual mounted snapshot binary protocol bytes. Factory entry59 names `stdio.native.png.v1`, artifact png and actual artifact_kind callback.

Contract `📇️registry/🧬️contract/🦀️.rs:914–935` resolves the binding by factory ID, invokes actual kind/codec, finds runtime capability by declared ID, derives codec-extension claim, and requires exact factory.artifact/source.artifact, kind.id/artifact_kind, runtime.category=codec, full runtime claims set, codec.schema/artifact_schema, codec.extension/extension and codec.pack_schema_hash/decoded declared hash. native_codec_executables937–950 additionally requires factory count equals executable-registration codec count. It does not silently replace declared metadata.

| Property | Actual / declared current value | Comparison |
| --- | --- | --- |
| factory/artifact | stdio.native.png.v1 / png | matches |
| kind | s.stdio.png | matches |
| schema | stdio.png | matches |
| extension | png | matches; represented format extension .png is a separate field |
| runtime ID/category | s.stdio.png.standard.1-2.codec.codec-stdio-png-extension-png.v1 / codec | actual declared runtime present |
| runtime claims | codec=stdio.png; codec-extension=9:stdio.png:png | matches derived exact claim set |
| actual protocol SHA256 | 94abe1a4afafee64bc8d4b2fa7ff405cfc4def66ce3a81582988888b030622cf | differs from declared pin |
| declared pin | fae0c8a340fe7d7486768794f7f3cad326ec9cd055fe271985154c383c48f9d0 | declaration JSON53 |

Predicted genuine native definition failure is the exact pack-hash inequality. Other inspected fields align. Root's actual metadata runtime witness remains necessary before correction, and source/file hashing alone is not an executed factory result.
