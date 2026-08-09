//! ⚡️ Mathematical artifact — OpText/OpBinary for `MathematicalMutation`.

pub use crate::artifacts::mathematical::mutations::{apply_mathematical_mutation, inverse_mathematical_mutation, MathematicalMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::mathematical::dsl::{math_graph_from_dsl, math_graph_to_dsl, mathematical_snapshot_from_dsl, mathematical_snapshot_to_dsl, MathematicalGraphDsl, MathematicalSnapshotDsl};
use crate::artifacts::mathematical::MathematicalGeometry;

//#region 🔖️OpText
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum MathematicalMutationDsl {
    SetGraph {
        #[dsl(block)]
        graph: MathematicalGraphDsl,
    },
    SetGeometry {
        #[dsl(block)]
        geometry: MathematicalGeometry,
    },
    SetSnapshot {
        #[dsl(block)]
        snapshot: MathematicalSnapshotDsl,
    },
}

impl protocol::OpText for MathematicalMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for MathematicalMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

fn mathematical_mutation_to_dsl(mutation: &MathematicalMutation) -> MathematicalMutationDsl {
    match mutation {
        MathematicalMutation::SetGraph { graph } => MathematicalMutationDsl::SetGraph { graph: math_graph_to_dsl(graph) },
        MathematicalMutation::SetGeometry { geometry } => MathematicalMutationDsl::SetGeometry { geometry: geometry.clone() },
        MathematicalMutation::SetSnapshot { snapshot } => MathematicalMutationDsl::SetSnapshot {
            snapshot: mathematical_snapshot_to_dsl(snapshot),
        },
    }
}

fn mathematical_mutation_from_dsl(mutation: MathematicalMutationDsl) -> Result<MathematicalMutation, String> {
    Ok(match mutation {
        MathematicalMutationDsl::SetGraph { graph } => MathematicalMutation::SetGraph { graph: math_graph_from_dsl(graph)? },
        MathematicalMutationDsl::SetGeometry { geometry } => MathematicalMutation::SetGeometry { geometry },
        MathematicalMutationDsl::SetSnapshot { snapshot } => MathematicalMutation::SetSnapshot {
            snapshot: mathematical_snapshot_from_dsl(snapshot)?,
        },
    })
}

impl protocol::OpText for MathematicalMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_mutation = <MathematicalMutationDsl as protocol::OpText>::parse_op(line)?;
        mathematical_mutation_from_dsl(dsl_mutation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <MathematicalMutationDsl as protocol::OpText>::print_op(&mathematical_mutation_to_dsl(self))
    }
}

impl protocol::OpBinary for MathematicalMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        mathematical_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let dsl_mutation = MathematicalMutationDsl::decode_op(bytes)?;
        mathematical_mutation_from_dsl(dsl_mutation).map_err(|message| protocol::ProtocolError::Malformed { what: "mathematical mutation", offset: 0, detail: message })
    }
}
//#endregion 🔖️OpText
