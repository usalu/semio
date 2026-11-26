//! 🌐️ The canonical gbXML element grammar over borrowed original model rows.

use super::*;
use GbxmlTextPart::{Label as TextLabel, Text};

#[derive(Clone, Copy)]
struct Polygon { source: SurfaceAddress, bearing: f64 }

#[derive(Clone, Copy)]
struct Geometry { azimuth: f64, tilt: f64, corner: P3, coordinates: usize, height: f64, width: f64 }

#[derive(Clone, Copy)]
enum Node<'a> {
    Root, Campus(usize), Location(usize), Building(usize), Storey(usize), Space(usize), Boundary(usize, Polygon),
    Surface(usize), Opening(usize, usize), Window(usize), Construction(usize), Door(usize), Schematic,
    Layer(usize), Material(usize), Zone(usize),
    Planar(Polygon), Loop(Polygon), Rectangular(Geometry), Point(P3, usize),
    Reference(&'static str, &'static str, Id, usize), LayerReference(usize),
    Label(&'static str, &'a str), Joined(&'static str, &'a str, &'static str, &'a str),
    Number(&'static str, Option<&'static str>, f64), SpaceDescription(usize), ZoneName(usize), ZoneCad(usize),
    ConductanceName(&'a str, bool, f64), MaterialName(&'a str, f64), FrameDescription(bool, Option<f64>),
    GlassCad(&'static str, &'a str), Solar(f64),
}

const ABSENT: &str = "gbXML original document source disappeared";

impl<'a> View<'a> {
    fn window(&self, rank: usize) -> Option<(Id, usize, &'a str, Option<f64>, Option<f64>, Option<f64>)> {
        let count = self.model.curtain_wall_types.len();
        if rank < count {
            let (_, row) = self.model.curtain_wall_types.get_index(rank)?;
            Some((Id::Curtain, rank, &row.name, row.frame_fraction, row.u_value, row.g_value))
        } else {
            let index = rank - count;
            let (_, row) = self.model.window_types.get_index(index)?;
            Some((Id::OpeningType, self.plan.used.window(index)?, &row.name, row.frame_fraction, row.u_value, row.g_value))
        }
    }

    fn space_label(&self, room: usize) -> Option<(&'a str, &'static str, &'a str)> {
        let (_, space) = self.model.spaces.get_index(room)?;
        let (first, last) = (space.number.trim_start(), space.name.trim_end());
        Some(if first.is_empty() { ("", "", last.trim_start()) }
            else if last.is_empty() { (first.trim_end(), "", "") }
            else { (first, " ", last) })
    }

    fn child<'s>(&'s self, parent: Node<'s>, slot: usize) -> Option<Node<'s>> {
        use Node::*;
        let model = self.model;
        let plan = self.plan;
        Some(match parent {
            Root => {
                let mut index = slot;
                if index < model.sites.len() { return Some(Campus(index)); }
                index -= model.sites.len();
                if index < plan.windows.len() { return Some(Window(plan.windows.get(index)?)); }
                index -= plan.windows.len();
                if index < plan.constructions.len() {
                    return Some(if index < plan.constructions.opaque_len() { Construction(index) } else { Schematic });
                }
                index -= plan.constructions.len();
                if index < model.door_types.len() {
                    return plan.used.door(index).map(|_| Door(index));
                }
                index -= model.door_types.len();
                if index < plan.materials.len() { return Some(Layer(self.layers?.get(index)?)); }
                index -= plan.materials.len();
                if index < plan.materials.len() { return Some(Material(plan.materials.sorted_at(index)?)); }
                Zone(index - plan.materials.len())
            }
            Campus(rank) => {
                let (id, site) = model.sites.get_index(rank)?;
                match slot {
                    0 => Label("Name", &site.name),
                    1 => Location(rank),
                    slot if slot - 2 < model.buildings.len() => {
                        let building = slot - 2;
                        if &model.buildings.get_index(building)?.1.site != id { return None; }
                        Building(building)
                    }
                    slot => {
                        let ordinal = slot - 2 - model.buildings.len();
                        let row = plan.surfaces.get(ordinal)?;
                        let (_, storey) = room_parts(model, self.inferred, row.surface.room)?;
                        if &model.buildings.get(&storey.building)?.site != id { return None; }
                        Surface(ordinal)
                    }
                }
            }
            Location(rank) => {
                let (_, site) = model.sites.get_index(rank)?;
                match slot {
                    0 => Label("ZipcodeOrPostalCode", ""), 1 => Number("Longitude", None, site.longitude),
                    2 => Number("Latitude", None, site.latitude), 3 => Number("Elevation", None, site.elevation),
                    4 => Number("CADModelAzimuth", None, 0.0), _ => Label("Name", &site.name),
                }
            }
            Building(rank) => {
                let (id, row) = model.buildings.get_index(rank)?;
                match slot {
                    0 => Label("Name", &row.name),
                    1 => Number("Area", None, plan.areas.get(rank)?),
                    slot if slot - 2 < plan.storeys.len() => {
                        let storey = plan.storeys.get(slot - 2)?;
                        if &model.storeys.get_index(storey)?.1.building != id { return None; }
                        Storey(storey)
                    }
                    slot => {
                        let room = slot - 2 - plan.storeys.len();
                        if &room_parts(model, self.inferred, room)?.1.building != id { return None; }
                        Space(room)
                    }
                }
            }
            Storey(rank) => {
                let (id, row) = model.storeys.get_index(rank)?;
                if slot == 0 { Label("Name", &row.name) }
                else { Number("Level", None, round9(self.inferred.storey_levels.get(id).map_or(0.0, |level| level.elevation))) }
            }
            Space(rank) => {
                let (id, room) = model.spaces.get_index(rank)?;
                let (env, _) = room_parts(model, self.inferred, rank)?;
                let conditions = model.space_conditions.get(id);
                match slot {
                    0 => { let (first, space, last) = self.space_label(rank)?; Joined("Name", first, space, last) }
                    1 => {
                        if !conditions.is_some_and(|row| row.occupancy.is_some() || row.schedule.is_some()) { return None; }
                        SpaceDescription(rank)
                    }
                    2 => Number("Area", None, round9(env.floor_area)),
                    3 => Number("Volume", None, round9(env.volume)),
                    4 => {
                        let density = conditions.and_then(|row| row.occupancy_density).or_else(|| {
                            room.zone.as_ref().and_then(|zone| model.zones.get(zone)).map(|zone| zone.occupancy_density).filter(|density| *density > 0.0)
                        })?;
                        Number("PeopleNumber", Some("NumberOfPeople"), round9(density * env.floor_area))
                    }
                    5 => Number("LightPowerPerArea", Some("WattPerSquareMeter"), conditions?.lighting_power_density?),
                    6 => Number("EquipPowerPerArea", Some("WattPerSquareMeter"), conditions?.equipment_power_density?),
                    7 => Label("CADObjectId", id),
                    slot => {
                        let ordinal = slot - 8;
                        let row = plan.surfaces.get(ordinal)?;
                        let source = if row.surface.room == rank { row.surface }
                            else { row.other.filter(|other| other.room == rank)? };
                        Boundary(ordinal, Polygon { source, bearing: row.bearing })
                    }
                }
            }
            Boundary(_, polygon) => Planar(polygon),
            Surface(index) => {
                let row = plan.surfaces.get(index)?;
                let side = surface(model, self.inferred, row.surface)?;
                let range = plan.openings.range(index)?;
                match slot {
                    0 => Label("Name", &side.id),
                    1 => { room_parts(model, self.inferred, row.surface.room)?; Reference("AdjacentSpaceId", "spaceIdRef", Id::Space, row.surface.room) }
                    2 => {
                        let other = row.other.map(|other| other.room).or_else(|| {
                            (!side.adjacent.is_empty()).then(|| index_of(&model.spaces, &side.adjacent)).flatten()
                        }).filter(|&other| other != row.surface.room)?;
                        room_parts(model, self.inferred, other)?;
                        Reference("AdjacentSpaceId", "spaceIdRef", Id::Space, other)
                    }
                    3 => Rectangular(Geometry { azimuth: row.azimuth, tilt: row.tilt, corner: row.corner, coordinates: 3, height: row.height, width: row.width }),
                    4 => Planar(Polygon { source: row.surface, bearing: row.bearing }),
                    slot if slot - 5 < range.count => Opening(index, range.first + slot - 5),
                    slot if slot - 5 == range.count => Label("CADObjectId", &side.id),
                    _ => Label("CADObjectId", &surface(model, self.inferred, row.other?)?.id),
                }
            }
            Opening(host, ordinal) => {
                let parent = plan.surfaces.get(host)?;
                let row = plan.openings.row(ordinal)?;
                let child = surface(model, self.inferred, row.address)?;
                match slot {
                    0 if row.glazing => GlassCad("Name", &child.id),
                    3 if row.glazing => GlassCad("CADObjectId", &child.id),
                    0 => Label("Name", &child.element),
                    1 => Rectangular(Geometry { azimuth: parent.azimuth, tilt: parent.tilt, corner: [row.offset[0], row.offset[1], 0.0], coordinates: 2, height: row.height, width: row.width }),
                    2 => Planar(Polygon { source: row.address, bearing: parent.bearing }),
                    _ => Label("CADObjectId", &child.element),
                }
            }
            Rectangular(row) => match slot {
                0 => Number("Azimuth", None, row.azimuth), 1 => Point(row.corner, row.coordinates),
                2 => Number("Tilt", None, row.tilt), 3 => Number("Height", None, row.height), _ => Number("Width", None, row.width),
            },
            Planar(polygon) => Loop(polygon),
            Loop(polygon) => Point(turned(surface(model, self.inferred, polygon.source)?.polygon.get(slot)?, polygon.bearing), 3),
            Point(point, _) => Number("Coordinate", None, *point.get(slot)?),
            Window(rank) => {
                let (kind, _, label, fraction, u, g) = self.window(rank)?;
                let curtain = matches!(kind, Id::Curtain);
                match slot {
                    0 => Label("Name", label),
                    1 if curtain || fraction.is_some() => FrameDescription(curtain, fraction),
                    1 => return None,
                    2 => Number("U-value", Some("WPerSquareMeterK"), round9(u?)),
                    _ => Solar(round9(g?)),
                }
            }
            Construction(index) => {
                let (_, label, u) = self.opaque(index)?;
                match slot {
                    0 => ConductanceName(label.unwrap_or_else(|kind| kind), label.is_err(), u),
                    1 => Number("U-value", Some("WPerSquareMeterK"), u),
                    _ => LayerReference(plan.materials.reference(index, slot - 2)?),
                }
            }
            Door(rank) => {
                let (_, row) = model.door_types.get_index(rank)?;
                let u = round9(row.u_value?);
                if slot == 0 { ConductanceName(&row.name, false, u) }
                else { Number("U-value", Some("WPerSquareMeterK"), u) }
            }
            Schematic => Label("Name", "Unspecified construction"),
            Layer(index) => Reference("MaterialId", "materialIdRef", Id::Material, index),
            Material(index) => {
                let layer = self.material(index)?;
                let row = model.materials.get(&layer.material)?;
                match slot {
                    0 => MaterialName(&row.name, layer.thickness),
                    1 => Number("R-value", Some("SquareMeterKPerW"), round9(layer.thickness / row.conductivity)),
                    2 => Number("Thickness", Some("Meters"), round9(layer.thickness)),
                    3 => Number("Conductivity", Some("WPerMeterK"), row.conductivity),
                    4 => Number("Density", Some("KgPerCubicM"), row.density),
                    _ => Number("SpecificHeat", Some("JPerKgK"), row.specific_heat),
                }
            }
            Zone(index) => {
                let row = plan.zones.get(model, self.inferred, index)?;
                match slot {
                    0 => ZoneName(index),
                    1 => Number("OAFlowPerArea", Some("LPerSecPerSquareM"), row.outdoor_air?),
                    2 => Number("DesignHeatT", Some("C"), row.heating?),
                    3 => Number("DesignCoolT", Some("C"), row.cooling?),
                    _ => ZoneCad(index),
                }
            }
            _ => return None,
        })
    }

    fn node(&self, path: &[usize]) -> Option<Node<'_>> {
        path.iter().try_fold(Node::Root, |parent, &slot| self.child(parent,slot))
    }
}

fn attribute<'a>(name: &'a str, value: &'a str) -> GbxmlAttribute<'a> {
    GbxmlAttribute { name, value: GbxmlText::Text(value) }
}

fn reference<'a>(name: &'a str, value: GbxmlText<'a>) -> GbxmlAttribute<'a> {
    GbxmlAttribute { name, value }
}

fn branch<'a>(name: &'a str, attributes: &'a [GbxmlAttribute<'a>], count: usize, sparse: bool) -> GbxmlElement<'a> {
    let contents = if count == 0 { GbxmlContent::Empty }
        else if sparse { GbxmlContent::Slots(count) }
        else { GbxmlContent::Elements(count) };
    GbxmlElement { name, attributes, contents }
}

fn text<'a>(name: &'a str, attributes: &'a [GbxmlAttribute<'a>], value: GbxmlText<'a>) -> GbxmlElement<'a> {
    GbxmlElement { name, attributes, contents: GbxmlContent::Text(value) }
}

fn label<'a>(name: &'a str, value: &'a str, read: impl FnOnce(GbxmlElement<'_>) -> ()) {
    read(text(name, &[], GbxmlText::Parts(&[TextLabel(value)])));
}

impl View<'_> {
    fn with_layer_id<R>(&self, index: usize, read: impl FnOnce(GbxmlText<'_>) -> R) -> Option<R> {
        self.with_id(Id::Material, index, |name| {
            let GbxmlText::Parts(name) = name else { unreachable!() };
            let mut parts = [Text(""); 10];
            parts[0] = Text("layer-");
            parts[1..=name.len()].copy_from_slice(name);
            read(GbxmlText::Parts(&parts[..name.len() + 1]))
        })
    }

    fn render(&self, node: Node<'_>, mut read: impl FnMut(GbxmlElement<'_>)) -> Option<()> {
        use Node::*;
        let model = self.model;
        let plan = self.plan;
        macro_rules! id_branch {
            ($kind:expr, $rank:expr, $tag:expr, $count:expr, $sparse:expr $(, $attr:expr)*) => {
                self.with_id($kind, $rank, |id| read(branch($tag, &[reference("id", id) $(, $attr)*], $count, $sparse)))?
            };
        }
        match node {
            Root => {
                let count = model.sites.len() + plan.windows.len() + plan.constructions.len() + model.door_types.len() + 2 * plan.materials.len() + plan.zones.len();
                read(branch("gbXML", &[
                    attribute("xmlns", NAMESPACE), attribute("version", VERSION), attribute("useSIUnitsForResults", "true"),
                    attribute("temperatureUnit", "C"), attribute("lengthUnit", "Meters"), attribute("areaUnit", "SquareMeters"), attribute("volumeUnit", "CubicMeters"),
                ], count, true));
            }
            Campus(rank) => id_branch!(Id::Campus, rank, "Campus", 2 + model.buildings.len() + plan.surfaces.extent(), true),
            Building(rank) => id_branch!(Id::Building, rank, "Building", 2 + plan.storeys.len() + model.spaces.len(), true, attribute("buildingType", "Unknown")),
            Storey(rank) => id_branch!(Id::Storey, rank, "BuildingStorey", 2, false),
            Location(_) => read(branch("Location", &[], 6, false)),
            Space(rank) => {
                let (room, _) = room_parts(model, self.inferred, rank)?;
                let (room_id, _) = model.spaces.get_index(rank)?;
                let storey = index_of(&model.storeys, &room.storey)?;
                let condition = model.space_conditions.get(room_id);
                let condition = attribute("conditionType", condition_type(condition.and_then(|row| row.heating_setpoint), condition.and_then(|row| row.cooling_setpoint)));
                self.with_id(Id::Space, rank, |id| self.with_id(Id::Storey, storey, |storey| {
                    let tail = reference("buildingStoreyIdRef", storey);
                    let id = reference("id", id);
                    let count = 8 + plan.surfaces.extent();
                    if let Some(zone) = plan.zones.member(rank) {
                        self.with_id(Id::Zone, zone, |zone| read(branch("Space", &[id, reference("zoneIdRef", zone), condition, tail], count, true)))
                    } else {
                        read(branch("Space", &[id, condition, tail], count, true));
                        Some(())
                    }
                })).flatten().flatten()?;
            }
            Boundary(index, _) => self.with_id(Id::Surface, index, |id| read(branch("SpaceBoundary", &[reference("surfaceIdRef", id)], 1, false)))?,
            Surface(index) => {
                let row = plan.surfaces.get(index)?;
                let side = surface(model, self.inferred, row.surface)?;
                let kind = surface_type(side);
                let construction = plan.constructions.reference(plan.surfaces.ordinal(row.surface)?)?;
                let (group, construction) = if construction == plan.constructions.opaque_len() { (Id::Schematic, 0) } else { (Id::Opaque, construction) };
                let children = 6 + plan.openings.range(index)?.count + usize::from(row.other.is_some());
                self.with_id(Id::Surface, index, |id| self.with_id(group, construction, |construction| {
                    read(branch("Surface", &[
                        reference("id", id), attribute("surfaceType", kind), reference("constructionIdRef", construction),
                        attribute("exposedToSun", if exposed_to_sun(kind) { "true" } else { "false" }),
                    ], children, true));
                })).flatten()?;
            }
            Opening(_, index) => {
                let row = plan.openings.row(index)?;
                let child = surface(model, self.inferred, row.address)?;
                let kind = if row.glazing { "FixedWindow" } else { opening_type(child.kind) };
                let linked = if row.glazing {
                    Some((Id::Curtain, index_of(&model.curtain_wall_types, &child.construction)?, "windowTypeIdRef"))
                } else {
                    match model.openings.get(&child.element).map(|row| &row.kind) {
                        Some(OpeningKind::Window { window_type }) => curtain_collision(model, &plan.used, window_type).map(|rank| (Id::Curtain, rank, "windowTypeIdRef"))
                            .or_else(|| index_of(&model.window_types, window_type).and_then(|index| plan.used.window(index)).map(|rank| (Id::OpeningType, rank, "windowTypeIdRef"))),
                        Some(OpeningKind::Door { door_type }) => index_of(&model.door_types, door_type).and_then(|index| plan.used.door(index)).map(|rank| (Id::OpeningType, rank, "constructionIdRef")),
                        _ => None,
                    }
                };
                self.with_id(Id::Opening, index, |id| {
                    let attrs = [reference("id", id), attribute("openingType", kind)];
                    if let Some((family, rank, reference_name)) = linked {
                        self.with_id(family, rank, |kind| read(branch("Opening", &[attrs[0], attrs[1], reference(reference_name, kind)], 4, false)))
                    } else {
                        read(branch("Opening", &attrs, 4, false));
                        Some(())
                    }
                }).flatten()?;
            }
            Window(rank) => {
                let (kind, index, ..) = self.window(rank)?;
                id_branch!(kind, index, "WindowType", 4, true);
            }
            Construction(index) => id_branch!(Id::Opaque, index, "Construction", 2 + layers(model, self.inferred, &plan.constructions, index).len(), true),
            Door(index) => id_branch!(Id::OpeningType, plan.used.door(index)?, "Construction", 2, false),
            Schematic => id_branch!(Id::Schematic, 0, "Construction", 1, false, attribute("constructionIsSchematic", "true")),
            Layer(index) => self.with_layer_id(index, |id| read(branch("Layer", &[reference("id", id)], 1, false)))?,
            Material(index) => id_branch!(Id::Material, index, "Material", 6, false),
            Zone(index) => id_branch!(Id::Zone, index, "Zone", 5, true),
            Rectangular(_) => read(branch("RectangularGeometry", &[], 5, false)),
            Planar(_) => read(branch("PlanarGeometry", &[], 1, false)),
            Loop(polygon) => read(branch("PolyLoop", &[], surface(model, self.inferred, polygon.source)?.polygon.len(), false)),
            Point(_, count) => read(branch("CartesianPoint", &[], count, false)),
            Reference(element, attr, family, index) => self.with_id(family, index, |id| read(branch(element, &[reference(attr, id)], 0, false)))?,
            LayerReference(index) => self.with_layer_id(index, |id| read(branch("LayerId", &[reference("layerIdRef", id)], 0, false)))?,
            Label(name, value) => label(name, value, &mut read),
            Joined(name, first, space, last) => read(text(name, &[], GbxmlText::Parts(&[TextLabel(first), Text(space), TextLabel(last)]))),
            Number(name, unit, number) => {
                let attrs = [attribute("unit", unit.unwrap_or_default())];
                with_number_text(number, |number| read(text(name, &attrs[..usize::from(unit.is_some())], GbxmlText::Text(number))));
            }
            SpaceDescription(index) => {
                let row = model.space_conditions.get(model.spaces.get_index(index)?.0)?;
                let occupancy = row.occupancy.as_deref();
                let schedule = row.schedule.as_deref();
                let parts = [Text(if occupancy.is_some() { "occupancy: " } else { "" }), TextLabel(occupancy.unwrap_or_default()),
                    Text(if occupancy.is_some() && schedule.is_some() { "; " } else { "" }),
                    Text(if schedule.is_some() { "schedule: " } else { "" }), TextLabel(schedule.unwrap_or_default())];
                read(text("Description", &[], GbxmlText::Parts(&parts)));
            }
            ZoneName(index) => {
                let zone = plan.zones.get(model, self.inferred, index)?;
                if let Some(row) = model.zones.get(zone.owner).filter(|_| !zone.synthetic) {
                    if let Some(variant) = zone.owner_variant {
                        with_count(variant, |variant| read(text("Name", &[], GbxmlText::Parts(&[TextLabel(&row.name), Text(" ("), Text(variant), Text(")")]))));
                    } else {
                        label("Name", &row.name, &mut read);
                    }
                } else {
                    let (first, space, last) = self.space_label(zone.first_room)?;
                    read(text("Name", &[], GbxmlText::Parts(&[TextLabel(first), Text(space), TextLabel(last)])));
                }
            }
            ZoneCad(index) => self.with_zone_cad(index, |cad| {
                let mut parts = [Text(""); 3];
                for (index, part) in cad.iter().enumerate() { parts[index] = match part { Text(raw) => TextLabel(raw), _ => *part }; }
                read(text("CADObjectId", &[], GbxmlText::Parts(&parts[..cad.len()])));
            })?,
            ConductanceName(name, fallback, u) => with_number_text(u, |u| {
                let detail = if fallback { " surface U=" } else { " U=" };
                read(text("Name", &[], GbxmlText::Parts(&[TextLabel(name), Text(detail), Text(u)])));
            }),
            MaterialName(name, thickness) => with_number_text(round9(thickness * 1000.0), |number| {
                read(text("Name", &[], GbxmlText::Parts(&[TextLabel(name), Text(" "), Text(number), Text(" mm")])));
            }),
            FrameDescription(curtain, fraction) => {
                if let Some(fraction) = fraction {
                    with_number_text(round9(fraction), |fraction| {
                        let prefix = if curtain { "curtain wall, frame fraction " } else { "frame fraction " };
                        read(text("Description", &[], GbxmlText::Parts(&[Text(prefix), Text(fraction)])));
                    });
                } else {
                    label("Description", "curtain wall", &mut read);
                }
            }
            GlassCad(name, id) => read(text(name, &[], GbxmlText::Parts(&[TextLabel(id), Text("/glass")]))),
            Solar(value) => with_number_text(value, |value| {
                read(text("SolarHeatGainCoeff", &[attribute("unit", "Fraction"), attribute("solarIncidentAngle", "0")], GbxmlText::Text(value)));
            }),
        }
        Some(())
    }
}

impl GbxmlTreeSource for View<'_> {
    fn with_element<R>(&self, path: &[usize], read: impl FnOnce(GbxmlElement<'_>) -> R) -> ResultFor<R> {
        let mut read = Some(read);
        let mut result = None;
        self.render(self.node(path).ok_or(ABSENT)?, |element| {
            result = Some(read.take().expect("gbXML element is lent exactly once")(element));
        }).ok_or(ABSENT)?;
        result.ok_or(ABSENT)
    }

    fn has_element(&self, path: &[usize]) -> ResultFor<bool> { Ok(self.node(path).is_some()) }
}
