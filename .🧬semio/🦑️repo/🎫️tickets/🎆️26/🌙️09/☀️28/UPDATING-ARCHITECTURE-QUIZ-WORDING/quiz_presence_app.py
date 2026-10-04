"""🧩️ Wires presence (design §15) into the quiz entry `🟦️.tsx`: the socket seam in `QuizOptions`, one `QuizPresence` per
app fed with the catalog, the learner, the place (with the run's task) and tab visibility, the pointer and focus
listeners, the navbar count, the provider every screen reads and the cursor overlay; plus the module's re-exports."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🟦️.tsx")
text = path.read_text(encoding="utf-8")
EDITS = [
    (" * home, run, results, leaderboard and badges; every string is English or German, the theme and text size are the\n * learner's, and answers survive short connection shortages on the device.",
     " * home, run, results, leaderboard and badges; every string is English or German, the theme and text size are the\n * learner's, and answers survive short connection shortages on the device. Learners see who else is online and where,\n * and the cursors of the learners on the same page."),
    ('import { BodyButton, CardIcon, QuizCard, cn } from "./🔨️modules/🪟️chrome/🟦️.tsx";\n',
     'import { BodyButton, CardIcon, QuizCard, cn } from "./🔨️modules/🪟️chrome/🟦️.tsx";\n'
     'import { PresenceOverlay, PresenceProvider, PresenceStatus, QuizPresence, presencePlace, presenceSelf, presenceView, useDocumentVisible, usePresencePointer, type PresenceConnect } from "./🔨️modules/👥️presence/🟦️.tsx";\n'),
    ('export { BadgesCard, BadgesScreen, awardOf, badgesEarnedIn } from "./🔨️modules/🏅️badges/🟦️.tsx";\n',
     'export { BadgesCard, BadgesScreen, awardOf, badgesEarnedIn } from "./🔨️modules/🏅️badges/🟦️.tsx";\n'
     'export {\n  EMPTY_PRESENCE_VIEW,\n  OnlineMark,\n  PRESENCE_ANCHORS,\n  PRESENCE_FRAME_HZ,\n  PRESENCE_FRAME_INTERVAL_MS,\n  PresenceList,\n  PresenceOverlay,\n  PresenceProvider,\n  PresenceRoom,\n  PresenceStatus,\n  QuizPresence,\n  browserPresenceConnect,\n  cursorAt,\n  placePeers,\n  placeText,\n  presencePlace,\n  presenceSelf,\n  presenceView,\n  useDocumentVisible,\n  usePresencePointer,\n  usePresenceTask,\n  usePresenceView,\n} from "./🔨️modules/👥️presence/🟦️.tsx";\n'
     'export type { PeerCursor, PresenceConnect, PresencePlace, PresenceRoomOptions, PresenceSelf, PresenceSocket, PresenceView, QuizPresenceInput, QuizPresenceOptions, QuizPresenceSnapshot, RoomMember, RoomSnapshot, RoomStatus } from "./🔨️modules/👥️presence/🟦️.tsx";\n'),
    ("/** ⚙️ What a quiz site passes in: the proctor base URL (`\"\"` is the site's own origin), the catalog id as tenant and\n * the site's logo (inline SVG markup) for the navbar. The optional seams replace the network, the storage, the\n * browser's language list and the retry timing. */",
     "/** ⚙️ What a quiz site passes in: the proctor base URL (`\"\"` is the site's own origin), the catalog id as tenant and\n * the site's logo (inline SVG markup) for the navbar. The optional seams replace the network, the presence sockets,\n * the storage, the browser's language list and the retry timing. */"),
    ("  readonly transport?: ProctorConnect;\n  readonly storage?: StorageArea;\n",
     "  readonly transport?: ProctorConnect;\n  readonly presence?: PresenceConnect;\n  readonly storage?: StorageArea;\n"),
    ("    return { store, session: new QuizSession({ proctor, store, timing: options.timing }) };\n  });\n  const { session, store } = setup;\n  useLayoutEffect(() => {\n    session.start();\n    return () => session.stop();\n  }, [session]);\n  const { state, connection } = useSyncExternalStore(session.subscribe, session.getSnapshot, session.getSnapshot);\n",
     "    const presence = new QuizPresence({ proctor: options.proctor, connect: options.presence, timing: options.timing });\n    return { store, presence, session: new QuizSession({ proctor, store, timing: options.timing }) };\n  });\n  const { session, store, presence } = setup;\n  useLayoutEffect(() => {\n    session.start();\n    return () => session.stop();\n  }, [session]);\n  useEffect(() => () => presence.stop(), [presence]);\n  const { state, connection } = useSyncExternalStore(session.subscribe, session.getSnapshot, session.getSnapshot);\n"),
    ("  const text = useMemo(() => quizText(locale), [locale]);\n",
     "  const text = useMemo(() => quizText(locale), [locale]);\n"
     "  const [task, setTask] = useState<string | undefined>(undefined);\n"
     "  const visible = useDocumentVisible();\n"
     "  const identified = state.introduced && state.learner !== undefined;\n"
     "  const self = useMemo(() => presenceSelf(identified ? state.learner : undefined), [identified, state.learner]);\n"
     "  const at = presencePlace(state.step, state, task);\n"
     "  const atKey = at === undefined ? \"\" : JSON.stringify(at);\n"
     "  useEffect(() => presence.update({ catalog: state.catalog?.id, self, at: atKey === \"\" ? undefined : (JSON.parse(atKey) as typeof at), active: visible }), [presence, state.catalog?.id, self, atKey, visible]);\n"
     "  usePresencePointer(presence);\n"
     "  const shared = useSyncExternalStore(presence.subscribe, presence.getSnapshot, presence.getSnapshot);\n"
     "  const view = useMemo(() => presenceView(shared, text), [shared, text]);\n"),
    ("  const identified = state.introduced && state.learner !== undefined;\n  const title = state.catalog",
     "  const title = state.catalog"),
    ('    { key: "connection", className: "min-w-0 shrink", content: <ConnectionStatus connection={connection} text={text} /> },\n',
     '    { key: "connection", className: "min-w-0 shrink", content: <ConnectionStatus connection={connection} text={text} /> },\n    ...(view.roster === undefined ? [] : [{ key: "presence", className: "shrink-0", content: <PresenceStatus text={text} /> }]),\n'),
    ('  return (\n    <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale}>\n',
     '  return (\n    <PresenceProvider view={view} setTask={setTask}>\n    <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground" style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale}>\n'),
    ("        <Screen session={session} state={state} connection={connection} text={text} locale={locale} preferences={preferences} onPreferences={change} />\n      </main>\n    </div>\n  );\n}\n",
     "        <Screen session={session} state={state} connection={connection} text={text} locale={locale} preferences={preferences} onPreferences={change} />\n      </main>\n      <PresenceOverlay view={view} show={preferences.showCursors} />\n    </div>\n    </PresenceProvider>\n  );\n}\n"),
]
for old, new in EDITS:
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:100]!r}")
    text = text.replace(old, new)
for attempt in range(20):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.25 * (attempt + 1))
print("[presence app] done")
