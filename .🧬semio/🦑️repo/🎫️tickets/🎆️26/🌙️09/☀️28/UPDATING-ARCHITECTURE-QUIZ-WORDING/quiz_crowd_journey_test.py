"""🚶️ The learner journey meets §17: the proctor double answers the `crowd` query with the core's `crowdView` over the
submitted results; on home the room of the overview watches every page's room and thinking room (the watch frame
follows the state frame after joining), a peer watched on the leaderboard page appears inside that page behind the
cards, a learner thinking along in a quiz shows on that quiz's page; in the run the quiz's thinking room is joined, the
learner's drafts go there, and the others' drafts show as "what others think now" in the task. Exact anchors.
Touched up afterwards with anchored edits: units match `\\s` (formatted quantities use a no-break space), the sorting
helper reads an item's label without its crowd line, and the results row of Desert carries the "Everyone" column."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🧪️tests/🚶️learner-journey/🟦️.tsx")
text = path.read_text(encoding="utf-8")
EDITS = [
    ("  catalogView,\n", "  catalogView,\n  crowdView,\n"),
    ("        return leaderboard([...this.learners.values()], this.view);\n",
     "        return leaderboard([...this.learners.values()], this.view);\n"
     "      case \"crowd\":\n"
     "        return crowdView(QUIZ, [...this.learners.values()].flatMap((state) => state.runs.flatMap((run) => (run.result === undefined ? [] : [run.result]))));\n"),
    ("    await deliver(home, { type: \"welcome\", session: \"r-me\", colour: 0, roster: [] });\n"
     "    await waitFor(() => expect(home.sent).toEqual([{ type: \"state\", state: { tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag } }]));\n",
     "    await deliver(home, { type: \"welcome\", session: \"r-me\", colour: 0, roster: [] });\n"
     "    const watched = [\"introduction\", \"leaderboard\", \"badges\", `quiz/${QUIZ.id}`, `quiz/${QUIZ.id}/thinking`].map((room) => `${CATALOG.id}/${room}`);\n"
     "    await waitFor(() =>\n"
     "      expect(home.sent).toEqual([\n"
     "        { type: \"state\", state: { tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag } },\n"
     "        { type: \"watch\", scopes: watched, intervalMs: 250 },\n"
     "      ]),\n"
     "    );\n"
     "    await deliver(home, { type: \"watched\", scope: `${CATALOG.id}/leaderboard`, entries: [{ session: \"w-mira\", colour: 4, surface: \"leaderboard\", state: { cursor: { anchor: \"leaderboard\", x: 0.5, y: 0.5 }, tag: learnerTag(mira) } }], left: [], snapshot: true });\n"
     "    const inBoard = await waitFor(() => {\n"
     "      const mark = document.querySelector<HTMLElement>(`[data-layered-pane=\"board\"] [data-pane-peers=\"${CATALOG.id}/leaderboard\"] [data-peer=\"cursor\"]`);\n"
     "      expect(mark).not.toBeNull();\n"
     "      return mark!;\n"
     "    });\n"
     "    expect(inBoard.textContent).toBe(\"Mira\");\n"
     "    expect(inBoard.closest(\"[aria-hidden]\")).not.toBeNull();\n"
     "    await deliver(home, { type: \"watched\", scope: `${CATALOG.id}/quiz/${QUIZ.id}/thinking`, entries: [{ session: \"t-mira\", colour: 4, surface: \"thinking\", state: { answers: {}, tag: learnerTag(mira) } }], left: [] });\n"
     "    await waitFor(() => expect(document.querySelector(`[data-layered-pane=\"${QUIZ.id}\"] [data-crowd-source=\"live\"]`)?.textContent).toBe(\"👥What others think now· Thinking along: 1\"));\n"
     "    await deliver(home, { type: \"watched\", scope: `${CATALOG.id}/leaderboard`, entries: [], left: [\"w-mira\"] });\n"
     "    expect(document.querySelector(\"[data-pane-peers]\")).toBeNull();\n"),
    ("    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: \"run\", quiz: QUIZ.id, task: task.getAttribute(\"data-presence-anchor\")!.slice(\"task:\".length) }));\n  });\n",
     "    await waitFor(() => expect((roster.sent.at(-1) as { readonly state: { readonly place: unknown } }).state.place).toEqual({ screen: \"run\", quiz: QUIZ.id, task: task.getAttribute(\"data-presence-anchor\")!.slice(\"task:\".length) }));\n"
     "\n"
     "    const thinking = open(`${CATALOG.id}/quiz/${QUIZ.id}/thinking`);\n"
     "    expect(thinking.url.endsWith(\"?surface=thinking\")).toBe(true);\n"
     "    const miraThinks = {\n"
     "      answers: {\n"
     "        climates: { kind: \"classification\", assignments: { desert: \"hot-dry\", fjord: \"hot-dry\" } },\n"
     "        lamps: { kind: \"matching\", values: { power: { floodlight: 2000, led: 8 } } },\n"
     "        masses: { kind: \"sorting\", order: [\"mouse\", \"cat\", \"horse\"] },\n"
     "      },\n"
     "      tag: learnerTag(mira),\n"
     "    };\n"
     "    await deliver(thinking, { type: \"welcome\", session: \"t-me\", colour: 0, roster: [{ session: \"t-mira\", colour: 4, surface: \"thinking\", state: miraThinks }] });\n"
     "    await waitFor(() => expect(thinking.sent.at(-1)).toEqual({ type: \"state\", state: { answers: {}, tag: (roster.sent.at(-1) as { readonly state: { readonly tag: string } }).state.tag } }));\n"
     "    await waitFor(() => expect(within(task).getByText(\"What others think now\").closest(\"[data-crowd-source]\")?.getAttribute(\"data-crowd-source\")).toBe(\"live\"));\n"
     "    const sentences = [...task.querySelectorAll(\"[data-crowd-item] .sr-only\")].map((sentence) => sentence.textContent);\n"
     "    expect(sentences.length).toBeGreaterThan(0);\n"
     "    for (const sentence of sentences) expect(sentence).toMatch(/^(The others on (Desert|Fjord): Hot and dry 1×|On average the others put (Mouse|Cat|Horse) at place [123] of 3|The others on (LED bulb|Floodlight): (8 W|2 kW) 1×)$/u);\n"
     "  });\n"),
]
for old, new in EDITS:
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:90]!r}")
    text = text.replace(old, new)
for attempt in range(40):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.5)
else:
    sys.exit("write failed")
print("[crowd journey test] done")
