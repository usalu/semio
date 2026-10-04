"""🧭️ Moves the learner-journey test onto the card-grid UI (2026-09-29): landmarks and regions instead of articles,
the navbar's language switch, the learner and badges cards. Exact, unique anchors; refuses to write otherwise."""
import io, sys
path = sys.argv[1]
text = io.open(path, encoding="utf-8", newline="").read()
replacements = [
    ('    await user.click(screen.getByRole("button", { name: "English" }));\n    expect(screen.getByRole("heading", { level: 1, name: "Welcome to the test catalog" })).toBeTruthy();\n    expect(screen.getByRole("button", { name: "English" }).getAttribute("aria-pressed")).toBe("true");',
     '    await user.click(within(navbar()).getByRole("button", { name: "English" }));\n    expect(screen.getByRole("heading", { level: 1, name: "Welcome to the test catalog" })).toBeTruthy();\n    expect(within(navbar()).getByRole("button", { name: "English" }).getAttribute("aria-pressed")).toBe("true");\n    expect(within(screen.getByRole("region", { name: "Settings" })).getByRole("button", { name: "English" }).getAttribute("aria-pressed")).toBe("true");'),
    ('    await screen.findByText("Welcome, Ada Lovelace");\n    expect(screen.getByText("Signed in as Ada Lovelace")).toBeTruthy();',
     '    const learnerCard = await screen.findByRole("region", { name: "Ada Lovelace" });\n    expect(within(learnerCard).getByText("0 points")).toBeTruthy();\n    expect(within(learnerCard).getByText("0 of 1 quizzes played")).toBeTruthy();'),
    ('    const quizCard = screen.getByRole("article", { name: "Household physics" });\n    expect(within(quizCard).getByText("Not attempted yet")).toBeTruthy();\n    expect(screen.getAllByText("Not yet earned")).toHaveLength(2);',
     '    const quizCard = screen.getByRole("region", { name: "Household physics" });\n    expect(within(quizCard).getByText("Not attempted yet")).toBeTruthy();\n    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("0 of 2 badges earned")).toBeTruthy();'),
    ('    expect(description.closest(".quiz-table-scroll")).toBeNull();',
     '    expect(table.parentElement?.contains(description)).toBe(false);'),
    ('    await screen.findByText("Welcome, Ada Lovelace");\n    expect(screen.getByRole("heading", { level: 1, name: "Quizzes" })).toBeTruthy();\n    const card = screen.getByRole("article", { name: "Household physics" });\n    await within(card).findByText("Best score: 100%");\n    expect(within(card).getByRole("button", { name: "Start again" })).toBeTruthy();\n    expect(screen.getAllByText(/^Earned on /u)).toHaveLength(2);',
     '    await screen.findByRole("region", { name: "Ada Lovelace" });\n    expect(screen.getByRole("heading", { level: 1, name: "Quizzes" })).toBeTruthy();\n    const card = screen.getByRole("region", { name: "Household physics" });\n    await within(card).findByText("Best score: 100%");\n    expect(within(card).getByRole("button", { name: "Start again" })).toBeTruthy();\n    expect(within(card).getByText(/^Earned here: /u).textContent).toContain("All done");\n    expect(within(screen.getByRole("region", { name: "Badges" })).getByText("2 of 2 badges earned")).toBeTruthy();'),
    ('    const card = await screen.findByRole("article", { name: "Household physics" });',
     '    const card = await screen.findByRole("region", { name: "Household physics" });'),
    ('function app(proctor: FakeProctor, storage: StorageArea) {',
     'function navbar(): HTMLElement {\n  return screen.getByRole("navigation", { name: /^(?:Main navigation|Hauptnavigation)$/u });\n}\n\nfunction app(proctor: FakeProctor, storage: StorageArea) {'),
]
for old, new in replacements:
    if text.count(old) != 1:
        sys.exit(f"anchor found {text.count(old)} times: {old[:80]!r}")
    text = text.replace(old, new)
io.open(path, "w", encoding="utf-8", newline="").write(text)
print("rewrote", path)
