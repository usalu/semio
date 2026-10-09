import sys
path = sys.argv[1]
s = open(path, encoding='utf-8', newline='').read()
anchor = "    sch_surface_describe:"
assert s.count(anchor) == 1
assert "panel_diagnostics" not in s
block = '''    panel_diagnostics: "Diagnostics", "Diagnose";
    diag_surface_describe: "Findings of the model grouped by severity, storey and kind. Open or close a severity group to filter; activating a finding selects its elements in the plan, the 3D view and the outliner.", "Befunde des Modells, gruppiert nach Schwere, Geschoss und Art. Öffnen oder schließen Sie eine Gruppe, um zu filtern; ein Befund wählt seine Bauteile im Grundriss, in der 3D-Ansicht und in der Struktur aus.";
    diag_errors: "Errors", "Fehler";
    diag_warnings: "Warnings", "Warnungen";
    diag_notes: "Notes", "Hinweise";
    diag_error: "Error", "Fehler";
    diag_warning: "Warning", "Warnung";
    diag_note: "Note", "Hinweis";
    diag_model_wide: "Whole model", "Gesamtes Modell";
    diag_empty: "No problems found", "Keine Probleme gefunden";
    diag_cat_annotation: "Annotations", "Beschriftungen";
    diag_cat_ceiling: "Ceilings", "Unterdecken";
    diag_cat_clash: "Clashes", "Kollisionen";
    diag_cat_degenerate: "Degenerate geometry", "Entartete Geometrie";
    diag_cat_opening: "Openings", "Öffnungen";
    diag_cat_railing: "Railings", "Geländer";
    diag_cat_ramp: "Ramps", "Rampen";
    diag_cat_reference: "References", "Verweise";
    diag_cat_roof: "Roofs", "Dächer";
    diag_cat_space: "Spaces", "Räume";
    diag_cat_stair: "Stairs", "Treppen";
    diag_cat_storey: "Storeys", "Geschosse";
    status_errors: "{count} errors", "{count} Fehler";
    status_warnings: "{count} warnings", "{count} Warnungen";
    status_notes: "{count} notes", "{count} Hinweise";
    status_no_problems: "No problems", "Keine Probleme";
    cmd_select_findings: "Select Findings", "Befunde auswählen";
    cmd_select_findings_describe: "Selects the elements a finding of the diagnostics names, in the plan, the 3D view and the outliner.", "Wählt die Bauteile, die ein Befund der Diagnose nennt, im Grundriss, in der 3D-Ansicht und in der Struktur aus.";
    fault_diagnostic_target_missing: "None of the elements the finding names exists any more.", "Keines der Bauteile, die der Befund nennt, gibt es noch.";
    cmd_set_classification: "Set Classification", "Klassifizierung setzen";
    cmd_set_classification_describe: "Classifies the given elements, or the selection, with a system, a code and a title.", "Klassifiziert die angegebenen Bauteile oder die Auswahl mit System, Code und Titel.";
    cmd_remove_classification: "Remove Classification", "Klassifizierung entfernen";
    cmd_remove_classification_describe: "Removes the classification of the given elements, or of the selection.", "Entfernt die Klassifizierung der angegebenen Bauteile oder der Auswahl.";
    arg_classification_system: "Classification system", "Klassifizierungssystem";
    arg_classification_code: "Classification code", "Klassifizierungscode";
    arg_classification_title: "Classification title", "Klassifizierungstitel";
    fault_classification_target_missing: "Select the elements to classify.", "Wählen Sie die zu klassifizierenden Bauteile aus.";
    fault_classification_invalid: "A classification needs a system and a code.", "Eine Klassifizierung braucht ein System und einen Code.";
    fault_classification_missing: "None of these elements is classified.", "Keines dieser Bauteile ist klassifiziert.";
'''
s = s.replace(anchor, block + anchor)
open(path, 'w', encoding='utf-8', newline='').write(s)
