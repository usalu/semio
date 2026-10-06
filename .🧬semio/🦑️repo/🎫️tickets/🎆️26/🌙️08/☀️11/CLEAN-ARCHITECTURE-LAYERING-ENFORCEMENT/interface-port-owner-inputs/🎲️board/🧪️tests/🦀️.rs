use super::would_create_cycle;

#[derive(semio_framework_value_derive::FromValue)]
struct Corpus { cases: Vec<Case> }

#[derive(semio_framework_value_derive::FromValue)]
struct Case { edges: Vec<Vec<String>>, source: String, target: String, cycle: bool }

#[test]
fn defining_engine_cycle_corpus_matches_neutral_sqlite_oracle() {
    let corpus: Corpus = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🪪️defining-engine/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral engine corpus");
    for case in corpus.cases {
        let edges: Vec<_> = case.edges.into_iter().map(|edge| (edge[0].clone(), edge[1].clone())).collect();
        assert_eq!(would_create_cycle(&edges, &case.source, &case.target), case.cycle);
    }
}
