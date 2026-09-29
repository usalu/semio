"""🌐️ C12 (2026-09-29 16:1x, live wave p33): the collab harness found a row action by its ENGLISH aria-label prefix only
(`button[aria-label^="share:" i]`), so every `de` run died at STEP 2 (the German Home labels the action "Teilen: …"). The row verbs
join the harness's locale table and a row action is found by its label in the run's locale. Idempotent; usage: python3 <this> [--apply]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts"
HUNKS = [
    ("""const COLLAB_E2E_LOCALES: Readonly<Record<CollabLocale, { readonly browser: string; readonly studio: string; readonly public: string; readonly author: string; readonly checkedIn: string }>> = {
  en: { browser: "en-US", studio: "Studio", public: "Public", author: "Author", checkedIn: "Checked in" },
  de: { browser: "de-DE", studio: "Studio", public: "Öffentlich", author: "Autor", checkedIn: "Eingecheckt" },
};""", """const COLLAB_E2E_LOCALES: Readonly<Record<CollabLocale, { readonly browser: string; readonly studio: string; readonly public: string; readonly author: string; readonly checkedIn: string; readonly open: string; readonly share: string }>> = {
  en: { browser: "en-US", studio: "Studio", public: "Public", author: "Author", checkedIn: "Checked in", open: "Open", share: "Share" },
  de: { browser: "de-DE", studio: "Studio", public: "Öffentlich", author: "Autor", checkedIn: "Eingecheckt", open: "Öffnen", share: "Teilen" },
};"""),
    ("""/** 🖱️ Activates one named row action (`Open: <id>`, `Share: <name>`, case-insensitive) from the keyboard — the ACTIONS column can sit past
 * the window's right edge, where a pointer click times out. */
async function collabRowAction(page: import("playwright").Page, prefix: "space" | "artifact", id: string, verb: "open" | "share"): Promise<void> {
  const action = page.locator(`[data-ui-node-key="${prefix}:${id}"] button[aria-label^="${verb}:" i]`).first();""", """/** 🖱️ Activates one named row action (`Open: <id>` / `Öffnen: <id>`, `Share: <name>` / `Teilen: <name>`, case-insensitive) from the
 * keyboard — the ACTIONS column can sit past the window's right edge, where a pointer click times out. The label is the one of
 * the shell's locale, so the selector accepts every locale's label of the verb. */
async function collabRowAction(page: import("playwright").Page, prefix: "space" | "artifact", id: string, verb: "open" | "share"): Promise<void> {
  const labels = [...new Set(Object.values(COLLAB_E2E_LOCALES).map((locale) => locale[verb]))];
  const action = page.locator(labels.map((label) => `[data-ui-node-key="${prefix}:${id}"] button[aria-label^="${label}:" i]`).join(", ")).first();"""),
]
text = open(PATH, encoding="utf-8").read()
plan = []
for old, new in HUNKS:
    if new in text:
        plan.append("present")
    elif text.count(old) == 1:
        text = text.replace(old, new)
        plan.append("hunk")
    else:
        plan.append(f"PROBLEM {text.count(old)}")
print(plan)
if "--apply" in sys.argv and not any(p.startswith("PROBLEM") for p in plan):
    open(PATH, "w", encoding="utf-8").write(text)
    print("applied")
