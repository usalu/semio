extern crate semio_os_kernel as protocol;

use protocol::OpText;
use semio_framework_test_fixtures::repo_text;
use semio_framework_dsl_record::DslVariants;
use semio_framework_value::FromValue;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::io::text::mutations::mutation_grammar_recognizer;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::io::text::mutations::MUTATION_GRAMMAR_SEMIO;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;

#[test]
fn component_grammar_semio_matches_the_exact_declared_mutation_language(){
    let grammar=semio_framework_dsl::parse_grammar(MUTATION_GRAMMAR_SEMIO).unwrap();assert_eq!(grammar.dialect,semio_framework_dsl::SemioDialect::Grammar);let recognizer=mutation_grammar_recognizer().unwrap();
    let cases=&[
        ("create-node",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/🦠️mutation/🔣️.json")),
        ("delete-node",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🕳️delete-node/🚫️removes/🦠️mutation/🔣️.json")),
        ("create-element",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧩️create-element/➖️appends/🦠️mutation/🔣️.json")),
        ("delete-element",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-element/🚫️removes/🦠️mutation/🔣️.json")),
        ("replace-element",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls/🦠️mutation/🔣️.json")),
        ("create-material",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🌱️create-material/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-material",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-material/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-material",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-material/🏗️hall/🦠️mutation/🔣️.json")),
        ("change-material",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/📉️softens/🦠️mutation/🔣️.json")),
        ("create-section",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/📐️create-section/🔳️appends/🦠️mutation/🔣️.json")),
        ("delete-section",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-section/🚫️removes/🦠️mutation/🔣️.json")),
        ("replace-section",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/📏️replace-section/🏗️hall/🦠️mutation/🔣️.json")),
        ("change-section",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-section/🩹️changes/🦠️mutation/🔣️.json")),
        ("create-support",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🛡️create-support/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-support",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-support/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-support",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-support/🏗️hall/🦠️mutation/🔣️.json")),
        ("create-solid",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧊️create-solid/🏠️appends/🦠️mutation/🔣️.json")),
        ("delete-solid",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🚫️delete-solid/🚫️removes/🦠️mutation/🔣️.json")),
        ("replace-solid",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🔄️replace-solid/📚️thickens/🦠️mutation/🔣️.json")),
        ("create-load-case",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/📋️create-load-case/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-load-case",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-load-case/🏗️hall/🦠️mutation/🔣️.json")),
        ("add-load",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/➕️add-load/🏠️lays/🦠️mutation/🔣️.json")),
        ("remove-load",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-load/➖️drops/🦠️mutation/🔣️.json")),
        ("change-load-case-self-weight",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall/🦠️mutation/🔣️.json")),
        ("create-combination",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🔗️create-combination/🏗️hall/🦠️mutation/🔣️.json")),
        ("delete-combination",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/✂️delete-combination/✂️removes-the-182f7b/🦠️mutation/🔣️.json")),
        ("update-analysis-settings",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-node",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-node/📍️lifts/🦠️mutation/🔣️.json")),
        ("change-node",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-node/🩹️changes/🦠️mutation/🔣️.json")),
        ("change-combination-term",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-term/⚖️weights/🦠️mutation/🔣️.json")),
        ("change-combination-name",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-combination-name/⚗️empty/🦠️mutation/🔣️.json")),
        ("change-load",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-load/📍️target/🦠️mutation/🔣️.json")),
        ("change-element",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-element/🔀️variant/🦠️mutation/🔣️.json")),
        ("change-support",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-support/📌️toggles/🦠️mutation/🔣️.json")),
        ("change-solid",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-solid/🩹️changes/🦠️mutation/🔣️.json")),
        ("replace-load",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-load/🔁️retunes/🦠️mutation/🔣️.json")),
        ("change-load-case-name",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-load-case-name/🏗️hall/🦠️mutation/🔣️.json")),
        ("replace-combination",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🧫️fixtures/🧬️mutations/🔁️replace-combination/🏗️hall/🦠️mutation/🔣️.json")),
        ("move-selection",repo_text!("✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/🦠️mutation/🔣️.json")),
    ];
    let variants=Fem3dMutation::variants();assert_eq!(variants.len(),cases.len());assert_eq!(grammar.productions.len(),cases.len()+1);
    for(kind,json)in cases{
        let mutation=semio_framework_pack_json::from_json_str::<Fem3dMutation>(json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let text=mutation.print_op();assert_eq!(mutation.to_named_record().0,*kind);assert!(recognizer.recognize(&text).unwrap(),"{kind}: {text}");assert_eq!(Fem3dMutation::parse_op(&text).unwrap(),mutation);
        for invalid in [format!("{text} unknown-field=1"),format!("{text} {text}"),format!("fem.3d-op {text}"),kind.to_string()]{assert!(!recognizer.recognize(&invalid).unwrap(),"{invalid}");assert!(Fem3dMutation::parse_op(&invalid).is_err(),"{invalid}");}
    }
    for text in ["add-node node=1","set-load load=1","set-support node=1","commit-step commit=\"x\"","delete-node id=\"n1\" id=\"n2\"","change-combination-term id=\"c1\" case-id=\"lc1\""]{assert!(!recognizer.recognize(text).unwrap(),"{text}");assert!(Fem3dMutation::parse_op(text).is_err(),"{text}");}
    for text in ["change-combination-term id=\"c1\" case-id=\"lc1\" factor { }","change-combination-term factor { value=2 } case-id=\"lc1\" id=\"c1\""]{assert!(recognizer.recognize(text).unwrap(),"{text}");assert!(Fem3dMutation::parse_op(text).is_ok(),"{text}");}
}
           