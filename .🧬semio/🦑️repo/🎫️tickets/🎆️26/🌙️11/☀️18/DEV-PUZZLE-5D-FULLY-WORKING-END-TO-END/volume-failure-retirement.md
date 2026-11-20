# Target-volume command failure retirement

Six integration tests reached the controlled-retirement destructor while formatting the publication failure. A native build with symbols traced the dropped owner through `DomainPayload<OriginalPublicationFailure>`, `PluginLifecycleFailure`, and the 5D fixture’s settlement error mapping. Inspecting the formatted failure before the destructor ran confirmed a `member.publication` grant refusal with zero effects.

The 5D testkit now projects the diagnostic, transfers the failure to the existing fixture retirement driver, and returns the diagnostic after retirement completes. The regular and traced dispatch paths use the same helper.

Changed source: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️testkit/🦀️.rs`.

The next full run returned the underlying diagnostic on all six document edits. The testkit paid for retiring each failure and the secondary destructor panic disappeared. The surfaced refusal led to the independent digest work repair described in [canonical-seal-work.md](canonical-seal-work.md).
