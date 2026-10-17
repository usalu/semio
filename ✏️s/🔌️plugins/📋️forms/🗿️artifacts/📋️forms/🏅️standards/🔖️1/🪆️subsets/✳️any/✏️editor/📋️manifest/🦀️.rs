//! 📋️ Forms manifest: the app manifest stitch of `create_forms_app`.

use super::*;

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
///
/// 🚧️ SDK GAP (contract §2.4, w0-f-report Gap 4): `EditorBuilder` has no `.example(...)`/`.workflow(...)`
/// methods — the three example registrations (`default`/`onboarding`/`building-component`) and the
/// `"forms"` workflow tag the pre-migration `App`-based manifest carried are dropped here, not ported.
/// The subset's own `📚️examples/🎬️demo` facet is the likely intended replacement mechanism; flagged
/// for the coordinator, not fixed locally.
pub fn create_forms_app() -> AppDefinition {
    Editor::builder(crate::FORMS_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
            .document(["semio", "forms"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("forms")
            .mode_def(blueprint::definition())
            .mode_def(fill::definition())
            .mode_def(responses::definition())
            .named_layout(fill::layout())
            .named_layout(responses::layout())
            .default_mode_id(blueprint::FORMS_PLAY_MODE_BLUEPRINT)
            .window_kind_def(builder::definition())
            .window_kind_def(try_window::definition())
            .window_kind_def(responses::results::definition())
            .default_layout(blueprint::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .mutation("addBlock", LocalizedLabel::native("Add Question", "Frage hinzufügen"))
            .mutation("removeBlock", LocalizedLabel::native("Remove Question", "Frage entfernen"))
            .mutation("patchQuestions", LocalizedLabel::native("Patch Questions", "Fragen aktualisieren"))
            .mutation("patchQuestionOptions", LocalizedLabel::native("Patch Question Options", "Fragenoptionen aktualisieren"))
            .mutation("addQuestionOption", LocalizedLabel::native("Add Question Option", "Fragenoption hinzufügen"))
            .mutation("removeQuestionOption", LocalizedLabel::native("Remove Question Option", "Fragenoption entfernen"))
            .mutation("patchVectorField", LocalizedLabel::native("Patch Vector Field", "Vektorfeld aktualisieren"))
            .mutation("addVectorField", LocalizedLabel::native("Add Vector Field", "Vektorfeld hinzufügen"))
            .mutation("removeVectorField", LocalizedLabel::native("Remove Vector Field", "Vektorfeld entfernen"))
            .mutation("moveBlock", LocalizedLabel::native("Move Question", "Frage verschieben"))
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .mutation("patchStep", LocalizedLabel::native("Patch Step", "Schritt aktualisieren"))
            .mutation("updateForm", LocalizedLabel::native("Update Form", "Formular aktualisieren"))
            .mutation("dropQuestionKind", LocalizedLabel::native("Drop Question Kind", "Frageart ablegen"))
            .action_audience("dropQuestionKind", semio_framework_plugin::CapabilityAudience::Agent)
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Use Form Template", "Formularvorlage verwenden"), ActionKind::Mutation, "panel-left"))
            .mutation("setSpecJson", LocalizedLabel::native("Import Form Design JSON", "Formularentwurf aus JSON importieren"))
            .view_action("setTryValue", LocalizedLabel::native("Set Try Value", "Testwert festlegen"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(set_try_value::SET_TRY_VALUE_STEP_ACTION_ID, LocalizedLabel::native("Set Try Value Step", "Testwert-Schritt festlegen"), ActionKind::View) })
            .action_interactive_job("setTryValue", InteractiveJobClassification::Migrated)
            .action_interactive_job(set_try_value::SET_TRY_VALUE_STEP_ACTION_ID, InteractiveJobClassification::Migrated)
            .view_action("setTryValues", LocalizedLabel::native("Set Try Values", "Testwerte festlegen"))
            .view_action("resetTry", LocalizedLabel::native("Reset Try", "Test zurücksetzen"))
            .view_action("previousStep", LocalizedLabel::native("Previous Step", "Vorheriger Schritt"))
            .view_action("nextStep", LocalizedLabel::native("Next Step", "Nächster Schritt"))
            .mutation("submit", LocalizedLabel::native("Submit", "Absenden"))
            .shell_action("exportSnapshot", LocalizedLabel::native("Export Form", "Formular exportieren"))
            .shell_action("exportResponses", LocalizedLabel::native("Export Responses", "Antworten exportieren"))
            .mutation("discardResponse", LocalizedLabel::native("Remove Response", "Antwort entfernen"))
            .action_args("exportResponses", vec![ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("json", LocalizedLabel::data("JSON")), ActionArgOption::new("csv", LocalizedLabel::data("CSV"))]).default_value(&"json")])
            .action_describe("exportResponses", LocalizedLabel::native("Downloads stored submissions with their original labels and typed answer values.", "Lädt gespeicherte Antworten mit ihren ursprünglichen Bezeichnungen und typisierten Werten herunter."))
            .action_describe("discardResponse", LocalizedLabel::native("Removes one saved response. Undo restores it.", "Entfernt eine gespeicherte Antwort. Rückgängig stellt sie wieder her."))
            // 💬️ Agent-facing descriptions (ticket 26/09/18 slice M5a) — EN first, DE second.
            .action_describe("addStep", LocalizedLabel::native("Appends a new step (a page of questions) to the form.", "Fügt dem Formular einen neuen Schritt (eine Seite mit Fragen) hinzu."))
            .action_use_when("addStep", vec!["add a page to the form".into(), "new step".into()])
            .action_describe("patchStep", LocalizedLabel::native("Changes a page title or description.", "Ändert den Titel oder die Beschreibung einer Seite."))
            .action_describe("removeStep", LocalizedLabel::native("Removes one step and every question on it from the form.", "Entfernt einen Schritt samt allen darauf liegenden Fragen aus dem Formular."))
            .action_describe("moveStep", LocalizedLabel::native("Reorders one step within the form.", "Ordnet einen Schritt im Formular um."))
            .action_describe("addBlock", LocalizedLabel::native("Adds a new question of the given kind to a step of the form.", "Fügt einem Schritt des Formulars eine neue Frage der angegebenen Art hinzu."))
            .action_use_when("addBlock", vec!["add a question".into(), "add a multiple choice field".into(), "add a text input to the form".into()])
            .action_describe("removeBlock", LocalizedLabel::native("Removes one question from the form by id.", "Entfernt eine Frage anhand ihrer Id aus dem Formular."))
            .action_describe("patchQuestions", LocalizedLabel::native("Sets one named property on one or more questions — label, help text, required flag, kind or validation.", "Setzt eine benannte Eigenschaft auf einer oder mehreren Fragen — Beschriftung, Hilfetext, Pflichtfeld, Art oder Validierung."))
            .action_use_when("patchQuestions", vec!["rename a question".into(), "make this question required".into()])
            .action_describe("moveBlock", LocalizedLabel::native("Reorders one question, within its step or onto another one.", "Ordnet eine Frage um — innerhalb ihres Schritts oder auf einen anderen."))
            .action_describe("dropQuestionKind", LocalizedLabel::native("Creates a question of the given kind at a drop target in the form outline.", "Erzeugt eine Frage der angegebenen Art an einer Ablagestelle in der Formularübersicht."))
            .action_describe("addQuestionOption", LocalizedLabel::native("Adds one selectable option to a choice question.", "Fügt einer Auswahlfrage eine auswählbare Option hinzu."))
            .action_describe("removeQuestionOption", LocalizedLabel::native("Removes one selectable option from a choice question.", "Entfernt eine auswählbare Option aus einer Auswahlfrage."))
            .action_describe("patchQuestionOptions", LocalizedLabel::native("Sets one named property on a choice question's options.", "Setzt eine benannte Eigenschaft auf den Optionen einer Auswahlfrage."))
            .action_describe("addVectorField", LocalizedLabel::native("Adds one component field to a vector-valued question.", "Fügt einer vektorwertigen Frage ein Komponentenfeld hinzu."))
            .action_describe("removeVectorField", LocalizedLabel::native("Removes one component field from a vector-valued question.", "Entfernt ein Komponentenfeld aus einer vektorwertigen Frage."))
            .action_describe("patchVectorField", LocalizedLabel::native("Sets one named property of a vector-valued question's component field.", "Setzt eine benannte Eigenschaft eines Komponentenfelds einer vektorwertigen Frage."))
            .action_describe("updateForm", LocalizedLabel::native("Changes the form title.", "Ändert den Formulartitel."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the form design with a template. Existing answers stay with this form.", "Ersetzt den Formularentwurf durch eine Vorlage. Vorhandene Antworten bleiben in diesem Formular."))
            .action_describe("setSpecJson", LocalizedLabel::native("Imports the title and questions from a saved Forms JSON document. Existing answers stay with this form.", "Importiert Titel und Fragen aus einem gespeicherten Forms-JSON-Dokument. Vorhandene Antworten bleiben in diesem Formular."))
            .action_describe("exportSnapshot", LocalizedLabel::native("Downloads a Forms document containing its design and saved answers.", "Lädt ein Forms-Dokument mit Entwurf und gespeicherten Antworten herunter."))
            .action_describe("submit", LocalizedLabel::native("Submits the answers currently entered in the form preview.", "Sendet die aktuell in der Formularvorschau eingegebenen Antworten ab."))
            // ⚠️ Discards content no later verb reconstructs — the gateway asks a human first.
            .action_destructive("removeBlock")
            .action_destructive("discardResponse")
            .action_destructive("removeQuestionOption")
            .action_destructive("removeStep")
            .action_destructive("removeVectorField")
            .action_destructive("setActiveExample")
            .action_destructive("setSpecJson")
            // 🖱️ Preview-runner input plumbing — the form preview feeds these, agents never do.
            .action_audience("setTryValue", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience(set_try_value::SET_TRY_VALUE_STEP_ACTION_ID, semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("setTryValues", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("resetTry", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("previousStep", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("nextStep", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_interactive_job("setTryValues", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetTry", InteractiveJobClassification::Migrated)
            .action_interactive_job("previousStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("nextStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("submit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
            .action_interactive_job("addStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("updateForm", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBlock", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeBlock", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchQuestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchQuestionOptions", InteractiveJobClassification::Migrated)
            .action_interactive_job("addQuestionOption", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeQuestionOption", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchVectorField", InteractiveJobClassification::Migrated)
            .action_interactive_job("addVectorField", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeVectorField", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveBlock", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropQuestionKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSpecJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportResponses", InteractiveJobClassification::Migrated)
            .action_interactive_job("discardResponse", InteractiveJobClassification::Migrated)
            // 📝️ Staged argument forms for the panel-visible create/switch actions.
            .action_args("addBlock", vec![
                ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value(&"text"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Template", "Vorlage"), vec![
                    ActionArgOption::new("", LocalizedLabel::native("Blank Form", "Leeres Formular")),
                    ActionArgOption::new("default", LocalizedLabel::native("Contact", "Kontakt")),
                    ActionArgOption::new("onboarding", LocalizedLabel::native("Onboarding", "Einführung")),
                    ActionArgOption::new("building-component", LocalizedLabel::native("Building Component", "Baukomponente")),
                ]).default_value(&"default"),
            ])
            .action_args("setSpecJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Spec JSON", "Spezifikations-JSON"))])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "fields" interaction
            // domain — two granularities ("field" default, "section"), `HierarchyProvider::Topology`
            // from the document's own step/question nesting (`forms_fields_topology`/
            // `FormsPlayApp::interaction_topology`), both hover and selection transitive (selecting/
            // hovering a step covers its questions). Selection is pick-only (no canvas marquee surface
            // exists for this domain today); the framework auto-injects interactionSelect/
            // interactionHover/clearSelection/selectAll/setSelectionMode/setInteractionGranularity.
            .interaction(InteractionDefinition {
                id: FORMS_INTERACTION_FIELDS.into(),
                label: LocalizedLabel::native("Fields", "Felder"),
                granularities: vec![
                    GranularityDefinition { id: FORMS_INTERACTION_GRANULARITY_FIELD.into(), label: LocalizedLabel::native("Field", "Feld"), icon_id: "help-circle".into() },
                    GranularityDefinition { id: FORMS_INTERACTION_GRANULARITY_SECTION.into(), label: LocalizedLabel::native("Section", "Abschnitt"), icon_id: "list-tree".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: true,
                    broadcast: true,
                },
            })
            .window_kind_interactions(builder::FORMS_PLAY_WINDOW_BLUEPRINT, vec![InteractionRef::new(FORMS_INTERACTION_FIELDS)])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`forms_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .config(FormsPlayApp::config_spec())
            .io(forms_io())
            .build_definition()
}
//#endregion 🔖️Manifest
