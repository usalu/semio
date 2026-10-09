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
        FamilySyntax => row("family.syntax", Error, "A formula of family {elements} does not parse.", "Eine Formel der Familie {elements} lässt sich nicht lesen."),
        FamilyKind => row("family.kind", Error, "A formula of family {elements} mixes kinds or computes the wrong kind.", "Eine Formel der Familie {elements} mischt Größenarten oder liefert die falsche Art."),
        FamilyCycle => row("family.cycle", Error, "Parameters of family {elements} depend on each other in a circle: {missing}.", "Parameter der Familie {elements} hängen im Kreis voneinander ab: {missing}."),
        FamilyUnknown => row("family.unknown", Error, "A formula of family {elements} uses the unknown name {missing}.", "Eine Formel der Familie {elements} verwendet den unbekannten Namen {missing}."),
        FamilyDivisionByZero => row("family.division-by-zero", Error, "A formula of family {elements} divides by zero.", "Eine Formel der Familie {elements} teilt durch null."),
        FamilyNegative => row("family.negative", Error, "A dimension of family {elements} is not positive.", "Eine Abmessung der Familie {elements} ist nicht positiv."),
        FamilyDependency => row("family.dependency", Warning, "A formula of family {elements} depends on {missing}, which has no value.", "Eine Formel der Familie {elements} hängt von {missing} ab, das keinen Wert hat."),
        FamilyDomain => row("family.domain", Error, "A formula of family {elements} has no finite value.", "Eine Formel der Familie {elements} hat keinen endlichen Wert."),
        FamilyOutline => row("family.outline", Warning, "The outline or section of family {elements} is not usable.", "Die Kontur oder der Schnitt der Familie {elements} ist nicht brauchbar."),
        RefProfileFamily => row("reference.profile-family", Error, "{elements} uses the profile family {missing}, which does not exist or is no profile family.", "{elements} verwendet die Profilfamilie {missing}, die nicht existiert oder keine Profilfamilie ist."),
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
        StairStringerIgnored => row("stair.stringer-ignored-on-spiral", Warning, "Stair {elements} winds as a spiral: its stringer is left out.", "Treppe {elements} windet sich als Spindeltreppe: ihre Wangen entfallen."),
        SpaceNotEnclosed => row("space.not-enclosed", Warning, "Space {elements} is not enclosed by walls.", "Raum {elements} ist nicht von Wänden umschlossen."),
        SpaceSeedInWall => row("space.seed-in-wall", Warning, "The seed of space {elements} lies inside a wall.", "Der Startpunkt von Raum {elements} liegt in einer Wand."),
        SpaceDuplicateNumber => row("space.duplicate-number", Warning, "Spaces {elements} share the same number.", "Räume {elements} haben dieselbe Nummer."),
        OpeningOutsideTrimmed => row("opening.outside-trimmed", Error, "Opening {elements} touches a joined corner, the end or the top of its wall and cannot be cut out.", "Öffnung {elements} berührt eine Wandecke, das Wandende oder die Wandoberkante und kann nicht ausgeschnitten werden."),
        RoofFlatCurved => row("roof.fallback-flat.curved-footprint", Warning, "Roof {elements} has a curved footprint and is built as a flat roof.", "Dach {elements} hat einen gekrümmten Grundriss und wird als Flachdach gebaut."),
        RoofFlatSkeleton => row("roof.fallback-flat.skeleton", Warning, "Roof {elements} has a footprint whose roof surface cannot be built and is built as a flat roof.", "Dach {elements} hat einen Grundriss, dessen Dachfläche nicht gebaut werden kann, und wird als Flachdach gebaut."),
        RoofFlatDegenerate => row("roof.fallback-flat.degenerate-footprint", Warning, "Roof {elements} has a footprint without area and is built as a flat roof.", "Dach {elements} hat einen Grundriss ohne Fläche und wird als Flachdach gebaut."),
        RoofFlatPitch => row("roof.fallback-flat.invalid-pitch", Warning, "Roof {elements} has a pitch that cannot be built and is built as a flat roof.", "Dach {elements} hat eine Neigung, die nicht gebaut werden kann, und wird als Flachdach gebaut."),
        ClashBeamCeiling => row("clash.beam-ceiling", Warning, "Beam and ceiling {elements} overlap by {overlap_volume} m3: the ceiling is hung higher than the underside of the beam.", "Träger und Decke {elements} überschneiden sich um {overlap_volume} m3: die Decke hängt höher als die Trägerunterkante."),
        ClashCeilingCeiling => row("clash.ceiling-ceiling", Warning, "Ceilings {elements} overlap by {overlap_volume} m3.", "Unterdecken {elements} überschneiden sich um {overlap_volume} m3."),
        RefCeilingType => row("reference.ceiling-type", Error, "Ceiling {elements} uses the missing ceiling type {missing}.", "Unterdecke {elements} verwendet den fehlenden Unterdeckentyp {missing}."),
        CeilingOutsideStorey => row("ceiling.outside-storey", Warning, "Ceiling {elements} reaches outside its storey: its top is {top} m and its underside {bottom} m above the storey floor, the storey is {height} m high.", "Unterdecke {elements} ragt aus ihrem Geschoss heraus: ihre Oberkante liegt {top} m und ihre Unterkante {bottom} m über dem Geschossboden, das Geschoss ist {height} m hoch."),
        RampSlope => row("ramp.slope", Error, "Ramp {elements} climbs at {slope_percent} %, above its limit of {limit_percent} % (rise {rise} m over {run} m).", "Rampe {elements} steigt mit {slope_percent} %, über dem Grenzwert von {limit_percent} % (Höhe {rise} m auf {run} m)."),
        RampNoRun => row("ramp.no-run", Error, "Ramp {elements} has a rise of {rise} m but no sloped run left between its landings.", "Rampe {elements} hat einen Höhenunterschied von {rise} m, aber zwischen den Podesten keine Neigungsstrecke."),
        RefRailingHost => row("reference.railing-host", Error, "Railing {elements} is hosted by {missing}, which is no stair, ramp or slab.", "Geländer {elements} sitzt auf {missing}, das weder Treppe, Rampe noch Decke ist."),
        RailingHostUnresolved => row("railing.host-unresolved", Warning, "Railing {elements} cannot follow its host: the slab edge is missing, curved or has no length.", "Geländer {elements} kann seinem Träger nicht folgen: die Deckenkante fehlt, ist gekrümmt oder hat keine Länge."),
        RoofOverhangCollapsed => row("roof.overhang-collapsed", Warning, "The overhang of roof {elements} collapses and is left out.", "Der Dachüberstand von Dach {elements} fällt in sich zusammen und entfällt."),
        RoofGableToHip => row("roof.gable-ends-adjust-to-hip", Warning, "Roof {elements} has a footprint with a reflex corner next to a gable end and is built as a hip roof.", "Dach {elements} hat einen Grundriss mit einer einspringenden Ecke neben einem Giebel und wird als Walmdach gebaut."),
        AnnotationAnchorMissing => row("annotation.anchor-missing", Error, "Annotation {elements} is anchored to the missing element {missing}.", "Beschriftung {elements} ist am fehlenden Element {missing} verankert."),
        AnnotationAnchorUnresolved => row("annotation.anchor-unresolved", Warning, "An anchor of annotation {elements} has no geometry now: a curved or collapsed face, a zero-length line, or a line parallel to the measuring direction.", "Ein Anker der Beschriftung {elements} hat keine Geometrie mehr: eine gekrümmte oder eingefallene Fläche, eine Linie ohne Länge oder eine Linie parallel zur Messrichtung."),
        AnnotationStyleMissing => row("annotation.style-missing", Error, "Annotation {elements} uses the missing annotation style {missing}.", "Beschriftung {elements} verwendet den fehlenden Beschriftungsstil {missing}."),
        DimensionZero => row("annotation.dimension-zero", Warning, "Dimension {elements} measures a span of zero length.", "Bemaßung {elements} misst eine Strecke der Länge null."),
        DimensionLockViolated => row("annotation.dimension-lock-violated", Warning, "Dimension {elements} measures {length} m but is locked to {lock} m (difference {difference} m).", "Bemaßung {elements} misst {length} m, ist aber auf {lock} m gesperrt (Abweichung {difference} m)."),
        PropertyRequiredMissing => row("property.required-missing", Warning, "{elements} lacks the required property {missing}.", "{elements} fehlt die erforderliche Eigenschaft {missing}."),
        PropertyKindMismatch => row("property.kind-mismatch", Warning, "Property {missing} of {elements} has a value of another type than its definition.", "Die Eigenschaft {missing} von {elements} hat einen anderen Werttyp als ihre Definition."),
        PropertyOutOfRange => row("property.out-of-range", Warning, "Property {missing} of {elements} lies outside the range of its definition.", "Die Eigenschaft {missing} von {elements} liegt außerhalb des Wertebereichs ihrer Definition."),
        PropertyNotAllowed => row("property.not-allowed", Warning, "Property {missing} of {elements} is not one of the allowed values of its definition.", "Die Eigenschaft {missing} von {elements} ist keiner der zulässigen Werte ihrer Definition."),
        ClassificationUnknownCode => row("classification.unknown-code", Warning, "{elements} is classified with the code {missing}, which the table of its classification system does not list.", "{elements} ist mit dem Code {missing} klassifiziert, den die Tabelle seines Klassifikationssystems nicht aufführt."),
        RefClassificationSystem => row("reference.classification-system", Error, "{elements} is classified in the missing classification system {missing}.", "{elements} ist im fehlenden Klassifikationssystem {missing} klassifiziert."),
        CurtainOverrideOutOfGrid => row("curtain-wall.override-out-of-grid", Warning, "Panel override {elements} addresses cell ({u}, {v}), but the grid of the curtain wall has only {u_panels} by {v_panels} cells: the override has no effect.", "Die Abweichung {elements} betrifft das Feld ({u}, {v}), das Raster der Vorhangfassade hat aber nur {u_panels} mal {v_panels} Felder: die Abweichung wirkt nicht."),
        CurtainDoorNotAtBase => row("curtain-wall.door-not-at-base", Warning, "A door panel of {elements} sits in row {v} of the curtain wall: a door belongs in the base row.", "Ein Türfeld von {elements} sitzt in Reihe {v} der Vorhangfassade: eine Tür gehört in die unterste Reihe."),
        CurtainGridLineOutside => row("curtain-wall.grid-line-outside", Warning, "Curtain wall {elements} has {ignored} grid line(s) outside its extent or listed twice: they are ignored.", "Vorhangfassade {elements} hat {ignored} Rasterlinie(n) außerhalb ihrer Ausdehnung oder doppelt: sie werden ignoriert."),
        CurtainDuplicateOverride => row("curtain-wall.duplicate-override", Warning, "Panel overrides {elements} address the same cell: only the first one counts.", "Die Abweichungen {elements} betreffen dasselbe Feld: nur die erste zählt."),
        RefCurtainWallType => row("reference.curtain-wall-type", Error, "Curtain wall {elements} uses the missing curtain wall type {missing}.", "Vorhangfassade {elements} verwendet den fehlenden Fassadentyp {missing}."),
        RefCurtainPanel => row("reference.curtain-panel", Error, "The curtain panel of {elements} names the missing door type, window type or material {missing}.", "Die Fassadenfüllung von {elements} nennt den fehlenden Türtyp, Fenstertyp oder das fehlende Material {missing}."),
        RefCurtainOverrideHost => row("reference.curtain-override-host", Error, "Panel override {elements} addresses the missing curtain wall {missing}.", "Die Abweichung {elements} betrifft die fehlende Vorhangfassade {missing}."),
        ColumnTiltInvalid => row("column.tilt-invalid", Error, "Column {elements} leans by {angle} rad, which is not a lean of more than 0 and at most 60 degrees.", "Stütze {elements} neigt sich um {angle} rad, das ist keine Neigung von mehr als 0 und höchstens 60 Grad."),
        RefWallSweepHost => row("reference.wall-sweep-host", Error, "Wall sweep {elements} runs along the missing wall {missing}.", "Wandprofil {elements} läuft an der fehlenden Wand {missing} entlang."),
        RefAttachTarget => row("reference.attach-target", Error, "Wall {elements} is attached to the missing roof, slab or ceiling {missing}.", "Wand {elements} ist an das fehlende Dach, die fehlende Decke oder Unterdecke {missing} angebunden."),
        WallAttachCycle => row("wall.attach-cycle", Error, "The attach references of wall {elements} loop.", "Die Anbindungen der Wand {elements} bilden eine Schleife."),
        WallAttachUnreached => row("wall.attach-unreached", Warning, "Wall {elements}: the roof, slab or ceiling it is attached to covers only {covered} % of its axis, the nearest covered height is held.", "Wand {elements}: das angebundene Dach, die Decke oder Unterdecke deckt nur {covered} % ihrer Achse ab, die nächste überdeckte Höhe wird gehalten."),
        WallAttachCollapsed => row("wall.attach-collapsed", Warning, "Wall {elements}: the surface it is attached to lies below its base, its top is lifted to the base.", "Wand {elements}: die angebundene Fläche liegt unter ihrer Unterkante, die Oberkante wird auf die Unterkante gehoben."),
        WallSweepAboveWall => row("wall-sweep.above-wall", Warning, "Sweep {elements} reaches {height} m above the base, higher than the {wall_height} m the wall keeps at its lowest.", "Wandprofil {elements} reicht {height} m über die Basis, höher als die {wall_height} m, die die Wand an ihrer niedrigsten Stelle hat."),
        WallSweepNoRun => row("wall-sweep.no-run", Info, "Sweep {elements} is interrupted over its whole length by openings.", "Wandprofil {elements} wird auf seiner ganzen Länge von Öffnungen unterbrochen."),
        OpeningRevealDepth => row("opening.reveal-depth", Warning, "Opening {elements}: the frame of {frame_depth} m set back by {reveal} m does not fit the wall thickness of {thickness} m.", "Öffnung {elements}: der Rahmen von {frame_depth} m, um {reveal} m zurückgesetzt, passt nicht in die Wanddicke von {thickness} m."),
        TagEmpty => row("annotation.tag-empty", Info, "Tag {elements} prints nothing: its element has no such property.", "Kennzeichnung {elements} gibt nichts aus: ihr Element hat diese Eigenschaft nicht."),
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
