//! 🧩 Puzzle5d hierarchical mutation fixtures — catalog under `🧩puzzle5d/`.

#[test]
fn puzzle5d_catalog_lists_every_semantic_kind() {
    let catalog = include_str!("🧩puzzle5d/🔣️catalog.json");
    let v: serde_json::Value = serde_json::from_str(catalog).expect("catalog json");
    let vectors = v.get("vectors").and_then(|x| x.as_array()).expect("vectors");
    let kinds = [
        "change-annex",
        "change-title",
        "change-design-working-life",
        "change-delta-c-dev",
        "change-cement-type",
        "change-concrete-f-ck",
        "change-reinforcement-f-yk",
        "insert-member",
        "remove-member",
        "reorder-members",
        "change-member-width",
        "change-member-height",
        "change-member-effective-depth",
        "change-member-cover",
        "change-member-exposure",
        "change-member-span",
        "change-member-stirrup-spacing",
        "change-member-axis-distance",
        "change-member-fire-rating",
        "change-bar-layer-count",
        "change-bar-layer-diameter",
        "change-action-mk",
        "change-action-n-ed",
        "change-action-v-ed",
        "insert-anchor",
        "remove-anchor",
        "change-anchor-h-ef",
        "change-anchor-as",
    ];
    assert_eq!(vectors.len(), kinds.len());
    for (i, kind) in kinds.iter().enumerate() {
        assert_eq!(vectors[i].get("kind").and_then(|k| k.as_str()), Some(*kind));
    }
}
