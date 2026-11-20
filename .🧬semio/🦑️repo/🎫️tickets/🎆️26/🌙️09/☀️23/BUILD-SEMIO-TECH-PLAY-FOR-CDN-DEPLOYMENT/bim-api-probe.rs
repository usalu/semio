use semio_framework_value_retained_map::ordered_map::RetainedOrderedMap;
use semio_framework_value_retained_map::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};

type Rows = RetainedOrderedMap<String, u32>;

#[derive(Default)]
struct Projection {
    rows: Rows,
    scopes: RetainedOrderedMap<String, Vec<String>>,
}

fn project(input: &[(String, u32)]) -> Projection {
    let mut groups = BTreeMap::<String, u32>::new();
    for (id, amount) in input {
        *groups.entry(id.clone()).or_default() += amount;
    }
    let mut projection = Projection { rows: groups.into_iter().collect(), ..Default::default() };
    for key in ["scope"] {
        projection.scopes.cold_insert(
            key.to_owned(),
            projection.rows.iter().filter(|(_, amount)| **amount > 9).map(|(id, _)| id.clone()).collect(),
        );
    }
    projection
}

fn update_total(totals: &mut Rows, key: String, update: impl FnOnce(&mut u32)) {
    if let Some(total) = totals.get_mut(&key) {
        update(total);
        return;
    }
    let mut total = 0;
    update(&mut total);
    totals.cold_insert(key, total);
}

#[derive(Default)]
struct Placements {
    before: BTreeMap<String, u32>,
    joins: BTreeSet<String>,
}

fn inverse(changes: Placements) -> (Option<Rows>, BTreeSet<String>) {
    let before = (!changes.before.is_empty()).then(|| changes.before.into_iter().collect());
    (before, changes.joins)
}

fn main() {
    let input = [("zeta".into(), 7u32), ("alpha".into(), 11u32), ("zeta".into(), 3u32)];
    let mut rows = project(&input).rows;
    update_total(&mut rows, "zeta".into(), |value| *value += 2);
    update_total(&mut rows, "new".into(), |value| *value += 5);
    let result: BTreeMap<String, u32> = rows.iter().map(|(id, amount)| (id.clone(), *amount)).collect();
    assert_eq!(result, BTreeMap::from([("alpha".into(), 11), ("new".into(), 5), ("zeta".into(), 12)]));
    assert_eq!(rows["zeta"], 12);
    let children: RetainedOrderedMap<String, Rows> = [("set".to_owned(), [("row".to_owned(), 8)].into())].into();
    assert_eq!(children.get("set").into_iter().flatten().next(), Some((&"row".to_owned(), &8)));
    let flattened: Vec<_> = (&rows).into_iter().map(|(id, amount)| (id.as_str(), *amount)).collect();
    assert_eq!(flattened, [("alpha", 11), ("new", 5), ("zeta", 12)]);
    assert_eq!(
        serde_json::to_string(&rows).unwrap(),
        serde_json::to_string(&result).unwrap(),
    );
    assert_eq!(serde_json::from_str::<Rows>(r#"{"alpha":11,"new":5,"zeta":12}"#).unwrap(), rows);
    assert_eq!(rows.to_value(), result.to_value());
    assert_eq!(Rows::from_value(result.to_value()).unwrap(), rows);
    let before = inverse(Placements { before: result, joins: BTreeSet::from(["a".into()]) });
    assert_eq!(before, (Some(rows), BTreeSet::from(["a".into()])));
}
