//! 🌉️ Traffic: the bridge deck with its notional lanes, the LM1 tandems and UDL of every lane, the footways, the alternative models LM2, LM3 and LM4 with the footbridge crowd (EN 1991-2),
//! and the crane runway with its wheel loads (EN 1991-3).

use super::kit::*;
use super::layout::{bands_of, deck_width, Band, DECK_THICKNESS, DECK_Z, FOOTWAY, RUNWAY_HEIGHT};
use super::model::{En1991Snapshot, StructureKind};
use super::scene::*;
use super::{checks, fields, handles, Ctx};
use checks::{part_2, part_3};

const LIFT: f64 = 0.02;
const LANE_FALLBACK: f64 = 3.0;
const WHEEL_TRACK: f64 = 2.0;
const AXLE_SPACING: f64 = 1.2;
const LM3_AXLES: usize = 4;
const LM3_SPACING: f64 = 1.5;
const LM3_WIDTH: f64 = 3.0;
pub const RAIL_AXLES: usize = 4;
pub const RAIL_AXLE_SPACING: f64 = 1.6;
pub const RAIL_LEAD: f64 = 0.8;
const RAIL_GAUGE: f64 = 1.435;
pub const SW0_LENGTH: f64 = 15.0;
pub const SW0_GAP: f64 = 5.3;
pub const SW2_LENGTH: f64 = 25.0;
pub const SW2_GAP: f64 = 7.0;

/// 🔀 Layer of an alternative load model: visible when the selected load group contains it (gr1b: LM2, gr4: LM4), else on the hidden alternatives layer.
fn model_layer(d: &En1991Snapshot, member: fn(part_2::GroupMembers) -> bool) -> &'static str { if part_2::group_members(&d.bridge_load_group).is_some_and(member) { LAYER_TRAFFIC } else { LAYER_TRAFFIC_ALT } }

fn wheel_forces(d: &En1991Snapshot, lane: u8) -> f64 {
    part_2::lm1_tandem_n(d.annex, lane) / 2.0
}

struct Deck<'a> {
    ctx: &'a Ctx<'a>,
    span: f64,
}

impl Deck<'_> {
    fn strip(&self, y0: f64, y1: f64) -> Vec<Vec3> {
        rect2(0.0, y0, self.span, y1).iter().map(|p| self.ctx.layout.bridge.at(p[0], p[1], DECK_Z + LIFT)).collect()
    }

    fn glyph(&self, out: &mut Out, id: String, geometry: LoadGlyph, name: LocalizedCopy, detail: LocalizedCopy, check: &[&str], layer: &'static str, points: &[Vec3]) {
        let d = self.ctx.input.document;
        out.push(
            Spec::new(EntityKind::Load, id, "@site", Geometry::Load(geometry), load(LoadKind::Variable), name, layer)
                .bound(binding_where(self.ctx.input.report, |c| check.contains(&c.id.as_str())))
                .channel(ColourChannel::Role)
                .opacity(0.8)
                .detail(detail)
                .handles(handles::ids(d, self.ctx.layout, "bridge")),
            points,
        );
    }

    fn area(&self, out: &mut Out, id: &str, y: (f64, f64), q: f64, name: LocalizedCopy, check: &[&str], layer: &'static str) {
        let points = self.strip(y.0, y.1);
        let detail = bi(|de| format!("q = {} kN/m²", num(q / 1000.0, 2, de)));
        let geometry = LoadGlyph::Area { polygon: polygon3(&points), direction: mul(&Vec3::Z, -1.0), intensity: pressure(q), case: case(id, LoadKind::Variable) };
        self.glyph(out, id.to_string(), geometry, name, detail, check, layer, &points);
    }

    fn braking(&self, out: &mut Out, y: f64) {
        let (d, lanes) = (self.ctx.input.document, self.ctx.input.document.bridge_lanes.iter().map(|l| (l.lm1_lane, l.width)).collect::<Vec<_>>());
        let n = part_2::braking_force_of_deck_kn(d.annex, self.span, d.bridge_lane_width, &lanes) * 1000.0;
        let at = self.ctx.layout.bridge.at(0.0, y, DECK_Z + LIFT);
        let geometry = LoadGlyph::Point { at: at.clone(), direction: self.ctx.layout.bridge.dir(&Vec3::X), magnitude: force(n), case: case("bridge:braking", LoadKind::Variable) };
        let detail = bi(|de| format!("Q_lk = {} kN {}", num(n / 1000.0, 0, de), if de { "(Q_1k, q_1k, w_1 mit alpha = 1 angenommen; ungeprüft)" } else { "(Q_1k, q_1k, w_1 with alpha = 1 assumed; unverified)" }));
        self.glyph(out, "bridge:braking".into(), geometry, loc("Braking and acceleration force Q_lk", "Brems- und Anfahrkraft Q_lk"), detail, &["en1991.2.braking"], LAYER_TRAFFIC, &[at]);
    }

    fn centrifugal(&self, out: &mut Out, y: f64) {
        let (d, widths) = (self.ctx.input.document, self.ctx.input.document.bridge_lanes.iter().map(|l| l.width).collect::<Vec<_>>());
        let lanes = part_2::lane_layout(d.bridge_lane_width.max(0.0), &widths).0;
        let n = part_2::centrifugal_force_kn(d.annex, part_2::tandem_weight_kn(d.annex, lanes), d.bridge_curve_radius) * 1000.0;
        if n <= 0.0 {
            return;
        }
        let at = self.ctx.layout.bridge.at(self.span / 2.0, y, DECK_Z + LIFT);
        let geometry = LoadGlyph::Point { at: at.clone(), direction: self.ctx.layout.bridge.dir(&Vec3::Y), magnitude: force(n), case: case("bridge:centrifugal", LoadKind::Variable) };
        let detail = bi(|de| format!("Q_tk = {} kN, r = {} m {}", num(n / 1000.0, 1, de), num(d.bridge_curve_radius, 0, de), if de { "(Tabelle 4.3, ungeprüft; Krümmungsmittelpunkt auf der -Y-Seite angenommen)" } else { "(Table 4.3, unverified; bend centre assumed on the -Y side)" }));
        self.glyph(out, "bridge:centrifugal".into(), geometry, loc("Centrifugal force Q_tk", "Fliehkraft Q_tk"), detail, &["en1991.2.centrifugal"], LAYER_TRAFFIC, &[at]);
    }

    fn point(&self, out: &mut Out, id: String, x: f64, y: f64, n: f64, name: LocalizedCopy, check: &[&str], layer: &'static str) {
        let at = self.ctx.layout.bridge.at(x, y, DECK_Z + LIFT);
        let detail = bi(|de| format!("F = {} kN", num(n / 1000.0, 0, de)));
        let geometry = LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: force(n), case: case(&id, LoadKind::Variable) };
        self.glyph(out, id, geometry, name, detail, check, layer, &[at]);
    }
}

pub fn bridge(ctx: &Ctx, out: &mut Out) {
    let (d, layout) = (ctx.input.document, ctx.layout);
    if d.structure_kind != StructureKind::Bridge || !(out.wants(global::BRIDGE_DECK) || d.bridge_lanes.iter().any(|l| out.wants(&l.id))) {
        return;
    }
    let pose = &layout.bridge;
    let span = d.bridge_span.max(1.0);
    let carriageway = d.bridge_lane_width.max(0.0);
    let deck_w = deck_width(d);
    let (bands, remaining, remaining_y0) = bands_of(d);
    let deck = Deck { ctx, span };
    let lane_owner = |band: &Band| if band.derived { global::BRIDGE_DECK.to_owned() } else { band.id.clone() };
    if out.own(global::BRIDGE_DECK) {
        deck_body(&deck, out, span, deck_w, carriageway);
    }
    for band in &bands {
        if !out.own(&lane_owner(band)) {
            continue;
        }
        lane(&deck, out, band);
    }
    if remaining > 1e-6 && out.own(global::BRIDGE_DECK) {
        deck.area(out, "bridge:lm1-remaining", (remaining_y0, remaining_y0 + remaining), part_2::remaining_area_udl_pa(d.annex), loc("LM1 remaining area", "LM1 Restfläche"), &["en1991.2.lm1-remaining"], LAYER_TRAFFIC);
    }
    for (n, (fraction, layer, ..)) in TANDEM_STOPS.iter().enumerate() {
        let at = if n == 0 { d.bridge_tandem_x.clamp(0.0, span) } else { span * fraction };
        for band in bands.iter().filter(|b| b.number <= 3) {
            if !out.own(&lane_owner(band)) {
                continue;
            }
            tandem(&deck, out, band, n, at, *layer);
        }
    }
    if !out.own(global::BRIDGE_DECK) {
        return;
    }
    let footway = if d.bridge_load_group == "gr1a" { part_2::footway_reduced_pa() } else { part_2::footway_pa(d.annex) };
    if part_2::group_members(&d.bridge_load_group).is_none_or(|m| m.footway) {
        deck.area(out, "bridge:footway-1", (0.0, FOOTWAY), footway, loc("Footway south", "Gehweg Süd"), &["en1991.2.footway"], LAYER_TRAFFIC);
        deck.area(out, "bridge:footway-2", (FOOTWAY + carriageway, deck_w), footway, loc("Footway north", "Gehweg Nord"), &["en1991.2.footway"], LAYER_TRAFFIC);
    }
    let centre = bands.first().map_or(FOOTWAY + LANE_FALLBACK / 2.0, |b| (b.y0 + b.y1) / 2.0);
    for (s, y) in [centre - WHEEL_TRACK / 2.0, centre + WHEEL_TRACK / 2.0].into_iter().enumerate() {
        deck.point(out, format!("bridge:lm2-wheel-{s}"), span / 2.0, y, part_2::lm2_axle_n(d.annex) / 2.0, loc("LM2 single axle", "LM2 Einzelachse"), &["en1991.2.lm2"], model_layer(d, |m| m.lm2));
    }
    let default_vehicle_axles = if d.special_vehicles.is_empty() { LM3_AXLES } else { 0 };
    for j in 0..default_vehicle_axles {
        let x = span / 2.0 + (j as f64 - (LM3_AXLES as f64 - 1.0) / 2.0) * LM3_SPACING;
        let (a, b) = (pose.at(x, centre - LM3_WIDTH / 2.0, DECK_Z + LIFT), pose.at(x, centre + LM3_WIDTH / 2.0, DECK_Z + LIFT));
        let per_metre = part_2::lm3_axle_n(d.annex) / LM3_WIDTH;
        deck.glyph(
            out,
            format!("bridge:lm3-axle-{j}"),
            LoadGlyph::Line { path: polyline(&[a.clone(), b.clone()]), direction: mul(&Vec3::Z, -1.0), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, per_metre) }, case: case(&format!("bridge:lm3-axle-{j}"), LoadKind::Variable) },
            loc("LM3 special vehicle axle line", "LM3 Achslinie Sonderfahrzeug"),
            bi(|de| format!("F = {} kN", num(part_2::lm3_axle_n(d.annex) / 1000.0, 0, de))),
            &["en1991.2.lm3"],
            LAYER_TRAFFIC_ALT,
            &[a, b],
        );
    }
    deck.braking(out, centre);
    deck.centrifugal(out, centre);
    rail(&deck, d, out, centre);
    deck.area(out, "bridge:lm4", (0.0, deck_w), part_2::lm4_crowd_pa(d.annex), loc("LM4 crowd loading", "LM4 Menschengedränge"), &["en1991.2.lm4"], model_layer(d, |m| m.lm4));
    if carriageway > 1e-6 {
        deck.area(out, fields::travel::OVERLAY, (FOOTWAY, FOOTWAY + carriageway), part_2::lm1_udl_pa(d.annex, 1), loc("LM1 tandem travel", "LM1-Tandemfahrt"), &["en1991.2.lm1-tandem"], LAYER_TRAVEL);
        fields::travel::apply(&mut out.entities, d);
    }
    deck.area(out, "bridge:footbridge-crowd", (0.0, deck_w), part_2::footbridge_load_kn_m2(span) * 1000.0, loc("Footbridge crowd", "Fußgängerbrücke Menschengedränge"), &["en1991.2.footway"], LAYER_TRAFFIC_ALT);
    let a = pose.at(0.0, -1.0, DECK_Z);
    let b = pose.at(span, -1.0, DECK_Z);
    out.annotate(Annotation::Dimension { id: "bridge:span".into(), from: a.clone(), to: b, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, span), binding: Some(FieldBinding::new("bridgeSpan", Affine::IDENTITY, "m")) }, &a);
    let c = pose.at(-1.0, FOOTWAY, DECK_Z);
    out.annotate(Annotation::Dimension { id: "bridge:width".into(), from: c.clone(), to: pose.at(-1.0, FOOTWAY + carriageway, DECK_Z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, carriageway), binding: Some(FieldBinding::new("bridgeLaneWidth", Affine::IDENTITY, "m")) }, &c);
}

fn deck_body(deck: &Deck, out: &mut Out, span: f64, deck_w: f64, carriageway: f64) {
    let (d, layout, report) = (deck.ctx.input.document, deck.ctx.layout, deck.ctx.input.report);
    let pose = &layout.bridge;
    let corners = [pose.at(0.0, 0.0, DECK_Z - DECK_THICKNESS), pose.at(span, deck_w, DECK_Z)];
    out.push(
        Spec::new(
            EntityKind::Solid,
            "bridge",
            "@site",
            Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, 0.0, DECK_Z - DECK_THICKNESS]), base: polygon2(&rect2(0.0, 0.0, span, deck_w)), height: DECK_THICKNESS } }),
            MaterialRole::Concrete,
            loc("Bridge deck", "Brückenfahrbahn"),
            LAYER_TRAFFIC,
        )
        .bound(binding_prefixed(report, &["en1991.2."]))
        .opacity(0.9)
        .detail(bi(|de| format!("L = {} m, w = {} m, {} {}", num(span, 2, de), num(carriageway, 2, de), if de { "Lastgruppe" } else { "load group" }, d.bridge_load_group)))
        .handles(handles::ids(d, layout, "bridge")),
        &corners,
    );
    for (tag, x, kind) in [("a", 0.0, SupportKind::Pinned), ("b", span, SupportKind::Roller { normal: Vec3::Z })] {
        for (side, y) in [("1", 0.5), ("2", deck_w - 0.5)] {
            let at = pose.at(x, y, DECK_Z - DECK_THICKNESS);
            out.push(
                Spec::new(EntityKind::Support, format!("bridge:support-{tag}{side}"), "@site", Geometry::Support(SupportGeometry { kind: kind.clone(), at: at.clone(), frame: Frame { origin: at.clone(), x: pose.dir(&Vec3::X), y: pose.dir(&Vec3::Y), z: Vec3::Z } }), MaterialRole::Support, if tag == "a" { loc("Pinned bearing", "Festlager") } else { loc("Roller bearing", "Gleitlager") }, LAYER_TRAFFIC),
                &[at],
            );
        }
    }
}

fn lane(deck: &Deck, out: &mut Out, band: &Band) {
    let (d, layout, span, pose) = (deck.ctx.input.document, deck.ctx.layout, deck.span, &deck.ctx.layout.bridge);
    let points = deck.strip(band.y0, band.y1);
    let label = |de| band.name(de);
    let (surface_id, collection, handle_ids) = if band.derived { (band.sub("lane", ""), "@site", Vec::new()) } else { (band.id.clone(), "bridgeLanes", handles::ids(d, layout, &band.id)) };
    out.push(
        Spec::new(EntityKind::Surface, surface_id, collection, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&points) } }), MaterialRole::Neutral, bi(label), LAYER_TRAFFIC)
            .opacity(0.5)
            .detail(bi(|de| format!("w = {} m", num(band.y1 - band.y0, 2, de))))
            .handles(handle_ids),
        &points,
    );
    out.label(band.label_id(), pose.at(span * 0.08, (band.y0 + band.y1) / 2.0, DECK_Z + LIFT), bi(label), Tone::Neutral);
    let udl_check = if band.derived { "en1991.2.lm1-udl".to_string() } else { format!("en1991.2.lm1-lane-udl.{}", band.id) };
    deck.area(out, &band.sub("lm1-udl", ""), (band.y0, band.y1), part_2::lm1_udl_pa(d.annex, band.number), bi(|de| format!("LM1 UDL, {}", band.name(de))), &[udl_check.as_str()], LAYER_TRAFFIC);
}

fn tandem(deck: &Deck, out: &mut Out, band: &Band, n: usize, at: f64, layer: &'static str) {
    let (d, span) = (deck.ctx.input.document, deck.span);
    let centre = (band.y0 + band.y1) / 2.0;
    let check = if band.derived { "en1991.2.lm1-tandem".to_string() } else { format!("en1991.2.lm1-lane.{}", band.id) };
    for (a, x) in [at - AXLE_SPACING / 2.0, at + AXLE_SPACING / 2.0].into_iter().enumerate() {
        for (s, y) in [centre - WHEEL_TRACK / 2.0, centre + WHEEL_TRACK / 2.0].into_iter().enumerate() {
            let id = if n == 0 { band.sub("lm1-tandem", &format!("-{a}{s}")) } else { band.sub(&format!("lm1-position-{n}-tandem"), &format!("-{a}{s}")) };
            deck.point(out, id, x.clamp(0.0, span), y, wheel_forces(d, band.number), bi(|de| format!("LM1 {} {}", if de { "Tandem" } else { "tandem" }, band.name(de))), &[check.as_str()], layer);
        }
    }
}

fn rail(deck: &Deck, d: &En1991Snapshot, out: &mut Out, centre: f64) {
    let (pose, span) = (&deck.ctx.layout.bridge, deck.span);
    let mid = span / 2.0;
    let segment = |out: &mut Out, id: String, (x0, x1): (f64, f64), q: f64, check: &str, name: LocalizedCopy| {
        let (x0, x1) = (x0.clamp(0.0, span), x1.clamp(0.0, span));
        if x1 - x0 < 1e-6 {
            return;
        }
        let (a, b) = (pose.at(x0, centre, DECK_Z + LIFT), pose.at(x1, centre, DECK_Z + LIFT));
        let detail = bi(|de| format!("q = {} kN/m, {} m", num(q / 1000.0, 1, de), num(x1 - x0, 1, de)));
        deck.glyph(out, id.clone(), LoadGlyph::Line { path: polyline(&[a.clone(), b.clone()]), direction: mul(&Vec3::Z, -1.0), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, q) }, case: case(&id, LoadKind::Variable) }, name, detail, &[check], LAYER_RAIL, &[a, b]);
    };
    for j in 0..RAIL_AXLES {
        let x = mid + (j as f64 - (RAIL_AXLES as f64 - 1.0) / 2.0) * RAIL_AXLE_SPACING;
        for (s, y) in [centre - RAIL_GAUGE / 2.0, centre + RAIL_GAUGE / 2.0].into_iter().enumerate() {
            deck.point(out, format!("bridge:rail-lm71-axle-{j}{s}"), x.clamp(0.0, span), y, d.assumed_rail_lm71 / 2.0, loc("LM71 axle", "LM71 Achse"), &["en1991.2.rail.lm71-axle"], LAYER_RAIL);
        }
    }
    let group = (RAIL_AXLES as f64 - 1.0) / 2.0 * RAIL_AXLE_SPACING + RAIL_LEAD;
    let udl = loc("LM71 distributed load", "LM71 Streckenlast");
    segment(out, "bridge:rail-lm71-udl-0".into(), (0.0, mid - group), d.assumed_rail_lm71_udl, "en1991.2.rail.lm71-udl", udl.clone());
    segment(out, "bridge:rail-lm71-udl-1".into(), (mid + group, span), d.assumed_rail_lm71_udl, "en1991.2.rail.lm71-udl", udl);
    for (tag, length, gap, q, check, name) in [("sw0", SW0_LENGTH, SW0_GAP, d.assumed_rail_sw0, "en1991.2.rail.sw0", loc("SW/0 loaded length", "SW/0 Belastungslänge")), ("sw2", SW2_LENGTH, SW2_GAP, d.assumed_rail_sw2, "en1991.2.rail.sw2", loc("SW/2 loaded length", "SW/2 Belastungslänge"))] {
        segment(out, format!("bridge:rail-{tag}-0"), (mid - gap / 2.0 - length, mid - gap / 2.0), q, check, name.clone());
        segment(out, format!("bridge:rail-{tag}-1"), (mid + gap / 2.0, mid + gap / 2.0 + length), q, check, name);
    }
}

/// 🪝 Crane hoists: the rated hoist load Q_H hanging from the trolley at the crane position and the static test load Q_T beside it, one pair per hoist row in a line across the runway, bound to the test-load checks of the row.
pub fn hoists(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    if !d.crane_claimed {
        return;
    }
    let pose = &layout.crane;
    for (index, row) in d.crane_hoists.iter().enumerate() {
        if !out.own(&row.id) {
            continue;
        }
        let grips = handles::ids(d, layout, &row.id);
        let y = HOIST_SPACING * index as f64;
        for (tag, offset, n, check, name) in [
            ("rated", 0.0, row.rated_load, "en1991.3.hoist-test-static", loc("Rated hoist load Q_H", "Hublast Q_H")),
            ("test", 0.6, part_3::test_load_static_n(row.rated_load), "en1991.3.hoist-test-static", loc("Static test load Q_T", "Statische Prüflast Q_T")),
        ] {
            let at = pose.at(d.crane_x + offset, y, RUNWAY_HEIGHT - 0.25);
            let id = if tag == "rated" { row.id.clone() } else { format!("{}:{tag}", row.id) };
            let wanted = format!("{check}.{}", row.id);
            out.push(
                Spec::new(EntityKind::Load, id.clone(), "craneHoists", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: force(n), case: case(&id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_HOISTS)
                    .bound(binding_where(report, |c| c.id == wanted))
                    .channel(ColourChannel::Role)
                    .detail(bi(|de| format!("{} = {} kN", if tag == "rated" { "Q_H" } else { "Q_T" }, num(n / 1000.0, 1, de))))
                    .handles(grips.clone()),
                &[at],
            );
        }
    }
}

pub fn crane(ctx: &Ctx, out: &mut Out) {
    let (d, layout, report) = (ctx.input.document, ctx.layout, ctx.input.report);
    if !out.own(global::CRANE) || !d.crane_claimed {
        return;
    }
    let pose = &layout.crane;
    let grips = handles::ids(d, layout, "crane");
    let (length, wheel_base) = (d.crane_runway_length.max(1.0), d.crane_wheel_base.max(0.1));
    let (start, end) = (pose.at(0.0, 0.0, RUNWAY_HEIGHT), pose.at(length, 0.0, RUNWAY_HEIGHT));
    out.push(
        Spec::new(
            EntityKind::Member,
            "crane",
            "@site",
            Geometry::Member(MemberGeometry {
                axis: polyline(&[start.clone(), end.clone()]),
                frame: Frame { origin: start.clone(), x: pose.dir(&Vec3::X), y: pose.dir(&Vec3::Y), z: Vec3::Z },
                section: SectionShape::parametric(ProfileKind::IShape, &[("h", 0.5), ("b", 0.25), ("tw", 0.014), ("tf", 0.025)]),
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: EndRelease::Pinned,
                end_release: EndRelease::Pinned,
                stations: Vec::new(),
            }),
            MaterialRole::Steel,
            loc("Crane runway", "Kranbahn"),
            LAYER_CRANE,
        )
        .bound(binding_prefixed(report, &["en1991.3."]))
        .detail(bi(|de| format!("{} {}, {} {}, v_h = {} m/s", if de { "Kranklasse" } else { "crane class" }, d.crane_class, if de { "Hubklasse" } else { "hoisting class" }, d.hoist_class, num(d.hoisting_speed, 2, de))))
        .handles(grips.clone()),
        &[start, end],
    );
    for (tag, x, kind) in [("a", 0.0, SupportKind::Pinned), ("b", length, SupportKind::Roller { normal: pose.dir(&Vec3::X) })] {
        let at = pose.at(x, 0.0, RUNWAY_HEIGHT - 0.25);
        out.push(Spec::new(EntityKind::Support, format!("crane:support-{tag}"), "@site", Geometry::Support(SupportGeometry { kind, at: at.clone(), frame: Frame { origin: at.clone(), x: pose.dir(&Vec3::X), y: pose.dir(&Vec3::Y), z: Vec3::Z } }), MaterialRole::Support, loc("Runway bearing", "Kranbahnlager"), LAYER_CRANE), &[at]);
    }
    let wheel = |out: &mut Out, id: String, x: f64, direction: Vec3, n: f64, check: &str, name: LocalizedCopy| {
        let at = pose.at(x, 0.0, RUNWAY_HEIGHT + 0.25);
        out.push(
            Spec::new(EntityKind::Load, id.clone(), "@site", Geometry::Load(LoadGlyph::Point { at: at.clone(), direction, magnitude: force(n), case: case(&id, LoadKind::Variable) }), load(LoadKind::Variable), name, LAYER_CRANE)
                .bound(binding_where(report, |c| c.id == check))
                .channel(ColourChannel::Role)
                .detail(bi(|de| format!("F = {} kN", num(n / 1000.0, 1, de))))
                .handles(grips.clone()),
            &[at],
        );
    };
    let centre = d.crane_x.clamp(wheel_base / 2.0, (length - wheel_base / 2.0).max(wheel_base / 2.0));
    if let Some(vertical) = part_3::design_vertical_wheel_n(&d.crane_class, &d.hoist_class, d.hoisting_speed) {
        for (tag, x) in [("a", centre - wheel_base / 2.0), ("b", centre + wheel_base / 2.0)] {
            wheel(out, format!("crane:wheel-{tag}"), x, mul(&Vec3::Z, -1.0), vertical, "en1991.3.crane-vertical", loc("Crane wheel load φ₂·Q_r", "Kranradlast φ₂·Q_r"));
        }
    }
    if let Some(horizontal) = part_3::design_horizontal_force_n(&d.crane_class, d.annex) {
        wheel(out, "crane:horizontal".into(), centre - wheel_base / 2.0, pose.dir(&Vec3::X), horizontal, "en1991.3.crane-horizontal", loc("Longitudinal force H_L", "Längskraft H_L"));
    }
    let a = pose.at(centre - wheel_base / 2.0, 0.6, RUNWAY_HEIGHT);
    out.annotate(Annotation::Dimension { id: "crane:wheel-base".into(), from: a.clone(), to: pose.at(centre + wheel_base / 2.0, 0.6, RUNWAY_HEIGHT), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, wheel_base), binding: None }, &a);
}
