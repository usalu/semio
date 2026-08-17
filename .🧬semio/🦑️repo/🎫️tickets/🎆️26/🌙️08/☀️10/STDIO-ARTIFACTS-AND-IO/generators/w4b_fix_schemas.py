#!/usr/bin/env python3
"""Fix w4b artifact schema roots and pdf snapshot."""
from pathlib import Path
import json

ROSTER = json.loads(Path(".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/STDIO-ARTIFACTS-AND-IO/🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
ZIP_ART = (PLUGIN / ROSTER["zip"]["dir"] / "🧬️schema/🦀️component.rs").read_text()

RASTER = """//! 🧬️ {Name}Artifact schema — full artifact state.

use crate::artifacts::{mid}::schema::snapshot::RasterImage;
use crate::artifacts::{mid}::{Name}Snapshot;
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.{mid}")]
pub struct {Name}Artifact {{
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub image: RasterImage,
}}

impl Default for {Name}Artifact {{
    fn default() -> Self {{ Self::from_snapshot({Name}Snapshot::default()) }}
}}

impl {Name}Artifact {{
    pub fn to_snapshot(&self) -> {Name}Snapshot {{
        {Name}Snapshot {{ schema: self.schema.clone(), image: self.image.clone() }}
    }}
    pub fn from_snapshot(snapshot: {Name}Snapshot) -> Self {{
        Self {{ schema: snapshot.schema, image: snapshot.image }}
    }}
    pub fn set_snapshot(&mut self, snapshot: {Name}Snapshot) {{
        self.schema = snapshot.schema;
        self.image = snapshot.image;
    }}
}}

pub fn {mid}_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "s.stdio.{mid}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        }},
    }}
}}
"""

PDF_SNAP = """//! 🧬️ PdfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDoc {
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub page: PageDoc,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
            page: PageDoc { width: 612.0, height: 792.0, text: String::new() },
        }
    }
}

impl store::DocumentDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str { "stdio.pdf" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
        }
        crate::artifacts::pdf::engine::decode_pdf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::engine::encode_pdf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::engine::encode_pdf(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pdf::engine::decode_pdf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
"""

PDF_ART = """//! 🧬️ PdfArtifact schema — full artifact state.

use crate::artifacts::pdf::schema::snapshot::PageDoc;
use crate::artifacts::pdf::PdfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub page: PageDoc,
}

impl Default for PdfArtifact {
    fn default() -> Self { Self::from_snapshot(PdfSnapshot::default()) }
}

impl PdfArtifact {
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot { schema: self.schema.clone(), page: self.page.clone() }
    }
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self { schema: snapshot.schema, page: snapshot.page }
    }
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.page = snapshot.page;
    }
}

pub fn pdf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pdf",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
"""

GLB_ART = RASTER.replace("image: RasterImage", "payload: GlbPayload").replace("RasterImage", "GlbPayload").replace("pub image", "pub payload")

for mid, name in [("png", "Png"), ("jpg", "Jpg"), ("gif", "Gif"), ("tiff", "Tiff")]:
    (PLUGIN / ROSTER[mid]["dir"] / "🧬️schema/🦀️component.rs").write_text(
        RASTER.format(mid=mid, Name=name), encoding="utf-8"
    )

(PLUGIN / ROSTER["pdf"]["dir"] / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(PDF_SNAP, encoding="utf-8")
(PLUGIN / ROSTER["pdf"]["dir"] / "🧬️schema/🦀️component.rs").write_text(PDF_ART, encoding="utf-8")

# glb
glb_art = """//! 🧬️ GlbArtifact schema — full artifact state.

use crate::artifacts::glb::schema::snapshot::GlbPayload;
use crate::artifacts::glb::GlbSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.glb")]
pub struct GlbArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub payload: GlbPayload,
}

impl Default for GlbArtifact {
    fn default() -> Self { Self::from_snapshot(GlbSnapshot::default()) }
}

impl GlbArtifact {
    pub fn to_snapshot(&self) -> GlbSnapshot {
        GlbSnapshot { schema: self.schema.clone(), payload: self.payload.clone() }
    }
    pub fn from_snapshot(snapshot: GlbSnapshot) -> Self {
        Self { schema: snapshot.schema, payload: snapshot.payload }
    }
    pub fn set_snapshot(&mut self, snapshot: GlbSnapshot) {
        self.schema = snapshot.schema;
        self.payload = snapshot.payload;
    }
}

pub fn glb_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.glb",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
"""
(PLUGIN / ROSTER["glb"]["dir"] / "🧬️schema/🦀️component.rs").write_text(glb_art, encoding="utf-8")

# office zip serializers
for mid in ("docx", "pptx", "xlsx", "bcf"):
    p = PLUGIN / ROSTER[mid]["dir"] / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"
    if p.exists():
        t = p.read_text()
        t = t.replace(f"encode_{mid}(from, true)", f"encode_{mid}(from)")
        p.write_text(t)

print("fixed")
