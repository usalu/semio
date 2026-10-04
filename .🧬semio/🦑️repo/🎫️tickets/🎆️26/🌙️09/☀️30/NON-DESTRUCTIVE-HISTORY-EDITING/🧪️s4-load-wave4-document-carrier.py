#!/usr/bin/env python3
"""🎞️ S4-LOAD wave 4 (coordinator GO 11:4x, branch b): ONE convention for a whole document on a media edge. `artifact:out`
(`produce_media`) answers the recursive document archive (`encode_document_archive_bytes(document_archive())`: parent pack + spr +
owned members) as `pk:` base64 text on the `Document{schema}` wire — the `Structured{schema, json: pk:…}` form every app's
`export_media` already uses, so `🏃️run`'s `Structured{json: String}` carries it losslessly — and `consume_media` decodes exactly
that form for its own schema into a stepped archive load (members included, the source document verbatim). A foreign schema is
refused with the localized framework code `plugin.media.schema-mismatch` (`{found}`, `{expected}`). Laws: guest round trip of a
composed document, refusal notice in both locales, run edge round trip.

Rule 39: one atomic write per file; every anchor must occur exactly once; an applied replacement is detected by its new text
(idempotent). Usage: `python3 🧪️s4-load-wave4-document-carrier.py [--check]`.
"""

import pathlib
import sys

OS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules")
KERNEL = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel")

NOTICE_EN = "This input is a {found} document, but only {expected} documents can be loaded here."
NOTICE_DE = "Diese Eingabe ist ein {found}-Dokument, hier lassen sich aber nur {expected}-Dokumente laden."

PLUGIN = [
    (
        """    impl MediaArtifact {
        /// 🎞️ Owns exact intrinsic bytes and descriptor text from one borrowed media owner.""",
        """    /// 🚫️ The framework code a whole document of a foreign schema is refused with (`{found}`, `{expected}`; notice in the framework
    /// table `FRAMEWORK_FAULT_NOTICE_LABELS`).
    pub const MEDIA_SCHEMA_MISMATCH_CODE: &str = "plugin.media.schema-mismatch";

    /// 🎞️ A whole document on a media edge (`artifact:out`): the recursive document archive (parent pack + `.spr` + owned members) as
    /// `pk:` base64 text on the `Document{schema}` wire — the `Structured{schema, json}` convention of `export_media`, which every host
    /// carries losslessly as text.
    pub async fn whole_document_media(port: &str, schema: String, archive: &protocol::DocumentArchivePack) -> Result<MediaArtifact, MediaArtifactError> {
        let bytes = protocol::encode_document_archive_bytes(archive).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
        Ok(MediaArtifact {
            descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port.to_string()), kind_id: None, media_type: None, wire: MediaWireFormat::Document { schema }, blob_hash: None },
            data: store::pack_rt::pack_value_to_base64(&bytes).into_bytes(),
        })
    }

    /// 🎞️ The archive a [`whole_document_media`] carrier holds.
    pub async fn whole_document_archive(data: &[u8]) -> Result<protocol::DocumentArchivePack, MediaArtifactError> {
        let text = std::str::from_utf8(data).map_err(|_| MediaArtifactError::Payload("a whole-document media carrier is pk: base64 text".into()))?;
        let bytes = store::pack_rt::pack_value_from_base64(text).map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
        protocol::decode_document_archive_bytes(&bytes).await.map_err(|error| MediaArtifactError::Payload(error.to_string()))
    }

    /// 🚫️ The fault a refused media artifact answers: a foreign document schema under [`MEDIA_SCHEMA_MISMATCH_CODE`] with both schemas
    /// as notice parameters, anything else as the runtime's internal fault.
    pub fn media_artifact_fault(error: MediaArtifactError) -> Fault {
        match error {
            MediaArtifactError::SchemaMismatch { expected, found } => Fault::new(FaultOrigin::Framework, FaultCode::new(MEDIA_SCHEMA_MISMATCH_CODE), format!("document schema mismatch: expected {expected}, found {found}")).with_param("expected", expected).with_param("found", found),
            other => Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.internal"), other.to_string()),
        }
    }

    impl MediaArtifact {
        /// 🎞️ Owns exact intrinsic bytes and descriptor text from one borrowed media owner.""",
    ),
    (
        """        /// 🎞️ ABI-level media artifact request for one port (`framework/wit/📜️world.wit`'s `produce-media`).
        /// Default: a whole-document passthrough (`wire: Document{schema: artifact_schema()}` wrapping
        /// `document_pack()`'s pack+spr bytes via `store::encode_document_pack_bytes`) — the fallback every
        /// `PluginApp` gets for free without declaring any `media_ports()`. Apps whose media output isn't
        /// simply their raw document (computed/derived outputs) override this directly; `port` is accepted
        /// for parity with `export_media` and ignored by the default (there is exactly one document to hand
        /// back).
        async fn produce_media(&mut self, port: &str) -> Result<MediaArtifact, MediaArtifactError> {
            let files = self.document_pack().await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
            Ok(MediaArtifact {
                descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port.to_string()), kind_id: None, media_type: None, wire: MediaWireFormat::Document { schema: self.artifact_schema().await.to_string() }, blob_hash: None },
                data: store::encode_document_pack_bytes(&files.pack, &files.spr).await,
            })
        }""",
        """        /// 🎞️ ABI-level media artifact request for one port (`framework/wit/📜️world.wit`'s `produce-media`).
        /// Default: the whole document ([`whole_document_media`] of `document_archive()`, members included) — the fallback every
        /// `PluginApp` gets for free without declaring any `media_ports()`. Apps whose media output isn't simply their raw document
        /// (computed/derived outputs) override this directly; `port` is accepted for parity with `export_media` and ignored by the
        /// default (there is exactly one document to hand back).
        async fn produce_media(&mut self, port: &str) -> Result<MediaArtifact, MediaArtifactError> {
            let archive = self.document_archive().await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
            whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await
        }""",
    ),
    (
        """        /// 🎞️ ABI-level media artifact delivery for one port (`framework/wit/📜️world.wit`'s `consume-media`).
        /// Default: a `Document{schema}` wire matching this app's own `artifact_schema()` is a whole document — the same pack+spr
        /// bytes `produce_media` hands out — answered as [`MediaConsumption::DocumentLoad`] with its stamped archive
        /// ([`Self::document_load_archive`]), which the runtime admits as a stepped archive load. Anything else (a foreign
        /// document schema, or a `Binary{format}` wire) has no SDK-level importer registry yet, so the default rejects it; apps
        /// that need one override this method directly.""",
        """        /// 🎞️ ABI-level media artifact delivery for one port (`framework/wit/📜️world.wit`'s `consume-media`).
        /// Default: a `Document{schema}` wire matching this app's own `artifact_schema()` is a whole document — the carrier
        /// `produce_media` hands out ([`whole_document_archive`]) — answered as [`MediaConsumption::DocumentLoad`] with its archive
        /// verbatim (the source document and its members), which the runtime admits as a stepped archive load. Anything else (a
        /// foreign document schema, or a `Binary{format}` wire) has no SDK-level importer registry yet, so the default rejects it;
        /// apps that need one override this method directly.""",
    ),
    (
        """
            match artifact.descriptor.wire {
                MediaWireFormat::Document { schema } if schema == schema_now => {
                    let (pack, spr) = store::decode_document_pack_bytes(&artifact.data).await.map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
                    self.document_load_archive(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).map(MediaConsumption::DocumentLoad).map_err(MediaArtifactError::Import)
                }""",
        """
            match artifact.descriptor.wire {
                MediaWireFormat::Document { schema } if schema == schema_now => whole_document_archive(&artifact.data).await.map(MediaConsumption::DocumentLoad),""",
    ),
    (
        """                    Err(MediaError::NotImplemented) => {}
                    Err(error) => return Err(MediaArtifactError::Payload(error.to_string())),
                }
            }
            let files = self.document_pack().await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
            Ok(MediaArtifact {
                descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port.to_string()), kind_id: None, media_type: None, wire: MediaWireFormat::Document { schema: self.artifact_schema().await.to_string() }, blob_hash: None },
                data: store::encode_document_pack_bytes(&files.pack, &files.spr).await,
            })
        }""",
        """                    Err(MediaError::NotImplemented) => {}
                    Err(error) => return Err(MediaArtifactError::Payload(error.to_string())),
                }
            }
            let archive = PluginApp::document_archive(self).await.map_err(|fault| MediaArtifactError::Payload(fault.message))?;
            whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await
        }""",
    ),
    (
        """                return match artifact.descriptor.wire {
                    MediaWireFormat::Document { schema } if schema == schema_now => {
                        let (pack, spr) = store::decode_document_pack_bytes(&artifact.data).await.map_err(|error| MediaArtifactError::Payload(error.to_string()))?;
                        self.document_load_archive(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).map(MediaConsumption::DocumentLoad).map_err(MediaArtifactError::Import)
                    }""",
        """                return match artifact.descriptor.wire {
                    MediaWireFormat::Document { schema } if schema == schema_now => whole_document_archive(&artifact.data).await.map(MediaConsumption::DocumentLoad),""",
    ),
    (
        """            match ::semio_framework_async::poll::resolve_ready(instance.app.consume_media(port_id, artifact)).map_err(|error| plugin_internal_fault(error.to_string()))? {""",
        """            match ::semio_framework_async::poll::resolve_ready(instance.app.consume_media(port_id, artifact)).map_err(crate::app::media_artifact_fault)? {""",
    ),
]

KERNEL_RS = [
    (
        "pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); 12] = [\n",
        "pub const FRAMEWORK_FAULT_NOTICE_LABELS: [(&str, &str, &str); 13] = [\n",
    ),
    (
        """    ("mutation.too-large", "This change is too large to record at once — split it into smaller steps.", "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen."),
""",
        f"""    ("mutation.too-large", "This change is too large to record at once — split it into smaller steps.", "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen."),
    ("plugin.media.schema-mismatch", "{NOTICE_EN}", "{NOTICE_DE}"),
""",
    ),
]

KERNEL_TS = [
    (
        """  { code: "mutation.too-large", en: "This change is too large to record at once — split it into smaller steps.", de: "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." },
""",
        f"""  {{ code: "mutation.too-large", en: "This change is too large to record at once — split it into smaller steps.", de: "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." }},
  {{ code: "plugin.media.schema-mismatch", en: "{NOTICE_EN}", de: "{NOTICE_DE}" }},
""",
    ),
]

KERNEL_FIXTURE = [
    (
        """    { "code": "mutation.too-large", "en": "This change is too large to record at once — split it into smaller steps.", "de": "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." },
""",
        f"""    {{ "code": "mutation.too-large", "en": "This change is too large to record at once — split it into smaller steps.", "de": "Diese Änderung ist zu groß, um sie auf einmal aufzuzeichnen — in kleinere Schritte aufteilen." }},
    {{ "code": "plugin.media.schema-mismatch", "en": "{NOTICE_EN}", "de": "{NOTICE_DE}" }},
""",
    ),
]

KERNEL_LAW = [
    (
        """#[test]
fn a_channel_mismatch_names_both_versions_in_both_locales() {""",
        """/// 🎞️ LAW (S4-LOAD): a whole document of a foreign schema on a media edge is refused under `plugin.media.schema-mismatch` and its
/// notice names both schemas in both locales; without them it never shows an unfilled placeholder.
#[test]
fn a_foreign_document_schema_on_a_media_edge_names_both_schemas_in_both_locales() {
    let fault = Fault::new(FaultOrigin::Framework, "plugin.media.schema-mismatch".to_string(), "document schema mismatch").with_param("found", "s.draw.drawing").with_param("expected", "s.note.note");
    let (en, de) = framework_fault_notice("plugin.media.schema-mismatch").expect("the framework labels plugin.media.schema-mismatch");
    for (locale, template) in [(semio_framework_ui_locale::Locale::En, en), (semio_framework_ui_locale::Locale::De, de)] {
        let notice = fault_notice(&fault, &[], semio_framework_ui_locale::Terminology::Native, locale).expect("a schema mismatch always earns its notice");
        assert_eq!(notice, FaultNotice { code: "plugin.media.schema-mismatch".to_string(), text: template.replace("{found}", "s.draw.drawing").replace("{expected}", "s.note.note") });
    }
    let unnamed = Fault::new(FaultOrigin::Framework, "plugin.media.schema-mismatch".to_string(), "no schemas named");
    assert_eq!(fault_notice(&unnamed, &[], semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), None);
}

#[test]
fn a_channel_mismatch_names_both_versions_in_both_locales() {""",
    ),
]

FILES = [
    (OS / "🔌️plugin/🦀️.rs", PLUGIN),
    (KERNEL / "🦀️.rs", KERNEL_RS),
    (KERNEL / "🟦️.ts", KERNEL_TS),
    (KERNEL / "🧫️fixtures/🧫️framework-notices/🔣️.json", KERNEL_FIXTURE),
    (KERNEL / "🧪️tests/🧪️framework-notices/🦀️.rs", KERNEL_LAW),
]


def apply(path, replacements, check):
    text = path.read_text(encoding="utf-8")
    applied = [new for old, new in replacements if new in text]
    pending = [(old, new) for old, new in replacements if new not in text and old in text]
    missing = [old for old, new in replacements if new not in text and old not in text]
    if missing:
        raise SystemExit(f"{path.name}: anchor missing and not applied: {missing[0][:140]!r}")
    for old, _ in pending:
        if text.count(old) != 1:
            raise SystemExit(f"{path.name}: anchor occurs {text.count(old)}x: {old[:140]!r}")
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
        print(f"{path.parent.name}/{path.name}: {pending} {'pending' if check else 'applied'}, {applied} already applied")
    return 1 if check and total else 0


if __name__ == "__main__":
    sys.exit(main())
