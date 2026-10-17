//! 🚪️ IO stdio.jpg (jfif-1.01/🧾️document) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
pub use binary::snapshot::encode_options::{JpgEncodeOptions,JpgEncodeComponent};
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_jfif_1_01::subsets::document::io::JpgAnalyzer;
    use crate::JpgSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct JpgComposerComposition;

    impl ArtifactComposition for JpgComposerComposition {
        type Snapshot = JpgSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "JpgComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = JpgAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "JpgComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the
// real baseline-sequential (SOF0) JPEG codec — Huffman entropy decoding, dequantization,
// integer-ish separable IDCT, YCbCr→RGB with nearest-neighbor chroma upsampling — relocated here
// verbatim (destination rule 2: codecs → `🚪️io/`; rule 6: pure format algorithms with no
// snapshot dependency stay WITH the codec here rather than promoting to a module engine, since
// they're JPEG-specific, not artifact-independent). Progressive/arithmetic/lossless SOFn variants
// are explicit `JpgError::Unsupported`, never decoded as garbage. `JpgEngine` (zero construction
// sites) and the dead `register`/`register_pilot_languages`/`register_artifact_inferences`/
// `register_schema_specs` cluster (superseded by `declaration()` in the artifact root, zero real
// callers) were deleted outright, not relocated. `blank_jpg_snapshot`/`demo_jpg_snapshot` moved to
// `../🧬️schema` (pure helpers over the document type).
use crate::schema::snapshot::{JfifDensityUnits,JfifThumbnail,JpgSegment};
use binary::snapshot::observations::{JpgFrameComponent,JpgFrameHeader,JpgHuffmanClass,JpgHuffmanTable,JpgQuantTable,JpgScanComponent,JpgNativeObservations,JpgDecodedDocument};
use crate::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};
use std::collections::HashMap;

#[path = "🧮️transforms-and-tables/🦀️.rs"]
mod transforms_and_tables;
pub use transforms_and_tables::*;

#[path = "🔡️huffman-and-bit-io/🦀️.rs"]
mod huffman_and_bit_io;
use huffman_and_bit_io::*;

#[path = "✍️encoder/🦀️.rs"]
mod encoder;
pub use encoder::*;

#[path = "🔎️header-and-decode-entry/🦀️.rs"]
mod header_and_decode_entry;
pub use header_and_decode_entry::*;

#[path = "🪜️step-decoder/🦀️.rs"]
mod step_decoder;
pub use step_decoder::*;

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_jfif_1_01::subsets::baseline::io::JpgBaselineComposer;
    use crate::standards::v_jfif_1_01::subsets::document::io::JpgComposer as JpgRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<JpgRawAnyComposer>(), composer_entry_of::<JpgBaselineComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{JpgDiff, JpgMutation, JpgSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.jpg` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct JpgBuilderConstruction {
        snapshot: JpgSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for JpgBuilderConstruction {
        type Snapshot = JpgSnapshot;
        type Mutation = JpgMutation;
        type Diff = JpgDiff;
        fn empty() -> Self {
            Self { snapshot: JpgSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<JpgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JpgSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::JpgSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.jpg` parts.
    #[derive(Clone, Debug, Default)]
    pub struct JpgParts {
        pub snapshot: Option<JpgSnapshot>,
        pub observations: Option<super::JpgNativeObservations>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.jpg` (jfif-1.01/🧾️document) sources.
    pub struct JpgAnalyzerAnalysis;

    impl ArtifactAnalysis for JpgAnalyzerAnalysis {
        type Parts = JpgParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            const SIG: [u8; 2] = [0xFF, 0xD8]; // SOI
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.jpg's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 2 bytes to sniff the real SOI marker.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(4).collect();
                    if hex.len() < 4 {
                        return semio_framework_plugin::io::Confidence::Low;
                    }
                    let mut decoded = [0u8; 2];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return semio_framework_plugin::io::Confidence::Low,
                        }
                    }
                    if decoded == SIG {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = JpgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JpgSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        if bytes.starts_with(&[0xff,0xd8]) {
                            match super::inspect_jpg_native_header(&bytes.as_ref()) {
                                Ok(observations)=>parts.observations=Some(observations),
                                Err(error)=>diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.native-header",semio_framework_diagnostic::TextSpan::at(1,1),error.to_string())),
                            }
                        }
                        match <JpgSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec JpgBuilderFacets {
        construction: JpgBuilderConstruction,
        analysis: JpgAnalyzerAnalysis,
        composition: JpgComposerComposition,
    }
    builder: JpgBuilder,
    analyzer: JpgAnalyzer,
    composer: JpgComposer,
);

/// 🆕️ Constructs the canonical decoded white image at native admission.
pub fn blank_jpg_snapshot() -> JpgSnapshot {
    use crate::standards::v_jfif_1_01::subsets::document::io::{decode_jpg, encode_jpg};
    let seed = JpgSnapshot { schema: STDIO_JPG_DOCUMENT_SCHEMA.into(), image: crate::schema::snapshot::JpgImage { width: 1,height: 1,pixels: vec![255, 255, 255, 255],..crate::schema::snapshot::JpgImage::default() } };
    encode_jpg(&seed, &JpgEncodeOptions::default()).and_then(|bytes| decode_jpg(&bytes)).expect("blank_jpg_snapshot: the 1×1 seed round-trips through the real codec")
}

/// 🧾️ Inspects exact native header observations even when raster decoding refuses that profile.
pub fn inspect_jpg_native_header(data:&dyn JpgByteSource)->Result<JpgNativeObservations,JpgError>{
 let header=parse_jpg_header(data,true)?;
 Ok(JpgNativeObservations{frame:header.frame,sof_marker:header.sof_marker,arithmetic:header.arithmetic,arithmetic_conditioning:header.arithmetic_conditioning,scan_components:header.scan_components,scan_parameters:header.scan_parameters,quant_tables:header.quant_tables,huffman_tables:header.huffman_tables,restart_interval:header.restart_interval})
}

/// 🚦️ Inspects native facts within an input budget and checkpoints every 256 byte visits.
pub fn inspect_jpg_native_header_controlled(data:&dyn JpgByteSource,maximum_input_bytes:usize,on_progress:&mut dyn FnMut(usize)->bool)->Result<JpgNativeObservations,semio_framework_value::ValueError>{
 use std::cell::{Cell,RefCell};
 use semio_framework_value::{ValueError,ValueRefusalKind};
 if data.len()>maximum_input_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"JPEG native input exceeds its admission budget"));}
 struct Source<'a>{data:&'a dyn JpgByteSource,on_progress:RefCell<&'a mut dyn FnMut(usize)->bool>,visited:Cell<usize>,stopped:Cell<bool>}
 impl JpgByteSource for Source<'_>{
  fn len(&self)->usize{self.data.len()}
  fn byte(&self,index:usize)->Option<u8>{
   let visited=self.visited.get();
   if self.stopped.get()||(visited%256==0&&!(self.on_progress.borrow_mut())(visited)){self.stopped.set(true);return None;}
   self.visited.set(visited.saturating_add(1));self.data.byte(index)
  }
 }
 let source=Source{data,on_progress:RefCell::new(on_progress),visited:Cell::new(0),stopped:Cell::new(false)};
 let result=inspect_jpg_native_header(&source);
 if source.stopped.get(){return Err(ValueError::new(ValueRefusalKind::Canceled,"JPEG native header inspection canceled"));}
 if !(source.on_progress.borrow_mut())(source.visited.get()){return Err(ValueError::new(ValueRefusalKind::Canceled,"JPEG native header inspection canceled"));}
 result.map_err(Into::into)
}
