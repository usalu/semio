//! 🗣️ Fem2d play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! (panels, windows, commands) reaches for. One struct for the whole app: the artifact tree, the
//! inspector and the results/playback panel all resolve their nouns, field names and verbs here, so
//! a term is spelled once per locale and never drifts between two panels showing the same entity.

use semio_framework_plugin::{Locale, ViewModel};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the fem2d app; one field per label makes every locale combination compile-checked.
    pub struct Fem2dLabels {
        artifact: native_en "Artifact", native_de "Artefakt", reuse_en "Artifact", reuse_de "Artefakt";
        inspection: native_en "Inspection", native_de "Inspektion", reuse_en "Inspection", reuse_de "Inspektion";
        results: native_en "Results", native_de "Ergebnisse", reuse_en "Results", reuse_de "Ergebnisse";
        analysis: native_en "Analysis", native_de "Analyse", reuse_en "Analysis", reuse_de "Analyse";
        summary: native_en "Summary", native_de "Übersicht", reuse_en "Summary", reuse_de "Übersicht";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        pending: native_en "Pending", native_de "In Arbeit", reuse_en "Pending", reuse_de "In Arbeit";
        selected: native_en "Selected", native_de "Ausgewählt", reuse_en "Selected", reuse_de "Ausgewählt";
        add_term: native_en "Add Term", native_de "Term hinzufügen", reuse_en "Add Term", reuse_de "Term hinzufügen";

        node: native_en "Node", native_de "Knoten", reuse_en "Node", reuse_de "Knoten";
        element: native_en "Element", native_de "Element", reuse_en "Element", reuse_de "Element";
        region: native_en "Region", native_de "Bereich", reuse_en "Region", reuse_de "Bereich";
        support: native_en "Support", native_de "Lager", reuse_en "Support", reuse_de "Lager";
        load: native_en "Load", native_de "Last", reuse_en "Load", reuse_de "Last";
        material: native_en "Material", native_de "Material", reuse_en "Material", reuse_de "Material";
        section: native_en "Section", native_de "Querschnitt", reuse_en "Section", reuse_de "Querschnitt";
        load_case: native_en "Load Case", native_de "Lastfall", reuse_en "Load Case", reuse_de "Lastfall";
        combination: native_en "Combination", native_de "Kombination", reuse_en "Combination", reuse_de "Kombination";
        term: native_en "Term", native_de "Term", reuse_en "Term", reuse_de "Term";

        nodes: native_en "Nodes", native_de "Knoten", reuse_en "Nodes", reuse_de "Knoten";
        elements: native_en "Elements", native_de "Elemente", reuse_en "Elements", reuse_de "Elemente";
        regions: native_en "Regions", native_de "Bereiche", reuse_en "Regions", reuse_de "Bereiche";
        supports: native_en "Supports", native_de "Lager", reuse_en "Supports", reuse_de "Lager";
        loads: native_en "Loads", native_de "Lasten", reuse_en "Loads", reuse_de "Lasten";
        materials: native_en "Materials", native_de "Materialien", reuse_en "Materials", reuse_de "Materialien";
        sections: native_en "Sections", native_de "Querschnitte", reuse_en "Sections", reuse_de "Querschnitte";
        load_cases: native_en "Load Cases", native_de "Lastfälle", reuse_en "Load Cases", reuse_de "Lastfälle";
        combinations: native_en "Combinations", native_de "Kombinationen", reuse_en "Combinations", reuse_de "Kombinationen";
        terms: native_en "Terms", native_de "Terme", reuse_en "Terms", reuse_de "Terme";

        bar: native_en "Bar", native_de "Stab", reuse_en "Bar", reuse_de "Stab";
        beam: native_en "Beam", native_de "Balken", reuse_en "Beam", reuse_de "Balken";

        id: native_en "ID", native_de "ID", reuse_en "ID", reuse_de "ID";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        start: native_en "Start", native_de "Start", reuse_en "Start", reuse_de "Start";
        end: native_en "End", native_de "Ende", reuse_en "End", reuse_de "Ende";
        youngs_modulus: native_en "Young's Modulus", native_de "Elastizitätsmodul", reuse_en "Young's Modulus", reuse_de "Elastizitätsmodul";
        poisson_ratio: native_en "Poisson Ratio", native_de "Querdehnzahl", reuse_en "Poisson Ratio", reuse_de "Querdehnzahl";
        density: native_en "Density", native_de "Dichte", reuse_en "Density", reuse_de "Dichte";
        area: native_en "Area", native_de "Fläche", reuse_en "Area", reuse_de "Fläche";
        second_moment_of_area: native_en "Second Moment of Area", native_de "Flächenträgheitsmoment", reuse_en "Second Moment of Area", reuse_de "Flächenträgheitsmoment";
        fixed: native_en "Fixed", native_de "Gesperrt", reuse_en "Fixed", reuse_de "Gesperrt";
        tx: native_en "Tx", native_de "Tx", reuse_en "Tx", reuse_de "Tx";
        ty: native_en "Ty", native_de "Ty", reuse_en "Ty", reuse_de "Ty";
        rz: native_en "Rz", native_de "Rz", reuse_en "Rz", reuse_de "Rz";
        dof: native_en "Degree of Freedom", native_de "Freiheitsgrad", reuse_en "Degree of Freedom", reuse_de "Freiheitsgrad";
        value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        wx: native_en "wx", native_de "wx", reuse_en "wx", reuse_de "wx";
        wy: native_en "wy", native_de "wy", reuse_en "wy", reuse_de "wy";
        pressure: native_en "Pressure", native_de "Druck", reuse_en "Pressure", reuse_de "Druck";
        thickness: native_en "Thickness", native_de "Dicke", reuse_en "Thickness", reuse_de "Dicke";
        mesh_size: native_en "Mesh Size", native_de "Netzweite", reuse_en "Mesh Size", reuse_de "Netzweite";
        outline: native_en "Outline", native_de "Umriss", reuse_en "Outline", reuse_de "Umriss";
        holes: native_en "Holes", native_de "Löcher", reuse_en "Holes", reuse_de "Löcher";
        self_weight: native_en "Self Weight", native_de "Eigengewicht", reuse_en "Self Weight", reuse_de "Eigengewicht";
        factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        points: native_en "pts", native_de "Pkt.", reuse_en "pts", reuse_de "Pkt.";

        modal_count: native_en "Modal Count", native_de "Anzahl Eigenformen", reuse_en "Modal Count", reuse_de "Anzahl Eigenformen";
        buckling_count: native_en "Buckling Count", native_de "Anzahl Knickformen", reuse_en "Buckling Count", reuse_de "Anzahl Knickformen";
        deformation_scale: native_en "Deformation Scale", native_de "Verformungsfaktor", reuse_en "Deformation Scale", reuse_de "Verformungsfaktor";

        display: native_en "Display", native_de "Anzeige", reuse_en "Display", reuse_de "Anzeige";
        playback: native_en "Playback", native_de "Wiedergabe", reuse_en "Playback", reuse_de "Wiedergabe";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        mode: native_en "Mode", native_de "Modus", reuse_en "Mode", reuse_de "Modus";
        static_mode: native_en "Static", native_de "Statisch", reuse_en "Static", reuse_de "Statisch";
        modal: native_en "Modal", native_de "Modal", reuse_en "Modal", reuse_de "Modal";
        buckling: native_en "Buckling", native_de "Knicken", reuse_en "Buckling", reuse_de "Knicken";
        mode_index: native_en "Mode Index", native_de "Modenindex", reuse_en "Mode Index", reuse_de "Modenindex";
        phase: native_en "Phase", native_de "Phase", reuse_en "Phase", reuse_de "Phase";
        playing: native_en "Playing", native_de "Läuft", reuse_en "Playing", reuse_de "Läuft";
        play: native_en "Play", native_de "Abspielen", reuse_en "Play", reuse_de "Abspielen";
        pause: native_en "Pause", native_de "Pause", reuse_en "Pause", reuse_de "Pause";
        speed: native_en "Speed", native_de "Geschwindigkeit", reuse_en "Speed", reuse_de "Geschwindigkeit";
        loop_mode: native_en "Loop", native_de "Schleife", reuse_en "Loop", reuse_de "Schleife";
        ping_pong: native_en "Ping-Pong", native_de "Ping-Pong", reuse_en "Ping-Pong", reuse_de "Ping-Pong";
        once: native_en "Once", native_de "Einmal", reuse_en "Once", reuse_de "Einmal";
        waveform: native_en "Waveform", native_de "Wellenform", reuse_en "Waveform", reuse_de "Wellenform";
        ramp: native_en "Ramp", native_de "Rampe", reuse_en "Ramp", reuse_de "Rampe";
        sine: native_en "Sine", native_de "Sinus", reuse_en "Sine", reuse_de "Sinus";
        step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        frequency: native_en "Frequency", native_de "Frequenz", reuse_en "Frequency", reuse_de "Frequenz";
        load_factor: native_en "Load Factor", native_de "Lastfaktor", reuse_en "Load Factor", reuse_de "Lastfaktor";

        focus: native_en "Focus", native_de "Fokussieren", reuse_en "Focus", reuse_de "Fokussieren";
        delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
    }
}

/// 🇩🇪️ Label fields whose German spelling IS the English one — a symbol (`X`, `Tx`, `wx`), an
/// international term (`Modal`, `Ping-Pong`, `Schema`) or a word German borrowed outright
/// (`Element`, `Material`, `Name`, `Start`, `Pause`, `Phase`, `Term`). Everything NOT named here must
/// differ, which is what turns "somebody pasted the English into `native_de`" into a failing test.
pub const FEM2D_LABELS_IDENTICAL_BY_DESIGN: &[&str] = &["schema", "element", "material", "term", "id", "name", "x", "y", "start", "tx", "ty", "rz", "wx", "wy", "modal", "phase", "pause", "ping_pong"];
//#endregion 🔖️Labels

//#region 🔖️Resolvers
pub fn fem2d_is_de_locale(view_state: &ViewModel) -> bool {
    view_state.locale == Locale::De
}

/// 🗣️ Resolves the active fem2d label set from the OS-owned view context.
pub fn fem2d_labels(view_state: &ViewModel) -> &'static Fem2dLabels {
    semio_framework_plugin::resolve_labels::<Fem2dLabels>(view_state)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
