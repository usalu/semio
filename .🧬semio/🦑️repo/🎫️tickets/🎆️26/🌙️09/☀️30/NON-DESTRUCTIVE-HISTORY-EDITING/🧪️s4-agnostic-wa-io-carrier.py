#!/usr/bin/env python3
"""🗃️ D2 W-a step 1, framework half (S4-AGNOSTIC, design §20.15): `io_mechanism::Serializer::serialize(from, children)`, the composed
head carrier recognised by `serializer_entry`/`serializer_entry_text` (`native_carrier`), the archive version named once in the channel
(`DOCUMENT_ARCHIVE_VERSION`), and the law `a_composed_head_carrier_hands_its_owned_children_to_the_serializer`. One count-asserted
write per file, every file re-read immediately before its write; idempotent (a file already carrying the change is skipped).
Usage: [--apply]; without it a dry run."""
import sys

ROOT = "/Users/ueli/Documents/semio"
CHANNEL = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs"
IO = "🧰️framework/🔨️modules/🚪️io/🦀️.rs"
LAWS = "🧰️framework/🔨️modules/🚪️io/🧪️tests/🔬️io-mechanism-laws/🦀️.rs"

EDITS = {
    CHANNEL: [
        (
            "pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4 * 1_024 * 1_024;\n",
            "pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES: usize = 4 * 1_024 * 1_024;\n/// 🔰️ The leading byte of every encoded document archive (a snapshot pack starts with its magic `0x89` instead), so a reader tells\n/// a composed carrier from a plain pack by its first byte.\npub const DOCUMENT_ARCHIVE_VERSION: u8 = 1;\n",
        ),
        ("    let mut bytes = vec![1];\n    write_document_archive(&mut bytes, archive);\n", "    let mut bytes = vec![DOCUMENT_ARCHIVE_VERSION];\n    write_document_archive(&mut bytes, archive);\n"),
        ("    if bytes[0] != 1 {\n        return Err(malformed(\"document archive version\", 0, \"unsupported or missing version\"));\n", "    if bytes[0] != DOCUMENT_ARCHIVE_VERSION {\n        return Err(malformed(\"document archive version\", 0, \"unsupported or missing version\"));\n"),
    ],
    IO: [
        (
            "    /// 🎹️ A typed native-value → foreign-payload encoder. `INTO`/`FIDELITY` are the foreign\n    /// dialect and the strongest fidelity this serializer achieves.\n    pub trait Serializer<S> {\n        const INTO: Dialect;\n        const FIDELITY: IoFidelity;\n        fn serialize(from: &S) -> impl std::future::Future<Output = IoResult<IoPayload>> + Send;\n    }\n",
            "    /// 🎹️ A typed native-value → foreign-payload encoder. `INTO`/`FIDELITY` are the foreign\n    /// dialect and the strongest fidelity this serializer achieves. `children` are the owned members of a composed native (design\n    /// §20.15: composed content is read on read, never through the parent's local owner) — the empty view for any other native.\n    pub trait Serializer<S> {\n        const INTO: Dialect;\n        const FIDELITY: IoFidelity;\n        fn serialize(from: &S, children: &ArchiveChildren) -> impl std::future::Future<Output = IoResult<IoPayload>> + Send;\n    }\n",
        ),
        (
            "        /// 🔎️ The archived member at `slot`/`child_id`.\n        fn find(&self, slot: &str, child_id: &str) -> Option<&store::channel::OwnedDocumentMemberPackEntry> {\n            self.members.iter().find(|member| member.owner.slot == slot && member.owner.child_id == child_id)\n        }\n    }\n",
            "        /// 🔎️ The archived member at `slot`/`child_id`.\n        fn find(&self, slot: &str, child_id: &str) -> Option<&store::channel::OwnedDocumentMemberPackEntry> {\n            self.members.iter().find(|member| member.owner.slot == slot && member.owner.child_id == child_id)\n        }\n    }\n\n    /// 📨️ A binary native payload split into the parent's native bytes and its owned children: a composed artifact's head carrier\n    /// (`store::channel::encode_document_archive_bytes` of `{parent HEAD native, empty parent history, members}`, first byte\n    /// [`store::channel::DOCUMENT_ARCHIVE_VERSION`]) yields its parent bytes and members; any other payload is the parent alone.\n    fn native_carrier(bytes: &[u8]) -> IoResult<(std::borrow::Cow<'_, [u8]>, ArchiveChildren)> {\n        if bytes.first() != Some(&store::channel::DOCUMENT_ARCHIVE_VERSION) {\n            return Ok((std::borrow::Cow::Borrowed(bytes), ArchiveChildren::empty()));\n        }\n        let archive = ::semio_framework_async::poll::resolve_ready(store::channel::decode_document_archive_bytes(bytes)).map_err(|error| refusal(ValueRefusalKind::InvalidValue, format!(\"composed carrier: {error}\")))?;\n        if !archive.parent_spr.is_empty() {\n            return Err(refusal(ValueRefusalKind::InvalidValue, \"composed carrier: a head carrier holds no parent history\"));\n        }\n        Ok((std::borrow::Cow::Owned(archive.parent_pack), ArchiveChildren { members: archive.members }))\n    }\n",
        ),
        (
            "    /// ever hand-writing an `IoEntry` literal while never leaking pack/DSL encoding into its code.\n    pub fn serializer_entry<S: store::ArtifactPack, T: Serializer<S>>(own: Dialect) -> IoEntry {",
            "    /// ever hand-writing an `IoEntry` literal while never leaking pack/DSL encoding into its code. A composed head carrier\n    /// ([`native_carrier`]) hands its owned children to the serializer.\n    pub fn serializer_entry<S: store::ArtifactPack, T: Serializer<S>>(own: Dialect) -> IoEntry {",
        ),
        (
            "                return Err(refusal(ValueRefusalKind::InvalidValue, \"serializer_entry: expected a binary native payload\"));\n            };\n            let value = S::decode_pack(bytes).map_err(|error| match error { store::PackError::TextRefusal(error) => text_refusal(error), error => IoError::from_value_error(error.into_value_error().under(\"native pack decode\")) })?;\n            ::semio_framework_async::poll::resolve_ready(T::serialize(&value))\n",
            "                return Err(refusal(ValueRefusalKind::InvalidValue, \"serializer_entry: expected a binary native payload\"));\n            };\n            let (parent, children) = native_carrier(bytes)?;\n            let value = S::decode_pack(&parent).map_err(|error| match error { store::PackError::TextRefusal(error) => text_refusal(error), error => IoError::from_value_error(error.into_value_error().under(\"native pack decode\")) })?;\n            ::semio_framework_async::poll::resolve_ready(T::serialize(&value, &children))\n",
        ),
        (
            "    /// payload law's native encoding (pack XOR DSL).\n    pub fn serializer_entry_text<S: store::ArtifactDsl, T: Serializer<S>>(own: Dialect) -> IoEntry {",
            "    /// payload law's native encoding (pack XOR DSL). A composed head carrier ([`native_carrier`]) whose parent bytes are the\n    /// parent's DSL text hands its owned children to the serializer.\n    pub fn serializer_entry_text<S: store::ArtifactDsl, T: Serializer<S>>(own: Dialect) -> IoEntry {",
        ),
        (
            "            let IoPayload::Text(text) = payload else {\n                return Err(refusal(ValueRefusalKind::InvalidValue, \"serializer_entry_text: expected a text native payload\"));\n            };\n            let value = S::parse_dsl(text).map_err(text_refusal)?;\n            ::semio_framework_async::poll::resolve_ready(T::serialize(&value))\n",
            "            let (value, children) = match payload {\n                IoPayload::Text(text) => (S::parse_dsl(text).map_err(text_refusal)?, ArchiveChildren::empty()),\n                IoPayload::Binary(bytes) if bytes.first() == Some(&store::channel::DOCUMENT_ARCHIVE_VERSION) => {\n                    let (parent, children) = native_carrier(bytes)?;\n                    let text = std::str::from_utf8(&parent).map_err(|_| refusal(ValueRefusalKind::InvalidValue, \"serializer_entry_text: the composed carrier's parent is not DSL text\"))?;\n                    (S::parse_dsl(text).map_err(text_refusal)?, children)\n                }\n                IoPayload::Binary(_) => return Err(refusal(ValueRefusalKind::InvalidValue, \"serializer_entry_text: expected a text native payload or a composed carrier\")),\n            };\n            ::semio_framework_async::poll::resolve_ready(T::serialize(&value, &children))\n",
        ),
    ],
    LAWS: [
        (
            "    #[semio_framework_async_macros::async_test]\n    async fn conformance_runs_after_deserialize() {",
            "    /// 🔁️ Echoes the parent value beside every owned child it received (`slot/childId`, archive order).\n    struct ChildrenEcho;\n    impl Serializer<semio_framework_value::DslValue> for ChildrenEcho {\n        const INTO: Dialect = C;\n        const FIDELITY: IoFidelity = IoFidelity::Exact;\n\n        async fn serialize(from: &semio_framework_value::DslValue, children: &ArchiveChildren) -> IoResult<IoPayload> {\n            let slots: Vec<String> = children.slots().into_iter().map(|(slot, child_id)| format!(\"{slot}/{child_id}\")).collect();\n            Ok(IoOutcome::clean(IoPayload::Text(format!(\"{from:?}|{}\", slots.join(\",\")))))\n        }\n    }\n\n    /// 🗃️ A carrier of `parent` owning one child at `content`/`child-1`, with `parent_spr` as its parent history.\n    fn composed_carrier(parent: &semio_framework_value::DslValue, parent_spr: Vec<u8>) -> Vec<u8> {\n        let artifact = |id: &str, dialect: Dialect| store::channel::DocumentArchiveArtifactRef { artifact_id: id.into(), artifact_kind: dialect.artifact_kind.into(), standard: dialect.standard.0.into(), subset: dialect.subset.0.into() };\n        let owner = store::channel::DocumentArchiveOwnerRef { parent: artifact(\"parent\", A), slot: \"content\".into(), child_id: \"child-1\".into() };\n        let member = store::channel::OwnedDocumentMemberPackEntry { ordinal: 0, reference: artifact(\"child-1\", B), owner, envelope_pack: vec![7] };\n        store::channel::encode_document_archive_bytes(&store::channel::DocumentArchivePack { parent_pack: store::ArtifactPack::encode_pack(parent), parent_spr, members: vec![member] }).expect(\"the carrier encodes\")\n    }\n\n    /// 🪆️ LAW (design §20.15, readers compose on read): a serializer entry run over a composed head carrier receives the parent and\n    /// its owned children; a plain pack reaches it with the empty view; a carrier with parent history or malformed bytes is refused.\n    #[semio_framework_async_macros::async_test]\n    async fn a_composed_head_carrier_hands_its_owned_children_to_the_serializer() {\n        let entry = serializer_entry::<semio_framework_value::DslValue, ChildrenEcho>(A);\n        let parent = semio_framework_value::DslValue::String(\"parent\".into());\n        let text = |outcome: IoResult<IoPayload>| match outcome.expect(\"the entry runs\").value {\n            IoPayload::Text(text) => text,\n            IoPayload::Binary(_) => panic!(\"the echo serializes text\"),\n        };\n        assert_eq!(text((entry.run)(&IoPayload::Binary(composed_carrier(&parent, Vec::new())))), format!(\"{parent:?}|content/child-1\"));\n        assert_eq!(text((entry.run)(&IoPayload::Binary(store::ArtifactPack::encode_pack(&parent)))), format!(\"{parent:?}|\"));\n        assert!((entry.run)(&IoPayload::Binary(composed_carrier(&parent, vec![1]))).is_err(), \"a carrier with parent history is not a head carrier\");\n        assert!((entry.run)(&IoPayload::Binary(vec![store::channel::DOCUMENT_ARCHIVE_VERSION, 0xff])).is_err(), \"a malformed carrier is refused\");\n    }\n\n    #[semio_framework_async_macros::async_test]\n    async fn conformance_runs_after_deserialize() {",
        ),
    ],
}
MARKERS = {CHANNEL: "pub const DOCUMENT_ARCHIVE_VERSION: u8 = 1;", IO: "fn native_carrier(bytes: &[u8])", LAWS: "fn a_composed_head_carrier_hands_its_owned_children_to_the_serializer"}


def main() -> None:
    apply = "--apply" in sys.argv
    staged = {}
    for path, edits in EDITS.items():
        with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
            text = handle.read()
        if MARKERS[path] in text:
            print(f"SKIP {path}: already carries the change")
            continue
        after = text
        for old, new in edits:
            count = after.count(old)
            if count != 1:
                sys.exit(f"ANCHOR {path}: {count} matches for {old[:80]!r}")
            after = after.replace(old, new)
        staged[path] = (text, after)
    for path, (before, after) in staged.items():
        if not apply:
            print(f"WOULD {path} (+{after.count(chr(10)) - before.count(chr(10))} lines)")
            continue
        with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
            if handle.read() != before:
                sys.exit(f"RACE {path}: changed while staging, nothing more written")
        with open(f"{ROOT}/{path}", "w", encoding="utf-8") as handle:
            handle.write(after)
        print(f"WROTE {path}")


if __name__ == "__main__":
    main()
