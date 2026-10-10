#!/usr/bin/env bun
/**
 * 🗣️ Package `w2-f3-editor`: inserts the en+de labels of the components and MEP editor (entity kinds, fields, MEP systems and sections, the override rows, the family browser, the place and route
 * utilities, their commands and the refusals) into the one `app_labels!` block of the BIM editor terminology, each block directly after a stable anchor row; rows that already exist are left
 * alone, so the script is idempotent and never rewrites a row.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { subset, child, RS } from "./r3-f1-paths.ts";

const file = join(child(child(subset, "editor"), "terminology"), RS);
const row = (field: string, en: string, de: string) => `    ${field}: ${JSON.stringify(en)}, ${JSON.stringify(de)};`;

const blocks: [anchor: string, rows: string[]][] = [
  ["    panel_library:", [
    row("panel_family_browser", "Family browser", "Familienbrowser"),
  ]],
  ["    kind_wall_sweep:", [
    row("kind_component", "Component", "Komponente"),
    row("kind_component_override", "Parameter override", "Parameter-Abweichung"),
    row("kind_mep_element", "MEP element", "TGA-Element"),
  ]],
  ["    group_wall_sweeps:", [
    row("group_components", "Components", "Komponenten"),
    row("group_component_overrides", "Parameter overrides", "Parameter-Abweichungen"),
    row("group_mep_elements", "MEP elements", "TGA-Elemente"),
  ]],
  ["    field_curtain_wall_type:", [
    row("field_family", "Family", "Familie"),
    row("field_mirrored", "Mirrored", "Gespiegelt"),
    row("field_system", "System", "System"),
    row("field_rotation_degrees", "Rotation (°)", "Drehung (°)"),
    row("field_cross_section", "Cross-section", "Querschnitt"),
    row("field_component", "Component", "Komponente"),
    row("field_parameter", "Parameter", "Parameter"),
    row("field_overrides", "Overrides", "Abweichungen"),
    row("sys_supply", "Supply air", "Zuluft"),
    row("sys_return", "Return air", "Abluft"),
    row("sys_exhaust", "Exhaust air", "Fortluft"),
    row("sys_domestic_water", "Domestic water", "Trinkwasser"),
    row("sys_waste", "Waste water", "Abwasser"),
    row("sys_gas", "Gas", "Gas"),
    row("sys_power", "Power", "Strom"),
    row("sys_data", "Data", "Daten"),
    row("sys_lighting", "Lighting circuit", "Beleuchtungskreis"),
    row("mep_duct", "Duct", "Luftkanal"),
    row("mep_pipe", "Pipe", "Rohr"),
    row("mep_tray", "Cable tray", "Kabeltrasse"),
    row("section_overrides", "Parameter overrides", "Parameter-Abweichungen"),
    row("override_inherited", "from the family", "von der Familie"),
    row("override_overridden", "overridden", "abweichend"),
    row("override_input_named", "Formula of {name} for this component", "Formel von {name} für diese Komponente"),
    row("override_reset_named", "Reset {name} to the family", "{name} auf die Familie zurücksetzen"),
    row("browser_search", "Search families", "Familien suchen"),
    row("browser_hits", "Matches", "Treffer"),
    row("browser_none", "The model has no family to place; add one in the library.", "Das Modell hat keine Familie zum Platzieren; legen Sie in der Bibliothek eine an."),
    row("browser_place_named", "Place {name}", "{name} platzieren"),
    row("browser_edit_named", "Open {name} in the family editor", "{name} im Familieneditor öffnen"),
    row("browser_selected", "Selected family", "Gewählte Familie"),
    row("browser_size", "Size (width × depth × height)", "Größe (Breite × Tiefe × Höhe)"),
    row("browser_surface_describe", "Browser of the families you can place, by category: search by name, select one to read its size and volume, place it with the component tool or open it in the family editor.", "Browser der platzierbaren Familien nach Kategorie: nach Namen suchen, eine auswählen, um Größe und Volumen zu lesen, sie mit dem Komponentenwerkzeug platzieren oder im Familieneditor öffnen."),
  ]],
  ["    utility_ramp:", [
    row("utility_component", "Component", "Komponente"),
    row("utility_route", "MEP route", "TGA-Leitung"),
  ]],
  ["    arg_key:", [
    row("arg_family", "Family", "Familie"),
    row("arg_component", "Component", "Komponente"),
  ]],
  ["    cmd_arm_ramp_describe:", [
    row("cmd_arm_component", "Component Tool", "Komponentenwerkzeug"),
    row("cmd_arm_component_describe", "Arms the component utility in the addressed window (Shift+C): a click places the selected family of the library, a basin or a luminaire clings to the wall beside the pointer (hold Ctrl to place it free-standing); Alt+R turns, Alt+M mirrors, Tab changes the family, Alt+T makes it a terminal of a system, Alt+PageUp and Alt+PageDown change its elevation; the tool goes on placing until Escape.", "Aktiviert das Komponentenwerkzeug im adressierten Fenster (Umschalt+C): Ein Klick platziert die gewählte Familie der Bibliothek, ein Waschbecken oder eine Leuchte haftet an der Wand neben dem Zeiger (mit Strg frei stehend); Alt+R dreht, Alt+M spiegelt, Tab wechselt die Familie, Alt+T macht sie zum Anschluss eines Systems, Alt+Bild auf und Alt+Bild ab ändern die Höhenlage; das Werkzeug platziert weiter bis Escape."),
    row("cmd_arm_route", "MEP Route Tool", "TGA-Leitungswerkzeug"),
    row("cmd_arm_route_describe", "Arms the MEP route utility in the addressed window (Shift+M): each click adds a vertex of a duct, pipe or cable tray at the current elevation, Enter writes the element, Backspace takes the last vertex back, Escape cancels; Tab changes the section kind, Alt+T the system, Alt+PageUp and Alt+PageDown the elevation of the next vertices.", "Aktiviert das TGA-Leitungswerkzeug im adressierten Fenster (Umschalt+M): Jeder Klick fügt einen Eckpunkt eines Luftkanals, Rohrs oder einer Kabeltrasse auf der aktuellen Höhe hinzu, Enter schreibt das Element, Rücktaste nimmt den letzten Punkt zurück, Escape bricht ab; Tab wechselt die Querschnittsart, Alt+T das System, Alt+Bild auf und Alt+Bild ab die Höhe der nächsten Punkte."),
  ]],
  ["    cmd_edit_schedule_describe:", [
    row("cmd_place_component", "Place Family", "Familie platzieren"),
    row("cmd_place_component_describe", "Selects the family in the library and arms the component utility in the addressed window, so the next click places an instance of it.", "Wählt die Familie in der Bibliothek und aktiviert das Komponentenwerkzeug im adressierten Fenster, sodass der nächste Klick eine Instanz von ihr platziert."),
    row("cmd_open_family", "Open Family", "Familie öffnen"),
    row("cmd_open_family_describe", "Selects the family in the library and opens the family editor, which then shows its parameters and solids.", "Wählt die Familie in der Bibliothek und öffnet den Familieneditor, der dann ihre Parameter und Körper zeigt."),
    row("cmd_search_families", "Search Families", "Familien suchen"),
    row("cmd_search_families_describe", "Selects the placeable families whose name or category contains the text, without regard to case, in the library; an empty text clears the search.", "Wählt die platzierbaren Familien, deren Name oder Kategorie den Text enthält, ohne Rücksicht auf Groß- und Kleinschreibung, in der Bibliothek; ein leerer Text hebt die Suche auf."),
    row("cmd_set_override", "Set Parameter Override", "Parameter-Abweichung festlegen"),
    row("cmd_set_override_describe", "Gives one family parameter of a component its own formula as one undoable step; an empty formula, or the formula of the family itself, takes the override away again; a formula that does not parse, names an unknown parameter or closes a circle is refused.", "Gibt einem Familienparameter einer Komponente eine eigene Formel als einen rückgängig zu machenden Schritt; eine leere Formel oder die Formel der Familie selbst nimmt die Abweichung wieder weg; eine Formel, die sich nicht lesen lässt, einen unbekannten Parameter nennt oder einen Zirkelbezug schließt, wird abgelehnt."),
    row("cmd_gesture_turn", "Turn Tool Forward", "Werkzeug vorwärts drehen"),
    row("cmd_gesture_turn_describe", "Turns the ghost of the component tool by 15 degrees counter-clockwise (Alt+R).", "Dreht die Vorschau des Komponentenwerkzeugs um 15 Grad gegen den Uhrzeigersinn (Alt+R)."),
    row("cmd_gesture_turn_back", "Turn Tool Back", "Werkzeug rückwärts drehen"),
    row("cmd_gesture_turn_back_describe", "Turns the ghost of the component tool by 15 degrees clockwise (Alt+Shift+R).", "Dreht die Vorschau des Komponentenwerkzeugs um 15 Grad im Uhrzeigersinn (Alt+Umschalt+R)."),
    row("cmd_gesture_quarter", "Quarter Turn", "Vierteldrehung"),
    row("cmd_gesture_quarter_describe", "Turns the ghost of the component tool by a quarter turn counter-clockwise (Ctrl+Alt+R).", "Dreht die Vorschau des Komponentenwerkzeugs um eine Vierteldrehung gegen den Uhrzeigersinn (Strg+Alt+R)."),
    row("cmd_gesture_mirror", "Mirror Tool", "Werkzeug spiegeln"),
    row("cmd_gesture_mirror_describe", "Mirrors the ghost of the component tool left to right (Alt+M).", "Spiegelt die Vorschau des Komponentenwerkzeugs von links nach rechts (Alt+M)."),
    row("cmd_gesture_next", "Next Family or Section", "Nächste Familie oder Querschnittsart"),
    row("cmd_gesture_next_describe", "Cycles the component tool to the next placeable family and the MEP route tool to the next kind of section (Tab).", "Schaltet das Komponentenwerkzeug zur nächsten platzierbaren Familie und das TGA-Leitungswerkzeug zur nächsten Querschnittsart weiter (Tab)."),
    row("cmd_gesture_previous", "Previous Family or Section", "Vorherige Familie oder Querschnittsart"),
    row("cmd_gesture_previous_describe", "Cycles the component tool to the previous placeable family and the MEP route tool to the previous kind of section (Shift+Tab).", "Schaltet das Komponentenwerkzeug zur vorherigen platzierbaren Familie und das TGA-Leitungswerkzeug zur vorherigen Querschnittsart zurück (Umschalt+Tab)."),
    row("cmd_gesture_raise", "Raise Elevation", "Höhenlage erhöhen"),
    row("cmd_gesture_raise_describe", "Raises the elevation the component or MEP route tool writes by a tenth of a metre (Alt+PageUp).", "Erhöht die Höhenlage, die das Komponenten- oder TGA-Leitungswerkzeug schreibt, um einen Dezimeter (Alt+Bild auf)."),
    row("cmd_gesture_lower", "Lower Elevation", "Höhenlage verringern"),
    row("cmd_gesture_lower_describe", "Lowers the elevation the component or MEP route tool writes by a tenth of a metre (Alt+PageDown).", "Verringert die Höhenlage, die das Komponenten- oder TGA-Leitungswerkzeug schreibt, um einen Dezimeter (Alt+Bild ab)."),
    row("cmd_gesture_system", "Next System", "Nächstes System"),
    row("cmd_gesture_system_describe", "Cycles the system of the component tool (none makes it a plain component, any other a terminal of that system) and of the MEP route tool (Alt+T).", "Schaltet das System des Komponentenwerkzeugs (keines macht sie zur einfachen Komponente, jedes andere zum Anschluss dieses Systems) und des TGA-Leitungswerkzeugs weiter (Alt+T)."),
  ]],
  ["    fault_template_value_invalid:", [
    row("fault_create_component_family_missing", "A component needs a family to place first.", "Eine Komponente braucht zuerst eine Familie zum Platzieren."),
    row("fault_place_family_unavailable", "That family does not exist or is a profile, which cannot be placed.", "Diese Familie gibt es nicht oder sie ist ein Profil, das sich nicht platzieren lässt."),
    row("fault_override_component_missing", "That component does not exist.", "Diese Komponente gibt es nicht."),
    row("fault_override_parameter_missing", "The family of the component has no such parameter.", "Die Familie der Komponente hat keinen solchen Parameter."),
    row("fault_browser_no_match", "No placeable family matches the search text.", "Keine platzierbare Familie passt zum Suchtext."),
    row("fault_tool_family_missing", "The model has no family to place; add one in the library first.", "Das Modell hat keine Familie zum Platzieren; legen Sie zuerst in der Bibliothek eine an."),
  ]],
];

let source = readFileSync(file, "utf8");
const crlf = source.includes("\r\n");
if (crlf) source = source.replaceAll("\r\n", "\n");
const lines = source.split("\n");
let added = 0;
for (const [anchor, rows] of blocks) {
  const at = lines.findIndex((line) => line.startsWith(anchor));
  if (at < 0) throw new Error(`anchor ${anchor} not found`);
  const fresh = rows.filter((candidate) => !lines.some((line) => line.startsWith(candidate.slice(0, candidate.indexOf(":") + 1))));
  lines.splice(at + 1, 0, ...fresh);
  added += fresh.length;
}
const next = lines.join("\n");
writeFileSync(file, crlf ? next.replaceAll("\n", "\r\n") : next);
console.log(`terminology: ${added} rows added`);
