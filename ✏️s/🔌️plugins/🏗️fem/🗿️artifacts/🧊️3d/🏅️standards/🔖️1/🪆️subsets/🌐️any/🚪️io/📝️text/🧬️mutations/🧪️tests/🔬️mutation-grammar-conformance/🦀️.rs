use super::*;
use protocol::OpText;
use semio_framework_dsl_record::DslVariants;
use semio_framework_value::FromValue;

#[test]
fn component_grammar_semio_matches_the_exact_declared_mutation_language(){
    let grammar=semio_framework_dsl::parse_grammar(MUTATION_GRAMMAR_SEMIO).unwrap();assert_eq!(grammar.dialect,semio_framework_dsl::SemioDialect::Grammar);let recognizer=mutation_grammar_recognizer().unwrap();
    let cases=&[
        ("create-node",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/🦠️mutation/🔣️.json")),
        ("delete-node",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🕳️delete-node/🚫️removes/🦠️mutation/🔣️.json")),
        ("create-element",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/🦠️mutation/🔣️.json")),
        ("delete-element",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes/🦠️mutation/🔣️.json")),
        ("replace-element",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/🦠️mutation/🔣️.json")),
        ("create-material",include_str!("../../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-material/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-material",include_str!("../../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-material/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-material",include_str!("../../../../../../🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-material/🏗️hall/🦠️mutation/🔣️.json")),
        ("change-material",include_str!("../../../../../../🧱️material/🧫️fixtures/🧬️mutations/🩹️change-material/📉️softens/🦠️mutation/🔣️.json")),
        ("create-section",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/📐️create-section/🔳️appends/🦠️mutation/🔣️.json")),
        ("delete-section",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-section/🚫️removes/🦠️mutation/🔣️.json")),
        ("replace-section",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/📏️replace-section/🏗️hall/🦠️mutation/🔣️.json")),
        ("change-section",include_str!("../../../../../../🕸️mesh/🧫️fixtures/🧬️mutations/🩹️change-section/🩹️changes/🦠️mutation/🔣️.json")),
        ("create-support",include_str!("../../../../../../🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🛡️create-support/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-support",include_str!("../../../../../../🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-support/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-support",include_str!("../../../../../../🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-support/🏗️hall/🦠️mutation/🔣️.json")),
        ("create-solid",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-solid/🏠️appends/🦠️mutation/🔣️.json")),
        ("delete-solid",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/🦠️mutation/🔣️.json")),
        ("replace-solid",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🔄️replace-solid/📚️thickens/🦠️mutation/🔣️.json")),
        ("create-load-case",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/📋️create-load-case/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-load-case",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-load-case/🏗️hall/🦠️mutation/🔣️.json")),
        ("add-load",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/➕️add-load/🏠️lays/🦠️mutation/🔣️.json")),
        ("remove-load",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-load/➖️drops/🦠️mutation/🔣️.json")),
        ("change-load-case-self-weight",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall/🦠️mutation/🔣️.json")),
        ("create-combination",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🔗️create-combination/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-combination",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-combination/✂️removes-the-182f7b/🦠️mutation/🔣️.json")),
        ("update-analysis-settings",include_str!("../../../../../../📈️analysis/🧪️tests/🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-node",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-node/📍️lifts/🦠️mutation/🔣️.json")),
        ("change-node",include_str!("../../../../../../🕸️mesh/🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/🦠️mutation/🔣️.json")),
        ("change-combination-term",include_str!("../../../../../../🏋️load/🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚖️weights/🦠️mutation/🔣️.json")),
        ("change-combination-name",include_str!("../../../../../../🏋️load/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⚗️empty/🦠️mutation/🔣️.json")),
        ("change-load",include_str!("../../../../../../🏋️load/🧫️fixtures/🧬️mutations/🩹️change-load/📍️target/🦠️mutation/🔣️.json")),
        ("change-element",include_str!("../../../../../../🕸️mesh/🧫️fixtures/🧬️mutations/🩹️change-element/🔀️variant/🦠️mutation/🔣️.json")),
        ("change-support",include_str!("../../../../../../🛡️boundary/🧫️fixtures/🧬️mutations/🩹️change-support/📌️toggles/🦠️mutation/🔣️.json")),
        ("change-solid",include_str!("../../../../../../🕸️mesh/🧫️fixtures/🧬️mutations/🩹️change-solid/🩹️changes/🦠️mutation/🔣️.json")),
        ("replace-load",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-load/🔁️retunes/🦠️mutation/🔣️.json")),
        ("change-load-case-name",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-load-case-name/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-combination",include_str!("../../../../../../🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-combination/🏗️hall/🦠️mutation/🔣️.json")),
        ("move-selection",include_str!("../../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/🦠️mutation/🔣️.json")),
    ];
    let variants=Fem3dMutation::variants();assert_eq!(variants.len(),cases.len());assert_eq!(grammar.productions.len(),cases.len()+1);
    for(kind,json)in cases{
        let mutation=semio_framework_pack_json::from_json_str::<Fem3dMutation>(json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let text=mutation.print_op();assert_eq!(mutation.to_named_record().0,*kind);assert!(recognizer.recognize(&text).unwrap(),"{kind}: {text}");assert_eq!(Fem3dMutation::parse_op(&text).unwrap(),mutation);
        for invalid in [format!("{text} unknown-field=1"),format!("{text} {text}"),format!("fem.3d-op {text}"),kind.to_string()]{assert!(!recognizer.recognize(&invalid).unwrap(),"{invalid}");assert!(Fem3dMutation::parse_op(&invalid).is_err(),"{invalid}");}
    }
    for text in ["add-node node=1","set-load load=1","set-support node=1","commit-step commit=\"x\"","delete-node id=\"n1\" id=\"n2\"","change-combination-term id=\"c1\" case-id=\"lc1\""]{assert!(!recognizer.recognize(text).unwrap(),"{text}");assert!(Fem3dMutation::parse_op(text).is_err(),"{text}");}
    for text in ["change-combination-term id=\"c1\" case-id=\"lc1\" factor { }","change-combination-term factor { value=2 } case-id=\"lc1\" id=\"c1\""]{assert!(recognizer.recognize(text).unwrap(),"{text}");assert!(Fem3dMutation::parse_op(text).is_ok(),"{text}");}
    println!("[DEBUG] FEM3d exact one-operation grammar39 payloads and162 refusals, nullable factor, unordered required fields; no artifact prefix or invented commands");
}
           