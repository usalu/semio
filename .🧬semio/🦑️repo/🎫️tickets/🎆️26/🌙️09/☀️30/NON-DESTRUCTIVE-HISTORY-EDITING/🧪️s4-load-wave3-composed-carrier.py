#!/usr/bin/env python3
"""🪆️ S4-LOAD W-b (c), one atomic write per file (fleet rule 39): the framework's `artifact:out` export of a COMPOSED document emits
the composed carrier `encode_document_archive_bytes({parent HEAD pack, empty spr, members})` (design §20.15, S3-AGNOSTIC format; first
byte 0x01 = archive version vs. pack magic), base64 in a `Structured` payload of the document schema — the head pack alone would lose
the child-lane content. The archive's member roster becomes one helper shared with `document_archive`. Plus its law in the builder
contract test.

Every anchor must occur exactly once; an already applied file is left untouched (idempotent).
Usage: `python3 🧪️s4-load-wave3-composed-carrier.py [--check]`.
"""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")

MEMBERS_OLD = """            let parent = store::print_document_pack(self.store.envelope()).await.map_err(|error| error.into_fault())?;
            let mut members = Vec::new();
            members.try_reserve_exact(self.children.len()).map_err(|_| plugin_sdk_fault("document archive member roster could not reserve its exact live extent"))?;
            for entry in self.children.entries() {"""

PLUGIN = [
    (
        """                })
                .collect()
        }
    }

    /// 🆔️ Deterministic session-local `ArtifactHandle` for a CHILD's real (string) artifact id.""",
        """                })
                .collect()
        }

        /// 🪆️ Every live owned child as its archive member (persisted envelope, identity, owner), sorted by owner and identity
        /// with dense ordinals — the member roster both the document archive and the composed `artifact:out` carrier hold.
        async fn archive_member_entries(&self) -> Result<Vec<protocol::OwnedDocumentMemberPackEntry>, Fault> {
            let mut members = Vec::new();
            members.try_reserve_exact(self.children.len()).map_err(|_| plugin_sdk_fault("document archive member roster could not reserve its exact live extent"))?;
            for entry in self.children.entries() {
                let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
                members.push(protocol::OwnedDocumentMemberPackEntry {
                    ordinal: 0,
                    reference: protocol::DocumentArchiveArtifactRef {
                        artifact_id: entry.reference.artifact_id.clone(),
                        artifact_kind: entry.reference.dialect.artifact_kind.clone(),
                        standard: entry.reference.dialect.standard.clone(),
                        subset: entry.reference.dialect.subset.clone(),
                    },
                    owner: protocol::DocumentArchiveOwnerRef {
                        parent: protocol::DocumentArchiveArtifactRef {
                            artifact_id: entry.owner.parent.artifact_id.clone(),
                            artifact_kind: entry.owner.parent.dialect.artifact_kind.clone(),
                            standard: entry.owner.parent.dialect.standard.clone(),
                            subset: entry.owner.parent.dialect.subset.clone(),
                        },
                        slot: entry.owner.slot.clone(),
                        child_id: entry.owner.child_id.clone(),
                    },
                    envelope_pack,
                });
            }
            members.sort_by(|left, right| {
                (&left.owner.parent.artifact_id, &left.owner.slot, &left.owner.child_id, &left.reference.artifact_kind, &left.reference.standard, &left.reference.subset).cmp(&(
                    &right.owner.parent.artifact_id,
                    &right.owner.slot,
                    &right.owner.child_id,
                    &right.reference.artifact_kind,
                    &right.reference.standard,
                    &right.reference.subset,
                ))
            });
            for (ordinal, entry) in members.iter_mut().enumerate() {
                entry.ordinal = u32::try_from(ordinal).map_err(|_| plugin_sdk_fault("document archive member ordinal exceeds u32"))?;
            }
            Ok(members)
        }

        /// 🪆️ The native `artifact:out` carrier of a composed document (design §20.15): `encode_document_archive_bytes` of the parent's
        /// HEAD snapshot pack, an empty `.spr` (the head marker) and every owned member, base64 in a `Structured` payload of the document
        /// schema — readers compose parent and children on read; the parent's head pack alone would drop the child-lane content.
        async fn composed_artifact_media(&mut self) -> Result<Media, Fault> {
            self.refresh_cache().await?;
            let parent_generation = self.store.generation_now();
            let child_generation = self.child_content_generation;
            let parent_pack = self.cache.as_ref().ok_or_else(|| plugin_sdk_fault("render cache unavailable after refresh"))?.1.encode_pack();
            let members = self.archive_member_entries().await?;
            if self.store.generation_now() != parent_generation || self.child_content_generation != child_generation {
                return Err(plugin_sdk_fault("composed artifact export authority changed during generation-fenced export"));
            }
            let bytes = protocol::encode_document_archive_bytes(&protocol::DocumentArchivePack { parent_pack, parent_spr: Vec::new(), members }).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            let media_type = A::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
            Ok(Media { media_type, payload: MediaPayload::Structured { schema: A::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
        }
    }

    /// 🆔️ Deterministic session-local `ArtifactHandle` for a CHILD's real (string) artifact id.""",
    ),
    (
        MEMBERS_OLD
        + """
                let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
                members.push(protocol::OwnedDocumentMemberPackEntry {
                    ordinal: 0,
                    reference: protocol::DocumentArchiveArtifactRef {
                        artifact_id: entry.reference.artifact_id.clone(),
                        artifact_kind: entry.reference.dialect.artifact_kind.clone(),
                        standard: entry.reference.dialect.standard.clone(),
                        subset: entry.reference.dialect.subset.clone(),
                    },
                    owner: protocol::DocumentArchiveOwnerRef {
                        parent: protocol::DocumentArchiveArtifactRef {
                            artifact_id: entry.owner.parent.artifact_id.clone(),
                            artifact_kind: entry.owner.parent.dialect.artifact_kind.clone(),
                            standard: entry.owner.parent.dialect.standard.clone(),
                            subset: entry.owner.parent.dialect.subset.clone(),
                        },
                        slot: entry.owner.slot.clone(),
                        child_id: entry.owner.child_id.clone(),
                    },
                    envelope_pack,
                });
            }
            members.sort_by(|left, right| {
                (&left.owner.parent.artifact_id, &left.owner.slot, &left.owner.child_id, &left.reference.artifact_kind, &left.reference.standard, &left.reference.subset).cmp(&(
                    &right.owner.parent.artifact_id,
                    &right.owner.slot,
                    &right.owner.child_id,
                    &right.reference.artifact_kind,
                    &right.reference.standard,
                    &right.reference.subset,
                ))
            });
            for (ordinal, entry) in members.iter_mut().enumerate() {
                entry.ordinal = u32::try_from(ordinal).map_err(|_| plugin_sdk_fault("document archive member ordinal exceeds u32"))?;
            }
            if self.store.generation_now() != parent_generation""",
        """            let parent = store::print_document_pack(self.store.envelope()).await.map_err(|error| error.into_fault())?;
            let members = self.archive_member_entries().await?;
            if self.store.generation_now() != parent_generation""",
    ),
    (
        """            self.refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
            let render_operation = self.live_render_operation();
            let parent_document_id = self.store.envelope().id.clone();
            let VcsArtifactApp { app: _, cache, child_content_root, transient_store, instance_operation_owner, .. } = self;""",
        """            if port == "artifact:out" && !self.children.is_empty() {
                return self.composed_artifact_media().await.map_err(|fault| MediaError::Payload(port.to_string(), fault.message));
            }
            self.refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
            let render_operation = self.live_render_operation();
            let parent_document_id = self.store.envelope().id.clone();
            let VcsArtifactApp { app: _, cache, child_content_root, transient_store, instance_operation_owner, .. } = self;""",
    ),
]

CONTRACT = [
    (
        """        assert_eq!(crate::inference_child_dependency(&heads[0].slot, &heads[0].child_id), "child:slot/child-1");
        drain_and_close_composed_fixture(&mut app);
    }""",
        """        assert_eq!(crate::inference_child_dependency(&heads[0].slot, &heads[0].child_id), "child:slot/child-1");
        drain_and_close_composed_fixture(&mut app);
    }

    /// 🪆️ LAW (design §20.15, W-b): the framework's `artifact:out` export of a composed document is the composed carrier — a document
    /// archive whose parent is the HEAD snapshot pack with an empty `.spr` and whose members are the owned children's envelopes — while a
    /// document without children keeps the plain head pack.
    #[semio_framework_async_macros::async_test]
    async fn the_artifact_out_export_of_a_composed_document_is_the_composed_carrier() {
        let mut app = contract_composed_app_raw().await;
        let plain = match PluginApp::export_media(&mut app, "artifact:out").await.expect("plain export").payload {
            crate::MediaPayload::Structured { json, .. } => store::pack_rt::pack_value_from_base64(&json).expect("base64 head pack"),
            other => panic!("artifact:out answers a structured payload, got {other:?}"),
        };
        assert_eq!(<TestSnapshot as ArtifactPack>::decode_pack(&plain).expect("a document without children exports its head pack"), app.test_snapshot().await);

        app.register_child("slot", "child-1", test_child_dialect().await, new_test_child("child-1").await.expect("construct child")).await.expect("register child");
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta()).await.expect("composite edit");
        artifact_app_laws::settle_registered_typed_operation(&mut app, meta().instance_id).await.expect("the composite gesture settles");
        let media = PluginApp::export_media(&mut app, "artifact:out").await.expect("composed export");
        let crate::MediaPayload::Structured { schema, json } = media.payload else { panic!("artifact:out answers a structured payload") };
        assert_eq!(schema, <TestApp as ArtifactApp>::DOCUMENT_SCHEMA);
        let carrier = store::pack_rt::pack_value_from_base64(&json).expect("base64 carrier");
        let archive = protocol::decode_document_archive_bytes(&carrier).await.expect("the carrier is a document archive");
        assert!(archive.parent_spr.is_empty(), "the head carrier holds no parent history");
        assert_eq!(<TestSnapshot as ArtifactPack>::decode_pack(&archive.parent_pack).expect("parent head pack"), app.test_snapshot().await);
        assert_eq!(archive.members.iter().map(|entry| (entry.ordinal, entry.owner.slot.as_str(), entry.owner.child_id.as_str())).collect::<Vec<_>>(), vec![(0, "slot", "child-1")]);
        assert!(!archive.members[0].envelope_pack.is_empty(), "each member carries its persisted envelope");
        drain_and_close_composed_fixture(&mut app);
    }""",
    ),
]

FILES = [(ROOT / "🦀️.rs", PLUGIN), (ROOT / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", CONTRACT)]


def apply(path, replacements, check):
    text = path.read_text(encoding="utf-8")
    applied = [new for old, new in replacements if new in text]
    pending = [(old, new) for old, new in replacements if new not in text and old in text]
    missing = [old for old, new in replacements if new not in text and old not in text]
    if missing:
        raise SystemExit(f"{path.name}: anchor missing and not applied: {missing[0][:120]!r}")
    for old, _ in pending:
        if text.count(old) != 1:
            raise SystemExit(f"{path.name}: anchor occurs {text.count(old)}x: {old[:120]!r}")
    if not check:
        for old, new in pending:
            text = text.replace(old, new)
        if pending:
            path.write_text(text, encoding="utf-8")
    return len(pending), len(applied)


def main():
    check = "--check" in sys.argv
    total = 0
    for path, replacements in FILES:
        pending, applied = apply(path, replacements, check)
        total += pending
        print(f"{path.relative_to(ROOT)}: {pending} {'pending' if check else 'applied'}, {applied} already applied")
    return 1 if check and total else 0


if __name__ == "__main__":
    sys.exit(main())
