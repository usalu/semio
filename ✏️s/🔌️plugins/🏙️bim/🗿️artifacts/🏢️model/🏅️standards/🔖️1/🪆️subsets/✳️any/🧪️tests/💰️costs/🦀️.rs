//! 💰️ BIM inference case `costs`, Rust adapter (subject role only). The third-party reproduction lives in `🐍️.py` beside this file (`sympy` rationals); this adapter
//! prices the same committed fixture with `infer_costs` of `semio-s-artifact-bim-model-costs`, and the platform compares both projections under `floating-point-v1`.
//! It registers no oracle handler: a subject that re-read the committed expectation would be a self-comparison reporting a pass.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_bim_model_costs::standards::v1::subsets::any::schema::inferences::costs::{infer_costs, CostBasis, CostItem, CostQuantity, ModelCosts, TypeCostLink};
    use std::collections::BTreeMap;

    const BASES: [(&str, CostBasis); 10] = [
        ("count", CostBasis::Count),
        ("length", CostBasis::Length),
        ("gross_area", CostBasis::GrossArea),
        ("net_area", CostBasis::NetArea),
        ("surface_area", CostBasis::SurfaceArea),
        ("gross_side_area", CostBasis::GrossSideArea),
        ("net_side_area", CostBasis::NetSideArea),
        ("gross_volume", CostBasis::GrossVolume),
        ("net_volume", CostBasis::NetVolume),
        ("mass", CostBasis::Mass),
    ];

    struct Case {
        items: BTreeMap<String, CostItem>,
        links: BTreeMap<String, TypeCostLink>,
        elements: BTreeMap<String, CostQuantity>,
    }

    fn members(value: Option<&Json>) -> Vec<(String, &Json)> {
        match value {
            Some(Json::Object(entries)) => entries.iter().map(|(key, member)| (key.clone(), member)).collect(),
            _ => Vec::new(),
        }
    }

    fn number(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => f64::NAN,
        }
    }

    fn items(value: Option<&Json>) -> BTreeMap<String, CostItem> {
        members(value)
            .into_iter()
            .map(|(id, item)| (id, CostItem { name: item.str("name"), currency: item.str("currency"), basis: CostBasis::parse(&item.str("basis")), unit_cost: number(item, "unit_cost"), precision: number(item, "precision") }))
            .collect()
    }

    fn links(value: Option<&Json>) -> BTreeMap<String, TypeCostLink> {
        members(value).into_iter().map(|(id, link)| (id, TypeCostLink { type_id: link.str("type_id"), item: link.str("item"), factor: number(link, "factor") })).collect()
    }

    fn case(ctx: &Context) -> Result<Case, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.ends_with("/💰️costs/🔣️.json")).ok_or_else(|| "the scenario names no costs fixture".to_string())?;
        let fixture = parse_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed costs are not UTF-8: {error}"))?)?;
        let elements = members(fixture.get("elements"))
            .into_iter()
            .map(|(id, row)| {
                let measures = BASES.iter().filter_map(|(key, basis)| row.get(key).map(|_| (*basis, number(row, key)))).collect();
                (id, CostQuantity { storey: row.str("storey"), type_id: row.str("type_id"), measures })
            })
            .collect();
        Ok(Case { items: items(fixture.get("items")), links: links(fixture.get("links")), elements })
    }

    fn totals(scope: &BTreeMap<String, BTreeMap<String, f64>>) -> Json {
        Json::Object(scope.iter().map(|(id, currencies)| (id.clone(), currency_totals(currencies))).collect())
    }

    fn currency_totals(currencies: &BTreeMap<String, f64>) -> Json {
        Json::Object(currencies.iter().map(|(currency, value)| (currency.clone(), Json::Number(*value))).collect())
    }

    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }

    fn projection(costs: &ModelCosts) -> Json {
        let lines = costs.lines.iter().map(|line| {
            Json::Object(vec![
                ("element".into(), text(&line.element)),
                ("link".into(), text(&line.link)),
                ("item".into(), text(&line.item)),
                ("type_id".into(), text(&line.type_id)),
                ("storey".into(), text(&line.storey)),
                ("currency".into(), text(&line.currency)),
                ("quantity".into(), Json::Number(line.quantity)),
                ("unit_cost".into(), Json::Number(line.unit_cost)),
                ("factor".into(), Json::Number(line.factor)),
                ("precision".into(), Json::Number(line.precision)),
                ("total".into(), Json::Number(line.total)),
            ])
        });
        let diagnostics = costs.diagnostics.iter().map(|row| Json::Object(vec![("code".into(), text(row.code)), ("link".into(), text(&row.link)), ("item".into(), text(&row.item)), ("element".into(), text(&row.element))]));
        Json::Object(vec![
            ("lines".into(), Json::Array(lines.collect())),
            ("diagnostics".into(), Json::Array(diagnostics.collect())),
            ("elements".into(), totals(&costs.elements)),
            ("types".into(), totals(&costs.types)),
            ("storeys".into(), totals(&costs.storeys)),
            ("items".into(), totals(&costs.items)),
            ("project".into(), currency_totals(&costs.project)),
        ])
    }

    fn answer(projection: Json) -> Result<Outcome, String> {
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    fn priced(case: &Case) -> ModelCosts {
        infer_costs(&case.items, &case.links, &case.elements)
    }

    /// 💰️ One type contributes several independently authored cost items.
    pub fn items_of_one_type(ctx: &Context) -> Result<Outcome, String> {
        answer(projection(&priced(&case(ctx)?)))
    }

    /// 💱️ The project retains one total per currency.
    pub fn currencies(ctx: &Context) -> Result<Outcome, String> {
        let costs = priced(&case(ctx)?);
        answer(Json::Object(vec![("project".into(), currency_totals(&costs.project)), ("storeys".into(), totals(&costs.storeys))]))
    }

    /// ↩️ Doubling one wall's quantities doubles its costs; restoring them returns every cost and no authored record changes.
    pub fn replay(ctx: &Context) -> Result<Outcome, String> {
        let case = case(ctx)?;
        let (items, links) = (case.items.clone(), case.links.clone());
        let original = priced(&case);
        let mut doubled_rows = case.elements.clone();
        for basis in [CostBasis::NetSideArea, CostBasis::NetVolume] {
            doubled_rows.get_mut("wall-a").and_then(|row| row.measures.get_mut(&basis)).map(|value| *value *= 2.0).ok_or_else(|| "wall-a carries no measure to double".to_string())?;
        }
        let doubled = infer_costs(&case.items, &case.links, &doubled_rows);
        let restored = priced(&case);
        if restored != original || items != case.items || links != case.links {
            return Err("replay changed the authored records or did not restore the costs".into());
        }
        answer(Json::Object(vec![("original".into(), projection(&original)), ("doubled".into(), projection(&doubled)), ("restored".into(), projection(&restored))]))
    }

    /// 🚫️ The fixture plus the further authored records of the scenario; invalid ones are reported and priced as nothing.
    pub fn diagnostics(ctx: &Context) -> Result<Outcome, String> {
        let mut case = case(ctx)?;
        let extra = ctx.doc_json()?;
        case.items.extend(items(extra.get("items")));
        case.links.extend(links(extra.get("links")));
        answer(projection(&priced(&case)))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; ids are the feature's `@id-*` tags.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("costs-items", subject::items_of_one_type).subject("costs-currencies", subject::currencies).subject("costs-replay", subject::replay).subject("costs-diagnostics", subject::diagnostics);
    }
    built
}
//#endregion 🔖️Registration
