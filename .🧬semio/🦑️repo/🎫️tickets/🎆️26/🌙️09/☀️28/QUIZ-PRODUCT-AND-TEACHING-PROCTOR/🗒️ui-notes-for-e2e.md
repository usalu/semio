# 🗒️ UI notes for the agent "dev-e2e"

Written by the agent "ui" (React client of the quiz, site shell, UI texts). Everything a browser spec can observe that
changed in this work package: flows first, then hooks, then every text (old → new, English and German).
Status: final (2026-10-02). Checked in a real browser against a private stack (site 6071, proctor 8801, own data folder).

## 1. Flows and structure that changed

| Where | Before | Now |
|---|---|---|
| Document before the app runs | `<html lang="en">`, English title | no `lang`; title `Architecture and Technology Quizzes · Quizze zu Architektur und Technologie`; loading text is the same pair, each part with its `lang`; a `<noscript>` with one paragraph per language |
| Language on a first visit | English unless the browser asked for German | the first **offered** language of `navigator.languages` is preselected (`en-GB` → English, `de-CH` → German). When the list names neither, the app shows a language chooser first (card `[data-card="language"]`, heading level 1 "Language · Sprache", a list of two buttons `English` and `Deutsch`, each beside "Choose your language." / "Wähle deine Sprache." in its own `lang`; the document title is "Language · Sprache" and `<html>` has no `lang`); choosing stores the preference and continues with the introduction. A Playwright context with `locale: "fr-FR"` gets the chooser |
| `<html lang>` and `document.title` | fixed | follow the chosen language; title is `<screen> · <catalog title>` in that language (e.g. `Leaderboard · Architecture and Technology Quizzes`, `Rangliste · Quizze zu Architektur und Technologie`) |
| Footer | none | `footer` (`contentinfo`) with `nav` "Legal and privacy" / "Rechtliches und Datenschutz": a button "What is stored" / "Was gespeichert wird" (opens a `dialog`), and links "Imprint" / "Privacy policy" **only when** `site.legal.imprint` / `site.legal.privacy` are set in `🚀️deploy/🔣️.json` (both unset now). The page therefore has **two** `navigation` landmarks: name them (`getByRole("navigation", { name: … })`) |
| Dialogs | rendered in place | every dialog is portaled into `.quiz-app`, has `aria-modal="true"`, makes its siblings `inert`, traps Tab, closes on Escape and returns focus to its opener. Roles: `alertdialog` for "Submit this quiz?" and "Switch identity?", `dialog` for "What this site stores" |
| Switch identity | one click switched | opens `alertdialog` "Switch identity?". Initial focus is "Keep this identity". Named learner: confirm button "Switch identity"; anonymous learner: confirm button "Switch and give up this progress" and a text saying the progress cannot be continued |
| Identity step | handle input with `maxlength=64`, trimmed client-side | no `maxlength`; the handle is sent as typed (NFC) and the server normalizes it (`  probe   FUCHS 71 ` recalls "Probe Fuchs 71"). A handle the policy refuses is reported at the field (`role="alert"`, `aria-invalid="true"`) before anything is sent, naming the refused characters. A known pseudonym is recalled by a **query** only (no command); a new one is registered by one command. Anonymous is the preselected radio |
| Submit button of a run | `disabled` until every task is complete | never `disabled`; `aria-disabled="true"` plus `aria-describedby` → the hint. Playwright's `toBeDisabled()` still holds (it honours `aria-disabled`); `click()` on it does nothing |
| Sorting, "Keep this order" | focus stayed on the button, which disappeared | focus moves to the list (`ol[role=list][tabindex=-1]`, named "Order by …"); the task's live region says "Order kept" |
| Sorting, move at an end | silent | live region says "… is already first / last" |
| Matching, the select of an item | choosing a card another item used took it away from that item | options another item uses are `disabled` and read "… – used by …"; remove the card there first (button in the last column, header "Remove") or drag it (dragging still takes over). `selectOption()` of a disabled option fails in Playwright |
| Matching table | column header "Item … / (empty)" | named by the dimension heading; last column header "Remove"; the row header is only the item label; the crowd line sits under the select in the value cell |
| Task live regions | every single choice was announced at once | classification and matching announce after 400 ms of quiet, latest only |
| Connection indicator (`header [data-tone]`) | `role="status"`, spoke on every change | not a live region. Beside it `p[role=status][data-connection-announcer]` (visually hidden) is empty until the connection enters `alert`, then carries the alert sentence once, then "Connection restored". `data-tone` values are unchanged (`calm`, `busy`, `alert`); a rate-limited proctor (`429`, or `503` with `Retry-After`) shows tone `busy` with "Server busy – retrying shortly" |
| Leaderboard page | every learner in one table | top rows (at most 100), a line `[data-board-count]` "Learners in total: n" (and "Shown: the top 100" when more), the own row apart in `tbody[data-board-own]` under a row header "Your place" **only when** the learner is ranked outside the shown rows, a note "Just for fun: …", and "Updated …" = when the standings last changed in this view. Column "Last activity" is "Last submission". A missing best score is "–" with hidden text "No score yet" |
| Leaderboard polling | fixed interval | interval ±10 % jitter; backs off (doubling, capped at 5 min, honours `Retry-After`) while the proctor does not answer; no re-render when the answer is unchanged |
| Runs table (learner page) | action column header was an empty `td` | every header cell is a `th`; the last one is "Actions" (visually hidden). Missing values are "–" with hidden text |
| Badges | earned / not earned by opacity | tile `li[data-state="earned"|"locked"]` (still `[data-earned]` when earned), mark `[data-badge-mark="check"|"lock"]`, visible text "Earned …" / "Not earned yet", solid vs dashed border, no opacity |
| Results | verdict as "✓ Correct" inside one string | glyph `aria-hidden` + text; each result table is named by its task heading |
| Layered home | Overview button last in the DOM | first in the DOM (tab order), `aria-keyshortcuts="Escape"`; list mode scrolls when cards do not fit; list mode is chosen below 480 px × text-scale height and the narrow/medium breakpoints scale with the text size |
| Peer labels | could cover the focused control | a label that would cover the control that has focus is hidden (`[data-shy]` on the peer) until focus moves |
| Lists | `list-none` lists without a role | every such list carries `role="list"` |

Unchanged hooks (still there, same meaning): `data-card`, `data-page`, `data-quiz-item`, `data-quiz-drop`, `data-quiz-drag`,
`data-complete`, `data-earned`, `data-tone`, `data-crowd-source`, `data-crowd-item`, `data-presence-status`,
`data-presence-list`, `data-presence-layer`, `data-presence-anchor`, `data-peer`, `data-tag`, `data-anchor`, every
`data-layered-*` and `data-overview-*`.

New hooks: `data-connection-announcer`, `data-board-count`, `data-board-own`, `data-badge-mark`, `data-state` (badge
tiles), `data-shy`, `data-autofocus` (the control a dialog focuses first).

## 2. Catalog texts (`🎓️teaching/🏛️architecture/❓️quiz/🔣️.json`)

| Text | Before | Now |
|---|---|---|
| `title.de` | Quizze Architektur und Technologie | Quizze zu Architektur und Technologie |
| introduction ¶2 en | Every run draws and shuffles the items anew, … | Every run picks and shuffles the items afresh, … |
| introduction ¶2 de | Jeder Durchgang zieht und mischt die Elemente neu, … | Jeder Durchgang wählt die Elemente neu aus und mischt sie, … |
| introduction ¶3 en | … between 0 and 100 % … costs the more, the further apart … | … between 0 and 100% … costs more the further apart … |
| introduction ¶3 de | … wo die Werte Größenordnungen umfassen … Bei Klassifikationen … | … wenn die Werte mehrere Größenordnungen umfassen … Bei Klassifizierungen … |
| introduction ¶4 en | … There is no password: whoever later enters the same pseudonym or name continues the same record, so choose something distinctive if your results should stay yours. | … A pseudonym or name is public – on the leaderboard and to the others who are online. There are no passwords: whoever enters the same pseudonym or name continues as that learner, so pick a pseudonym others will not guess if you want your results to stay yours. Anonymous progress stays in the browser you played in and is lost when you switch identity or clear its data. |
| introduction ¶4 de | … Es gibt kein Passwort: Wer später dasselbe Pseudonym oder denselben Namen eingibt, setzt dieselbe Aufzeichnung fort. Wähle also etwas Unverwechselbares, … | … Ein Pseudonym oder Name ist öffentlich – in der Rangliste und für die anderen, die online sind. Es gibt keine Passwörter: Wer dasselbe Pseudonym oder denselben Namen eingibt, macht als diese Person weiter. Wähle also ein Pseudonym, das andere nicht erraten, … Anonymer Fortschritt bleibt in dem Browser, in dem du gespielt hast, und geht verloren, wenn du die Identität wechselst oder seine Daten löschst. |
| introduction ¶5 en | … and for completing every quiz. The leaderboard ranks everyone by the sum of their best quiz scores. | … one more badge is for submitting a run of every quiz. The leaderboard ranks … It is all for fun: while you play you see what the others think, and they see what you answer. |
| introduction ¶5 de | … und dafür, jedes Quiz abgeschlossen zu haben. Die Bestenliste ordnet alle nach der Summe ihrer besten Quiz-Ergebnisse. | … ein weiteres Abzeichen gibt es dafür, von jedem Quiz einen Durchgang abgegeben zu haben. Die Rangliste ordnet alle nach der Summe ihrer besten Quiz-Wertungen. Das Ganze ist zum Spaß da: … |
| four badge descriptions en | Scored 100 % in a run … | Scored 100% in a run … |
| badge "Musterblick" de | Jede Klassifikationsaufgabe … | Jede Klassifizierungsaufgabe … |

The four quiz content files (`⚡️energy/*/❓️quiz/🔣️.json`) changed in wording and typography only (no id, value, order or
solution changed); every change is listed in `📓️ui-content-wording-report.md`. Specs that read texts from the catalog and
the quiz files (as `🚶️learner` does) need no change.

## 3. UI texts (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts`)

Regenerate with `bun ui_text_changes.ts` in this ticket folder (compares the working tree with the last commit).

### Changed (31)

| Key | English before | English now | German before | German now |
|---|---|---|---|---|
| `quiz.connection.reconnectingWaiting` | Connection lost – retrying ({{waiting}} answers kept on this device) | Connection lost – retrying. Answers kept on this device: {{waiting}} | Verbindung unterbrochen – neuer Versuch läuft ({{waiting}} Antworten auf diesem Gerät gesichert) | Verbindung unterbrochen – neuer Versuch läuft. Auf diesem Gerät gesicherte Antworten: {{waiting}} |
| `quiz.preferences.textLargest` | Largest | Largest | Am größten | Maximal |
| `quiz.preferences.summary` | Language, colour theme, text size and others' cursors. | Language, colour theme, text size, others' cursors and what others think. | Sprache, Farbschema, Textgröße und die Cursor der anderen. | Sprache, Farbschema, Textgröße, die Cursor der anderen und was die anderen denken. |
| `quiz.preferences.answers` | Show what others think | Show what others think | Zeigen, was die anderen denken | Anzeigen, was die anderen denken |
| `quiz.identity.lead` | Choose how your results appear on the leaderboard. | Choose how you appear on the public leaderboard and to the others who are online. | Wähle, wie deine Ergebnisse in der Rangliste erscheinen. | Wähle, wie du in der öffentlichen Rangliste und für die anderen erscheinst, die online sind. |
| `quiz.identity.anonymousHint` | You appear as “Anonymous” with a short code. Your progress stays tied to this device. | You appear as “Anonymous” with a short code. Your progress stays on this device only: it is lost when you switch identity or clear this browser's data. | Du erscheinst als „Anonym“ mit einem kurzen Code. Dein Fortschritt bleibt an dieses Gerät gebunden. | Du erscheinst als „Anonym“ mit einem kurzen Code. Dein Fortschritt bleibt nur auf diesem Gerät: Er geht verloren, wenn du die Identität wechselst oder die Daten dieses Browsers löschst. |
| `quiz.identity.pseudonymHint` | You appear under a name you make up. | You appear under a name you make up. Recommended. | Du erscheinst unter einem selbst gewählten Namen. | Du erscheinst unter einem selbst gewählten Namen. Empfohlen. |
| `quiz.identity.nameHint` | You appear under your name. | You appear under your own name, visible to everyone. | Du erscheinst unter deinem Namen. | Du erscheinst unter deinem eigenen Namen, für alle sichtbar. |
| `quiz.home.points` | {{points}} points | Points: {{points}} | {{points}} Punkte | Punkte: {{points}} |
| `quiz.home.open` | In progress | In progress | Begonnen | In Bearbeitung |
| `quiz.home.lastResult` | View last result | View last result | Letztes Ergebnis ansehen | Letztes Ergebnis |
| `quiz.learner.started` | Started | Started | Begonnen | Gestartet |
| `quiz.learner.statusOpen` | In progress | In progress | Läuft | In Bearbeitung |
| `quiz.learner.statusVoided` | Voided | Voided | Ungültig | Verworfen |
| `quiz.quizPage.hint` | Every run draws and shuffles the items anew. Your best run counts. | Every run picks and shuffles the items afresh. Your best run counts. | Jeder Durchgang zieht und mischt die Elemente neu. Dein bester Durchgang zählt. | Jeder Durchgang wählt die Elemente neu aus und mischt sie. Dein bester Durchgang zählt. |
| `quiz.run.voided` | This quiz has been revised, so your run was voided. Please start again. | This quiz has been revised, so your run was voided. Please start a new run. | Dieses Quiz wurde überarbeitet, daher wurde dein Durchgang verworfen. Bitte starte neu. | Dieses Quiz wurde überarbeitet, daher wurde dein Durchgang verworfen. Bitte starte einen neuen Durchgang. |
| `quiz.classification.binEmpty` | No items yet – drop items here. | No items yet. | Noch keine Elemente – Elemente hier ablegen. | Noch keine Elemente. |
| `quiz.classification.assigned` | {{item}} assigned to {{category}} | {{item}} assigned to {{category}} | {{item}} zugeordnet zu {{category}} | {{item}} wurde {{category}} zugeordnet |
| `quiz.sorting.keep` | Keep this order | Keep this order | Diese Reihenfolge übernehmen | Reihenfolge übernehmen |
| `quiz.matching.hint` | Assign each item one value card per quantity. Each card can be used once. | Assign each item one value card per quantity. Each card can be used once: to give a used card to another item, first remove it from the item that uses it, or drag the card. | Ordne jedem Element pro Größe eine Wertkarte zu. Jede Karte kann einmal verwendet werden. | Ordne jedem Element pro Größe eine Wertkarte zu. Jede Karte kann einmal verwendet werden: Um eine verwendete Karte einem anderen Element zu geben, entferne sie zuerst bei dem Element, das sie verwendet, oder ziehe die Karte. |
| `quiz.matching.assigned` | {{value}} assigned to {{item}} | {{value}} assigned to {{item}} | {{value}} zugeordnet zu {{item}} | {{value}} wurde {{item}} zugeordnet |
| `quiz.results.credit` | Credit | Credit | Anrechnung | Punkte |
| `quiz.leaderboard.caption` | All learners with a submitted quiz, ranked by total points. Select a column heading to sort. | Learners with a submitted quiz, ranked by total points. Select a column heading to sort. | Alle Lernenden mit mindestens einem abgegebenen Quiz, nach Gesamtpunkten gereiht. Wähle eine Spaltenüberschrift zum Sortieren. | Lernende mit einem abgegebenen Quiz, nach Gesamtpunkten geordnet. Wähle eine Spaltenüberschrift zum Sortieren. |
| `quiz.leaderboard.learner` | Learner | Learner | Person | Lernende |
| `quiz.leaderboard.lastActivity` | Last activity | Last submission | Zuletzt aktiv | Letzte Abgabe |
| `quiz.leaderboard.full` | Full leaderboard | Full leaderboard | Ganze Rangliste | Vollständige Rangliste |
| `quiz.crowd.thinking` | Thinking along: {{count}} | Also working on it: {{count}} | Denken gerade mit: {{count}} | Arbeiten gerade mit: {{count}} |
| `quiz.crowd.positionShort` | ⌀ place {{position}} | avg. place {{position}} | ⌀ Platz {{position}} | ⌀ Platz {{position}} |
| `quiz.rejection.quizRevised` | This quiz has been revised; please start again. | This quiz has been revised; please start again. | Dieses Quiz wurde überarbeitet; bitte starte neu. | Dieses Quiz wurde überarbeitet; bitte starte einen neuen Durchgang. |
| `quiz.rejection.handleInvalid` | Please enter 1 to 64 characters. | The quiz server does not accept this pseudonym or name. Allowed are 1 to 64 characters: Latin letters, digits, single spaces between words and ' . _ - (at least one letter or digit). | Bitte gib 1 bis 64 Zeichen ein. | Der Quiz-Server nimmt dieses Pseudonym oder diesen Namen nicht an. Erlaubt sind 1 bis 64 Zeichen: lateinische Buchstaben, Ziffern, einzelne Leerzeichen zwischen Wörtern und ' . _ - (mindestens ein Buchstabe oder eine Ziffer). |
| `quiz.rejection.refused` | The quiz server refused the request ({{detail}}). | The quiz server refused the request. | Der Quiz-Server hat die Anfrage abgelehnt ({{detail}}). | Der Quiz-Server hat die Anfrage abgelehnt. |

### Removed (2)

| Key | English before | English now | German before | German now |
|---|---|---|---|---|
| `quiz.identity.noPassword` | No password is needed. Entering a pseudonym or name that is already known loads its existing progress – on any device. Anyone who enters the same pseudonym or name continues that progress, so pick one others will not guess. | — | Es ist kein Passwort nötig. Wer ein bereits bekanntes Pseudonym oder einen bekannten Namen eingibt, lädt den bisherigen Fortschritt – auf jedem Gerät. Wer dasselbe Pseudonym oder denselben Namen eingibt, setzt diesen Fortschritt fort; wähle also etwas, das andere nicht erraten. | — |
| `quiz.identity.handleInvalid` | Please enter 1 to 64 characters. | — | Bitte gib 1 bis 64 Zeichen ein. | — |

### New (50)

| Key | English before | English now | German before | German now |
|---|---|---|---|---|
| `quiz.app.busy` | — | The quiz server is busy right now – trying again shortly. | — | Der Quiz-Server ist gerade ausgelastet – neuer Versuch in Kürze. |
| `quiz.app.close` | — | Close | — | Schließen |
| `quiz.app.details` | — | Technical details | — | Technische Details |
| `quiz.connection.throttled` | — | Server busy – retrying shortly | — | Server ausgelastet – neuer Versuch in Kürze |
| `quiz.connection.restored` | — | Connection restored | — | Verbindung wiederhergestellt |
| `quiz.preferences.choose` | — | Choose your language. | — | Wähle deine Sprache. |
| `quiz.identity.handleRule` | — | 1 to 64 characters: Latin letters, digits, single spaces between words and ' . _ - (at least one letter or digit). | — | 1 bis 64 Zeichen: lateinische Buchstaben, Ziffern, einzelne Leerzeichen zwischen Wörtern und ' . _ - (mindestens ein Buchstabe oder eine Ziffer). |
| `quiz.identity.handleEmpty` | — | Please enter at least one letter or digit. | — | Bitte gib mindestens einen Buchstaben oder eine Ziffer ein. |
| `quiz.identity.handleLong` | — | This is {{length}} characters long; at most 64 are allowed. | — | Das sind {{length}} Zeichen; erlaubt sind höchstens 64. |
| `quiz.identity.handleCharacters` | — | Not allowed here: {{characters}}. Use Latin letters, digits, single spaces between words and ' . _ - | — | Hier nicht erlaubt: {{characters}}. Verwende lateinische Buchstaben, Ziffern, einzelne Leerzeichen zwischen Wörtern und ' . _ - |
| `quiz.identity.handleLetter` | — | Please include at least one letter or digit. | — | Bitte verwende mindestens einen Buchstaben oder eine Ziffer. |
| `quiz.identity.public` | — | A pseudonym or name is shown publicly – on the leaderboard and to the others who are online. There are no passwords: anyone who enters the same pseudonym or name continues as that learner, on any device. Pick a pseudonym others will not guess. | — | Ein Pseudonym oder Name wird öffentlich angezeigt – in der Rangliste und für die anderen, die online sind. Es gibt keine Passwörter: Wer dasselbe Pseudonym oder denselben Namen eingibt, macht als diese Person weiter, auf jedem Gerät. Wähle ein Pseudonym, das andere nicht erraten. |
| `quiz.identity.forFun` | — | The quizzes and the leaderboard are for fun: the others can see what you answer, by design. | — | Die Quizze und die Rangliste sind zum Spaß da: Die anderen können sehen, was du antwortest – das ist so gewollt. |
| `quiz.identity.switchTitle` | — | Switch identity? | — | Identität wechseln? |
| `quiz.identity.switchAnonymous` | — | You are anonymous, so this browser holds the only key to your progress. If you switch, your runs, scores and badges stay on the leaderboard as “{{name}}”, but nobody – you included – can ever continue them. | — | Du bist anonym, deshalb hat nur dieser Browser den Schlüssel zu deinem Fortschritt. Wenn du wechselst, bleiben deine Durchgänge, Wertungen und Abzeichen als „{{name}}“ in der Rangliste, aber niemand – auch du nicht – kann sie je fortsetzen. |
| `quiz.identity.switchNamed` | — | Your progress stays saved. To come back, enter “{{name}}” again when you are asked how you want to appear – on this or any other device. | — | Dein Fortschritt bleibt gespeichert. Um zurückzukommen, gib „{{name}}“ erneut ein, wenn du gefragt wirst, wie du erscheinen möchtest – auf diesem oder einem anderen Gerät. |
| `quiz.identity.switchLose` | — | Switch and give up this progress | — | Wechseln und Fortschritt aufgeben |
| `quiz.identity.switchKeep` | — | Keep this identity | — | Identität behalten |
| `quiz.learner.actions` | — | Actions | — | Aktionen |
| `quiz.learner.noScore` | — | No score | — | Keine Wertung |
| `quiz.learner.notSubmitted` | — | Not submitted | — | Nicht abgegeben |
| `quiz.run.savingAnswers` | — | Saving answers… | — | Antworten werden gespeichert… |
| `quiz.sorting.first` | — | {{item}} is already first | — | {{item}} steht bereits ganz oben |
| `quiz.sorting.last` | — | {{item}} is already last | — | {{item}} steht bereits ganz unten |
| `quiz.sorting.kept` | — | Order kept | — | Reihenfolge übernommen |
| `quiz.matching.actions` | — | Remove | — | Entfernen |
| `quiz.radar.values` | — | {{values}}. The values follow as a table. | — | {{values}}. Die Werte folgen als Tabelle. |
| `quiz.radar.entry` | — | {{axis}}: {{value}} | — | {{axis}}: {{value}} |
| `quiz.radar.missing` | — | No value | — | Kein Wert |
| `quiz.leaderboard.learners` | — | Learners in total: {{count}} | — | Lernende insgesamt: {{count}} |
| `quiz.leaderboard.shown` | — | Shown: the top {{count}} | — | Angezeigt: die besten {{count}} |
| `quiz.leaderboard.own` | — | Your place | — | Dein Platz |
| `quiz.leaderboard.forFun` | — | Just for fun: names are not verified, and anyone can play under any pseudonym. | — | Nur zum Spaß: Namen werden nicht geprüft, und jede Person kann unter jedem Pseudonym spielen. |
| `quiz.leaderboard.noBest` | — | No score yet | — | Noch keine Wertung |
| `quiz.legal.label` | — | Legal and privacy | — | Rechtliches und Datenschutz |
| `quiz.legal.imprint` | — | Imprint | — | Impressum |
| `quiz.legal.policy` | — | Privacy policy | — | Datenschutzerklärung |
| `quiz.legal.external` | — | opens in a new tab | — | öffnet in einem neuen Tab |
| `quiz.legal.privacy` | — | What is stored | — | Was gespeichert wird |
| `quiz.legal.privacyTitle` | — | What this site stores | — | Was diese Seite speichert |
| `quiz.legal.server` | — | On the quiz server: the pseudonym or name you entered (none while you are anonymous), your runs with their answers and scores, and your badges. Your pseudonym or name, points, best scores, badges, number of runs and the time of your last submission are public on the leaderboard. | — | Auf dem Quiz-Server: das Pseudonym oder der Name, den du eingegeben hast (keiner, solange du anonym bist), deine Durchgänge mit ihren Antworten und Wertungen sowie deine Abzeichen. Dein Pseudonym oder Name, deine Punkte, besten Wertungen, Abzeichen, die Anzahl deiner Durchgänge und der Zeitpunkt deiner letzten Abgabe sind in der Rangliste öffentlich. |
| `quiz.legal.browser` | — | In this browser's storage: your learner id – the only key to your progress, there are no passwords –, your settings, a copy of your runs, and answers that are not sent yet. | — | Im Speicher dieses Browsers: deine Lern-ID – der einzige Schlüssel zu deinem Fortschritt, es gibt keine Passwörter –, deine Einstellungen, eine Kopie deiner Durchgänge und Antworten, die noch nicht gesendet sind. |
| `quiz.legal.presence` | — | While you are online, the others see that you are here, which page you are on, your pointer and what you currently answer. This is shown live and not saved. | — | Solange du online bist, sehen die anderen, dass du da bist, auf welcher Seite du bist, deinen Mauszeiger und was du gerade antwortest. Das wird live angezeigt und nicht gespeichert. |
| `quiz.legal.contact` | — | For questions, or to have your entries removed, contact the operator named in the imprint. | — | Bei Fragen oder wenn deine Einträge entfernt werden sollen, wende dich an die im Impressum genannte Person. |
| `quiz.rejection.handleClaimed` | — | Someone else registered this pseudonym or name a moment ago. Try again to continue as that learner, or choose another one. | — | Jemand anderes hat dieses Pseudonym oder diesen Namen gerade eben registriert. Versuche es erneut, um als diese Person weiterzumachen, oder wähle ein anderes. |
| `quiz.rejection.idInvalid` | — | The quiz server could not read this request. Please reload the page and try again. | — | Der Quiz-Server konnte diese Anfrage nicht lesen. Bitte lade die Seite neu und versuche es erneut. |
| `quiz.rejection.learnerExists` | — | The quiz server already knows this identity. Please try again. | — | Der Quiz-Server kennt diese Identität bereits. Bitte versuche es erneut. |
| `quiz.rejection.rosterFull` | — | The quiz server has reached its maximum number of learners. Please try again later, or continue with a pseudonym that already exists. | — | Der Quiz-Server hat die maximale Anzahl an Lernenden erreicht. Bitte versuche es später erneut oder mache mit einem bestehenden Pseudonym weiter. |
| `quiz.rejection.runsExhausted` | — | You have reached the maximum number of runs. Your results stay. | — | Du hast die maximale Anzahl an Durchgängen erreicht. Deine Ergebnisse bleiben erhalten. |
| `quiz.rejection.answersExhausted` | — | This run has reached its maximum number of saved changes. You can still submit it. | — | Dieser Durchgang hat die maximale Anzahl gespeicherter Änderungen erreicht. Du kannst ihn weiterhin abgeben. |

Keys: 224 before, 272 now.
