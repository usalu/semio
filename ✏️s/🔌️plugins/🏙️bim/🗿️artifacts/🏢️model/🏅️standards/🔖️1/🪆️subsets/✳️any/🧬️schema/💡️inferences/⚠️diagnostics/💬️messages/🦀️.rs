//! 💬️ Message keys and texts of the diagnostics: one row per code with its slug, severity and the English and German message template.
//!
//! A template names its inputs in braces: `{elements}` is the comma separated element ids, `{missing}` the referenced ids that do not exist,
//! and every other `{name}` a number of `Diagnostic::values`. The key of a code is `bim.diagnostic.<slug>`; locales are `en` and `de`, there
//! is no default locale.

use super::{Diagnostic, DiagnosticCode, Severity};

/// 🌐️ The locales of the diagnostic texts, English first.
pub const LOCALES: [&str; 2] = ["en", "de"];

/// 📖️ Slug, severity and templates of one code.
pub struct Row {
    pub slug: &'static str,
    pub severity: Severity,
    pub en: &'static str,
    pub de: &'static str,
}

const fn row(slug: &'static str, severity: Severity, en: &'static str, de: &'static str) -> Row {
    Row { slug, severity, en, de }
}

/// 📖️ The row of a code.
pub fn row_of(code: DiagnosticCode) -> Row {
    use DiagnosticCode::*;
    use Severity::{Error, Info, Warning};
    match code {
        ClashWallWall => row("clash.wall-wall", Warning, "Walls {elements} overlap by {overlap_volume} m3 beyond their joins.", "Wände {elements} überschneiden sich um {overlap_volume} m3 über ihre Verbindungen hinaus."),
        ClashWallColumn => row("clash.wall-column", Warning, "Wall and column {elements} overlap by {overlap_volume} m3.", "Wand und Stütze {elements} überschneiden sich um {overlap_volume} m3."),
        ClashColumnColumn => row("clash.column-column", Error, "Columns {elements} overlap by {overlap_volume} m3.", "Stützen {elements} überschneiden sich um {overlap_volume} m3."),
        ClashWallBeam => row("clash.wall-beam", Warning, "Beam passes through wall {elements} by {overlap_volume} m3.", "Träger durchdringt Wand {elements} um {overlap_volume} m3."),
        ClashBeamColumn => row("clash.beam-column", Warning, "Beam passes through column {elements} by {overlap_volume} m3.", "Träger durchdringt Stütze {elements} um {overlap_volume} m3."),
        ClashBeamBeam => row("clash.beam-beam", Warning, "Beams {elements} overlap by {overlap_volume} m3.", "Träger {elements} überschneiden sich um {overlap_volume} m3."),
        ClashBeamSlab => row("clash.beam-slab", Warning, "Beam and slab {elements} overlap by {overlap_volume} m3.", "Träger und Decke {elements} überschneiden sich um {overlap_volume} m3."),
        ClashStairWall => row("clash.stair-wall", Warning, "Stair and wall {elements} overlap by {overlap_volume} m3.", "Treppe und Wand {elements} überschneiden sich um {overlap_volume} m3."),
        ClashStairColumn => row("clash.stair-column", Warning, "Stair and column {elements} overlap by {overlap_volume} m3.", "Treppe und Stütze {elements} überschneiden sich um {overlap_volume} m3."),
        ClashStairBeam => row("clash.stair-beam", Warning, "Stair and beam {elements} overlap by {overlap_volume} m3.", "Treppe und Träger {elements} überschneiden sich um {overlap_volume} m3."),
        ClashStairStair => row("clash.stair-stair", Warning, "Stairs {elements} overlap by {overlap_volume} m3.", "Treppen {elements} überschneiden sich um {overlap_volume} m3."),
        ClashSlabSlab => row("clash.slab-slab", Warning, "Slabs {elements} overlap by {overlap_volume} m3.", "Decken {elements} überschneiden sich um {overlap_volume} m3."),
        RefWallType => row("reference.wall-type", Error, "Wall {elements} uses the missing wall type {missing}.", "Wand {elements} verwendet den fehlenden Wandtyp {missing}."),
        RefColumnType => row("reference.column-type", Error, "Column {elements} uses the missing column type {missing}.", "Stütze {elements} verwendet den fehlenden Stützentyp {missing}."),
        RefBeamType => row("reference.beam-type", Error, "Beam {elements} uses the missing beam type {missing}.", "Träger {elements} verwendet den fehlenden Trägertyp {missing}."),
        RefSlabType => row("reference.slab-type", Error, "Slab {elements} uses the missing slab type {missing}.", "Decke {elements} verwendet den fehlenden Deckentyp {missing}."),
        RefRoofType => row("reference.roof-type", Error, "Roof {elements} uses the missing roof type {missing}.", "Dach {elements} verwendet den fehlenden Dachtyp {missing}."),
        RefWindowType => row("reference.window-type", Error, "Window {elements} uses the missing window type {missing}.", "Fenster {elements} verwendet den fehlenden Fenstertyp {missing}."),
        RefDoorType => row("reference.door-type", Error, "Door {elements} uses the missing door type {missing}.", "Tür {elements} verwendet den fehlenden Türtyp {missing}."),
        RefTopStorey => row("reference.top-storey", Error, "{elements} is constrained to the missing storey {missing}.", "{elements} ist an das fehlende Geschoss {missing} gebunden."),
        RefOpeningHost => row("reference.opening-host", Error, "Opening {elements} is hosted by the missing wall {missing}.", "Öffnung {elements} sitzt in der fehlenden Wand {missing}."),
        RefElementStorey => row("reference.element-storey", Error, "{elements} stands on the missing storey {missing}.", "{elements} steht auf dem fehlenden Geschoss {missing}."),
        RefStoreyBuilding => row("reference.storey-building", Error, "Storey {elements} belongs to the missing building {missing}.", "Geschoss {elements} gehört zum fehlenden Gebäude {missing}."),
        RefBuildingSite => row("reference.building-site", Error, "Building {elements} stands on the missing site {missing}.", "Gebäude {elements} steht auf dem fehlenden Grundstück {missing}."),
        RefGridBuilding => row("reference.grid-building", Error, "Grid line {elements} belongs to the missing building {missing}.", "Rasterlinie {elements} gehört zum fehlenden Gebäude {missing}."),
        RefLayerMaterial => row("reference.layer-material", Error, "Type {elements} has a layer of the missing material {missing}.", "Typ {elements} hat eine Schicht aus dem fehlenden Material {missing}."),
        RefTypeMaterial => row("reference.type-material", Warning, "{elements} uses the missing material {missing}.", "{elements} verwendet das fehlende Material {missing}."),
        RefPropertyElement => row("reference.property-element", Warning, "Properties or classification are attached to the missing element {elements}.", "Eigenschaften oder Klassifizierung hängen am fehlenden Element {elements}."),
        DuplicateId => row("reference.duplicate-id", Error, "The id {elements} names more than one element.", "Die Kennung {elements} bezeichnet mehr als ein Element."),
        OpeningOutsideHost => row("opening.outside-host", Error, "Opening {elements} reaches beyond the ends of its wall.", "Öffnung {elements} ragt über die Enden ihrer Wand hinaus."),
        OpeningBelowBase => row("opening.below-base", Error, "Opening {elements} starts below the base of its wall.", "Öffnung {elements} beginnt unterhalb der Wandunterkante."),
        OpeningAboveTop => row("opening.above-top", Error, "Opening {elements} ends above the top of its wall.", "Öffnung {elements} endet oberhalb der Wandoberkante."),
        OpeningOverlap => row("opening.overlap", Error, "Openings {elements} overlap each other.", "Öffnungen {elements} überschneiden sich."),
        OpeningSize => row("opening.non-positive-size", Error, "Opening {elements} has no positive width and height.", "Öffnung {elements} hat keine positive Breite und Höhe."),
        DegenerateAxis => row("degenerate.axis-length", Error, "{elements} has an axis of length {length} m.", "{elements} hat eine Achse der Länge {length} m."),
        DegenerateThickness => row("degenerate.wall-thickness", Error, "Wall {elements} has a wall type without thickness.", "Wand {elements} hat einen Wandtyp ohne Dicke."),
        DegenerateHeight => row("degenerate.height", Error, "{elements} has a height of {height} m.", "{elements} hat eine Höhe von {height} m."),
        DegenerateProfile => row("degenerate.profile", Error, "{elements} has a profile without size.", "{elements} hat ein Profil ohne Abmessung."),
        DegenerateLoop => row("degenerate.loop", Error, "{elements} has an outline of area {area} m2.", "{elements} hat eine Umrisslinie mit der Fläche {area} m2."),
        SelfIntersectingLoop => row("degenerate.self-intersection", Error, "The outline of {elements} crosses itself {crossings} times.", "Die Umrisslinie von {elements} kreuzt sich {crossings} Mal."),
        DegeneratePath => row("degenerate.railing-path", Error, "Railing {elements} has no path of positive length.", "Geländer {elements} hat keinen Pfad mit positiver Länge."),
        NonFinite => row("degenerate.non-finite", Error, "{elements} has a coordinate that is not a finite number.", "{elements} hat eine Koordinate, die keine endliche Zahl ist."),
        DegenerateSpacing => row("degenerate.curtain-spacing", Error, "Curtain wall {elements} has a grid spacing of zero or less.", "Vorhangfassade {elements} hat einen Rasterabstand von null oder weniger."),
        DegenerateStorey => row("degenerate.storey-height", Error, "Storey {elements} has a height of {height} m.", "Geschoss {elements} hat eine Höhe von {height} m."),
        StoreyLevelGap => row("storey.level-gap", Warning, "Storeys {elements} are not contiguous: level {from} is followed by level {to}.", "Geschosse {elements} sind nicht lückenlos: auf Ebene {from} folgt Ebene {to}."),
        StoreyLevelDuplicate => row("storey.level-duplicate", Error, "Storeys {elements} share the level {level} and overlap.", "Geschosse {elements} teilen sich die Ebene {level} und überlappen sich."),
        StoreyNoDatum => row("storey.no-datum", Info, "Building {elements} has no storey at level 0, the datum.", "Gebäude {elements} hat kein Geschoss auf Ebene 0, dem Bezugsniveau."),
        StairNoRise => row("stair.no-rise", Error, "Stair {elements} has no rise to climb or no valid riser limit.", "Treppe {elements} hat keine zu überwindende Höhe oder keinen gültigen Steigungsgrenzwert."),
        StairRiserHeight => row("stair.riser-too-high", Error, "Stair {elements} needs risers of {riser_height} m, above its limit of {limit} m.", "Treppe {elements} braucht Steigungen von {riser_height} m, über dem Grenzwert von {limit} m."),
        StairTreadDepth => row("stair.tread-too-shallow", Warning, "Stair {elements} has treads of {tread} m, below its minimum of {limit} m.", "Treppe {elements} hat Auftritte von {tread} m, unter dem Mindestmaß von {limit} m."),
        StairComfort => row("stair.comfort-rule", Warning, "Stair {elements} breaks the comfort rule 2R + T = {stride} m (allowed {minimum} to {maximum} m).", "Treppe {elements} verletzt die Schrittmaßregel 2h + a = {stride} m (zulässig {minimum} bis {maximum} m)."),
        SpaceNotEnclosed => row("space.not-enclosed", Warning, "Space {elements} is not enclosed by walls.", "Raum {elements} ist nicht von Wänden umschlossen."),
        SpaceSeedInWall => row("space.seed-in-wall", Warning, "The seed of space {elements} lies inside a wall.", "Der Startpunkt von Raum {elements} liegt in einer Wand."),
        SpaceDuplicateNumber => row("space.duplicate-number", Warning, "Spaces {elements} share the same number.", "Räume {elements} haben dieselbe Nummer."),
        OpeningOutsideTrimmed => row("opening.outside-trimmed", Error, "Opening {elements} touches a joined corner, the end or the top of its wall and cannot be cut out.", "Öffnung {elements} berührt eine Wandecke, das Wandende oder die Wandoberkante und kann nicht ausgeschnitten werden."),
        RoofFlatCurved => row("roof.fallback-flat.curved-footprint", Warning, "Roof {elements} has a curved footprint and is built as a flat roof.", "Dach {elements} hat einen gekrümmten Grundriss und wird als Flachdach gebaut."),
        RoofFlatNonConvex => row("roof.fallback-flat.non-convex-footprint", Warning, "Roof {elements} has a footprint that is not convex and is built as a flat roof.", "Dach {elements} hat einen nicht konvexen Grundriss und wird als Flachdach gebaut."),
        RoofFlatDegenerate => row("roof.fallback-flat.degenerate-footprint", Warning, "Roof {elements} has a footprint without area and is built as a flat roof.", "Dach {elements} hat einen Grundriss ohne Fläche und wird als Flachdach gebaut."),
        RoofFlatPitch => row("roof.fallback-flat.invalid-pitch", Warning, "Roof {elements} has a pitch that cannot be built and is built as a flat roof.", "Dach {elements} hat eine Neigung, die nicht gebaut werden kann, und wird als Flachdach gebaut."),
        RoofOverhangCollapsed => row("roof.overhang-collapsed", Warning, "The overhang of roof {elements} collapses and is left out.", "Der Dachüberstand von Dach {elements} fällt in sich zusammen und entfällt."),
    }
}

fn number(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.3}")
    }
}

/// 🌐️ The text of a diagnostic in a locale (`en` or `de`); `None` for any other locale.
pub fn render(diagnostic: &Diagnostic, locale: &str) -> Option<String> {
    let row = row_of(diagnostic.code);
    let template = match locale {
        "en" => row.en,
        "de" => row.de,
        _ => return None,
    };
    let mut text = template.replace("{elements}", &diagnostic.elements.join(", ")).replace("{missing}", &diagnostic.missing.join(", "));
    for (name, value) in &diagnostic.values {
        text = text.replace(&format!("{{{name}}}"), &number(*value));
    }
    Some(text)
}
