"""EN 1995 typed physical entities (wave 2 v4): one declarative spec read by `schema.py` (model, twins, diff, io, sqlite, field-meta, fixtures) and `kinds.py` (insert / remove / change mutation kinds).
Types: String, f64, u32. Units: m (shown in mm), rad (shown in deg), N (shown in kN), Pa (shown in kPa), s, kg_m2, n_m2_per_m (flexural stiffness per metre), none."""

UNIT_UI = {
    "m": dict(unit="m", displayUnit="mm", displayFactor=1000, precision=1, step=0.005),
    "rad": dict(unit="rad", displayUnit="°", displayFactor=57.29577951308232, precision=2, step=0.0174532925),
    "N": dict(unit="N", displayUnit="kN", displayFactor=0.001, precision=2, step=100),
    "Pa": dict(unit="Pa", displayUnit="kN/m²", displayFactor=0.001, precision=2, step=100),
    "s": dict(unit="s", displayUnit="min", displayFactor=0.016666666666666666, precision=0, step=60),
    "kg_m2": dict(unit="kg/m²", precision=1, step=5),
    "n_m2_per_m": dict(unit="N·m²/m", precision=0, step=1000),
    "n_per_m": dict(unit="N/m", displayUnit="N/mm", displayFactor=0.001, precision=2, step=100),
    "stress_pa": dict(unit="Pa", displayUnit="N/mm²", displayFactor=0.000001, precision=3, step=50000),
    "none": dict(precision=3, step=0.01),
}
FIELD_META_UNIT = {"stress_pa": "UnitId::Pascal", "m": "UnitId::Metre", "rad": "UnitId::Radian", "N": "UnitId::Newton", "Pa": "UnitId::Pascal", "s": "UnitId::Second", "kg_m2": "UnitId::KilogramPerSquareMetre", "n_per_m": "UnitId::NewtonPerMetre"}
DSL_UNIT = {"stress_pa": "Pa", "m": "m", "N": "N", "Pa": "Pa", "s": "s", "n_per_m": "N/m"}


ENUMS = {
    "TaperKind": dict(
        doc="Variable-depth shape of a glulam member (EN 1995-1-1 6.4).",
        variants=[("single-tapered", "SingleTapered", "Single tapered", "Einfach geneigt"), ("double-tapered", "DoubleTapered", "Double tapered", "Satteldach"), ("curved", "Curved", "Curved", "Gekrümmt"), ("pitched-cambered", "PitchedCambered", "Pitched cambered", "Gekrümmter Satteldachträger")],
    ),
    "TensionSide": dict(
        doc="Edge of a tapered member that carries the taper cut (EN 1995-1-1 6.4.2).",
        variants=[("tension", "Tension", "Tension edge", "Zugrand"), ("compression", "Compression", "Compression edge", "Druckrand")],
    ),
    "CutoutKind": dict(
        doc="Kind of a cut-out in a member (EN 1995-1-1 5.2, 6.5.2).",
        variants=[("notch", "Notch", "Notch", "Ausklinkung"), ("hole", "Hole", "Hole", "Loch")],
    ),
    "CutoutFace": dict(
        doc="Face of the member on which a cut-out is made.",
        variants=[("bottom", "Bottom", "Bottom face", "Unterseite"), ("top", "Top", "Top face", "Oberseite")],
    ),
    "LayerOrientation": dict(
        doc="Grain direction of one CLT layer relative to the panel span.",
        variants=[("longitudinal", "Longitudinal", "Longitudinal (along the span)", "Längs (in Spannrichtung)"), ("transverse", "Transverse", "Transverse (across the span)", "Quer (quer zur Spannrichtung)")],
    ),
    "DiaphragmKind": dict(
        doc="Kind of the diaphragm (EN 1995-1-1 9.2.3, 9.2.4).",
        variants=[("wall", "Wall", "Wall diaphragm", "Wandscheibe"), ("floor", "Floor", "Floor diaphragm", "Deckenscheibe"), ("roof", "Roof", "Roof diaphragm", "Dachscheibe")],
    ),
    "DiaphragmMethod": dict(
        doc="Racking resistance method of a wall diaphragm (EN 1995-1-1 9.2.4.2, 9.2.4.3).",
        variants=[("method-a", "MethodA", "Method A", "Verfahren A"), ("method-b", "MethodB", "Method B", "Verfahren B")],
    ),
    "TrussKind": dict(
        doc="Web pattern of a timber truss.",
        variants=[("warren", "Warren", "Warren", "Warren"), ("pratt", "Pratt", "Pratt", "Pratt"), ("howe", "Howe", "Howe", "Howe"), ("king-post", "KingPost", "King post", "Hängewerk")],
    ),
    "BracingProduct": dict(
        doc="Product family of the braced member; selects the divisor of the stabilising force (EN 1995-1-1 9.2.5).",
        variants=[("solid", "Solid", "Solid timber", "Vollholz"), ("engineered", "Engineered", "Glulam or LVL", "Brettschichtholz oder Furnierschichtholz")],
    ),
    "BracingKind": dict(
        doc="Kind of the bracing system.",
        variants=[("plan-bracing", "PlanBracing", "Plan bracing", "Dachverband"), ("bridging", "Bridging", "Bridging", "Queraussteifung"), ("elastic-restraint", "ElasticRestraint", "Elastic restraint", "Elastische Halterung")],
    ),
    "LaminationProduct": dict(
        doc="Product of the laminations of a stress-laminated deck.",
        variants=[("solid", "Solid", "Solid timber laminations", "Vollholzlamellen"), ("glulam", "Glulam", "Glulam laminations", "Brettschichtholzlamellen")],
    ),
    "PrestressBarProduct": dict(
        doc="Product of the prestress bars of a stress-laminated deck.",
        variants=[("threaded-bar", "ThreadedBar", "Threaded bar", "Gewindestab"), ("smooth-bar", "SmoothBar", "Smooth bar with upset thread", "Glatter Stab mit aufgestauchtem Gewinde")],
    ),
}


def f(name, ty, unit, en, de, example, ref=None, options=None, desc=None, enum=None):
    if enum:
        options = [(v[0], v[2], v[3]) for v in ENUMS[enum]["variants"]]
    return dict(name=name, ty=ty, unit=unit, en=en, de=de, example=example, ref=ref, options=options, desc=desc, enum=enum)


ENTITIES = [
    dict(
        plural="tapers", ent="Taper", table="taper", slug="taper", emoji="📐️", ref_kind="taper",
        doc="Variable-depth shape of one glulam member: single tapered, double tapered, curved or pitched cambered (EN 1995-1-1 6.4).",
        label_en="Tapered and curved members", label_de="Verjüngte und gekrümmte Bauteile", id_en="Shape id", id_de="Formen-Id",
        member_ref=True,
        fields=[
            f("member_id", "String", "none", "Member", "Bauteil", "beam-B1", ref="member"),
            f("kind", "String", "none", "Shape", "Form", "double-tapered", enum="TaperKind"),
            f("taper_angle_rad", "f64", "rad", "Taper angle α", "Neigungswinkel α", 0.1),
            f("apex_depth_m", "f64", "m", "Depth at the apex h_ap", "Höhe im First h_ap", 0.8),
            f("inner_radius_m", "f64", "m", "Inner radius r_in", "Innenradius r_in", 15.0),
            f("lamella_thickness_m", "f64", "m", "Lamella thickness t_lam", "Lamellendicke t_lam", 0.033),
            f("apex_angle_rad", "f64", "rad", "Roof pitch at the apex α_ap", "Dachneigung im First α_ap", 0.1),
            f("tension_side", "String", "none", "Tapered face", "Angeschnittener Rand", "tension", enum="TensionSide"),
            f("rods", "u32", "none", "Apex reinforcement rods n", "Anzahl der Firstverstärkungsstäbe n", 0),
            f("rod_resistance_n", "f64", "N", "Axial rod resistance R_ax,k", "Axialer Stabwiderstand R_ax,k", 0.0),
        ],
    ),
    dict(
        plural="cutouts", ent="Cutout", table="cutout", slug="cutout", emoji="🕳️", ref_kind="cutout",
        doc="Notch or hole in one member: net section (EN 1995-1-1 5.2) and the notch at a support (6.5.2).",
        label_en="Notches and holes", label_de="Ausklinkungen und Löcher", id_en="Cut-out id", id_de="Ausschnitt-Id",
        member_ref=True,
        fields=[
            f("member_id", "String", "none", "Member", "Bauteil", "beam-B1", ref="member"),
            f("kind", "String", "none", "Kind", "Art", "notch", enum="CutoutKind"),
            f("face", "String", "none", "Face", "Seite", "bottom", enum="CutoutFace"),
            f("distance_m", "f64", "m", "Distance x from the support", "Abstand x vom Auflager", 0.0),
            f("length_m", "f64", "m", "Notch length or hole diameter", "Ausklinkungslänge oder Lochdurchmesser", 0.15),
            f("depth_m", "f64", "m", "Notch depth", "Ausklinkungstiefe", 0.06),
            f("centre_height_m", "f64", "m", "Hole centre above the bottom", "Lochmitte über der Unterkante", 0.0),
            f("slope", "f64", "none", "Notch slope i", "Neigung i der Ausklinkung", 0.0),
            f("rods", "u32", "none", "Reinforcement rods n", "Anzahl der Verstärkungsstäbe n", 0),
            f("rod_resistance_n", "f64", "N", "Axial rod resistance R_ax,k", "Axialer Stabwiderstand R_ax,k", 0.0),
        ],
    ),
    dict(
        plural="fasteners", ent="Fastener", table="fastener", slug="fastener", emoji="🔩️", ref_kind="fastener",
        doc="Fastener pattern of one connection on a member: rows, columns, spacings and distances (EN 1995-1-1 8.3, 8.1.2).",
        label_en="Fastener patterns", label_de="Verbindungsmittelbilder", id_en="Pattern id", id_de="Bild-Id",
        member_ref=True,
        fields=[
            f("connection_id", "String", "none", "Connection", "Verbindung", "conn-C1", ref="connection"),
            f("member_id", "String", "none", "Member the pattern stands on", "Bauteil, auf dem das Bild steht", "beam-B1", ref="member"),
            f("columns", "u32", "none", "Fasteners in a row along the grain", "Verbindungsmittel je Reihe in Faserrichtung", 4),
            f("lines", "u32", "none", "Rows across the grain", "Reihen quer zur Faser", 2),
            f("spacing_parallel_m", "f64", "m", "Spacing a₁ along the grain", "Abstand a₁ in Faserrichtung", 0.08),
            f("spacing_perpendicular_m", "f64", "m", "Spacing a₂ across the grain", "Abstand a₂ quer zur Faser", 0.04),
            f("end_distance_m", "f64", "m", "End distance a₃", "Hirnholzabstand a₃", 0.10),
            f("edge_distance_m", "f64", "m", "Edge distance a₄", "Randabstand a₄", 0.04),
            f("offset_x_m", "f64", "m", "Pattern origin along the member", "Bildursprung entlang des Bauteils", 0.1),
            f("offset_y_m", "f64", "m", "Pattern origin above the bottom", "Bildursprung über der Unterkante", 0.05),
            f("load_angle_rad", "f64", "rad", "Load to grain angle α", "Winkel α zwischen Kraft und Faser", 0.0),
        ],
    ),
    dict(
        plural="panels", ent="Panel", table="panel", slug="panel", emoji="🧱️", ref_kind="panel",
        doc="Cross-laminated timber panel with its typed layers (child rows) and fire exposure (EN 1995-1-1 Annex CLT, 1-2 charring with fall-off).",
        label_en="CLT panels", label_de="BSP-Platten", id_en="Panel id", id_de="Platten-Id",
        member_ref=False,
        fields=[
            f("strength_class", "String", "none", "Strength class of the layers", "Festigkeitsklasse der Lagen", "C24", ref="strength-class"),
            f("span_m", "f64", "m", "Span", "Spannweite", 5.0),
            f("width_m", "f64", "m", "Width", "Breite", 1.0),
            f("load_pa", "f64", "Pa", "Design area load q_d", "Bemessungsflächenlast q_d", 5000.0),
            f("fire_duration_s", "f64", "s", "Fire resistance time", "Feuerwiderstandsdauer", 3600.0),
        ],
    ),
    dict(
        plural="floors", ent="Floor", table="floor", slug="floor", emoji="🏠️", ref_kind="floor",
        doc="Timber floor on its joists: stiffness per metre in both directions, mass, damping and loads for deflection, creep and vibration (EN 1995-1-1 7.2, 7.3).",
        label_en="Timber floors", label_de="Holzdecken", id_en="Floor id", id_de="Decken-Id",
        member_ref=True,
        fields=[
            f("member_id", "String", "none", "Joist member", "Deckenbalken", "beam-B1", ref="member"),
            f("span_m", "f64", "m", "Span of the joists", "Spannweite der Balken", 4.0),
            f("width_m", "f64", "m", "Floor width", "Deckenbreite", 5.0),
            f("joist_spacing_m", "f64", "m", "Joist spacing", "Balkenabstand", 0.625),
            f("ei_l_n_m2_per_m", "f64", "n_m2_per_m", "Flexural stiffness along the joists (EI)ₗ", "Biegesteifigkeit längs (EI)ₗ", 1.0e6),
            f("ei_b_n_m2_per_m", "f64", "n_m2_per_m", "Flexural stiffness across (EI)_b", "Biegesteifigkeit quer (EI)_b", 2.0e5),
            f("mass_kg_per_m2", "f64", "kg_m2", "Mass per area m", "Flächenmasse m", 150.0),
            f("damping_ratio", "f64", "none", "Modal damping ratio ζ", "Dämpfungsgrad ζ", 0.02),
            f("permanent_load_pa", "f64", "Pa", "Permanent load g_k", "Ständige Last g_k", 1000.0),
            f("imposed_load_pa", "f64", "Pa", "Imposed load q_k", "Nutzlast q_k", 2000.0),
            f("psi_2", "f64", "none", "Quasi-permanent factor ψ₂", "Quasi-ständiger Beiwert ψ₂", 0.3),
        ],
    ),
    dict(
        rows_group="structure-rows", plural="diaphragms", ent="Diaphragm", table="diaphragm", slug="diaphragm", emoji="🛡️", ref_kind="diaphragm",
        doc="Wall, floor or roof diaphragm with its sheathing panels and fastener lines: racking resistance, hold-down and shear flow (EN 1995-1-1 9.2.3, 9.2.4; catalogue entry T-94).",
        label_en="Diaphragms", label_de="Scheiben", id_en="Diaphragm id", id_de="Scheiben-Id",
        member_ref=False,
        groups=[("diaphragm-sheathing", ["kind", "method", "panel_width_m", "wall_height_m", "fastener_spacing_m", "panels"]), ("diaphragm-forces", ["fastener_capacity_n", "shear_ed_n", "hold_down_n", "diaphragm_force_n", "diaphragm_length_m", "shear_resistance_n_per_m"])],
        fields=[
            f("kind", "String", "none", "Kind", "Art", "wall", enum="DiaphragmKind"),
            f("method", "String", "none", "Racking method", "Nachweisverfahren", "method-a", enum="DiaphragmMethod"),
            f("panel_width_m", "f64", "m", "Sheathing panel width b_i", "Plattenbreite b_i", 1.2),
            f("wall_height_m", "f64", "m", "Wall height h", "Wandhöhe h", 2.4),
            f("fastener_spacing_m", "f64", "m", "Fastener spacing s along the panel edges", "Verbindungsmittelabstand s am Plattenrand", 0.075),
            f("panels", "u32", "none", "Sheathing panels n", "Anzahl der Beplankungsplatten n", 2),
            f("fastener_capacity_n", "f64", "N", "Fastener capacity F_f,Rd", "Tragfähigkeit des Verbindungsmittels F_f,Rd", 3000.0),
            f("shear_ed_n", "f64", "N", "Horizontal load on the wall F_v,Ed", "Horizontallast auf die Wand F_v,Ed", 3000.0),
            f("hold_down_n", "f64", "N", "Hold-down resistance R_hold-down", "Widerstand des Zugankers R_Zuganker", 20000.0),
            f("diaphragm_force_n", "f64", "N", "Diaphragm force F_d", "Scheibenkraft F_d", 15000.0),
            f("diaphragm_length_m", "f64", "m", "Diaphragm length l", "Scheibenlänge l", 12.0),
            f("shear_resistance_n_per_m", "f64", "n_per_m", "Shear flow resistance q_v,Rd", "Schubflusswiderstand q_v,Rd", 3000.0),
        ],
    ),
    dict(
        rows_group="structure-rows", plural="trusses", ent="Truss", table="truss", slug="truss", emoji="🔺️", ref_kind="truss",
        doc="Timber truss with parallel chords and a web pattern: members, joints and the stabilising load of the compression chord (EN 1995-1-1 9.2.5; catalogue entry T-95).",
        label_en="Trusses", label_de="Fachwerkträger", id_en="Truss id", id_de="Fachwerk-Id",
        member_ref=False,
        groups=[("truss-geometry", ["kind", "product", "span_m", "height_m", "bays", "chord_width_m", "chord_depth_m"]), ("truss-forces", ["web_width_m", "web_depth_m", "chord_force_n", "restraint_length_m", "bracing_capacity_n_per_m"])],
        fields=[
            f("kind", "String", "none", "Web pattern", "Strebenbild", "warren", enum="TrussKind"),
            f("product", "String", "none", "Product of the chord", "Produkt des Gurts", "solid", enum="BracingProduct"),
            f("span_m", "f64", "m", "Span", "Spannweite", 12.0),
            f("height_m", "f64", "m", "Height between the chord axes", "Höhe zwischen den Gurtachsen", 1.5),
            f("bays", "u32", "none", "Bays n", "Anzahl der Felder n", 6),
            f("chord_width_m", "f64", "m", "Chord width", "Gurtbreite", 0.1),
            f("chord_depth_m", "f64", "m", "Chord depth", "Gurthöhe", 0.2),
            f("web_width_m", "f64", "m", "Web member width", "Füllstabbreite", 0.08),
            f("web_depth_m", "f64", "m", "Web member depth", "Füllstabhöhe", 0.12),
            f("chord_force_n", "f64", "N", "Chord compression N_d", "Gurtdruckkraft N_d", 20000.0),
            f("restraint_length_m", "f64", "m", "Restraint length l", "Haltelänge l", 6.0),
            f("bracing_capacity_n_per_m", "f64", "n_per_m", "Bracing system capacity q_Rd", "Tragfähigkeit des Aussteifungssystems q_Rd", 1500.0),
        ],
    ),
    dict(
        rows_group="structure-rows", plural="bracings", ent="Bracing", table="bracing", slug="bracing", emoji="✖️", ref_kind="bracing",
        doc="Bracing system or restraint that stabilises a row of members: bracing force, capacity and deflection (EN 1995-1-1 9.2.5; catalogue entry T-96).",
        label_en="Bracings", label_de="Aussteifungen", id_en="Bracing id", id_de="Aussteifungs-Id",
        member_ref=True,
        groups=[("bracing-layout", ["kind", "member_id", "stabilised_count", "spacing_m", "length_m"]), ("bracing-forces", ["brace_width_m", "brace_depth_m", "compression_force_n", "resistance_n", "stiffness_n_per_m"])],
        fields=[
            f("kind", "String", "none", "Kind", "Art", "plan-bracing", enum="BracingKind"),
            f("member_id", "String", "none", "Stabilised member", "Ausgesteiftes Bauteil", "beam-B1", ref="member"),
            f("stabilised_count", "u32", "none", "Stabilised members", "Anzahl der ausgesteiften Bauteile", 4),
            f("spacing_m", "f64", "m", "Spacing of the stabilised members", "Abstand der ausgesteiften Bauteile", 0.8),
            f("length_m", "f64", "m", "Bracing length l", "Länge der Aussteifung l", 6.0),
            f("brace_width_m", "f64", "m", "Bracing member width", "Breite des Aussteifungsstabs", 0.06),
            f("brace_depth_m", "f64", "m", "Bracing member depth", "Höhe des Aussteifungsstabs", 0.12),
            f("compression_force_n", "f64", "N", "Compression force N_d", "Druckkraft N_d", 20000.0),
            f("resistance_n", "f64", "N", "Bracing resistance F_Rd", "Aussteifungswiderstand F_Rd", 2000.0),
            f("stiffness_n_per_m", "f64", "n_per_m", "Bracing stiffness k_br", "Steifigkeit der Aussteifung k_br", 500000.0),
        ],
    ),
    dict(
        rows_group="structure-rows", plural="decks", ent="Deck", table="deck", slug="deck", emoji="🌉️", ref_kind="deck",
        doc="Stress-laminated timber deck of a bridge: laminations, prestress bars with their staged losses, minimum prestress after losses, friction against slip, bar force, anchor plate bearing and the wheel load patch (EN 1995-2 stress-laminated decks; catalogue entry T-122).",
        label_en="Stress-laminated decks", label_de="Vorgespannte Brettstapeldecks", id_en="Deck id", id_de="Deck-Id",
        member_ref=False,
        groups=[("deck-geometry", ["lamination_product", "bar_product", "span_m", "deck_width_m", "deck_depth_m", "lamination_width_m", "bar_spacing_m", "bar_diameter_m", "plate_side_m"]), ("deck-prestress", ["initial_prestress_pa", "loss_relaxation", "loss_creep", "loss_moisture", "bar_resistance_pa", "shear_stress_pa", "compression_perpendicular_pa", "wheel_load_n", "wheel_contact_m"])],
        fields=[
            f("lamination_product", "String", "none", "Product of the laminations", "Produkt der Lamellen", "solid", enum="LaminationProduct"),
            f("bar_product", "String", "none", "Product of the prestress bars", "Produkt der Spannstäbe", "threaded-bar", enum="PrestressBarProduct"),
            f("span_m", "f64", "m", "Span", "Spannweite", 8.0),
            f("deck_width_m", "f64", "m", "Deck width", "Deckbreite", 4.0),
            f("deck_depth_m", "f64", "m", "Deck depth h", "Deckhöhe h", 0.25),
            f("lamination_width_m", "f64", "m", "Lamination width", "Lamellenbreite", 0.05),
            f("bar_spacing_m", "f64", "m", "Prestress bar spacing s", "Abstand s der Spannstäbe", 0.8),
            f("bar_diameter_m", "f64", "m", "Prestress bar diameter", "Durchmesser der Spannstäbe", 0.032),
            f("plate_side_m", "f64", "m", "Anchor plate side", "Seitenlänge der Ankerplatte", 0.15),
            f("initial_prestress_pa", "f64", "stress_pa", "Initial prestress σ_p,0", "Anfangsvorspannung σ_p,0", 1000000.0),
            f("loss_relaxation", "f64", "none", "Prestress loss by bar relaxation", "Vorspannverlust durch Relaxation", 0.15),
            f("loss_creep", "f64", "none", "Prestress loss by timber creep", "Vorspannverlust durch Kriechen", 0.25),
            f("loss_moisture", "f64", "none", "Prestress loss by moisture content change", "Vorspannverlust durch Holzfeuchteänderung", 0.1),
            f("bar_resistance_pa", "f64", "stress_pa", "Design bar resistance f_bar,d", "Bemessungswiderstand des Stabes f_bar,d", 700000000.0),
            f("shear_stress_pa", "f64", "stress_pa", "Shear stress to transfer τ_h", "Zu übertragende Schubspannung τ_h", 100000.0),
            f("compression_perpendicular_pa", "f64", "stress_pa", "Design compression strength across the grain f_c,90,d", "Bemessungsdruckfestigkeit quer zur Faser f_c,90,d", 1000000.0),
            f("wheel_load_n", "f64", "N", "Wheel load Q", "Radlast Q", 100000.0),
            f("wheel_contact_m", "f64", "m", "Wheel contact width a", "Aufstandsbreite a des Rades", 0.4),
        ],
    ),
]

CHILDREN = [
    dict(
        parent="panels", pent="Panel", pslug="panel", plural="layers", ent="PanelLayer", table="panel_layer", slug="panel-layer", emoji="🧱️", ref_kind="panel-layer", kind_prefix="panel-layer",
        doc="One layer of a CLT panel, listed bottom to top: thickness, grain orientation and strength grade (EN 1995-1-1 Annex CLT, layered section).",
        label_en="CLT layers", label_de="BSP-Lagen", id_en="Layer id", id_de="Lagen-Id",
        fields=[
            f("thickness_m", "f64", "m", "Layer thickness t", "Lagendicke t", 0.04),
            f("orientation", "String", "none", "Grain orientation", "Faserrichtung", "longitudinal", enum="LayerOrientation"),
            f("grade", "String", "none", "Strength grade", "Festigkeitsklasse", "C24", ref="strength-class"),
        ],
    ),
    dict(
        parent="fasteners", pent="Fastener", pslug="fastener", plural="positions", ent="FastenerPosition", table="fastener_position", slug="fastener-position", emoji="📍️", ref_kind="fastener-position", kind_prefix="fastener-position",
        doc="One explicit fastener position of an irregular pattern, from the member end along the grain and from the bottom edge; when present the positions replace the regular pattern (EN 1995-1-1 8.3).",
        label_en="Explicit fastener positions", label_de="Explizite Verbindungsmittellagen", id_en="Position id", id_de="Lagen-Id",
        fields=[
            f("x_m", "f64", "m", "Position x along the member", "Lage x entlang des Bauteils", 0.1),
            f("y_m", "f64", "m", "Position y above the bottom edge", "Lage y über der Unterkante", 0.05),
        ],
    ),
]

RS_TY = {"String": "String", "f64": "f64", "u32": "u32"}
TS_TY = {"String": "string", "f64": "number", "u32": "number"}
WIRE = {"String": "normWireString", "f64": "normWireNumber", "u32": 'normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})'}
SNAP_URL = "https://json.schemas.assets.semio-tech.com/s/norm/en1995/snapshot.json"
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/"


def camel(s):
    p = s.split("_")
    return p[0] + "".join(x[:1].upper() + x[1:] for x in p[1:])


for _e in ENTITIES:
    _e["fields"] = [dict(name="id", ty="String", unit="none", camel="id", en=_e["id_en"], de=_e["id_de"], example=_e["slug"] + "-1", ref=None, options=None, desc=None)] + _e["fields"]
    for _f in _e["fields"]:
        _f["camel"] = camel(_f["name"])
    _e["rest"] = _e["fields"][1:]
    _e["floats"] = [i + 4 for i, _f in enumerate(_e["rest"]) if _f["ty"] == "f64"]
    _e["cols"] = 4 + len(_e["rest"])
    _e["width"] = _e["cols"] + 2 * len(_e["floats"])
    _e["tbl"] = "en1995_" + _e["table"]
    _e["const"] = _e["table"].upper() + "_FLOATS"

for _c in CHILDREN:
    _c["fields"] = [dict(name="id", ty="String", unit="none", camel="id", en=_c["id_en"], de=_c["id_de"], example=_c["slug"] + "-1", ref=None, options=None, desc=None)] + _c["fields"]
    for _f in _c["fields"]:
        _f["camel"] = camel(_f["name"])
    _c["rest"] = _c["fields"][1:]
    _c["floats"] = [i + 4 for i, _f in enumerate(_c["rest"]) if _f["ty"] == "f64"]
    _c["cols"] = 4 + len(_c["rest"])
    _c["width"] = _c["cols"] + 2 * len(_c["floats"])
    _c["tbl"] = "en1995_" + _c["table"]
    _c["const"] = _c["table"].upper() + "_FLOATS"
    _c["pcamel"] = camel(_c["parent"])
