"""🛂️ C12 (2026-09-29 16:2x, live wave p33): STEP 7 asked the hub's admin surface for both humans' connections right after STEP 6 had
navigated user1 back to the Space index (to read the table row), i.e. while user1 held no document session — the admin listing
(`source: recorded-sync-sessions`) then rightly named user2 only. On p24 STEP 7 only passed because STEP 6 failed before leaving the
editor. STEP 7 now re-opens user1's document first and polls the listing until both sessions are recorded (bounded, 30 s).
Idempotent; usage: python3 <this> [--apply]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts"
OLD = """  else try {
    const connectionsRes = await fetch(`${hubBaseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${capability}` } });
    spaceE2eAssert(connectionsRes.ok, `GET /admin/api/connections returned ${connectionsRes.status}`);
    const connections = (await connectionsRes.json()) as readonly Record<string, unknown>[];
    const text = JSON.stringify(connections);
    const [user1Id, user2Id] = [await collabHubUserId(user1), await collabHubUserId(user2)];
"""
NEW = """  else try {
    if (spaceId && artifactId) {
      await collabOpenSpace(user1, spaceId);
      await collabWaitForRow(user1, "artifact", artifactId, 30_000);
      await collabRowAction(user1, "artifact", artifactId, "open");
      spaceE2eAssert(await collabWaitForEditor(user1, 120_000), "user1's writer editor never re-mounted after STEP 6, so user1 holds no document connection to list");
    }
    const [user1Id, user2Id] = [await collabHubUserId(user1), await collabHubUserId(user2)];
    let text = "";
    for (const deadline = Date.now() + 30_000; ; ) {
      const connectionsRes = await fetch(`${hubBaseUrl}/admin/api/connections`, { headers: { authorization: `Bearer ${capability}` } });
      spaceE2eAssert(connectionsRes.ok, `GET /admin/api/connections returned ${connectionsRes.status}`);
      text = JSON.stringify(await connectionsRes.json());
      if ((user1Id !== null && text.includes(user1Id) && user2Id !== null && text.includes(user2Id)) || Date.now() >= deadline) break;
      await user1.waitForTimeout(1_000);
    }
"""
text = open(PATH, encoding="utf-8").read()
if NEW in text:
    print("present")
elif text.count(OLD) == 1:
    if "--apply" in sys.argv:
        open(PATH, "w", encoding="utf-8").write(text.replace(OLD, NEW))
        print("applied")
    else:
        print("hunk")
else:
    print("PROBLEM", text.count(OLD))
