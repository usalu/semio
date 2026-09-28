"""🤝️ C12 session 14c (first live run on 7800/p24, `c12collab-p24-en`): two harness defects the run exposed.
STEP 6 waited for the check-in MESSAGE to appear in the History panel, which the shell never renders (the hub Check In reports
through `#s-checkin-status`, the checkpoint through `data-history-json.currentCheckpointId`) — the check-in itself succeeded
("Checked in"). STEP 15 cut the links while both humans were still on STEP 14's draw/puzzle3d documents, so the writer editors it
compares did not exist. One helper re-opens the writer document for both humans (STEP 14's caret leg and STEP 15).
Idempotent; `--dry-run` prints the plan; every anchor must match exactly once or nothing is written."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts"

HUNKS = [
    ("step-6-name",
     '  "user1 checks in with a message; history shows it and the space table\'s updated column moves for both",\n',
     '  "user1 checks in with a message; the check-in status reads checked in, the active checkpoint moves and the space table\'s updated column moves for both",\n'),
    ("locale-checked-in",
     'const COLLAB_E2E_LOCALES: Readonly<Record<CollabLocale, { readonly browser: string; readonly studio: string; readonly public: string; readonly author: string }>> = {\n  en: { browser: "en-US", studio: "Studio", public: "Public", author: "Author" },\n  de: { browser: "de-DE", studio: "Studio", public: "Öffentlich", author: "Autor" },\n};\n',
     'const COLLAB_E2E_LOCALES: Readonly<Record<CollabLocale, { readonly browser: string; readonly studio: string; readonly public: string; readonly author: string; readonly checkedIn: string }>> = {\n  en: { browser: "en-US", studio: "Studio", public: "Public", author: "Author", checkedIn: "Checked in" },\n  de: { browser: "de-DE", studio: "Studio", public: "Öffentlich", author: "Autor", checkedIn: "Eingecheckt" },\n};\n'),
    ("locale-doc",
     " * Space app's own `LocalizedLabel`s (`🪐️space/…/🏠️home/…/✏️editor/🦀️.rs`). */\ntype CollabLocale",
     " * Space app's own `LocalizedLabel`s (`🪐️space/…/🏠️home/…/✏️editor/🦀️.rs`); `checkedIn` is the shell's ready Check In status\n * (`CHECKIN_STATUS_LABELS.ready`, `🛠️ShellHelpers/🟦️.tsx`). */\ntype CollabLocale"),
    ("step-6-body",
     """      const checkinButton = await collabOpenCheckin(user1);
      await checkinButton.click();
      const message = `collab check-in ${Date.now()}`;
      await user1.locator('[id="s-checkin-message"]').fill(message);
      const historyEntryVisible = user1.getByText(message, { exact: false });
      await user1.locator('[id="s-checkin-message"]').press("Enter");
      await historyEntryVisible.first().waitFor({ state: "visible", timeout: 15_000 });
""",
     """      const checkinButton = await collabOpenCheckin(user1);
      const checkpointBefore = await collabCurrentCheckpointId(user1);
      await checkinButton.click();
      const message = `collab check-in ${Date.now()}`;
      await user1.locator('[id="s-checkin-message"]').fill(message);
      await user1.locator('[id="s-checkin-message"]').press("Enter");
      const status = await collabAwaitCheckinStatus(user1, 60_000);
      spaceE2eAssert(status === COLLAB_E2E_LOCALES[locale].checkedIn, `the hub Check In ended as ${JSON.stringify(status)}, not ${JSON.stringify(COLLAB_E2E_LOCALES[locale].checkedIn)}`);
      const checkpointAfter = await collabCurrentCheckpointId(user1);
      spaceE2eAssert(checkpointAfter !== null && checkpointAfter !== checkpointBefore, `the active checkpoint did not move (${checkpointBefore} → ${checkpointAfter})`);
"""),
    ("step-6-record",
     '      record(6, true, "check-in dispatched and the space table\'s row changed for both users");\n',
     '      record(6, true, `check-in "${message}" reads ${JSON.stringify(COLLAB_E2E_LOCALES[locale].checkedIn)}, checkpoint moved, the space table\'s row changed for both users`);\n'),
    ("helpers",
     """/** 🪪️ The hub user id of the human signed in on `page`""",
     """/** 📌️ The focused program's active checkpoint id, from the shell root's `data-history-json` (`null` before any). */
async function collabCurrentCheckpointId(page: import("playwright").Page): Promise<string | null> {
  return page.evaluate(() => (JSON.parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json") ?? "null") as { readonly currentCheckpointId?: string | null } | null)?.currentCheckpointId ?? null);
}

/** ⏳️ The hub Check In's terminal status sentence on `page` (`#s-checkin-status`: ready, cancelled or a refusal), or the last
 * sentence it showed when `deadlineMs` ran out — a running check-in reads "Checking in… n/m". */
async function collabAwaitCheckinStatus(page: import("playwright").Page, deadlineMs: number): Promise<string> {
  const status = page.locator('[id="s-checkin-status"]').first();
  const deadline = Date.now() + deadlineMs;
  let text = "";
  while (Date.now() < deadline) {
    text = ((await status.innerText().catch(() => "")) ?? "").trim();
    if (text !== "" && (await page.locator('[id="s-checkin-abort"]').count()) === 0) return text;
    await page.waitForTimeout(250);
  }
  return text;
}

/** 🚪️ Re-opens the writer document of steps 3–13 for both humans from the Space index (a step before navigated away) and
 * waits until both editors mounted. */
async function collabReopenWriter(opts: { readonly user1: import("playwright").Page; readonly user2: import("playwright").Page; readonly spaceId: string; readonly artifactId: string; readonly purpose: string }): Promise<void> {
  for (const [label, page] of [["user1", opts.user1], ["user2", opts.user2]] as const) {
    await collabOpenSpace(page, opts.spaceId);
    await collabWaitForRow(page, "artifact", opts.artifactId, 30_000);
    await collabRowAction(page, "artifact", opts.artifactId, "open").catch(() => undefined);
    spaceE2eAssert(await collabWaitForEditor(page, 120_000), `${label}'s writer editor never re-mounted for ${opts.purpose}`);
  }
}

/** 🪪️ The hub user id of the human signed in on `page`"""),
    ("step-14-caret-reopen",
     """      if (opts.spaceId && opts.artifactId) {
        await collabOpenSpace(opts.user1, opts.spaceId!);
        await collabOpenSpace(opts.user2, opts.spaceId!);
        await collabWaitForRow(opts.user1, "artifact", opts.artifactId, 30_000);
        await collabWaitForRow(opts.user2, "artifact", opts.artifactId, 30_000);
        await collabRowAction(opts.user1, "artifact", opts.artifactId, "open").catch(() => undefined);
        await collabRowAction(opts.user2, "artifact", opts.artifactId, "open").catch(() => undefined);
        spaceE2eAssert(await collabWaitForEditor(opts.user1, 120_000), "user1's writer editor never re-mounted for the caret leg");
        spaceE2eAssert(await collabWaitForEditor(opts.user2, 120_000), "user2's writer editor never re-mounted for the caret leg");
        details.push""",
     """      if (opts.spaceId && opts.artifactId) {
        await collabReopenWriter({ user1: opts.user1, user2: opts.user2, spaceId: opts.spaceId, artifactId: opts.artifactId, purpose: "the caret leg" });
        details.push"""),
    ("step-15-reopen",
     """  try {
    const rounds: string[] = [];
    for (const outageMs of COLLAB_E2E_LINK_CUTS_MS) {
""",
     """  try {
    const rounds: string[] = [];
    await collabReopenWriter({ user1: opts.user1, user2: opts.user2, spaceId: opts.spaceId!, artifactId: opts.artifactId!, purpose: "the link cuts (STEP 14 left both humans on its draw/puzzle3d documents)" });
    for (const outageMs of COLLAB_E2E_LINK_CUTS_MS) {
"""),
]

dry = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
problems, plan = [], []
for name, old, new in HUNKS:
    if new in text and text.count(old) == 0:
        plan.append(f"{name}: already applied")
        continue
    count = text.count(old)
    if count != 1:
        problems.append(f"{name}: anchor matches {count}×")
        continue
    text = text.replace(old, new)
    plan.append(f"{name}: planned")
print("\n".join(plan))
print(f"{len(problems)} problems" + "".join(f"\n  {p}" for p in problems))
if problems:
    sys.exit(1)
if not dry:
    open(PATH, "w", encoding="utf-8").write(text)
    print("written")
