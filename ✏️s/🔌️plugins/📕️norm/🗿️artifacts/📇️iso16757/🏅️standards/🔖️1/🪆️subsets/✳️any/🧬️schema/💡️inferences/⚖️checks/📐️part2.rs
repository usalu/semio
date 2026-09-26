//! 📐️ ISO 16757-2 geometry graph, solids, ports, spaces, placement.

use super::common::{assess, catalogue_subject, copy, fail, na, pass, q_dim, q_len, q_vol, subject};
use crate::document::{CheckReport, CheckStatus, Quantity, QuantityKind, Remedy, RemedyBound};
use crate::part_2::{GeometryCatalogue, GeometryNode, GeometryObject, SpaceKind};
use crate::standards::v1::subsets::any::schema::part_2 as helpers;
use crate::Iso16757Snapshot;
use std::collections::HashSet;

const INSTALL_CLEARANCE_M: f64 = 0.05;

pub fn check_part_2(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    check_ports_primitives_and_spaces(doc, report);

    let geometry = &doc.geometry;
    if geometry.objects.is_empty() {
        report.push(na(
            "iso16757.2.geometry",
            "2",
            "6.1",
            subject("geometry", "geometry", "Geometry catalogue", "Geometriekatalog"),
            copy("Geometry objects", "Geometrieobjekte"),
            copy("Geometry catalogue contains no objects.", "Geometriekatalog enthält keine Objekte."),
        ));
        return;
    }

    let mut graph_fail = 0u32;
    let mut volume_fail = 0u32;
    let mut port_fail = 0u32;
    let mut space_fail = 0u32;
    let mut binding_fail = 0u32;
    let mut objects_ok = 0u32;

    for (id, obj) in &geometry.objects {
        let mut visited = HashSet::new();
        let issues = helpers::validate_geometry_graph(obj, geometry, &mut visited);
        if !issues.is_empty() {
            graph_fail += 1;
            for issue in issues {
                report.push(fail(
                    format!("iso16757.2.6.1.graph.{id}"),
                    "2",
                    "6.1",
                    subject(id, format!("geometry.objects.{id}"), id, id),
                    copy("Geometry graph validity", "Gültigkeit des Geometriegraphen"),
                    copy(
                        format!("Geometry graph issue on '{id}': {issue}"),
                        format!("Geometriegraph-Problem an '{id}': {issue}"),
                    ),
                    vec![Remedy {
                        target: subject(id, format!("geometry.objects.{id}.shape"), id, id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        options: geometry.objects.keys().filter(|k| *k != id).cloned().collect(),
                        bound: RemedyBound::OneOf,
                        action: copy(
                            format!("Retarget Reference on '{id}' to an existing geometry object or complete parameter bindings."),
                            format!("Reference an '{id}' auf ein vorhandenes Geometrieobjekt umbiegen oder Parameterbindungen vervollständigen."),
                        ),
                        applicable: true,
                    }],
                ));
            }
        }

        match &obj.shape {
            Some(shape) => match helpers::evaluate_bounding_box(shape, geometry) {
                Ok(bbox) => {
                    let volume = bbox.volume_m3();
                    let dx = bbox.max[0] - bbox.min[0];
                    let dy = bbox.max[1] - bbox.min[1];
                    let dz = bbox.max[2] - bbox.min[2];
                    if volume <= 0.0 || dx <= 0.0 || dy <= 0.0 || dz <= 0.0 {
                        volume_fail += 1;
                        report.push(assess(
                            format!("iso16757.2.7.1.volume.{id}"),
                            "2",
                            "7.1",
                            subject(id, format!("geometry.objects.{id}.shape"), id, id),
                            copy("Closed solid positive volume", "Geschlossener Körper mit positivem Volumen"),
                            copy(
                                format!("Geometry '{id}' bounding volume is {volume:.6} m³ (extents {dx:.4}×{dy:.4}×{dz:.4} m) — must be strictly positive."),
                                format!("Geometrie '{id}' hat Volumen {volume:.6} m³ (Abmessungen {dx:.4}×{dy:.4}×{dz:.4} m) — muss streng positiv sein."),
                            ),
                            CheckStatus::Fail,
                            q_vol(volume.max(0.0)),
                            q_vol(1e-9),
                            vec![Remedy::at_least(
                                subject(id, format!("geometry.objects.{id}.shape"), id, id),
                                q_vol(volume.max(0.0)),
                                q_vol(1e-6),
                                copy(
                                    format!("Increase primitive extents of '{id}' so width, height and depth are all > 0 (volume ≥ 1e-6 m³)."),
                                    format!("Primitive Abmessungen von '{id}' erhöhen, sodass Breite, Höhe und Tiefe alle > 0 sind (Volumen ≥ 1e-6 m³)."),
                                ),
                            )],
                        ));
                    } else {
                        objects_ok += 1;
                        report.push(assess(
                            format!("iso16757.2.7.1.volume.{id}"),
                            "2",
                            "7.1",
                            subject(id, format!("geometry.objects.{id}.shape"), id, id),
                            copy("Closed solid positive volume", "Geschlossener Körper mit positivem Volumen"),
                            copy(
                                format!("Geometry '{id}' solid volume is {volume:.6} m³."),
                                format!("Geometrie '{id}' hat Körpervolumen {volume:.6} m³."),
                            ),
                            CheckStatus::Pass,
                            q_vol(volume),
                            q_vol(volume),
                            Vec::new(),
                        ));
                    }

                    if let Some(install) = obj.spaces.iter().find(|s| s.kind == SpaceKind::Installation) {
                        // overlaps with clearance means product grows by clearance and still intersects install — insufficient space
                        // Actually: clearance_ok = install envelope leaves INSTALL_CLEARANCE around product.
                        // Helper: !product.overlaps(install, clearance) means after expanding product by clearance it doesn't overlap install?
                        // Looking at BoundingBox::overlaps - need to check semantics.
                        let needed = [
                            (bbox.max[0] - bbox.min[0]) + 2.0 * INSTALL_CLEARANCE_M,
                            (bbox.max[1] - bbox.min[1]) + 2.0 * INSTALL_CLEARANCE_M,
                            (bbox.max[2] - bbox.min[2]) + 2.0 * INSTALL_CLEARANCE_M,
                        ];
                        let have = [
                            install.bounds.max[0] - install.bounds.min[0],
                            install.bounds.max[1] - install.bounds.min[1],
                            install.bounds.max[2] - install.bounds.min[2],
                        ];
                        let axis_ok = have[0] + 1e-9 >= needed[0] && have[1] + 1e-9 >= needed[1] && have[2] + 1e-9 >= needed[2];
                        let product_inside = bbox.min[0] >= install.bounds.min[0] + INSTALL_CLEARANCE_M - 1e-9
                            && bbox.min[1] >= install.bounds.min[1] + INSTALL_CLEARANCE_M - 1e-9
                            && bbox.min[2] >= install.bounds.min[2] + INSTALL_CLEARANCE_M - 1e-9
                            && bbox.max[0] <= install.bounds.max[0] - INSTALL_CLEARANCE_M + 1e-9
                            && bbox.max[1] <= install.bounds.max[1] - INSTALL_CLEARANCE_M + 1e-9
                            && bbox.max[2] <= install.bounds.max[2] - INSTALL_CLEARANCE_M + 1e-9;
                        let ok = axis_ok && product_inside;
                        if ok {
                            report.push(assess(
                                format!("iso16757.2.5.3.5.clearance.{id}"),
                                "2",
                                "5.3.5",
                                subject(id, format!("geometry.objects.{id}.spaces[id={}].bounds.max", install.id), id, id),
                                copy("Installation clearance", "Einbaufreiraum"),
                                copy(
                                    format!("Installation space of '{id}' provides ≥ {INSTALL_CLEARANCE_M} m clearance around the solid (envelope {:.4}×{:.4}×{:.4} m).", have[0], have[1], have[2]),
                                    format!("Einbauraum von '{id}' bietet ≥ {INSTALL_CLEARANCE_M} m Freiraum um den Körper (Hüllraum {:.4}×{:.4}×{:.4} m).", have[0], have[1], have[2]),
                                ),
                                CheckStatus::Pass,
                                q_len(have[0] * have[1] * have[2]),
                                q_len(needed[0] * needed[1] * needed[2]),
                                Vec::new(),
                            ));
                        } else {
                            space_fail += 1;
                            report.push(assess(
                                format!("iso16757.2.5.3.5.clearance.{id}"),
                                "2",
                                "5.3.5",
                                subject(id, format!("geometry.objects.{id}.spaces"), id, id),
                                copy("Installation clearance", "Einbaufreiraum"),
                                copy(
                                    format!(
                                        "Installation space of '{id}' is {:.3}×{:.3}×{:.3} m but needs at least {:.3}×{:.3}×{:.3} m (product + {INSTALL_CLEARANCE_M} m clearance).",
                                        have[0], have[1], have[2], needed[0], needed[1], needed[2]
                                    ),
                                    format!(
                                        "Einbauraum von '{id}' ist {:.3}×{:.3}×{:.3} m, benötigt mindestens {:.3}×{:.3}×{:.3} m (Produkt + {INSTALL_CLEARANCE_M} m Freiraum).",
                                        have[0], have[1], have[2], needed[0], needed[1], needed[2]
                                    ),
                                ),
                                CheckStatus::Fail,
                                q_len(have[0].min(have[1]).min(have[2])),
                                q_len(needed[0].min(needed[1]).min(needed[2])),
                                {
                                    let space_id = install.id.clone();
                                    let mut remedies = Vec::new();
                                    for axis in 0..3 {
                                        let required_max = install.bounds.min[axis] + needed[axis];
                                        if install.bounds.max[axis] + 1e-9 < required_max {
                                            remedies.push(Remedy::at_least(
                                                subject(id, format!("geometry.objects.{id}.spaces[id={space_id}].bounds.max[{axis}]"), id, id),
                                                q_len(install.bounds.max[axis]),
                                                q_len(required_max),
                                                copy(
                                                    format!("Enlarge Installation space '{space_id}' of '{id}' axis {axis} max to ≥ {required_max:.3} m."),
                                                    format!("Installation-Raum '{space_id}' von '{id}' Achse {axis} max auf ≥ {required_max:.3} m vergrößern."),
                                                ),
                                            ));
                                        }
                                    }
                                    if remedies.is_empty() {
                                        for axis in 0..3 {
                                            let required_max = bbox.max[axis] + INSTALL_CLEARANCE_M;
                                            remedies.push(Remedy::at_least(
                                                subject(id, format!("geometry.objects.{id}.spaces[id={space_id}].bounds.max[{axis}]"), id, id),
                                                q_len(install.bounds.max[axis]),
                                                q_len(required_max),
                                                copy(
                                                    format!("Expand Installation max[{axis}] of '{id}' to ≥ {required_max:.3} m for {INSTALL_CLEARANCE_M} m inset."),
                                                    format!("Installation max[{axis}] von '{id}' auf ≥ {required_max:.3} m für {INSTALL_CLEARANCE_M} m Einrückung erweitern."),
                                                ),
                                            ));
                                        }
                                    }
                                    remedies
                                },
                            ));
                        }
                    }
                }
                Err(err) => {
                    volume_fail += 1;
                    report.push(fail(
                        format!("iso16757.2.7.1.bbox.{id}"),
                        "2",
                        "7.1",
                        subject(id, format!("geometry.objects.{id}.shape"), id, id),
                        copy("Bounding box evaluation", "Bounding-Box-Auswertung"),
                        copy(format!("Cannot evaluate bbox for '{id}': {err}"), format!("Bounding Box für '{id}' nicht auswertbar: {err}")),
                        vec![Remedy {
                            target: subject(id, format!("geometry.objects.{id}.shape"), id, id),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::Exactly,
                            options: vec!["box".into(), "cylinder".into(), "sphere".into()],
                            action: copy(
                                format!("Provide complete primitive parameters for '{id}' (box: width/height/depth > 0)."),
                                format!("Vollständige Primitivparameter für '{id}' angeben (box: width/height/depth > 0)."),
                            ),
                            applicable: true,
                        }],
                    ));
                }
            },
            None => {
                report.push(fail(
                    format!("iso16757.2.6.1.shape.{id}"),
                    "2",
                    "6.1",
                    subject(id, format!("geometry.objects.{id}.shape"), id, id),
                    copy("Geometry shape present", "Geometrieform vorhanden"),
                    copy(format!("Geometry object '{id}' has no shape."), format!("Geometrieobjekt '{id}' hat keine Form."),),
                    vec![Remedy {
                        target: subject(id, format!("geometry.objects.{id}.shape"), id, id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: Vec::new(),
                        action: copy(
                            format!("Assign a CSG shape (e.g. box primitive) to geometry '{id}'."),
                            format!("CSG-Form (z. B. Box-Primitiv) der Geometrie '{id}' zuweisen."),
                        ),
                        applicable: true,
                    }],
                ));
                graph_fail += 1;
            }
        }

        if obj.ports.is_empty() {
            port_fail += 1;
            report.push(fail(
                format!("iso16757.2.5.4.ports.{id}"),
                "2",
                "5.4",
                subject(id, format!("geometry.objects.{id}.ports"), id, id),
                copy("Connection ports", "Anschlusspunkte"),
                copy(
                    format!("Geometry '{id}' declares no connection ports."),
                    format!("Geometrie '{id}' deklariert keine Anschlusspunkte."),
                ),
                vec![Remedy {
                    target: subject(id, format!("geometry.objects.{id}.ports"), id, id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::AtLeast,
                    options: Vec::new(),
                    action: copy(
                        format!("Add at least one PortDefinition on '{id}' with non-zero direction and medium."),
                        format!("Mindestens eine PortDefinition an '{id}' mit nicht-nuller Richtung und Medium hinzufügen."),
                    ),
                    applicable: true,
                }],
            ));
        } else {
            for (pi, port) in obj.ports.iter().enumerate() {
                let dir_len = (port.direction[0].powi(2) + port.direction[1].powi(2) + port.direction[2].powi(2)).sqrt();
                if dir_len < 1e-9 || port.medium.trim().is_empty() || port.port_type.trim().is_empty() {
                    port_fail += 1;
                    report.push(fail(
                        format!("iso16757.2.5.4.port.{id}.{}", port.id),
                        "2",
                        "5.4",
                        subject(&port.id, format!("geometry.objects.{id}.ports[id={}]", port.id), &port.id, &port.id),
                        copy("Port direction and medium", "Port-Richtung und Medium"),
                        copy(
                            format!("Port '{}' on '{id}' needs non-zero direction, medium and portType.", port.id),
                            format!("Port '{}' an '{id}' braucht nicht-nulle Richtung, Medium und portType.", port.id),
                        ),
                        vec![Remedy {
                            target: subject(&port.id, format!("geometry.objects.{id}.ports[id={}].direction", port.id), &port.id, &port.id),
                            current: q_len(dir_len),
                            required: q_len(1.0),
                            bound: RemedyBound::AtLeast,
                            options: vec!["water".into(), "air".into(), "refrigerant".into()],
                            action: copy(
                                format!("Set port '{}' direction to a unit vector and provide medium/portType.", port.id),
                                format!("Richtung von Port '{}' auf Einheitsvektor setzen und medium/portType angeben.", port.id),
                            ),
                            applicable: true,
                        }],
                    ));
                }
            }
        }

        for (param, binding) in &obj.parameter_bindings {
            if binding.trim().is_empty() {
                binding_fail += 1;
                report.push(fail(
                    format!("iso16757.2.6.2.binding.{id}.{param}"),
                    "2",
                    "6.2",
                    subject(id, format!("geometry.objects.{id}.parameterBindings.{param}"), id, id),
                    copy("Geometry parameter binding", "Geometrie-Parameterbindung"),
                    copy(
                        format!("Parameter binding '{param}' on '{id}' is empty."),
                        format!("Parameterbindung '{param}' an '{id}' ist leer."),
                    ),
                    vec![Remedy {
                        target: subject(id, format!("geometry.objects.{id}.parameterBindings.{param}"), id, id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: doc.catalogue.property_definitions.iter().map(|d| d.id.clone()).collect(),
                        action: copy(
                            format!("Bind '{param}' to a catalogue property definition id."),
                            format!("'{param}' an eine Katalog-Eigenschaftsdefinitions-Id binden."),
                        ),
                        applicable: true,
                    }],
                ));
            } else if doc.catalogue.property_definitions.iter().all(|d| d.id != *binding) {
                binding_fail += 1;
                report.push(fail(
                    format!("iso16757.2.6.2.binding.{id}.{param}"),
                    "2",
                    "6.2",
                    subject(id, format!("geometry.objects.{id}.parameterBindings.{param}"), id, id),
                    copy("Geometry parameter binding", "Geometrie-Parameterbindung"),
                    copy(
                        format!("Parameter '{param}' on '{id}' binds to unknown property '{binding}'."),
                        format!("Parameter '{param}' an '{id}' bindet unbekannte Eigenschaft '{binding}'."),
                    ),
                    vec![Remedy::one_of(
                        subject(id, format!("geometry.objects.{id}.parameterBindings.{param}"), id, id),
                        doc.catalogue.property_definitions.iter().map(|d| d.id.clone()).collect(),
                        copy(
                            format!("Set binding of '{param}' to an existing property definition id."),
                            format!("Bindung von '{param}' auf eine vorhandene Eigenschaftsdefinitions-Id setzen."),
                        ),
                    )],
                ));
            }
        }
    }

    for (id, obj) in &geometry.objects {
        for surface in &obj.surfaces {
            let dx = surface.bounds.max[0] - surface.bounds.min[0];
            let dy = surface.bounds.max[1] - surface.bounds.min[1];
            let dz = surface.bounds.max[2] - surface.bounds.min[2];
            let ok = !surface.purpose.trim().is_empty() && (dx > 0.0 || dy > 0.0 || dz > 0.0);
            if ok {
                report.push(pass(
                    format!("iso16757.2.5.4.surface.{id}.{}", surface.id),
                    "2",
                    "5.4",
                    subject(&surface.id, format!("geometry.objects.{id}.surfaces[id={}]", surface.id), &surface.id, &surface.id),
                    copy("Semantic surface", "Semantische Fläche"),
                    copy(
                        format!("Surface '{}' on '{id}' declares purpose '{}' with positive extents.", surface.id, surface.purpose),
                        format!("Fläche '{}' an '{id}' deklariert Zweck '{}' mit positiven Abmessungen.", surface.id, surface.purpose),
                    ),
                ));
            } else {
                report.push(fail(
                    format!("iso16757.2.5.4.surface.{id}.{}", surface.id),
                    "2",
                    "5.4",
                    subject(&surface.id, format!("geometry.objects.{id}.surfaces[id={}].purpose", surface.id), &surface.id, &surface.id),
                    copy("Semantic surface", "Semantische Fläche"),
                    copy(
                        format!("Surface '{}' on '{id}' lacks purpose or positive extents.", surface.id),
                        format!("Fläche '{}' an '{id}' fehlt Zweck oder positive Abmessungen.", surface.id),
                    ),
                    vec![Remedy {
                        target: subject(&surface.id, format!("geometry.objects.{id}.surfaces[id={}].purpose", surface.id), &surface.id, &surface.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec!["installation".into(), "maintenance".into(), "connection".into()],
                        action: copy(
                            format!("Set purpose of surface '{}' to a semantic role and ensure bounds extents > 0.", surface.id),
                            format!("Zweck der Fläche '{}' auf eine semantische Rolle setzen und Bounds > 0 sicherstellen.", surface.id),
                        ),
                        applicable: true,
                    }],
                ));
            }
        }
    }

    for (id, obj) in &geometry.objects {
        if let Some(shape) = &obj.shape {
            let values: std::collections::HashMap<String, f64> = obj
                .parameter_bindings
                .iter()
                .filter_map(|(k, v)| v.parse::<f64>().ok().map(|n| (k.clone(), n)))
                .collect();
            let substituted = helpers::substitute_parameters(shape, &values);
            match helpers::evaluate_bounding_box(&substituted, geometry) {
                Ok(_) => {
                    report.push(pass(
                        format!("iso16757.2.6.2.substitute.{id}"),
                        "2",
                        "6.2",
                        subject(id, format!("geometry.objects.{id}.parameterBindings"), id, id),
                        copy("Parameter substitution", "Parameterersetzung"),
                        copy(
                            format!("Parameter substitution for '{id}' yields an evaluable geometry graph."),
                            format!("Parameterersetzung für '{id}' liefert einen auswertbaren Geometriegraphen."),
                        ),
                    ));
                }
                Err(err) => {
                    report.push(fail(
                        format!("iso16757.2.6.2.substitute.{id}"),
                        "2",
                        "6.2",
                        subject(id, format!("geometry.objects.{id}.parameterBindings"), id, id),
                        copy("Parameter substitution", "Parameterersetzung"),
                        copy(
                            format!("Parameter substitution for '{id}' failed: {err}"),
                            format!("Parameterersetzung für '{id}' fehlgeschlagen: {err}"),
                        ),
                        vec![Remedy {
                            target: subject(id, format!("geometry.objects.{id}.parameterBindings"), id, id),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::Exactly,
                            options: values.keys().cloned().collect(),
                            action: copy(
                                format!("Complete numeric parameterBindings for '{id}' so substitute_parameters resolves all placeholders."),
                                format!("Numerische parameterBindings für '{id}' vervollständigen, damit substitute_parameters alle Platzhalter auflöst."),
                            ),
                            applicable: true,
                        }],
                    ));
                }
            }
        }
    }

    if graph_fail == 0 && volume_fail == 0 && port_fail == 0 && space_fail == 0 && binding_fail == 0 {
        let _ = (objects_ok, catalogue_subject);
    }
}

/// 📐️ Positive extent helper used by tests/oracles.

pub fn solid_volume_m3(obj: &GeometryObject, catalogue: &GeometryCatalogue) -> Option<f64> {
    obj.shape.as_ref().and_then(|s| helpers::evaluate_bounding_box(s, catalogue).ok().map(|b| b.volume_m3()))
}

pub fn has_reference_cycle(node: &GeometryNode, catalogue: &GeometryCatalogue, stack: &mut HashSet<String>) -> bool {
    match node {
        GeometryNode::Reference { geometry_id } => {
            if !stack.insert(geometry_id.clone()) {
                return true;
            }
            let cyclic = catalogue.objects.get(geometry_id).and_then(|o| o.shape.as_ref()).map(|s| has_reference_cycle(s, catalogue, stack)).unwrap_or(false);
            stack.remove(geometry_id);
            cyclic
        }
        GeometryNode::Transform { child, .. } => has_reference_cycle(child, catalogue, stack),
        GeometryNode::Boolean { children, .. } => children.iter().any(|c| has_reference_cycle(c, catalogue, stack)),
        GeometryNode::Primitive { .. } => false,
    }
}

/// 📐️ Part 2 §5.3 / §6.1 — space envelopes, ports, and primitive registry completeness.
fn check_ports_primitives_and_spaces(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    const EXPECTED: &[(&str, &[&str])] = &[
        ("box", &["width", "height", "depth"]),
        ("cylinder", &["radius", "height"]),
        ("sphere", &["radius"]),
    ];
    for (id, obj) in &doc.geometry.objects {
        // Map key must equal object.id (geometry catalogue integrity, Part 2 §5.1).
        if obj.id != *id {
            report.push(fail(
                format!("iso16757.2.5.1.objectId.{id}"),
                "2",
                "5.1",
                subject(id, format!("geometry.objects.{id}.id"), id, id),
                copy("Geometry object id", "Geometrieobjekt-Id"),
                copy(
                    format!("Geometry map key '{id}' does not match object.id '{}'.", obj.id),
                    format!("Geometrie-Map-Schlüssel '{id}' stimmt nicht mit object.id '{}' überein.", obj.id),
                ),
                vec![Remedy {
                    target: subject(id, format!("geometry.objects.{id}.id"), id, id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec![id.clone()],
                    action: copy(format!("Set geometry.objects.{id}.id to '{id}'."), format!("geometry.objects.{id}.id auf '{id}' setzen.")),
                    applicable: true,
                }],
            ));
        } else {
            report.push(assess(
                format!("iso16757.2.5.1.objectId.{id}"),
                "2",
                "5.1",
                subject(id, format!("geometry.objects.{id}.id"), id, id),
                copy("Geometry object id", "Geometrieobjekt-Id"),
                copy(
                    format!("Geometry object '{id}' id matches its catalogue key."),
                    format!("Geometrieobjekt '{id}' Id stimmt mit dem Katalogschlüssel überein."),
                ),
                CheckStatus::Pass,
                q_dim(1.0),
                q_dim(1.0),
                Vec::new(),
            ));
        }
        let mut space_ids = HashSet::new();
        for space in &obj.spaces {
            if !space_ids.insert(space.id.clone()) {
                report.push(fail(
                    format!("iso16757.2.5.3.5.spaceId.{id}.{}", space.id),
                    "2",
                    "5.3.5",
                    subject(id, format!("geometry.objects.{id}.spaces[id={}].id", space.id), id, id),
                    copy("Space id uniqueness", "Raum-Id-Eindeutigkeit"),
                    copy(
                        format!("Duplicate space id '{}' on geometry '{id}'.", space.id),
                        format!("Doppelte Raum-Id '{}' an Geometrie '{id}'.", space.id),
                    ),
                    vec![Remedy {
                        target: subject(id, format!("geometry.objects.{id}.spaces[id={}].id", space.id), id, id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec![format!("{}-b", space.id)],
                        action: copy("Rename the duplicate space id.", "Doppelte Raum-Id umbenennen."),
                        applicable: true,
                    }],
                ));
            }
            let vol = (space.bounds.max[0] - space.bounds.min[0])
                * (space.bounds.max[1] - space.bounds.min[1])
                * (space.bounds.max[2] - space.bounds.min[2]);
            if vol <= 0.0 {
                report.push(fail(
                    format!("iso16757.2.5.3.5.spaceVolume.{id}.{}", space.id),
                    "2",
                    "5.3.5",
                    subject(id, format!("geometry.objects.{id}.spaces[id={}].bounds.max", space.id), id, id),
                    copy("Space envelope volume", "Raumhüllvolumen"),
                    copy(
                        format!("Space '{}' on '{id}' has non-positive envelope volume {vol:.6} m³.", space.id),
                        format!("Raum '{}' an '{id}' hat nicht-positives Hüllvolumen {vol:.6} m³.", space.id),
                    ),
                    vec![Remedy::at_least(
                        subject(id, format!("geometry.objects.{id}.spaces[id={}].bounds.max[0]", space.id), id, id),
                        q_len(space.bounds.max[0]),
                        q_len(space.bounds.min[0] + 0.01),
                        copy("Enlarge space max bounds so envelope volume > 0.", "Raum-max-Grenzen vergrößern, sodass Hüllvolumen > 0."),
                    )],
                ));
            } else {
                report.push(assess(
                    format!("iso16757.2.5.3.5.spaceVolume.{id}.{}", space.id),
                    "2",
                    "5.3.5",
                    subject(id, format!("geometry.objects.{id}.spaces[id={}].bounds.max", space.id), id, id),
                    copy("Space envelope volume", "Raumhüllvolumen"),
                    copy(
                        format!("Space '{}' on '{id}' envelope volume is {vol:.6} m³.", space.id),
                        format!("Raum '{}' an '{id}' Hüllvolumen ist {vol:.6} m³.", space.id),
                    ),
                    CheckStatus::Pass,
                    q_vol(vol),
                    q_vol(vol),
                    Vec::new(),
                ));
            }
        }
        for port in &obj.ports {
            let dir_n = (port.direction[0].powi(2) + port.direction[1].powi(2) + port.direction[2].powi(2)).sqrt();
            let ok_dir = (dir_n - 1.0).abs() < 1e-3;
            if !ok_dir {
                report.push(fail(
                    format!("iso16757.2.6.1.portDir.{id}.{}", port.id),
                    "2",
                    "6.1",
                    subject(id, format!("geometry.objects.{id}.ports[id={}].direction", port.id), id, id),
                    copy("Port direction unit vector", "Anschluss-Richtungs-Einheitsvektor"),
                    copy(
                        format!("Port '{}' direction magnitude is {dir_n:.4}, expected 1.0.", port.id),
                        format!("Anschluss '{}' Richtungbetrag ist {dir_n:.4}, erwartet 1.0.", port.id),
                    ),
                    vec![Remedy {
                        target: subject(id, format!("geometry.objects.{id}.ports[id={}].direction[0]", port.id), id, id),
                        current: q_dim(dir_n),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: vec![],
                        action: copy("Normalize port.direction to a unit vector.", "port.direction auf einen Einheitsvektor normalisieren."),
                        applicable: true,
                    }],
                ));
            } else {
                report.push(assess(
                    format!("iso16757.2.6.1.portDir.{id}.{}", port.id),
                    "2",
                    "6.1",
                    subject(id, format!("geometry.objects.{id}.ports[id={}].direction", port.id), id, id),
                    copy("Port direction unit vector", "Anschluss-Richtungs-Einheitsvektor"),
                    copy(
                        format!("Port '{}' direction is a unit vector (|dir|={dir_n:.4}).", port.id),
                        format!("Anschluss '{}' Richtung ist ein Einheitsvektor (|dir|={dir_n:.4}).", port.id),
                    ),
                    CheckStatus::Pass,
                    q_dim(dir_n),
                    q_dim(1.0),
                    Vec::new(),
                ));
            }
            if port.medium.trim().is_empty() || port.port_type.trim().is_empty() {
                report.push(fail(
                    format!("iso16757.2.6.1.portMeta.{id}.{}", port.id),
                    "2",
                    "6.1",
                    subject(id, format!("geometry.objects.{id}.ports[id={}].medium", port.id), id, id),
                    copy("Port medium and type", "Anschluss-Medium und -Art"),
                    copy(
                        format!("Port '{}' requires non-empty medium and portType.", port.id),
                        format!("Anschluss '{}' erfordert nicht-leeres Medium und portType.", port.id),
                    ),
                    vec![Remedy {
                        target: subject(id, format!("geometry.objects.{id}.ports[id={}].medium", port.id), id, id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec!["water".into()],
                        action: copy("Set port.medium and portType to Part 2 connection metadata.", "port.medium und portType auf Teil-2-Anschlussmetadaten setzen."),
                        applicable: true,
                    }],
                ));
            } else {
                let pos_n = (port.position[0].powi(2) + port.position[1].powi(2) + port.position[2].powi(2)).sqrt();
                report.push(assess(
                    format!("iso16757.2.6.1.portMeta.{id}.{}", port.id),
                    "2",
                    "6.1",
                    subject(id, format!("geometry.objects.{id}.ports[id={}].position", port.id), id, id),
                    copy("Port placement", "Anschlussplatzierung"),
                    copy(
                        format!("Port '{}' medium '{}' type '{}' |pos|={pos_n:.4} m.", port.id, port.medium, port.port_type),
                        format!("Anschluss '{}' Medium '{}' Typ '{}' |pos|={pos_n:.4} m.", port.id, port.medium, port.port_type),
                    ),
                    CheckStatus::Pass,
                    q_len(pos_n),
                    q_len(pos_n),
                    Vec::new(),
                ));
            }
        }
    }
    for prim in &doc.geometry.primitive_registry {
        let expected = EXPECTED.iter().find(|(k, _)| *k == prim.id.as_str()).map(|(_, p)| *p);
        match expected {
            Some(params) => {
                let ok = prim.parameters.iter().map(|s| s.as_str()).collect::<HashSet<_>>() == params.iter().copied().collect::<HashSet<_>>();
                if ok {
                    report.push(assess(
                        format!("iso16757.2.5.1.primitive.{}", prim.id),
                        "2",
                        "5.1",
                        subject(&prim.id, format!("geometry.primitiveRegistry[id={}].parameters", prim.id), &prim.id, &prim.id),
                        copy("Primitive parameter set", "Primitiv-Parametersatz"),
                        copy(
                            format!("Primitive '{}' declares Part 2 parameters {:?}.", prim.id, prim.parameters),
                            format!("Primitiv '{}' deklariert Teil-2-Parameter {:?}.", prim.id, prim.parameters),
                        ),
                        CheckStatus::Pass,
                        q_dim(params.len() as f64),
                        q_dim(params.len() as f64),
                        Vec::new(),
                    ));
                } else {
                    report.push(fail(
                        format!("iso16757.2.5.1.primitive.{}", prim.id),
                        "2",
                        "5.1",
                        subject(&prim.id, format!("geometry.primitiveRegistry[id={}].parameters", prim.id), &prim.id, &prim.id),
                        copy("Primitive parameter set", "Primitiv-Parametersatz"),
                        copy(
                            format!("Primitive '{}' parameters {:?} must equal {:?}.", prim.id, prim.parameters, params),
                            format!("Primitiv '{}' Parameter {:?} müssen {:?} entsprechen.", prim.id, prim.parameters, params),
                        ),
                        vec![Remedy {
                            target: subject(&prim.id, format!("geometry.primitiveRegistry[id={}].parameters[0]", prim.id), &prim.id, &prim.id),
                            current: q_dim(prim.parameters.len() as f64),
                            required: q_dim(params.len() as f64),
                            bound: RemedyBound::OneOf,
                            options: params.iter().map(|s| s.to_string()).collect(),
                            action: copy("Restore Part 2 primitive parameter names.", "Teil-2-Primitiv-Parameternamen wiederherstellen."),
                            applicable: true,
                        }],
                    ));
                }
            }
            None => {
                report.push(fail(
                    format!("iso16757.2.5.1.primitive.{}", prim.id),
                    "2",
                    "5.1",
                    subject(&prim.id, format!("geometry.primitiveRegistry[id={}].id", prim.id), &prim.id, &prim.id),
                    copy("Primitive kind", "Primitivart"),
                    copy(
                        format!("Primitive id '{}' is not a Part 2 solid (box|cylinder|sphere).", prim.id),
                        format!("Primitiv-Id '{}' ist kein Teil-2-Körper (box|cylinder|sphere).", prim.id),
                    ),
                    vec![Remedy {
                        target: subject(&prim.id, format!("geometry.primitiveRegistry[id={}].id", prim.id), &prim.id, &prim.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec!["box".into(), "cylinder".into(), "sphere".into()],
                        action: copy("Use a Part 2 primitive id.", "Eine Teil-2-Primitiv-Id verwenden."),
                        applicable: true,
                    }],
                ));
            }
        }
    }
}
