//! 🔗️ Product link and geometry checks: the dangling accessory and component links, the link quantities and the geometry reference of one catalogue product. See [`super`].

use super::*;

/// 🔗 Checks that every accessory and component link points at a declared product.
pub(super) fn check_dangling_links(
    out: &mut Vec<CheckResult>,
    product_ids: &BTreeSet<String>,
    product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct,
    article: &str,
) {
    let accessory_ids: Vec<String> = product.accessories.iter().map(|a| a.accessory_id.clone()).collect();
    let dangling_acc: Vec<_> = accessory_ids.iter().filter(|id| !product_ids.contains(*id)).cloned().collect();
    let mut c = CheckResult::assess(
        format!("vdi3805.1.accessories.{article}"),
        part_label(1),
        clause("1", "4.5"),
        subject_product(product),
        copy("Product accessories", "Produktzubehör"),
    )
    .annex(ANNEX);
    if dangling_acc.is_empty() {
        c = c.status(CheckStatus::Pass).explanation(copy(
            "All accessory references resolve.",
            "Alle Zubehörreferenzen sind auflösbar.",
        ));
    } else {
        let ai = product
            .accessories
            .iter()
            .position(|a| !product_ids.contains(&a.accessory_id))
            .unwrap_or(0);
        c = c
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("accessories reference unknown products {dangling_acc:?}."),
                format!("accessories verweist auf unbekannte Produkte {dangling_acc:?}."),
            ))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, &format!("accessories[{ai}].accessoryId")), copy("Accessory id", "Zubehör-ID")),
                product_ids.iter().cloned().collect(),
                copy(
                    "Point accessories[].accessoryId at an existing catalogue product id.",
                    "accessories[].accessoryId auf eine vorhandene Katalog-Produkt-ID setzen.",
                ),
            ));
    }
    out.push(c.build());

    let component_ids: Vec<String> = product.components.iter().map(|c| c.component_id.clone()).collect();
    let dangling_comp: Vec<_> = component_ids.iter().filter(|id| !product_ids.contains(*id)).cloned().collect();
    let mut c = CheckResult::assess(
        format!("vdi3805.1.components.{article}"),
        part_label(1),
        clause("1", "4.5"),
        subject_product(product),
        copy("Product components", "Produktkomponenten"),
    )
    .annex(ANNEX);
    if dangling_comp.is_empty() {
        c = c.status(CheckStatus::Pass).explanation(copy(
            "All component references resolve.",
            "Alle Komponentenreferenzen sind auflösbar.",
        ));
    } else {
        let ci = product
            .components
            .iter()
            .position(|c| !product_ids.contains(&c.component_id))
            .unwrap_or(0);
        c = c
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("components reference unknown products {dangling_comp:?}."),
                format!("components verweist auf unbekannte Produkte {dangling_comp:?}."),
            ))
            .remedy(Remedy::one_of(
                SubjectRef::new(article, product_path(article, &format!("components[{ci}].componentId")), copy("Component id", "Komponenten-ID")),
                product_ids.iter().cloned().collect(),
                copy(
                    "Point components[].componentId at an existing catalogue product id.",
                    "components[].componentId auf eine vorhandene Katalog-Produkt-ID setzen.",
                ),
            ));
    }
    out.push(c.build());
}

/// 🔢 Checks the accessory and component quantities and the required accessories.
pub(super) fn check_link_quantities(
    out: &mut Vec<CheckResult>,
    product_ids: &BTreeSet<String>,
    product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct,
    article: &str,
) {
    for (i, link) in product.accessories.iter().enumerate() {
        let qty_ok = link.quantity >= 1;
        let mut q = CheckResult::assess(
            format!("vdi3805.1.accessories.quantity.{article}.{i}"),
            part_label(1),
            clause("1", "4.5"),
            subject_product(product),
            copy("Accessory quantity", "Zubehöranzahl"),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, link.quantity as f64), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if qty_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if qty_ok {
            q = q.explanation(copy(format!("Accessory `{}` quantity {} ≥ 1.", link.accessory_id, link.quantity), format!("Zubehör `{}` Anzahl {} ≥ 1.", link.accessory_id, link.quantity)));
        } else {
            q = q
                .explanation(copy("Accessory quantity must be ≥ 1.", "Zubehöranzahl muss ≥ 1 sein."))
                .remedy(Remedy::at_least(
                    SubjectRef::new(article, product_path(article, &format!("accessories[{i}].quantity")), copy("Accessory quantity", "Zubehöranzahl")),
                    Quantity::new(QuantityKind::Dimensionless, link.quantity as f64),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy("Set accessories[].quantity to at least 1.", "accessories[].quantity auf mindestens 1 setzen."),
                ));
        }
        out.push(q.build());
        let present = product_ids.contains(&link.accessory_id);
        let req_ok = !link.required || present;
        let mut rq = CheckResult::assess(
            format!("vdi3805.1.accessories.required.{article}.{i}"),
            part_label(1),
            clause("1", "4.5"),
            subject_product(product),
            copy("Required accessory present", "Erforderliches Zubehör vorhanden"),
        )
        .annex(ANNEX)
        .minimum(
            Quantity::new(QuantityKind::Dimensionless, if present { 1.0 } else { 0.0 }),
            Quantity::new(QuantityKind::Dimensionless, if link.required { 1.0 } else { 0.0 }),
        )
        .status(if req_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if req_ok {
            rq = rq.explanation(copy(
                format!("Accessory `{}` required={} present={}.", link.accessory_id, link.required, present),
                format!("Zubehör `{}` required={} present={}.", link.accessory_id, link.required, present),
            ));
        } else {
            rq = rq
                .explanation(copy(
                    format!("Required accessory `{}` is missing from the catalogue.", link.accessory_id),
                    format!("Erforderliches Zubehör `{}` fehlt im Katalog.", link.accessory_id),
                ))
                .remedy(Remedy::exactly(
                    SubjectRef::new(article, product_path(article, &format!("accessories[{i}].accessoryId")), copy("Accessory id", "Zubehör-ID")),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy("Add the required accessory product to the catalogue.", "Erforderliches Zubehörprodukt dem Katalog hinzufügen."),
                ));
        }
        out.push(rq.build());
    }
    for (i, link) in product.components.iter().enumerate() {
        let qty_ok = link.quantity >= 1;
        let mut q = CheckResult::assess(
            format!("vdi3805.1.components.quantity.{article}.{i}"),
            part_label(1),
            clause("1", "4.6"),
            subject_product(product),
            copy("Component quantity", "Komponentenanzahl"),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, link.quantity as f64), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if qty_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if qty_ok {
            q = q.explanation(copy(format!("Component `{}` quantity {} ≥ 1.", link.component_id, link.quantity), format!("Komponente `{}` Anzahl {} ≥ 1.", link.component_id, link.quantity)));
        } else {
            q = q
                .explanation(copy("Component quantity must be ≥ 1.", "Komponentenanzahl muss ≥ 1 sein."))
                .remedy(Remedy::at_least(
                    SubjectRef::new(article, product_path(article, &format!("components[{i}].quantity")), copy("Component quantity", "Komponentenanzahl")),
                    Quantity::new(QuantityKind::Dimensionless, link.quantity as f64),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    copy("Set components[].quantity to at least 1.", "components[].quantity auf mindestens 1 setzen."),
                ));
        }
        out.push(q.build());
    }
}

/// 📐 Checks the geometry bounding box, connections and parameters when a geometry reference is set.
pub(super) fn check_product_geometry(
    document: &Vdi3805Snapshot,
    out: &mut Vec<CheckResult>,
    product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct,
    article: &str,
    id_ok: bool,
) {
    if let Some(gref) = product.configuration.geometry_ref.as_ref() {
        if let Some(geom) = document.geometry.get(gref) {
            let bbox = &geom.bbox;
            let mut outside = Vec::new();
            for conn in &geom.connections {
                let [x, y, z] = conn.position;
                if x < bbox.min_x || x > bbox.max_x || y < bbox.min_y || y > bbox.max_y || z < bbox.min_z || z > bbox.max_z {
                    outside.push(conn.id.clone());
                }
            }
            let bbox_ok = outside.is_empty() && bbox.max_x >= bbox.min_x && bbox.max_y >= bbox.min_y && bbox.max_z >= bbox.min_z;
            let volume = ((bbox.max_x - bbox.min_x) * (bbox.max_y - bbox.min_y) * (bbox.max_z - bbox.min_z)).abs();
            let mut g = CheckResult::assess(
                format!("vdi3805.1.geometry.bbox.{article}"),
                part_label(1),
                clause("1", "5.2"),
                subject_product(product),
                copy("Geometry bounding box", "Geometrie-Begrenzungsrahmen"),
            )
            .annex(ANNEX)
            .minimum(Quantity::new(QuantityKind::Volume, if bbox_ok { volume.max(1e-12) } else { 0.0 }), Quantity::new(QuantityKind::Volume, 1e-12))
            .status(if bbox_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if bbox_ok {
                g = g.explanation(copy("All connection points lie inside the geometry bbox.", "Alle Anschlusspunkte liegen im Geometrie-BBox."));
            } else {
                g = g
                    .explanation(copy(format!("Connection points outside bbox: {outside:?}."), format!("Anschlusspunkte außerhalb der BBox: {outside:?}.")))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(gref, format!("geometry[id={gref}].bbox.maxX"), copy("BBox maximum X", "Maximale X-Koordinate der Bounding-Box")),
                        Quantity::new(QuantityKind::Length, bbox.max_x),
                        Quantity::new(QuantityKind::Length, bbox.max_x.max(bbox.min_x) + 0.01),
                        copy("Expand geometry bbox so every connection point is inside.", "Geometrie-BBox so erweitern, dass jeder Anschlusspunkt innen liegt."),
                    ));
            }
            out.push(g.build());
            let scale = geom.parameters.get("scale").copied().unwrap_or(0.0);
            let params_ok = scale > 0.0;
            let mut p = CheckResult::assess(
                format!("vdi3805.1.geometry.parameters.{article}"),
                part_label(1),
                clause("1", "5.2"),
                subject_product(product),
                copy("Geometry parameters", "Geometrieparameter"),
            )
            .annex(ANNEX)
            .minimum(Quantity::new(QuantityKind::Dimensionless, scale), Quantity::new(QuantityKind::Dimensionless, 1e-6))
            .status(if params_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if params_ok {
                p = p.explanation(copy(format!("geometry.parameters.scale = {scale}."), format!("geometry.parameters.scale = {scale} (Geometrie).")));
            } else {
                p = p
                    .explanation(copy("geometryRef set but parameters.scale is missing or not positive.", "geometryRef gesetzt, aber parameters.scale fehlt oder ist nicht positiv."))
                    .remedy(Remedy::at_least(
                        SubjectRef::new(gref, format!("geometry[id={gref}].parameters.scale"), copy("Geometry scale", "Geometrie-Maßstab")),
                        Quantity::new(QuantityKind::Dimensionless, scale),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set geometry.parameters.scale to a positive value.", "geometry.parameters.scale auf einen positiven Wert setzen."),
                    ));
            }
            out.push(p.build());
            for (pk, pv) in &geom.parameters {
                let mut pkc = CheckResult::assess(
                    format!("vdi3805.1.geometry.parameter.{article}.{pk}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Geometry parameter", "Geometrieparameter"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, *pv), Quantity::new(QuantityKind::Dimensionless, 0.0))
                .status(if pv.is_finite() && *pv >= 0.0 { CheckStatus::Pass } else { CheckStatus::Fail });
                if pv.is_finite() && *pv >= 0.0 {
                    pkc = pkc.explanation(copy(format!("parameter `{pk}` = {pv}."), format!("Parameter `{pk}` = {pv} (Geometrie).")));
                } else {
                    pkc = pkc
                        .explanation(copy(format!("parameter `{pk}` must be a finite non-negative number."), format!("Parameter `{pk}` muss eine endliche nicht-negative Zahl sein.")))
                        .remedy(Remedy::at_least(
                            SubjectRef::new(gref, format!("geometry[id={gref}].parameters.{pk}"), copy("Geometry parameter", "Geometrieparameter")),
                            Quantity::new(QuantityKind::Dimensionless, *pv),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            copy("Set the geometry parameter to a non-negative finite value.", "Geometrieparameter auf einen nicht-negativen endlichen Wert setzen."),
                        ));
                }
                out.push(pkc.build());
            }
            for (ci, conn) in geom.connections.iter().enumerate() {
                let dia = conn.diameter_mm.unwrap_or(0.0);
                let mut cd = CheckResult::assess(
                    format!("vdi3805.1.geometry.connection.{article}.{ci}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Geometry connection", "Geometrieanschluss"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Length, dia / 1000.0), Quantity::new(QuantityKind::Length, 0.001));
                let medium_ok = CONNECTION_MEDIUM_CODES.iter().any(|m| *m == conn.medium.as_str());
                let id_ok = CONNECTION_ID_CODES.iter().any(|m| *m == conn.id.as_str());
                let conn_ok = dia >= 1.0 && medium_ok && id_ok;
                cd = cd.status(if conn_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if conn_ok {
                    cd = cd.explanation(copy(
                        format!("Connection `{}` medium={} Ø={} mm.", conn.id, conn.medium, dia),
                        format!("Anschluss `{}` Medium={} Ø={} mm.", conn.id, conn.medium, dia),
                    ));
                } else if !id_ok {
                    cd = cd
                        .explanation(copy(
                            format!("Connection id `{}` is not in the admitted connection id list.", conn.id),
                            format!("Anschluss-ID `{}` ist nicht in der zugelassenen Anschluss-ID-Liste.", conn.id),
                        ))
                        .remedy(Remedy::one_of(
                            SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].id", conn.id), copy("Connection id", "Anschluss-ID")),
                            CONNECTION_ID_CODES.iter().map(|s| (*s).to_string()).collect(),
                            copy("Set connection.id to an admitted connection id.", "connection.id auf eine zugelassene Anschluss-ID setzen."),
                        ));
                } else {
                    cd = cd
                        .explanation(copy("Connection needs id, medium and diameterMm ≥ 1.", "Anschluss braucht id, medium und diameterMm ≥ 1."))
                        .remedy(Remedy::at_least(
                            SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].diameterMm", conn.id), copy("Connection diameter", "Anschlussdurchmesser")),
                            Quantity::new(QuantityKind::Length, dia / 1000.0),
                            Quantity::new(QuantityKind::Length, 0.015),
                            copy("Set connection diameterMm to a positive millimetre value.", "connection diameterMm auf einen positiven Millimeterwert setzen."),
                        ));
                }
                out.push(cd.build());
                let [px, py, pz] = conn.position;
                for (axis, val, min_b, max_b) in [
                    ("x", px, bbox.min_x, bbox.max_x),
                    ("y", py, bbox.min_y, bbox.max_y),
                    ("z", pz, bbox.min_z, bbox.max_z),
                ] {
                    let inside = val >= min_b && val <= max_b;
                    let span = (max_b - min_b).abs().max(1e-9);
                    let mid = (min_b + max_b) * 0.5;
                    let mut pos = CheckResult::assess(
                        format!("vdi3805.1.geometry.connection.position.{article}.{ci}.{axis}"),
                        part_label(1),
                        clause("1", "5.2"),
                        subject_product(product),
                        copy("Connection position", "Anschlussposition"),
                    )
                    .annex(ANNEX)
                    .minimum(Quantity::new(QuantityKind::Length, val), Quantity::new(QuantityKind::Length, min_b))
                    .utilization(Quantity::new(QuantityKind::Length, (val - min_b).abs()), Quantity::new(QuantityKind::Length, span))
                    .status(if inside { CheckStatus::Pass } else { CheckStatus::Fail });
                    if inside {
                        pos = pos.explanation(copy(
                            format!("Connection `{}` {axis}={val:.4} within bbox [{min_b:.4}, {max_b:.4}].", conn.id),
                            format!("Anschluss `{}` {axis}={val:.4} innerhalb BBox [{min_b:.4}, {max_b:.4}].", conn.id),
                        ));
                    } else {
                        pos = pos
                            .explanation(copy(
                                format!("Connection `{}` {axis}={val:.4} outside bbox [{min_b:.4}, {max_b:.4}].", conn.id),
                                format!("Anschluss `{}` {axis}={val:.4} außerhalb BBox [{min_b:.4}, {max_b:.4}].", conn.id),
                            ))
                            .remedy(Remedy::exactly(
                                SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].position.{axis}", conn.id), copy("Connection position", "Anschlussposition")),
                                Quantity::new(QuantityKind::Length, val),
                                Quantity::new(QuantityKind::Length, mid),
                                copy("Move the connection point inside the geometry bbox.", "Anschlusspunkt in die Geometrie-BBox verschieben."),
                            ));
                    }
                    out.push(pos.build());
                }
                let dir_norm = (conn.direction[0].powi(2) + conn.direction[1].powi(2) + conn.direction[2].powi(2)).sqrt();
                let dir_ok = (dir_norm - 1.0).abs() < 0.05;
                let mut cp = CheckResult::assess(
                    format!("vdi3805.1.geometry.connection.direction.{article}.{ci}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Connection direction unit vector", "Anschlussrichtungs-Einheitsvektor"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, dir_norm), Quantity::new(QuantityKind::Dimensionless, 0.95))
                .status(if dir_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if dir_ok {
                    cp = cp.explanation(copy(format!("Connection `{}` direction ‖n‖={dir_norm:.3}.", conn.id), format!("Anschluss `{}` Richtung ‖n‖={dir_norm:.3}.", conn.id)));
                } else {
                    cp = cp
                        .explanation(copy(format!("Connection direction norm {dir_norm:.3} must be ≈ 1.",), format!("Anschlussrichtungsbetrag {dir_norm:.3} muss ≈ 1 sein.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].direction[0]", conn.id), copy("Direction x", "Richtung x")),
                            Quantity::new(QuantityKind::Dimensionless, conn.direction[0]),
                            Quantity::new(QuantityKind::Dimensionless, conn.direction[0] / dir_norm.max(1e-12)),
                            copy("Normalize the connection direction to a unit vector.", "Anschlussrichtung auf einen Einheitsvektor normalisieren."),
                        ));
                }
                out.push(cp.build());
            }
        }
    }
}
