//! 🧪️ The bundled examples as a roster — ids, ordering and the solve each one is meant to have.

#[test]
fn the_roster_is_the_four_bundled_problems() {
    let sources = crate::examples::sources();
    let ids: Vec<&str> = sources.iter().map(|source| source.id()).collect();
    assert_eq!(ids, vec!["two-room-corridor", "wall-roof-facade-strip", "hex-ring", "terrain-ring"]);
    assert_eq!(crate::examples::example_source_slice().len(), 4);
}

#[test]
fn every_example_keeps_its_collections_in_canonical_order() {
    for document in crate::examples::documents() {
        for ids in [
            document.slots.iter().map(|row| row.id.clone()).collect::<Vec<_>>(),
            document.edges.iter().map(|row| row.id.clone()).collect::<Vec<_>>(),
            document.tiles.iter().map(|row| row.id.clone()).collect::<Vec<_>>(),
            document.rules.iter().map(|row| row.id.clone()).collect::<Vec<_>>(),
        ] {
            let mut sorted = ids.clone();
            sorted.sort();
            assert_eq!(ids, sorted, "collections must stay in ascending id order");
        }
    }
}

#[test]
fn every_example_solves_without_contradiction() {
    for document in crate::examples::documents() {
        let commit = crate::schema::inferences::solve_with_job(&document).expect("example solves");
        assert!(!commit.contradiction, "a bundled example must be satisfiable");
        assert_eq!(commit.assignments.len(), document.slots.len());
    }
}
