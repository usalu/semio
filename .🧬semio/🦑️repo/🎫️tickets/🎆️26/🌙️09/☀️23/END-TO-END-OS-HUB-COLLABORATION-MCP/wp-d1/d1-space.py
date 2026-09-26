"""🪐️ D1 codemod: agent-facing en/de descriptions for the space plugin's Home launcher, Space index and Studio apps."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-d1")
from d1_apply import describe

S = "✏️s/🔌️plugins/🪐️space"
describe(f"{S}/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_home_app", [
    ("createStudio", "Creates a new studio with the given name and kind, either a temporary local-only one or one kept in a folder when a folder path is given; sharing stays off until it is promoted.", "Erstellt ein neues Studio mit dem angegebenen Namen und der Art, entweder temporär und nur lokal oder mit Ordnerpfad in einem Ordner gespeichert; Teilen bleibt gesperrt, bis es hochgestuft wird."),
    ("bindSpaceFile", "Binds a studio to a file on disk by path, so the studio's events are persisted to and read from that file.", "Verknüpft ein Studio anhand eines Pfads mit einer Datei auf dem Datenträger, sodass seine Ereignisse in diese Datei geschrieben und daraus gelesen werden."),
    ("importSpace", "Imports a studio from the given .os DSL text, or opens the host's file picker for an .os file when no text is given.", "Importiert ein Studio aus dem angegebenen .os-DSL-Text oder öffnet ohne Text die Dateiauswahl des Hosts für eine .os-Datei."),
    ("openSpace", "Opens the space or studio with the given id, navigating the shell to it.", "Öffnet den Space oder das Studio mit der angegebenen Id und navigiert die Shell dorthin."),
    ("navigateVirtualFileSystemNode", "Navigates the shell to the space behind one node of the Home file tree.", "Navigiert die Shell zum Space hinter einem Knoten des Home-Dateibaums."),
    ("deleteVirtualFileSystemNode", "Deletes the local studio behind one node of the Home file tree, including its draft; its content is gone.", "Löscht das lokale Studio hinter einem Knoten des Home-Dateibaums samt Entwurf; sein Inhalt ist fort."),
    ("goHome", "Navigates the shell back to the Home launcher.", "Navigiert die Shell zurück zum Home-Starter."),
    ("createSpace", "Creates a new shared space on the hub with the given name; without a name it opens the Create Space dialog.", "Erstellt auf dem Hub einen neuen geteilten Space mit dem angegebenen Namen; ohne Namen öffnet es den Dialog Space erstellen."),
    ("deleteSpace", "Deletes one space on the hub for every member; the first call opens a confirmation dialog, and only the confirmed call deletes it.", "Löscht einen Space auf dem Hub für alle Mitglieder; der erste Aufruf öffnet einen Bestätigungsdialog, erst der bestätigte Aufruf löscht."),
    ("renameSpace", "Renames one space on the hub; without a new name it opens the Rename Space dialog seeded with the current name.", "Benennt einen Space auf dem Hub um; ohne neuen Namen öffnet es den Dialog Space umbenennen mit dem aktuellen Namen."),
    ("shareSpace", "Adds or updates a member of one space on the hub by email and role; without an email it opens the Share Space dialog.", "Fügt auf dem Hub einem Space ein Mitglied per E-Mail und Rolle hinzu oder aktualisiert es; ohne E-Mail öffnet es den Dialog Space teilen."),
    ("manageSpace", "Opens the shell's administration pane for one space, where the hub decides what the user may manage.", "Öffnet den Verwaltungsbereich der Shell für einen Space, in dem der Hub entscheidet, was der Nutzer verwalten darf."),
    ("copyInviteLink", "Asks the hub to mint an invite for one space and copies the redeemable link to the clipboard.", "Lässt den Hub eine Einladung für einen Space erzeugen und kopiert den einlösbaren Link in die Zwischenablage."),
    ("promoteToHubSpace", "Promotes a temporary local studio to a shared space on the hub under the given name, so it can be shared and edited together.", "Stuft ein temporäres lokales Studio unter dem angegebenen Namen zu einem geteilten Space auf dem Hub hoch, damit es geteilt und gemeinsam bearbeitet werden kann."),
    ("persistLocally", "Saves a temporary local studio into a folder on this machine so it survives restarts; it stays local-only and unshared.", "Speichert ein temporäres lokales Studio in einen Ordner auf diesem Rechner, damit es Neustarts übersteht; es bleibt lokal und ungeteilt."),
], [("applyDirectoryEventPage", "Chrome"), ("presenceHeartbeat", "Chrome")])

describe(f"{S}/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "create_space_index_editor", [
    ("createArtifact", "Creates a new artifact of the chosen kind with the given name in this space; without a name or kind it opens the Create Artifact dialog.", "Erstellt in diesem Space ein neues Artefakt der gewählten Art mit dem angegebenen Namen; ohne Namen oder Art öffnet es den Dialog Artefakt erstellen."),
    ("deleteArtifact", "Removes one artifact by id from this space's index for every member.", "Entfernt ein Artefakt anhand seiner Id für alle Mitglieder aus dem Index dieses Space."),
    ("renameArtifact", "Renames one artifact of this space's index by id.", "Benennt ein Artefakt im Index dieses Space anhand seiner Id um."),
    ("touchArtifact", "Records that an actor opened or edited one artifact at the given time, updating its last-touched entry in the index.", "Hält fest, dass ein Akteur ein Artefakt zum angegebenen Zeitpunkt geöffnet oder bearbeitet hat, und aktualisiert dessen Zuletzt-berührt-Eintrag im Index."),
    ("requestDeleteArtifact", "Opens the confirmation dialog for deleting one artifact; confirming it deletes the artifact for every member.", "Öffnet den Bestätigungsdialog zum Löschen eines Artefakts; die Bestätigung löscht es für alle Mitglieder."),
    ("openArtifact", "Opens one artifact of this space with the user's preferred app for its kind.", "Öffnet ein Artefakt dieses Space mit der bevorzugten App des Nutzers für seine Art."),
    ("openArtifactWith", "Opens one artifact of this space with an explicitly chosen app and role (editor or viewer).", "Öffnet ein Artefakt dieses Space mit einer ausdrücklich gewählten App und Rolle (Editor oder Betrachter)."),
    ("inviteMember", "Invites a person to this space on the hub by email with the given role.", "Lädt eine Person per E-Mail mit der angegebenen Rolle auf dem Hub in diesen Space ein."),
    ("requestInviteMember", "Opens the Invite Member dialog (email and role) for this space.", "Öffnet für diesen Space den Dialog Mitglied einladen (E-Mail und Rolle)."),
    ("removeMember", "Removes one member from this space on the hub; they lose access to its artifacts.", "Entfernt ein Mitglied auf dem Hub aus diesem Space; es verliert den Zugriff auf dessen Artefakte."),
    ("setVisibility", "Sets who can find this space on the hub (such as private or public).", "Legt fest, wer diesen Space auf dem Hub finden kann (etwa privat oder öffentlich)."),
    ("copyInviteLink", "Asks the hub to mint an invite link with the given role for this space and copies it to the clipboard.", "Lässt den Hub einen Einladungslink mit der angegebenen Rolle für diesen Space erzeugen und kopiert ihn in die Zwischenablage."),
], [("foldDirectoryEvents", "Chrome"), ("presenceHeartbeat", "Chrome")])

describe(f"{S}/⚙️engine/🪐️space/🦀️.rs", "create_space_app", [
    ("patchParameter", "Sets one field (such as name, value or range) of one studio parameter by id; the value may be JSON or plain text.", "Setzt ein Feld (etwa Name, Wert oder Bereich) eines Studioparameters anhand seiner Id; der Wert darf JSON oder Klartext sein."),
    ("addParameter", "Adds a studio parameter with the given name and type (numeric, categorical, toggle or text) that app fields can be bound to.", "Fügt einen Studioparameter mit dem angegebenen Namen und Typ (Zahl, Auswahl, Schalter oder Text) hinzu, an den App-Felder gebunden werden können."),
    ("removeParameter", "Removes one studio parameter by id together with its field bindings.", "Entfernt einen Studioparameter anhand seiner Id samt seiner Feldbindungen."),
    ("spawnApp", "Adds an instance of a plugin app (plugin id and app id) to the studio workflow at x, y and makes it the active instance.", "Fügt dem Studio-Workflow an x, y eine Instanz einer Plugin-App (Plugin-Id und App-Id) hinzu und macht sie zur aktiven Instanz."),
    ("moveMediaNode", "Moves one app instance to the workflow canvas position x, y.", "Verschiebt eine App-Instanz an die Position x, y der Workflow-Fläche."),
    ("connectMediaPorts", "Connects a media output port of one app instance to a media input port of another, so its output flows into the second.", "Verbindet einen Medienausgang einer App-Instanz mit einem Medieneingang einer anderen, sodass deren Ausgabe in die zweite fließt."),
    ("disconnectMediaEdge", "Removes one media connection between two app instances by id.", "Entfernt eine Medienverbindung zwischen zwei App-Instanzen anhand ihrer Id."),
    ("removeAppInstance", "Removes one app instance (the given one, or the selected one) from the workflow with its connections.", "Entfernt eine App-Instanz (die angegebene oder die ausgewählte) samt ihrer Verbindungen aus dem Workflow."),
    ("deleteSelection", "Removes every selected app instance and connection from the studio workflow.", "Entfernt alle ausgewählten App-Instanzen und Verbindungen aus dem Studio-Workflow."),
    ("copyAppInstance", "Copies the selected app instances to the studio clipboard for a later paste; the workflow is not changed.", "Kopiert die ausgewählten App-Instanzen für ein späteres Einfügen in die Studio-Zwischenablage; der Workflow ändert sich nicht."),
    ("duplicateAppInstance", "Adds a copy of each selected app instance next to it, labelled with Copy.", "Fügt neben jeder ausgewählten App-Instanz eine mit Kopie beschriftete Kopie hinzu."),
    ("pasteAppInstance", "Adds a copy of every app instance on the studio clipboard to the workflow, offset from the original.", "Fügt dem Workflow eine versetzte Kopie jeder App-Instanz aus der Studio-Zwischenablage hinzu."),
    ("renameAppInstance", "Renames one app instance of the workflow.", "Benennt eine App-Instanz des Workflows um."),
    ("patchMediaNodes", "Sets one field (such as the position along an axis) on several app instances at once.", "Setzt ein Feld (etwa die Position entlang einer Achse) auf mehreren App-Instanzen zugleich."),
    ("patchAppInstances", "Sets the label of several app instances at once.", "Setzt die Beschriftung mehrerer App-Instanzen zugleich."),
    ("bindParameterField", "Binds one field of an app instance (by field path) to a studio parameter, so the parameter's value drives it; an empty parameter unbinds it.", "Bindet ein Feld einer App-Instanz (per Feldpfad) an einen Studioparameter, sodass dessen Wert es steuert; ein leerer Parameter löst die Bindung."),
    ("unbindParameterField", "Removes the parameter binding from one field of an app instance, so the field keeps its own value again.", "Entfernt die Parameterbindung von einem Feld einer App-Instanz, sodass das Feld wieder seinen eigenen Wert behält."),
    ("reorganizeWorkflow", "Lays out every app instance of the workflow automatically, overwriting their manual positions.", "Ordnet alle App-Instanzen des Workflows automatisch an und überschreibt ihre manuellen Positionen."),
    ("exportMedia", "Writes the media one app instance produces in the chosen format to a downloaded file on the user's machine.", "Schreibt die Medien, die eine App-Instanz erzeugt, im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."),
    ("importMedia", "Opens the host's file picker for a file of the given format; the chosen file is then imported into the given app instance.", "Öffnet die Dateiauswahl des Hosts für eine Datei des angegebenen Formats; die gewählte Datei wird dann in die angegebene App-Instanz importiert."),
    ("importMediaPayload", "Imports a picked file's content (data URL) into the app instance that Import Media was started for.", "Importiert den Inhalt einer gewählten Datei (Data-URL) in die App-Instanz, für die Medien importieren gestartet wurde."),
    ("setActivePanelTab", "Switches the studio's side panel to the given tab; only the view changes.", "Schaltet das Seitenpanel des Studios auf den angegebenen Reiter um; nur die Ansicht ändert sich."),
    ("setActiveExample", "Opens one of the bundled example studios by navigating the shell to it; the current studio is not changed.", "Öffnet eines der mitgelieferten Beispielstudios, indem die Shell dorthin navigiert; das aktuelle Studio ändert sich nicht."),
    ("exportStudioPack", "Writes the whole studio as a .pack file and an .ops event log to two downloaded files on the user's machine.", "Schreibt das gesamte Studio als .pack-Datei und .ops-Ereignisprotokoll in zwei heruntergeladene Dateien auf dem Rechner des Nutzers."),
    ("exportStudioDsl", "Writes the whole studio as DSL text to a downloaded file on the user's machine.", "Schreibt das gesamte Studio als DSL-Text in eine heruntergeladene Datei auf dem Rechner des Nutzers."),
    ("importSpacePack", "Opens the host's file picker for a studio .pack file; the chosen file is then imported.", "Öffnet die Dateiauswahl des Hosts für eine Studio-.pack-Datei; die gewählte Datei wird dann importiert."),
    ("importSpacePackPayload", "Imports a studio from the content (data URL) of a picked .pack file.", "Importiert ein Studio aus dem Inhalt (Data-URL) einer gewählten .pack-Datei."),
    ("openInstance", "Opens the given or selected app instance in its own window so its artifact can be viewed or edited.", "Öffnet die angegebene oder ausgewählte App-Instanz in einem eigenen Fenster, damit ihr Artefakt angesehen oder bearbeitet werden kann."),
    ("closeFocusedInstance", "Closes the app instance window that is currently focused; the workflow is not changed.", "Schließt das gerade fokussierte App-Instanz-Fenster; der Workflow ändert sich nicht."),
    ("openSpace", "Opens the studio with the given id, navigating the shell to it.", "Öffnet das Studio mit der angegebenen Id und navigiert die Shell dorthin."),
    ("navigateVirtualFileSystemNode", "Navigates the shell to the space with the given id.", "Navigiert die Shell zum Space mit der angegebenen Id."),
    ("goHome", "Navigates the shell back to the Home launcher.", "Navigiert die Shell zurück zum Home-Starter."),
], [("nodeGraphEdit", "Input"), ("nodeGraphViewport", "Chrome"), ("presenceHeartbeat", "Chrome")], awaited=True, anchor='.keybinding("mod+s", "commitCheckpoint").await;', destructive=["reorganizeWorkflow"])
