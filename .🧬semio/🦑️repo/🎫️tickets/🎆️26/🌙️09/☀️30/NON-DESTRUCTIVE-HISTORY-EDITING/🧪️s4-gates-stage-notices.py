"""📦️ S4-GATES rule-45 staging: renders the framework notice rows for the SDK's unlabelled tool-run, time-travel, transaction and
interactive-job refusals (Rust table rows, TypeScript twin rows, fixture rows, widened schema pattern) and the SDK patch naming its
21 anonymous `plugin.internal` refusals on those flows, into `🗑️generated/s4-gates/stage-r45/` for landing once framework saves reopen."""
import difflib, json, os, sys

REPO = "/Users/ueli/Documents/semio"
T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "s4-gates", "stage-r45")
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
PATTERN = "^(app|mutation|plugin|history-filter|window-transient|timeTravel|toolRun|toolTransaction|transaction|interactive-job)(\\.[a-z][a-z0-9]*(-[a-z0-9]+)*)+$"
RENAMES = [
    ("⏪️time-travel/🦀️.rs", "fn retire_step", "plugin_sdk_fault(error.into_message())", "timeTravel.snapshot-retirement"),
    ("⏪️time-travel/🦀️.rs", None, "plugin_sdk_fault(\"a composed member's history-edit owners belong to another store kind\")", "timeTravel.member-store-kind"),
    ("⏪️time-travel/🦀️.rs", None, "plugin_sdk_fault(\"a history-edit preview answered another command\")", "timeTravel.preview-mismatch"),
    ("⏯️tool-run/🦀️.rs", None, "plugin_sdk_fault(\"tool run publication handoff lost its exact retirement owner\")", "toolRun.publication-handoff"),
    ("⏯️tool-run/🦀️.rs", "if let Some(retirement) = self.snapshot_retirement.as_mut()", "plugin_sdk_fault(error.into_message())", "toolRun.snapshot-retirement"),
    ("⏯️tool-run/🦀️.rs", None, "plugin_sdk_fault(\"a member tool run's owners belong to another store kind\")", "toolRun.member-store-kind"),
    ("⏯️tool-run/🦀️.rs", None, "plugin_sdk_fault(\"a member tool run lost its base\")", "toolRun.member-base-lost"),
    ("⏯️tool-run/🦀️.rs", None, "inject_tool_run_trace_lane_into(surface, &lane).map_err(|error| plugin_sdk_fault(error.to_string()))", "toolRun.trace-lane"),
    ("⏯️tool-run/🦀️.rs", "if let Some(publication) = finalize.publication.as_mut()", "plugin_sdk_fault(error.into_message())", "toolRun.publication-retirement"),
    ("🦀️.rs", None, "plugin_sdk_fault(\"composition history requires the parent's exact declared dialect\")", "transaction.group-history-dialect"),
    ("🦀️.rs", None, "plugin_sdk_fault(\"group history moved a child without exact immutable-root authority\")", "transaction.group-history-root"),
    ("🦀️.rs", None, "plugin_sdk_fault(format!(\n                    \"transaction_{action}: no member of this instance carries group", "transaction.group-history-tail"),
    ("🦀️.rs", None, "plugin_sdk_fault(format!(\"transaction_rollback: no pending transaction matches txn_id {txn_id:?}\"))", "transaction.rollback-unknown"),
    ("🦀️.rs", None, "plugin_sdk_fault(format!(\"transaction_undo: this instance's tail edit does not belong to group {group_id:?}\"))", "transaction.undo-foreign-tail"),
    ("🦀️.rs", "self.store.undo().await", "plugin_sdk_fault(format!(\"{error:?}\"))", "transaction.undo-failed"),
    ("🦀️.rs", "self.store.redo().await", "plugin_sdk_fault(format!(\"{error:?}\"))", "transaction.redo-failed"),
    ("🦀️.rs", None, "plugin_sdk_fault(format!(\"transaction_redo: this instance's redo-tail edit does not belong to group {group_id:?}\"))", "transaction.redo-foreign-tail"),
]
ROWS = [
    ("timeTravel.member-owners", "A history edit could not finish closing a composed part — try again.", "Eine Verlaufsbearbeitung konnte einen zusammengesetzten Teil nicht fertig schließen — erneut versuchen."),
    ("timeTravel.snapshot-close", "The history view could not close its snapshot cleanly — try again.", "Die Verlaufsansicht konnte ihren Schnappschuss nicht sauber schließen — erneut versuchen."),
    ("timeTravel.snapshot-retirement", "The history view could not release its snapshot — try again.", "Die Verlaufsansicht konnte ihren Schnappschuss nicht freigeben — erneut versuchen."),
    ("timeTravel.unknown-action", "This history action is not known.", "Diese Verlaufsaktion ist unbekannt."),
    ("timeTravel.member-store-kind", "This part keeps its history differently and cannot be edited here.", "Dieser Teil führt seinen Verlauf anders und kann hier nicht bearbeitet werden."),
    ("timeTravel.preview-mismatch", "The history preview answered a different request — try again.", "Die Verlaufsvorschau hat auf eine andere Anfrage geantwortet — erneut versuchen."),
    ("toolRun.unknown-action", "This tool run action is not known.", "Diese Werkzeuglauf-Aktion ist unbekannt."),
    ("toolRun.tool-id", "Choose a tool before starting a run.", "Vor dem Start eines Laufs ein Werkzeug wählen."),
    ("toolRun.unknown-tool", "This tool cannot run step by step.", "Dieses Werkzeug kann nicht schrittweise laufen."),
    ("toolRun.busy", "A tool run is still active — finish or cancel it first.", "Ein Werkzeuglauf ist noch aktiv — ihn zuerst beenden oder abbrechen."),
    ("toolRun.tick-bytes", "A tool run step is too large to show.", "Ein Schritt des Werkzeuglaufs ist zu groß zum Anzeigen."),
    ("toolRun.tick-decode", "A tool run step could not be read.", "Ein Schritt des Werkzeuglaufs konnte nicht gelesen werden."),
    ("toolRun.trace-lane", "The tool run's progress could not be shown.", "Der Fortschritt des Werkzeuglaufs konnte nicht angezeigt werden."),
    ("toolRun.member-unavailable", "This tool run needs exactly one open part to work on.", "Dieser Werkzeuglauf braucht genau einen geöffneten Teil zum Bearbeiten."),
    ("toolRun.member-gone", "The part this tool run edits is no longer there.", "Der Teil, den dieser Werkzeuglauf bearbeitet, ist nicht mehr vorhanden."),
    ("toolRun.member-store-kind", "This part keeps its history differently and cannot run this tool.", "Dieser Teil führt seinen Verlauf anders und kann dieses Werkzeug nicht ausführen."),
    ("toolRun.member-base-lost", "The tool run lost the state it started from — start it again.", "Der Werkzeuglauf hat seinen Ausgangszustand verloren — ihn erneut starten."),
    ("toolRun.job-close", "The tool run could not close cleanly — try again.", "Der Werkzeuglauf konnte nicht sauber beendet werden — erneut versuchen."),
    ("toolRun.snapshot-close", "The tool run could not close its snapshot cleanly — try again.", "Der Werkzeuglauf konnte seinen Schnappschuss nicht sauber schließen — erneut versuchen."),
    ("toolRun.snapshot-retirement", "The tool run could not release its snapshot — try again.", "Der Werkzeuglauf konnte seinen Schnappschuss nicht freigeben — erneut versuchen."),
    ("toolRun.publication-close", "The tool run could not finish publishing its result — try again.", "Der Werkzeuglauf konnte sein Ergebnis nicht fertig veröffentlichen — erneut versuchen."),
    ("toolRun.publication-handoff", "The tool run lost track of its result — try again.", "Der Werkzeuglauf hat den Überblick über sein Ergebnis verloren — erneut versuchen."),
    ("toolRun.publication-retirement", "The tool run could not release its result — try again.", "Der Werkzeuglauf konnte sein Ergebnis nicht freigeben — erneut versuchen."),
    ("toolTransaction.shape", "This tool change does not have the shape its tool declares.", "Diese Werkzeugänderung hat nicht die Form, die ihr Werkzeug angibt."),
    ("transaction.instance-busy", "Another linked change is still pending — wait for it to finish.", "Eine andere verknüpfte Änderung steht noch aus — warten, bis sie abgeschlossen ist."),
    ("transaction.member-rejected", "One part refused the linked change.", "Ein Teil hat die verknüpfte Änderung abgelehnt."),
    ("transaction.unknown-mutation", "A linked change holds an edit this part does not know.", "Eine verknüpfte Änderung enthält eine Bearbeitung, die dieser Teil nicht kennt."),
    ("transaction.generation-mismatch", "The part changed while the linked change was prepared — try again.", "Der Teil hat sich geändert, während die verknüpfte Änderung vorbereitet wurde — erneut versuchen."),
    ("transaction.commit-failed", "A linked change could not be completed in every part.", "Eine verknüpfte Änderung konnte nicht in allen Teilen abgeschlossen werden."),
    ("transaction.child-groups-malformed", "A linked change to the parts could not be read.", "Eine verknüpfte Änderung der Teile konnte nicht gelesen werden."),
    ("transaction.rollback-unknown", "There is no pending linked change to roll back.", "Es gibt keine ausstehende verknüpfte Änderung zum Zurücknehmen."),
    ("transaction.undo-foreign-tail", "The latest step of this part belongs to another change — undo that first.", "Der letzte Schritt dieses Teils gehört zu einer anderen Änderung — diese zuerst rückgängig machen."),
    ("transaction.redo-foreign-tail", "The next redo step of this part belongs to another change.", "Der nächste Wiederherstellungsschritt dieses Teils gehört zu einer anderen Änderung."),
    ("transaction.undo-failed", "This linked change could not be undone.", "Diese verknüpfte Änderung konnte nicht rückgängig gemacht werden."),
    ("transaction.redo-failed", "This linked change could not be redone.", "Diese verknüpfte Änderung konnte nicht wiederhergestellt werden."),
    ("transaction.group-history-dialect", "This composed document's type does not allow this history step.", "Der Typ dieses zusammengesetzten Dokuments erlaubt diesen Verlaufsschritt nicht."),
    ("transaction.group-history-root", "This history step would move a part it may not move.", "Dieser Verlaufsschritt würde einen Teil verschieben, den er nicht verschieben darf."),
    ("transaction.group-history-tail", "This change is no longer the latest step of any part.", "Diese Änderung ist bei keinem Teil mehr der letzte Schritt."),
    ("interactive-job.cancelled", "The action was cancelled.", "Die Aktion wurde abgebrochen."),
    ("interactive-job.tool-completion-busy", "The tool is still finishing — try again in a moment.", "Das Werkzeug wird noch abgeschlossen — gleich erneut versuchen."),
    ("interactive-job.maintenance-tool-authority", "A running tool changed during upkeep — try again.", "Ein laufendes Werkzeug hat sich während der Wartung geändert — erneut versuchen."),
    ("interactive-job.tool-document-retirement-invariant", "The tool could not release its document cleanly — try again.", "Das Werkzeug konnte sein Dokument nicht sauber freigeben — erneut versuchen."),
    ("interactive-job.publication-authority-missing", "This command publishes its result in a way this app does not support.", "Dieser Befehl veröffentlicht sein Ergebnis auf eine Weise, die diese App nicht unterstützt."),
    ("interactive-job.missing-owned-reducer", "This command has no editing step of its own in this app.", "Dieser Befehl hat in dieser App keinen eigenen Bearbeitungsschritt."),
    ("interactive-job.missing-factory", "This command is not registered as a tool of this app.", "Dieser Befehl ist nicht als Werkzeug dieser App registriert."),
    ("interactive-job.incomplete-operation-authority", "This command lacks a complete prepare, edit and commit path.", "Diesem Befehl fehlt ein vollständiger Ablauf aus Vorbereiten, Bearbeiten und Übernehmen."),
    ("interactive-job.catalog-controller", "This app's tools are registered to a different controller.", "Die Werkzeuge dieser App sind bei einer anderen Steuerung registriert."),
    ("interactive-job.catalog-authority", "A tool of this app is registered without its full authority.", "Ein Werkzeug dieser App ist ohne seine vollständige Berechtigung registriert."),
    ("interactive-job.catalog-incomplete", "A command of this app lacks its registered editing step.", "Einem Befehl dieser App fehlt sein registrierter Bearbeitungsschritt."),
]


def rust(text: str) -> str:
    return json.dumps(text, ensure_ascii=False)


def main() -> None:
    os.makedirs(OUT, exist_ok=True)
    codes = [code for code, _, _ in ROWS]
    assert len(codes) == len(set(codes)), "duplicate code"
    import re
    assert all(re.match(PATTERN, code) for code in codes), [code for code in codes if not re.match(PATTERN, code)]
    assert {code for _, _, _, code in RENAMES} <= set(codes)
    with open(os.path.join(OUT, "framework-notice-rows.json"), "w", encoding="utf8") as handle:
        json.dump({"schemaPattern": PATTERN, "notices": [{"code": code, "en": en, "de": de} for code, en, de in ROWS]}, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    with open(os.path.join(OUT, "kernel-rows.rs.txt"), "w", encoding="utf8") as handle:
        handle.write("".join(f"    ({rust(code)}, {rust(en)}, {rust(de)}),\n" for code, en, de in ROWS))
    with open(os.path.join(OUT, "kernel-rows.ts.txt"), "w", encoding="utf8") as handle:
        handle.write("".join(f"  [{rust(code)}, {rust(en)}, {rust(de)}],\n" for code, en, de in ROWS))
    patches = []
    applied = 0
    for file in sorted({file for file, _, _, _ in RENAMES}):
        path = os.path.join(REPO, SDK, file)
        before = open(path, encoding="utf8").read()
        after = before
        for target, anchor, needle, code in [row for row in RENAMES if row[0] == file]:
            replacement = needle.replace("plugin_sdk_fault(", f"Fault::new(FaultOrigin::Plugin, FaultCode::new({rust(code)}), ", 1)
            if anchor is None:
                count = after.count(needle)
                assert count >= 1, (file, needle)
                after = after.replace(needle, replacement)
                applied += count
            else:
                start = after.index(anchor)
                at = after.index(needle, start)
                after = after[:at] + replacement + after[at + len(needle):]
                applied += 1
        patches.append("".join(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"a/{SDK}/{file}", f"b/{SDK}/{file}", n=2)))
    with open(os.path.join(OUT, "sdk-naming.patch"), "w", encoding="utf8") as handle:
        handle.write("".join(patches))
    print(f"{len(ROWS)} rows, {applied} SDK sites renamed into {len(set(code for _, _, _, code in RENAMES))} codes -> {OUT}")


if __name__ == "__main__":
    main()
