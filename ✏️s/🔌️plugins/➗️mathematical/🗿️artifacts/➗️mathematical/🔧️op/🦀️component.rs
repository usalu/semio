//! ⚡️ Mathematical artifact — OpText/OpBinary for `MathMutation`.

pub use crate::artifacts::mathematical::mutations::{apply_math_mutation, inverse_math_mutation, MathMutation};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::mathematical::dsl::{math_graph_from_dsl, math_graph_to_dsl, MathGraphDsl};
use crate::artifacts::mathematical::MathGeometry;

//#region 🔖️OpText
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum MathMutationDsl {
    SetGraph {
        #[dsl(block)]
        graph: MathGraphDsl,
    },
    SetGeometry {
        #[dsl(block)]
        geometry: MathGeometry,
    },
}

impl protocol::OpText for MathMutationDsl {
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

impl protocol::OpBinary for MathMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

fn math_mutation_to_dsl(mutation: &MathMutation) -> MathMutationDsl {
    match mutation {
        MathMutation::SetGraph { graph } => MathMutationDsl::SetGraph { graph: math_graph_to_dsl(graph) },
        MathMutation::SetGeometry { geometry } => MathMutationDsl::SetGeometry { geometry: geometry.clone() },
    }
}

fn math_mutation_from_dsl(mutation: MathMutationDsl) -> Result<MathMutation, String> {
    Ok(match mutation {
        MathMutationDsl::SetGraph { graph } => MathMutation::SetGraph { graph: math_graph_from_dsl(graph)? },
        MathMutationDsl::SetGeometry { geometry } => MathMutation::SetGeometry { geometry },
    })
}

impl protocol::OpText for MathMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_mutation = <MathMutationDsl as protocol::OpText>::parse_op(line)?;
        math_mutation_from_dsl(dsl_mutation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <MathMutationDsl as protocol::OpText>::print_op(&math_mutation_to_dsl(self))
    }
}

impl protocol::OpBinary for MathMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        math_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let dsl_mutation = MathMutationDsl::decode_op(bytes)?;
        math_mutation_from_dsl(dsl_mutation).map_err(|message| protocol::ProtocolError::Malformed { what: "math mutation", offset: 0, detail: message })
    }
}
//#endregion 🔖️OpText
