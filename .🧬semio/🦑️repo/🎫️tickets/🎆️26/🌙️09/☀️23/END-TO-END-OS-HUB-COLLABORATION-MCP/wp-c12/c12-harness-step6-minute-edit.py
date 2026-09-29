"""🕐️ C12 14c: collab STEP 6 judged "the space table's updated column moves" by any change of the row text, but the column shows
MINUTES ("2026-09-28 21:58 UTC", de "28.09.2026, 21:58 UTC") — a check-in in the artifact's creation minute leaves the text as
it was (run `c12collab-8010-dbg2`: check-in PASSED, row 21:58 → 21:58 → false FAIL). The step now requires each user's row to
show the check-in's minute or later (a real move whenever the minute differs; the displayed truth when it cannot). Idempotent."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts"
ROW_WAIT = """      const rowAfter{n}Deadline = Date.now() + 30_000;
      let rowAfter{n} = rowBefore{n};
      while (Date.now() < rowAfter{n}Deadline) {{
        rowAfter{n} =
          (await user{n}
            .locator(`[data-ui-node-key="artifact:${{artifactId}}"]`)
            .innerText()
            .catch(() => "")) ?? "";
        if (rowAfter{n} !== rowBefore{n}) break;
        await user{n}.waitForTimeout(1_000);
      }}
"""
ROW_WAIT_NEW = """      const rowAfter{n} = await collabAwaitRowUpdatedMinute(user{n}, artifactId, checkedInMinute, 30_000);
"""
HUNKS = [
    ('      await user1.locator(\'[id="s-checkin-message"]\').fill(message);\n      await user1.locator(\'[id="s-checkin-message"]\').press("Enter");\n',
     '      await user1.locator(\'[id="s-checkin-message"]\').fill(message);\n      const checkedInMinute = Math.floor(Date.now() / 60_000);\n      await user1.locator(\'[id="s-checkin-message"]\').press("Enter");\n'),
    (ROW_WAIT.format(n=1), ROW_WAIT_NEW.format(n=1)),
    ("      spaceE2eAssert(rowAfter1 !== rowBefore1, `user1's space table row for ${artifactId} did not change after check-in (before: ${JSON.stringify(rowBefore1)}, after: ${JSON.stringify(rowAfter1)})`);\n",
     "      spaceE2eAssert(rowAfter1.minute !== null && rowAfter1.minute >= checkedInMinute, `user1's space table row for ${artifactId} does not show the check-in's minute (before: ${JSON.stringify(rowBefore1)}, after: ${JSON.stringify(rowAfter1.text)})`);\n"),
    (ROW_WAIT.format(n=2), ROW_WAIT_NEW.format(n=2)),
    ("      spaceE2eAssert(rowAfter2 !== rowBefore2, `user2's space table row for ${artifactId} did not change after user1's check-in (before: ${JSON.stringify(rowBefore2)}, after: ${JSON.stringify(rowAfter2)})`);\n",
     "      spaceE2eAssert(rowAfter2.minute !== null && rowAfter2.minute >= checkedInMinute, `user2's space table row for ${artifactId} does not show user1's check-in minute (before: ${JSON.stringify(rowBefore2)}, after: ${JSON.stringify(rowAfter2.text)})`);\n"),
    ("      record(6, true, `check-in \"${message}\" reads ${JSON.stringify(COLLAB_E2E_LOCALES[locale].checkedIn)}, checkpoint moved, the space table's row changed for both users`);\n",
     "      record(6, true, `check-in \"${message}\" reads ${JSON.stringify(COLLAB_E2E_LOCALES[locale].checkedIn)}, checkpoint moved, both users' space table rows show the check-in minute`);\n"),
    ("/** 📌️ The focused program's active checkpoint id, from the shell root's `data-history-json` (`null` before any). */\n",
     "/** 🕐️ The minute (epoch minutes, UTC) an artifact row's \"updated\" cell shows — en `2026-09-28 21:58 UTC`, de\n"
     " * `28.09.2026, 21:58 UTC` — or `null` when the row shows none. */\n"
     "function collabRowUpdatedMinute(text: string): number | null {\n"
     "  const en = /(\\d{4})-(\\d{2})-(\\d{2}) (\\d{2}):(\\d{2}) UTC/u.exec(text);\n"
     "  const de = /(\\d{2})\\.(\\d{2})\\.(\\d{4}), (\\d{2}):(\\d{2}) UTC/u.exec(text);\n"
     "  const parts = en ? [en[1], en[2], en[3], en[4], en[5]] : de ? [de[3], de[2], de[1], de[4], de[5]] : null;\n"
     "  return parts === null ? null : Math.floor(Date.UTC(Number(parts[0]), Number(parts[1]) - 1, Number(parts[2]), Number(parts[3]), Number(parts[4])) / 60_000);\n"
     "}\n\n"
     "/** 🕐️ Waits until `page`'s row for `artifactId` shows `minute` or later in its \"updated\" cell (bounded); answers the last row read. */\n"
     "async function collabAwaitRowUpdatedMinute(page: import(\"playwright\").Page, artifactId: string, minute: number, deadlineMs: number): Promise<{ readonly text: string; readonly minute: number | null }> {\n"
     "  const deadline = Date.now() + deadlineMs;\n"
     "  let text = \"\";\n"
     "  while (Date.now() < deadline) {\n"
     "    text = (await page.locator(`[data-ui-node-key=\"artifact:${artifactId}\"]`).innerText().catch(() => \"\")) ?? \"\";\n"
     "    const shown = collabRowUpdatedMinute(text);\n"
     "    if (shown !== null && shown >= minute) return { text, minute: shown };\n"
     "    await page.waitForTimeout(1_000);\n"
     "  }\n"
     "  return { text, minute: collabRowUpdatedMinute(text) };\n"
     "}\n\n"
     "/** 📌️ The focused program's active checkpoint id, from the shell root's `data-history-json` (`null` before any). */\n"),
]
dry = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
problems = []
for old, new in HUNKS:
    if new in text:
        continue
    if text.count(old) != 1:
        problems.append((old[:70], text.count(old)))
        continue
    text = text.replace(old, new)
print(f"{len(problems)} problems {problems}")
if problems:
    sys.exit(1)
if not dry:
    open(PATH, "w", encoding="utf-8").write(text)
    print("written")
